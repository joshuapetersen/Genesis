//! calltip_w.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Label, LEFT, SOLID, TclError};
// use crate::idlelib::{TooltipBase};
// use crate::unittest::{main};

pub const HIDE_EVENT: &str = "<<calltipwindow-hide>>";
pub const HIDE_SEQUENCES: &str = ("<Key-Escape>" ,"<FocusOut>" );
pub const CHECKHIDE_EVENT: &str = "<<calltipwindow-checkhide>>";
pub const CHECKHIDE_SEQUENCES: &str = ("<KeyRelease>" ,"<ButtonRelease>" );
pub const CHECKHIDE_TIME: u64 = 100;
pub const MARK_RIGHT: &str = "calltipwindowregion_right";
pub struct CalltipWindow {
    pub label: String, // TODO: infer type
    pub text: String, // TODO: infer type
    pub parenline: String, // TODO: infer type
    pub parencol: String, // TODO: infer type
    pub lastline: String, // TODO: infer type
    pub hideid: String, // TODO: infer type
    pub checkhideid: String, // TODO: infer type
    pub checkhide_after_id: String, // TODO: infer type
}

impl CalltipWindow {
}

pub fn _calltip_window(parent: &str) {
        from tkinter import Toplevel , Text , LEFT , BOTH;
        top = Toplevel ( parent );
        top . title ( "Test call-tips" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "250x100+%d+%d" % ( x + 175 , y + 150 ) );
        text = Text ( top );
        text . pack ( side = LEFT , fill = BOTH , expand = 1 );
        text . insert ( "insert" , "string.split" );
        top . update ( );
        calltip = CalltipWindow ( text );
        pub fn calltip_show ( event )  {
        calltip . showtip ( "(s='Hello world')" , "insert" , "end" );
        pub fn calltip_hide ( event )  {
        calltip . hidetip ( );
        text . event_add ( "<<calltip-show>>" , "(" );
        text . event_add ( "<<calltip-hide>>" , ")" );
        text . bind ( "<<calltip-show>>" , calltip_show );
        text . bind ( "<<calltip-hide>>" , calltip_hide );
        text . focus_set ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_calltip_w" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _calltip_window );
}

