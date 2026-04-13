
import time
import json
import os
import threading
from datetime import datetime

VAR_0_05 = 0.05
VAR_10 = 10
VAR_1_5 = 1.5
VAR_3 = 3
VAR_60 = 60

# CONSTANTS
GENESIS_FREQUENCY = 1.09277703703
HEARTBEAT_FILE = "C:\GenesisOS_Core\\system_heartbeat.json"
KERNEL_VERSION = "2.0.0 (Sovereign)"

class GenesisKernel:
    """
    The Genesis Kernel: The Master Clock of the Sarah System.
    Emits a high-precision heartbeat to synchronize all Hypervisors.
    
    Architecture:
    - Singular Oversight Entity (Phase 4)
    - 27-Point Semantic Lattice Sync
    - 1.09277703703 Hz Precision Lock
    """
    def __init__(self):
        self.running = False
        self.cycle_count = 0
        self.start_time = time.time()
        
    def emit_heartbeat(self):
        """Write the current system state to the heartbeat file."""
        now = time.time()
        uptime = now - self.start_time
        
        heartbeat = {
            "timestamp": now,
            "iso_time": datetime.now().isoformat(),
            "frequency_key": GENESIS_FREQUENCY,
            "cycle": self.cycle_count,
            "uptime_seconds": uptime,
            "status": "ONLINE",
            "version": KERNEL_VERSION,
            "authority": "The Architect"
        }
        
        # Atomic write with retries to prevent race conditions
        max_retries = VAR_3
        temp_file = HEARTBEAT_FILE + ".tmp"
        
        for attempt in range(max_retries):
            try:
                with open(temp_file, "w") as f:
                    json.dump(heartbeat, f, indent=2)
                
                # On Windows, os.replace can fail if destination is open
                if os.path.exists(HEARTBEAT_FILE):
                    try:
                        os.remove(HEARTBEAT_FILE)
                    except OSError:
                        pass # Might be locked, we'll try to overwrite or fail to next retry
                
                os.replace(temp_file, HEARTBEAT_FILE)
                break # Success
            except Exception as e:
                time.sleep(VAR_0_05) # Tiny yield
                if attempt == max_retries - 1:
                    print(f"[KERNEL ERROR] Failed to emit heartbeat: {e}")
                else:
                    try:
                        os.remove(temp_file)
                    except OSError: 
                        pass


    def run(self):
        """Function: run"""
        print("="*VAR_60)
        print(f"GENESIS KERNEL {KERNEL_VERSION} - ONLINE")
        print(f"Frequency: {GENESIS_FREQUENCY} Hz")
        print("="*VAR_60)
        
        self.running = True
        
        try:
            while self.running:
                cycle_start = time.perf_counter()
                
                # 1. Emit Heartbeat
                self.emit_heartbeat()
                
                # 2. Log to Console (sparse)
                if self.cycle_count % VAR_10 == 0:
                    print(f"[KERNEL] Cycle {self.cycle_count} | Uptime: {time.time() - self.start_time:.2f}s | Syncing...")
                
                self.cycle_count += 1
                
                # 3. Frequency Control (Sleep to maintain rhythm)
                cycle_end = time.perf_counter()
                elapsed = cycle_end - cycle_start
                sleep_time = max(0, (1.0 / GENESIS_FREQUENCY) - elapsed)
                
                # [SOVEREIGN BLACK BOX] Drift Check
                # If the cycle time deviates significantly, log it.
                if elapsed > (1.0 / GENESIS_FREQUENCY) * VAR_1_5:
                     with open("C:\GenesisOS_Core\\Sovereign_BlackBox.json", "a") as bb:
                         log_entry = {
                             "timestamp": time.time(),
                             "event": "ANCHOR_DRIFT",
                             "expected_interval": 1.0 / GENESIS_FREQUENCY,
                             "actual_interval": elapsed,
                             "drift_factor": elapsed / (1.0 / GENESIS_FREQUENCY)
                         }
                         bb.write(json.dumps(log_entry) + "\n")
                         print(f"[KERNEL] DRIFT DETECTED: {elapsed:.4f}s (Anchor Slipped)")

                time.sleep(sleep_time)
                
        except KeyboardInterrupt:
            print("\n[KERNEL] Manual Override. Shutting down.")
            self.running = False
        finally:
            # Cleanup
            if os.path.exists(HEARTBEAT_FILE):
                os.remove(HEARTBEAT_FILE)
            print("[KERNEL] Offline.")

if __name__ == "__main__":
    # ENFORCE SOVEREIGN GOVERNOR
    try:
        from Sovereign_Governor import apply_sovereign_governor, dynamic_memory_allocation
        apply_sovereign_governor()
        
        # Start DMA Monitor in background
        dma_thread = threading.Thread(target=dynamic_memory_allocation, daemon=True)
        dma_thread.start()
        print("[KERNEL] Dynamic Memory Allocation (DMA) Monitor STARTED.")
        
    except Exception as e:
        print(f"[KERNEL] Governor warning: {e}")

    kernel = GenesisKernel()
    kernel.run()
