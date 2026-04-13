"""
create_tables_psycopg2.py
Direct PostgreSQL connection to Supabase using the service_role JWT as password.
"""
import subprocess, sys
subprocess.check_call([sys.executable, "-m", "pip", "install", "psycopg2-binary", "-q"])
import psycopg2

PROJECT = "duuycxgqbhrqmwapnjhk"
SERVICE_KEY = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6ImR1dXljeGdxYmhycW13YXBuamhrIiwicm9sZSI6InNlcnZpY2Vfcm9sZSIsImlhdCI6MTc2Nzc0MDE4MCwiZXhwIjoyMDgzMzE2MTgwfQ.O7RD5ELSm0xxw53B-o-k0Xxg4XhtO8WB-3f3hj5temA"

# Supabase direct postgres connection - JWT is used as password
# Connection string format: postgresql://postgres.{ref}:{service_role_key}@aws-0-us-east-1.pooler.supabase.com:6543/postgres
CONN_STRINGS = [
    f"postgresql://postgres.{PROJECT}:{SERVICE_KEY}@aws-0-us-east-1.pooler.supabase.com:6543/postgres?sslmode=require",
    f"postgresql://postgres:{SERVICE_KEY}@db.{PROJECT}.supabase.co:5432/postgres?sslmode=require",
]

SQL = """
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

for conn_str in CONN_STRINGS:
    host = conn_str.split("@")[1].split(":")[0]
    print(f"\nTrying {host}...")
    try:
        conn = psycopg2.connect(conn_str, connect_timeout=10)
        conn.autocommit = True
        cur = conn.cursor()
        cur.execute(SQL)
        print("[SUCCESS] Tables created!")
        cur.close()
        conn.close()
        print("\nSoul Vault is LIVE in Supabase. Restart Genesis_Societal_Ecology.py.")
        break
    except Exception as e:
        print(f"[FAILED] {str(e)[:150]}")
