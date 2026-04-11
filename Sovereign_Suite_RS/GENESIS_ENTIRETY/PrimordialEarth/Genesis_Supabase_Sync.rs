//! Genesis_Supabase_Sync.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;
// use std::time;
// use serde_json;
// use crate::supabase::{create_client};

pub const SUPABASE_URL: &str = os . environ . get ("SUPABASE_URL" ,"https://duuycxgqbhrqmwapnjhk.supabase.co" );
pub const SUPABASE_KEY: &str = os . environ . get ("SUPABASE_SERVICE_KEY" ) or os . environ . get ("SUPABASE_KEY" ,"" );
pub const LOCAL_DB: &str = os . environ . get ("GENESIS_DATA" , r"C:\PrimordialEarth" ) + r"\Genesis_Soul_Vault.sqlite";
pub const SYNC_INTERVAL_TICKS: u64 = 10;
pub const _sync_enabled: f64 = False;
pub const _supabase_client: f64 = None;
pub fn init_sync() {
        "Initialize Supabase connection. Call once at engine startup.";
        global _sync_enabled , _supabase_client;
        // try {
        from supabase import create_client;
        _supabase_client = create_client ( SUPABASE_URL , SUPABASE_KEY );
        _sync_enabled = true;
        println!( f "[SUPABASE] Soul Vault sync ACTIVE >> {SUPABASE_URL}" );
        _ensure_table ( );
        // } catch  ImportError  {
        println!( "[SUPABASE] supabase-py !installed. Run: pip install supabase" );
        // } catch  Exception as e  {
        println!( f "[SUPABASE] Init failed: {e}" );
        pub fn _ensure_table ( )  {
        "Check that the souls table exists in Supabase (create via dashboard if not).";
        // try {
        result = _supabase_client . table ( "souls" ) . select ( "soul_id" ) . limit ( 1 ) . execute ( );
        println!( f "[SUPABASE] souls table confirmed." );
        // } catch  Exception as e  {
        println!( f "[SUPABASE] Table check failed: {e}" );
        println!( "[SUPABASE] → Create the table via Supabase dashboard SQL editor." );
        println!( "            → Use: PrimordialEarth/schema_supabase.sql" );
        pub fn sync_tick ( tick  {  int ) ; }
        "Called from the engine. Triggers async sync every SYNC_INTERVAL_TICKS.";
        if !_sync_enabled {
        return;
        if tick % SYNC_INTERVAL_TICKS != 0 {
        return;
        t = threading . Thread ( target = _do_sync , daemon = true );
        t . start ( );
        pub fn _do_sync ( )  {
        "Background: read local SQLite, upsert to Supabase.";
        if !_supabase_client {
        return;
        // try {
        conn = sqlite3 . connect ( format!("file:{LOCAL_DB}?mode=ro" , uri = true ));
        cur = conn . cursor ( );
        cur . execute ( "
            SELECT soul_id, genome, x, y, energy, moral_alignment, is_active,
                   species, generation, current_action, vit, str, agi, int_stat,
                   wis, luk, blessing, leader_id, hope_log, reasoning_path,
                   name, divine_mandate, pregnancy_timer, age_ticks
            FROM souls WHERE is_active=1 LIMIT 500
        " );
        rows = cur . fetchall ( );
        cols = [ "soul_id" , "genome" , "x" , "y" , "energy" , "moral_alignment" , "is_active" ,;
        "species" , "generation" , "current_action" , "vit" , "str" , "agi" , "int_stat" ,;
        "wis" , "luk" , "blessing" , "leader_id" , "hope_log" , "reasoning_path" ,;
        "name" , "divine_mandate" , "pregnancy_timer" , "age_ticks" ];
        conn . close ( );
        records = [ ];
        for row in rows .iter() {
        record = dict ( zip ( cols , row ) );
        if record . get ( "reasoning_path" ) {
        record [ "reasoning_path" ] = record [ "reasoning_path" ] [ -300 : ];
        records . append ( record );
        if records {
        for i in range ( 0 , len ( records ) , 100 ) .iter() {
        batch = records [ i : i + 100 ];
        for attempt in range ( 3 ) .iter() {
        // try {
        _supabase_client . table ( "souls" ) . upsert ( batch , on_conflict = "soul_id" ) . execute ( );
        break;
        // } catch  Exception as batch_err  {
        if attempt < 2 {
        time . sleep ( 2 ** attempt );
        } else {
        println!( f "[SUPABASE] Batch {i//100} failed after 3 attempts: {str(batch_err)[:80]}" );
        println!( f "[SUPABASE] Synced {len(records)} souls to cloud vault." );
        // } catch  Exception as e  {
        println!( f "[SUPABASE] Sync error: {e}" );
        pub fn sync_pantheon_event ( soul_id  {  str , event : str , data : dict ) ; }
        "Push a specific high-priority event immediately (e.g. Apotheosis, Full Authorship).";
        if !_sync_enabled || !_supabase_client {
        return;
        // try {
        _supabase_client . table ( "pantheon_events" ) . insert ( {;
        "soul_id" : soul_id ,;
        "event" : event ,;
        "data" : json . dumps ( data );
        } ) . execute ( );
        // } catch  Exception as e  {
        println!( f "[SUPABASE] Event push failed: {e}" );
}

