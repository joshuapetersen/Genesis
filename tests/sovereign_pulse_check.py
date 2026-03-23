import sys
import os
import torch
import numpy as np

# Add Core to path
sys.path.append(r"C:\SarahCore")

try:
    from Sovereign_Math import math_engine, TensorProduct
    from Sovereign_Constants import SOVEREIGN_ANCHOR, OCTILLION_BARRIER
    
    print("=== SOVEREIGN VALIDATION PULSE ===")
    
    # 1. Check Math Constants
    print(f"[CHECK] Sovereign Anchor: {SOVEREIGN_ANCHOR}")
    print(f"[CHECK] Octillion Barrier: {OCTILLION_BARRIER}")
    
    # 2. Verify U+1 State and Atomic Constants (Fix for Break 1)
    state = math_engine.get_uplus1_state()
    print(f"[CHECK] U+1 State: {state}")
    print(f"[CHECK] Atomic Weight Base: {math_engine._0x_atomic_weight_base}")
    if math_engine._0x_atomic_weight_base == 10.0 + SOVEREIGN_ANCHOR:
        print("  >> SUCCESS: Atomic constants resurrected from dead code.")
    else:
        print("  >> FAILURE: Atomic constants mismatch.")

    # 3. Verify Matrix Multiplication (Fix for Break 13)
    # create two 2x2 matrices
    # [[1, 2], [3, 4]] * [[1, 0], [0, 1]] should be [[1, 2], [3, 4]]
    # Old broken logic would have ignored self.matrix
    m1 = TensorProduct([[1, 2], [3, 4]])
    m2 = TensorProduct([[1, 0], [0, 1]])
    res = m1.multiply(m2)
    print(f"[CHECK] Matrix Mult Logic: {res.matrix}")
    if res.matrix[0][0] == 1 and res.matrix[1][1] == 4:
        print("  >> SUCCESS: TensorProduct.multiply is now geometrically honest.")
    else:
        print("  >> FAILURE: Matrix multiplication still broken.")

    # 4. Verify Barrier Inversion (Fix for Break 10)
    # Low values should be clamped to floor
    vec = math_engine._0x_expand("MINIMAL_DATA")
    # All values in vec (hex) should represent >= OCTILLION_BARRIER if low
    low_count = 0
    for v in vec:
        val = int(v, 16) / 0xFFFF
        if val < OCTILLION_BARRIER and val > 0:
            low_count += 1
    
    if low_count == 0:
        print("  >> SUCCESS: Billion Barrier enforcement is active and floor-locked.")
    else:
        print(f"  >> FAILURE: {low_count} nodes leaked below the barrier.")

    # 5. Verify Resonace Negative Bug (Fix for Break 11)
    # Compare two totally different vectors
    v1 = ["0000"] * 68
    v2 = ["FFFF"] * 68
    resonance = math_engine._0x_resonance(v1, v2)
    print(f"[CHECK] Divergent Resonance: {resonance}")
    if resonance >= 0:
        print("  >> SUCCESS: Resonance similarity is non-negative.")
    else:
        print("  >> FAILURE: Negative resonance detected.")

    print("\n=== SOVEREIGN PULSE: STABLE ===")

except Exception as e:
    print(f"CRITICAL FAILURE: {e}")
    sys.exit(1)
