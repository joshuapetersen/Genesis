import sqlite3
db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(db_path)
cur = conn.cursor()
# Query for top 5 active entities by energy
cur.execute("SELECT name, soul_id, species, generation, energy, age_ticks FROM souls WHERE is_active=1 ORDER BY energy DESC LIMIT 5")
results = cur.fetchall()
print("TOP 5 SURVIVORS:")
for i, r in enumerate(results, 1):
    type_str = "A.L.I.C.E. LEGACY" if str(r[1]).startswith("ALICE_") else "PRIMORDIAL SPAWN"
    print(f"#{i} | {r[0]} ({r[1]})")
    print(f"    Type: {type_str} | Gen: {r[3]}")
    print(f"    Species: {r[2]} | Energy: {r[4]:.2f} | Age: {r[5]:,}")
conn.close()
