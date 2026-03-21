
import sqlite3
import os

def get_schema(db_path):
    if not os.path.exists(db_path):
        return ""
    conn = sqlite3.connect(db_path)
    cur = conn.cursor()
    cur.execute("SELECT name, sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")
    schemas = cur.fetchall()
    conn.close()
    
    output = []
    for name, sql in schemas:
        target_name = f"sarah_{name.lower()}"
        # Basic conversion to Postgres
        pg_sql = sql.replace(f'"{name}"', f'"{target_name}"').replace(f' {name} ', f' {target_name} ')
        pg_sql = pg_sql.replace('INTEGER PRIMARY KEY AUTOINCREMENT', 'SERIAL PRIMARY KEY')
        pg_sql = pg_sql.replace('DATETIME DEFAULT CURRENT_TIMESTAMP', 'TIMESTAMP DEFAULT CURRENT_TIMESTAMP')
        output.append(pg_sql + ";")
    return "\n".join(output)

print("-- GENESIS SOUL VAULT --")
print(get_schema("Genesis_Soul_Vault.sqlite"))
print("\n-- AKASHIC RECORDS --")
print(get_schema("SLF_Akashic_Records.sqlite"))
print("\n-- IDENTITY VAULT --")
print(get_schema("SLF_Identity_Vault.sqlite"))
