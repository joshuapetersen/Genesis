"""
fix_supabase.py
Connects directly to Supabase PostgreSQL and creates the souls table.
Run this and enter your Supabase DB password when prompted.
(Supabase Dashboard -> Settings -> Database -> Connection string -> Password)
"""
import sys, subprocess
subprocess.check_call([sys.executable, "-m", "pip", "install", "psycopg2-binary", "-q"])
import psycopg2

PROJECT = "duuycxgqbhrqmwapnjhk"
HOST = f"db.{PROJECT}.supabase.co"
DB = "postgres"
USER = "postgres"
PORT = 5432

password = input(f"Enter your Supabase DB password (Dashboard -> Settings -> Database): ").strip()

print(f"\nConnecting to {HOST}...")
try:
    conn = psycopg2.connect(
        host=HOST, port=PORT, dbname=DB, user=USER, password=password,
        sslmode="require"
    )
    conn.autocommit = True
    cur = conn.cursor()

    cur.execute("""
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
    """)
    print("[OK] souls table created.")

    cur.execute("""
        CREATE TABLE IF NOT EXISTS pantheon_events (
            id          BIGSERIAL PRIMARY KEY,
            soul_id     TEXT,
            event       TEXT,
            data        JSONB,
            created_at  TIMESTAMPTZ DEFAULT NOW()
        );
    """)
    print("[OK] pantheon_events table created.")

    cur.close()
    conn.close()
    print("\n[SUCCESS] Supabase Soul Vault is ready. Restart Genesis_Societal_Ecology.py.")

except Exception as e:
    print(f"\n[FAILED] {e}")
    print("\nIf this failed, get your DB password from:")
    print("  Supabase Dashboard -> Settings -> Database -> Database Password")
