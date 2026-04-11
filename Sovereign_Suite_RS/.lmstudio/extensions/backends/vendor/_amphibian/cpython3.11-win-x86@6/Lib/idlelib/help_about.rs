//! help_about.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::webbrowser;
// use crate::python_version;
// use crate::tkinter::{Toplevel, Frame, Label, Button, PhotoImage};
// use crate::idlelib::{textview};
// use crate::unittest::{main};

pub const pyver: f64 = python_version ( );
pub struct AboutDialog {
    pub bg: String, // TODO: infer type
    pub fg: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub _current_textview: String, // TODO: infer type
    pub _utest: String, // TODO: infer type
    pub button_ok: String, // TODO: infer type
    pub icon_image: String, // TODO: infer type
    pub py_license: String, // TODO: infer type
    pub py_copyright: String, // TODO: infer type
    pub py_credits: String, // TODO: infer type
    pub readme: String, // TODO: infer type
    pub idle_news: String, // TODO: infer type
    pub idle_credits: String, // TODO: infer type
}

impl AboutDialog {
    pub fn new(parent: &str, title: &str, _htest: &str, _utest: &str) -> Self {
        "Create popup, do !return until tk widget destroyed.

        parent - parent of this dialog
        title - string which == title of popup dialog
        _htest - bool, change box location when running htest
        _utest - bool, don't wait_window when running unittest
        ";
        Toplevel . __init__ ( self , parent );
        self . configure ( borderwidth = 5 );
        self . geometry ( "+%d+%d" % (;
        parent . winfo_rootx ( ) + 30 ,;
        parent . winfo_rooty ( ) + ( 30 if !_htest else 100 ) ) );
        self . bg = "#bbbbbb";
        self . fg = "#000000";
        self . create_widgets ( );
        self . resizable ( height = false , width = false );
        self . title ( title or;
        format!("About IDLE {pyver} ({bits} bit)" ));
        self . transient ( parent );
        self . grab_set ( );
        self . protocol ( "WM_DELETE_WINDOW" , self . ok );
        self . parent = parent;
        self . button_ok . focus_set ( );
        self . bind ( "<Return>" , self . ok );
        self . bind ( "<Escape>" , self . ok );
        self . _current_textview = None /* Option */;
        self . _utest = _utest;
    }

}

