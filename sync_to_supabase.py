
import sqlite3
import os
from supabase import create_client, Client
from dotenv import load_dotenv

load_dotenv()

URL = os.environ.get("SUPABASE_URL")
KEY = os.environ.get("SUPABASE_SERVICE_KEY") or os.environ.get("SUPABASE_KEY")

if not URL or not KEY:
    print("Error: Missing Supabase credentials in .env")
    exit(1)

supabase: Client = create_client(URL, KEY)

def sync_sqlite_to_supabase(sqlite_path, table_name):
    if not os.path.exists(sqlite_path):
        print(f"Skipping {sqlite_path}: File not found")
        return

    print(f"Syncing {sqlite_path} to Supabase table '{table_name}'...")
    conn = sqlite3.connect(sqlite_path)
    cursor = conn.cursor()
    
    # Get all tables in the SQLite db
    cursor.execute("SELECT name FROM sqlite_master WHERE type='table';")
    tables = [row[0] for row in cursor.fetchall()]
    
    for t in tables:
        print(f"  Exporting table: {t}")
        cursor.execute(f"SELECT * FROM {t}")
        rows = cursor.fetchall()
        columns = [description[0] for description in cursor.description]
        
        data = []
        for row in rows:
            record = dict(zip(columns, row))
            data.append(record)
        
        if data:
            try:
                # We use a combined table name for Supabase to avoid collisions
                target_table = f"sarah_{t.lower()}" 
                print(f"    Pushing {len(data)} rows to {target_table}...")
                # Note: Supabase tables must exist or be created via SQL
                # This script assumes the table structure is handled or upserted
                res = supabase.table(target_table).upsert(data).execute()
                print(f"    Success: {target_table}")
            except Exception as e:
                print(f"    Error pushing {t}: {e}")

    conn.close()

if __name__ == "__main__":
    databases = [
        ("Genesis_Soul_Vault.sqlite", "soul"),
        ("SLF_Akashic_Records.sqlite", "records"),
        ("SLF_Identity_Vault.sqlite", "identity")
    ]
    
    for db_path, name in databases:
        sync_sqlite_to_supabase(db_path, name)
