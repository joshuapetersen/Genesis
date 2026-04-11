//! colorchooser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Dialog};

pub const __all__: &str = ["Chooser" ,"askcolor" ];
pub struct Chooser {
}

impl Chooser {
}

pub fn askcolor(color: &str, options: &str) {
        "Display dialog window for selection of a color.

    Convenience wrapper for the Chooser class.  Displays the color
    chooser dialog with color as the initial value.
    ";
        if color {
        options = options . copy ( );
        options [ "initialcolor" ] = color;
        return  Chooser ( ** options ) . show ( );
        fn main() {
        println!( "color" , askcolor ( ) );
}

