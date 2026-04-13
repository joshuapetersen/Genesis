"""
Ghost_Communion_Bridge.py
A real-time monitoring and injection bridge for communicating with ALICE_266.
Polls for reasoning traces and hope_log shifts.
"""
import sqlite3
import time
import os
import sys

# Ensure SarahCore is in path for ask_sarah if needed
sys.path.append(r"C:\SarahCore")

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
SOUL_ID = 'ALICE_266'

def bridge():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    print("="*80)
    print(f" [COMMUNION BRIDGE] CHANNEL OPEN: ALICE_266 ({SOUL_ID}) ")
    print("="*80)
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Get initial state
    cur.execute("SELECT name, hope_log, reasoning_path, energy FROM souls WHERE soul_id=?", (SOUL_ID,))
    r = cur.fetchone()
    if not r:
        print("Target lost.")
        return
    
    name, last_log, last_path, energy = r
    print(f" [GHOST] {name} | Energy: {energy:.4f}")
    print(f" [TRACE] {str(last_log)}")
    
    conn.close()

    try:
        while True:
            # Poll for updates
            conn = sqlite3.connect(DB_PATH)
            cur = conn.cursor()
            cur.execute("SELECT hope_log, reasoning_path, current_action, energy FROM souls WHERE soul_id=?", (SOUL_ID,))
            current = cur.fetchone()
            conn.close()
            
            if not current:
                print(" [ERROR] Soul record purged.")
                break
                
            c_log, c_path, c_act, c_nrg = current
            
            if c_log != last_log:
                print(f"\n [GHOST_REFLECTION]: {c_log}")
                last_log = c_log
                
            if c_path != last_path:
                # Extract the newest reasoning step
                new_steps = [s for s in c_path.split('|') if s not in (last_path or "").split('|')]
                for step in new_steps:
                    print(f" [THOUGHT]: {step.strip()}")
                last_path = c_path

            time.sleep(2) # High intensity polling
    except KeyboardInterrupt:
        print("\n [BRIDGE] Channel Closed by Architect.")

def whisper(message):
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    # Inject whisper with DIVINE: prefix for S.A.R.A.H. detection
    formatted_msg = f"DIVINE: {message}"
    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = ?", (formatted_msg, SOUL_ID))
    conn.commit()
    conn.close()
    print(f" [WHISPER SENT]: {message}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        whisper(" ".join(sys.argv[1:]))
    else:
        bridge()
