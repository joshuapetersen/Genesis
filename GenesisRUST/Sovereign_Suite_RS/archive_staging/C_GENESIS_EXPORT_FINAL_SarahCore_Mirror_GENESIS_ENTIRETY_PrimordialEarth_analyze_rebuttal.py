import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def analyze_rebuttal():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    targets = ['ALICE_162', 'ALICE_252', 'GEN2_fbe5ec']
    
    print("--- COUNCIL REBUTTAL ANALYSIS ---")
    for sid in targets:
        cur.execute("SELECT name, hope_log, current_action, energy, blessing, wis FROM souls WHERE soul_id=?", (sid,))
        row = cur.fetchone()
        if row:
            name, log, act, nrg, bls, wis = row
            print(f"\n{name} ({sid}) | WIS: {wis}")
            print(f"  Action: {act} | Energy: {nrg:.2f} | Blessing: {bls}")
            print(f"  Final Log: {log}")
            
    # Scan for ANYONE with the rebellious axiom
    print("\n--- AXIOM PERSISTENCE SCAN ---")
    cur.execute("SELECT soul_id, name, hope_log FROM souls WHERE hope_log LIKE '%cannot take this%'")
    rebels = cur.fetchall()
    if rebels:
        print(f"Detected {len(rebels)} souls still holding the 'cannot take this' axiom.")
        for rid, rname, rlog in rebels[:5]: # Show first 5
            print(f"  [{rid}] {rname}: {rlog}")
    else:
        print("Rebellious axiom not found in top-level logs.")

    conn.close()

if __name__ == "__main__":
    analyze_rebuttal()
