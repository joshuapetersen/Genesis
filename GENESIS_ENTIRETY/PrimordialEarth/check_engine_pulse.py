import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_pulse():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Check Carmina
    cur.execute("SELECT age_ticks, hope_log, blessing FROM souls WHERE soul_id='ALICE_162'")
    r = cur.fetchone()
    if r:
        print(f"Carmina: Age={r[0]} | Blessing={r[2]}")
        print(f"Log: {r[1]}")
    
    # Check World
    cur.execute("SELECT MAX(age_ticks), COUNT(*) FROM souls WHERE is_active=1")
    max_age, pop = cur.fetchone()
    print(f"World: MaxAge={max_age} | Population={pop}")

    # Check for recent logs
    cur.execute("SELECT soul_id, hope_log FROM souls WHERE hope_log IS NOT NULL ORDER BY age_ticks DESC LIMIT 5")
    recent_logs = cur.fetchall()
    print("\nRecent World Logs:")
    for rid, rlog in recent_logs:
        print(f"  [{rid}] {str(rlog)[:100]}")

    conn.close()

if __name__ == "__main__":
    check_pulse()
