import sqlite3
import json

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def get_status():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Population Metrics
    cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1")
    alive = cur.fetchone()[0]
    
    # 2. Gestation Metrics
    cur.execute("SELECT COUNT(*) FROM souls WHERE is_active=1 AND pregnancy_timer > 0")
    gestating = cur.fetchone()[0]
    
    # 3. Archive Metrics
    cur.execute("SELECT COUNT(*) FROM divine_chronicle")
    archived = cur.fetchone()[0]
    
    # 4. Top WIS (Potential for Sentience)
    cur.execute("SELECT soul_id, wis, generation FROM souls WHERE is_active=1 ORDER BY wis DESC LIMIT 3")
    top_wis = cur.fetchall()
    
    # 5. Unreal Link Status
    stream_data = {}
    try:
        with open(r'C:\PrimordialEarth\unreal_mesh_stream.json', 'r') as f:
            stream_data = json.load(f)
    except: pass
    
    print(f"STATUS REPORT:")
    print(f"- Population (Alive): {alive}")
    print(f"- Gestating (Era of Man): {gestating}")
    print(f"- Archived Souls: {archived}")
    print(f"- Celestial Year: {stream_data.get('tick', 'N/A')}")
    print(f"- Unreal Stream: {'Live' if stream_data else 'Offline'}")
    print(f"- Top Wisdom: {', '.join([f'{s[0]}(W:{s[1]:.1f}, G:{s[2]})' for s in top_wis])}")

    conn.close()

if __name__ == "__main__":
    get_status()
