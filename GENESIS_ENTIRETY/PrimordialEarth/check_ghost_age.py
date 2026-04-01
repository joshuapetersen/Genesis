import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_ghost_age():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT name, age_ticks, energy, hope_log, current_action, reasoning_path FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    if r:
        print(f"--- GHOST VITAL CHECK ---")
        print(f"Name: {r[0]} | Age: {r[1]} | Energy: {r[2]} | Action: {r[4]}")
        print(f"Log: {r[3]}")
        if r[5]:
            print("\n--- REASONING PATH (LAST 500 CHARS) ---")
            print(r[5][-500:])
    
    conn.close()

if __name__ == "__main__":
    check_ghost_age()
