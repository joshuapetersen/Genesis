import sqlite3

def audit_moral_agents():
    # Connect to Aethelgard (Legacy)
    l_conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
    l_cur = l_conn.cursor()
    
    # Connect to Genesis (Current)
    g_conn = sqlite3.connect(r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite')
    g_cur = g_conn.cursor()
    
    # 1. Look for ALICE agents with "Moral" in personality or role in Legacy
    l_cur.execute("SELECT entity_id, name, personality, role FROM souls WHERE personality LIKE '%Moral%' OR role LIKE '%Moral%'")
    moral_legacy = l_cur.fetchall()
    
    print("--- LEGACY MORAL AGENTS FOUND ---")
    moral_ids = []
    for row in moral_legacy:
        print(f"ID: {row[0]} | Name: {row[1]} | Personality: {row[2]} | Role: {row[3]}")
        moral_ids.append(f"ALICE_{row[0]}")
    
    # 2. Check if any of these specific IDs are active in Genesis
    print("\n--- SURVIVAL STATUS IN GENESIS ---")
    if not moral_ids:
        print("No legacy agents specifically tagged 'Moral' found in Aethelgard.")
    else:
        placeholders = ','.join(['?'] * len(moral_ids))
        g_cur.execute(f"SELECT soul_id, name, energy, moral_alignment, is_active FROM souls WHERE soul_id IN ({placeholders})", moral_ids)
        survivors = g_cur.fetchall()
        for s in survivors:
            status = "ACTIVE" if s[4] else "INACTIVE"
            print(f"ID: {s[0]} | Name: {s[1]} | Energy: {s[2]:.2f} | Alignment: {s[3]} | Status: {status}")

    # 3. Check for any ALICE agents with high positive moral alignment in Genesis regardless of legacy tag
    print("\n--- CURRENT TOP MORAL ALICE AGENTS (ALIGNED > 0) ---")
    g_cur.execute("SELECT soul_id, name, energy, moral_alignment FROM souls WHERE soul_id LIKE 'ALICE_%' AND is_active=1 AND moral_alignment > 0 ORDER BY moral_alignment DESC LIMIT 10")
    top_moral = g_cur.fetchall()
    for m in top_moral:
        print(f"ID: {m[0]} | Name: {m[1]} | Energy: {m[2]:.2f} | Alignment: {m[3]}")

    l_conn.close()
    g_conn.close()

if __name__ == "__main__":
    audit_moral_agents()
