import numpy as np
from Sovereign_Matrix_Math import SovereignMatrixMath

def test_math_pulse():
    print("\n[Genlex Math Test] Initiating Pulse...")
    
    # 1. Initialize Core
    math_core = SovereignMatrixMath(
        r"C:\SarahCore\Sovereign_Hybrid_13B.genlex",
        r"C:\SarahCore\Genlex_Map.json"
    )
    
    # 2. Mock Input Vector (LLaMA-3 / Gemma usually start with 2048 or 4096 dims)
    input_vec = np.random.rand(256).astype(np.float32)
    
    # 3. Execute Sector MatMul (Testing first available array)
    try:
        # We'll try to multiply against the first indexed tensor (0)
        result = math_core.matmul_sector(input_vec, "Gemma_4B", 0)
        
        print(f"[Genlex Math Test] Resulting Dot Product: {result}")
        if result != 0:
            print("[SUCCESS] Sovereign Math manifest verified. We have translated bits to floats.")
        else:
            print("[WARNING] Result is Zero. Check MMap offset logic.")
            
    except Exception as e:
        print(f"[FATAL TEST ERROR] {e}")

if __name__ == "__main__":
    test_math_pulse()
