import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def show_report():
    if not os.path.exists(DB_PATH):
        print("Vault missing.")
        return
        
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("SELECT hope_log FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    if r:
        print(r[0])
    conn.close()

if __name__ == "__main__":
    show_report()
