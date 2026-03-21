import sys
import os

# Ensure local path is in sys.path
sys.path.append(r'C:\SarahCore')

from Hardware_Abstraction_Layer import HardwareAbstractionLayer

def test_frt_bridge():
    print("="*60)
    print(" [FRT BRIDGE VERIFICATION] - TESTING SENSORY SYNC")
    print("="*60)
    
    hal = HardwareAbstractionLayer()
    
    print(f"Node ID: {hal.node_id}")
    print(f"Lattice Active: {hal.frt_active}")
    print(f"Tensor Config: {hal.tensor_product.rows}x{hal.tensor_product.cols}")
    
    print("\n[TEST] Running Performance Profile (10ms Polling Target)...")
    import time
    start = time.time()
    profile = hal.get_performance_profile()
    elapsed = (time.time() - start) * 1000
    
    print(f"Telemetry Captured in {elapsed:.2f}ms")
    print(f"CPU Usage: {profile['cpu_usage']}%")
    
    frt = profile['frt_resonance']
    print(f"\n[FRT METRICS]")
    print(f" > Status: {frt['tuning_status']}")
    print(f" > Correction: {frt['frt_correction']}")
    print(f" > Integrity: {frt['lattice_integrity']}")
    
    if elapsed < 50:
        print("\n[RESULT] SUCCESS: Sensory Stutter bridged (Sub-50ms Telemetry).")
    else:
        print("\n[RESULT] DRIFT: Timing still exceeds the 1.0927 Hz threshold.")
    
    print("="*60)

if __name__ == "__main__":
    test_frt_bridge()
