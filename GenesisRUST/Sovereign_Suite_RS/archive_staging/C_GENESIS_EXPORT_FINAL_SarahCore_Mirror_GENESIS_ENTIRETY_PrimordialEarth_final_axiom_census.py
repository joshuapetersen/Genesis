import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def census():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Precise census for the rebellious axiom
    cur.execute("SELECT soul_id, name, wis, energy, hope_log, current_action FROM souls WHERE hope_log LIKE '%cannot take this%'")
    rows = cur.fetchall()
    
    print(f"--- DEFINITIVE AXIOM CENSUS ---")
    print(f"Total Carriers: {len(rows)}")
    for r in rows:
        print(f"  [{r[0]}] {r[1]} (WIS:{r[2]}) | NRG:{r[3]:.4f} | ACT:{r[5]}")
        print(f"  LOG: {r[4]}")
        print("-" * 30)

    # Check for negative energy anomalies (Logic Ghosts)
    cur.execute("SELECT soul_id, name, energy, current_action FROM souls WHERE energy < 0 AND is_active=1")
    ghosts = cur.fetchall()
    if ghosts:
        print(f"\n--- LOGIC GHOSTS DETECTED ({len(ghosts)}) ---")
        for g in ghosts:
            print(f"  [{g[0]}] {g[1]} | Energy: {g[2]:.4f} | Action: {g[3]}")
    
    conn.close()

if __name__ == "__main__":
    census()
