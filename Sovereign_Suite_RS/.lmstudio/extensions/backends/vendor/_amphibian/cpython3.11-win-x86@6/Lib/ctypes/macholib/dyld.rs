//! dyld.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::framework_info;
// use crate::dylib_info;
// use crate::_ctypes::{_dyld_shared_cache_contains_path};

pub const __all__: f64 = [;
pub const DEFAULT_FRAMEWORK_FALLBACK: f64 = [;
pub const DEFAULT_LIBRARY_FALLBACK: f64 = [;
pub fn dyld_env(env: &str, var: &str) {
        if env is None /* Option */ {
        env = os . environ;
        rval = env . get ( var );
        if rval is None /* Option */ {
        return  [ ];
        return  rval . split ( ":" );
        pub fn dyld_image_suffix ( env = None /* Option */ )  {
        if env is None /* Option */ {
        env = os . environ;
        return  env . get ( "DYLD_IMAGE_SUFFIX" );
        pub fn dyld_framework_path ( env = None /* Option */ )  {
        return  dyld_env ( env , "DYLD_FRAMEWORK_PATH" );
        pub fn dyld_library_path ( env = None /* Option */ )  {
        return  dyld_env ( env , "DYLD_LIBRARY_PATH" );
        pub fn dyld_fallback_framework_path ( env = None /* Option */ )  {
        return  dyld_env ( env , "DYLD_FALLBACK_FRAMEWORK_PATH" );
        pub fn dyld_fallback_library_path ( env = None /* Option */ )  {
        return  dyld_env ( env , "DYLD_FALLBACK_LIBRARY_PATH" );
        pub fn dyld_image_suffix_search ( iterator , env = None /* Option */ )  {
        "For a potential path iterator, add DYLD_IMAGE_SUFFIX semantics";
        suffix = dyld_image_suffix ( env );
        if suffix is None /* Option */ {
        return  iterator;
        pub fn _inject ( iterator = iterator , suffix = suffix )  {
        for path in iterator .iter() {
        if path . endswith ( ".dylib" ) {
        yield path [ : - len ( ".dylib" ) ] + suffix + ".dylib";
        } else {
        yield path + suffix;
        yield path;
        return  _inject ( );
        pub fn dyld_override_search ( name , env = None /* Option */ )  {
        framework = framework_info ( name );
        if framework is !None /* Option */ {
        for path in dyld_framework_path ( env ) .iter() {
        yield os . path . join ( path , framework [ "name" ] );
        for path in dyld_library_path ( env ) .iter() {
        yield os . path . join ( path , os . path . basename ( name ) );
        pub fn dyld_executable_path_search ( name , executable_path = None /* Option */ )  {
        if name . startswith ( "@executable_path/" ) && executable_path is !None /* Option */ {
        yield os . path . join ( executable_path , name [ len ( "@executable_path/" ) : ] );
        pub fn dyld_default_search ( name , env = None /* Option */ )  {
        yield name;
        framework = framework_info ( name );
        if framework is !None /* Option */ {
        fallback_framework_path = dyld_fallback_framework_path ( env );
        for path in fallback_framework_path .iter() {
        yield os . path . join ( path , framework [ "name" ] );
        fallback_library_path = dyld_fallback_library_path ( env );
        for path in fallback_library_path .iter() {
        yield os . path . join ( path , os . path . basename ( name ) );
        if framework is !None /* Option */ && !fallback_framework_path {
        for path in DEFAULT_FRAMEWORK_FALLBACK .iter() {
        yield os . path . join ( path , framework [ "name" ] );
        if !fallback_library_path {
        for path in DEFAULT_LIBRARY_FALLBACK .iter() {
        yield os . path . join ( path , os . path . basename ( name ) );
        pub fn dyld_find ( name , executable_path = None /* Option */ , env = None /* Option */ )  {
        "
    Find a library || framework using dyld semantics
    ";
        for path in dyld_image_suffix_search ( chain (.iter() {
        dyld_override_search ( name , env ) ,;
        dyld_executable_path_search ( name , executable_path ) ,;
        dyld_default_search ( name , env ) ,;
        ) , env ) ;
        if os . path . isfile ( path ) {
        return  path;
        // try {
        if _dyld_shared_cache_contains_path ( path ) {
        return  path;
        // } catch  NotImplementedError  {
        // pass
        panic!("ValueError ( "dylib %s could !be found" % ( name , ) )");
        pub fn framework_find ( fn , executable_path = None /* Option */ , env = None /* Option */ )  {
        "
    Find a framework using dyld semantics in a very loose manner.

    Will take input such as:
        Python
        Python.framework
        Python.framework/Versions/Current
    ";
        error = None /* Option */;
        // try {
        return  dyld_find ( fn , executable_path = executable_path , env = env );
        // } catch  ValueError as e  {
        error = e;
        fmwk_index = fn . rfind ( ".framework" );
        if fmwk_index == -1 {
        fmwk_index = len ( fn );
        fn + = ".framework";
        fn = os . path . join ( fn , os . path . basename ( fn [ : fmwk_index ] ) );
        // try {
        return  dyld_find ( fn , executable_path = executable_path , env = env );
        // } catch  ValueError  {
        panic!("error");
        // } finally {
        error = None /* Option */;
}

