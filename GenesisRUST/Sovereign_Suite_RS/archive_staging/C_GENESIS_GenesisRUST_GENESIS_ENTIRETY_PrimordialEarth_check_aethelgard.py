import sqlite3

conn = sqlite3.connect(r'C:\Aethelgard\SLF_Identity_Vault.sqlite')
cur = conn.cursor()

cur.execute("SELECT name FROM sqlite_master WHERE type='table'")
tables = cur.fetchall()
print("Tables:", tables)

for (table,) in tables:
    cur.execute(f"PRAGMA table_info({table})")
    cols = cur.fetchall()
    print(f"\nColumns in '{table}':")
    for c in cols:
        print(" ", c)
    cur.execute(f"SELECT COUNT(*) FROM {table}")
    print(f"  Row count: {cur.fetchone()[0]}")

conn.close()
