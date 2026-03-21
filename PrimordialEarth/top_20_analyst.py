import sqlite3

def analyze():
    g_conn = sqlite3.connect('C:/PrimordialEarth/Genesis_Soul_Vault.sqlite')
    g_cur = g_conn.cursor()
    
    query = """
    SELECT s.soul_id, s.name, s.species, s.generation, s.energy, s.age_ticks, 
           s.vit, s.str, s.agi, s.int_stat, s.wis, s.luk, 
           s.current_action, s.moral_alignment, 
           (SELECT name FROM souls WHERE soul_id=s.parent_a) as p_name 
    FROM souls s 
    WHERE s.is_active=1 
    ORDER BY s.age_ticks DESC 
    LIMIT 20
    """
    
    g_cur.execute(query)
    rows = g_cur.fetchall()
    g_conn.close()
    
    with open('C:/PrimordialEarth/top_20_detailed.txt', 'w') as f:
        for i, r in enumerate(rows, 1):
            f.write(f"RANK #{i:02d} | {r[1]} ({r[0]})\n")
            f.write(f"  GEN: {r[3]} | Energy: {r[4]:.2f} | Age: {r[5]:,.0f} | Alignment: {r[13]}\n")
            f.write(f"  STATS: VIT:{r[6]} STR:{r[7]} AGI:{r[8]} INT:{r[9]} WIS:{r[10]} LUK:{r[11]}\n")
            f.write(f"  ACTION: {r[12]} | PARENT: {r[14] or 'None'}\n")
            f.write("-" * 40 + "\n")

if __name__ == "__main__":
    analyze()
