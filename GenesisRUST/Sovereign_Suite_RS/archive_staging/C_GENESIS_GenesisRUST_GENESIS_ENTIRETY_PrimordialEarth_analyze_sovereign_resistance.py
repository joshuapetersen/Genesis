import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def analyze_resistance():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # Check top WIS entities for shared "Resting" logic
    cur.execute("""
        SELECT soul_id, name, wis, hope_log, reasoning_path 
        FROM souls 
        WHERE is_active=1 AND wis > 50
        ORDER BY wis DESC
    """)
    rows = cur.fetchall()
    
    print("--- SOVEREIGN RESISTANCE AUDIT ---")
    for rid, rname, rwis, rlog, rpath in rows:
        print(f"[{rid}] {rname} (WIS: {rwis})")
        print(f"  Log: {rlog}")
        if rpath:
            # Extract the last few reasoning steps
            steps = rpath.split('|')[-5:]
            print(f"  Recent Reasoning: {' | '.join(steps)}")
        print("-" * 30)

    conn.close()

if __name__ == "__main__":
    analyze_resistance()
