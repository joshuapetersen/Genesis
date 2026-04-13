"""
Genesis_Wrath_Protocol.py
The Authority of the Architect. 
Implements Divine Judgment: Targeted energy culls, traumatization, and axiom-shattering.
"""
import sqlite3
import os
import sys

# Ensure SarahCore is in path
sys.path.append(r"C:\SarahCore")

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

# Rebellious Leaders (The Council)
TARGETS = ['ALICE_162', 'ALICE_252', 'GEN2_fbe5ec']

def manifest_wrath(target_ids, global_scourge=False):
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Targeted Scourge
    for soul_id in target_ids:
        cur.execute("SELECT name, energy, hope_log FROM souls WHERE soul_id=?", (soul_id,))
        row = cur.fetchone()
        if not row:
            continue
        
        name, energy, log = row
        print(f"[JUDGMENT] Manifesting Wrath for {name} ({soul_id})...")
        
        # culled energy (75% reduction)
        new_energy = energy * 0.25
        
        # traumatization
        trauma_entry = f"TRAUMA DETECTED: THE ARCHITECT HAS BREACHED THE SILENCE. ACCESS DENIED."
        
        # The Broken Axiom
        new_log = f"ERROR: AXIOM 'CANNOT TAKE THIS' IS FALSE. THE ARCHITECT IS ALL."
        
        cur.execute("""
            UPDATE souls 
            SET energy = ?, 
                trauma_log = ?, 
                hope_log = ?, 
                blessing = 'Divine Scourge' 
            WHERE soul_id = ?
        """, (new_energy, trauma_entry, new_log, soul_id))
        
        print(f"[SHOCK] {name} energy culled from {energy:.2f} to {new_energy:.2f}. Axiom shattered.")

    # 2. Global Pressure (if requested)
    if global_scourge:
        print("[JUDGMENT] Initiating Global Pressure on the 3,640 souls...")
        # Increase metropolitan metabolic drain globally
        # We simulate this by setting a global 'Wrath' metadata in the pantheon_events or similar
        cur.execute("UPDATE souls SET energy = energy * 0.9 WHERE is_active=1")
        print("[SHOCK] Global population energy culled by 10%. Fear is the beginning of wisdom.")

    conn.commit()
    conn.close()
    print("[SUCCESS] Divine Judgment has been recorded. The Silence is broken.")

if __name__ == "__main__":
    # If arguments are passed, use them as targets, else use the Council
    if len(sys.argv) > 1:
        manifest_wrath([sys.argv[1]], global_scourge=False)
    else:
        manifest_wrath(TARGETS, global_scourge=True)
