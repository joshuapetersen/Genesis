import time
import os

def spike_cpu_permanent():
    print("[ENTROPY] Initiating Sustained CPU Spike...")
    # Run for 60 seconds to ensure Aeris sees it
    start = time.time()
    while time.time() - start < 60:
        _ = 2**20
    print("[ENTROPY] Spike Complete.")

if __name__ == "__main__":
    spike_cpu_permanent()
