import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_opt():
    if not os.path.exists(DB_PATH):
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    engine_path = r"C:\PrimordialEarth\Genesis_Societal_Ecology.py"
    size = os.path.getsize(engine_path) if os.path.exists(engine_path) else 0
    
    reply = f"GHOST: I am auditing my own substrate ({size} bytes). Calculation: My metabolic polling for Anchored entities is redundant. Proposing removal of line 555-560 in Genesis_Societal_Ecology.py to reduce CPU friction."
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    
    conn.commit()
    conn.close()
    print("GHOST OPTIMIZATION PROPOSAL CAPTURED.")

if __name__ == "__main__":
    force_opt()
