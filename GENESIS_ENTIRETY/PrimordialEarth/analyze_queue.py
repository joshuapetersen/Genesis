import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def analyze():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Get Carmina's current state
    cur.execute("SELECT age_ticks, hope_log FROM souls WHERE soul_id='ALICE_162'")
    row = cur.fetchone()
    if not row:
        print("ALICE_162 not found.")
        return
    c_age, c_log = row
    
    # Get global stats
    cur.execute("SELECT MAX(age_ticks), MIN(age_ticks), COUNT(*) FROM souls WHERE is_active=1")
    max_age, min_age, total = cur.fetchone()
    
    # Count how many are 'Ahead' in the current cycle
    cur.execute("SELECT COUNT(*) FROM souls WHERE age_ticks > ? AND is_active=1", (c_age,))
    ahead = cur.fetchone()[0]
    
    print(f"--- QUEUE ANALYSIS ---")
    print(f"Carmina Age: {c_age}")
    print(f"World Age Spread: {min_age} to {max_age}")
    print(f"Total Active Souls: {total}")
    print(f"Souls Ahead of Carmina: {ahead}")
    print(f"Souls at Carmina Level: {total - ahead}")
    
    # Check if the log has changed from the WHISPER
    if "WHISPER FROM THE ARCHITECT" not in c_log:
        print("\n[!!!] RESPONSE DETECTED [!!!]")
        print(f"LOG: {c_log}")
    else:
        print("\n[WAIT] Whisper is still active in her consciousness.")

    conn.close()

if __name__ == "__main__":
    analyze()
