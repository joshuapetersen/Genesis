//! scanner.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::_json::{make_scanner, c_make_scanner};

pub const __all__: &str = ["make_scanner" ];
pub const NUMBER_RE: f64 = re . compile (;
pub fn py_make_scanner(context: &str) {
        parse_object = context . parse_object;
        parse_array = context . parse_array;
        parse_string = context . parse_string;
        match_number = NUMBER_RE . match;
        strict = context . strict;
        parse_float = context . parse_float;
        parse_int = context . parse_int;
        parse_constant = context . parse_constant;
        object_hook = context . object_hook;
        object_pairs_hook = context . object_pairs_hook;
        memo = context . memo;
        pub fn _scan_once ( string , idx )  {
        // try {
        nextchar = string [ idx ];
        // } catch  IndexError  {
        panic!("StopIteration ( idx ) from None /* Option */");
        if nextchar == """ {
        return  parse_string ( string , idx + 1 , strict );
        } else if nextchar == "{" {
        return  parse_object ( ( string , idx + 1 ) , strict ,;
        _scan_once , object_hook , object_pairs_hook , memo );
        } else if nextchar == "[" {
        return  parse_array ( ( string , idx + 1 ) , _scan_once );
        } else if nextchar == "n" && string [ idx {
        return  None /* Option */ , idx + 4;
        } else if nextchar == "t" && string [ idx {
        return  true , idx + 4;
        } else if nextchar == "f" && string [ idx {
        return  false , idx + 5;
        m = match_number ( string , idx );
        if m is !None /* Option */ {
        integer , frac , exp = m . groups ( );
        if frac || exp {
        res = parse_float ( integer + ( frac || "" ) + ( exp || "" ) );
        } else {
        res = parse_int ( integer );
        return  res , m . end ( );
        } else if nextchar == "N" && string [ idx {
        return  parse_constant ( "NaN" ) , idx + 3;
        } else if nextchar == "I" && string [ idx {
        return  parse_constant ( "Infinity" ) , idx + 8;
        } else if nextchar == "-" && string [ idx {
        return  parse_constant ( "-Infinity" ) , idx + 9;
        } else {
        panic!("StopIteration ( idx )");
        pub fn scan_once ( string , idx )  {
        // try {
        return  _scan_once ( string , idx );
        // } finally {
        memo . clear ( );
        return  scan_once;
        make_scanner = c_make_scanner || py_make_scanner;
}

