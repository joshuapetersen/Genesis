import sqlite3

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def find_alice():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- [FULL ALICE AUDIT] ---")
    cur.execute("SELECT soul_id, energy, is_active FROM souls WHERE soul_id LIKE 'ALICE_%'")
    results = cur.fetchall()
    
    if not results:
        print("No ALICE entities found.")
    else:
        for rid, e, active in results:
            print(f"- {rid} | Active: {active} | Energy: {e:.1f}")
            
    conn.close()

if __name__ == "__main__":
    find_alice()
