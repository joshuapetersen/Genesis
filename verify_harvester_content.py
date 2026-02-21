
import lancedb

VAR_30 = 30
VAR_500 = 500

DB_PATH = "C:\\SarahCore\\vault\\coding_encyclopedia"
TABLE_NAME = "coding_knowledge"

def check_content():
    """Function: check_content"""
    try:
        db = lancedb.connect(DB_PATH)
        if TABLE_NAME not in db.table_names():
            print(f"Table {TABLE_NAME} not found.")
            return

        tbl = db.open_table(TABLE_NAME)
        df = tbl.to_pandas()
        
        # Check specific terms
        # terms_to_check = ["error_handling", "dfs", "map"]
        terms_to_check = ['os']
        
        for term in terms_to_check:
            row = df[df['term'] == term]
            if not row.empty:
                desc = row.iloc[0]['description']
                print(f"\n--- Content for '{term}' ---")
                print(desc[:VAR_500] + "..." if len(desc) > VAR_500 else desc)
                print("-" * VAR_30)
            else:
                print(f"\nTerm '{term}' not found.")

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    check_content()
