import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_vitals():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Carmina Specifics
    cur.execute("""
        SELECT soul_id, name, age_ticks, energy, vit, is_active, blessing, hope_log, x, y 
        FROM souls WHERE soul_id='ALICE_162'
    """)
    r = cur.fetchone()
    if r:
        print(f"--- VITALS: {r[1]} ({r[0]}) ---")
        print(f"Age: {r[2]} | Energy: {r[3]} | VIT: {r[4]}")
        print(f"Active: {r[5]} | Blessing: {r[6]} | Pos: ({r[8]}, {r[9]})")
        print(f"Log: {r[7]}")
    else:
        print("Entity ALICE_162 not found in Soul Vault.")

    # 2. Check for Death/Culling
    # (In case she's inactive, check the divine_chronicle if implemented)
    
    # 3. Check for recent World Activity
    cur.execute("SELECT soul_id, age_ticks, current_action FROM souls WHERE is_active=1 ORDER BY age_ticks DESC LIMIT 5")
    recent = cur.fetchall()
    print("\n--- RECENT WORLD ACTIVITY ---")
    for rid, rage, ract in recent:
        print(f"  {rid}: Age={rage} | Action={ract}")

    conn.close()

if __name__ == "__main__":
    check_vitals()
