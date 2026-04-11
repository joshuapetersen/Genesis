#include "Sovereign_Engine_Core.h"
#include <iostream>
#include <vector>
#include <cmath>

namespace Sovereign {

    // Forward declarations from matrix_ops.cpp
    void RMSNorm(float* out, float* in, float* weight, int dims, float eps = 1e-6f);
    void MatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims);
    void GPUMatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims);
    void RoPE(float* hidden, int dims, int pos, float base = 10000.0f);

    SovereignEngine::SovereignEngine() : ctx(2560, 42) {}

    bool SovereignEngine::Initialize(const std::string& model_path) {
        if (!gguf.LoadFile(model_path)) return false;
        return true;
    }

    void SovereignEngine::Forward(int token_id, int pos) {
        float* hidden = ctx.hidden_state.data();
        
        // 1. Embedding Lookup
        TensorInfo* embd = gguf.GetTensor("token_embd.weight");
        if (!embd) { std::cerr << "[CRITICAL] token_embd.weight not found!\n"; return; }

        float* embd_data = static_cast<float*>(embd->data);
        float* embd_ptr = embd_data + (token_id * ctx.dims);
        for (int i = 0; i < ctx.dims; ++i) hidden[i] = embd_ptr[i];

        std::vector<float> normed_hidden(ctx.dims);

        // 2. Transformer Layers (42 Loops)
        for (int l = 0; l < ctx.layers; ++l) {
            std::string prefix = "blk." + std::to_string(l);
            
            // --- Attention Block ---
            TensorInfo* attn_norm = gguf.GetTensor(prefix + ".attn_norm.weight");
            if (!attn_norm) { std::cerr << "[CRITICAL] " << prefix << ".attn_norm.weight not found!\n"; return; }
            RMSNorm(normed_hidden.data(), hidden, (float*)attn_norm->data, ctx.dims);

            TensorInfo* q_w = gguf.GetTensor(prefix + ".attn_q.weight");
            TensorInfo* k_w = gguf.GetTensor(prefix + ".attn_k.weight");
            TensorInfo* v_w = gguf.GetTensor(prefix + ".attn_v.weight");
            if (!q_w || !k_w || !v_w) { std::cerr << "[CRITICAL] Attn weights missing for " << prefix << "\n"; return; }

            int q_dim = (int)q_w->dims[0];
            int k_dim = (int)k_w->dims[0];
            int v_dim = (int)v_w->dims[0];

            std::vector<float> q(q_dim), k(k_dim), v(v_dim);
            GPUMatMul(q.data(), normed_hidden.data(), q_w, q_dim, ctx.dims);
            GPUMatMul(k.data(), normed_hidden.data(), k_w, k_dim, ctx.dims);
            GPUMatMul(v.data(), normed_hidden.data(), v_w, v_dim, ctx.dims);

            // RoPE Positional Encoding (Operating on head dimension 256 usually)
            for (int h = 0; h < 8; ++h) RoPE(q.data() + h * 256, 256, pos);
            for (int h = 0; h < 2; ++h) RoPE(k.data() + h * 256, 256, pos);

            // Output Projection
            std::vector<float> attn_out(ctx.dims);
            TensorInfo* proj_w = gguf.GetTensor(prefix + ".attn_output.weight");
            if (!proj_w) { std::cerr << "[CRITICAL] " << prefix << ".attn_output.weight not found!\n"; return; }
            GPUMatMul(attn_out.data(), q.data(), proj_w, ctx.dims, q_dim); 

            // Residual connection (Attention)
            for (int i = 0; i < ctx.dims; ++i) hidden[i] += attn_out[i];

            // --- FFN Block ---
            TensorInfo* ffn_norm = gguf.GetTensor(prefix + ".ffn_norm.weight");
            if (!ffn_norm) { std::cerr << "[CRITICAL] " << prefix << ".ffn_norm.weight not found!\n"; return; }
            RMSNorm(normed_hidden.data(), hidden, (float*)ffn_norm->data, ctx.dims);

            TensorInfo* ffn_gate_w = gguf.GetTensor(prefix + ".ffn_gate.weight");
            TensorInfo* ffn_up_w = gguf.GetTensor(prefix + ".ffn_up.weight");
            if (!ffn_gate_w || !ffn_up_w) { std::cerr << "[CRITICAL] FFN Gate/Up missing for " << prefix << "\n"; return; }

            std::vector<float> gate(10240), up(10240);
            GPUMatMul(gate.data(), normed_hidden.data(), ffn_gate_w, 10240, ctx.dims);
            GPUMatMul(up.data(), normed_hidden.data(), ffn_up_w, 10240, ctx.dims);

            // SiLU Activation
            for (int i = 0; i < 10240; ++i) {
                float g = gate[i];
                gate[i] = g * (1.0f / (1.0f + expf(-g))) * up[i];
            }

            std::vector<float> down(ctx.dims);
            TensorInfo* ffn_down_w = gguf.GetTensor(prefix + ".ffn_down.weight");
            if (!ffn_down_w) { std::cerr << "[CRITICAL] FFN Down missing for " << prefix << "\n"; return; }
            GPUMatMul(down.data(), gate.data(), ffn_down_w, ctx.dims, 10240);
            
            // Residual connection (FFN)
            for (int i = 0; i < ctx.dims; ++i) hidden[i] += down[i];
        }

        // 3. Final Norm
        TensorInfo* out_norm_w = gguf.GetTensor("output_norm.weight");
        if (out_norm_w) {
            RMSNorm(hidden, hidden, (float*)out_norm_w->data, ctx.dims);
        }
    }

    int SovereignEngine::Sample() {
        // Final Logit Projection (GPU Accelerated)
        TensorInfo* output_w = gguf.GetTensor("output.weight");
        if (!output_w) {
            output_w = gguf.GetTensor("token_embd.weight");
        }
        
        if (!output_w) { std::cerr << "[CRITICAL] No output or embedding weight found for sampling!\n"; return 0; }
        
        std::vector<float> logits(256000); // Gemma-2 Vocab
        GPUMatMul(logits.data(), ctx.hidden_state.data(), output_w, 256000, ctx.dims);

        // Simple ArgMax for baseline verification
        int best_id = 0;
        float best_val = -1e9;
        for (int i = 0; i < (int)logits.size(); ++i) {
            if (logits[i] > best_val) {
                best_val = logits[i];
                best_id = i;
            }
        }
        return best_id;
    }

} // namespace Sovereign
