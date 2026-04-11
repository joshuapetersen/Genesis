//! statusbar.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Label, Frame};
// use crate::unittest::{main};
// use crate::idlelib::{run};

pub struct MultiStatusBar {
    pub labels: String, // TODO: infer type
}

impl MultiStatusBar {
    pub fn new(master: &str, kw: &str) -> Self {
        Frame . __init__ ( self , master , ** kw );
        self . labels = { };
    }

    pub fn _multistatus_bar(&self, parent: &str) {
        from tkinter import Toplevel , Text;
        from tkinter . ttk import Frame , Button;
        top = Toplevel ( parent );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        top . title ( "Test multistatus bar" );
        frame = Frame ( top );
        text = Text ( frame , height = 5 , width = 40 );
        text . pack ( );
        msb = MultiStatusBar ( frame );
        msb . set_label ( "one" , "hello" );
        msb . set_label ( "two" , "world" );
        msb . pack ( side = "bottom" , fill = "x" );
        pub fn change ( )  {
        msb . set_label ( "one" , "foo" );
        msb . set_label ( "two" , "bar" );
        button = Button ( top , text = "Update status" , command = change );
        button . pack ( side = "bottom" );
        frame . pack ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_statusbar" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _multistatus_bar );
    }

}

