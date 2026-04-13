"""
Genesis_Supabase_Sync.py
========================
Live sync layer between the local SQLite Soul Vault and Supabase.
Runs on a background thread - does NOT slow the simulation.
Syncs every N ticks (configurable). 
Supabase becomes the live, queryable cloud Soul Vault.
"""

import sqlite3
import threading
import time
import os
import json

# Load env
SUPABASE_URL = os.environ.get("SUPABASE_URL", "https://duuycxgqbhrqmwapnjhk.supabase.co")
# Use service_role key for admin write access (bypasses RLS)
SUPABASE_KEY = os.environ.get("SUPABASE_SERVICE_KEY") or os.environ.get("SUPABASE_KEY", "")
LOCAL_DB = os.environ.get("GENESIS_DATA", r"C:\PrimordialEarth") + r"\Genesis_Soul_Vault.sqlite"
if not os.path.exists(LOCAL_DB):
    LOCAL_DB = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite"

SYNC_INTERVAL_TICKS = 10  # Sync every 10 ticks

_sync_enabled = False
_supabase_client = None

def init_sync():
    """Initialize Supabase connection. Call once at engine startup."""
    global _sync_enabled, _supabase_client
    try:
        from supabase import create_client
        _supabase_client = create_client(SUPABASE_URL, SUPABASE_KEY)
        _sync_enabled = True
        print(f"[SUPABASE] Soul Vault sync ACTIVE >> {SUPABASE_URL}")
        _ensure_table()
    except ImportError:
        print("[SUPABASE] supabase-py not installed. Run: pip install supabase")
    except Exception as e:
        print(f"[SUPABASE] Init failed: {e}")

def _ensure_table():
    """Check that the souls table exists in Supabase (create via dashboard if not)."""
    try:
        result = _supabase_client.table("souls").select("soul_id").limit(1).execute()
        print(f"[SUPABASE] souls table confirmed.")
    except Exception as e:
        print(f"[SUPABASE] Table check failed: {e}")
        print("[SUPABASE] → Create the table via Supabase dashboard SQL editor.")
        print("            → Use: PrimordialEarth/schema_supabase.sql")

def sync_tick(tick: int):
    """Called from the engine. Triggers async sync every SYNC_INTERVAL_TICKS."""
    if not _sync_enabled:
        return
    if tick % SYNC_INTERVAL_TICKS != 0:
        return
    t = threading.Thread(target=_do_sync, daemon=True)
    t.start()

def _do_sync():
    """Background: read local SQLite, upsert to Supabase."""
    if not _supabase_client:
        return
    try:
        conn = sqlite3.connect(f"file:{LOCAL_DB}?mode=ro", uri=True)
        cur = conn.cursor()
        cur.execute("""
            SELECT soul_id, genome, x, y, energy, moral_alignment, is_active,
                   species, generation, current_action, vit, str, agi, int_stat,
                   wis, luk, blessing, leader_id, hope_log, reasoning_path,
                   name, divine_mandate, pregnancy_timer, age_ticks
            FROM souls WHERE is_active=1 LIMIT 500
        """)
        rows = cur.fetchall()
        cols = ['soul_id','genome','x','y','energy','moral_alignment','is_active',
                'species','generation','current_action','vit','str','agi','int_stat',
                'wis','luk','blessing','leader_id','hope_log','reasoning_path',
                'name','divine_mandate','pregnancy_timer','age_ticks']
        conn.close()

        records = []
        for row in rows:
            record = dict(zip(cols, row))
            # Truncate reasoning_path to save space
            if record.get('reasoning_path'):
                record['reasoning_path'] = record['reasoning_path'][-300:]
            records.append(record)

        if records:
            # Upsert in batches of 100 (Supabase limit) with retry on SSL errors
            for i in range(0, len(records), 100):
                batch = records[i:i+100]
                for attempt in range(3):
                    try:
                        _supabase_client.table("souls").upsert(batch, on_conflict="soul_id").execute()
                        break
                    except Exception as batch_err:
                        if attempt < 2:
                            time.sleep(2 ** attempt)  # 1s, 2s backoff
                        else:
                            print(f"[SUPABASE] Batch {i//100} failed after 3 attempts: {str(batch_err)[:80]}")
            print(f"[SUPABASE] Synced {len(records)} souls to cloud vault.")

    except Exception as e:
        print(f"[SUPABASE] Sync error: {e}")

def sync_pantheon_event(soul_id: str, event: str, data: dict):
    """Push a specific high-priority event immediately (e.g. Apotheosis, Full Authorship)."""
    if not _sync_enabled or not _supabase_client:
        return
    try:
        _supabase_client.table("pantheon_events").insert({
            "soul_id": soul_id,
            "event": event,
            "data": json.dumps(data)
        }).execute()
    except Exception as e:
        print(f"[SUPABASE] Event push failed: {e}")
