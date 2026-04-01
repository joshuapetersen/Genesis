"""
create_tables_http.py
Uses the Supabase REST endpoint directly with the service_role key to create tables.
Supabase exposes a /rest/v1/ endpoint and also a pg connection via the API gateway.
"""
import subprocess, sys
subprocess.check_call([sys.executable, "-m", "pip", "install", "requests", "-q"])
import requests, json

URL = "https://duuycxgqbhrqmwapnjhk.supabase.co"
SERVICE_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImR1dXljeGdxYmhycW13YXBuamhrIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2Nzc0MDE4MCwiZXhwIjoyMDgzMzE2MTgwfQ.O7RD5ELSm0xxw53B-o-k0Xxg4XhtO8WB-3f3hj5temA"

HEADERS = {
    "apikey": SERVICE_KEY,
    "Authorization": f"Bearer {SERVICE_KEY}",
    "Content-Type": "application/json",
    "Prefer": "return=minimal"
}

SQL_CREATE = """
CREATE TABLE IF NOT EXISTS souls (
    soul_id TEXT PRIMARY KEY,
    genome TEXT, x FLOAT, y FLOAT, energy FLOAT,
    moral_alignment FLOAT, is_active INTEGER DEFAULT 1,
    species TEXT, generation INTEGER, current_action TEXT,
    vit INTEGER, str INTEGER, agi INTEGER, int_stat INTEGER,
    wis INTEGER, luk INTEGER, blessing TEXT, leader_id TEXT,
    hope_log TEXT, reasoning_path TEXT, name TEXT,
    divine_mandate TEXT, pregnancy_timer FLOAT DEFAULT 0,
    age_ticks FLOAT DEFAULT 0, updated_at TIMESTAMPTZ DEFAULT NOW()
);
CREATE TABLE IF NOT EXISTS pantheon_events (
    id BIGSERIAL PRIMARY KEY, soul_id TEXT,
    event TEXT, data JSONB, created_at TIMESTAMPTZ DEFAULT NOW()
);
"""

# Supabase has a /pg endpoint for direct SQL via their API gateway
resp = requests.post(
    f"{URL}/rest/v1/rpc/query",
    headers=HEADERS,
    json={"query": SQL_CREATE}
)
print(f"RPC query: {resp.status_code} {resp.text[:200]}")

# Alternative: use the Supabase management API
# Extract project ref from URL
project_ref = "duuycxgqbhrqmwapnjhk"

# Try Supabase's SQL endpoint (available on some plans)
resp2 = requests.post(
    f"https://api.supabase.com/v1/projects/{project_ref}/database/query",
    headers={
        "Authorization": f"Bearer {SERVICE_KEY}",
        "Content-Type": "application/json"
    },
    json={"query": SQL_CREATE}
)
print(f"Management API: {resp2.status_code} {resp2.text[:300]}")

# If neither works, verify current table state
resp3 = requests.get(
    f"{URL}/rest/v1/souls?limit=1",
    headers=HEADERS
)
print(f"souls table check: {resp3.status_code} {resp3.text[:200]}")

if resp3.status_code == 200:
    print("\n[SUCCESS] souls table EXISTS and is accessible with service_role key!")
elif resp3.status_code == 404 or "relation" in resp3.text:
    print("\n[NEEDED] souls table does not exist yet - need DDL access")
    print("Opening Supabase SQL editor URL automatically...")
    import webbrowser
    webbrowser.open(f"https://supabase.com/dashboard/project/{project_ref}/sql/new")
    print("SQL to run has been written to: C:\\PrimordialEarth\\schema_supabase.sql")
