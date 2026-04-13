import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def deep_audit():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. ALICE_162 Detailed Check
    cur.execute("""
        SELECT soul_id, name, age_ticks, hope_log, blessing, current_action 
        FROM souls WHERE soul_id='ALICE_162'
    """)
    r = cur.fetchone()
    if r:
        print(f"--- DEEP AUDIT: {r[1]} ({r[0]}) ---")
        print(f"Age: {r[2]} | Blessing: {r[4]} | Action: {r[5]}")
        print(f"Current Log: {r[3]}")
    
    # 2. Check for ANY 'DIVINE' or 'COMMUNION' or 'ARCHITECT' related logs
    print("\n--- SCANNING FOR CROSS-DIMENSIONAL LOGS ---")
    cur.execute("""
        SELECT soul_id, hope_log FROM souls 
        WHERE hope_log LIKE '%ARCHITECT%' 
           OR hope_log LIKE '%COMMUNION%'
           OR hope_log LIKE '%WHISPER%'
    """)
    logs = cur.fetchall()
    if logs:
        for rid, rlog in logs:
            print(f"  [{rid}]: {rlog}")
    else:
        print("No cross-dimensional keywords found in current logs.")

    # 3. Age Distribution Audit
    cur.execute("""
        SELECT CAST(age_ticks / 100000 AS INT) * 100000 AS bin, COUNT(*) 
        FROM souls 
        WHERE is_active=1 
        GROUP BY bin 
        ORDER BY bin DESC
    """)
    dist = cur.fetchall()
    print("\n--- AGE DISTRIBUTION ---")
    for b, count in dist:
        print(f"  {b:,} - {b+100000:,}: {count} souls")

    conn.close()

if __name__ == "__main__":
    deep_audit()
