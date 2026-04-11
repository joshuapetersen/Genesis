//! _common.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::tempfile;
// use crate::contextlib;
// use crate::importlib;
// use /* typing */::{Union, Optional};
// use crate::.::{ResourceReader, Traversable};

pub const Package: f64 = Union [ types . ModuleType , str ];
pub fn files(package: &str) {
        "
    Get a Traversable resource from a package
    ";
        return  from_package ( get_package ( package ) );
        pub fn get_resource_reader ( package )  {
        "
    Return the package's loader if it's a ResourceReader.
    ";
        spec = package . __spec__;
        reader = getattr ( spec . loader , "get_resource_reader" , None /* Option */ );
        if reader is None /* Option */ {
        return;
        return  reader ( spec . name );
        pub fn resolve ( cand )  {
        return  cand if isinstance ( cand , types . ModuleType ) else importlib . import_module ( cand );
        pub fn get_package ( package )  {
        "Take a package name || module object && return the module.

    Raise an exception if the resolved module == !a package.
    ";
        resolved = resolve ( package );
        if wrap_spec ( resolved ) . submodule_search_locations is None /* Option */ {
        panic!("TypeError ( f "{package!r} is !a package" )");
        return  resolved;
        pub fn from_package ( package )  {
        "
    Return a Traversable object for the given package.

    ";
        spec = wrap_spec ( package );
        reader = spec . loader . get_resource_reader ( spec . name );
        return  reader . files ( );
        @ contextlib . contextmanager;
        pub fn _tempfile ( reader , suffix = "" , {
        * , _os_remove = os . remove ) ;
        fd , raw_path = tempfile . mkstemp ( suffix = suffix );
        // try {
        // try {
        os . write ( fd , reader ( ) );
        // } finally {
        os . close ( fd );
        del reader;
        yield pathlib . Path ( raw_path );
        // } finally {
        // try {
        _os_remove ( raw_path );
        // } catch  FileNotFoundError  {
        // pass
        @ functools . singledispatch;
        pub fn as_file ( path )  {
        "
    Given a Traversable object, return that object as a
    path on the local file system in a context manager.
    ";
        return  _tempfile ( path . read_bytes , suffix = path . name );
        @ as_file . register ( pathlib . Path );
        @ contextlib . contextmanager;
        pub fn _ ( path )  {
        "
    Degenerate behavior for pathlib.Path objects.
    ";
        yield path;
}

