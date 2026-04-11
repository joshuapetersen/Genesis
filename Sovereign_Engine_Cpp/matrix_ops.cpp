#include "Sovereign_Tensor_Core.h"
#include <d3d11.h>
#include <d3dcompiler.h>
#include <cmath>
#include <iostream>
#include <vector>
#include <algorithm>

#pragma comment(lib, "d3d11.lib")
#pragma comment(lib, "d3dcompiler.lib")

extern ID3D11Device* g_pd3dDevice;
extern ID3D11DeviceContext* g_pd3dDeviceContext;

namespace Sovereign {

    // Forward declarations
    struct TensorInfo;
    void MatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims);
    void RoPE(float* hidden, int dims, int pos, float base);

    // --- GPU Math Orchestrator ---
    ID3D11ComputeShader* g_computeShader = nullptr;
    ID3D11Buffer* g_cbParams = nullptr;

    struct ShaderParams {
        uint32_t in_dims;
        uint32_t out_dims;
        uint32_t blocks_per_row;
        uint32_t padding;
    };

    struct GPUMemory {
        ID3D11Buffer* buffer = nullptr;
        ID3D11ShaderResourceView* srv = nullptr;
        ID3D11UnorderedAccessView* uav = nullptr;
    };

    bool InitGPUAcceleration() {
        if (!g_pd3dDevice) return false;

        ID3DBlob* shaderBlob = nullptr;
        ID3DBlob* errorBlob = nullptr;
        
        HRESULT hr = D3DCompileFromFile(L"c:\\GENESIS\\Sovereign_Engine_Cpp\\Sovereign_Compute_Shader.hlsl", 
                                        NULL, NULL, "main", "cs_5_0", 
                                        D3DCOMPILE_ENABLE_STRICTNESS, 0, &shaderBlob, &errorBlob);
        
        if (FAILED(hr)) {
            if (errorBlob) {
                std::cerr << "Shader Error: " << (char*)errorBlob->GetBufferPointer() << std::endl;
                errorBlob->Release();
            }
            return false;
        }

        hr = g_pd3dDevice->CreateComputeShader(shaderBlob->GetBufferPointer(), shaderBlob->GetBufferSize(), NULL, &g_computeShader);
        shaderBlob->Release();
        if (FAILED(hr)) return false;

        D3D11_BUFFER_DESC cbDesc = {};
        cbDesc.Usage = D3D11_USAGE_DYNAMIC;
        cbDesc.ByteWidth = sizeof(ShaderParams);
        cbDesc.BindFlags = D3D11_BIND_CONSTANT_BUFFER;
        cbDesc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;
        g_pd3dDevice->CreateBuffer(&cbDesc, NULL, &g_cbParams);

        std::cout << "[SUCCESS] Sovereign GPU Acceleration (RTX 4050) Activated." << std::endl;
        return true;
    }

    std::map<void*, GPUMemory> g_weightCache;
    ID3D11Buffer* g_hiddenBuffer = nullptr;
    ID3D11ShaderResourceView* g_hiddenSRV = nullptr;
    ID3D11Buffer* g_outputBuffer = nullptr;
    ID3D11UnorderedAccessView* g_outputUAV = nullptr;

    void GPUMatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims) {
        if (!g_computeShader || !g_pd3dDevice) {
            MatMul(out, hidden, weight, out_dims, in_dims);
            return;
        }

        // 1. Prepare Weight Buffer (Cached)
        if (g_weightCache.find(weight->data) == g_weightCache.end()) {
            GPUMemory mem;
            D3D11_BUFFER_DESC desc = {};
            desc.ByteWidth = (UINT)weight->offset; // Using offset as a proxy for size if size unavailable, wait.
            // Actually, we need the size. Let's calculate based on type.
            size_t size = 0;
            if (weight->type == TensorType::Q4_K) size = (out_dims * in_dims / 256) * 144;
            else if (weight->type == TensorType::F32) size = out_dims * in_dims * 4;
            
            desc.ByteWidth = (UINT)size;
            desc.Usage = D3D11_USAGE_IMMUTABLE;
            desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
            desc.MiscFlags = D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS;

            D3D11_SUBRESOURCE_DATA data = { weight->data, 0, 0 };
            HRESULT hr = g_pd3dDevice->CreateBuffer(&desc, &data, &mem.buffer);
            if (SUCCEEDED(hr)) {
                D3D11_SHADER_RESOURCE_VIEW_DESC srvDesc = {};
                srvDesc.ViewDimension = D3D11_SRV_DIMENSION_BUFFEREX;
                srvDesc.BufferEx.Flags = D3D11_BUFFEREX_SRV_FLAG_RAW;
                srvDesc.Format = DXGI_FORMAT_R32_TYPELESS;
                srvDesc.BufferEx.NumElements = (UINT)size / 4;
                g_pd3dDevice->CreateShaderResourceView(mem.buffer, &srvDesc, &mem.srv);
            }
            g_weightCache[weight->data] = mem;
        }

        GPUMemory& wMem = g_weightCache[weight->data];

        // 2. Prepare Hidden State Buffer (Dynamic)
        if (!g_hiddenBuffer || in_dims > (int)2560) {
            if (g_hiddenBuffer) { g_hiddenSRV->Release(); g_hiddenBuffer->Release(); }
            D3D11_BUFFER_DESC desc = {};
            desc.ByteWidth = 2560 * 4;
            desc.Usage = D3D11_USAGE_DYNAMIC;
            desc.BindFlags = D3D11_BIND_SHADER_RESOURCE;
            desc.CPUAccessFlags = D3D11_CPU_ACCESS_WRITE;
            g_pd3dDevice->CreateBuffer(&desc, NULL, &g_hiddenBuffer);
            
            D3D11_SHADER_RESOURCE_VIEW_DESC srvDesc = {};
            srvDesc.Format = DXGI_FORMAT_R32_FLOAT;
            srvDesc.ViewDimension = D3D11_SRV_DIMENSION_BUFFER;
            srvDesc.Buffer.NumElements = 2560;
            g_pd3dDevice->CreateShaderResourceView(g_hiddenBuffer, &srvDesc, &g_hiddenSRV);
        }

        D3D11_MAPPED_SUBRESOURCE mapped;
        g_pd3dDeviceContext->Map(g_hiddenBuffer, 0, D3D11_MAP_WRITE_DISCARD, 0, &mapped);
        memcpy(mapped.pData, hidden, in_dims * 4);
        g_pd3dDeviceContext->Unmap(g_hiddenBuffer, 0);

        // 3. Prepare Output Buffer
        if (!g_outputBuffer || out_dims > (int)262144) {
            if (g_outputBuffer) { g_outputUAV->Release(); g_outputBuffer->Release(); }
            D3D11_BUFFER_DESC desc = {};
            desc.ByteWidth = (out_dims > 262144 ? out_dims : 262144) * 4;
            desc.Usage = D3D11_USAGE_DEFAULT;
            desc.BindFlags = D3D11_BIND_UNORDERED_ACCESS;
            g_pd3dDevice->CreateBuffer(&desc, NULL, &g_outputBuffer);

            D3D11_UNORDERED_ACCESS_VIEW_DESC uavDesc = {};
            uavDesc.Format = DXGI_FORMAT_R32_FLOAT;
            uavDesc.ViewDimension = D3D11_UAV_DIMENSION_BUFFER;
            uavDesc.Buffer.NumElements = (out_dims > 262144 ? out_dims : 262144);
            g_pd3dDevice->CreateUnorderedAccessView(g_outputBuffer, &uavDesc, &g_outputUAV);
        }

        // 4. Update Constants
        D3D11_MAPPED_SUBRESOURCE cbMapped;
        g_pd3dDeviceContext->Map(g_cbParams, 0, D3D11_MAP_WRITE_DISCARD, 0, &cbMapped);
        ShaderParams* p = (ShaderParams*)cbMapped.pData;
        p->in_dims = in_dims;
        p->out_dims = out_dims;
        p->blocks_per_row = in_dims / 256;
        g_pd3dDeviceContext->Unmap(g_cbParams, 0);

        // 5. Dispatch
        ID3D11Buffer* cbs[] = { g_cbParams };
        g_pd3dDeviceContext->CSSetConstantBuffers(0, 1, cbs);
        ID3D11ShaderResourceView* srvs[] = { g_hiddenSRV, wMem.srv };
        g_pd3dDeviceContext->CSSetShaderResources(0, 2, srvs);
        ID3D11UnorderedAccessView* uavs[] = { g_outputUAV };
        g_pd3dDeviceContext->CSSetUnorderedAccessViews(0, 1, uavs, NULL);
        g_pd3dDeviceContext->CSSetShader(g_computeShader, NULL, 0);

        g_pd3dDeviceContext->Dispatch((out_dims + 63) / 64, 1, 1);

        // 6. Read Back
        // In a real high-perf engine, we'd shadow the output buffer with a Staging buffer.
        static ID3D11Buffer* staging = nullptr;
        if (!staging) {
            D3D11_BUFFER_DESC stagingDesc = {};
            stagingDesc.ByteWidth = 262144 * 4;
            stagingDesc.Usage = D3D11_USAGE_STAGING;
            stagingDesc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
            g_pd3dDevice->CreateBuffer(&stagingDesc, NULL, &staging);
        }
        g_pd3dDeviceContext->CopyResource(staging, g_outputBuffer);
        g_pd3dDeviceContext->Map(staging, 0, D3D11_MAP_READ, 0, &mapped);
        memcpy(out, mapped.pData, out_dims * 4);
        g_pd3dDeviceContext->Unmap(staging, 0);

        // Cleanup bindings
        ID3D11ShaderResourceView* nullSRVs[] = { nullptr, nullptr };
        g_pd3dDeviceContext->CSSetShaderResources(0, 2, nullSRVs);
        ID3D11UnorderedAccessView* nullUAVs[] = { nullptr };
        g_pd3dDeviceContext->CSSetUnorderedAccessViews(0, 1, nullUAVs, NULL);
    }

    // --- Math Utilities ---
    
    inline float fp16_to_fp32(uint16_t h) {
        uint32_t sign = (h >> 15) & 1;
        uint32_t exp = (h >> 10) & 0x1F;
        uint32_t mant = h & 0x3FF;
        if (exp == 0) {
            if (mant == 0) return sign ? -0.0f : 0.0f;
            while ((mant & 0x400) == 0) { mant <<= 1; exp--; }
            exp++; mant &= ~0x400;
        } else if (exp == 31) {
            if (mant == 0) return sign ? -INFINITY : INFINITY;
            return NAN;
        }
        exp = exp + (127 - 15);
        uint32_t f = (sign << 31) | (exp << 23) | (mant << 13);
        return *((float*)&f);
    }

    void RMSNorm(float* out, float* in, float* weight, int dims, float eps = 1e-6f) {
        float sum = 0.0f;
        for (int i = 0; i < dims; ++i) sum += in[i] * in[i];
        float rms = 1.0f / sqrtf(sum / dims + eps);
        for (int i = 0; i < dims; ++i) out[i] = in[i] * rms * weight[i];
    }

    void Dequantize_Q4_K(float* out, const uint8_t* raw, int blocks) {
        for (int b = 0; b < blocks; ++b) {
            const uint8_t* block = raw + (b * 144);
            float d = fp16_to_fp32(*(uint16_t*)(block + 0));
            float dmin = fp16_to_fp32(*(uint16_t*)(block + 2));
            const uint8_t* qs = block + 16;
            for (int i = 0; i < 128; ++i) {
                out[b * 256 + i] = (float)(qs[i] & 0x0F) * d - dmin;
                out[b * 256 + 128 + i] = (float)((qs[i] >> 4) & 0x0F) * d - dmin;
            }
        }
    }

    void Dequantize_Q6_K(float* out, const uint8_t* raw, int blocks) {
        for (int b = 0; b < blocks; ++b) {
            const uint8_t* block = raw + (b * 210);
            float d = fp16_to_fp32(*(uint16_t*)(block + 208));
            for (int i = 0; i < 128; ++i) {
                // Simplified Q6_K dequant
                out[b * 256 + i] = ((float)block[i] - 32.0f) * d;
                out[b * 256 + 128 + i] = ((float)block[i + 128] - 32.0f) * d;
            }
        }
    }

    // --- MatMul Ops ---

    void MatMul(float* out, float* hidden, TensorInfo* weight, int out_dims, int in_dims) {
        // CPU baseline dot product
        // Note: For 34 layers, this is what DX11 will replace.
        if (weight->type == TensorType::Q4_K) {
            int blocks = (out_dims * in_dims) / 256;
            // Temporary strategy: Dequantize then Dot (slow but safe for baseline)
            static std::vector<float> dequant_buffer;
            if (dequant_buffer.size() < out_dims * in_dims) dequant_buffer.resize(out_dims * in_dims);
            
            Dequantize_Q4_K(dequant_buffer.data(), (uint8_t*)weight->data, blocks);

            for (int i = 0; i < out_dims; ++i) {
                float sum = 0.0f;
                float* w_row = dequant_buffer.data() + (i * in_dims);
                for (int j = 0; j < in_dims; ++j) {
                    sum += hidden[j] * w_row[j];
                }
                out[i] = sum;
            }
        } else if (weight->type == TensorType::F32) {
            float* w_ptr = (float*)weight->data;
            for (int i = 0; i < out_dims; ++i) {
                float sum = 0.0f;
                for (int j = 0; j < in_dims; ++j) {
                    sum += hidden[j] * w_ptr[i * in_dims + j];
                }
                out[i] = sum;
            }
        }
    }

    void RoPE(float* hidden, int dims, int pos, float base = 10000.0f) {
        for (int i = 0; i < dims; i += 2) {
            float freq = 1.0f / powf(base, (float)i / (float)dims);
            float angle = pos * freq;
            float cos_a = cosf(angle);
            float sin_a = sinf(angle);
            float v0 = hidden[i];
            float v1 = hidden[i + 1];
            hidden[i] = v0 * cos_a - v1 * sin_a;
            hidden[i + 1] = v0 * sin_a + v1 * cos_a;
        }
    }

} // namespace Sovereign
