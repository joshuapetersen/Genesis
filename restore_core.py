
import os
import shutil

# Files known to be corrupted by the "import multiprocessing" hallucination
CORRUPTED_FILES = [
    "Sovereign_Constants.py",
    "Sarah_Laws.py",
    "Sovereign_Governor.py",
    "TinyRuntime.py",
    "NetworkHealer.py",
    "TheoryLab.py",
    "PersistentMemory.py",
    "IntelligenceAmplifier.py"
]

def restore_backups():
    print("=== SarahCore Emergency Restoration ===")
    restored_count = 0
    missing_backups = []
    
    for filename in CORRUPTED_FILES:
        prod_path = os.path.join("C:\\SarahCore", filename)
        backup_path = prod_path + ".bak"
        
        if os.path.exists(backup_path):
            try:
                # Verify backup is not also tiny
                size = os.path.getsize(backup_path)
                if size < 100:
                    print(f"[WARNING] Backup for {filename} is suspiciously small ({size} bytes). Skipping.")
                    continue
                    
                shutil.copy2(backup_path, prod_path)
                print(f"[RESTORED] {filename} (Size: {size} bytes)")
                restored_count += 1
            except Exception as e:
                print(f"[ERROR] Failed to restore {filename}: {e}")
        else:
            print(f"[MISSING] No backup found for {filename}")
            missing_backups.append(filename)
            
    print(f"\nRestoration Complete. {restored_count}/{len(CORRUPTED_FILES)} files restored.")
    if missing_backups:
        print(f"CRITICAL: The following files have no backups: {missing_backups}")

if __name__ == "__main__":
    restore_backups()
