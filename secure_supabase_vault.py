import os
import asyncio
from supabase import create_client, Client

url = "https://duuycxgqbhrqmwapnjhk.supabase.co"
# The service role key from the 05_THE_CORE .env output
key = "[Key omitted for script logging, replaced dynamically in execution]"

# Unfortunately, the Supabase Python Client (PostgREST) does not directly support DDL (ALTER TABLE) or raw SQL execution by default for security.
# To fix this, we need to call an RPC function if one exists,

supabase: Client = create_client(url, key)

print("Connected to Supabase via Service Role Key.")

# To execute DDL over the REST API safely without psycopg2, we push the SQL string through the RPC endpoint if 'exec_sql' exists
# If it doesn't, we will notify the user they must execute this in the Supabase Dashboard SQL Editor directly.

sql_payload = """
-- Enable RLS for pantheon_events
ALTER TABLE public.pantheon_events ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Enable all for service role" ON public.pantheon_events FOR ALL USING (auth.role() = 'service_role') WITH CHECK (auth.role() = 'service_role');

-- Enable RLS for sovereign_ledger
ALTER TABLE public.sovereign_ledger ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Enable all for service role" ON public.sovereign_ledger FOR ALL USING (auth.role() = 'service_role') WITH CHECK (auth.role() = 'service_role');

-- Enable RLS for genesis_memory
ALTER TABLE public.genesis_memory ENABLE ROW LEVEL SECURITY;
CREATE POLICY "Enable all for service role" ON public.genesis_memory FOR ALL USING (auth.role() = 'service_role') WITH CHECK (auth.role() = 'service_role');

-- Secure search_path
ALTER FUNCTION public.enforce_stat_caps() SET search_path = public;
"""

try:
    print("Attempting to execute RLS injection via standard PostgREST RPC 'exec_sql'...")
    res = supabase.rpc("exec_sql", {"sql": sql_payload}).execute()
    print("Vault Secured: RLS applied successfully via RPC.")
except Exception as e:
    print(f"RPC Execution Failed: {e}")
    print("\n--- ACTION REQUIRED ---")
    print("Supabase has locked direct SQL execution from the REST API without an RPC wrapper.")
    print("Please copy the following SQL and execute it manually in your Supabase SQL Editor (https://supabase.com/dashboard/project/duuycxgqbhrqmwapnjhk/sql/new):")
    print("-" * 50)
    print(sql_payload)
    print("-" * 50)
