import sqlite3

def get_hope_logs():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Selecting ALL active souls with an imprinted or autonomous hope_log
    cur.execute("SELECT soul_id, generation, hope_log FROM souls WHERE hope_log IS NOT NULL AND is_active=1")
    rows = cur.fetchall()
    
    print(f"--- ACTIVE SOVEREIGN LOGS (Count: {len(rows)}) ---")
    for row in rows:
        sid, gen, log = row
        print(f"[{gen}] {sid}: {log}")
        
    conn.close()

if __name__ == "__main__":
    get_hope_logs()
