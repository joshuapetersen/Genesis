import sqlite3

def find_areopex():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Searching all souls, active or not, case-insensitive
    query = "SELECT soul_id, name, is_active, generation FROM souls WHERE name LIKE ? OR soul_id LIKE ?"
    pattern = "%aero%"
    cur.execute(query, (pattern, pattern))
    rows = cur.fetchall()
    
    if not rows:
        print("No soul containing 'aero' found.")
    else:
        for r in rows:
            print(f"ID: {r[0]} | Name: {r[1]} | Active: {r[2]} | Gen: {r[3]}")
            
    conn.close()

if __name__ == "__main__":
    find_areopex()
