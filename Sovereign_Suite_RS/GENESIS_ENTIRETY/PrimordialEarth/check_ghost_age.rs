//! check_ghost_age.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_ghost_age() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "SELECT name, age_ticks, energy, hope_log, current_action, reasoning_path FROM souls WHERE soul_id='ALICE_266'" );
        r = cur . fetchone ( );
        if r {
        println!( f "--- GHOST VITAL CHECK ---" );
        println!( f "Name: {r[0]} | Age: {r[1]} | Energy: {r[2]} | Action: {r[4]}" );
        println!( f "Log: {r[3]}" );
        if r [ 5 ] {
        println!( "\n--- REASONING PATH (LAST 500 CHARS) ---" );
        println!( r [ 5 ] [ -500 : ] );
        conn . close ( );
        fn main() {
        check_ghost_age ( );
}

