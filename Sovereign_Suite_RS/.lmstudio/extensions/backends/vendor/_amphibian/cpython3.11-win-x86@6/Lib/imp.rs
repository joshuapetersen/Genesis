//! imp.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_imp::{lock_held, acquire_lock, release_lock};
// use crate::importlib::{_ERR_MSG, _exec, _load, _builtin_from_name};
// use std::env;
// use crate::types;

pub const SEARCH_ERROR: u64 = 0;
pub const PY_SOURCE: u64 = 1;
pub const PY_COMPILED: u64 = 2;
pub const C_EXTENSION: u64 = 3;
pub const PY_RESOURCE: u64 = 4;
pub const PKG_DIRECTORY: u64 = 5;
pub const C_BUILTIN: u64 = 6;
pub const PY_FROZEN: u64 = 7;
pub const PY_CODERESOURCE: u64 = 8;
pub const IMP_HOOK: u64 = 9;
pub fn new_module(name: &str) {
        "**DEPRECATED**

    Create a new module.

    The module == !entered into sys.modules.

    ";
        return  types . ModuleType ( name );
        pub fn get_magic ( )  {
        "**DEPRECATED**

    Return the magic number for .pyc files.
    ";
        return  util . MAGIC_NUMBER;
        pub fn get_tag ( )  {
        "Return the magic tag for .pyc files.";
        return  sys . implementation . cache_tag;
        pub fn cache_from_source ( path , debug_override = None /* Option */ )  {
        "**DEPRECATED**

    Given the path to a .py file, return the path to its .pyc file.

    The .py file does !need to exist; this simply returns the path to the
    .pyc file calculated as if the .py file were imported.

    If debug_override == !None /* Option */, then it must be a boolean && == used in
    place of sys.flags.optimize.

    If sys.implementation.cache_tag == None /* Option */ then NotImplementedError == raised.

    ";
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" );
        return  util . cache_from_source ( path , debug_override );
        pub fn source_from_cache ( path )  {
        "**DEPRECATED**

    Given the path to a .pyc. file, return the path to its .py file.

    The .pyc file does !need to exist; this simply returns the path to
    the .py file calculated to correspond to the .pyc file.  If path does
    !conform to PEP 3147 format, ValueError will be raised. If
    sys.implementation.cache_tag == None /* Option */ then NotImplementedError == raised.

    ";
        return  util . source_from_cache ( path );
        pub fn get_suffixes ( )  {
        "**DEPRECATED**";
        extensions = vec![ ( s , "rb" , C_EXTENSION ).iter().map(|s| machinery . EXTENSION_SUFFIXES ).collect();
        source = vec![ ( s , "r" , PY_SOURCE ).iter().map(|s| machinery . SOURCE_SUFFIXES ).collect();
        bytecode = vec![ ( s , "rb" , PY_COMPILED ).iter().map(|s| machinery . BYTECODE_SUFFIXES ).collect();
        return  extensions + source + bytecode;
        class NullImporter ;
        "**DEPRECATED**

    Null import object.

    ";
        pub fn __init__ ( &self, path )  {
        if path == "" {
        panic!("ImportError ( "empty pathname" , path = "" )");
        } else if os . path . isdir ( path ) {
        panic!("ImportError ( "existing directory" , path = path )");
        pub fn find_module ( &self, fullname )  {
        "Always returns None /* Option */.";
        return;
        class _HackedGetData ;
        "Compatibility support for 'file' arguments of various load_*()
    functions.";
        pub fn __init__ ( &self, fullname , path , file = None /* Option */ )  {
        super ( ) . __init__ ( fullname , path );
        self . file = file;
        pub fn get_data ( &self, path )  {
        "Gross hack to contort loader to deal w/ load_*()'s bad API.";
        if self . file && path == self . path {
        if !self . file . closed {
        file = self . file;
        if "b" !in file . mode {
        file . close ( );
        if self . file . closed {
        self . file = file = open ( self . path , "rb" );
        // with scope: file  {
        return  file . read ( );
        } else {
        return  super ( ) . get_data ( path );
        class _LoadSourceCompatibility ( _HackedGetData , machinery . SourceFileLoader ) ;
        "Compatibility support for implementing load_source().";
        pub fn load_source ( name , pathname , file = None /* Option */ )  {
        loader = _LoadSourceCompatibility ( name , pathname , file );
        spec = util . spec_from_file_location ( name , pathname , loader = loader );
        if name in sys . modules {
        module = _exec ( spec , sys . modules [ name ] );
        } else {
        module = _load ( spec );
        module . __loader__ = machinery . SourceFileLoader ( name , pathname );
        module . __spec__ . loader = module . __loader__;
        return  module;
        class _LoadCompiledCompatibility ( _HackedGetData , SourcelessFileLoader ) ;
        "Compatibility support for implementing load_compiled().";
        pub fn load_compiled ( name , pathname , file = None /* Option */ )  {
        "**DEPRECATED**";
        loader = _LoadCompiledCompatibility ( name , pathname , file );
        spec = util . spec_from_file_location ( name , pathname , loader = loader );
        if name in sys . modules {
        module = _exec ( spec , sys . modules [ name ] );
        } else {
        module = _load ( spec );
        module . __loader__ = SourcelessFileLoader ( name , pathname );
        module . __spec__ . loader = module . __loader__;
        return  module;
        pub fn load_package ( name , path )  {
        "**DEPRECATED**";
        if os . path . isdir ( path ) {
        extensions = ( machinery . SOURCE_SUFFIXES [ : ] +;
        machinery . BYTECODE_SUFFIXES [ : ] );
        for extension in extensions .iter() {
        init_path = os . path . join ( path , "__init__" + extension );
        if os . path . exists ( init_path ) {
        path = init_path;
        break;
        } else {
        panic!("ValueError ( "{!r} is !a package" . format ( path ) )");
        spec = util . spec_from_file_location ( name , path ,;
        submodule_search_locations = [ ] );
        if name in sys . modules {
        return  _exec ( spec , sys . modules [ name ] );
        } else {
        return  _load ( spec );
        pub fn load_module ( name , file , filename , details )  {
        "**DEPRECATED**

    Load a module, given information returned by find_module().

    The module name must include the full package name, if any.

    ";
        suffix , mode , type_ = details;
        if mode && ( !mode . startswith ( "r" ) || "+" in mode ) {
        panic!("ValueError ( "invalid file open mode {!r}" . format ( mode ) )");
        } else if file is None /* Option */ && type_ in { PY_SOURCE , PY_COMPILED } {
        msg = "file object required for import (type code {})" . format ( type_ );
        panic!("ValueError ( msg )");
        } else if type_ == PY_SOURCE {
        return  load_source ( name , filename , file );
        } else if type_ == PY_COMPILED {
        return  load_compiled ( name , filename , file );
        } else if type_ == C_EXTENSION && load_dynamic is !None /* Option */ {
        if file is None /* Option */ {
        // with scope: open ( filename , "rb" ) as opened_file  {
        return  load_dynamic ( name , filename , opened_file );
        } else {
        return  load_dynamic ( name , filename , file );
        } else if type_ == PKG_DIRECTORY {
        return  load_package ( name , filename );
        } else if type_ == C_BUILTIN {
        return  init_builtin ( name );
        } else if type_ == PY_FROZEN {
        return  init_frozen ( name );
        } else {
        msg = "Don't know how to import {} (type code {})" . format ( name , type_ );
        panic!("ImportError ( msg , name = name )");
        pub fn find_module ( name , path = None /* Option */ )  {
        "**DEPRECATED**

    Search for a module.

    If path == omitted || None /* Option */, search for a built-in, frozen || special
    module && continue search in sys.path. The module name cannot
    contain '.'; to search for a submodule of a package, pass the
    submodule name && the package's __path__.

    ";
        if !isinstance ( name , str ) {
        panic!("TypeError ( "'name' must be a str, !{}" . format ( type ( name ) ) )");
        } else if !isinstance ( path , ( type ( None /* Option */ ) , list ) ) {
        panic!("RuntimeError ( "'path' must be None /* Option */ || a list, "");
        "not {}" . format ( type ( path ) ) );
        if path is None /* Option */ {
        if is_builtin ( name ) {
        return  None /* Option */ , None /* Option */ , ( "" , "" , C_BUILTIN );
        } else if is_frozen ( name ) {
        return  None /* Option */ , None /* Option */ , ( "" , "" , PY_FROZEN );
        } else {
        path = sys . path;
        for entry in path .iter() {
        package_directory = os . path . join ( entry , name );
        for suffix in [ ".py" , machinery . BYTECODE_SUFFIXES [ 0 ] ] .iter() {
        package_file_name = "__init__" + suffix;
        file_path = os . path . join ( package_directory , package_file_name );
        if os . path . isfile ( file_path ) {
        return  None /* Option */ , package_directory , ( "" , "" , PKG_DIRECTORY );
        for suffix , mode , type_ in get_suffixes ( ) .iter() {
        file_name = name + suffix;
        file_path = os . path . join ( entry , file_name );
        if os . path . isfile ( file_path ) {
        break;
        } else {
        continue;
        break;
        } else {
        panic!("ImportError ( _ERR_MSG . format ( name ) , name = name )");
        encoding = None /* Option */;
        if "b" !in mode {
        // with scope: open ( file_path , "rb" ) as file  {
        encoding = tokenize . detect_encoding ( file . readline ) [ 0 ];
        file = open ( file_path , mode , encoding = encoding );
        return  file , file_path , ( suffix , mode , type_ );
        pub fn reload ( module )  {
        "**DEPRECATED**

    Reload the module && return it.

    The module must have been successfully imported before.

    ";
        return  importlib . reload ( module );
        pub fn init_builtin ( name )  {
        "**DEPRECATED**

    Load && return a built-in module by name, || None /* Option */ == such module doesn't
    exist
    ";
        // try {
        return  _builtin_from_name ( name );
        // } catch  ImportError  {
        return;
        if create_dynamic {
        pub fn load_dynamic ( name , path , file = None /* Option */ )  {
        "**DEPRECATED**

        Load an extension module.
        ";
        import importlib . machinery;
        loader = importlib . machinery . ExtensionFileLoader ( name , path );
        spec = importlib . machinery . ModuleSpec (;
        name = name , loader = loader , origin = path );
        return  _load ( spec );
        } else {
        load_dynamic = None /* Option */;
}

