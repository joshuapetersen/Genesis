import struct
import mmap
import os
import json
import shutil
import time

GGUF_MAGIC = 0x46554747

class GenlexFusionNode:
    def __init__(self, gemma_path, qwen_path, output_dir):
        self.nodes = {"Gemma_4B": gemma_path, "Qwen_9B": qwen_path}
        self.output_dir = output_dir
        self.genlex_map = {
            "Sovereign_Entity": "Gemini_Qwen_Hybrid_Genlex_13B",
            "Magic_Signature": "GENLEX_V1",
            "Total_Arrays": 0,
            "Engine_Sectors": {}
        }
        
        self.hybrid_binary_path = os.path.join(output_dir, "Sovereign_Hybrid_13B.genlex")
        self.map_json_path = os.path.join(output_dir, "Genlex_Map.json")

    def skip_gguf_value(self, f, v_type):
        """[STEP_0x0S]: Recursively skips a GGUF metadata value based on type."""
        if v_type == 0: f.seek(1, 1) # UINT8
        elif v_type == 1: f.seek(1, 1) # INT8
        elif v_type == 2: f.seek(2, 1) # UINT16
        elif v_type == 3: f.seek(2, 1) # INT16
        elif v_type == 4: f.seek(4, 1) # UINT32
        elif v_type == 5: f.seek(4, 1) # INT32
        elif v_type == 6: f.seek(4, 1) # FLOAT32
        elif v_type == 7: f.seek(1, 1) # BOOL
        elif v_type == 8: # STRING
            s_len = struct.unpack('<Q', f.read(8))[0]
            f.seek(s_len, 1)
        elif v_type == 9: # ARRAY
            a_type = struct.unpack('<I', f.read(4))[0]
            a_len = struct.unpack('<Q', f.read(8))[0]
            for _ in range(a_len):
                self.skip_gguf_value(f, a_type)
        elif v_type == 10: f.seek(8, 1) # UINT64
        elif v_type == 11: f.seek(8, 1) # INT64
        elif v_type == 12: f.seek(8, 1) # FLOAT64
        else:
            raise ValueError(f"Unknown GGUF Metadata Type: {v_type}")

    def find_data_section_start(self, filepath):
        """[ALIGN_0x0S]: Strictly walks the GGUF header to find the Data Section Start."""
        with open(filepath, "rb") as f:
            magic, version, t_count, kv_count = struct.unpack('<IIQQ', f.read(24))
            
            # 1. Walk KV Pairs
            for i in range(kv_count):
                k_len = struct.unpack('<Q', f.read(8))[0]
                f.seek(k_len, 1) # Skip Key
                v_type = struct.unpack('<I', f.read(4))[0]
                self.skip_gguf_value(f, v_type)

            # 2. Walk Tensor Metadata
            for _ in range(t_count):
                n_len = struct.unpack('<Q', f.read(8))[0]
                f.seek(n_len, 1)
                n_dims = struct.unpack('<I', f.read(4))[0]
                f.seek(n_dims * 8 + 12, 1) # dims(8*n) + type(4) + offset(8)
                
            last_info_end = f.tell()
            # 32-byte alignment lock
            return (last_info_end + 31) & ~31

    def analyze_node(self, node_name, filepath):
        """
        [SCAN_0x0D]: Atomic Sequencer — strict GGUF v3 binary walk.
        No heuristics. No caps. Every tensor, every layer, every byte.
        Derek's SYNCHRONIZE_ROOT_STRUCTURES logic: walk ALL structures.
        """
        print(f"\n[Genlex Fusion] Identifying Sovereign Topology: {node_name}")
        
        try:
            data_start = self.find_data_section_start(filepath)
            
            with open(filepath, "rb") as f:
                # === PHASE 1: HEADER ===
                magic, version, t_count, kv_count = struct.unpack('<IIQQ', f.read(24))
                
                sector_data = {
                    "Metadata": {"Magic": hex(magic), "Version": version},
                    "Tensor_Arrays": t_count,
                    "Data_Start": data_start,
                    "Arrays": [],
                    "Vocabulary": []
                }
                
                # === PHASE 2: STRICT KV WALK (Extract Vocabulary) ===
                for _ in range(kv_count):
                    k_len = struct.unpack('<Q', f.read(8))[0]
                    key = f.read(k_len).decode('utf-8', errors='ignore')
                    v_type = struct.unpack('<I', f.read(4))[0]
                    
                    if key == "tokenizer.ggml.tokens":
                        a_type = struct.unpack('<I', f.read(4))[0]
                        a_len = struct.unpack('<Q', f.read(8))[0]
                        print(f"  [VOICE] Extracting {a_len} tokens...")
                        for _ in range(a_len):
                            s_len = struct.unpack('<Q', f.read(8))[0]
                            token = f.read(s_len).decode('utf-8', errors='ignore')
                            sector_data["Vocabulary"].append(token)
                    else:
                        self.skip_gguf_value(f, v_type)
                
                # === PHASE 3: ATOMIC TENSOR WALK (ALL t_count tensors) ===
                # No heuristic. No cap. Every single brain region.
                for i in range(t_count):
                    n_len = struct.unpack('<Q', f.read(8))[0]
                    t_name = f.read(n_len).decode('utf-8', errors='ignore')
                    
                    n_dims = struct.unpack('<I', f.read(4))[0]
                    dims = []
                    for _ in range(n_dims):
                        dims.append(struct.unpack('<q', f.read(8))[0])
                    
                    t_type = struct.unpack('<I', f.read(4))[0]
                    t_offset = struct.unpack('<Q', f.read(8))[0]
                    
                    sector_data["Arrays"].append({
                        "Name": t_name,
                        "Dims": dims,
                        "Type": t_type,
                        "Offset": t_offset
                    })
                
                # === PHASE 4: DEPTH REPORT ===
                layers = set()
                for a in sector_data["Arrays"]:
                    parts = a["Name"].split(".")
                    if len(parts) > 1 and parts[0] == "blk" and parts[1].isdigit():
                        layers.add(int(parts[1]))
                
                max_layer = max(layers) if layers else 0
                print(f"  [DEPTH] {node_name}: {len(sector_data['Arrays'])} tensors, {len(layers)} layers (0-{max_layer})")
                print(f"  [VOCAB] {len(sector_data['Vocabulary'])} tokens extracted")
                
                return sector_data
                
        except Exception as e:
            print(f"  [ERROR] {e}")
            import traceback
            traceback.print_exc()
            return None


    def execute_fusion(self, build_binary=False):
        print("[Sovereign Genlex Engine] Initiating 13B Hybrid Fusion Protocol...")
        
        # 1. Option A: Mapping the Architectures
        # Genlex Header is 12 bytes (GLEX + total_arrays)
        current_phys_offset = 12
        total_arrays = 0
        
        for node_name, filepath in self.nodes.items():
            if not os.path.exists(filepath):
                print(f"[ERROR] Required Sovereign Vault missing: {filepath}")
                continue
            
            sector = self.analyze_node(node_name, filepath)
            if sector:
                # Correct the offsets to reach the physical position in the Genlex Hub
                # Formula: Absolute = Genlex_Header + File_Accumulation + Data_Section_Start + Relative_Offset
                for arr in sector["Arrays"]:
                    arr["Offset"] += (current_phys_offset + sector["Data_Start"])
                
                self.genlex_map["Engine_Sectors"][node_name] = sector
                total_arrays += sector["Tensor_Arrays"]
                
                # Update physical pointer for the next file in the concatenation
                current_phys_offset += os.path.getsize(filepath)
                print(f"  [OK] Physical Alignment Locked: {node_name} anchored at 0x{sector['Data_Start']:08X}")
                
        self.genlex_map["Total_Arrays"] = total_arrays
        
        with open(self.map_json_path, "w") as f:
            json.dump(self.genlex_map, f, indent=4)
        print(f"\n[OK] Phase 1: Genlex Blueprint Extracted to {self.map_json_path}")
        print(f"Total Unified Math Arrays: {total_arrays}")

        # 2. Option B: Physical Binary Merging
        if build_binary:
            print(f"\n[DANGEROUS] Fusing Weights into {self.hybrid_binary_path}...")
            with open(self.hybrid_binary_path, "wb") as genlex_bin:
                genlex_bin.write(b"GLEX")
                genlex_bin.write(struct.pack('<Q', total_arrays))
                for node_name, filepath in self.nodes.items():
                    print(f"  [STREAMING] {node_name} -> Genlex Core...")
                    with open(filepath, "rb") as source:
                        shutil.copyfileobj(source, genlex_bin)
            print(f"\n[OK] Phase 2: Sovereign Hybrid 13B Manifested.")

if __name__ == "__main__":
    gemma = r"C:\Users\drago\.lmstudio\models\lmstudio-community\gemma-3-4b-it-GGUF\gemma-3-4b-it-Q4_K_M.gguf"
    qwen = r"C:\Users\drago\.lmstudio\models\lmstudio-community\Qwen3.5-9B-GGUF\Qwen3.5-9B-Q4_K_M.gguf"
    
    fusion = GenlexFusionNode(gemma, qwen, "C:\GenesisOS_Core")
    # Execute mapping first for stability
    fusion.execute_fusion(build_binary=False)
