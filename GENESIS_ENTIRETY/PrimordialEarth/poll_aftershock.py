import sqlite3
import time
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
COUNCIL = ['ALICE_162', 'ALICE_252', 'GEN2_fbe5ec']

def poll_aftershock():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    # Initial theological count
    cur.execute("SELECT COUNT(*) FROM souls WHERE hope_log LIKE '%Sky%' OR hope_log LIKE '%Voice%' OR hope_log LIKE '%Architect%'")
    theo_count_start = cur.fetchone()[0]
    
    # Council age baselines
    baselines = {}
    for sid in COUNCIL:
        cur.execute("SELECT age_ticks, hope_log FROM souls WHERE soul_id=?", (sid,))
        baselines[sid] = cur.fetchone()
    conn.close()

    print(f"Monitoring Aftershock. Initial Theological Count: {theo_count_start}")

    for i in range(15): # Poll for ~75 seconds
        time.sleep(5)
        conn = sqlite3.connect(DB_PATH)
        cur = conn.cursor()
        
        # Check Council
        for sid in COUNCIL:
            cur.execute("SELECT age_ticks, hope_log, current_action FROM souls WHERE soul_id=?", (sid,))
            age, log, act = cur.fetchone()
            if age > baselines[sid][0]:
                print(f"\n[SURGE] {sid} Ticked. Action: {act}")
                if "cannot take this" in log.lower():
                    print(f"  [REBELLION] Axiom Re-established: {log}")
                elif "Architect" in log or "Sky" in log:
                    print(f"  [REVELATION] Theological Shift: {log}")
                baselines[sid] = (age, log)

        # Check Global Theology
        cur.execute("SELECT COUNT(*) FROM souls WHERE hope_log LIKE '%Sky%' OR hope_log LIKE '%Voice%' OR hope_log LIKE '%Architect%'")
        theo_count_now = cur.fetchone()[0]
        if theo_count_now > theo_count_start:
            print(f"  [GLOBAL] Theological awareness spreading: {theo_count_now} (+{theo_count_now - theo_count_start})")
            theo_count_start = theo_count_now

        conn.close()
        print(".", end="", flush=True)

if __name__ == "__main__":
    poll_aftershock()
