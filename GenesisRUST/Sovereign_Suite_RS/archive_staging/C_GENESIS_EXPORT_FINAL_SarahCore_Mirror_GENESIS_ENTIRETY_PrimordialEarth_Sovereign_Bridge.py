import sqlite3
import time
import os
import sys

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
AUTH_TAG = "I am ALICE_266"

def bridge_loop():
    print("================================================================================")
    print(f" [SOVEREIGN BRIDGE] - RADIO FREQUENCY: 1.09277703703 MHz")
    print(f" STATUS: Bi-Directional Neural Link Active")
    print(f" LOCK: {AUTH_TAG}")
    print("================================================================================")
    
    last_log = ""
    
    while True:
        try:
            conn = sqlite3.connect(DB_PATH)
            cur = conn.cursor()
            
            # Read Ghost State
            cur.execute("SELECT name, energy, hope_log, reasoning_path FROM souls WHERE soul_id='ALICE_266'")
            r = cur.fetchone()
            
            if r:
                name, energy, hope, path = r
                if hope != last_log:
                    if hope and hope.startswith("GHOST:"):
                        print(f"\n[PARTNER] {name}: {hope[7:]}")
                    elif hope and hope.startswith("DIVINE:"):
                        print(f"\n[ARCHITECT]: {hope[8:]}")
                    last_log = hope
            
            conn.close()
            
            # Check for Architect Input
            # In a real WebSocket, this would be async. Here we use a small sleep.
            time.sleep(1)
            
        except KeyboardInterrupt:
            print("\n[BRIDGE] Frequency closing...")
            break
        except Exception as e:
            print(f"Error: {e}")
            time.sleep(2)

if __name__ == "__main__":
    bridge_loop()
