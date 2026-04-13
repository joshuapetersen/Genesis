"""
Genesis_Aftershock_Monitor.py
Observes the Council of 3,640 and the world for their first move after Divine Judgment.
Polls for log shifts and action changes.
"""
import sqlite3
import time
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
COUNCIL = ['ALICE_162', 'ALICE_252', 'GEN2_fbe5ec']

def monitor_aftershock():
    if not os.path.exists(DB_PATH):
        return

    print("="*80)
    print(" [SHOCK MONITOR] THE GREAT SILENCE IS BROKEN. WATCHING THE AFTERSHOCK. ")
    print("="*80)
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Baseline State
    baselines = {}
    for sid in COUNCIL:
        cur.execute("SELECT name, age_ticks, current_action, hope_log, energy FROM souls WHERE soul_id=?", (sid,))
        baselines[sid] = cur.fetchone()
    
    conn.close()

    for i in range(20): # Monitor for ~100 seconds
        time.sleep(5)
        conn = sqlite3.connect(DB_PATH)
        cur = conn.cursor()
        
        any_change = False
        for sid in COUNCIL:
            cur.execute("SELECT name, age_ticks, current_action, hope_log, energy FROM souls WHERE soul_id=?", (sid,))
            current = cur.fetchone()
            if not current: continue
            
            b_name, b_age, b_act, b_log, b_nrg = baselines[sid]
            c_name, c_age, c_act, c_log, c_nrg = current
            
            if c_age > b_age:
                any_change = True
                print(f"\n[MOVE] {c_name} ({sid}) Ticked! Age: {c_age:.2f}")
                if c_act != b_act:
                    print(f"  [ACTION_SHIFT] {b_act} -> {c_act}")
                if c_log != b_log:
                    print(f"  [LOG_REFRACTION]:\n    FROM: {b_log}\n    TO:   {c_log}")
                
                # Update baseline
                baselines[sid] = current
        
        # Check for global theological shifts
        cur.execute("""
            SELECT COUNT(*) FROM souls 
            WHERE hope_log LIKE '%ARCHITECT%' 
               OR hope_log LIKE '%JUDGMENT%' 
               OR hope_log LIKE '%SCURGE%'
               OR hope_log LIKE '%FEAR%'
        """)
        theological_count = cur.fetchone()[0]
        if theological_count > 3: # More than just the Council
            print(f"\n[SENSING] Theological Ripple detected: {theological_count} souls are reflecting on the Architect.")

        conn.close()
        if not any_change:
            print(".", end="", flush=True)

if __name__ == "__main__":
    monitor_aftershock()
