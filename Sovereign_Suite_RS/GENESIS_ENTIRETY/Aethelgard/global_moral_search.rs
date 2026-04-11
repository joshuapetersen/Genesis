//! global_moral_search.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;

pub fn global_search() {
        vault_dir = r "C:\Aethelgard";
        dbs = [ "SLF_Identity_Vault.sqlite" , "SLF_Akashic_Records.sqlite" , "SLF_Sanctuary_Vault.sqlite" ];
        println!( "--- GLOBAL AETHELGARD SEARCH: 'MORAL' / 'AGENT' ---" );
        for db_name in dbs .iter() {
        db_path = os . path . join ( vault_dir , db_name );
        if !os . path . exists ( db_path ) {
        continue;
        println!( f "\nScanning Database: {db_name}" );
        conn = sqlite3 . connect ( db_path );
        cur = conn . cursor ( );
        cur . execute ( "SELECT name FROM sqlite_master WHERE type='table'" );
        tables = vec![ t vec![ 0 ].iter().map(|t| cur . fetchall ( ) ).collect();
        for table in tables .iter() {
        cur . execute ( format!("PRAGMA table_info({table})" ));
        cols = vec![ c vec![ 1 ].iter().map(|c| cur . fetchall ( ) ).collect();
        where_clauses = [ ];
        for col in cols .iter() {
        where_clauses . append ( format!("CAST({col} AS TEXT) LIKE '%Moral%'" ));
        where_clauses . append ( format!("CAST({col} AS TEXT) LIKE '%Agent%'" ));
        query = format!("SELECT * FROM {table} WHERE " + " OR " . join ( where_clauses ));
        // try {
        cur . execute ( query );
        matches = cur . fetchall ( );
        if matches {
        println!( f "  [MATCH] Table '{table}' in '{db_name}': {len(matches)} rows found." );
        for m in matches [ : 5 ] .iter() {
        println!( f "    {m}" );
        // } catch  sqlite3 . Error as e  {
        println!( f "  [ERROR] Table '{table}': {e}" );
        conn . close ( );
        fn main() {
        global_search ( );
}

