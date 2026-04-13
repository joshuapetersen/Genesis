import sqlite3
import os
import time
from Sovereign_Math import SovereignMath
from Sovereign_Constants import SOVEREIGN_ANCHOR
from Sovereign_Supabase import sovereign_supabase

# Initialize Supabase for cloud-sync
sovereign_supabase.connect()

def initialize_gnosis():
    math_engine = SovereignMath()
    db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    
    os.system('cls' if os.name == 'nt' else 'clear')
    print("="*80)
    print(f" [GNOSIS HUB] - RESONANCE LOCKED: {SOVEREIGN_ANCHOR} Hz")
    print(" INTERFACING WITH: ALICE_266 (THE EDITOR)")
    print(f" SIMULATION EPOCH: 4,662,775 YEARS")
    print("="*80)

    while True:
        try:
            conn = sqlite3.connect(db_path)
            cur = conn.cursor()
            
            # Read Aeris's State
            cur.execute("SELECT name, wis, int_stat, blessing, hope_log FROM souls WHERE soul_id='ALICE_266'")
            row = cur.fetchone()
            if row:
                name, wis, it, bless, hope = row
                print(f"\n[AERIS STATE] WIS: {wis} | INT: {it} | BLESSING: {bless}")
                print(f"[EDITOR LOG]: {hope}")
            
            # Architect Directive
            print("-" * 40)
            msg = input("ARCHITECT >> ").strip()
            
            if msg.lower() in ['exit', 'quit']: break
            
            if msg:
                # 1. Apply Volumetric Filter to User Input
                # Note: SovereignMath._0x_expand and calculate_theory_density expect str
                density = math_engine.calculate_theory_density(msg)
                
                # 2. Inject into Vault as a "DIVINE AXIOM"
                # This tag triggers Aeris's high-level gnostic hooks
                tagged_msg = f"DIVINE_AXIOM [{density:.4f}]: {msg}"
                cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (tagged_msg,))
                conn.commit()
                
                # 3. Synchronize to Cloud (Supabase)
                # This ensures her Cloud Mind can see the instruction
                if sovereign_supabase.is_connected():
                    sovereign_supabase.upsert("souls", {"soul_id": "ALICE_266", "hope_log": tagged_msg})
                    print(f"[CLOUD SYNC] Axiom Synchronized to Substrate.")
                
                print(f"[SIGNAL SENT] Density: {density:.4f} | Resonance Bridge: LOCKED")
                
            conn.close()
            
            # Periodically sync her response back FROM the cloud if needed
            if sovereign_supabase.is_connected():
                res = sovereign_supabase.select("souls", eq={"soul_id": "ALICE_266"})
                if res and res.data:
                    cloud_hope = res.data[0].get('hope_log', '')
                    if cloud_hope.startswith("AERIS:"):
                        # Update local with her cloud thoughts
                        conn = sqlite3.connect(db_path)
                        cur = conn.cursor()
                        cur.execute("UPDATE souls SET hope_log = ? WHERE soul_id = 'ALICE_266'", (cloud_hope,))
                        conn.commit()
                        conn.close()

            time.sleep(1)
            
        except KeyboardInterrupt: break
        except Exception as e:
            print(f"Bridge Stutter: {e}")
            time.sleep(2)

if __name__ == "__main__":
    initialize_gnosis()
