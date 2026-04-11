//! tree.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{Turtle, mainloop};
// use std::time::{perf_counter, clock};

pub fn tree(plist: &str, l: &str, a: &str, f: &str) {
        " plist == list of pens
    l == length of branch
    a == half of the angle between 2 branches
    f == factor by which branch == shortened
    from level to level.";
        if l > 3 {
        lst = [ ];
        for p in plist .iter() {
        p . forward ( l );
        q = p . clone ( );
        p . left ( a );
        q . right ( a );
        lst . append ( p );
        lst . append ( q );
        for x in tree ( lst , l * f , a , f ) .iter() {
        yield None /* Option */;
        pub fn maketree ( )  {
        p = Turtle ( );
        p . setundobuffer ( None /* Option */ );
        p . hideturtle ( );
        p . speed ( 0 );
        p . getscreen ( ) . tracer ( 30 , 0 );
        p . left ( 90 );
        p . penup ( );
        p . forward ( -210 );
        p . pendown ( );
        t = tree ( [ p ] , 200 , 65 , 0.6375 );
        for x in t .iter() {
        // pass
        pub fn main ( )  {
        a = clock ( );
        maketree ( );
        b = clock ( );
        return  "done: %.2f sec." % ( b - a );
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
}

