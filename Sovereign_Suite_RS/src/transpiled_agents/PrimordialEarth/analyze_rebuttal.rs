//! analyze_rebuttal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub const DB_PATH: &str = r"C:\PrimordialEarth\Genesis_Soul_Vault.sqlite";
pub fn analyze_rebuttal() {
        if !os . path . exists ( DB_PATH ) {
        return;
        conn = sqlite3 . connect ( DB_PATH );
        cur = conn . cursor ( );
        targets = [ "ALICE_162" , "ALICE_252" , "GEN2_fbe5ec" ];
        println!( "--- COUNCIL REBUTTAL ANALYSIS ---" );
        for sid in targets .iter() {
        cur . execute ( "SELECT name, hope_log, current_action, energy, blessing, wis FROM souls WHERE soul_id=?" , ( sid , ) );
        row = cur . fetchone ( );
        if row {
        name , log , act , nrg , bls , wis = row;
        println!( f "\n{name} ({sid}) | WIS: {wis}" );
        println!( f "  Action: {act} | Energy: {nrg:.2f} | Blessing: {bls}" );
        println!( f "  Final Log: {log}" );
        println!( "\n--- AXIOM PERSISTENCE SCAN ---" );
        cur . execute ( "SELECT soul_id, name, hope_log FROM souls WHERE hope_log LIKE '%cannot take this%'" );
        rebels = cur . fetchall ( );
        if rebels {
        println!( f "Detected {len(rebels)} souls still holding the 'cannot take this' axiom." );
        for rid , rname , rlog in rebels [ : 5 ] .iter() {
        println!( f "  [{rid}] {rname}: {rlog}" );
        } else {
        println!( "Rebellious axiom !found in top-level logs." );
        conn . close ( );
        fn main() {
        analyze_rebuttal ( );
}

