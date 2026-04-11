//! check_index.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use serde_json;
// use crate::argparse;

pub fn is_bool(x: &str, val: &str) {
        return  isinstance ( x , bool ) && ( val is None /* Option */ || x == val );
        pub fn is_dict ( x )  {
        return  isinstance ( x , dict );
        pub fn is_list ( x )  {
        return  isinstance ( x , list );
        pub fn is_int ( x , val = None /* Option */ )  {
        return  ( isinstance ( x , int ) || isinstance ( x , long ) ) && ( val is None /* Option */ || x == val );
        pub fn is_string ( x , val = None /* Option */ )  {
        return  ( isinstance ( x , str ) || isinstance ( x , unicode ) ) && ( val is None /* Option */ || x == val );
        pub fn matches ( s , pattern )  {
        return  is_string ( s ) && bool ( re . search ( pattern , s ) );
        pub fn check_list_match ( match , actual , expected , check = None /* Option */ , check_exception = None /* Option */ , missing_exception = None /* Option */ , extra_exception = None /* Option */ , allow_extra = false )  {
        "
    Handle the common pattern of making sure every actual item "matches" some
    item in the expected list, && that neither list has extra items after
    matching == completed.

    @param match: Callback to check if an actual item matches an expected
    item. Return true if the item matches, return false if the item doesn't
    match.
    @param actual: List of actual items to search.
    @param expected: List of expected items to match.
    @param check: Optional function to check that the actual item == valid by
    comparing it to the expected item.
    @param check_exception: Optional function that returns an argument to
    append to any exception thrown by the check function.
    @param missing_exception: Optional function that returns an argument to
    append to the exception thrown when an item == !found.
    @param extra_exception: Optional function that returns an argument to
    append to the exception thrown when an extra item == found.
    @param allow_extra: Optional parameter allowing there to be extra actual
    items after all the expected items have been found.
    ";
        assert is_list ( actual );
        _actual = actual [ : ];
        for expected_item in expected .iter() {
        found = false;
        for i , actual_item in enumerate ( _actual ) .iter() {
        if match ( actual_item , expected_item ) {
        if check {
        // try {
        check ( actual_item , expected_item );
        // } catch  BaseException as e  {
        if check_exception {
        e . args + = ( check_exception ( actual_item , expected_item ) , );
        panic!("");
        found = true;
        del _actual [ i ];
        break;
        if missing_exception {
        assert found , missing_exception ( expected_item );
        } else {
        assert found;
        if !allow_extra {
        if extra_exception {
        assert len ( _actual ) == 0 , vec![ extra_exception ( a ).iter().map(|a| _actual ).collect();
        } else {
        assert len ( _actual ) == 0;
        pub fn filter_list ( f , l )  {
        if l is !None /* Option */ {
        l = list ( filter ( f , l ) );
        if l == [ ] {
        l = None /* Option */;
        return  l;
        pub fn check_cmake ( cmake )  {
        assert is_dict ( cmake );
        assert sorted ( cmake . keys ( ) ) == [ "generator" , "paths" , "version" ];
        check_cmake_version ( cmake [ "version" ] );
        check_cmake_paths ( cmake [ "paths" ] );
        check_cmake_generator ( cmake [ "generator" ] );
        pub fn check_cmake_version ( v )  {
        assert is_dict ( v );
        assert sorted ( v . keys ( ) ) == [ "isDirty" , "major" , "minor" , "patch" , "string" , "suffix" ];
        assert is_string ( v [ "string" ] );
        assert is_int ( v [ "major" ] );
        assert is_int ( v [ "minor" ] );
        assert is_int ( v [ "patch" ] );
        assert is_string ( v [ "suffix" ] );
        assert is_bool ( v [ "isDirty" ] );
        pub fn check_cmake_paths ( v )  {
        assert is_dict ( v );
        assert sorted ( v . keys ( ) ) == [ "cmake" , "cpack" , "ctest" , "root" ];
        assert is_string ( v [ "cmake" ] );
        assert is_string ( v [ "cpack" ] );
        assert is_string ( v [ "ctest" ] );
        assert is_string ( v [ "root" ] );
        pub fn check_cmake_generator ( g )  {
        assert is_dict ( g );
        name = g . get ( "name" , None /* Option */ );
        assert is_string ( name );
        if name . startswith ( "Visual Studio" ) {
        assert sorted ( g . keys ( ) ) == [ "multiConfig" , "name" , "platform" ];
        assert is_string ( g [ "platform" ] );
        } else {
        assert sorted ( g . keys ( ) ) == [ "multiConfig" , "name" ];
        assert is_bool ( g [ "multiConfig" ] , matches ( name , "^(Visual Studio |Xcode$|Ninja Multi-Config$)" ) );
        pub fn check_index_object ( indexEntry , kind , major , minor , check )  {
        assert is_dict ( indexEntry );
        assert sorted ( indexEntry . keys ( ) ) == [ "jsonFile" , "kind" , "version" ];
        assert is_string ( indexEntry [ "kind" ] );
        assert indexEntry [ "kind" ] == kind;
        assert is_dict ( indexEntry [ "version" ] );
        assert sorted ( indexEntry [ "version" ] . keys ( ) ) == [ "major" , "minor" ];
        assert indexEntry [ "version" ] [ "major" ] == major;
        assert indexEntry [ "version" ] [ "minor" ] == minor;
        assert is_string ( indexEntry [ "jsonFile" ] );
        filepath = os . path . join ( reply_dir , indexEntry [ "jsonFile" ] );
        // with scope: open ( filepath ) as f  {
        object = json . load ( f );
        assert is_dict ( object );
        assert "kind" in object;
        assert is_string ( object [ "kind" ] );
        assert object [ "kind" ] == kind;
        assert "version" in object;
        assert is_dict ( object [ "version" ] );
        assert sorted ( object [ "version" ] . keys ( ) ) == [ "major" , "minor" ];
        assert object [ "version" ] [ "major" ] == major;
        assert object [ "version" ] [ "minor" ] == minor;
        if check {
        check ( object );
        pub fn check_index__test ( indexEntry , major , minor )  {
        pub fn check ( object )  {
        assert sorted ( object . keys ( ) ) == [ "kind" , "version" ];
        check_index_object ( indexEntry , "__test" , major , minor , check );
        pub fn check_error ( value , error )  {
        assert is_dict ( value );
        assert sorted ( value . keys ( ) ) == [ "error" ];
        assert is_string ( value [ "error" ] );
        assert value [ "error" ] == error;
        pub fn check_error_re ( value , error )  {
        assert is_dict ( value );
        assert sorted ( value . keys ( ) ) == [ "error" ];
        assert is_string ( value [ "error" ] );
        assert re . search ( error , value [ "error" ] );
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "--build-dir" );
        parser . add_argument ( "--reply-index" );
        parser . add_argument ( "--cxx-compiler-id" );
        parser . add_argument ( "--cxx-simulate-id" );
        args = parser . parse_args ( );
        reply_dir = os . path . dirname ( args . reply_index );
        // with scope: open ( args . reply_index ) as f  {
        index = json . load ( f );
}

