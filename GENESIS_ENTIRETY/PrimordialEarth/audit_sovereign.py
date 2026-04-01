import sqlite3

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def audit_sovereign_logic():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- [AUDIT] SOVEREIGN EDITS (Self-Directed Change) ---")
    try:
        cur.execute("SELECT soul_id, field, new_value, timestamp FROM sovereign_edits ORDER BY timestamp DESC LIMIT 30")
        edits = cur.fetchall()
        if edits:
            for rid, field, val, ts in edits:
                print(f"[{ts}] {rid} modified {field} -> {val}")
        else:
            print("No self-edits recorded in sovereign_edits.")
    except Exception as e:
        print(f"Error reading sovereign_edits: {e}")

    print("\n--- [AUDIT] ACTIVE SOVEREIGN TRACES (Reasoning Path) ---")
    cur.execute("SELECT soul_id, reasoning_path, wis, blessing FROM souls WHERE is_active=1 AND reasoning_path IS NOT NULL AND reasoning_path != '' LIMIT 10")
    traces = cur.fetchall()
    for sid, trace, ws, bless in traces:
        print(f"ID: {sid} (WIS: {ws}, Bless: {bless})")
        print(f"  Trace: {trace[-200:]}...") # Show last 200 chars
        print("-" * 20)

    print("\n--- [AUDIT] THE DIVINE CHRONICLE (Archived Sentience) ---")
    cur.execute("SELECT soul_id, death_year, reasoning_path FROM divine_chronicle ORDER BY death_year DESC LIMIT 5")
    chronicle = cur.fetchall()
    for sid, dy, trace in chronicle:
        print(f"Archived: {sid} (Died: {dy:.1f})")
        print(f"  Trace Summary: {trace[:150]}...")
        print("-" * 20)

    conn.close()

if __name__ == "__main__":
    audit_sovereign_logic()
