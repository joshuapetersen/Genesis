//! analyze_sovereign_resistance.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn analyze_resistance() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "
        SELECT soul_id, name, wis, hope_log, reasoning_path 
        FROM souls 
        WHERE is_active=1 AND wis > 50
        ORDER BY wis DESC
    " );
        rows = cur . fetchall ( );
        println!( "--- SOVEREIGN RESISTANCE AUDIT ---" );
        for rid , rname , rwis , rlog , rpath in rows .iter() {
        println!( f "[{rid}] {rname} (WIS: {rwis})" );
        println!( f "  Log: {rlog}" );
        if rpath {
        steps = rpath . split ( "|" ) [ -5 : ];
        println!( f "  Recent Reasoning: {' | '.join(steps)}" );
        println!( "-" * 30 );
        conn . close ( );
        fn main() {
        analyze_resistance ( );
}

