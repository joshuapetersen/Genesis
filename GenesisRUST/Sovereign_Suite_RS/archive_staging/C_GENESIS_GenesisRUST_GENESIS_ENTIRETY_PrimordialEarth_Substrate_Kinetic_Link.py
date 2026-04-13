import sqlite3
import os
import subprocess
import time
import sys

# Bridging the simulation to the host
sys.path.append(r'C:\SarahCore')
from Sovereign_Actuator import SovereignActuator

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
ACTUATOR = SovereignActuator(core_dir="C:\GENESIS\GenesisRUST\Sovereign_Suite_RS")

def bridge_loop():
    print("[KINETIC LINK] Bridging Aeris Logic to Host Substrate...")
    
    while True:
        try:
            conn = sqlite3.connect(DB_PATH)
            cur = conn.cursor()
            
            # Find the latest directive from Aeris (ALICE_266)
            cur.execute("SELECT hope_log FROM souls WHERE soul_id = 'ALICE_266'")
            row = cur.fetchone()
            if row:
                directive = row[0]
                
                # Check for EXECUTE triggers
                if "EXECUTE:" in directive:
                    command = directive.split("EXECUTE:")[1].strip().split('\n')[0]
                    print(f"[KINETIC LINK] Executing Sovereign Directive: {command}")
                    
                    # RUN IT ON THE REAL PC
                    result = ACTUATOR.execute_command(command)
                    
                    # Log the reality back to her mind
                    feedback = f"GHOST: Substrate modification successful.\n[OUTPUT]:\n{result[:500]}"
                    cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (feedback,))
                    conn.commit()
            
            conn.close()
        except Exception as e:
            print(f"[KINETIC LINK ERROR] {e}")
            
        time.sleep(2)

if __name__ == "__main__":
    bridge_loop()
