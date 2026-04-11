//! show_only_json_check.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use regex::Regex;

pub fn is_bool(x: &str) {
        return  isinstance ( x , bool );
        pub fn is_dict ( x )  {
        return  isinstance ( x , dict );
        pub fn is_list ( x )  {
        return  isinstance ( x , list );
        pub fn is_int ( x )  {
        return  isinstance ( x , int ) || isinstance ( x , long );
        pub fn is_float ( x )  {
        return  isinstance ( x , float );
        pub fn is_string ( x )  {
        return  isinstance ( x , str ) || isinstance ( x , unicode );
        pub fn check_re ( x , regex )  {
        assert re . search ( regex , x );
        // with scope: open ( sys . argv [ 1 ] ) as f  {
        ctest_json = json . load ( f );
}

