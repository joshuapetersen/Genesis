
import lancedb

VAR_50 = 50

DB_PATH = "C:\\SarahCore\\vault\\coding_encyclopedia"
TABLE_NAME = "coding_knowledge"

def check_error_handling_lattice():
    """Function: check_error_handling_lattice"""
    try:
        db = lancedb.connect(DB_PATH)
        tbl = db.open_table(TABLE_NAME)
        df = tbl.to_pandas()
        
        row = df[df['term'] == 'error_handling']
        if not row.empty:
            coord = row.iloc[0].get('lattice_coordinate', 'Column Missing')
            who = row.iloc[0].get('who_vector', 'Column Missing')
            print(f"--- Entry for 'error_handling' ---")
            print(f"Lattice Coordinate: {coord}")
            print(f"WHO Vector: {who[:VAR_50]}...")
            print("-----------------------")
            
            if coord != "0-0-0" and coord != "Column Missing":
                print("SUCCESS: Valid Lattice Coordinate assigned.")
            else:
                print("FAILURE: Invalid or missing Lattice Coordinate.")
        else:
            print("Term 'error_handling' not found.")

    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    check_error_handling_lattice()
