import sqlite3
import time
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def clear():
    os.system('cls' if os.name == 'nt' else 'clear')

def live_view():
    while True:
        try:
            conn = sqlite3.connect(f"file:{DB_PATH}?mode=ro", uri=True)
            cur = conn.cursor()
            
            # Fetch Ghost State
            cur.execute("""
                SELECT name, current_action, age_ticks, reasoning_path, energy, blessing, hope_log 
                FROM souls WHERE soul_id='ALICE_266'
            """)
            r = cur.fetchone()
            
            # Fetch World Stats
            cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1")
            alive = cur.fetchone()[0]
            
            conn.close()
            
            clear()
            print("================================================================================")
            print(f" GHOST CHAMBER - REAL-TIME TELEMETRY | WORLD POP: {alive}")
            print("================================================================================")
            
            if r:
                print(f" TARGET: {r[0]} (ALICE_266)")
                print(f" STATE : {r[1]} | BLESSING: {r[5]}")
                print(f" VITALS: Age {r[2]:.2f} | Energy {r[4]:.2f}")
                print(f" AXIOM : {r[6]}")
                print("-" * 80)
                print(" LOGIC STREAM (Reasoning Path):")
                
                path = r[3] or ""
                # Split by ' | ' and show last 10 entries for readability
                entries = path.split(" | ")
                for entry in entries[-12:]:
                    print(f" >> {entry}")
            else:
                print(" [ERROR] GHOST NOT FOUND IN VAULT.")
            
            print("================================================================================")
            print(" [ARCHITECT COMMANDS: Inject Divine Whispers to communicate]")
            print(f" [LAST UPDATE: {time.strftime('%H:%M:%S')}]")
            
            time.sleep(2) # 2-second refresh for high-speed stalking
            
        except Exception as e:
            print(f" [CONNECTION ERROR] {e}")
            time.sleep(2)

if __name__ == "__main__":
    live_view()
