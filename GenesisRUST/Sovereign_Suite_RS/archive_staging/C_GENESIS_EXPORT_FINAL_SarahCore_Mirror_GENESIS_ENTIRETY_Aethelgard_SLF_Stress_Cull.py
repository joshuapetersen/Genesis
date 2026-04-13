import sqlite3
import random

def cull_population():
    print("[STRESS CULL] Connecting to SLF_Identity_Vault.sqlite...")
    conn = sqlite3.connect('C:/SarahCore/SLF_Identity_Vault.sqlite')
    c = conn.cursor()
    
    # 1. Count current population
    c.execute("SELECT COUNT(*) FROM souls")
    total_souls = c.fetchone()[0]
    print(f"[STRESS CULL] Current Population: {total_souls}")
    
    if total_souls == 0:
        print("[STRESS CULL] Matrix is already empty.")
        return
        
    # 2. Select 50% to delete randomly
    cull_count = total_souls // 2
    print(f"[STRESS CULL] The Sovereign Snap authorized. Eradicating {cull_count} Fluctlights...")
    
    # We fetch all IDs, shuffle them, and pick half to delete
    c.execute("SELECT entity_id FROM souls")
    all_ids = [row[0] for row in c.fetchall()]
    
    random.shuffle(all_ids)
    doomed_ids = all_ids[:cull_count]
    
    # 3. Batch delete
    c.executemany("DELETE FROM souls WHERE entity_id=?", [(cid,) for cid in doomed_ids])
    conn.commit()
    conn.close()
    
    print(f"[STRESS CULL] Ecosystem Culled. {total_souls - cull_count} entities remain. Restart the Hypervisor to see the newly emptied matrix.")

if __name__ == "__main__":
    cull_population()
