#include "Sovereign_Tensor_Core.h"
#include <iostream>
#include <cstring>
#include <sstream>

namespace Sovereign {

    SovereignGGUF::SovereignGGUF() : hFile(INVALID_HANDLE_VALUE), hMapping(NULL), pMemory(nullptr), fileSize(0), dataOffset(0), alignment(32) {}

    SovereignGGUF::~SovereignGGUF() {
        if (pMemory) UnmapViewOfFile(pMemory);
        if (hMapping) CloseHandle(hMapping);
        if (hFile != INVALID_HANDLE_VALUE) CloseHandle(hFile);
    }

    size_t SovereignGGUF::ReadString(uint8_t* ptr, std::string& out) {
        uint64_t len = *reinterpret_cast<uint64_t*>(ptr);
        out.assign(reinterpret_cast<char*>(ptr + sizeof(uint64_t)), static_cast<size_t>(len));
        return sizeof(uint64_t) + len;
    }

    size_t SkipValue(uint8_t* ptr, uint32_t type) {
        size_t offset = 0;
        switch (static_cast<KVType>(type)) {
            case KVType::UINT8: case KVType::INT8: case KVType::BOOL: return 1;
            case KVType::UINT16: case KVType::INT16: return 2;
            case KVType::UINT32: case KVType::INT32: case KVType::FLOAT32: return 4;
            case KVType::UINT64: case KVType::INT64: case KVType::FLOAT64: return 8;
            case KVType::STRING: {
                uint64_t len = *reinterpret_cast<uint64_t*>(ptr);
                return 8 + len;
            }
            case KVType::ARRAY: {
                uint32_t sub_type = *reinterpret_cast<uint32_t*>(ptr);
                uint64_t count = *reinterpret_cast<uint64_t*>(ptr + 4);
                size_t sub_offset = 12;
                for (uint64_t i = 0; i < count; ++i) {
                    sub_offset += SkipValue(ptr + sub_offset, sub_type);
                }
                return sub_offset;
            }
            default: return 0;
        }
    }

    size_t SovereignGGUF::ReadKV(uint8_t* ptr) {
        size_t offset = 0;
        std::string key;
        offset += ReadString(ptr + offset, key);

        uint32_t type = *reinterpret_cast<uint32_t*>(ptr + offset);
        offset += sizeof(uint32_t);

        if (type == static_cast<uint32_t>(KVType::STRING)) {
            std::string val;
            ReadString(ptr + offset, val);
            metadata[key] = val;
        } else if (type == static_cast<uint32_t>(KVType::UINT32)) {
            uint32_t val = *reinterpret_cast<uint32_t*>(ptr + offset);
            metadata[key] = std::to_string(val);
            if (key == "general.alignment") alignment = val;
        }
        
        return offset + SkipValue(ptr + offset, type);
    }

    TensorInfo* SovereignGGUF::GetTensor(const std::string& name) {
        if (tensorMap.count(name)) return tensorMap[name];
        return nullptr;
    }

    size_t SovereignGGUF::ReadTensorMeta(uint8_t* ptr, TensorInfo& info) {
        size_t offset = 0;
        offset += ReadString(ptr + offset, info.name);
        uint32_t n_dims = *reinterpret_cast<uint32_t*>(ptr + offset);
        offset += sizeof(uint32_t);
        info.dims.resize(n_dims);
        for (uint32_t i = 0; i < n_dims; ++i) {
            info.dims[i] = *reinterpret_cast<uint64_t*>(ptr + offset);
            offset += sizeof(uint64_t);
        }
        info.type = static_cast<TensorType>(*reinterpret_cast<uint32_t*>(ptr + offset));
        offset += sizeof(uint32_t);
        info.offset = *reinterpret_cast<uint64_t*>(ptr + offset);
        offset += sizeof(uint64_t);
        return offset;
    }

    bool SovereignGGUF::LoadFile(const std::string& filepath) {
        hFile = CreateFileA(filepath.c_str(), GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
        if (hFile == INVALID_HANDLE_VALUE) { std::cerr << "Failed to open GGUF file: " << filepath << std::endl; return false; }

        LARGE_INTEGER li;
        GetFileSizeEx(hFile, &li);
        fileSize = li.QuadPart;

        hMapping = CreateFileMappingA(hFile, NULL, PAGE_READONLY, 0, 0, NULL);
        if (!hMapping) return false;

        pMemory = MapViewOfFile(hMapping, FILE_MAP_READ, 0, 0, 0);
        if (!pMemory) return false;

        uint8_t* p = static_cast<uint8_t*>(pMemory);
        uint8_t* start = p;

        // 1. Verify Magic Bytes
        uint32_t magic = *reinterpret_cast<uint32_t*>(p);
        if (magic != GGUF_MAGIC) { std::cerr << "NOT A VALID GGUF FILE." << std::endl; return false; }
        p += sizeof(uint32_t);

        // 2. Version
        uint32_t version = *reinterpret_cast<uint32_t*>(p);
        p += sizeof(uint32_t);
        if (version < 2 || version > 3) { std::cerr << "Unsupported GGUF version: " << version << std::endl; return false; }

        // 3. Tensor and KV Count
        uint64_t tensorCount = *reinterpret_cast<uint64_t*>(p); p += sizeof(uint64_t);
        uint64_t kvCount = *reinterpret_cast<uint64_t*>(p); p += sizeof(uint64_t);

        std::cout << "[Sovereign] Initializing GGUF Substrate... Parsing " << kvCount << " Keys." << std::endl;

        // 4. Parse KV Pairs
        for (uint64_t i = 0; i < kvCount; ++i) {
            size_t kv_size = ReadKV(p);
            if (kv_size == 0) { std::cerr << "[ERROR] KV Size 0 at index " << i << std::endl; return false; } 
            p += kv_size;
        }

        std::cout << "[Sovereign] Mapping " << tensorCount << " Volumetric Matrices." << std::endl;

        // 5. Parse Tensor Metadata
        tensors.resize(tensorCount);
        for (uint64_t i = 0; i < tensorCount; ++i) {
            p += ReadTensorMeta(p, tensors[i]);
            tensorMap[tensors[i].name] = &tensors[i];
        }

        // 6. Calculate Data Start Offset (Aligned)
        uint64_t currentOffset = (uint64_t)(p - start);
        dataOffset = (currentOffset + alignment - 1) & ~(alignment - 1);
        
        // 7. Assign Pointers
        uint8_t* dataStart = start + dataOffset;
        for (auto& t : tensors) {
            t.data = dataStart + t.offset;
        }

        std::cout << "[SUCCESS] Sovereign Substrate Anchored. Data Offset: " << dataOffset << std::endl;
        return true;
    }

    void SovereignGGUF::PrintTopology() {
        std::cout << "--- Sovereign GGUF Topology ---" << std::endl;
        for (const auto& t : tensors) {
            std::cout << "Tensor: " << t.name << " | Type: " << (uint32_t)t.type << " | Dims: ";
            for (auto d : t.dims) std::cout << d << " ";
            std::cout << "| Offset: " << t.offset << std::endl;
        }
    }

} // namespace Sovereign
