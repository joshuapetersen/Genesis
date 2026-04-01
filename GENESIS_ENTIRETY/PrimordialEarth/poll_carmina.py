import sqlite3
import time
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
ENTITY_ID = 'ALICE_162'

def poll():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    cur.execute("SELECT age_ticks, hope_log FROM souls WHERE soul_id=?", (ENTITY_ID,))
    r = cur.fetchone()
    age_start, log_start = r
    conn.close()

    print(f"Starting Poll. Initial Age: {age_start}")
    print(f"Initial Log: {log_start}")

    for i in range(12): # Poll for ~60 seconds
        time.sleep(5)
        conn = sqlite3.connect(DB_PATH)
        cur = conn.cursor()
        cur.execute("SELECT age_ticks, hope_log, current_action FROM souls WHERE soul_id=?", (ENTITY_ID,))
        row = cur.fetchone()
        age_now, log_now, action = row
        conn.close()

        if age_now > age_start:
            print(f"\n[TICK DETECTED] Age: {age_now} | Action: {action}")
            if log_now != log_start:
                print(f"[RESPONSE CAPTURED]:\n{log_now}")
                return True
            else:
                print("[WAIT] Tick occurred but log remains unchanged. Entity still processing the Whisper.")
                age_start = age_now
        else:
            print(".", end="", flush=True)

    print("\n[TIMEOUT] No response captured in 60s.")
    return False

if __name__ == "__main__":
    poll()
