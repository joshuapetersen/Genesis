#pragma once

#include <string>
#include <vector>
#include <map>
#include <cstdint>
#include <windows.h>

namespace Sovereign {

    // GGUF Magic "GGUF"
    const uint32_t GGUF_MAGIC = 0x46554747;
    const uint32_t GGUF_VERSION = 3;

    // Core Type Enums (Mirroring ggml types needed for Q4)
    enum class TensorType : uint32_t {
        F32 = 0,
        F16 = 1,
        Q4_0 = 2,
        Q4_1 = 3,
        Q4_K = 12, // Q4_K is widely used in Q4_K_M quantization
        Q5_K = 13,
        Q6_K = 14,
        Q8_0 = 8,
        Q8_K = 15,
        I32 = 28,
        BF16 = 30
    };

    enum class KVType : uint32_t {
        UINT8 = 0, INT8 = 1, UINT16 = 2, INT16 = 3,
        UINT32 = 4, INT32 = 5, FLOAT32 = 6, BOOL = 7,
        STRING = 8, ARRAY = 9, UINT64 = 10, INT64 = 11,
        FLOAT64 = 12
    };

    struct TensorInfo {
        std::string name;
        uint32_t n_dims;
        std::vector<uint64_t> dims;
        TensorType type;
        uint64_t offset;
        void* data; // Pointer to mapped physical memory
    };

    struct TransformerContext {
        uint32_t dims = 2560;
        uint32_t layers = 42;
        std::vector<float> hidden_state;
        std::vector<float> work_buffer;
        std::vector<float> kv_cache_k;
        std::vector<float> kv_cache_v;
        
        TransformerContext(uint32_t d, uint32_t l, uint32_t max_seq = 2048) : dims(d), layers(l) {
            hidden_state.resize(dims, 0.0f);
            work_buffer.resize(dims * 4, 0.0f); // Large buffer for FFN mid-states
            kv_cache_k.resize(layers * max_seq * dims, 0.0f);
            kv_cache_v.resize(layers * max_seq * dims, 0.0f);
        }
    };

    // The Memory Mapped GGUF Context
    class SovereignGGUF {
    public:
        SovereignGGUF();
        ~SovereignGGUF();

        bool LoadFile(const std::string& filepath);
        void PrintTopology();
        TensorInfo* GetTensor(const std::string& name);

        // Native Mapped Weights
        std::vector<TensorInfo> tensors;
        std::map<std::string, TensorInfo*> tensorMap;
        std::map<std::string, std::string> metadata;

    private:
        HANDLE hFile;
        HANDLE hMapping;
        void* pMemory;
        uint64_t fileSize;
        uint64_t dataOffset;
        uint64_t alignment;

        size_t ReadString(uint8_t* ptr, std::string& out);
        size_t ReadKV(uint8_t* ptr);
        size_t ReadTensorMeta(uint8_t* ptr, TensorInfo& info);
    };

} // namespace Sovereign
