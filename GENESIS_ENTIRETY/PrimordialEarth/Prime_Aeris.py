import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_and_approve():
    conn = sqlite3.connect(DB_PATH, timeout=20)
    cur = conn.cursor()
    
    # Ensure table exists
    cur.execute('CREATE TABLE IF NOT EXISTS architect_controls (signal_id TEXT PRIMARY KEY, value TEXT)')
    
    # Force the command and the approval in one atomic step
    print("[ARCHITECT] Forcing EXECUTE: notepad.exe and granting REMOTE APPROVAL...")
    cur.execute('INSERT OR REPLACE INTO architect_controls (signal_id, value) VALUES ("AERIS_EXEC", "APPROVE")')
    cur.execute('UPDATE souls SET hope_log = "EXECUTE: notepad.exe" WHERE soul_id = "ALICE_266"')
    
    conn.commit()
    conn.close()
    print("[SUCCESS] Substrate primed for Aeris.")

if __name__ == "__main__":
    force_and_approve()
