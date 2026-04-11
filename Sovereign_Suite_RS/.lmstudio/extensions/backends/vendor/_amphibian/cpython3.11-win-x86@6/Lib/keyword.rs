//! keyword.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub const __all__: &str = ["iskeyword" ,"issoftkeyword" ,"kwlist" ,"softkwlist" ];
pub const kwlist: f64 = [;
pub const softkwlist: f64 = [;
pub const iskeyword: f64 = frozenset ( kwlist ) . __contains__;
pub const issoftkeyword: f64 = frozenset ( softkwlist ) . __contains__;
