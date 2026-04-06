//! compare_bio_stats.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn compare_species() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        println!( "--- SPECIES STAT COMPARISON ---" );
        for spec in [ "BIO-001" , "BIO-009" ] .iter() {
        cur . execute ( "
            SELECT AVG(vit), AVG(wis), AVG(int_stat), AVG(energy), COUNT(*) 
            FROM souls WHERE species=? AND is_active=1
        " , ( spec , ) );
        vit , wis , int_ , nrg , count = cur . fetchone ( );
        println!( f "{spec} (n={count}):" );
        println!( f "  Avg VIT: {vit:.2f} | Avg WIS: {wis:.2f} | Avg INT: {int_:.2f}" );
        println!( f "  Avg Energy: {nrg:.2f}" );
        cur . execute ( "SELECT COUNT(*) FROM souls WHERE species='BIO-009' AND energy < 0" );
        ghost_count = cur . fetchone ( ) [ 0 ];
        println!( f "\nBIO-009 Ghost Count (Energy < 0): {ghost_count}" );
        cur . execute ( "SELECT blessing, COUNT(*) FROM souls WHERE species='BIO-009' GROUP BY blessing" );
        println!( "\nBIO-009 Blessings:" );
        for b in cur . fetchall ( ) .iter() {
        println!( f "  {b[0]}: {b[1]}" );
        conn . close ( );
        fn main() {
        compare_species ( );
}

