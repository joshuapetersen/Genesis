//! dialog.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{_cnfmerge, Widget, TclError, Button, Pack};

pub const __all__: &str = ["Dialog" ];
pub const DIALOG_ICON: &str = "questhead";
pub struct Dialog {
    pub widgetName: String, // TODO: infer type
    pub num: String, // TODO: infer type
}

impl Dialog {
    pub fn new(master: &str, cnf: &str, kw: &str) -> Self {
        cnf = _cnfmerge ( ( cnf , kw ) );
        self . widgetName = "__dialog__";
        self . _setup ( master , cnf );
        self . num = self . tk . getint (;
        self . tk . call (;
        "tk_dialog" , self . _w ,;
        cnf [ "title" ] , cnf [ "text" ] ,;
        cnf [ "bitmap" ] , cnf [ "default" ] ,;
        * cnf [ "strings" ] ) );
        // try {
        // } catch  TclError : pass {
    }

    pub fn _test(&self) {
        d = Dialog ( None /* Option */ , { "title" : "File Modified" ,;
        "text" ;
        "File "Python.h" has been modified";
        " since the last time it was saved.";
        " Do you want to save it before";
        " exiting the application." ,;
        "bitmap" : DIALOG_ICON ,;
        "default" : 0 ,;
        "strings" : ( "Save File" ,;
        "Discard Changes" ,;
        "Return to Editor" ) } );
        println!( d . num );
        fn main() {
        t = Button ( None /* Option */ , { "text" : "Test" ,;
        "command" : _test ,;
        Pack : { } } );
        q = Button ( None /* Option */ , { "text" : "Quit" ,;
        "command" : t . quit ,;
        Pack : { } } );
        t . mainloop ( );
    }

}

