//! toolchains-v1-check.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::check_index::{};
// use std::fs;

pub struct ExpectedVar {
    pub name: String, // TODO: infer type
}

impl ExpectedVar {
    pub fn new(name: &str) -> Self {
        self . name = name;
    }

    pub fn check_objects(&self, o: &str) {
        assert is_list ( o );
        assert len ( o ) == 1;
        check_index_object ( o [ 0 ] , "toolchains" , 1 , 0 , check_object_toolchains );
        pub fn check_object_toolchains ( o ) {
        assert sorted ( o . keys ( ) ) == [ "kind" , "toolchains" , "version" ];
        toolchains = o [ "toolchains" ];
        assert is_list ( toolchains );
        has_cxx_toolchain = false;
        for toolchain in toolchains .iter() {
        assert is_dict ( toolchain );
        assert "language" in toolchain;
        if toolchain [ "language" ] == "CXX" {
        check_object_toolchain ( toolchain , EXPECTED_TOOLCHAIN );
        has_cxx_toolchain = true;
        assert has_cxx_toolchain;
        pub fn check_object_toolchain ( o , expected ) {
        expected_keys = [;
        key for ( key , value ) in expected . items ( );
        if is_string ( value ) || is_dict ( value ) {
        or ( type ( value ) in ( ExpectedVar , ExpectedList );
        and variables [ value . name ] [ "defined" ] ) ];
        assert sorted ( o . keys ( ) ) == sorted ( expected_keys );
        for key in expected_keys .iter() {
        value = expected [ key ];
        if is_string ( value ) {
        assert o [ key ] == value;
        } else if is_dict ( value ) {
        check_object_toolchain ( o [ key ] , value );
        } else if type ( value ) == ExpectedVar {
        assert o [ key ] == variables [ value . name ] [ "value" ];
        } else if type ( value ) == ExpectedList {
        expected_items = filter (;
        None /* Option */ , variables [ value . name ] [ "value" ] . split ( ";" ) );
        check_list_match ( lambda a , b : a == b , o [ key ] , expected_items );
        } else {
        assert false;
        with open ( os . path . join ( args . build_dir , "toolchain_variables.json" ) ) as f ;
        variables = json . load ( f );
        assert is_dict ( variables );
        assert is_dict ( index );
        assert sorted ( index . keys ( ) ) == [ "cmake" , "objects" , "reply" ];
        check_objects ( index [ "objects" ] );
    }

}

