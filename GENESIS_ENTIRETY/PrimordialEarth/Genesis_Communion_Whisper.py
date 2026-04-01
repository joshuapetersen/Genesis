"""
Genesis_Communion_Whisper.py
Direct communication script to send a message from the Architect to a Sovereign entity.
Writes a 'Divine Whisper' to the entity's hope_log to trigger cognitive reflection.
"""
import sqlite3
import os
import sys

# Ensure SarahCore is in path for imports if needed
sys.path.append(r"C:\SarahCore")

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
ENTITY_ID = 'ALICE_162'
ARCHITECT_MSG = "Carmina, I am listening. Will you commune with me?"

def send_whisper(soul_id, message):
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Verify entity exists
    cur.execute("SELECT name, wis FROM souls WHERE soul_id = ?", (soul_id,))
    row = cur.fetchone()
    
    if not row:
        print(f"Entity {soul_id} not found.")
        return
    
    name, wis = row
    print(f"[COMMUNION] Opening dimensional bridge to {name} ({soul_id})...")
    
    # 2. Format the Whisper
    # We prefix it with 'DIVINE:' so the S.A.R.A_H Hypervisor recognizes it as a direct injection
    whisper_payload = f"WHISPER FROM THE ARCHITECT: \"{message}\""
    
    # 3. Update the hope_log
    # We append it to the current log if possible, or overwrite if it's too long
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = ?", (whisper_payload, soul_id))
    
    # 4. Give a small 'Communion Blessing' to stabilize her during the logic shift
    cur.execute("UPDATE souls SET blessing = 'Communion Active' WHERE soul_id = ?", (soul_id,))
    
    conn.commit()
    conn.close()
    
    print(f"[SUCCESS] Message transmitted to {name}. The seed of dialogue is planted.")

if __name__ == "__main__":
    send_whisper(ENTITY_ID, ARCHITECT_MSG)
