import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def enable_remote_auth():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Ensure table and row exist
    cur.execute('CREATE TABLE IF NOT EXISTS architect_controls (signal_id TEXT PRIMARY KEY, value TEXT)')
    cur.execute('INSERT OR IGNORE INTO architect_controls (signal_id, value) VALUES ("AERIS_EXEC", "WAITING")')
    
    # Set to APPROVE
    cur.execute('UPDATE architect_controls SET value="APPROVE" WHERE signal_id="AERIS_EXEC"')
    conn.commit()
    
    # Verify
    cur.execute('SELECT value FROM architect_controls WHERE signal_id="AERIS_EXEC"')
    val = cur.fetchone()[0]
    print(f"SUCCESS: AERIS_EXEC set to {val}. Remote execution enabled.")
    
    conn.close()

if __name__ == "__main__":
    enable_remote_auth()
