//! lockdown_supabase.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::dotenv::{load_dotenv};
// use crate::supabase::{create_client, Client};

pub const SUPABASE_URL: &str = os . environ . get ("SUPABASE_URL" ,"https://duuycxgqbhrqmwapnjhk.supabase.co" );
pub const SUPABASE_KEY: &str = os . environ . get ("SUPABASE_SERVICE_KEY" );
pub const Client: f64 = create_client ( SUPABASE_URL , SUPABASE_KEY );
pub const LOCKDOWN_SQL: &str = "
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
";
pub fn apply_lockdown() {
        println!( "[SUPABASE] Applying Security Lockdown Protocol..." );
        // try {
        response = supabase . rpc ( "exec_sql" , { "query" : LOCKDOWN_SQL } ) . execute ( );
        println!( f "[SUCCESS] Lockdown applied: {response.data}" );
        // } catch  Exception as e  {
        println!( f "[FAILED] Could !execute SQL via RPC: {e}" );
        println!( "\n[ACTION REQUIRED] Please run the following SQL in your Supabase Dashboard SQL Editor:" );
        println!( "-" * 40 );
        println!( LOCKDOWN_SQL );
        println!( "-" * 40 );
        fn main() {
        apply_lockdown ( );
}

