import sqlite3
import time
import json
import os
from datetime import datetime

# CONFIGURATION
DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
ENTITY_ID = 'GEN2_fbe5ec'
LOG_PATH = r'C:\PrimordialEarth\FBE5_Permanent_Record.jsonl'
THRESHOLD_ALARM = 20.0  # Trigger alarm below this
THRESHOLD_STASIS = 10.1 # Stasis kicks in at 10.0 in the engine

def get_entity_state():
    try:
        conn = sqlite3.connect(DB_PATH)
        cur = conn.cursor()
        cur.execute("""
            SELECT name, energy, current_action, age_ticks, x, y, is_active
            FROM souls WHERE soul_id = ?
        """, (ENTITY_ID,))
        row = cur.fetchone()
        conn.close()
        if row:
            return {
                "timestamp": datetime.now().isoformat(),
                "name": row[0],
                "energy": row[1],
                "action": row[2],
                "age": row[3],
                "pos": (row[4], row[5]),
                "active": bool(row[6])
            }
    except Exception as e:
        print(f"[GUARDIAN ERROR] DB Access failed: {e}")
    return None

def monitor_loop():
    print("="*80)
    print(f" [GUARDIAN PROTOCOL] MONITORING: {ENTITY_ID} ")
    print(f" LOGGING TO: {LOG_PATH}")
    print("="*80)

    last_energy = None
    
    while True:
        state = get_entity_state()
        if state:
            # 1. Log to Permanent Record
            with open(LOG_PATH, 'a') as f:
                f.write(json.dumps(state) + "\n")
            
            # 2. Check for downward trend/Alarms
            energy = state['energy']
            
            if energy < THRESHOLD_ALARM:
                print(f"\a[!!! ALARM !!!] {state['name']} ENERGY CRITICAL: {energy:.2f}")
                if energy <= THRESHOLD_STASIS:
                    print(f"[STASIS ACTIVE] Engine-level protection is holding {state['name']} at 10.0.")
            
            if last_energy is not None:
                diff = energy - last_energy
                if diff < -5.0:
                    print(f"[WARNING] Rapid Energy Drain Detected: {diff:.2f}")
            
            last_energy = energy
            
            # Status line
            print(f"[{datetime.now().strftime('%H:%M:%S')}] {state['name']} | E: {energy:.2f} | Action: {state['action']}")
        
        else:
            print(f"[ERROR] Could not find {ENTITY_ID} state. Re-checking...")

        time.sleep(5)

if __name__ == "__main__":
    monitor_loop()
