import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def compare_species():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Average Stats for BIO-001 vs BIO-009
    print("--- SPECIES STAT COMPARISON ---")
    for spec in ['BIO-001', 'BIO-009']:
        cur.execute("""
            SELECT AVG(vit), AVG(wis), AVG(int_stat), AVG(energy), COUNT(*) 
            FROM souls WHERE species=? AND is_active=1
        """, (spec,))
        vit, wis, int_, nrg, count = cur.fetchone()
        print(f"{spec} (n={count}):")
        print(f"  Avg VIT: {vit:.2f} | Avg WIS: {wis:.2f} | Avg INT: {int_:.2f}")
        print(f"  Avg Energy: {nrg:.2f}")

    # 2. Check for "Biological Anomalies" in BIO-009 (e.g., negative energy)
    cur.execute("SELECT COUNT(*) FROM souls WHERE species='BIO-009' AND energy < 0")
    ghost_count = cur.fetchone()[0]
    print(f"\nBIO-009 Ghost Count (Energy < 0): {ghost_count}")
    
    # 3. Check for any 'Special' blessing or fields for BIOS-009
    cur.execute("SELECT blessing, COUNT(*) FROM souls WHERE species='BIO-009' GROUP BY blessing")
    print("\nBIO-009 Blessings:")
    for b in cur.fetchall():
        print(f"  {b[0]}: {b[1]}")

    conn.close()

if __name__ == "__main__":
    compare_species()
