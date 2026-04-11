//! planet_and_moon.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{Shape, Turtle, mainloop, Vec2D, Vec};

pub const G: u64 = 8;
pub struct GravSys {
    pub planets: String, // TODO: infer type
    pub t: String, // TODO: infer type
    pub dt: String, // TODO: infer type
    pub m: String, // TODO: infer type
    pub v: String, // TODO: infer type
    pub gravSys: String, // TODO: infer type
    pub a: String, // TODO: infer type
}

impl GravSys {
    pub fn new() -> Self {
        self . planets = [ ];
        self . t = 0;
        self . dt = 0.01;
        pub fn init ( self )  {
        for p in self . planets .iter() {
        p . init ( );
        pub fn start ( self )  {
        for i in range ( 10000 ) .iter() {
        self . t + = self . dt;
        for p in self . planets .iter() {
        p . step ( );
    }

    pub fn main(&self) {
        s = Turtle ( );
        s . reset ( );
        s . getscreen ( ) . tracer ( 0 , 0 );
        s . ht ( );
        s . pu ( );
        s . fd ( 6 );
        s . lt ( 90 );
        s . begin_poly ( );
        s . circle ( 6 , 180 );
        s . end_poly ( );
        m1 = s . get_poly ( );
        s . begin_poly ( );
        s . circle ( 6 , 180 );
        s . end_poly ( );
        m2 = s . get_poly ( );
        planetshape = Shape ( "compound" );
        planetshape . addcomponent ( m1 , "orange" );
        planetshape . addcomponent ( m2 , "blue" );
        s . getscreen ( ) . register_shape ( "planet" , planetshape );
        s . getscreen ( ) . tracer ( 1 , 0 );
        gs = GravSys ( );
        sun = Star ( 1000000 , Vec ( 0 , 0 ) , Vec ( 0 , -2.5 ) , gs , "circle" );
        sun . color ( "yellow" );
        sun . shapesize ( 1.8 );
        sun . pu ( );
        earth = Star ( 12500 , Vec ( 210 , 0 ) , Vec ( 0 , 195 ) , gs , "planet" );
        earth . pencolor ( "green" );
        earth . shapesize ( 0.8 );
        moon = Star ( 1 , Vec ( 220 , 0 ) , Vec ( 0 , 295 ) , gs , "planet" );
        moon . pencolor ( "blue" );
        moon . shapesize ( 0.5 );
        gs . init ( );
        gs . start ( );
        return  "Done!";
        fn main() {
        main ( );
        mainloop ( );
    }

}

