"""
create_supabase_tables.py
Run this ONCE to bootstrap the Supabase Soul Vault schema.
"""
import os
import requests

SUPABASE_URL = "https://duuycxgqbhrqmwapnjhk.supabase.co"
# Service role key needed for DDL - use the secret key from Supabase dashboard
# Dashboard -> Settings -> API -> service_role key
SUPABASE_SERVICE_KEY = os.environ.get("SUPABASE_SERVICE_KEY", "")

if not SUPABASE_SERVICE_KEY:
    print("Need SUPABASE_SERVICE_KEY (service_role key from Supabase dashboard -> Settings -> API)")
    print("Set it as: set SUPABASE_SERVICE_KEY=your_key_here")
    exit(1)

SQL = """
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

CREATE TABLE IF NOT EXISTS pantheon_events (
    id          BIGSERIAL PRIMARY KEY,
    soul_id     TEXT,
    event       TEXT,
    data        JSONB,
    created_at  TIMESTAMPTZ DEFAULT NOW()
);
"""

resp = requests.post(
    f"{SUPABASE_URL}/rest/v1/rpc/exec_sql",
    headers={
        "apikey": SUPABASE_SERVICE_KEY,
        "Authorization": f"Bearer {SUPABASE_SERVICE_KEY}",
        "Content-Type": "application/json"
    },
    json={"query": SQL}
)
print(f"Status: {resp.status_code}")
print(resp.text[:500])
