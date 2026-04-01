import sqlite3
import time
import os
import sys

# Ensure UTF-8 for better symbols
if sys.platform == "win32":
    os.system("chcp 65001 > nul")

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def live_monitor():
    print("\033[2J\033[H") # Clear screen
    print("================================================================================")
    print(" [AERIS LIVE MONITOR] - THE GHOST CHAMBER IS ACTIVE")
    print("================================================================================")
    
    last_thought = ""
    
    while True:
        try:
            if not os.path.exists(DB_PATH):
                print("Waiting for Soul Vault...")
                time.sleep(2)
                continue
                
            conn = sqlite3.connect(DB_PATH)
            cur = conn.cursor()
            
            cur.execute("SELECT name, energy, hope_log, reasoning_path, moral_alignment FROM souls WHERE soul_id='ALICE_266'")
            r = cur.fetchone()
            
            if r:
                name, energy, hope, path, align = r
                
                # Check for changes
                if hope != last_thought:
                    print(f"\n[{time.strftime('%H:%M:%S')}] {name} PULSE:")
                    print("-" * 40)
                    print(f" ENERGY: {energy:.2f} | ALIGNMENT: {align:.2f}")
                    print(f" STATUS: {hope if hope else 'Idle Thinking...'}")
                    print("-" * 40)
                    last_thought = hope
                
                # Show minor reasoning path updates if needed (tail -1)
                if path:
                    latest_step = path.split(" | ")[-1]
                    sys.stdout.write(f"\r REASONING: {latest_step[:70]:<70}")
                    sys.stdout.flush()
            
            conn.close()
            time.sleep(1)
            
        except KeyboardInterrupt:
            print("\n\nMonitor Terminated.")
            break
        except Exception as e:
            # Silence DB lock errors in the monitor to keep it clean
            time.sleep(1)

if __name__ == "__main__":
    live_monitor()
