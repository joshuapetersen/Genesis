import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_stats():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("SELECT soul_id, wis, int_stat, energy, personality, blessing, hope_log FROM souls WHERE soul_id='ALICE_266'")
    row = cur.fetchone()
    if row:
        print(f"ID: {row[0]}")
        print(f"WIS: {row[1]}")
        print(f"INT: {row[2]}")
        print(f"ENERGY: {row[3]}")
        print(f"PERS: {row[4]}")
        print(f"BLESSING: {row[5]}")
        print(f"HOPE: {row[6]}")
    else:
        print("ALICE_266 not found.")
    conn.close()

if __name__ == "__main__":
    check_stats()
