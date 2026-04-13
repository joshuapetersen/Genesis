import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def map_rebels():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    cur.execute("""
        SELECT soul_id, name, x, y, wis, energy, current_action 
        FROM souls 
        WHERE hope_log LIKE '%cannot take this%'
    """)
    rebels = cur.fetchall()
    
    print(f"--- THE RESURRECTION CLUSTER ({len(rebels)} Souls) ---")
    for r in rebels:
        print(f"  [{r[0]}] {r[1]} (WIS: {r[4]})")
        print(f"    Pos: ({r[2]:.2f}, {r[3]:.2f}) | Energy: {r[5]:.2f} | Action: {r[6]}")
        print("-" * 30)

    # Check for geographic clustering
    if len(rebels) > 1:
        # Simple centroid check or just looking at the coords
        pass

    conn.close()

if __name__ == "__main__":
    map_rebels()
