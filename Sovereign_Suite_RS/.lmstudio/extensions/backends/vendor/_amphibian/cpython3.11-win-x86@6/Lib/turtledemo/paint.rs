//! paint.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::turtle::{};

pub fn switchupdown(x: &str, y: &str) {
        if pen ( ) [ "pendown" ] {
        end_fill ( );
        up ( );
        } else {
        down ( );
        begin_fill ( );
        pub fn changecolor ( x = 0 , y = 0 )  {
        global colors;
        colors = colors [ 1 : ] + colors [ : 1 ];
        color ( colors [ 0 ] );
        pub fn main ( )  {
        global colors;
        shape ( "circle" );
        resizemode ( "user" );
        shapesize ( . 5 );
        width ( 3 );
        colors = [ "red" , "green" , "blue" , "yellow" ];
        color ( colors [ 0 ] );
        switchupdown ( );
        onscreenclick ( goto , 1 );
        onscreenclick ( changecolor , 2 );
        onscreenclick ( switchupdown , 3 );
        return  "EVENTLOOP";
        fn main() {
        msg = main ( );
        println!( msg );
        mainloop ( );
}

