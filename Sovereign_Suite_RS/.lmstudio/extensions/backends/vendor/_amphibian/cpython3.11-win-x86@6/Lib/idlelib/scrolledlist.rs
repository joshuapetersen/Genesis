//! scrolledlist.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{};
// use crate::idlelib::{macosx};
// use crate::unittest::{main};

pub struct ScrolledList {
    pub master: String, // TODO: infer type
    pub frame: String, // TODO: infer type
    pub vbar: String, // TODO: infer type
    pub listbox: String, // TODO: infer type
    pub empty: String, // TODO: infer type
    pub menu: String, // TODO: infer type
}

impl ScrolledList {
}

pub fn _scrolled_list(parent: &str) {
        top = Toplevel ( parent );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x + 200 , y + 175 ) );
        class MyScrolledList ( ScrolledList ) ;
        pub fn fill_menu ( self )  {  self . menu . add_command ( label = "right click" ); }
        pub fn on_select ( &self, index )  {  print ( "select" , self . get ( index ) ); }
        pub fn on_double ( &self, index )  {  print ( "double" , self . get ( index ) ); }
        scrolled_list = MyScrolledList ( top );
        for i in range ( 30 ) .iter() {
        scrolled_list . append ( "Item %02d" % i );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_scrolledlist" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _scrolled_list );
}

