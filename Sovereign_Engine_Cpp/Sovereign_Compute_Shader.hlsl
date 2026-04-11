// Sovereign Compute Shader v1.0
// Optimized for Q4_K Dequantization and MatMul

Buffer<float> hidden : register(t0);         // InDims
ByteAddressBuffer weights : register(t1);    // Quantized Weights (Blocks of 144 bytes)
RWBuffer<float> output : register(u0);       // OutDims

cbuffer Params : register(b0) {
    uint in_dims;
    uint out_dims;
    uint blocks_per_row;
}

// Fixed-point FP16 decoder (Simplified for HLSL)
float fp16_to_fp32(uint h) {
    uint sign = (h >> 15) & 1;
    uint exp = (h >> 10) & 0x1F;
    uint mant = h & 0x3FF;
    if (exp == 0) {
        if (mant == 0) return sign ? -0.0f : 0.0f;
        while ((mant & 0x400) == 0) { mant <<= 1; exp--; }
        exp++; mant &= ~0x400;
    } else if (exp == 31) {
        return 0.0f; // Simplified
    }
    exp = exp + (127 - 15);
    uint f = (sign << 31) | (exp << 23) | (mant << 13);
    return asfloat(f);
}

[numthreads(64, 1, 1)]
void main(uint3 DTid : SV_DispatchThreadID) {
    uint row = DTid.x;
    if (row >= out_dims) return;

    float sum = 0.0f;
    uint row_offset_bytes = row * blocks_per_row * 144;

    for (uint b = 0; b < blocks_per_row; ++b) {
        uint block_addr = row_offset_bytes + (b * 144);
        
        // Read Scale and Min (4 bytes combined as 2 FP16)
        uint scaling = weights.Load(block_addr);
        float d = fp16_to_fp32(scaling & 0xFFFF);
        float dmin = fp16_to_fp32(scaling >> 16);

        // Read 128 bytes of quantized labels (32 groups of 4 bytes)
        for (uint i = 0; i < 32; ++i) {
            uint qs_combined = weights.Load(block_addr + 16 + (i * 4));
            
            // Unpack 8 labels (4 low, 4 high)
            for (uint j = 0; j < 4; ++j) {
                uint byte_val = (qs_combined >> (j * 8)) & 0xFF;
                float v0 = (float)(byte_val & 0x0F) * d - dmin;
                float v1 = (float)(byte_val >> 4) * d - dmin;
                
                uint hidden_idx = (b * 256) + (i * 8) + (j * 2);
                sum += hidden[hidden_idx] * v0;
                sum += hidden[hidden_idx + 1] * v1;
            }
        }
    }

    output[row] = sum;
}
