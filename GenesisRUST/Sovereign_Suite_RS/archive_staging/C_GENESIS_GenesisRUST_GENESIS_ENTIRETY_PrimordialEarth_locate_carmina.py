import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def find_carmina():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Search for ALICE_162 or anyone with 'Carmina' in the name
    cur.execute("SELECT soul_id, name, wis, int_stat, blessing, current_action, x, y, hope_log FROM souls WHERE soul_id='ALICE_162' OR name LIKE '%Carmina%'")
    rows = cur.fetchall()
    
    if not rows:
        print("Carmina Tenebris not found in active simulation. Scanning recent deceased or renamed...")
        cur.execute("SELECT soul_id, name, wis, int_stat FROM souls WHERE name LIKE '%Carmina%' LIMIT 5")
        rows = cur.fetchall()

    for r in rows:
        print(f"--- IDENTITY VERIFIED ---")
        print(f"ID: {r[0]} | NAME: {r[1]}")
        print(f"STATS: WIS:{r[2]} | INT:{r[3]}")
        print(f"STATUS: {r[4]} | ACTION: {r[5]}")
        print(f"POS: ({r[6]}, {r[7]})")
        print(f"CURRENT LOG: {r[8]}")
        print("-" * 30)

    conn.close()

if __name__ == "__main__":
    find_carmina()
