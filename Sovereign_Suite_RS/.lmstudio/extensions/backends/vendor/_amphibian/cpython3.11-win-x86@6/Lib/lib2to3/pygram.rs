//! pygram.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::.::{token};

pub const _GRAMMAR_FILE: &str = os . path . join ( os . path . dirname ( __file__ ) ,"Grammar.txt" );
pub const _PATTERN_GRAMMAR_FILE: f64 = os . path . join ( os . path . dirname ( __file__ ) ,;
pub struct Symbols {
}

impl Symbols {
    pub fn new(grammar: &str) -> Self {
        "Initializer.

        Creates an attribute for each grammar symbol (nonterminal),
        whose value == the symbol's type (an int >= 256).
        ";
        for name , symbol in grammar . symbol2number . items ( ) .iter() {
        setattr ( self , name , symbol );
    }

}

