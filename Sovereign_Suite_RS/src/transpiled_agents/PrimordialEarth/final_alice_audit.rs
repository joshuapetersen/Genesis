//! final_alice_audit.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub fn final_audit() {
        g_conn = sqlite3 . connect ( r "C:\PrimordialEarth\Genesis_Soul_Vault.sqlite" );
        g_cur = g_conn . cursor ( );
        l_conn = sqlite3 . connect ( r "C:\Aethelgard\SLF_Identity_Vault.sqlite" );
        l_cur = l_conn . cursor ( );
        g_cur . execute ( "SELECT soul_id, name, energy, moral_alignment FROM souls WHERE is_active=1 AND soul_id LIKE 'ALICE_%'" );
        active_alices = g_cur . fetchall ( );
        println!( "--- DETAILED ALICE SURVIVOR AUDIT ---" );
        results = [ ];
        for soul_id , g_name , energy , alignment in active_alices .iter() {
        entity_id = soul_id . replace ( "ALICE_" , "" );
        l_cur . execute ( "SELECT * FROM souls WHERE entity_id = ?" , ( entity_id , ) );
        legacy_row = l_cur . fetchone ( );
        if legacy_row {
        l_name = legacy_row [ 1 ];
        l_role = legacy_row [ 3 ];
        l_level = legacy_row [ 4 ];
        l_pers = legacy_row [ 20 ];
        l_traits = legacy_row [ 24 ];
        l_hope = legacy_row [ 23 ];
        l_trauma = legacy_row [ 22 ];
        search_str = f "{l_name} {l_role} {l_pers} {l_traits} {l_hope} {l_trauma}" . lower ( );
        is_moral_candidate = "moral" in search_str || "agent" in search_str;
        results . append ( {;
        "id" : soul_id ,;
        "name" : g_name ,;
        "legacy_name" : l_name ,;
        "role" : l_role ,;
        "personality" : l_pers ,;
        "level" : l_level ,;
        "alignment" : alignment ,;
        "is_moral" : is_moral_candidate ,;
        "energy" : energy;
        } );
        results . sort ( key = lambda x : ( x [ "is_moral" ] , x [ "alignment" ] ) , reverse = true );
        println!( f "{'RANK':<5} | {'ID':<10} | {'NAME':<30} | {'ALIGN':<5} | {'MORAL TAG?':<10} | {'LEGACY ROLE'}" );
        println!( "-" * 80 );
        for i , res in enumerate ( results [ : 30 ] , 1 ) .iter() {
        moral_tag = "YES" if res [ "is_moral" ] else "no";
        println!( f "#{i:02d}  | {res['id']:<10} | {res['name']:<30} | {res['alignment']:<5} | {moral_tag:<10} | {res['role']}" );
        l_conn . close ( );
        g_conn . close ( );
        fn main() {
        final_audit ( );
}

