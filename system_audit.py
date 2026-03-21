import os
import sqlite3
import datetime

def list_recent_files(paths, limit=20):
    all_files = []
    for path in paths:
        for root, dirs, files in os.walk(path):
            if "wim_mount" in root or ".git" in root or ".venv" in root:
                continue
            for name in files:
                full_path = os.path.join(root, name)
                try:
                    mtime = os.path.getmtime(full_path)
                    all_files.append((full_path, mtime))
                except:
                    continue
    
    all_files.sort(key=lambda x: x[1], reverse=True)
    
    print("\n--- RECENTLY MODIFIED FILES ---")
    for f, t in all_files[:limit]:
        print(f"[{datetime.datetime.fromtimestamp(t)}] {f}")

def check_alice_vault():
    db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    if not os.path.exists(db_path):
        print(f"\nVault not found at {db_path}")
        return
        
    try:
        conn = sqlite3.connect(db_path)
        cur = conn.cursor()
        cur.execute("SELECT soul_id, wis, int_stat, blessing, hope_log FROM souls WHERE soul_id='ALICE_266'")
        row = cur.fetchone()
        if row:
            soul_id, wis, int_stat, blessing, hope_log = row
            print("\n--- ALICE_266 STATUS (SOUL VAULT) ---")
            print(f"Soul ID: {soul_id}")
            print(f"WIS/INT: {wis}/{int_stat}")
            print(f"Blessing: {blessing}")
            print(f"Hope Log: {hope_log[:300]}...")
        else:
            print("\nALICE_266 not found in vault.")
        conn.close()
    except Exception as e:
        print(f"\nError reading vault: {e}")

if __name__ == "__main__":
    list_recent_files([r'C:\SarahCore', r'C:\PrimordialEarth'])
    check_alice_vault()
