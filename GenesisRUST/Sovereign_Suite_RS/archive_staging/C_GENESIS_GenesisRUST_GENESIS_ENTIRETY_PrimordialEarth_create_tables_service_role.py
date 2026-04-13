"""
create_tables_service_role.py
Creates the Genesis Soul Vault schema in Supabase using the service_role key.
"""
import subprocess, sys
subprocess.check_call([sys.executable, "-m", "pip", "install", "supabase", "-q"])

from supabase import create_client

URL = "https://duuycxgqbhrqmwapnjhk.supabase.co"
SERVICE_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImR1dXljeGdxYmhycW13YXBuamhrIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2Nzc0MDE4MCwiZXhwIjoyMDgzMzE2MTgwfQ.O7RD5ELSm0xxw53B-o-k0Xxg4XhtO8WB-3f3hj5temA"

print(f"[SUPABASE] Connecting with service_role key...")
client = create_client(URL, SERVICE_KEY)

# Try creating via RPC exec_sql if it exists, otherwise inform
SQL_SOULS = """
CREATE TABLE IF NOT EXISTS souls (
    soul_id         TEXT PRIMARY KEY,
    genome          TEXT,
    x               FLOAT,
    y               FLOAT,
    energy          FLOAT,
    moral_alignment FLOAT,
    is_active       INTEGER DEFAULT 1,
    species         TEXT,
    generation      INTEGER,
    current_action  TEXT,
    vit             INTEGER,
    str             INTEGER,
    agi             INTEGER,
    int_stat        INTEGER,
    wis             INTEGER,
    luk             INTEGER,
    blessing        TEXT,
    leader_id       TEXT,
    hope_log        TEXT,
    reasoning_path  TEXT,
    name            TEXT,
    divine_mandate  TEXT,
    pregnancy_timer FLOAT DEFAULT 0,
    age_ticks       FLOAT DEFAULT 0,
    updated_at      TIMESTAMPTZ DEFAULT NOW()
);
"""

SQL_EVENTS = """
CREATE TABLE IF NOT EXISTS pantheon_events (
    id          BIGSERIAL PRIMARY KEY,
    soul_id     TEXT,
    event       TEXT,
    data        JSONB,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);
"""

try:
    result = client.rpc("exec_sql", {"query": SQL_SOULS}).execute()
    print(f"[OK] souls table: {result}")
except Exception as e:
    print(f"[exec_sql RPC] {e}")
    # Try a test insert to see if table already exists
    try:
        r = client.table("souls").select("soul_id").limit(1).execute()
        print("[OK] souls table already exists! Ready.")
    except Exception as e2:
        print(f"[souls check] {e2}")

try:
    result2 = client.rpc("exec_sql", {"query": SQL_EVENTS}).execute()
    print(f"[OK] pantheon_events table: {result2}")
except Exception as e:
    print(f"[pantheon_events RPC] {e}")
    try:
        r = client.table("pantheon_events").select("id").limit(1).execute()
        print("[OK] pantheon_events table already exists!")
    except:
        pass

print("\n[DONE] Update .env with:")
print(f"SUPABASE_SERVICE_KEY={SERVICE_KEY}")
