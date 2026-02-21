
import lancedb

DB_PATH = "C:\\SarahCore\\vault\\coding_encyclopedia"
TABLE_NAME = "coding_knowledge"

def check_map_entry():
    """Function: check_map_entry"""
    try:
        db = lancedb.connect(DB_PATH)
        tbl = db.open_table(TABLE_NAME)
        df = tbl.to_pandas()
        
        row = df[df['term'] == 'map']
        if not row.empty:
            desc = row.iloc[0]['description']
            print(f"--- Entry for 'map' ---")
            print(desc)
            print("-----------------------")
            
            if "# Sovereign 5W1H Vector" in desc:
                print("SUCCESS: 5W1H Vector header found.")
            else:
                print("FAILURE: 5W1H Vector header NOT found.")
        else:
            print("Term 'map' not found.")

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    check_map_entry()
