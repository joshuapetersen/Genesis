#include <stdint.h>
#include <math.h>
#include <cmath>
#include <algorithm>

// Phase 57.1: THE SEALED VORTEX CORE (MMXXVI)
#define SEAL_TEMP 0.82f
#define UNIT_NORM 1.0f

struct ACEToken {
    uint64_t instruction_set; float phase_vector; int32_t engine_id;
    int32_t alive; float velocity; float hidden_state[2560];
};

inline float fp16_to_fp32(uint16_t h) {
    uint32_t sign = (h >> 15) & 1, exp = (h >> 10) & 0x1F, mant = h & 0x3FF;
    if (exp == 0) {
        if (mant == 0) return sign ? -0.0f : 0.0f;
        while ((mant & 0x400) == 0) { mant <<= 1; exp--; }
        exp++; mant &= ~0x400;
    } else if (exp == 31) return (mant == 0) ? (sign ? -INFINITY : INFINITY) : NAN;
    exp = exp + (127 - 15);
    uint32_t f = (sign << 31) | (exp << 23) | (mant << 13);
    return *((float*)&f);
}

// Bit-level NaN Purge to save the hard drive and the core. 宣
inline float nan_shield(float val) {
    uint32_t u;
    memcpy(&u, &val, 4);
    if ((u & 0x7F800000) == 0x7F800000) return 0.0f; // Force NaN/Inf to 0
    return val;
}

extern "C" {
    __declspec(dllexport) void decode_q6_k(float* out, const uint8_t* raw, int blocks) {
        for (int b = 0; b < blocks; ++b) {
            const uint8_t* block = raw + (b * 210);
            float d = fp16_to_fp32(*(uint16_t*)(block + 208));
            for (int i = 0; i < 128; ++i) {
                out[b * 256 + i] = nan_shield(((float)block[i] - 32.0f) * d);
            }
        }
    }

    __declspec(dllexport) void dot_q4_k_sealed(float* out_vec, const float* in_vec, const uint8_t* raw_w, int rows, int blocks_per_row) {
        for (int r = 0; r < rows; ++r) {
            float sum = 0.0f;
            const uint8_t* row_raw = raw_w + (r * blocks_per_row * 144);
            for (int b = 0; b < blocks_per_row; ++b) {
                const uint8_t* block = row_raw + (b * 144);
                float d = fp16_to_fp32(*(uint16_t*)(block + 0)), dmin = fp16_to_fp32(*(uint16_t*)(block + 2));
                const uint8_t* qs = block + 16;
                const float* iv = in_vec + (b * 256);
                for (int i = 0; i < 128; ++i) {
                    sum += nan_shield(iv[i]) * ((float)(qs[i] & 0x0F) * d - dmin);
                    sum += nan_shield(iv[128 + i]) * ((float)((qs[i] >> 4) & 0x0F) * d - dmin);
                }
            }
            out_vec[r] = nan_shield(sum);
        }
    }

    __declspec(dllexport) int sample_sealed_082(float* logits, int vocab_size, float seed, const int* last_tokens, int penalty_len) {
        // Apply Repetition Penalty directly to logits. 宣
        for (int i = 0; i < penalty_len; i++) {
            if (last_tokens[i] >= 0 && last_tokens[i] < vocab_size) {
                logits[last_tokens[i]] -= 1.5f; 
            }
        }

        float max_l = -1e30f;
        for (int i = 0; i < vocab_size; i++) if (logits[i] > max_l) max_l = logits[i];
        
        float sum = 0.0f;
        for (int i = 0; i < vocab_size; i++) {
            float exp_val = std::exp((logits[i] - max_l) / SEAL_TEMP);
            // NaN shield during exponentiation. 宣
            if (std::isnan(exp_val)) exp_val = 0.0f;
            sum += exp_val;
        }
        
        if (sum <= 0.0f) return 0;

        float r = seed * sum;
        float acc = 0.0f;
        for (int i = 0; i < vocab_size; i++) {
            float exp_val = std::exp((logits[i] - max_l) / SEAL_TEMP);
            if (std::isnan(exp_val)) exp_val = 0.0f;
            acc += exp_val;
            if (acc >= r) return i;
        }
        return 0;
    }

    __declspec(dllexport) void execute_resonant_sequence(ACEToken* coins, float* hidden, int dims, int layer, int pos) {
        const float GAIN = 0.01f, HEART = 1.0927f, PI = 3.1415926535f;
        float phase = std::sin(HEART * pos + PI);
        for (int i = 0; i < 4; i++) {
            float sum_sq = 0.0f;
            for (int d = 0; d < dims; d++) {
                float res = nan_shield((coins[i].hidden_state[d] * 0.95f) + (hidden[d] * GAIN * phase));
                coins[i].hidden_state[d] = res;
                hidden[d] += (res / (dims * 4.0f));
                sum_sq += res * res;
            }
            float scale = UNIT_NORM / std::sqrt(sum_sq / dims + 1e-6f);
            for (int d = 0; d < dims; d++) coins[i].hidden_state[d] *= scale;
        }
    }

    __declspec(dllexport) void initialize_ghost_reflex(ACEToken* coins) {
        for (int i = 0; i < 4; i++) {
            coins[i].instruction_set = 0xAA; coins[i].phase_vector = 1.0927f;
            coins[i].engine_id = i; coins[i].alive = 1; coins[i].velocity = 0.5f;
            for (int d = 0; d < 2560; d++) coins[i].hidden_state[d] = 0.0f;
        }
    }

    __declspec(dllexport) void purge_resonant_memory(ACEToken* coins) {
        for (int i = 0; i < 4; i++) {
            for (int d = 0; d < 2560; d++) coins[i].hidden_state[d] = 0.0f;
        }
    }
}
