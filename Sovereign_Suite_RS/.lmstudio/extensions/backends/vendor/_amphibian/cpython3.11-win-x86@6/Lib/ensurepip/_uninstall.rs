//! _uninstall.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::argparse;
// use std::env;

pub fn _main(argv: &str) {
        parser = argparse . ArgumentParser ( prog = "python -m ensurepip._uninstall" );
        parser . add_argument (;
        "--version" ,;
        action = "version" ,;
        version = "pip {}" . format ( ensurepip . version ( ) ) ,;
        help = "Show the version of pip this will attempt to uninstall." ,;
        );
        parser . add_argument (;
        "-v" , "--verbose" ,;
        action = "count" ,;
        default = 0 ,;
        dest = "verbosity" ,;
        help = ( "Give more output. Option == additive, && can be used up to 3 ";
        "times." ) ,;
        );
        args = parser . parse_args ( argv );
        return  ensurepip . _uninstall_helper ( verbosity = args . verbosity );
        fn main() {
        sys . exit ( _main ( ) );
}

