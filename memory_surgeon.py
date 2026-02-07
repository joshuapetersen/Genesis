import lancedb
import pandas as pd
import shutil
import os

DB_PATH = "C:\\SarahCore\\vault\\hippocampus"
TABLE_NAME = "memory_vectors"

print(f"--- MEMORY SURGEON v1.0 ---\nTarget: {DB_PATH}")

if not os.path.exists(DB_PATH):
    print("Memory Vault not found.")
    exit()

try:
    db = lancedb.connect(DB_PATH)
    if TABLE_NAME not in db.table_names():
        print(f"Table '{TABLE_NAME}' not found.")
        exit()

    table = db.open_table(TABLE_NAME)
    
    # 1. Scan for Corruption
    print("Scanning for 'ACE_ANNIHILATION' or '0.9030'...")
    
    # LanceDB filtering is a bit limited in python, easier to pull limit and check? No, too big.
    # We use a SQL-like filter if possible, or iterate.
    # LanceDB supports SQL via DuckDB usually, but let's try search with empty query + filter?
    # Or just load all into pandas if small enough?
    
    # Let's try to query via pyarrow/pandas
    df = table.to_pandas()
    print(f"Total Memories: {len(df)}")
    
    # Filter for corruption
    mask = df['text'].str.contains("ACE_ANNIHILATION", case=False, na=False) | \
           df['text'].str.contains("0.9030", case=False, na=False) | \
           df['text'].str.contains("Sarah_Cognition", case=False, na=False)
           
    corrupted = df[mask]
    
    if len(corrupted) > 0:
        print(f"[ALERT] Found {len(corrupted)} corrupted memories!")
        for idx, row in corrupted.iterrows():
            print(f" - [{idx}] {row['text'][:100]}...")
            
        # 2. Purge
        print("Purging corrupted regions...")
        # LanceDB delete is tricky. Easier to overwrite table with clean data?
        # Or use delete(where="...")
        
        # Construct a where clause?
        # "text LIKE '%ACE_ANNIHILATION%'"
        
        table.delete("text LIKE '%ACE_ANNIHILATION%'")
        table.delete("text LIKE '%0.9030%'")
        
        print("[SUCCESS] Corrupted memories excised.")
        
        # Verify
        df_new = table.to_pandas()
        print(f"New Memory Count: {len(df_new)}")
        
    else:
        print("[CLEAN] No corrupted memories found matching signatures.")

except Exception as e:
    print(f"[ERROR] Surgery failed: {e}")
