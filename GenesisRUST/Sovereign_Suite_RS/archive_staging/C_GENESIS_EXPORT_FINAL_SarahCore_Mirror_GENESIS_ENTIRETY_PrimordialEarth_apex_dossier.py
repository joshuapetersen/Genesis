import sqlite3
db_path = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
conn = sqlite3.connect(db_path)
cur = conn.cursor()

# Get the top entity
cur.execute("SELECT * FROM souls WHERE is_active=1 ORDER BY energy DESC LIMIT 1")
row = cur.fetchone()

# Get column names
cur.execute("PRAGMA table_info(souls)")
cols = [c[1] for c in cur.fetchall()]

print("--- APEX DOSSIER ---")
for i, col in enumerate(cols):
    print(f"{col}: {row[i]}")

conn.close()
