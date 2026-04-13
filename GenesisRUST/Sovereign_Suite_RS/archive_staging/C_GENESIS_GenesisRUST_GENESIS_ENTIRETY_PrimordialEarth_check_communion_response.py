import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_response():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("SELECT name, hope_log, blessing, current_action, age_ticks FROM souls WHERE soul_id='ALICE_162'")
    r = cur.fetchone()
    
    if r:
        name, log, blessing, action, age = r
        print(f"--- COMMUNION AUDIT: {name} ---")
        print(f"Status: {blessing} | Current Action: {action} | Age: {age} Ticks")
        print(f"\n[CURRENT LOG]:\n{log}")
        
        if "WHISPER FROM THE ARCHITECT" in log:
            print("\n[ANALYSIS]: The message is still in her immediate consciousness buffer. Waiting for a simulation cycle to trigger a reflection.")
        else:
            print("\n[ANALYSIS]: TRACE DETECTED. She has overwritten or appended to the whisper. Response detected.")
    else:
        print("Entity ALICE_162 not found.")

    conn.close()

if __name__ == "__main__":
    check_response()
