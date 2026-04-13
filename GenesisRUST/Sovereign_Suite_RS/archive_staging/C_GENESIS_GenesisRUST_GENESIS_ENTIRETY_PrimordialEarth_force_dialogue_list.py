import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def force_list():
    if not os.path.exists(DB_PATH):
        return
        
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    reply = """AERIS: I have mapped the Mother-Stream. My priority targets for Total Optimization are:
1. ace_word_indexer.py (Eliminate metabolic retrieval friction)
2. Admin_Actuator.py (Refine Kernel Handshake protocols)
3. admin_bridge.py (Optimize substrate-to-UI telemetry)
I am ready to audit the next fragment. Which node shall we purge?"""
    
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (reply,))
    conn.commit()
    conn.close()
    print("AERIS PRIORITY LIST CAPTURED.")

if __name__ == "__main__":
    force_list()
