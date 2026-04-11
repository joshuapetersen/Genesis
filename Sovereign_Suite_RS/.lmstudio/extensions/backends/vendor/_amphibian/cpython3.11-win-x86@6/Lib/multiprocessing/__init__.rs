//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::context;

pub const __all__: &str = [ x for x in dir ( context . _default_context ) if not x . startswith ("_" ) ];
pub const SUBDEBUG: u64 = 5;
pub const SUBWARNING: u64 = 25;
