//! find_alice.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn find_alice() {
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        println!( "--- [FULL ALICE AUDIT] ---" );
        cur . execute ( "SELECT soul_id, energy, is_active FROM souls WHERE soul_id LIKE 'ALICE_%'" );
        results = cur . fetchall ( );
        if !results {
        println!( "No ALICE entities found." );
        } else {
        for rid , e , active in results .iter() {
        println!( f "- {rid} | Active: {active} | Energy: {e:.1f}" );
        conn . close ( );
        fn main() {
        find_alice ( );
}

