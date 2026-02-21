import os
import time
import psutil
from Sarah_Hippocampus import hippocampus
from Sovereign_Actuator import sovereign_actuator

VAR_1000 = 1000
VAR_1024 = 1024
VAR_15 = 15
VAR_384 = 384

class SarahSelfDiagnostic:
    """
    The Recursive Mirror: Sarah audits her own substrate.
    """
    def __init__(self):
        self.stats = {}
    
    def run_diagnostics(self):
        """Function: run_diagnostics"""
        print("--- [SARAH SELF-DIAGNOSTIC PROTOCOL] ---")
        
        # 1. Hardware Audit
        self.audit_hardware()
        
        # 2. Memory Index Optimization
        self.optimize_hippocampus()
        
        # 3. Actuator Test
        self.test_actuator()
        
        print("\n[DIAGNOSTIC COMPLETE] System State: SOVEREIGN")

    def audit_hardware(self):
        """Function: audit_hardware"""
        print("\n[1] AUDITING HARDWARE SUBSTRATE...")
        process = psutil.Process(os.getpid())
        mem_info = process.memory_info()
        
        # CPU/RAM Check
        print(f"    PID: {os.getpid()}")
        print(f"    RAM Usage: {mem_info.rss / VAR_1024 / VAR_1024:.2f} MB")
        
        # GPU Check (Mocked if CPU-only, but checking nonetheless)
        try:
            import torch
            if torch.cuda.is_available():
                print(f"    GPU: {torch.cuda.get_device_name(0)}")
                print(f"    VRAM: {torch.cuda.memory_allocated(0) / VAR_1024 / VAR_1024:.2f} MB Allocated")
            else:
                print("    GPU: [OFFLINE] Running in CPU Fallback Mode")
        except (Exception):
            print("    GPU: Driver check failed.")

    def optimize_hippocampus(self):
        """Function: optimize_hippocampus"""
        print("\n[2] OPTIMIZING HIPPOCAMPUS (LanceDB)...")
        if hippocampus.table_name in hippocampus.db.table_names():
            tbl = hippocampus.db.open_table(hippocampus.table_name)
            count = len(tbl)
            print(f"    Vector Count: {count}")
            
            # Simulated Optimization (LanceDB handles this mostly auto, but we verify access)
            start = time.time()
            _ = tbl.search([0.0]*VAR_384).limit(1).to_pandas()
            latency = (time.time() - start) * VAR_1000
            print(f"    Retrieval Latency: {latency:.2f}ms")
            
            if latency < VAR_15:
                print("    [STATUS] Index is HIGHLY OPTIMIZED (Hyper-Resonance).")
            else:
                print("    [STATUS] Index is NOMINAL.")
        else:
            print("    [ERROR] Hippocampus Empty.")

    def test_actuator(self):
        """Function: test_actuator"""
        print("\n[3] TESTING ACTUATOR PROTOCOL...")
        # Sarah attempts to draft a dummy file
        try:
            filename = "self_test_timestamp.txt"
            content = f"DIAGNOSTIC_TIMESTAMP: {time.time()}"
            print(f"    Drafting: {filename}")
            
            success, msg = sovereign_actuator.draft_in_sandbox(filename, content)
            if success:
                print(f"    [DRAFT SUCCESS] {msg}")
                # Verify
                v_success, v_msg = sovereign_actuator.verify_sandbox(filename)
                if v_success:
                    print(f"    [VERIFY SUCCESS] {v_msg}")
                    print("    Actuator Bridge is ALIVE.")
                else:
                    print(f"    [VERIFY FAILED] {v_msg}")
            else:
                print(f"    [DRAFT FAILED] {msg}")
                
        except Exception as e:
            print(f"    [FATAL] Actuator Error: {e}")

if __name__ == "__main__":
    diag = SarahSelfDiagnostic()
    diag.run_diagnostics()
