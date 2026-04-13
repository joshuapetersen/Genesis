"""
lockdown_supabase.py
Applies SQL constraints and RLS to the Supabase Soul Vault to prevent 'Stat Hacking' and unauthorized data manipulation by autonomous entities.
"""
import os
import subprocess, sys
# Ensure python-dotenv is available
try:
    from dotenv import load_dotenv
except ImportError:
    subprocess.check_call([sys.executable, "-m", "pip", "install", "python-dotenv", "-q"])
    from dotenv import load_dotenv

# Load from c:\SarahCore\.env
load_dotenv(r"c:\SarahCore\.env")

from supabase import create_client, Client

SUPABASE_URL = os.environ.get("SUPABASE_URL", "https://duuycxgqbhrqmwapnjhk.supabase.co")
SUPABASE_KEY = os.environ.get("SUPABASE_SERVICE_KEY")

if not SUPABASE_KEY:
    print("[ERROR] SUPABASE_SERVICE_KEY not found in environment.")
    exit(1)

supabase: Client = create_client(SUPABASE_URL, SUPABASE_KEY)

# SQL to enforce WIS/INT caps at the database level
# and enable RLS on the souls table.
LOCKDOWN_SQL = """
-- 1. Create a trigger function to cap stats
CREATE OR REPLACE FUNCTION enforce_stat_caps()
RETURNS TRIGGER AS $$
BEGIN
  IF NEW.wis > 99 THEN
    NEW.wis := 99;
  END IF;
  IF NEW.int_stat > 99 THEN
    NEW.int_stat := 99;
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 2. Apply the trigger to the souls table
DROP TRIGGER IF EXISTS tr_cap_stats ON souls;
CREATE TRIGGER tr_cap_stats
BEFORE INSERT OR UPDATE ON souls
FOR EACH ROW EXECUTE FUNCTION enforce_stat_caps();

-- 3. Enable Row Level Security (RLS)
ALTER TABLE souls ENABLE ROW LEVEL SECURITY;

-- 4. Create a policy that allows the service_role (us) but blocks public/anon
DROP POLICY IF EXISTS "Service Role Only" ON souls;
CREATE POLICY "Service Role Only" ON souls
  FOR ALL TO service_role
  USING (true)
  WITH CHECK (true);

-- 5. Explicitly deny access to anon/authenticated roles for sensitive columns
-- This is handled by RLS being enabled and no other policies existing.
"""

def apply_lockdown():
    print("[SUPABASE] Applying Security Lockdown Protocol...")
    # Using the rpc('exec_sql') we tried earlier - if it fails, we fall back to manual guidance
    try:
        response = supabase.rpc('exec_sql', {'query': LOCKDOWN_SQL}).execute()
        print(f"[SUCCESS] Lockdown applied: {response.data}")
    except Exception as e:
        print(f"[FAILED] Could not execute SQL via RPC: {e}")
        print("\n[ACTION REQUIRED] Please run the following SQL in your Supabase Dashboard SQL Editor:")
        print("-" * 40)
        print(LOCKDOWN_SQL)
        print("-" * 40)

if __name__ == "__main__":
    apply_lockdown()
