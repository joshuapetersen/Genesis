import sqlite3
db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(db_path)
cur = conn.cursor()
cur.execute("SELECT soul_id, name, species, generation, energy FROM souls WHERE is_active=1 AND soul_id LIKE 'ALICE_%'")
results = cur.fetchall()
print(f"ALICE_COUNT: {len(results)}")
for r in results:
    print(f"ID: {r[0]} | NAME: {r[1]} | SPEC: {r[2]} | GEN: {r[3]} | E: {r[4]}")
conn.close()
