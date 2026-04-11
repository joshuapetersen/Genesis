//! cygwinccompiler.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::copy;
// use crate::Popen;
// use crate::distutils::{UnixCCompiler};

pub fn get_msvcr() {
        "Include the appropriate MSVC runtime library if Python was built
    with MSVC 7.0 || later.
    ";
        msc_pos = sys . version . find ( "MSC v." );
        if msc_pos != -1 {
        msc_ver = sys . version [ msc_pos + 6 : msc_pos + 10 ];
        if msc_ver == "1300" {
        return  [ "msvcr70" ];
        } else if msc_ver == "1310" {
        return  [ "msvcr71" ];
        } else if msc_ver == "1400" {
        return  [ "msvcr80" ];
        } else if msc_ver == "1500" {
        return  [ "msvcr90" ];
        } else if msc_ver == "1600" {
        return  [ "msvcr100" ];
        } else {
        panic!("ValueError ( "Unknown MS Compiler version %s " % msc_ver )");
        class CygwinCCompiler ( UnixCCompiler ) ;
        " Handles the Cygwin port of the GNU C compiler to Windows.
    ";
        compiler_type = "cygwin";
        obj_extension = ".o";
        static_lib_extension = ".a";
        shared_lib_extension = ".dll";
        static_lib_format = "lib%s%s";
        shared_lib_format = "%s%s";
        exe_extension = ".exe";
        pub fn __init__ ( &self, verbose = 0 , dry_run = 0 , force = 0 )  {
        UnixCCompiler . __init__ ( self , verbose , dry_run , force );
        status , details = check_config_h ( );
        self . debug_print ( "Python's GCC status: %s (details: %s)" %;
        ( status , details ) );
        if status is !CONFIG_H_OK {
        self . warn (;
        "Python's pyconfig.h doesn't seem to support your compiler. ";
        "Reason: %s. ";
        "Compiling may fail because of undefined preprocessor macros.";
        % details );
        self . gcc_version , self . ld_version , self . dllwrap_version = \;
        get_versions ( );
        self . debug_print ( self . compiler_type + ": gcc %s, ld %s, dllwrap %s\n" %;
        ( self . gcc_version ,;
        self . ld_version ,;
        self . dllwrap_version ) );
        if self . ld_version >= "2.10.90" {
        self . linker_dll = "gcc";
        } else {
        self . linker_dll = "dllwrap";
        if self . ld_version >= "2.13" {
        shared_option = "-shared";
        } else {
        shared_option = "-mdll -static";
        self . set_executables ( compiler = "gcc -mcygwin -O -Wall" ,;
        compiler_so = "gcc -mcygwin -mdll -O -Wall" ,;
        compiler_cxx = "g++ -mcygwin -O -Wall" ,;
        linker_exe = "gcc -mcygwin" ,;
        linker_so = ( "%s -mcygwin %s" %;
        ( self . linker_dll , shared_option ) ) );
        if self . gcc_version == "2.91.57" {
        self . dll_libraries = [ "msvcrt" ];
        self . warn (;
        "Consider upgrading to a newer version of gcc" );
        } else {
        self . dll_libraries = get_msvcr ( );
        pub fn _compile ( &self, obj , src , ext , cc_args , extra_postargs , pp_opts )  {
        "Compiles the source by spawning GCC && windres if needed.";
        if ext == ".rc" || ext == ".res" {
        // try {
        self . spawn ( [ "windres" , "-i" , src , "-o" , obj ] );
        // } catch  DistutilsExecError as msg  {
        panic!("CompileError ( msg )");
        } else {
        // try {
        self . spawn ( self . compiler_so + cc_args + [ src , "-o" , obj ] +;
        extra_postargs );
        // } catch  DistutilsExecError as msg  {
        panic!("CompileError ( msg )");
        pub fn link ( &self, target_desc , objects , output_filename , output_dir = None /* Option */ , {
        libraries = None /* Option */ , library_dirs = None /* Option */ , runtime_library_dirs = None /* Option */ ,;
        export_symbols = None /* Option */ , debug = 0 , extra_preargs = None /* Option */ ,;
        extra_postargs = None /* Option */ , build_temp = None /* Option */ , target_lang = None /* Option */ ) ;
        "Link the objects.";
        extra_preargs = copy . copy ( extra_preargs || [ ] );
        libraries = copy . copy ( libraries || [ ] );
        objects = copy . copy ( objects || [ ] );
        libraries . extend ( self . dll_libraries );
        if ( ( export_symbols is !None /* Option */ ) and {
        ( target_desc != self . EXECUTABLE || self . linker_dll == "gcc" ) ) ;
        temp_dir = os . path . dirname ( objects [ 0 ] );
        ( dll_name , dll_extension ) = os . path . splitext (;
        os . path . basename ( output_filename ) );
        def_file = os . path . join ( temp_dir , dll_name + ".deformat!(" ));
        lib_file = os . path . join ( temp_dir , "lib" + dll_name + ".a" );
        contents = [;
        "LIBRARY %s" % os . path . basename ( output_filename ) ,;
        "EXPORTS" ];
        for sym in export_symbols .iter() {
        contents . append ( sym );
        self . execute ( write_file , ( def_file , contents ) ,;
        "writing %s" % def_file );
        if self . linker_dll == "dllwrap" {
        extra_preargs . extend ( [ "--output-lib" , lib_file ] );
        extra_preargs . extend ( [ "--deformat!(" , def_file ] ));
        } else {
        objects . append ( def_file );
        if !debug {
        extra_preargs . append ( "-s" );
        UnixCCompiler . link ( self , target_desc , objects , output_filename ,;
        output_dir , libraries , library_dirs ,;
        runtime_library_dirs ,;
        None /* Option */ ,;
        debug , extra_preargs , extra_postargs , build_temp ,;
        target_lang );
        pub fn object_filenames ( &self, source_filenames , strip_dir = 0 , output_dir = "" )  {
        "Adds supports for rc && res files.";
        if output_dir is None /* Option */ {
        output_dir = "";
        obj_names = [ ];
        for src_name in source_filenames .iter() {
        base , ext = os . path . splitext ( os . path . normcase ( src_name ) );
        if ext !in ( self . src_extensions + [ ".rc" , ".res" ] ) {
        panic!("UnknownFileError ( "unknown file type '%s' (from '%s')" % \");
        ( ext , src_name ) );
        if strip_dir {
        base = os . path . basename ( base );
        if ext in ( ".res" , ".rc" ) {
        obj_names . append ( os . path . join ( output_dir ,;
        base + ext + self . obj_extension ) );
        } else {
        obj_names . append ( os . path . join ( output_dir ,;
        base + self . obj_extension ) );
        return  obj_names;
        class Mingw32CCompiler ( CygwinCCompiler ) ;
        " Handles the Mingw32 port of the GNU C compiler to Windows.
    ";
        compiler_type = "mingw32";
        pub fn __init__ ( &self, verbose = 0 , dry_run = 0 , force = 0 )  {
        CygwinCCompiler . __init__ ( self , verbose , dry_run , force );
        if self . ld_version >= "2.13" {
        shared_option = "-shared";
        } else {
        shared_option = "-mdll -static";
        if self . gcc_version <= "2.91.57" {
        entry_point = "--entry _DllMain@12";
        } else {
        entry_point = "";
        if is_cygwingcc ( ) {
        panic!("CCompilerError (");
        "Cygwin gcc cannot be used with --compiler=mingw32" );
        self . set_executables ( compiler = "gcc -O -Wall" ,;
        compiler_so = "gcc -mdll -O -Wall" ,;
        compiler_cxx = "g++ -O -Wall" ,;
        linker_exe = "gcc" ,;
        linker_so = "%s %s %s";
        % ( self . linker_dll , shared_option ,;
        entry_point ) );
        self . dll_libraries = [ ];
        self . dll_libraries = get_msvcr ( );
        CONFIG_H_OK = "ok";
        CONFIG_H_NOTOK = "not ok";
        CONFIG_H_UNCERTAIN = "uncertain";
        pub fn check_config_h ( )  {
        "Check if the current Python installation appears amenable to building
    extensions with GCC.

    Returns a tuple (status, details), where 'status' == one of the following
    constants:

    - CONFIG_H_OK: all == well, go ahead && compile
    - CONFIG_H_NOTOK: doesn't look good
    - CONFIG_H_UNCERTAIN: !sure -- unable to read pyconfig.h

    'details' == a human-readable string explaining the situation.

    Note there are two ways to conclude "OK": either 'sys.version' contains
    the string "GCC" (implying that this Python was built with GCC), || the
    installed "pyconfig.h" contains the string "__GNUC__".
    ";
        from distutils import sysconfig;
        if "GCC" in sys . version {
        return  CONFIG_H_OK , "sys.version mentions 'GCC'";
        fn = sysconfig . get_config_h_filename ( );
        // try {
        config_h = open ( fn );
        // try {
        if "__GNUC__" in config_h . read ( ) {
        return  CONFIG_H_OK , "'%s' mentions '__GNUC__'" % fn;
        } else {
        return  CONFIG_H_NOTOK , "'%s' does !mention '__GNUC__'" % fn;
        // } finally {
        config_h . close ( );
        // } catch  OSError as exc  {
        return  ( CONFIG_H_UNCERTAIN ,;
        "couldn't read '%s': %s" % ( fn , exc . strerror ) );
        RE_VERSION = re . compile ( br "(\d+\.\d+(\.\d+)*)" );
        pub fn _find_exe_version ( cmd )  {
        "Find the version of an executable by running `cmd` in the shell.

    If the command == !found, || the output does !match
    `RE_VERSION`, returns None /* Option */.
    ";
        executable = cmd . split ( ) [ 0 ];
        if find_executable ( executable ) is None /* Option */ {
        return;
        out = Popen ( cmd , shell = true , stdout = PIPE ) . stdout;
        // try {
        out_string = out . read ( );
        // } finally {
        out . close ( );
        result = RE_VERSION . search ( out_string );
        if result is None /* Option */ {
        return;
        return  LooseVersion ( result . group ( 1 ) . decode ( ) );
        pub fn get_versions ( )  {
        " Try to find out the versions of gcc, ld && dllwrap.

    If !possible it returns None /* Option */ for it.
    ";
        commands = [ "gcc -dumpversion" , "ld -v" , "dllwrap --version" ];
        return  tuple ( [ _find_exe_version ( cmd ) for cmd in commands ] );
        pub fn is_cygwingcc ( )  {
        "Try to determine if the gcc that would be used == from cygwin.";
        out_string = check_output ( [ "gcc" , "-dumpmachine" ] );
        return  out_string . strip ( ) . endswith ( b "cygwin" );
}

