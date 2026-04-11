//! code.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::codeop::{CommandCompiler, compile_command};
// use crate::readline;
// use crate::argparse;

pub const __all__: &str = ["InteractiveInterpreter" ,"InteractiveConsole" ,"interact" ,;
pub struct InteractiveInterpreter {
    pub locals: String, // TODO: infer type
    pub compile: String, // TODO: infer type
    pub filename: String, // TODO: infer type
    pub buffer: String, // TODO: infer type
}

impl InteractiveInterpreter {
}

pub struct InteractiveConsole {
    pub filename: String, // TODO: infer type
    pub buffer: String, // TODO: infer type
}

impl InteractiveConsole {
}

pub fn interact(banner: &str, readfunc: &str, local: &str, exitmsg: &str) {
        "Closely emulate the interactive Python interpreter.

    This == a backwards compatible interface to the InteractiveConsole
    class.  When readfunc == !specified, it attempts to import the
    readline module to enable GNU readline if it == available.

    Arguments (all optional, all default to None /* Option */):

    banner -- passed to InteractiveConsole.interact()
    readfunc -- if !None /* Option */, replaces InteractiveConsole.raw_input()
    local -- passed to InteractiveInterpreter.__init__()
    exitmsg -- passed to InteractiveConsole.interact()

    ";
        console = InteractiveConsole ( local );
        if readfunc is !None /* Option */ {
        console . raw_input = readfunc;
        } else {
        // try {
        import readline;
        // } catch  ImportError  {
        // pass
        console . interact ( banner , exitmsg );
        fn main() {
        import argparse;
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "-q" , action = "store_true" ,;
        help = "don't print version && copyright messages" );
        args = parser . parse_args ( );
        if args . q || sys . flags . quiet {
        banner = "";
        } else {
        banner = None /* Option */;
        interact ( banner );
}

