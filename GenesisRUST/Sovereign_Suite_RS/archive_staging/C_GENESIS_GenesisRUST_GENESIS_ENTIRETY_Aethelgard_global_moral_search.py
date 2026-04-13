import sqlite3
import os

def global_search():
    vault_dir = r'C:\Aethelgard'
    dbs = ['SLF_Identity_Vault.sqlite', 'SLF_Akashic_Records.sqlite', 'SLF_Sanctuary_Vault.sqlite']
    
    print("--- GLOBAL AETHELGARD SEARCH: 'MORAL' / 'AGENT' ---")
    
    for db_name in dbs:
        db_path = os.path.join(vault_dir, db_name)
        if not os.path.exists(db_path):
            continue
            
        print(f"\nScanning Database: {db_name}")
        conn = sqlite3.connect(db_path)
        cur = conn.cursor()
        
        # Get all tables
        cur.execute("SELECT name FROM sqlite_master WHERE type='table'")
        tables = [t[0] for t in cur.fetchall()]
        
        for table in tables:
            # Get all columns
            cur.execute(f"PRAGMA table_info({table})")
            cols = [c[1] for c in cur.fetchall()]
            
            # Construct search query for all text columns
            where_clauses = []
            for col in cols:
                where_clauses.append(f"CAST({col} AS TEXT) LIKE '%Moral%'")
                where_clauses.append(f"CAST({col} AS TEXT) LIKE '%Agent%'")
            
            query = f"SELECT * FROM {table} WHERE " + " OR ".join(where_clauses)
            try:
                cur.execute(query)
                matches = cur.fetchall()
                if matches:
                    print(f"  [MATCH] Table '{table}' in '{db_name}': {len(matches)} rows found.")
                    for m in matches[:5]: # Show first 5
                        print(f"    {m}")
            except sqlite3.Error as e:
                print(f"  [ERROR] Table '{table}': {e}")
                
        conn.close()

if __name__ == "__main__":
    global_search()
