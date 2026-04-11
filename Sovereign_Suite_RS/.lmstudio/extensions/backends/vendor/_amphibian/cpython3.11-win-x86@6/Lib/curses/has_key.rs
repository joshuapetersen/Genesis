//! has_key.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_curses;

pub const _capability_names: f64 = {;
pub fn has_key(ch: &str) {
        if isinstance ( ch , str ) {
        ch = ord ( ch );
        capability_name = _capability_names . get ( ch );
        if capability_name is None /* Option */ {
        return  false;
        if _curses . tigetstr ( capability_name ) {
        return  true;
        } else {
        return  false;
        fn main() {
        // try {
        L = [ ];
        _curses . initscr ( );
        for key in _capability_names . keys ( ) .iter() {
        system = _curses . has_key ( key );
        python = has_key ( key );
        if system != python {
        L . append ( "Mismatch for key %s, system=%i, Python=%i";
        % ( _curses . keyname ( key ) , system , python ) );
        // } finally {
        _curses . endwin ( );
        for i in L : print ( i ).iter() {
}

