#pragma once
#include "Sovereign_Tensor_Core.h"
#include "Sovereign_Acoustics.h"

namespace Sovereign {

    // --- GPU Orchestration (matrix_ops.cpp) ---
    bool InitGPUAcceleration();
    void GPUMatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims);

    // --- Transformer Engine (transformer_engine.cpp) ---
    class SovereignEngine {
    public:
        SovereignGGUF gguf;
        TransformerContext ctx;

        SovereignEngine();
        bool Initialize(const std::string& model_path);
        void Forward(int token_id, int pos);
        int Sample();
    };

} // namespace Sovereign
