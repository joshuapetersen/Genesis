import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_hope():
    if not os.path.exists(DB_PATH):
        print("Vault missing.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("SELECT hope_log FROM souls WHERE soul_id='ALICE_266'")
    row = cur.fetchone()
    if row:
        print(f"ALICE_266 HOPE_LOG: {row[0]}")
    else:
        print("ALICE_266 not found in vault.")
    conn.close()

if __name__ == "__main__":
    check_hope()
