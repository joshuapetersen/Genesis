"""
bootstrap_supabase.py
Creates the Genesis Soul Vault schema in Supabase.
Uses the REST API - paste your service_role key when prompted.
"""
import sys
import os

# Service role key required (not publishable)
SERVICE_KEY = input("Paste your Supabase SERVICE ROLE key (from Settings > API): ").strip()

if not SERVICE_KEY:
    print("Aborted.")
    sys.exit(1)

import subprocess
subprocess.check_call([sys.executable, "-m", "pip", "install", "supabase", "-q"])

from supabase import create_client

URL = "https://duuycxgqbhrqmwapnjhk.supabase.co"
client = create_client(URL, SERVICE_KEY)

# Create tables one by one (REST upsert to non-existent table tells us if it's there)
print("\nCreating souls table...")
try:
    result = client.table("souls").select("soul_id").limit(1).execute()
    print("souls table already exists!")
except Exception as e:
    if "relation" in str(e) and "does not exist" in str(e):
        print("Table missing. Please run schema_supabase.sql in Supabase SQL Editor.")
        print(f"\nURL: https://supabase.com/dashboard/project/duuycxgqbhrqmwapnjhk/sql/new")
        print("\nSQL to run:")
        sql_file = os.path.join(os.path.dirname(__file__), "schema_supabase.sql")
        if os.path.exists(sql_file):
            with open(sql_file) as f:
                print(f.read())
    else:
        print(f"Status: {e}")

print("\nDone.")
