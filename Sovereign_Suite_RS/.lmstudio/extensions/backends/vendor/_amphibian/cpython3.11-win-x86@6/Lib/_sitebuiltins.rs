//! _sitebuiltins.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::fs;
// use crate::pydoc;

pub struct Quitter {
    pub name: String, // TODO: infer type
    pub eof: String, // TODO: infer type
    pub __name: String, // TODO: infer type
    pub __data: String, // TODO: infer type
    pub __lines: String, // TODO: infer type
    pub __filenames: String, // TODO: infer type
    pub __linecnt: String, // TODO: infer type
}

impl Quitter {
    pub fn new(name: &str, eof: &str) -> Self {
        self . name = name;
        self . eof = eof;
        pub fn __repr__ ( self )  {
        return  "Use %s() || %s to exit" % ( self . name , self . eof );
        pub fn __call__ ( &self, code = None /* Option */ )  {
        // try {
        sys . stdin . close ( );
        // } catch   {
        // pass
        panic!("SystemExit ( code )");
    }

}

