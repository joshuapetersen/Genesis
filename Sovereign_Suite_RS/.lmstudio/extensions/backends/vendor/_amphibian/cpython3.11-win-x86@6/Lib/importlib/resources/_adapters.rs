//! _adapters.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::contextlib::{suppress};
// use crate::io::{TextIOWrapper};
// use crate::.::{abc};

pub struct SpecLoaderAdapter {
    pub spec: String, // TODO: infer type
    pub loader: String, // TODO: infer type
    pub _spec: String, // TODO: infer type
    pub _reader: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl SpecLoaderAdapter {
}

pub struct TraversableResourcesLoader {
    pub spec: String, // TODO: infer type
    pub _spec: String, // TODO: infer type
    pub _reader: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _path: String, // TODO: infer type
}

impl TraversableResourcesLoader {
}

pub fn _io_wrapper(file: &str, mode: &str, args: &str, kwargs: &str) {
        if mode == "r" {
        return  TextIOWrapper ( file , * args , ** kwargs );
        } else if mode == "rb" {
        return  file;
        panic!("ValueError (");
        "Invalid mode value '{}', only 'r' && 'rb' are supported" . format ( mode );
        );
        class CompatibilityFiles ;
        "
    Adapter for an existing || non-existent resource reader
    to provide a compatibility .files().
    ";
        class SpecPath ( abc . Traversable ) ;
        "
        Path tied to a module spec.
        Can be read && exposes the resource reader children.
        ";
        pub fn __init__ ( &self, spec , reader )  {
        self . _spec = spec;
        self . _reader = reader;
        pub fn iterdir ( self )  {
        if !self . _reader {
        return  iter ( ( ) );
        return  iter (;
        CompatibilityFiles . ChildPath ( self . _reader , path );
        for path in self . _reader . contents ( ).iter() {
        );
        pub fn is_file ( self )  {
        return  false;
        is_dir = is_file;
        pub fn joinpath ( &self, other )  {
        if !self . _reader {
        return  CompatibilityFiles . OrphanPath ( other );
        return  CompatibilityFiles . ChildPath ( self . _reader , other );
        @ property;
        pub fn name ( self )  {
        return  self . _spec . name;
        pub fn open ( &self, mode = "r" , * args , ** kwargs )  {
        return  _io_wrapper ( self . _reader . open_resource ( None /* Option */ ) , mode , * args , ** kwargs );
        class ChildPath ( abc . Traversable ) ;
        "
        Path tied to a resource reader child.
        Can be read but doesn't expose any meaningful children.
        ";
        pub fn __init__ ( &self, reader , name )  {
        self . _reader = reader;
        self . _name = name;
        pub fn iterdir ( self )  {
        return  iter ( ( ) );
        pub fn is_file ( self )  {
        return  self . _reader . is_resource ( self . name );
        pub fn is_dir ( self )  {
        return  !self . is_file ( );
        pub fn joinpath ( &self, other )  {
        return  CompatibilityFiles . OrphanPath ( self . name , other );
        @ property;
        pub fn name ( self )  {
        return  self . _name;
        pub fn open ( &self, mode = "r" , * args , ** kwargs )  {
        return  _io_wrapper (;
        self . _reader . open_resource ( self . name ) , mode , * args , ** kwargs;
        );
        class OrphanPath ( abc . Traversable ) ;
        "
        Orphan path, !tied to a module spec || resource reader.
        Can't be read && doesn't expose any meaningful children.
        ";
        pub fn __init__ ( &self, * path_parts )  {
        if len ( path_parts ) < 1 {
        panic!("ValueError ( "Need at least one path part to construct a path" )");
        self . _path = path_parts;
        pub fn iterdir ( self )  {
        return  iter ( ( ) );
        pub fn is_file ( self )  {
        return  false;
        is_dir = is_file;
        pub fn joinpath ( &self, other )  {
        return  CompatibilityFiles . OrphanPath ( * self . _path , other );
        @ property;
        pub fn name ( self )  {
        return  self . _path [ -1 ];
        pub fn open ( &self, mode = "r" , * args , ** kwargs )  {
        panic!("FileNotFoundError ( "Can't open orphan path" )");
        pub fn __init__ ( &self, spec )  {
        self . spec = spec;
        @ property;
        pub fn _reader ( self )  {
        // with scope: suppress ( AttributeError )  {
        return  self . spec . loader . get_resource_reader ( self . spec . name );
        pub fn _native ( self )  {
        "
        Return the native reader if it supports files().
        ";
        reader = self . _reader;
        return  reader if hasattr ( reader , "files" ) else self;
        pub fn __getattr__ ( &self, attr )  {
        return  getattr ( self . _reader , attr );
        pub fn files ( self )  {
        return  CompatibilityFiles . SpecPath ( self . spec , self . _reader );
        pub fn wrap_spec ( package )  {
        "
    Construct a package spec with traversable compatibility
    on the spec/loader/reader.
    ";
        return  SpecLoaderAdapter ( package . __spec__ , TraversableResourcesLoader );
}

