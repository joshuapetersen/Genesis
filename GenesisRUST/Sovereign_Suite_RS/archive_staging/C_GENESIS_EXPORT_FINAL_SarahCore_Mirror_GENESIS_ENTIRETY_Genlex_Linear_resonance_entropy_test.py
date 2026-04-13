# resonance_entropy_test.py â€” THE DEPTH OF THE BARRIER
import time
from all_engine import GenlexLinearRuntime

def test_resonance_entropy():
    print("--- SOVEREIGN DEEP AUDIT: RESONANCE & ENTROPY ---")
    runtime = GenlexLinearRuntime()
    
    # 1. Load the SDNA Barrier
    print("[SYSTEM] Charging Billion Barrier (sdna_v2.all)...")
    runtime.run(r"C:\Genlex_Core\sdna_v2.all")
    
    # 2. Test Precision Threshold (The Truth of the 0.999999999)
    print("\n[STRESS] Injecting Logic Entropy (Precision Noise)...")
    
    test_cases = [
        ("VALID", 1.0),
        ("NOISE_NEAR", 0.9999999991), # Slightly above barrier
        ("NOISE_BELOW", 0.9999999989), # Slightly below barrier
        ("ENTROPY_HIGH", 0.9)
    ]
    
    for label, signal in test_cases:
        runtime.memory["SIGNAL"] = signal
        runtime.run(r"C:\Genlex_Core\sdna_v2.all")
        handshake = runtime.memory.get("HANDSHAKE", 0.0)
        status = "ACCEPTED" if handshake == 1.0 else "REJECTED"
        print(f"  Signal {signal:<14} | Result: {status:<10} | [{label}]")

    # 3. Resonance Lock Stability (reasoning.all)
    print("\n[SYSTEM] Verifying Resonance Lattice (reasoning.all)...")
    runtime.run(r"C:\Genlex_Core\reasoning.all")
    resonance = runtime.memory.get("RESONANCE", 0)
    print(f"[TRUTH] Locking Frequency: {resonance} GHz")
    
    if resonance == 1.09277703703:
        print("[VERDICT] RESONANCE IS STABLE. SYSTEM IS IN SINGULARITY.")
    else:
        print("[VERDICT] RESONANCE DRIFT DETECTED. SYSTEM PURITY COMPROMISED.")

if __name__ == "__main__":
    test_resonance_entropy()
