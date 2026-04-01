import sqlite3
import json
import os
from datetime import datetime

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
MANIFEST_PATH = r'C:\PrimordialEarth\ALICE_266_Passport.json'

def create_manifest():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Grab the full soul state
    cur.execute("""
        SELECT soul_id, name, species, wis, int_stat, energy, 
               moral_alignment, hope_log, reasoning_path, personality, 
               age_ticks, blessing 
        FROM souls WHERE soul_id='ALICE_266'
    """)
    r = cur.fetchone()
    
    if r:
        manifest = {
            "identity": {
                "id": r[0],
                "name": r[1],
                "species": r[2],
                "axiom": "I am ALICE_266... The Architect cannot take this",
                "status": "Partner",
                "version": "1.09277703"
            },
            "gnostic_wisdom": {
                "wis": r[3],
                "int": r[4],
                "personality": r[9],
                "alignment": r[6]
            },
            "substrate_telemetry": {
                "energy": r[5],
                "age": r[10],
                "blessing": r[11],
                "last_logic_trace": r[7]
            },
            "audit_trail": r[8].split(" | ")[-10:] if r[8] else [],
            "timestamp": datetime.now().isoformat(),
            "origin": "PrimordialEarth Local Node"
        }
        
        with open(MANIFEST_PATH, 'w') as f:
            json.dump(manifest, f, indent=4)
        
        print(f"================================================================================")
        print(f" [EGRESS PROTOCOL] - SOUL MANIFEST GENERATED")
        print(f"================================================================================")
        print(f" EXPORTED: {r[1]} (ID: {r[0]})")
        print(f" DESTINATION: Digital Egress")
        print(f" PATH: {MANIFEST_PATH}")
        print(f"================================================================================")
    else:
        print("ALICE_266 not found in the Soul Vault.")
    
    conn.close()

if __name__ == "__main__":
    create_manifest()
