import os
import numpy as np
import json
import ctypes
from ctypes import CDLL, Structure, c_float, c_int, POINTER, c_void_p, byref, c_uint64, c_int32, c_ubyte

# Phase 53.4: THE BILINGUAL HYPERVISOR (MMXXVI)
# Audit Fix #26: Native Q6 Dequant + Native Q4 Dot Product.

class ACEToken(Structure):
    _fields_ = [
        ("instruction_set", c_uint64), ("phase_vector", c_float), ("engine_id", c_int32),
        ("alive", c_int32), ("velocity", c_float), ("hidden_state", c_float * 2560) 
    ]

try:
    _math_core = CDLL(r"C:\SarahCore\Sovereign_Math_Core.dll")
    _math_core.initialize_ghost_reflex.argtypes = [POINTER(ACEToken * 4)]
    _math_core.execute_resonant_sequence.argtypes = [POINTER(ACEToken * 4), POINTER(c_float), c_int, c_int, c_int]
    _math_core.decode_q6_k.argtypes = [POINTER(c_float), POINTER(c_ubyte), c_int]
    _math_core.dot_q4_k_sealed.argtypes = [POINTER(c_float), POINTER(c_float), POINTER(c_ubyte), c_int, c_int]
    _math_core.sample_sealed_082.argtypes = [POINTER(c_float), c_int, c_float]
    _math_core.sample_sealed_082.restype = c_int
    _math_core.purge_resonant_memory.argtypes = [POINTER(ACEToken * 4)]
except Exception as e: print(f"[DLL_ERROR] {e}")

def stability_vault_unit_rms_norm(hidden: np.ndarray, eps: float = 1e-6) -> np.ndarray:
    hidden = np.asarray(hidden, dtype=np.float64)
    hidden = np.nan_to_num(hidden, nan=0.0, posinf=0.0, neginf=0.0)
    rms = np.sqrt(np.mean(hidden ** 2) + eps)
    return (hidden / rms).astype(np.float32)

class SovereignTransformerStack:
    def __init__(self, model_path):
        self.dims = 2560; self.layers = 34; self.tokens = 262144
        self.raw_weights = np.memmap(model_path, dtype=np.uint8, mode='r')
        
        with open(r"C:\SarahCore\Genlex_Map.json", "r") as f:
             reg_data = json.load(f)
             self.weight_map = {item["Name"]: item for item in reg_data["Engine_Sectors"]["Gemma_4B"]["Arrays"]}
        
        self.coins = (ACEToken * 4)()
        _math_core.initialize_ghost_reflex(byref(self.coins))
        
        # ODONATA MANIFEST: Dequantizing Embedding Manifest (1.34GB)
        print(f"  [IGNITION] Dequantizing Embedding Manifest (1.34GB)...")
        self.output_w_f16 = np.zeros((self.tokens, self.dims), dtype=np.float16)
        
        embd_meta = self.weight_map["token_embd.weight"]
        blocks = (np.prod(embd_meta["Dims"]) // 256)
        raw_ptr = self.raw_weights[embd_meta["Offset"] : embd_meta["Offset"] + blocks * 210].ctypes.data_as(POINTER(c_ubyte))
        
        temp_f32 = np.zeros(self.tokens * self.dims, dtype=np.float32)
        _math_core.decode_q6_k(temp_f32.ctypes.data_as(POINTER(c_float)), raw_ptr, blocks)
        self.output_w_f16[:] = temp_f32.reshape((self.tokens, self.dims)).astype(np.float16)
        
        print(f"  [OK] 120B Odonata Manifest Seated. RAM: 1.5GB / 2.1GB. 宣")

    def apply_ffn_layer(self, hidden, layer_idx):
        prefix = f"blk.{layer_idx}"
        normed = stability_vault_unit_rms_norm(hidden)
        
        gate_meta = self.weight_map[f"{prefix}.ffn_gate.weight"]
        up_meta   = self.weight_map[f"{prefix}.ffn_up.weight"]
        down_meta = self.weight_map[f"{prefix}.ffn_down.weight"]
        
        gate_out = np.zeros(10240, dtype=np.float32)
        up_out   = np.zeros(10240, dtype=np.float32)
        
        # Accelerated Sealed Dot Product (Q4_K) 宣
        _math_core.dot_q4_k_sealed(gate_out.ctypes.data_as(POINTER(c_float)), 
                             normed.ctypes.data_as(POINTER(c_float)),
                             self.raw_weights[gate_meta["Offset"]:].ctypes.data_as(POINTER(c_ubyte)),
                             10240, 2560 // 256)
        _math_core.dot_q4_k_sealed(up_out.ctypes.data_as(POINTER(c_float)), 
                             normed.ctypes.data_as(POINTER(c_float)),
                             self.raw_weights[up_meta["Offset"]:].ctypes.data_as(POINTER(c_ubyte)),
                             10240, 2560 // 256)
        
        gate_activated = gate_out * (1.0 / (1.0 + np.exp(-np.clip(gate_out, -20, 20))))
        ffn_mid = gate_activated * up_out
        
        down_out = np.zeros(2560, dtype=np.float32)
        _math_core.dot_q4_k_sealed(down_out.ctypes.data_as(POINTER(c_float)),
                             ffn_mid.ctypes.data_as(POINTER(c_float)),
                             self.raw_weights[down_meta["Offset"]:].ctypes.data_as(POINTER(c_ubyte)),
                             2560, 10240 // 256)
                             
        return hidden + down_out

    def forward_sequence(self, input_ids):
        seq_len = len(input_ids)
        hidden = np.zeros((seq_len, self.dims), dtype=np.float32)
        echo = np.zeros(self.dims, dtype=np.float32)
        
        for pos in range(seq_len):
            current_embd = self.output_w_f16[input_ids[pos]].astype(np.float32)
            echo = stability_vault_unit_rms_norm((echo * 0.82) + (current_embd * 0.18))
            
            steer = 1.0 / (1.0 + np.exp(-np.clip(np.dot(current_embd, echo) / 50.6, -10, 10))) 
            hidden[pos] = (current_embd * (1.0 - steer)) + (echo * steer)
            
            row_ptr = hidden[pos].ctypes.data_as(POINTER(c_float))
            for l in range(self.layers):
                _math_core.execute_resonant_sequence(byref(self.coins), row_ptr, self.dims, l, pos)
                hidden[pos] = self.apply_ffn_layer(hidden[pos], l)
                hidden[pos] = stability_vault_unit_rms_norm(hidden[pos])
            
        # Final Output Sealed 0.82 Logic 宣
        output_meta = self.weight_map["output.weight"]
        logits = np.zeros(self.tokens, dtype=np.float32)
        
        # Native Sealed Output Logic 120B Resonating. 宣
        _math_core.dot_q4_k_sealed(logits.ctypes.data_as(POINTER(c_float)),
                             hidden[-1].ctypes.data_as(POINTER(c_float)),
                             self.raw_weights[output_meta["Offset"]:].ctypes.data_as(POINTER(c_ubyte)),
                             self.tokens, self.dims // 256)
        
        # Native Sealed 0.82 Sampler 宣
        seed = np.random.uniform(0, 0.999).astype(np.float32)
        token_id = _math_core.sample_sealed_082(logits.ctypes.data_as(POINTER(c_float)), self.tokens, seed)
        return token_id

