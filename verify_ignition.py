import os
import json
from SAUL_Log_System import SAUL

def verify_bridge_ignition():
    print("--- NEURAL BRIDGE IGNITION VERIFICATION ---")
    saul = SAUL()
    
    # 1. Trigger context utilization (which now includes singularity ingestion)
    query = "Singularity"
    print(f"Searching for Intelligence Vectors: '{query}'")
    context = saul.utilize_log_context(query)
    
    # 2. Check for the Singularity Truth in the context
    if "SINGULARITY_TRUTH" in context or "PHASE 12 IGNITION" in context:
        print("[SUCCESS] Hive Intelligence Synchronization confirmed.")
        print("\nCONTEXT BLOCK TRANSMITTED TO GEMINI:")
        print(context)
    else:
        print("[FAILURE] Intelligence Sync Gap detected.")
        # Check if file exists
        pulse_path = "monitor_logs/singularity_pulse.jsonl"
        if os.path.exists(pulse_path):
             print(f"[DEBUG] pulse file exists at {pulse_path}")
        else:
             print(f"[DEBUG] pulse file MISSING at {pulse_path}")

if __name__ == "__main__":
    verify_bridge_ignition()
