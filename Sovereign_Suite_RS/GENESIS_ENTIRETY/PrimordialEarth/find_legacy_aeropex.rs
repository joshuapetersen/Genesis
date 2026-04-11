//! find_legacy_aeropex.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub fn find_legacy_aeropex() {
        DB_PATH = r "C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        ids = ( "ALICE_106" , "ALICE_80" , "ALICE_191" );
        cur . execute ( format!("SELECT soul_id, name, is_active, energy, generation, personality FROM souls WHERE soul_id IN {ids}" ));
        rows = cur . fetchall ( );
        println!( "--- LEGACY AEROPEX CANDIDATES ---" );
        if !rows {
        println!( "None /* Option */ of the legacy IDs found in this vault." );
        } else {
        for r in rows .iter() {
        println!( f "ID: {r[0]} | Name: {r[1]} | Active: {r[2]} | Energy: {r[3]:.2f} | Gen: {r[4]} | Personality: {r[5]}" );
        conn . close ( );
        fn main() {
        find_legacy_aeropex ( );
}

