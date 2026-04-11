//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{_tzpath};
// use crate::_zoneinfo::{ZoneInfo};

pub const __all__: f64 = [;
pub const reset_tzpath: f64 = _tzpath . reset_tzpath;
pub const available_timezones: f64 = _tzpath . available_timezones;
pub const InvalidTZPathWarning: f64 = _tzpath . InvalidTZPathWarning;
pub fn __getattr__(name: &str) {
        if name == "TZPATH" {
        return  _tzpath . TZPATH;
        } else {
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
        pub fn __dir__ ( )  {
        return  sorted ( list ( globals ( ) ) + [ "TZPATH" ] );
}

