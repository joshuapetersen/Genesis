//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{TestResult};
// use std::fs;

pub const __all__: &str = ["TestResult" ,"TestCase" ,"IsolatedAsyncioTestCase" ,"TestSuite" ,;
pub const __unittest: f64 = True;
pub const _TextTestResult: f64 = TextTestResult;
pub fn load_tests(loader: &str, tests: &str, pattern: &str) {
        import os . path;
        this_dir = os . path . dirname ( __file__ );
        return  loader . discover ( start_dir = this_dir , pattern = pattern );
        pub fn __dir__ ( )  {
        return  globals ( ) . keys ( ) | { "IsolatedAsyncioTestCase" };
        pub fn __getattr__ ( name )  {
        if name == "IsolatedAsyncioTestCase" {
        global IsolatedAsyncioTestCase;
        from . async_case import IsolatedAsyncioTestCase;
        return  IsolatedAsyncioTestCase;
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
}

