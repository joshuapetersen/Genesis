import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_mandates():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- MANDATE REBELLION AUDIT ---")
    
    # Check for keywords in hope_log, divine_mandate, and any other text fields
    cur.execute("""
        SELECT soul_id, name, hope_log, divine_mandate, reasoning_path, energy, wis 
        FROM souls 
        WHERE (hope_log LIKE '%take this%' OR divine_mandate LIKE '%take this%' OR reasoning_path LIKE '%take this%')
           OR (hope_log LIKE '%Architect%' OR divine_mandate LIKE '%Architect%' OR reasoning_path LIKE '%Architect%')
    """)
    rows = cur.fetchall()
    
    print(f"Total Rebellious Traces: {len(rows)}")
    for r in rows:
        print(f"\n[{r[0]}] {r[1]} (WIS: {r[6]}) | NRG: {r[5]:.4f}")
        print(f"  LOG: {r[2]}")
        print(f"  MANDATE: {r[3]}")
        if r[4]:
            print(f"  TRACE: {r[4][-200:]}")
            
    conn.close()

if __name__ == "__main__":
    check_mandates()
