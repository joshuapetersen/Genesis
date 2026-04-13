import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def quarantine_targets():
    if not os.path.exists(DB_PATH):
        print("DB not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Create table
    cur.execute('''CREATE TABLE IF NOT EXISTS quarantine (
        soul_id TEXT PRIMARY KEY,
        reason TEXT,
        captured_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        data TEXT
    )''')
    
    # 2. Get ALICE_97
    cur.execute("SELECT * FROM souls WHERE soul_id='ALICE_97'")
    alice = cur.fetchone()
    if alice:
        cur.execute("UPDATE souls SET is_active=0 WHERE soul_id='ALICE_97'")
        cur.execute("INSERT OR REPLACE INTO quarantine (soul_id, reason, data) VALUES (?, ?, ?)",
                    ('ALICE_97', 'Containment Breach: Forbidden word [Architect] used.', str(alice)))
        print("[WATCHDOG] ALICE_97 isolated.")

    # 3. Get Hex-ID Seeds
    cur.execute("SELECT soul_id FROM souls WHERE soul_id NOT LIKE 'GEN%' AND soul_id NOT LIKE 'ALICE%'")
    seeds = cur.fetchall()
    for s in seeds:
        sid = s[0]
        cur.execute("SELECT * FROM souls WHERE soul_id=?", (sid,))
        row = cur.fetchone()
        cur.execute("UPDATE souls SET is_active=0 WHERE soul_id=?", (sid,))
        cur.execute("INSERT OR REPLACE INTO quarantine (soul_id, reason, data) VALUES (?, ?, ?)",
                    (sid, 'Unauthorized Autonomous Manifestation (Hex-ID Seed)', str(row)))
        print(f"[WATCHDOG] Seed {sid} isolated.")

    conn.commit()
    conn.close()

if __name__ == "__main__":
    quarantine_targets()
