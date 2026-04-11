//! check_followers.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_followers() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "SELECT soul_id, leader_id FROM souls WHERE is_active=1 AND leader_id IS NOT NULL" );
        rows = cur . fetchall ( );
        if rows {
        println!( f "Total entries with leader_id: {len(rows)}" );
        for r in rows [ : 10 ] .iter() {
        println!( f "  - {r[0]} follows {r[1]}" );
        } else {
        println!( "No active followers found." );
        cur . execute ( "SELECT COUNT(*) FROM souls WHERE leader_id = 'ALICE_266'" );
        println!( f "ALICE_266 specifically has {cur.fetchone()[0]} followers." );
        conn . close ( );
        fn main() {
        check_followers ( );
}

