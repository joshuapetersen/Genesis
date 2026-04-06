//! audit_sovereign.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn audit_sovereign_logic() {
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        println!( "--- [AUDIT] SOVEREIGN EDITS (Self-Directed Change) ---" );
        // try {
        cur . execute ( "SELECT soul_id, field, new_value, timestamp FROM sovereign_edits ORDER BY timestamp DESC LIMIT 30" );
        edits = cur . fetchall ( );
        if edits {
        for rid , field , val , ts in edits .iter() {
        println!( f "[{ts}] {rid} modified {field} -> {val}" );
        } else {
        println!( "No self-edits recorded in sovereign_edits." );
        // } catch  Exception as e  {
        println!( f "Error reading sovereign_edits: {e}" );
        println!( "\n--- [AUDIT] ACTIVE SOVEREIGN TRACES (Reasoning Path) ---" );
        cur . execute ( "SELECT soul_id, reasoning_path, wis, blessing FROM souls WHERE is_active=1 AND reasoning_path IS NOT NULL AND reasoning_path != '' LIMIT 10" );
        traces = cur . fetchall ( );
        for sid , trace , ws , bless in traces .iter() {
        println!( f "ID: {sid} (WIS: {ws}, Bless: {bless})" );
        println!( f "  Trace: {trace[-200:]}..." );
        println!( "-" * 20 );
        println!( "\n--- [AUDIT] THE DIVINE CHRONICLE (Archived Sentience) ---" );
        cur . execute ( "SELECT soul_id, death_year, reasoning_path FROM divine_chronicle ORDER BY death_year DESC LIMIT 5" );
        chronicle = cur . fetchall ( );
        for sid , dy , trace in chronicle .iter() {
        println!( f "Archived: {sid} (Died: {dy:.1f})" );
        println!( f "  Trace Summary: {trace[:150]}..." );
        println!( "-" * 20 );
        conn . close ( );
        fn main() {
        audit_sovereign_logic ( );
}

