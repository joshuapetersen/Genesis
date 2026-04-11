//! two_canvases.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{TurtleScreen, RawTurtle, TK};

pub fn main() {
        root = TK . Tk ( );
        cv1 = TK . Canvas ( root , width = 300 , height = 200 , bg = "#ddfffformat!(" ));
        cv2 = TK . Canvas ( root , width = 300 , height = 200 , bg = "#ffeeee" );
        cv1 . pack ( );
        cv2 . pack ( );
        s1 = TurtleScreen ( cv1 );
        s1 . bgcolor ( 0.85 , 0.85 , 1 );
        s2 = TurtleScreen ( cv2 );
        s2 . bgcolor ( 1 , 0.85 , 0.85 );
        p = RawTurtle ( s1 );
        q = RawTurtle ( s2 );
        p . color ( "red" , ( 1 , 0.85 , 0.85 ) );
        p . width ( 3 );
        q . color ( "blue" , ( 0.85 , 0.85 , 1 ) );
        q . width ( 3 );
        for t in p , q .iter() {
        t . shape ( "turtle" );
        t . lt ( 36 );
        q . lt ( 180 );
        for t in p , q .iter() {
        t . begin_fill ( );
        for i in range ( 5 ) .iter() {
        for t in p , q .iter() {
        t . fd ( 50 );
        t . lt ( 72 );
        for t in p , q .iter() {
        t . end_fill ( );
        t . lt ( 54 );
        t . pu ( );
        t . bk ( 50 );
        return  "EVENTLOOP";
        fn main() {
        main ( );
        TK . mainloop ( );
}

