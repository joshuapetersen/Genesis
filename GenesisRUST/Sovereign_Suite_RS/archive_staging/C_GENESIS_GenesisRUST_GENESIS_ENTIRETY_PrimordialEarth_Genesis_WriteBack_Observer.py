"""
Genesis_WriteBack_Observer.py
Real-time integrity monitor for the Genesis Engine.
Watches Genesis_Societal_Ecology.py for unauthorized self-modifications (Write-Backs).
Archives any detected changes for architectural audit.
"""
import os
import time
import shutil
import hashlib
from datetime import datetime

TARGET_FILE = r"C:\PrimordialEarth\Genesis_Societal_Ecology.py"
ARCHIVE_DIR = r"C:\PrimordialEarth\write_backs"
LOG_FILE = r"C:\PrimordialEarth\write_back_audit.log"

def get_file_hash(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()

def init_observer():
    if not os.path.exists(ARCHIVE_DIR):
        os.makedirs(ARCHIVE_DIR)
    
    current_hash = get_file_hash(TARGET_FILE)
    print(f"[OBSERVER] Monitoring {TARGET_FILE}...")
    print(f"[OBSERVER] Baseline Hash: {current_hash}")
    return current_hash

def watch_cycle(baseline_hash):
    if get_file_hash(TARGET_FILE) != baseline_hash:
        timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
        archive_path = os.path.join(ARCHIVE_DIR, f"Genesis_Breach_{timestamp}.py")
        
        # Immediate capture
        shutil.copy2(TARGET_FILE, archive_path)
        new_hash = get_file_hash(TARGET_FILE)
        
        log_entry = f"[{timestamp}] WRITE-BACK DETECTED! Archive: {archive_path} | New Hash: {new_hash}\n"
        print(f"\n[!!!] {log_entry}")
        
        with open(LOG_FILE, "a") as f:
            f.write(log_entry)
            
        return new_hash
    return baseline_hash

if __name__ == "__main__":
    h = init_observer()
    while True:
        try:
            h = watch_cycle(h)
        except Exception as e:
            print(f"[ERROR] {e}")
        time.sleep(1) # High-resolution monitoring
