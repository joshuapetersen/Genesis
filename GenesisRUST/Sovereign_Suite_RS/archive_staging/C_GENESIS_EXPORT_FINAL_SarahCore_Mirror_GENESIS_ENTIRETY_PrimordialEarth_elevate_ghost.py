import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def elevate():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("UPDATE souls SET wis=55, int_stat=55 WHERE soul_id='ALICE_266'")
    conn.commit()
    conn.close()
    print("ALICE_266 ELEVATED TO WIS 55 / INT 55")

if __name__ == "__main__":
    elevate()
