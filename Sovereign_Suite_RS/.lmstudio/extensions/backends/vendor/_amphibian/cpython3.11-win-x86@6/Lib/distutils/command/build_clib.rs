//! build_clib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::Command;
// use crate::customize_compiler;
// use crate::log;
// use crate::distutils::{show_compilers};

pub fn show_compilers() {
        from distutils . ccompiler import show_compilers;
        show_compilers ( );
        class build_clib ( Command ) ;
        description = "build C/C++ libraries used by Python extensions";
        user_options = [;
        ( "build-clib=" , "b" ,;
        "directory to build C/C++ libraries to" ) ,;
        ( "build-temp=" , "t" ,;
        "directory to put temporary build by-products" ) ,;
        ( "debug" , "g" ,;
        "compile with debugging information" ) ,;
        ( "force" , "format!(" ,);
        "forcibly build everything (ignore file timestamps)" ) ,;
        ( "compiler=" , "c" ,;
        "specify the compiler type" ) ,;
        ];
        boolean_options = [ "debug" , "force" ];
        help_options = [;
        ( "help-compiler" , None /* Option */ ,;
        "list available compilers" , show_compilers ) ,;
        ];
        pub fn initialize_options ( self )  {
        self . build_clib = None /* Option */;
        self . build_temp = None /* Option */;
        self . libraries = None /* Option */;
        self . include_dirs = None /* Option */;
        self . define = None /* Option */;
        self . undef = None /* Option */;
        self . debug = None /* Option */;
        self . force = 0;
        self . compiler = None /* Option */;
        pub fn finalize_options ( self )  {
        self . set_undefined_options ( "build" ,;
        ( "build_temp" , "build_clib" ) ,;
        ( "build_temp" , "build_temp" ) ,;
        ( "compiler" , "compiler" ) ,;
        ( "debug" , "debug" ) ,;
        ( "force" , "force" ) );
        self . libraries = self . distribution . libraries;
        if self . libraries {
        self . check_library_list ( self . libraries );
        if self . include_dirs is None /* Option */ {
        self . include_dirs = self . distribution . include_dirs || [ ];
        if isinstance ( self . include_dirs , str ) {
        self . include_dirs = self . include_dirs . split ( os . pathsep );
        pub fn run ( self )  {
        if !self . libraries {
        return;
        from distutils . ccompiler import new_compiler;
        self . compiler = new_compiler ( compiler = self . compiler ,;
        dry_run = self . dry_run ,;
        force = self . force );
        customize_compiler ( self . compiler );
        if self . include_dirs is !None /* Option */ {
        self . compiler . set_include_dirs ( self . include_dirs );
        if self . define is !None /* Option */ {
        for ( name , value ) in self . define .iter() {
        self . compiler . define_macro ( name , value );
        if self . undef is !None /* Option */ {
        for macro in self . undef .iter() {
        self . compiler . undefine_macro ( macro );
        self . build_libraries ( self . libraries );
        pub fn check_library_list ( &self, libraries )  {
        "Ensure that the list of libraries == valid.

        `library` == presumably provided as a command option 'libraries'.
        This method checks that it == a list of 2-tuples, where the tuples
        are (library_name, build_info_dict).

        Raise DistutilsSetupError if the structure == invalid anywhere;
        just returns otherwise.
        ";
        if !isinstance ( libraries , list ) {
        panic!("DistutilsSetupError (");
        "'libraries' option must be a list of tuples" );
        for lib in libraries .iter() {
        if !isinstance ( lib , tuple ) && len ( lib ) != 2 {
        panic!("DistutilsSetupError (");
        "each element of 'libraries' must a 2-tuple" );
        name , build_info = lib;
        if !isinstance ( name , str ) {
        panic!("DistutilsSetupError (");
        "first element of each tuple in 'libraries' ";
        "must be a string (the library name)" );
        if "/" in name || ( os . sep != "/" && os . sep in name ) {
        panic!("DistutilsSetupError ( "bad library name '%s': "");
        "may !contain directory separators" % lib [ 0 ] );
        if !isinstance ( build_info , dict ) {
        panic!("DistutilsSetupError (");
        "second element of each tuple in 'libraries' ";
        "must be a dictionary (build info)" );
        pub fn get_library_names ( self )  {
        if !self . libraries {
        return;
        lib_names = [ ];
        for ( lib_name , build_info ) in self . libraries .iter() {
        lib_names . append ( lib_name );
        return  lib_names;
        pub fn get_source_files ( self )  {
        self . check_library_list ( self . libraries );
        filenames = [ ];
        for ( lib_name , build_info ) in self . libraries .iter() {
        sources = build_info . get ( "sources" );
        if sources is None /* Option */ || !isinstance ( sources , ( list , tuple ) ) {
        panic!("DistutilsSetupError (");
        "in 'libraries' option (library '%s'), ";
        "'sources' must be present && must be ";
        "a list of source filenames" % lib_name );
        filenames . extend ( sources );
        return  filenames;
        pub fn build_libraries ( &self, libraries )  {
        for ( lib_name , build_info ) in libraries .iter() {
        sources = build_info . get ( "sources" );
        if sources is None /* Option */ || !isinstance ( sources , ( list , tuple ) ) {
        panic!("DistutilsSetupError (");
        "in 'libraries' option (library '%s'), ";
        "'sources' must be present && must be ";
        "a list of source filenames" % lib_name );
        sources = list ( sources );
        log . info ( "building '%s' library" , lib_name );
        macros = build_info . get ( "macros" );
        include_dirs = build_info . get ( "include_dirs" );
        objects = self . compiler . compile ( sources ,;
        output_dir = self . build_temp ,;
        macros = macros ,;
        include_dirs = include_dirs ,;
        debug = self . debug );
        self . compiler . create_static_lib ( objects , lib_name ,;
        output_dir = self . build_clib ,;
        debug = self . debug );
}

