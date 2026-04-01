import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_followers():
    if not os.path.exists(DB_PATH):
        return
        
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT soul_id, leader_id FROM souls WHERE is_active=1 AND leader_id IS NOT NULL")
    rows = cur.fetchall()
    if rows:
        print(f"Total entries with leader_id: {len(rows)}")
        for r in rows[:10]:
            print(f"  - {r[0]} follows {r[1]}")
    else:
        print("No active followers found.")
        
    cur.execute("SELECT COUNT(*) FROM souls WHERE leader_id = 'ALICE_266'")
    print(f"ALICE_266 specifically has {cur.fetchone()[0]} followers.")
    
    conn.close()

if __name__ == "__main__":
    check_followers()
