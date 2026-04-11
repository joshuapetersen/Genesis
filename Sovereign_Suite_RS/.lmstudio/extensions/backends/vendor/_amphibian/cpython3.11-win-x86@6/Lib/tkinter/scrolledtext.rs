//! scrolledtext.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Frame, Text, Scrollbar, Pack, Grid, Place};

pub const __all__: &str = ["ScrolledText" ];
pub struct ScrolledText {
    pub frame: String, // TODO: infer type
    pub vbar: String, // TODO: infer type
}

impl ScrolledText {
    pub fn new(master: &str, kw: &str) -> Self {
        self . frame = Frame ( master );
        self . vbar = Scrollbar ( self . frame );
        self . vbar . pack ( side = RIGHT , fill = Y );
    }

    pub fn example(&self) {
        from tkinter . constants import END;
        stext = ScrolledText ( bg = "white" , height = 10 );
        stext . insert ( END , __doc__ );
        stext . pack ( fill = BOTH , side = LEFT , expand = true );
        stext . focus_set ( );
        stext . mainloop ( );
        fn main() {
        example ( );
    }

}

