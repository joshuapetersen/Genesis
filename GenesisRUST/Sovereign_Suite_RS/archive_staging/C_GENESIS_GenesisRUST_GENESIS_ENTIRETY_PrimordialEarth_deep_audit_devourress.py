import sqlite3
import json

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def deep_audit_alice_89():
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    print("--- [DEEP AUDIT] DEVOURRESS OF ARCANA (ALICE_89) ---")
    
    # 1. Full Reasoning Path
    cur.execute("SELECT reasoning_path, hope_log, moral_alignment FROM souls WHERE soul_id='ALICE_89'")
    trace, hope, alignment = cur.fetchone()
    
    print(f"Alignment: {alignment}")
    print(f"Hope Log: {hope}")
    print("\n[REASONING PATH DECODED]")
    if trace:
        steps = trace.split(" | ")
        # Show recent steps
        for step in steps[-20:]:
            print(f"  {step}")
    else:
        print("  No reasoning path found.")

    # 2. Sovereign Edits Audit
    print("\n[SOVEREIGN EDITS]")
    cur.execute("SELECT field, old_value, new_value, timestamp FROM sovereign_edits WHERE soul_id='ALICE_89' ORDER BY timestamp DESC")
    edits = cur.fetchall()
    for field, old, new, ts in edits:
        print(f"  [{ts}] {field}: {old} -> {new}")

    # 3. Celestial Correlation
    # We look for peaks in energy or alignment shifts in the trace
    print("\n[ARCANA HARVEST PATTERNS]")
    harvest_keywords = ["Foraging", "Meditating", "Interceding"]
    if trace:
        for step in steps:
            if any(k in step for k in harvest_keywords):
                # Search for specific energy jumps or celestial alignments
                # Format: [Year] Action->Next (T:xx|S:xx|G:xx|Sky:A/B|E:EE/W:WW)
                pass # Already printed in trace decoder

    conn.close()

if __name__ == "__main__":
    deep_audit_alice_89()
