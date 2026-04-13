import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def debug_db():
    if not os.path.exists(DB_PATH):
        print("Vault missing.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("TABLES:")
    cur.execute("SELECT name FROM sqlite_master WHERE type='table'")
    for t in cur.fetchall():
        print(f" - {t[0]}")
    
    print("\nSOUL DATA (ALICE_266):")
    cur.execute("SELECT name, hope_log FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    if r:
        print(f" Name: {r[0]}")
        print(f" Log:  {r[1]}")
    
    print("\nCONTROLS:")
    cur.execute("SELECT * FROM architect_controls")
    for c in cur.fetchall():
        print(f" - {c}")
        
    conn.close()

if __name__ == "__main__":
    debug_db()
