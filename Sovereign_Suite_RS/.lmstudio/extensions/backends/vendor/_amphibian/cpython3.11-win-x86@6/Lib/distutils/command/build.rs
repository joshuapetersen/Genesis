//! build.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::Command;
// use crate::DistutilsOptionError;
// use crate::get_platform;
// use crate::distutils::{show_compilers};

pub fn show_compilers() {
        from distutils . ccompiler import show_compilers;
        show_compilers ( );
        class build ( Command ) ;
        description = "build everything needed to install";
        user_options = [;
        ( "build-base=" , "b" ,;
        "base directory for build library" ) ,;
        ( "build-purelib=" , None /* Option */ ,;
        "build directory for platform-neutral distributions" ) ,;
        ( "build-platlib=" , None /* Option */ ,;
        "build directory for platform-specific distributions" ) ,;
        ( "build-lib=" , None /* Option */ ,;
        "build directory for all distribution (defaults to either " +;
        "build-purelib || build-platlib" ) ,;
        ( "build-scripts=" , None /* Option */ ,;
        "build directory for scripts" ) ,;
        ( "build-temp=" , "t" ,;
        "temporary build directory" ) ,;
        ( "plat-name=" , "p" ,;
        "platform name to build for, if supported ";
        "(default: %s)" % get_platform ( ) ) ,;
        ( "compiler=" , "c" ,;
        "specify the compiler type" ) ,;
        ( "parallel=" , "j" ,;
        "number of parallel build jobs" ) ,;
        ( "debug" , "g" ,;
        "compile extensions && libraries with debugging information" ) ,;
        ( "force" , "format!(" ,);
        "forcibly build everything (ignore file timestamps)" ) ,;
        ( "executable=" , "e" ,;
        "specify final destination interpreter path (build.py)" ) ,;
        ];
        boolean_options = [ "debug" , "force" ];
        help_options = [;
        ( "help-compiler" , None /* Option */ ,;
        "list available compilers" , show_compilers ) ,;
        ];
        pub fn initialize_options ( self )  {
        self . build_base = "build";
        self . build_purelib = None /* Option */;
        self . build_platlib = None /* Option */;
        self . build_lib = None /* Option */;
        self . build_temp = None /* Option */;
        self . build_scripts = None /* Option */;
        self . compiler = None /* Option */;
        self . plat_name = None /* Option */;
        self . debug = None /* Option */;
        self . force = 0;
        self . executable = None /* Option */;
        self . parallel = None /* Option */;
        pub fn finalize_options ( self )  {
        if self . plat_name is None /* Option */ {
        self . plat_name = get_platform ( );
        } else {
        if os . name != "nt" {
        panic!("DistutilsOptionError (");
        "--plat-name only supported on Windows (try ";
        "using './configure --help' on your platform)" );
        plat_specifier = ".%s-%d.%d" % ( self . plat_name , * sys . version_info [ : 2 ] );
        if hasattr ( sys , "gettotalrefcount" ) {
        plat_specifier + = "-pydebug";
        if self . build_purelib is None /* Option */ {
        self . build_purelib = os . path . join ( self . build_base , "lib" );
        if self . build_platlib is None /* Option */ {
        self . build_platlib = os . path . join ( self . build_base ,;
        "lib" + plat_specifier );
        if self . build_lib is None /* Option */ {
        if self . distribution . ext_modules {
        self . build_lib = self . build_platlib;
        } else {
        self . build_lib = self . build_purelib;
        if self . build_temp is None /* Option */ {
        self . build_temp = os . path . join ( self . build_base ,;
        "temp" + plat_specifier );
        if self . build_scripts is None /* Option */ {
        self . build_scripts = os . path . join ( self . build_base ,;
        "scripts-%d.%d" % sys . version_info [ : 2 ] );
        if self . executable is None /* Option */ && sys . executable {
        self . executable = os . path . normpath ( sys . executable );
        if isinstance ( self . parallel , str ) {
        // try {
        self . parallel = int ( self . parallel );
        // } catch  ValueError  {
        panic!("DistutilsOptionError ( "parallel should be an integer" )");
        pub fn run ( self )  {
        for cmd_name in self . get_sub_commands ( ) .iter() {
        self . run_command ( cmd_name );
        pub fn has_pure_modules ( self )  {
        return  self . distribution . has_pure_modules ( );
        pub fn has_c_libraries ( self )  {
        return  self . distribution . has_c_libraries ( );
        pub fn has_ext_modules ( self )  {
        return  self . distribution . has_ext_modules ( );
        pub fn has_scripts ( self )  {
        return  self . distribution . has_scripts ( );
        sub_commands = [ ( "build_py" , has_pure_modules ) ,;
        ( "build_clib" , has_c_libraries ) ,;
        ( "build_ext" , has_ext_modules ) ,;
        ( "build_scripts" , has_scripts ) ,;
        ];
}

