//! fractalcurves.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};
// use std::time::{sleep, perf_counter, clock};
// use std::f64::consts;

pub struct CurvesTurtle {
}

impl CurvesTurtle {
    pub fn hilbert(&self, size: &str, level: &str, parity: &str) {
        if level == 0 {
        return;
        self . left ( parity * 90 );
        self . hilbert ( size , level - 1 , - parity );
        self . forward ( size );
        self . right ( parity * 90 );
        self . hilbert ( size , level - 1 , parity );
        self . forward ( size );
        self . hilbert ( size , level - 1 , parity );
        self . right ( parity * 90 );
        self . forward ( size );
        self . hilbert ( size , level - 1 , - parity );
        self . left ( parity * 90 );
    }

    pub fn main(&self) {
        ft = CurvesTurtle ( );
        ft . reset ( );
        ft . speed ( 0 );
        ft . ht ( );
        ft . getscreen ( ) . tracer ( 1 , 0 );
        ft . pu ( );
        size = 6;
        ft . setpos ( -33 * size , -32 * size );
        ft . pd ( );
        ta = clock ( );
        ft . fillcolor ( "red" );
        ft . begin_fill ( );
        ft . fd ( size );
        ft . hilbert ( size , 6 , 1 );
        ft . fd ( size );
        for i in range ( 3 ) .iter() {
        ft . lt ( 90 );
        ft . fd ( size * ( 64 + i % 2 ) );
        ft . pu ( );
        for i in range ( 2 ) .iter() {
        ft . fd ( size );
        ft . rt ( 90 );
        ft . pd ( );
        for i in range ( 4 ) .iter() {
        ft . fd ( size * ( 66 + i % 2 ) );
        ft . rt ( 90 );
        ft . end_fill ( );
        tb = clock ( );
        res = "Hilbert: %.2fsec. " % ( tb - ta );
        sleep ( 3 );
        ft . reset ( );
        ft . speed ( 0 );
        ft . ht ( );
        ft . getscreen ( ) . tracer ( 1 , 0 );
        ta = clock ( );
        ft . color ( "black" , "blue" );
        ft . begin_fill ( );
        ft . fractalgon ( 3 , 250 , 4 , 1 );
        ft . end_fill ( );
        ft . begin_fill ( );
        ft . color ( "red" );
        ft . fractalgon ( 3 , 200 , 4 , -1 );
        ft . end_fill ( );
        tb = clock ( );
        res + = "Koch: %.2fsec." % ( tb - ta );
        return  res;
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
    }

}

