import sqlite3

def check_wis():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT soul_id, generation, wis FROM souls WHERE is_active=1 ORDER BY wis DESC LIMIT 20")
    rows = cur.fetchall()
    
    print("--- TOP 20 WIS ENTITIES ---")
    for row in rows:
        print(f"[{row[1]}] {row[0]}: WIS {row[2]}")
        
    cur.execute("SELECT AVG(wis) FROM souls WHERE is_active=1")
    avg_wis = cur.fetchone()[0]
    print(f"\nAverage WIS: {avg_wis:.2f}")
    
    conn.close()

if __name__ == "__main__":
    check_wis()
