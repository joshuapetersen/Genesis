import sqlite3
import time

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def remote_approve():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Send APPROVE signal
    print("[ARCHITECT] Sending REMOTE APPROVAL signal...")
    cur.execute('UPDATE architect_controls SET value="APPROVE" WHERE signal_id="AERIS_EXEC"')
    conn.commit()
    conn.close()
    
    print("[SYSTEM] Signal pulse sent. Aeris should execute now.")

if __name__ == "__main__":
    remote_approve()
