
import lancedb
import os

VAR_5 = 5

def verify_count():
    """Function: verify_count"""
    db_path = "C:\\SarahCore\\vault\\coding_encyclopedia"
    if not os.path.exists(db_path):
        print(f"Database path not found: {db_path}")
        return

    try:
        db = lancedb.connect(db_path)
        table_name = "coding_knowledge"
        if table_name in db.table_names():
            tbl = db.open_table(table_name)
            count = len(tbl.to_pandas())
            print(f"Total entries in '{table_name}': {count}")
            
            # Optional: Print some sample terms to verify variety
            df = tbl.to_pandas()
            print("Sample terms:")
            print(df['term'].head(VAR_5))
            print(df['term'].tail(VAR_5))
        else:
            print(f"Table '{table_name}' not found in database.")
    except Exception as e:
        print(f"Error verifying database: {e}")

if __name__ == "__main__":
    verify_count()
