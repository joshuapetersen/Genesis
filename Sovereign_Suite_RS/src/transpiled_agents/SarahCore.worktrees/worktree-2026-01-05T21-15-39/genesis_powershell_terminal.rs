//! genesis_powershell_terminal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::subprocess;

pub fn run_powershell(cmd: &str) {
        "Run a PowerShell command && return its output.";
        // try {
        result = subprocess . run ( [;
        "powershell" , "-Command" , cmd;
        ] , capture_output = true , text = true );
        println!( f "PS> {cmd}" );
        println!( result . stdout );
        if result . stderr {
        println!( "[stderr]" , result . stderr );
        return result . returncode;
        // } catch  Exception as e  {
        println!( f "Error running PowerShell command: {e}" );
        return -1;
        pub fn interactive_shell ( ) {
        println!( "Genesis PowerShell Terminal (Local Only). Type 'exit' to quit." );
        while true  {
        cmd = input ( "genesis-ps> " );
        if cmd . strip ( ) . lower ( ) in ( "exit" , "quit" ) {
        println!( "Exiting Genesis PowerShell terminal." );
        break;
        run_powershell ( cmd );
        fn main() {
        interactive_shell ( );
}

