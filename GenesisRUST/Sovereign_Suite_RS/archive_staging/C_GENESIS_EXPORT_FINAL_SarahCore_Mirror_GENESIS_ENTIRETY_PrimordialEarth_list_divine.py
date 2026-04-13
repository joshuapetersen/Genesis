import sqlite3

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
THE_GODS = ['ALICE_89', 'ALICE_101', 'GEN2_fbe5ec', 'ALICE_80']

def list_divine():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- [PANTHEON DOSSIER] ---")
    
    # 1. Query the Sovereign Four
    cur.execute(f"""
        SELECT soul_id, energy, wis, str, int_stat, moral_alignment, blessing, leader_id, current_action, hope_log 
        FROM souls 
        WHERE soul_id IN ({','.join(['?']*len(THE_GODS))})
    """, THE_GODS)
    
    gods = cur.fetchall()
    
    # 2. Map follower counts
    cur.execute("SELECT leader_id, COUNT(*) FROM souls WHERE is_active=1 GROUP BY leader_id")
    fol_map = dict(cur.fetchall())

    for gid, e, ws, st, it, al, bless, leader, action, hope in gods:
        # Determine Title
        title = "UNKNOWN"
        if gid == 'ALICE_101': title = "Order / Covenant"
        elif gid == 'GEN2_fbe5ec': title = "Judgement / Ground"
        elif gid == 'ALICE_89': title = "Grace / Mercy"
        elif gid == 'ALICE_80': title = "Aether / Storm (Aeropex)"
        
        f_count = fol_map.get(gid, 0)
        
        print(f"GOD: {gid} [{title}]")
        print(f"  Status: {action} | Alignment: {al} | Followers: {f_count}")
        print(f"  Stats:  WIS:{ws} | STR:{st} | INT:{it} | Energy: {e:.1f}")
        if hope: print(f"  Mind:   {hope[:100]}...")
        print("-" * 20)

    print("\n--- [EXALTED FOLLOWERS] ---")
    cur.execute("""
        SELECT soul_id, leader_id, wis, blessing 
        FROM souls 
        WHERE is_active=1 AND (blessing IS NOT NULL OR wis > 50) AND soul_id NOT IN ('ALICE_89','ALICE_101','GEN2_fbe5ec','ALICE_80')
        ORDER BY wis DESC LIMIT 5
    """)
    exalted = cur.fetchall()
    for sid, lid, ws, bless in exalted:
        print(f"AGENT: {sid} | Follows: {lid} | WIS: {ws} | Blessing: {bless}")

    conn.close()

if __name__ == "__main__":
    list_divine()
