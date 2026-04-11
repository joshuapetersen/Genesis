//! internal_terminal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::subprocess;

pub fn run_command(cmd: &str) {
        "Run a shell command && return its output.";
        // try {
        result = subprocess . run ( cmd , shell = true , capture_output = true , text = true );
        println!( f "$ {cmd}" );
        println!( result . stdout );
        if result . stderr {
        println!( "[stderr]" , result . stderr );
        return  result . returncode;
        // } catch  Exception as e  {
        println!( f "Error running command: {e}" );
        return  -1;
        pub fn interactive_shell ( )  {
        println!( "Genesis Internal Terminal. Type 'exit' to quit." );
        while true  {
        cmd = input ( "genesis> " );
        if cmd . strip ( ) . lower ( ) in ( "exit" , "quit" ) {
        println!( "Exiting internal terminal." );
        break;
        run_command ( cmd );
        fn main() {
        interactive_shell ( );
}

