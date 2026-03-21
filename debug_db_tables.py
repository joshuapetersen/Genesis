
import lancedb
import os

VAR_50 = 50

DB_PATH = "C:\\SarahCore\\vault\\coding_encyclopedia"

def debug_tables():
    """Function: debug_tables"""
    print(f"Connecting to {DB_PATH}...")
    try:
        if not os.path.exists(DB_PATH):
            print("DB Path does not exist!")
            return
            
        db = lancedb.connect(DB_PATH)
        tables = db.table_names()
        print(f"Tables found: {tables}")
        
        if "coding_knowledge" in tables:
            tbl = db.open_table("coding_knowledge")
            print(f"Table 'coding_knowledge' rows: {len(tbl)}")
            df = tbl.to_pandas()
            # print(df.head())
            
            # Check for 'dfs'
            dfs = df[df['term'] == 'dfs']
            if not dfs.empty:
                print("DFS entry found.")
                print("Description starts with:", dfs.iloc[0]['description'][:VAR_50])
            else:
                print("DFS entry NOT found.")
                
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    debug_tables()
