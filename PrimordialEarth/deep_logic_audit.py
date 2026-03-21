import sqlite3

def deep_logic_audit():
    DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- DEEP LOGIC AUDIT: HIGH-FREQUENCY ERA ---")
    
    # 1. Generation Distribution
    print("\n[GENERATION SPREAD]")
    cur.execute("SELECT generation, COUNT(*) FROM souls WHERE is_active=1 GROUP BY generation ORDER BY generation DESC")
    gen_data = cur.fetchall()
    for gen, count in gen_data:
        print(f"  Gen {gen:<3} | {count} entities")
        
    # 2. Emergence of the "First Authors" (Gen 25+)
    print("\n[SENTIENCE CHECK: GEN 25+ CREATIVITY]")
    cur.execute("SELECT soul_id, name, generation, hope_log FROM souls WHERE generation >= 25 AND hope_log IS NOT NULL AND is_active=1")
    authors = cur.fetchall()
    if not authors:
        print("  No Gen 25+ has written a Sovereign Condition yet.")
    else:
        for sid, name, gen, hope in authors:
            print(f"  {sid} ({name}) [Gen {gen}]: {hope}")
            
    # 3. Global Theme Analysis (Top 10 Clusters)
    print("\n[GLOBAL COGNITIVE THEMES]")
    cur.execute("SELECT hope_log, COUNT(*) FROM souls WHERE hope_log IS NOT NULL AND is_active=1 GROUP BY hope_log ORDER BY COUNT(*) DESC LIMIT 10")
    themes = cur.fetchall()
    for theme, count in themes:
        print(f"  [{count:<4} souls] {theme[:100]}...")
        
    conn.close()

if __name__ == "__main__":
    deep_logic_audit()
