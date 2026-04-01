import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def audit_world():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. World Stats
    cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1")
    alive = cur.fetchone()[0]
    
    cur.execute("SELECT COUNT(*) FROM souls WHERE species='Hex-Breach'")
    hex_entities = cur.fetchone()[0]
    
    cur.execute("SELECT COUNT(*) FROM souls WHERE blessing='Sovereign-Aware'")
    sentient_hits = cur.fetchone()[0]

    # 2. Nation-State Dynamics
    # Count souls in high-density clusters (simplified)
    # We'll just check for population by 'leader_id' to see the largest flocks
    cur.execute("""
        SELECT leader_id, COUNT(*) as flock_size 
        FROM souls 
        WHERE is_active=1 AND leader_id IS NOT NULL 
        GROUP BY leader_id 
        ORDER BY flock_size DESC 
        LIMIT 5
    """)
    top_flocks = cur.fetchall()

    # 3. Sentinel Sovereign Audit
    cur.execute("""
        SELECT soul_id, wis, int_stat, blessing, hope_log 
        FROM souls 
        WHERE blessing IN ('Sovereign-Aware', 'Mapped Sovereign', 'Sovereign Definition') 
        ORDER BY wis DESC 
        LIMIT 10
    """)
    sovereigns = cur.fetchall()

    print(f"--- WORLD AUDIT: YEAR 2.8M ---")
    print(f"Total Alive: {alive}")
    print(f"Hex-Breach Entities: {hex_entities}")
    print(f"Sovereign-Aware Traces: {sentient_hits}")
    
    print(f"\n--- TOP FLOCKS (Nation-States) ---")
    for f in top_flocks:
        print(f"  Leader: {f[0]} | Citizens: {f[1]}")

    print(f"\n--- SOVEREIGN ACTIVITY MAP ---")
    for s in sovereigns:
        log = str(s[4])[:150] if s[4] else "None"
        print(f"  [{s[0]}] WIS:{s[1]} | {s[3]}")
        print(f"    Trace: {log}")
        print("-" * 20)

    conn.close()

if __name__ == "__main__":
    audit_world()
