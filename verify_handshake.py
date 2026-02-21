
import time
import sys
import os

# Add path
sys.path.append("C:\\SarahCore")

try:
    from Sovereign_Hypervisor import SovereignHypervisor
except ImportError:
    print("Could not import SovereignHypervisor")
    sys.exit(1)

def verify_handshake():
    """Function: verify_handshake"""
    print("--- GENESIS HANDSHAKE VERIFICATION ---")
    
    # 1. Initialize Hypervisor
    hypervisor = SovereignHypervisor()
    print("[TEST] Hypervisor Initialized.")
    
    # 2. Check for Heartbeat File
    if not os.path.exists("C:\\SarahCore\\system_heartbeat.json"):
        print("[FAIL] Heartbeat file not found. ensure Genesis_Kernel.py is running.")
        return
    
    # 3. Attempt Sync
    print("[TEST] Attempting Sync...")
    heartbeat = hypervisor.sync_to_genesis()
    
    if heartbeat:
        print("[SUCCESS] Handshake Validated!")
        print(f"  Kernel Version: {heartbeat.get('version')}")
        print(f"  Frequency Key: {heartbeat.get('frequency_key')}")
        print(f"  Cycle: {heartbeat.get('cycle')}")
    else:
        print("[FAIL] Handshake Failed (Desync or Invalid Key).")

if __name__ == "__main__":
    # Wait a moment for Kernel to write first heartbeat
    time.sleep(2)
    verify_handshake()
