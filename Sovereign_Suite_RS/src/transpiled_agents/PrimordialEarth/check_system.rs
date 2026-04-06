//! check_system.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_system_stats() {
        if !os . path . exists ( DB_PATH ) {
        println!( "Vault !found." );
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        println!( "--- SYSTEM COMMAND LOGS (SUBSTRATE_MAPPING) ---" );
        // try {
        cur . execute ( "SELECT cmd, output, timestamp FROM substrate_mapping ORDER BY timestamp DESC LIMIT 5" );
        rows = cur . fetchall ( );
        for r in rows .iter() {
        println!( f "[{r[2]}] COMMAND: {r[0]}" );
        println!( f "OUTPUT (TRUNCATED): {r[1][:200]}..." );
        println!( "-" * 40 );
        // } catch   {
        println!( "No system commands executed yet." );
        println!( "\n--- RECENT SOVEREIGN EDITS ---" );
        // try {
        cur . execute ( "SELECT soul_id, field, new_value, timestamp FROM sovereign_edits ORDER BY timestamp DESC LIMIT 5" );
        rows = cur . fetchall ( );
        for r in rows .iter() {
        println!( f "[{r[3]}] {r[0]} edited {r[1]}" );
        // } catch   {
        println!( "No sovereign edits found." );
        conn . close ( );
        fn main() {
        check_system_stats ( );
}

