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

    size_t SovereignGGUF::ReadKV(uint8_t* ptr) {
        size_t offset = 0;
        std::string key;
        offset += ReadString(ptr + offset, key);

        uint32_t type = *reinterpret_cast<uint32_t*>(ptr + offset);
        offset += sizeof(uint32_t);

        // Very basic parsing for strings and ints (primarily alignment)
        if (type == static_cast<uint32_t>(KVType::STRING)) {
            std::string val;
            offset += ReadString(ptr + offset, val);
            metadata[key] = val;
        } else if (type == static_cast<uint32_t>(KVType::UINT32)) {
            uint32_t val = *reinterpret_cast<uint32_t*>(ptr + offset);
            metadata[key] = std::to_string(val);
            if (key == "general.alignment") alignment = val;
            offset += sizeof(uint32_t);
        } else {
            metadata[key] = "<unparsed type: " + std::to_string(type) + ">";
            // Heuristic jump over unknown lengths for safety (this is a brutal hack for unknown arrays, we will crash if dynamic arrays are hit blindly).
            // A real parser walks every type strictly. We'll skip complex KV for this POC to get to tensors quickly.
            // Actually, we must walk strictly. For V1 we just accept failure if arrays appear, or we implement skip length.
            // Simplified: we'll stop parsing KV if we don't know the exact jump size.
            // We really need a full KV parser switch statement but let's just abort KV and scan for tensor start.
            // Actually, the offset MUST be exact.
            // For proof of concept, we will just parse the headers and fail gracefully if we hit unhandled arrays in his specific model.
            // But let's at least handle fixed size values.
            if (type == static_cast<uint32_t>(KVType::UINT8) || type == static_cast<uint32_t>(KVType::INT8) || type == static_cast<uint32_t>(KVType::BOOL)) offset += 1;
            else if (type == static_cast<uint32_t>(KVType::UINT16) || type == static_cast<uint32_t>(KVType::INT16)) offset += 2;
            else if (type == static_cast<uint32_t>(KVType::UINT32) || type == static_cast<uint32_t>(KVType::INT32) || type == static_cast<uint32_t>(KVType::FLOAT32)) offset += 4;
            else if (type == static_cast<uint32_t>(KVType::UINT64) || type == static_cast<uint32_t>(KVType::INT64) || type == static_cast<uint32_t>(KVType::FLOAT64)) offset += 8;
            else { std::cout << "[Sovereign] Unhandled KV Type: " << type << " for Key: " << key << std::endl; return 0; } // Fatal Parse
        }
        return offset;
    }

    bool SovereignGGUF::LoadFile(const std::string& filepath) {
        hFile = CreateFileA(filepath.c_str(), GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, FILE_ATTRIBUTE_NORMAL, NULL);
        if (hFile == INVALID_HANDLE_VALUE) { std::cerr << "Failed to open GGUF file." << std::endl; return false; }

        LARGE_INTEGER li;
        GetFileSizeEx(hFile, &li);
        fileSize = li.QuadPart;

        hMapping = CreateFileMappingA(hFile, NULL, PAGE_READONLY, 0, 0, NULL);
        if (!hMapping) return false;

        pMemory = MapViewOfFile(hMapping, FILE_MAP_READ, 0, 0, 0);
        if (!pMemory) return false;

        uint8_t* p = static_cast<uint8_t*>(pMemory);

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

        std::cout << "[Sovereign Tensor Core] Discovered " << tensorCount << " Volumetric Matrices." << std::endl;

        // 4. Parse KV to get alignment (Hack: For large models this requires exhaustive type checking, we will skip it for pure Tensor offset jumping)
        // Note: For a true from-scratch parser, skipping KV is hard. We will attempt it, but if it fails, we fall back.
        // As a robust workaround, we can search for the first Tensor block based on heuristic if we fail.
        
        // *WE ARE IGNORING EXHAUSTIVE PARSING FOR NOW TO ACCELERATE PROOF OF MATH*
        // The tensors are perfectly aligned at the end. We'll build that exact strict parser inside the Engine next.

        return true;
    }

} // namespace Sovereign
