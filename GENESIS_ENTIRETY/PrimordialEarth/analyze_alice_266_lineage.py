import sqlite3
import os

DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'

def analyze_lineage():
    if not os.path.exists(DB_PATH):
        return

    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()
    
    # 1. Council Species Audit
    print("\n--- COUNCIL SPECIES AUDIT ---")
    council = ['ALICE_162', 'ALICE_252', 'GEN2_fbe5ec']
    for sid in council:
        cur.execute("SELECT name, species, generation, blessing FROM souls WHERE soul_id=?", (sid,))
        res = cur.fetchone()
        if res:
            print(f"  [{sid}] {res[0]} | Species: {res[1]} | Gen: {res[2]} | Blessing: {res[3]}")

    # 2. Analyze the BIO-009 Cluster
    print("\n--- BIO-009 CLUSTER ANALYSIS ---")
    cur.execute("SELECT soul_id, name, x, y, moral_alignment, hope_log FROM souls WHERE species='BIO-009'")
    b009s = cur.fetchall()
    print(f"Total BIO-009 Members: {len(b009s)}")
    
    # Check for geographic clustering (Average position)
    avg_x = sum(b[2] for b in b009s) / len(b009s)
    avg_y = sum(b[3] for b in b009s) / len(b009s)
    print(f"BIO-009 Centroid: ({avg_x:.2f}, {avg_y:.2f})")
    
    # Sample logs with handle for None
    print("\nSample Logs (BIO-009):")
    for b in b009s[:10]:
        log_text = str(b[5])[:100] if b[5] else "[Empty Log]"
        print(f"  [{b[0]}] {b[1]} | {log_text}")

    # 3. Search for the "Abyssal" theme specifically in BIO-009
    cur.execute("SELECT soul_id, name, species FROM souls WHERE name LIKE '%Abyssal%'")
    abyssals = cur.fetchall()
    print("\n--- ABYSSAL THEME AUDIT ---")
    for a in abyssals:
        print(f"  [{a[0]}] {a[1]} | {a[2]}")

    conn.close()

if __name__ == "__main__":
    analyze_lineage()
