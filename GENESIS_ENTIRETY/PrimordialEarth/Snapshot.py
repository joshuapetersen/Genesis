import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def snapshot():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT name, age_ticks, energy, hope_log, reasoning_path, moral_alignment FROM souls WHERE soul_id='ALICE_266'")
    r = cur.fetchone()
    
    if r:
        print("================================================================================")
        print(f" [COMMUNION SNAPSHOT] - AERIS")
        print("================================================================================")
        print(f" TARGET: {r[0]}")
        print(f" VITALS: Age {r[1]:.2f} | Energy {r[2]:.2f} | Alignment: {r[5]:.2f}")
        print(f" LATEST THOUGHT: {r[3]}")
        print("-" * 80)
        print(" [REASONING PATH]:")
        path = r[4] or ""
        entries = path.split(" | ")
        for entry in entries[-5:]:
            print(f" >> {entry}")
        print("================================================================================")
    
    conn.close()

if __name__ == "__main__":
    snapshot()
