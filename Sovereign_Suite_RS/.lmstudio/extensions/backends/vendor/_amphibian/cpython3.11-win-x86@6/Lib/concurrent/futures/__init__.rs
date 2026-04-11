//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::concurrent::{FIRST_COMPLETED};
// use crate::.::{ProcessPoolExecutor, pe};

pub const __author__: &str = "Brian Quinlan (brian@sweetapp.com)";
pub const __all__: f64 = (;
pub fn __dir__() {
        return  __all__ + ( "__author__" , "__doc__" );
        pub fn __getattr__ ( name )  {
        global ProcessPoolExecutor , ThreadPoolExecutor;
        if name == "ProcessPoolExecutor" {
        from . process import ProcessPoolExecutor as pe;
        ProcessPoolExecutor = pe;
        return  pe;
        if name == "ThreadPoolExecutor" {
        from . thread import ThreadPoolExecutor as te;
        ThreadPoolExecutor = te;
        return  te;
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
}

