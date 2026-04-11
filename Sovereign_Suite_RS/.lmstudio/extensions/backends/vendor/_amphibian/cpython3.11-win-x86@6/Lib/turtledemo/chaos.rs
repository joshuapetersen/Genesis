//! chaos.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};

pub const N: u64 = 80;
pub fn f(x: &str) {
        return  3.9 * x * ( 1 - x );
        pub fn g ( x )  {
        return  3.9 * ( x - x ** 2 );
        pub fn h ( x )  {
        return  3.9 * x -3.9 * x * x;
        pub fn jumpto ( x , y )  {
        penup ( ) ; goto ( x , y );
        pub fn line ( x1 , y1 , x2 , y2 )  {
        jumpto ( x1 , y1 );
        pendown ( );
        goto ( x2 , y2 );
        pub fn coosys ( )  {
        line ( -1 , 0 , N + 1 , 0 );
        line ( 0 , -0.1 , 0 , 1.1 );
        pub fn plot ( fun , start , color )  {
        pencolor ( color );
        x = start;
        jumpto ( 0 , x );
        pendown ( );
        dot ( 5 );
        for i in range ( N ) .iter() {
        x = fun ( x );
        goto ( i + 1 , x );
        dot ( 5 );
        pub fn main ( )  {
        reset ( );
        setworldcoordinates ( -1.0 , -0.1 , N + 1 , 1.1 );
        speed ( 0 );
        hideturtle ( );
        coosys ( );
        plot ( f , 0.35 , "blue" );
        plot ( g , 0.35 , "green" );
        plot ( h , 0.35 , "red" );
        for s in range ( 100 ) .iter() {
        setworldcoordinates ( 0.5 * s , -0.1 , N + 1 , 1.1 );
        return  "Done!";
        fn main() {
        main ( );
        mainloop ( );
}

