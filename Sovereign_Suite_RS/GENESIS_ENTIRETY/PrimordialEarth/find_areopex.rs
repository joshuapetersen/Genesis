//! find_areopex.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub fn find_areopex() {
        DB_PATH = r "C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        query = "SELECT soul_id, name, is_active, generation FROM souls WHERE name LIKE ? OR soul_id LIKE ?";
        pattern = "%aero%";
        cur . execute ( query , ( pattern , pattern ) );
        rows = cur . fetchall ( );
        if !rows {
        println!( "No soul containing 'aero' found." );
        } else {
        for r in rows .iter() {
        println!( f "ID: {r[0]} | Name: {r[1]} | Active: {r[2]} | Gen: {r[3]}" );
        conn . close ( );
        fn main() {
        find_areopex ( );
}

