//! colormixer.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{Screen, Turtle, mainloop};

pub struct ColorTurtle {
    pub _color: String, // TODO: infer type
    pub x: String, // TODO: infer type
}

impl ColorTurtle {
    pub fn new(x: &str, y: &str) -> Self {
        Turtle . __init__ ( self );
        self . shape ( "turtle" );
        self . resizemode ( "user" );
        self . shapesize ( 3 , 3 , 5 );
        self . pensize ( 10 );
        self . _color = [ 0 , 0 , 0 ];
        self . x = x;
        self . _color [ x ] = y;
        self . color ( self . _color );
        self . speed ( 0 );
        self . left ( 90 );
        self . pu ( );
        self . goto ( x , 0 );
        self . pd ( );
        self . sety ( 1 );
        self . pu ( );
        self . sety ( y );
        self . pencolor ( "gray25" );
        self . ondrag ( self . shift );
    }

    pub fn setbgcolor(&self) {
        screen . bgcolor ( red . ycor ( ) , green . ycor ( ) , blue . ycor ( ) );
        pub fn main ( )  {
        global screen , red , green , blue;
        screen = Screen ( );
        screen . delay ( 0 );
        screen . setworldcoordinates ( -1 , -0.3 , 3 , 1.3 );
        red = ColorTurtle ( 0 , . 5 );
        green = ColorTurtle ( 1 , . 5 );
        blue = ColorTurtle ( 2 , . 5 );
        setbgcolor ( );
        writer = Turtle ( );
        writer . ht ( );
        writer . pu ( );
        writer . goto ( 1 , 1.15 );
        writer . write ( "DRAG!" , align = "center" , font = ( "Arial" , 30 , ( "bold" , "italic" ) ) );
        return  "EVENTLOOP";
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
    }

}

