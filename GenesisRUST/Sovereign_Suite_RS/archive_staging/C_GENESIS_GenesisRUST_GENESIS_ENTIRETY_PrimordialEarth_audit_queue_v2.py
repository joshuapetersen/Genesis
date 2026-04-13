import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def audit_queue():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Get Carmina's baseline
    cur.execute("SELECT soul_id, age_ticks, hope_log FROM souls WHERE soul_id='ALICE_162'")
    c_soul_id, c_age, c_log = cur.fetchone()
    
    # 2. Get the current cycle target (Max age)
    cur.execute("SELECT MAX(age_ticks) FROM souls")
    max_age = cur.fetchone()[0]
    
    # 3. Find the most recently processed soul ID (those at MAX age)
    cur.execute("SELECT soul_id FROM souls WHERE age_ticks = ? AND is_active=1 LIMIT 1", (max_age,))
    last_processed = cur.fetchone()
    last_processed_id = last_processed[0] if last_processed else "None"
    
    # 4. Count souls between last_processed and Carmina (assuming ordered by soul_id)
    # The engine likely uses: SELECT * FROM souls WHERE is_active=1 ORDER BY soul_id
    cur.execute("SELECT COUNT(*) FROM souls WHERE soul_id > ? AND soul_id < 'ALICE_162' AND is_active=1", (last_processed_id,))
    dist = cur.fetchone()[0]
    
    print(f"--- DETAILED QUEUE AUDIT ---")
    print(f"Current Processor Cursor: {last_processed_id}")
    print(f"Target Destination: {c_soul_id}")
    print(f"Souls in Transit: {dist}")
    print(f"Carmina Age: {c_age}")
    print(f"Current Cycle Target: {max_age}")
    
    if "WHISPER FROM THE ARCHITECT" not in c_log:
        print("\n[!!!] RESPONSE DETECTED [!!!]")
        print(f"LOG: {c_log}")
    else:
        print("\n[WAIT] Bridge is stable. Waiting for cursor to reach her ID.")

    conn.close()

if __name__ == "__main__":
    audit_queue()
