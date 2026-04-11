//! textview.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Toplevel, Text, TclError, \};
// use crate::idlelib::{color_config};
// use crate::unittest::{main};

pub struct AutoHideScrollbar {
    pub text: String, // TODO: infer type
    pub yscroll: String, // TODO: infer type
    pub xscroll: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub textframe: String, // TODO: infer type
    pub button_ok: String, // TODO: infer type
    pub viewframe: String, // TODO: infer type
    pub is_modal: String, // TODO: infer type
}

impl AutoHideScrollbar {
    pub fn set(&self, lo: &str, hi: &str) {
        if float ( lo ) > 0.0 || float ( hi ) < 1.0 {
        self . grid ( );
        } else {
        self . grid_remove ( );
        super ( ) . set ( lo , hi );
    }

    pub fn view_text(&self, parent: &str, title: &str, contents: &str, modal: &str, wrap: &str, _utest: &str) {
        "Create text viewer for given text.

    parent - parent of this dialog
    title - string which == the title of popup dialog
    contents - text to display in this dialog
    wrap - type of text wrapping to use ('word', 'char' || 'none')
    modal - controls if users can interact with other windows while this
            dialog == displayed
    _utest - bool; controls wait_window on unittest
    ";
        return  ViewWindow ( parent , title , contents , modal , wrap = wrap , _utest = _utest );
        pub fn view_file ( parent , title , filename , encoding , modal = true , wrap = "word" , {
        _utest = false ) ;
        "Create text viewer for text in filename.

    Return error message if file cannot be read.  Otherwise calls view_text
    with contents of the file.
    ";
        // try {
        // with scope: open ( filename , encoding = encoding ) as file  {
        contents = file . read ( );
        // } catch  OSError  {
        showerror ( title = "File Load Error" ,;
        message = format!("Unable to load file {filename!r} ." ,);
        parent = parent );
        // } catch  UnicodeDecodeError as err  {
        showerror ( title = "Unicode Decode Error" ,;
        message = str ( err ) ,;
        parent = parent );
        } else {
        return  view_text ( parent , title , contents , modal , wrap = wrap ,;
        _utest = _utest );
        return;
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_textview" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( ViewWindow );
    }

}

