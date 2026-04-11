//! check_wis.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub fn check_wis() {
        DB_PATH = r "C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "SELECT soul_id, generation, wis FROM souls WHERE is_active=1 ORDER BY wis DESC LIMIT 20" );
        rows = cur . fetchall ( );
        println!( "--- TOP 20 WIS ENTITIES ---" );
        for row in rows .iter() {
        println!( f "[{row[1]}] {row[0]}: WIS {row[2]}" );
        cur . execute ( "SELECT AVG(wis) FROM souls WHERE is_active=1" );
        avg_wis = cur . fetchone ( ) [ 0 ];
        println!( f "\nAverage WIS: {avg_wis:.2f}" );
        conn . close ( );
        fn main() {
        check_wis ( );
}

