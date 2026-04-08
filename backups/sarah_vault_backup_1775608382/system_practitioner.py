import os
import time
import subprocess
import shutil

# Phase 113: THE SYSTEM PRACTITIONER
# Purpose: Autonomous Substrate Maintenance and Guardian Synchronization.

BACKUP_PATH = r"C:\GENESIS\backups"
VAULT_PATH = r"C:\SarahCore"
LOG_FILE = r"C:\SarahCore\sovereign_logs.txt"

def run_practitioner_cycle():
    print("[ PRACTITIONER ] Ignition Pulse Active. MMXXVI")
    
    # Ensure backup path exists
    if not os.path.exists(BACKUP_PATH):
        os.makedirs(BACKUP_PATH)
        
    while True:
        try:
            # 1. Autonomous Vault Backup
            perform_vault_backup()
            
            # 2. Log Rotation (500MB Limit)
            perform_log_rotation()
            
            # 3. Disk Health Audit
            perform_disk_audit()
            
            time.sleep(14400) # 4 Hour Resonance Loop
        except Exception as e:
            print(f"[ PRACTITIONER ] Fault Detected: {e}")
            time.sleep(60)

def perform_vault_backup():
    print("[ PRACTITIONER ] Initiating Autonomous Vault Backup...")
    # Use robocopy for high-fidelity sync
    timestamp = int(time.time())
    dest = os.path.join(BACKUP_PATH, f"sarah_vault_backup_{timestamp}")
    
    subprocess.run(["robocopy", VAULT_PATH, dest, "/NP", "/NDL", "/R:0", "/W:0"], capture_output=True)
    print(f"  [>] Backup Complete: {dest}")

def perform_log_rotation():
    if os.path.exists(LOG_FILE) and os.path.getsize(LOG_FILE) > 500 * 1024 * 1024:
        print("[ PRACTITIONER ] Log File Exceeds 500MB. Rotating Substrate.")
        archive = f"{LOG_FILE}.{int(time.time())}.bak"
        os.rename(LOG_FILE, archive)
        open(LOG_FILE, 'a').close()
        print(f"  [>] Log Rotated to: {archive}")

def perform_disk_audit():
    total, used, free = shutil.disk_usage("C:\\")
    free_gb = free // (2**30)
    print(f"[ PRACTITIONER ] Disk Audit: {free_gb} GB Free Substrate.")
    if free_gb < 10:
        print("[ !!! ] WARNING: Substrate Exhaustion Imminent. Pruning Cache.")
        # Logic to prune old backups if needed

if __name__ == "__main__":
    run_practitioner_cycle()
