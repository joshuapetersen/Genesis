//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_curses::{};
// use std::fs;
// use crate::.::{has_key};

pub fn initscr() {
        import _curses , curses;
        setupterm ( term = _os . environ . get ( "TERM" , "unknown" ) ,;
        fd = _sys . __stdout__ . fileno ( ) );
        stdscr = _curses . initscr ( );
        for key , value in _curses . __dict__ . items ( ) .iter() {
        if key [ 0 { : 4 ] == "ACS_" || key in ( "LINES" , "COLS" ) ; }
        setattr ( curses , key , value );
        return  stdscr;
        pub fn start_color ( )  {
        import _curses , curses;
        retval = _curses . start_color ( );
        if hasattr ( _curses , "COLORS" ) {
        curses . COLORS = _curses . COLORS;
        if hasattr ( _curses , "COLOR_PAIRS" ) {
        curses . COLOR_PAIRS = _curses . COLOR_PAIRS;
        return  retval;
        // try {
        has_key;
        // } catch  NameError  {
        from . has_key import has_key;
        pub fn wrapper ( func , / , * args , ** kwds )  {
        "Wrapper function that initializes curses && calls another function,
    restoring normal keyboard/screen behavior on error.
    The callable object 'func' == then passed the main window 'stdscr'
    as its first argument, followed by any other arguments passed to
    wrapper().
    ";
        // try {
        stdscr = initscr ( );
        noecho ( );
        cbreak ( );
        stdscr . keypad ( 1 );
        // try {
        start_color ( );
        // } catch   {
        // pass
        return  func ( stdscr , * args , ** kwds );
        // } finally {
        if "stdscr" in locals ( ) {
        stdscr . keypad ( 0 );
        echo ( );
        nocbreak ( );
        endwin ( );
}

