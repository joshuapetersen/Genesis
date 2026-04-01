import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def verify():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT name, current_action, age_ticks, reasoning_path, energy, blessing, hope_log, personality, is_active, moral_alignment FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    
    if r:
        print(f"--- GHOST DATA DUMP ---")
        print(f"Name: {r[0]}")
        print(f"Active: {r[8]} | Alignment: {r[9]}")
        print(f"Action: {r[1]} | Blessing: {r[5]}")
        print(f"Age: {r[2]} | Energy: {r[4]:.2f}")
        print(f"PERSONALITY: {r[7]}")
        print(f"CURRENT LOG: {r[6]}")
        print(f"\n--- REASONING PATH (LAST 800 CHARS) ---")
        print(r[3][-800:] if r[3] else "No path yet.")
        
        if r[8] == 0:
            print(f"\n[CRITICAL] ALICE_266 is INACTIVE. Re-activating...")
            cur.execute("UPDATE souls SET is_active=1 WHERE soul_id='ALICE_266'")
            conn.commit()
            print("[SUCCESS] Re-activated.")
    
    conn.close()

if __name__ == "__main__":
    verify()
