//! yinyang.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};

pub fn yin(radius: &str, color1: &str, color2: &str) {
        width ( 3 );
        color ( "black" , color1 );
        begin_fill ( );
        circle ( radius / 2. , 180 );
        circle ( radius , 180 );
        left ( 180 );
        circle ( - radius / 2. , 180 );
        end_fill ( );
        left ( 90 );
        up ( );
        forward ( radius * 0.35 );
        right ( 90 );
        down ( );
        color ( color1 , color2 );
        begin_fill ( );
        circle ( radius * 0.15 );
        end_fill ( );
        left ( 90 );
        up ( );
        backward ( radius * 0.35 );
        down ( );
        left ( 90 );
        pub fn main ( )  {
        reset ( );
        yin ( 200 , "black" , "white" );
        yin ( 200 , "white" , "black" );
        ht ( );
        return  "Done!";
        fn main() {
        main ( );
        mainloop ( );
}

