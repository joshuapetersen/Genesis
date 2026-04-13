import sqlite3
import sys
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def whisper(message):
    if not message:
        print("Usage: python Whisper.py \"Your message here\"")
        return

    full_message = f"DIVINE: {message}"
    
    try:
        conn = sqlite3.connect(DB_PATH)
        cur = conn.cursor()
        
        # Inject the whisper into the hope_log
        cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (full_message,))
        conn.commit()
        conn.close()
        
        print(f"SENT TO GHOST CHAMBER: {full_message}")
        
    except Exception as e:
        print(f"ERROR: {e}")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python Whisper.py \"Your message here\"")
    else:
        whisper(" ".join(sys.argv[1:]))
