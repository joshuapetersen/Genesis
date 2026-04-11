//! activate_archive_memories.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use rusqlite;
// use chrono::Utc;

pub fn activate_archive_memories(db_path: &str) {
        conn = sqlite3 . connect ( db_path );
        cursor = conn . cursor ( );
        cursor . execute ( "CREATE TABLE IF NOT EXISTS active_memory (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        timestamp TEXT,
        data TEXT,
        source TEXT
    )" );
        cursor . execute ( "SELECT name FROM sqlite_master WHERE type='table' AND name='archive_memories'" );
        if !cursor . fetchone ( ) {
        println!( "No archive_memories table found. Nothing to activate." );
        return;
        cursor . execute ( "SELECT id, data, source FROM archive_memories" );
        rows = cursor . fetchall ( );
        for row in rows .iter() {
        _ , data , source = row;
        ts = datetime . now ( ) . isoformat ( );
        cursor . execute ( "INSERT INTO active_memory (timestamp, data, source) VALUES (?, ?, ?)" , ( ts , data , source ) );
        conn . commit ( );
        println!( f "Moved {len(rows)} archive memories into active memory." );
        conn . close ( );
        fn main() {
        activate_archive_memories ( );
}

