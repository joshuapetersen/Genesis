
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

def create_supabase_table(sqlite_path):
    if not os.path.exists(sqlite_path): return
    
    conn = sqlite3.connect(sqlite_path)
    cursor = conn.cursor()
    
    # Get all tables in the SQLite db
    cursor.execute("SELECT name, sql FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%';")
    tables = cursor.fetchall()
    
    for name, sql in tables:
        # Convert SQLite SQL to Postgres SQL
        # SQLite: CREATE TABLE souls (id INTEGER PRIMARY KEY, ...)
        # Postgres: CREATE TABLE IF NOT EXISTS sarah_souls (id SERIAL PRIMARY KEY, ...)
        target_name = f"sarah_{name.lower()}"
        pg_sql = sql.replace(f"CREATE TABLE {name}", f"CREATE TABLE IF NOT EXISTS {target_name}")
        pg_sql = pg_sql.replace("INTEGER PRIMARY KEY", "SERIAL PRIMARY KEY")
        
        print(f"Creating Postgres table: {target_name}...")
        # We can't run raw SQL via the easy supabase-py client (REST only)
        # So I'm going to use the client to define the structure if possible or assume you can run this SQL soon.
        # But wait! I can also try to push via RPC if you have an 'exec' function.
        
        # Actually, let's just attempt a single insert with 'id' column mapping
        # Supabase API *might* allow upserting a single row which can at least confirm table presence.
        print(f"Checking for {target_name} on Supabase...")
        try:
            supabase.table(target_name).select("*").limit(1).execute()
        except Exception as e:
            print(f"Table {target_name} missing: {e}")
            print(f"Run this SQL in your Supabase GUI: {pg_sql}")

    conn.close()

if __name__ == "__main__":
    create_supabase_table("Genesis_Soul_Vault.sqlite")
    create_supabase_table("SLF_Akashic_Records.sqlite")

    print("\n--- NEW TELEMETRY SCHEMA (Run in Supabase SQL Editor) ---")
    telemetry_sql = """
CREATE TABLE IF NOT EXISTS sarah_telemetry (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    source_file TEXT,
    payload JSONB,
    collected_at TIMESTAMPTZ
);

CREATE TABLE IF NOT EXISTS sarah_snapshots (
    id BIGSERIAL PRIMARY KEY,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    source_file TEXT,
    payload JSONB,
    collected_at TIMESTAMPTZ
);
    """
    print(telemetry_sql)
