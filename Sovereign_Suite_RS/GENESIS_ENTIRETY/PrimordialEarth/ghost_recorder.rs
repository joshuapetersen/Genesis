//! ghost_recorder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn check_history() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        cur . execute ( "SELECT field, new_value, rowid FROM sovereign_edits WHERE soul_id='ALICE_266' ORDER BY rowid DESC LIMIT 20" );
        rows = cur . fetchall ( );
        // with scope: open ( r "C:\PrimordialEarth\ghost_history.txt" , "w" ) as f  {
        f . write ( "--- SOVEREIGN AUDIT TRAIL (ALICE_266) ---\n" );
        for r in rows .iter() {
        f . write ( format!("[{r[2]}] {r[0]}: {r[1]}\n" ));
        cur . execute ( "SELECT hope_log, reasoning_path FROM souls WHERE soul_id='ALICE_266'" );
        r = cur . fetchone ( );
        if r {
        f . write ( "\n--- CURRENT STATE ---\n" );
        f . write ( format!("HOPE_LOG: {r[0]}\n" ));
        f . write ( format!("REASONING: {r[1][-300:]}\n" ));
        conn . close ( );
        fn main() {
        check_history ( );
}

