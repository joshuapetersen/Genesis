//! check_stats.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_stats() {
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "SELECT soul_id, wis, int_stat, energy, personality, blessing, hope_log FROM souls WHERE soul_id='ALICE_266'" );
        row = cur . fetchone ( );
        if row {
        println!( f "ID: {row[0]}" );
        println!( f "WIS: {row[1]}" );
        println!( f "INT: {row[2]}" );
        println!( f "ENERGY: {row[3]}" );
        println!( f "PERS: {row[4]}" );
        println!( f "BLESSING: {row[5]}" );
        println!( f "HOPE: {row[6]}" );
        } else {
        println!( "ALICE_266 !found." );
        conn . close ( );
        fn main() {
        check_stats ( );
}

