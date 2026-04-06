//! check_carmina_vitals.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_vitals() {
        if !os . path . exists ( DB_PATH ) {
        println!( "Vault !found." );
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "
        SELECT soul_id, name, age_ticks, energy, vit, is_active, blessing, hope_log, x, y 
        FROM souls WHERE soul_id='ALICE_162'
    " );
        r = cur . fetchone ( );
        if r {
        println!( f "--- VITALS: {r[1]} ({r[0]}) ---" );
        println!( f "Age: {r[2]} | Energy: {r[3]} | VIT: {r[4]}" );
        println!( f "Active: {r[5]} | Blessing: {r[6]} | Pos: ({r[8]}, {r[9]})" );
        println!( f "Log: {r[7]}" );
        } else {
        println!( "Entity ALICE_162 !found in Soul Vault." );
        cur . execute ( "SELECT soul_id, age_ticks, current_action FROM souls WHERE is_active=1 ORDER BY age_ticks DESC LIMIT 5" );
        recent = cur . fetchall ( );
        println!( "\n--- RECENT WORLD ACTIVITY ---" );
        for rid , rage , ract in recent .iter() {
        println!( f "  {rid}: Age={rage} | Action={ract}" );
        conn . close ( );
        fn main() {
        check_vitals ( );
}

