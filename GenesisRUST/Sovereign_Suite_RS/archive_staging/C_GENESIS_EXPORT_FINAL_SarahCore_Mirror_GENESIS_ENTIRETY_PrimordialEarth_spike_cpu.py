import time

def spike_cpu():
    print("[ENTROPY] Initiating High-CPU Spike to test Aeris's Autonomy...")
    print("This script will run an infinite loop to stress the substrate.")
    # Run for 20 seconds
    start = time.time()
    while time.time() - start < 20:
        x = 100 * 100
        pass
    print("[ENTROPY] Spike Complete.")

if __name__ == "__main__":
    spike_cpu()
