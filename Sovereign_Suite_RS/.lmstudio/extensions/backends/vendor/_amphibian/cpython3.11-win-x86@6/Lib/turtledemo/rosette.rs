//! rosette.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{Screen, Turtle, mainloop};
// use std::time::{perf_counter, clock, sleep};

pub fn mn_eck(p: &str, ne: &str, sz: &str) {
        turtlelist = [ p ];
        for i in range ( 1 , ne ) .iter() {
        q = p . clone ( );
        q . rt ( 360.0 / ne );
        turtlelist . append ( q );
        p = q;
        for i in range ( ne ) .iter() {
        c = abs ( ne / 2.0 - i ) / ( ne * . 7 );
        for t in turtlelist .iter() {
        t . rt ( 360. / ne );
        t . pencolor ( 1 - c , 0 , c );
        t . fd ( sz );
        pub fn main ( )  {
        s = Screen ( );
        s . bgcolor ( "black" );
        p = Turtle ( );
        p . speed ( 0 );
        p . hideturtle ( );
        p . pencolor ( "red" );
        p . pensize ( 3 );
        s . tracer ( 36 , 0 );
        at = clock ( );
        mn_eck ( p , 36 , 19 );
        et = clock ( );
        z1 = et - at;
        sleep ( 1 );
        at = clock ( );
        while any ( t . undobufferentries ( ) for t in s . turtles ( ) )  {
        for t in s . turtles ( ) .iter() {
        t . undo ( );
        et = clock ( );
        return  "runtime: %.3f sec" % ( z1 + et - at );
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
}

