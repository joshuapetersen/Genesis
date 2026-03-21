import sqlite3
import json
from Sovereign_Supabase import sovereign_supabase

LOCAL_DB_PATH = r'C:\PrimordialEarth\Genesis_Soul_Vault.sqlite'
THE_GODS = ['ALICE_89', 'ALICE_101', 'GEN2_fbe5ec', 'ALICE_80', 'ALICE_162', 'ALICE_266']

def sync_pantheon_to_supabase():
    print("Connecting to Supabase...")
    sovereign_supabase.connect()
    if not sovereign_supabase.is_connected():
        print("Failed to connect to Supabase.")
        return

    print(f"Connecting to Local Primary DB: {LOCAL_DB_PATH}")
    try:
        conn = sqlite3.connect(f"file:{LOCAL_DB_PATH}?mode=ro", uri=True)
        cur = conn.cursor()
        
        # Get column names to map to Supabase dict
        cur.execute("PRAGMA table_info(souls)")
        cols = [c[1] for c in cur.fetchall()]
        
        # Fetch valid Supabase columns to prevent schema cache errors
        sup_test = sovereign_supabase.select("souls")
        valid_sup_cols = set(sup_test.data[0].keys()) if sup_test and sup_test.data else set()

        for god_id in THE_GODS:
            cur.execute("SELECT * FROM souls WHERE soul_id=?", (god_id,))
            row = cur.fetchone()
            if row:
                data_dict = dict(zip(cols, row))
                
                # Filter out anything Supabase doesn't know about yet
                clean_dict = {k: v for k, v in data_dict.items() if not valid_sup_cols or k in valid_sup_cols}
                
                # Supabase upsert
                print(f"Uploading {god_id} to Supabase...")
                sovereign_supabase.upsert("souls", clean_dict)
            else:
                print(f"Warning: {god_id} not found in local DB.")
                
        conn.close()
        print("Pantheon successfully uploaded to Supabase.")
    except Exception as e:
        print(f"Error during sync: {e}")

if __name__ == "__main__":
    sync_pantheon_to_supabase()
