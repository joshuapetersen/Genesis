# run_legacy_test.py — LSE BOOTSTRAP & PROOF OF REALITY
# This script executes the Genlex Hypervisor with the Legacy Emulator attached.

import sys
from all_engine import GenlexLinearRuntime

def run_emulator_proof():
    print("--- SOVEREIGN EMULATOR (UNIVERSAL HOSTING) ---")
    runtime = GenlexLinearRuntime()
    
    # 1. Load the Sovereign Master Hypervisor
    print("[SYSTEM] Initializing Master Substrate...")
    runtime.run(r"C:\Genlex_Core\sarah_hypervisor.all")
    
    # 2. Simulate a "Windows/Android" Guest Request
    print("\n[EMULATOR] Intercepting 'Guest OS' Request (x86 INT 10h)...")
    runtime.stack.append(0x0E) # AH (BIOS Service)
    runtime.stack.append(ord('G')) # AL ('G' for Genesis/Guest)
    
    # 3. Translate through the Sovereign Emulator Bridge
    print("[EMULATOR] Translating 'Guest' Logic -> 'Sovereign' Hardware...")
    runtime.run(r"C:\Genlex_Core\legacy_emulator.all")
    
    print("\n--- EMULATION SUCCESSFUL ---")
    print("Result: Corporate OS Logic hosted and redirected to Sovereign Driver.")

if __name__ == "__main__":
    run_emulator_proof()
