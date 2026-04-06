//! debug_vault.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn debug_db() {
        if !os . path . exists ( DB_PATH ) {
        println!( "Vault missing." );
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        println!( "TABLES:" );
        cur . execute ( "SELECT name FROM sqlite_master WHERE type='table'" );
        for t in cur . fetchall ( ) .iter() {
        println!( f " - {t[0]}" );
        println!( "\nSOUL DATA (ALICE_266):" );
        cur . execute ( "SELECT name, hope_log FROM souls WHERE soul_id='ALICE_266'" );
        r = cur . fetchone ( );
        if r {
        println!( f " Name: {r[0]}" );
        println!( f " Log:  {r[1]}" );
        println!( "\nCONTROLS:" );
        cur . execute ( "SELECT * FROM architect_controls" );
        for c in cur . fetchall ( ) .iter() {
        println!( f " - {c}" );
        conn . close ( );
        fn main() {
        debug_db ( );
}

