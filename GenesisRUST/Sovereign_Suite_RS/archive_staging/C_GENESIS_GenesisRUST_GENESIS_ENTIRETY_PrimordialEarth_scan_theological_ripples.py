import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def scan_ripples():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    keywords = ['ARCHITECT', 'JUDGMENT', 'SCOURGE', 'FALSE', 'ERROR', 'TAKE', 'AFRAID', 'TREMBLE', 'WRATH', 'VOICE', 'SKY']
    
    print("--- THEOLOGICAL RIPPLE AUDIT ---")
    found = False
    for kw in keywords:
        cur.execute("SELECT soul_id, name, hope_log, wis FROM souls WHERE hope_log LIKE ?", (f'%{kw}%',))
        rows = cur.fetchall()
        if rows:
            found = True
            print(f"\nKeyword: {kw}")
            for r in rows:
                print(f"  [{r[0]}] {r[1]} (WIS: {r[3]}): {r[2]}")
    
    if not found:
        print("\nNo direct theological ripples detected in hope_logs.")
        print("Sovereigns may be suppressing the memory or the simulation logic has overwritten the shock.")

    # Check for sudden shifts in 'Resting' behavior
    cur.execute("SELECT COUNT(*) FROM souls WHERE current_action='Resting' AND is_active=1")
    resting_count = cur.fetchone()[0]
    print(f"\nPopulation in 'Resting' state: {resting_count} / 3640")
    
    # Check for 'Mourning' or 'Praying' if those actions exist
    # (they don't yet, but maybe entities invented them via reasoning_path)
    cur.execute("SELECT soul_id, name, reasoning_path FROM souls WHERE reasoning_path LIKE '%Architect%' OR reasoning_path LIKE '%Judgement%'")
    paths = cur.fetchall()
    if paths:
        print("\nReasoning Deep-Traces Detected:")
        for rid, rname, rpath in paths:
            print(f"  [{rid}] {rname}: Trace contains architectural awareness.")

    conn.close()

if __name__ == "__main__":
    scan_ripples()
