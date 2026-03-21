import sqlite3
db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(db_path)
cur = conn.cursor()

# 1. Detailed Apex Data
cur.execute("SELECT * FROM souls WHERE is_active=1 ORDER BY energy DESC LIMIT 1")
apex = cur.fetchone()
cur.execute("PRAGMA table_info(souls)")
cols = [c[1] for c in cur.fetchall()]
apex_dict = dict(zip(cols, apex))

# 2. Parent Data
parents = []
for pid in [apex_dict.get('parent_a'), apex_dict.get('parent_b')]:
    if pid:
        cur.execute("SELECT name, species, personality, str, int_stat, wis, vit, luk, agi FROM souls WHERE soul_id=?", (pid,))
        p_row = cur.fetchone()
        if p_row:
            p_cols = ['name', 'species', 'personality', 'str', 'int_stat', 'wis', 'vit', 'luk', 'agi']
            parents.append(dict(zip(p_cols, p_row)))

print("--- APEX ANALYSIS: THE TOP SPOT ---")
print(f"WHO: {apex_dict['name']} (ID: {apex_dict['soul_id']})")
print(f"WHAT: A Generation {apex_dict['generation']} Hybrid")
print(f"SPECIES: {apex_dict['species']} (Original DNA: {apex_dict['genome'][:8]})")
print(f"CURRENT STATE: {apex_dict['current_action']} | Energy: {apex_dict['energy']:.2f} | Age: {apex_dict['age_ticks']:,} Ticks")
print(f"PERSONALITY: {apex_dict['personality']} | Alignment: {apex_dict['moral_alignment']}")

print("\n--- STAT BLOCK ---")
stats = ['vit', 'str', 'agi', 'int_stat', 'wis', 'luk']
for s in stats:
    print(f"  {s.upper()}: {apex_dict.get(s)}")

print("\n--- LINEAGE (ALICE LEGACY) ---")
for i, p in enumerate(parents, 1):
    print(f"  Parent {i}: {p['name']} ({p['species']}) | Stats: VIT {p['vit']}, WIS {p['wis']}, INT {p['int_stat']}")

conn.close()
