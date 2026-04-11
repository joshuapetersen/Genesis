//! test_reality.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::csv;
// use crate::all_engine::{GenlexLinearRuntime};

pub fn test_reality() {
        println!( "[REALITY CHECK] INITIALIZING GENLEX RUNTIME..." );
        engine = GenlexLinearRuntime ( );
        script_path = r "C:\Genlex_Core\genesis_one.all";
        if !os . path . exists ( script_path ) {
        println!( f "[ERROR] SCRIPT NOT FOUND: {script_path}" );
        return;
        println!( f "[REALITY CHECK] EXECUTING: {script_path}" );
        println!( "-" * 50 );
        // with scope: open ( script_path , "r" , encoding = "utf-8" ) as f  {
        for line in f .iter() {
        clean = line . split ( "#" ) [ 0 ] . strip ( );
        if !clean { : continue; }
        tokens = clean . split ( );
        for t in tokens .iter() {
        if t in engine . lexicon {
        op = engine . lexicon [ t ] [ "op" ];
        println!( f "[RUN] GLYPH: {t} -> OP: {op}" );
        } else {
        // try {
        float ( t );
        println!( f "[RUN] DATA: {t} -> STACK_PUSH" );
        // } catch   {
        println!( f "[RUN] TOKEN: {t} (STRICT)" );
        println!( "-" * 50 );
        println!( "[REALITY CHECK] EXECUTION COMPLETE. THIS IS PHYSICAL CODE." );
        fn main() {
        test_reality ( );
}

