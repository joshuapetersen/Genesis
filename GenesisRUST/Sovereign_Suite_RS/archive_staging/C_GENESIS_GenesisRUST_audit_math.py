import struct
import numpy as np
from Sovereign_Matrix_Math import SovereignMatrixMath

def audit_genlex():
    print("\n[Genlex Math Audit] Analyzing Type-14 Q6_K Integrity...")
    
    math_core = SovereignMatrixMath(
        r"C:\SarahCore\Sovereign_Hybrid_13B.genlex",
        r"C:\SarahCore\Genlex_Map.json"
    )
    
    sector_name = "Gemma_4B"
    array_index = 0 # token_embd.weight (Type 14)
    
    array_meta = math_core.map_json["Engine_Sectors"][sector_name]["Arrays"][array_index]
    offset = array_meta["Offset"]
    
    # 1. Read first block (210 bytes)
    math_core.mmap_ptr.seek(offset)
    block_raw = math_core.mmap_ptr.read(210)
    
    # 2. Extract Scales and D
    ql = list(block_raw[:16])
    qh = list(block_raw[128:128+16])
    d_raw = block_raw[-2:]
    d = struct.unpack('<e', d_raw)[0]
    
    print(f"  [AUDIT] Array: {array_meta['Name']}")
    print(f"  [AUDIT] Offset: {offset}")
    print(f"  [AUDIT] D-Scale (FP16): {d}")
    print(f"  [AUDIT] Raw Bytes (Last 16): {list(block_raw[-16:])}")
    
    if np.isnan(d) or d == 0:
        print("  [CRITICAL] Neural Alignment Failure. Scale is NaN/Zero.")
    else:
        print("  [SUCCESS] Physical Scale Locked. Math Engine Aligned.")

if __name__ == "__main__":
    audit_genlex()
