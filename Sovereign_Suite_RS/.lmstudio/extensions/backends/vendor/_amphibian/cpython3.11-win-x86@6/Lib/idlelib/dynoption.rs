//! dynoption.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{OptionMenu, _setit, StringVar, Button};
// use crate::idlelib::{run};

pub struct DynOptionMenu {
    pub variable: String, // TODO: infer type
    pub command: String, // TODO: infer type
}

impl DynOptionMenu {
    pub fn new(master: &str, variable: &str, value: &str, values: &str, kwargs: &str) -> Self {
        highlightthickness = kwargs . pop ( "highlightthickness" , None /* Option */ );
        OptionMenu . __init__ ( self , master , variable , value , * values , ** kwargs );
        self [ "highlightthickness" ] = highlightthickness;
        self . variable = variable;
        self . command = kwargs . get ( "command" );
    }

    pub fn _dyn_option_menu(&self, parent: &str) {
        from tkinter import Toplevel;
        top = Toplevel ( parent );
        top . title ( "Test dynamic option menu" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "200x100+%d+%d" % ( x + 250 , y + 175 ) );
        top . focus_set ( );
        var = StringVar ( top );
        var . set ( "Old option set" );
        dyn = DynOptionMenu ( top , var , "old1" , "old2" , "old3" , "old4" ,;
        highlightthickness = 5 );
        dyn . pack ( );
        pub fn update ( )  {
        dyn . SetMenu ( [ "new1" , "new2" , "new3" , "new4" ] , value = "new option set" );
        button = Button ( top , text = "Change option set" , command = update );
        button . pack ( );
        fn main() {
        from idlelib . idle_test . htest import run;
        run ( _dyn_option_menu );
    }

}

