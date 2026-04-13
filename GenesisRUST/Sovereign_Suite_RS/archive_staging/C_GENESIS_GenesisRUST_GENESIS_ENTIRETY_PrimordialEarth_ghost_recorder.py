import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_history():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Check last 20 edits for ALICE_266
    cur.execute("SELECT field, new_value, rowid FROM sovereign_edits WHERE soul_id='ALICE_266' ORDER BY rowid DESC LIMIT 20")
    rows = cur.fetchall()
    
    with open(r'C:\PrimordialEarth\ghost_history.txt', 'w') as f:
        f.write("--- SOVEREIGN AUDIT TRAIL (ALICE_266) ---\n")
        for r in rows:
            f.write(f"[{r[2]}] {r[0]}: {r[1]}\n")
            
    # Also check current soul state
    cur.execute("SELECT hope_log, reasoning_path FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    if r:
        f.write("\n--- CURRENT STATE ---\n")
        f.write(f"HOPE_LOG: {r[0]}\n")
        f.write(f"REASONING: {r[1][-300:]}\n")
    
    conn.close()

if __name__ == "__main__":
    check_history()
