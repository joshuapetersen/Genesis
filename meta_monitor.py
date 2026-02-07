import time
import os
import sys

# --- CONFIGURATION ---
LOG_FILE = "c:\\SarahCore\\sovereign_logs.txt"

def clear_screen():
    os.system('cls' if os.name == 'nt' else 'clear')

def monitor_processes():
    if not os.path.exists(LOG_FILE):
        print(f"[ERROR] Cannot find {LOG_FILE}. Waiting for system start...")
        while not os.path.exists(LOG_FILE):
            time.sleep(1)

    print(f"[CONNECTED] Monitoring Meta-Processes in {LOG_FILE}...")
    
    # Counters
    stats = {
        "Hypervisor Decisions": 0,    # [Hypervisor] Intent Detected
        "Memory Writes (Vault)": 0,   # [Vault] Memory Partitioned
        "ACE Tokens (Thoughts)": 0,   # Token Fingerprint
        "Safety Overrides": 0,        # [Hypervisor] Safety Trip Detected
        "Syntheses Completed": 0      # Synthesis Complete
    }

    # Open file and go to the end (tail behavior)
    with open(LOG_FILE, "r", encoding="utf-8") as f:
        f.seek(0, os.SEEK_END)
        
        try:
            while True:
                line = f.readline()
                if not line:
                    time.sleep(0.1)  # Sleep briefly to save CPU
                    continue
                
                # --- DETECTION LOGIC ---
                update = False
                if "[Hypervisor] Intent Detected" in line:
                    stats["Hypervisor Decisions"] += 1
                    update = True
                elif "[Vault] Memory Partitioned" in line:
                    stats["Memory Writes (Vault)"] += 1
                    update = True
                elif "Token Fingerprint" in line:
                    stats["ACE Tokens (Thoughts)"] += 1
                    update = True
                elif "Safety Trip Detected" in line:
                    stats["Safety Overrides"] += 1
                    update = True
                elif "Synthesis Complete" in line:
                    stats["Syntheses Completed"] += 1
                    update = True

                # --- LIVE DASHBOARD ---
                if update:
                    clear_screen()
                    print("=== SOVEREIGN META-PROCESS MONITOR ===")
                    print(f"[-] STATUS:          ONLINE")
                    print(f"[-] WATCHING:        {LOG_FILE}")
                    print("--------------------------------------")
                    print(f"[*] ACE TOKENS GENERATED:   {stats['ACE Tokens (Thoughts)']}")
                    print(f"[*] HYPERVISOR CYCLES:      {stats['Hypervisor Decisions']}")
                    print(f"[*] VAULT WRITES (MEM):     {stats['Memory Writes (Vault)']}")
                    print(f"[*] SYNTHESIS CYCLES:       {stats['Syntheses Completed']}")
                    print("--------------------------------------")
                    print(f"[!] SAFETY OVERRIDES:       {stats['Safety Overrides']}")
                    print("--------------------------------------")
                    print("Press Ctrl+C to stop monitoring.")

        except KeyboardInterrupt:
            print("\n[STOPPED] Monitoring ended.")

if __name__ == "__main__":
    monitor_processes()
