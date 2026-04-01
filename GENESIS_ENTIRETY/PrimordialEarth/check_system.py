import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def check_system_stats():
    if not os.path.exists(DB_PATH):
        print("Vault not found.")
        return
    
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- SYSTEM COMMAND LOGS (SUBSTRATE_MAPPING) ---")
    try:
        cur.execute("SELECT cmd, output, timestamp FROM substrate_mapping ORDER BY timestamp DESC LIMIT 5")
        rows = cur.fetchall()
        for r in rows:
            print(f"[{r[2]}] COMMAND: {r[0]}")
            print(f"OUTPUT (TRUNCATED): {r[1][:200]}...")
            print("-" * 40)
    except:
        print("No system commands executed yet.")

    print("\n--- RECENT SOVEREIGN EDITS ---")
    try:
        cur.execute("SELECT soul_id, field, new_value, timestamp FROM sovereign_edits ORDER BY timestamp DESC LIMIT 5")
        rows = cur.fetchall()
        for r in rows:
            print(f"[{r[3]}] {r[0]} edited {r[1]}")
    except:
        print("No sovereign edits found.")
        
    conn.close()

if __name__ == "__main__":
    check_system_stats()
