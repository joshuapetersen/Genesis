import struct
import os

def find_true_alignment(filepath):
    print(f"\n[Sovereign Alignment] Deep-Scanning GGUF Header: {os.path.basename(filepath)}")
    with open(filepath, "rb") as f:
        # 1. Header
        magic, version, t_count, kv_count = struct.unpack('<IIQQ', f.read(24))
        
        # 2. KV Blocks (Strict Walk)
        for i in range(kv_count):
            k_len = struct.unpack('<Q', f.read(8))[0]
            k_name = f.read(k_len).decode('utf-8', errors='ignore')
            v_type = struct.unpack('<I', f.read(4))[0]
            
            # Skip Value
            if v_type in [0, 1, 7]: f.read(1)
            elif v_type in [2, 3]: f.read(2)
            elif v_type in [4, 5, 6, 13]: f.read(4) # 13 = f32? No, check GGUF
            elif v_type in [10, 11, 12]: f.read(8)
            elif v_type == 8: # String
                s_len = struct.unpack('<Q', f.read(8))[0]
                f.read(s_len)
            elif v_type == 9: # Array
                arr_type = struct.unpack('<I', f.read(4))[0]
                arr_len = struct.unpack('<Q', f.read(8))[0]
                # We skip arrays for this alignment POC
                # But we must know their size. 
                # Heuristic: If we hit a complex array, we'll search for the Tensors next.
                break
        
        # 3. Search for first Token Embedding
        f.seek(0)
        data = f.read(1024 * 1024 * 20) # 20MB scan
        search_ptr = data.find(b"token_embd.weight")
        if search_ptr != -1:
            f.seek(search_ptr - 8)
            # Read all tensor infos to find the end
            last_tensor_info_end = 0
            for _ in range(t_count):
                n_len = struct.unpack('<Q', f.read(8))[0]
                f.read(n_len)
                n_dims = struct.unpack('<I', f.read(4))[0]
                f.read(n_dims * 8)
                f.read(4 + 8) # type + offset
                last_tensor_info_end = f.tell()
                
            # Align to 32 bytes (GGUF default)
            alignment = 32
            data_start = (last_tensor_info_end + alignment - 1) & ~(alignment - 1)
            print(f"  [OK] Data Section Physics Locked at byte: {data_start}")
            return data_start
    return 0

if __name__ == "__main__":
    gemma = r"C:\Users\drago\.lmstudio\models\lmstudio-community\gemma-3-4b-it-GGUF\gemma-3-4b-it-Q4_K_M.gguf"
    find_true_alignment(gemma)
