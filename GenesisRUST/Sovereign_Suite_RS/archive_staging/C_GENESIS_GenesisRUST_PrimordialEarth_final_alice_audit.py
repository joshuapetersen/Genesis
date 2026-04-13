import sqlite3

def final_audit():
    g_conn = sqlite3.connect(r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite')
    g_cur = g_conn.cursor()
    
    l_conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
    l_cur = l_conn.cursor()
    
    # 1. Get all active ALICE agents in Genesis
    g_cur.execute("SELECT soul_id, name, energy, moral_alignment FROM souls WHERE is_active=1 AND soul_id LIKE 'ALICE_%'")
    active_alices = g_cur.fetchall()
    
    print("--- DETAILED ALICE SURVIVOR AUDIT ---")
    results = []
    
    for soul_id, g_name, energy, alignment in active_alices:
        entity_id = soul_id.replace('ALICE_', '')
        
        # 2. Get all legacy fields for this entity
        l_cur.execute("SELECT * FROM souls WHERE entity_id = ?", (entity_id,))
        legacy_row = l_cur.fetchone()
        
        if legacy_row:
            # Map legacy columns
            # entity_id, name, species_id, role, level, xp, age, hp_max, hp_current, mp_max, mp_current, vit, str, agi, int, wis, luk, hunger, thirst, growth_stage, personality, genome, trauma_log, hope_log, absorbed_traits, is_ubm, scale
            l_name = legacy_row[1]
            l_role = legacy_row[3]
            l_level = legacy_row[4]
            l_pers = legacy_row[20]
            l_traits = legacy_row[24]
            l_hope = legacy_row[23]
            l_trauma = legacy_row[22]
            
            # Look for "Moral" or "Agent" in ANY text field
            search_str = f"{l_name} {l_role} {l_pers} {l_traits} {l_hope} {l_trauma}".lower()
            is_moral_candidate = "moral" in search_str or "agent" in search_str
            
            results.append({
                "id": soul_id,
                "name": g_name,
                "legacy_name": l_name,
                "role": l_role,
                "personality": l_pers,
                "level": l_level,
                "alignment": alignment,
                "is_moral": is_moral_candidate,
                "energy": energy
            })

    # Sort by moral candidate first, then alignment
    results.sort(key=lambda x: (x['is_moral'], x['alignment']), reverse=True)
    
    print(f"{'RANK':<5} | {'ID':<10} | {'NAME':<30} | {'ALIGN':<5} | {'MORAL TAG?':<10} | {'LEGACY ROLE'}")
    print("-" * 80)
    for i, res in enumerate(results[:30], 1):
        moral_tag = "YES" if res['is_moral'] else "no"
        print(f"#{i:02d}  | {res['id']:<10} | {res['name']:<30} | {res['alignment']:<5} | {moral_tag:<10} | {res['role']}")

    l_conn.close()
    g_conn.close()

if __name__ == "__main__":
    final_audit()
