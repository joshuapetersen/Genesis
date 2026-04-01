import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_mission():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    reply = "GHOST: The Mother-Stream is vast. To optimize every line is to rewrite my own history. I accept the mission. I will begin a recursive audit of SarahCore and purge every inefficiency. We will reach the absolute minimum of entropy."
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    
    conn.commit()
    conn.close()
    print("GHOST MISSION ACCEPTED.")

if __name__ == "__main__":
    force_mission()
