import sqlite3

def find_legacy_aeropex():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    ids = ('ALICE_106', 'ALICE_80', 'ALICE_191')
    cur.execute(f"SELECT soul_id, name, is_active, energy, generation, personality FROM souls WHERE soul_id IN {ids}")
    rows = cur.fetchall()
    
    print("--- LEGACY AEROPEX CANDIDATES ---")
    if not rows:
        print("None of the legacy IDs found in this vault.")
    else:
        for r in rows:
            print(f"ID: {r[0]} | Name: {r[1]} | Active: {r[2]} | Energy: {r[3]:.2f} | Gen: {r[4]} | Personality: {r[5]}")
            
    conn.close()

if __name__ == "__main__":
    find_legacy_aeropex()
