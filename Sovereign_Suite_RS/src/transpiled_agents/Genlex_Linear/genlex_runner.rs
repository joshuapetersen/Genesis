//! genlex_runner.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::io;
// use crate::all_engine::{GenlexLinearRuntime};

pub fn main() {
        if len ( sys . argv ) < 2 {
        println!( "GENLEX NATIVE RUNNER v1.0" );
        println!( "Usage: genlex_runner.exe <file.all | file.cgl>" );
        input ( "\nPress Enter to exit..." );
        sys . exit ( 1 );
        target_file = sys . argv [ 1 ];
        if !os . path . exists ( target_file ) {
        println!( f "[ ERROR ] File !found: {target_file}" );
        input ( "\nPress Enter to exit..." );
        sys . exit ( 1 );
        println!( f "--- NATIVE GENLEX EXECUTION: {os.path.basename(target_file)} ---" );
        // try {
        runtime = GenlexLinearRuntime ( );
        runtime . run ( target_file );
        // } catch  Exception as e  {
        println!( f "[ CRITICAL ERROR ] {e}" );
        println!( "\n--- EXECUTION FINISHED ---" );
        input ( "Press Enter to close terminal..." );
        fn main() {
        main ( );
}

