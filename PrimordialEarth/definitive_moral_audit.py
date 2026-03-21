import sqlite3

def definitive_moral_audit():
    g_conn = sqlite3.connect(r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite')
    g_cur = g_conn.cursor()
    
    l_conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
    l_cur = l_conn.cursor()
    
    # 1. Get all entities in Aethelgard with the "[A.L.I.C.E." trait
    l_cur.execute("SELECT entity_id, name, level, role, absorbed_traits FROM souls WHERE absorbed_traits LIKE '%[A.L.I.C.E.%'")
    moral_agents = l_cur.fetchall()
    
    print(f"--- DEFINITIVE MORAL AGENT AUDIT (Total Found in Legacy: {len(moral_agents)}) ---")
    
    results = []
    
    for eid, name, level, role, traits in moral_agents:
        soul_id = f"ALICE_{eid}"
        
        # 3. Check survival in Genesis
        g_cur.execute("SELECT name, energy, moral_alignment, is_active FROM souls WHERE soul_id = ?", (soul_id,))
        g_row = g_cur.fetchone()
        
        if g_row:
            name_gen, energy, alignment, active = g_row
            status = "ACTIVE" if active else "INACTIVE"
            results.append({
                "id": soul_id,
                "legacy_name": name,
                "current_name": name_gen,
                "energy": energy,
                "alignment": alignment,
                "status": status,
                "legacy_role": role,
                "legacy_level": level
            })

    # Sort by Energy (Darwinian Fitness for the user)
    results.sort(key=lambda x: x['energy'], reverse=True)
    
    print(f"{'RANK':<5} | {'ID':<10} | {'NAME (CURRENT)':<30} | {'ENERGY':<8} | {'STATUS'} | {'LEGACY NAME'}")
    print("-" * 100)
    for i, res in enumerate(results, 1):
        print(f"#{i:02d}  | {res['id']:<10} | {res['current_name']:<30} | {res['energy']:<8.2f} | {res['status']:<8} | {res['legacy_name']}")

    l_conn.close()
    g_conn.close()

if __name__ == "__main__":
    definitive_moral_audit()
