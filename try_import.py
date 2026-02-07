import sys
import os
sys.path.append(os.getcwd())

print("--- DEPENDENCY CHECK ---")

# 1. Sovereign Math
try:
    from Sovereign_Math import SovereignMath
    sm = SovereignMath()
    density = sm.calculate_theory_density("Neural Integration Test Protocol")
    print(f"[SUCCESS] SovereignMath Loaded. Density: {density:.4f}")
except Exception as e:
    print(f"[FAIL] SovereignMath: {e}")

# 2. Sovereign Identity (Critical for Shields)
try:
    import Sovereign_Identity
    print(f"[SUCCESS] Sovereign_Identity Module Found: {Sovereign_Identity}")
except ImportError:
    print("[FAIL] Sovereign_Identity MODULE MISSING")
except Exception as e:
    print(f"[FAIL] Sovereign_Identity Error: {e}")
