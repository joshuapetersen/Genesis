//! bytedesign.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{Turtle, mainloop};
// use std::time::{perf_counter, clock};

pub struct Designer {
}

impl Designer {
    pub fn design(&self, homePos: &str, scale: &str) {
        self . up ( );
        for i in range ( 5 ) .iter() {
        self . forward ( 64.65 * scale );
        self . down ( );
        self . wheel ( self . position ( ) , scale );
        self . up ( );
        self . backward ( 64.65 * scale );
        self . right ( 72 );
        self . up ( );
        self . goto ( homePos );
        self . right ( 36 );
        self . forward ( 24.5 * scale );
        self . right ( 198 );
        self . down ( );
        self . centerpiece ( 46 * scale , 143.4 , scale );
        self . getscreen ( ) . tracer ( true );
    }

    pub fn main(&self) {
        t = Designer ( );
        t . speed ( 0 );
        t . hideturtle ( );
        t . getscreen ( ) . delay ( 0 );
        t . getscreen ( ) . tracer ( 0 );
        at = clock ( );
        t . design ( t . position ( ) , 2 );
        et = clock ( );
        return  "runtime: %.2f sec." % ( et - at );
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
    }

}

