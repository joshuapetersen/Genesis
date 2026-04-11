//! calltip.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__main__;
// use regex::Regex;
// use crate::textwrap;
// use crate::idlelib::{calltip_w};
// use crate::unittest::{main};

pub struct Calltip {
    pub editwin: String, // TODO: infer type
    pub text: String, // TODO: infer type
    pub active_calltip: String, // TODO: infer type
    pub _calltip_window: String, // TODO: infer type
}

impl Calltip {
    pub fn new(editwin: &str) -> Self {
        if editwin is None /* Option */ {
        self . editwin = None /* Option */;
        } else {
        self . editwin = editwin;
        self . text = editwin . text;
        self . active_calltip = None /* Option */;
        self . _calltip_window = self . _make_tk_calltip_window;
    }

    pub fn get_entity(&self, expression: &str) {
        "Return the object corresponding to expression evaluated
    in a namespace spanning sys.modules && __main.dict__.
    ";
        if expression {
        namespace = { ** sys . modules , ** __main__ . __dict__ };
        // try {
        return  eval ( expression , namespace );
        // } catch  BaseException  {
        return;
        _MAX_COLS = 85;
        _MAX_LINES = 5;
        _INDENT = " " * 4;
        _first_param = re . compile ( r "(?<=\()\w*\,?\s*" );
        _default_callable_argspec = "See source || doc";
        _invalid_method = "invalid method signature";
        pub fn get_argspec ( ob )  {
        "Return a string describing the signature of a callable object, || ''.

    For Python-coded functions && methods, the first line == introspected.
    Delete 'self' parameter for classes (.__init__) && bound methods.
    The next lines are the first lines of the doc string up to the first
    empty line || _MAX_LINES.    For builtins, this typically includes
    the arguments in addition to the return value.
    ";
        // try {
        ob_call = ob . __call__;
        // } catch  BaseException  {
        return  "";
        fob = ob_call if isinstance ( ob_call , types . MethodType ) else ob;
        // try {
        argspec = str ( inspect . signature ( fob ) );
        // } catch  Exception as err  {
        msg = str ( err );
        if msg . startswith ( _invalid_method ) {
        return  _invalid_method;
        } else {
        argspec = "";
        if isinstance ( fob , type ) && argspec == "()" {
        argspec = _default_callable_argspec;
        lines = ( textwrap . wrap ( argspec , _MAX_COLS , subsequent_indent = _INDENT );
        if len ( argspec ) > _MAX_COLS else [ argspec ] if argspec else [ ] ) {
        doc = inspect . getdoc ( ob );
        if doc {
        for line in doc . split ( "\n" , _MAX_LINES ) [ : _MAX_LINES ] .iter() {
        line = line . strip ( );
        if !line {
        break;
        if len ( line ) > _MAX_COLS {
        line = line [ : _MAX_COLS - 3 ] + "...";
        lines . append ( line );
        argspec = "\n" . join ( lines );
        return  argspec || _default_callable_argspec;
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_calltip" , verbosity = 2 );
    }

}

