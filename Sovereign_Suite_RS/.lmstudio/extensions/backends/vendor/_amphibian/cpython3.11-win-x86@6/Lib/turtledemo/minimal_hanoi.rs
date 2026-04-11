//! minimal_hanoi.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};

pub struct Disc {
    pub x: String, // TODO: infer type
}

impl Disc {
    pub fn new(n: &str) -> Self {
        Turtle . __init__ ( self , shape = "square" , visible = false );
        self . pu ( );
        self . shapesize ( 1.5 , n * 1.5 , 2 );
        self . fillcolor ( n / 6. , 0 , 1 - n / 6. );
        self . st ( );
    }

    pub fn hanoi(&self, n: &str, from_: &str, with_: &str, to_: &str) {
        if n > 0 {
        hanoi ( n -1 , from_ , to_ , with_ );
        to_ . push ( from_ . pop ( ) );
        hanoi ( n -1 , with_ , from_ , to_ );
        pub fn play ( )  {
        onkey ( None /* Option */ , "space" );
        clear ( );
        // try {
        hanoi ( 6 , t1 , t2 , t3 );
        write ( "press STOP button to exit" ,;
        align = "center" , font = ( "Courier" , 16 , "bold" ) );
        // } catch  Terminator  {
        // pass
        pub fn main ( )  {
        global t1 , t2 , t3;
        ht ( ) ; penup ( ) ; goto ( 0 , -225 );
        t1 = Tower ( -250 );
        t2 = Tower ( 0 );
        t3 = Tower ( 250 );
        for i in range ( 6 , 0 , -1 ) .iter() {
        t1 . push ( Disc ( i ) );
        write ( "press spacebar to start game" ,;
        align = "center" , font = ( "Courier" , 16 , "bold" ) );
        onkey ( play , "space" );
        listen ( );
        return  "EVENTLOOP";
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
    }

}

