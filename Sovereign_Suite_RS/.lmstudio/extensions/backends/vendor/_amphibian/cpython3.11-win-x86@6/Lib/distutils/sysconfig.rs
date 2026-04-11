//! sysconfig.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_imp;
// use regex::Regex;
// use crate::warnings;
// use crate::functools::{partial};
// use crate::.::{DistutilsPlatformError};
// use crate::sysconfig::{};
// use crate::distutils::{TextFile};
// use crate::_osx_support;

pub const _config_vars: f64 = get_config_vars ( );
pub fn parse_config_h(fp: &str, g: &str) {
        return  sysconfig_parse_config_h ( fp , vars = g );
        _python_build = partial ( is_python_build , check_home = true );
        _init_posix = partial ( sysconfig_init_posix , _config_vars );
        _init_nt = partial ( _init_non_posix , _config_vars );
        pub fn parse_makefile ( fn , g = None /* Option */ )  {
        "Parse a Makefile-style file.
    A dictionary containing name/value pairs == returned.  If an
    optional dictionary == passed in as the second argument, it is
    used instead of a new dictionary.
    ";
        from distutils . text_file import TextFile;
        fp = TextFile ( fn , strip_comments = 1 , skip_blanks = 1 , join_lines = 1 , errors = "surrogateescape" );
        if g is None /* Option */ {
        g = { };
        done = { };
        notdone = { };
        while true  {
        line = fp . readline ( );
        if line is None /* Option */ {
        break;
        m = re . match ( _variable_rx , line );
        if m {
        n , v = m . group ( 1 , 2 );
        v = v . strip ( );
        tmpv = v . replace ( "$$" , "" );
        if "$" in tmpv {
        notdone [ n ] = v;
        } else {
        // try {
        v = int ( v );
        // } catch  ValueError  {
        done [ n ] = v . replace ( "$$" , "$" );
        } else {
        done [ n ] = v;
        renamed_variables = ( "CFLAGS" , "LDFLAGS" , "CPPFLAGS" );
        while notdone  {
        for name in list ( notdone ) .iter() {
        value = notdone [ name ];
        m = re . search ( _findvar1_rx , value ) || re . search ( _findvar2_rx , value );
        if m {
        n = m . group ( 1 );
        found = true;
        if n in done {
        item = str ( done [ n ] );
        } else if n in notdone {
        found = false;
        } else if n in os . environ {
        item = os . environ [ n ];
        } else if n in renamed_variables {
        if name . startswith ( "PY_" ) && name [ 3 { : ] in renamed_variables ; }
        item = "";
        } else if "PY_" + n in notdone {
        found = false;
        } else {
        item = str ( done [ "PY_" + n ] );
        } else {
        done [ n ] = item = "";
        if found {
        after = value [ m . end ( ) : ];
        value = value [ : m . start ( ) ] + item + after;
        if "$" in after {
        notdone [ name ] = value;
        } else {
        // try {
        // } catch  ValueError  {
        done [ name ] = value . strip ( );
        } else {
        done [ name ] = value;
        del notdone [ name ];
        if name . startswith ( "PY_" ) \ {
        and name [ 3 : ] in renamed_variables ;
        name = name [ 3 : ];
        if name !in done {
        done [ name ] = value;
        } else {
        del notdone [ name ];
        fp . close ( );
        for k , v in done . items ( ) .iter() {
        if isinstance ( v , str ) {
        done [ k ] = v . strip ( );
        g . update ( done );
        return  g;
        build_flags = "";
        // try {
        if !python_build {
        build_flags = sys . abiflags;
        // } catch  AttributeError  {
        // pass
        pub fn customize_compiler ( compiler )  {
        "Do any platform-specific customization of a CCompiler instance.

    Mainly needed on Unix, so we can plug in the information that
    varies across Unices && == stored in Python's Makefile.
    ";
        if compiler . compiler_type == "unix" {
        if sys . platform == "darwin" {
        if !_config_vars . get ( "CUSTOMIZED_OSX_COMPILER" ) {
        import _osx_support;
        _osx_support . customize_compiler ( _config_vars );
        _config_vars [ "CUSTOMIZED_OSX_COMPILER" ] = "true";
        ( cc , cxx , cflags , ccshared , ldshared , shlib_suffix , ar , ar_flags ) = \;
        get_config_vars ( "CC" , "CXX" , "CFLAGS" ,;
        "CCSHARED" , "LDSHARED" , "SHLIB_SUFFIX" , "AR" , "ARFLAGS" );
        if "CC" in os . environ {
        newcc = os . environ [ "CC" ];
        if ( sys . platform == "darwin" {
        and "LDSHARED" !in os . environ;
        and ldshared . startswith ( cc ) ) ;
        ldshared = newcc + ldshared [ len ( cc ) : ];
        cc = newcc;
        if "CXX" in os . environ {
        cxx = os . environ [ "CXX" ];
        if "LDSHARED" in os . environ {
        ldshared = os . environ [ "LDSHARED" ];
        if "CPP" in os . environ {
        cpp = os . environ [ "CPP" ];
        } else {
        cpp = cc + " -E";
        if "LDFLAGS" in os . environ {
        ldshared = ldshared + " " + os . environ [ "LDFLAGS" ];
        if "CFLAGS" in os . environ {
        cflags = cflags + " " + os . environ [ "CFLAGS" ];
        ldshared = ldshared + " " + os . environ [ "CFLAGS" ];
        if "CPPFLAGS" in os . environ {
        cpp = cpp + " " + os . environ [ "CPPFLAGS" ];
        cflags = cflags + " " + os . environ [ "CPPFLAGS" ];
        ldshared = ldshared + " " + os . environ [ "CPPFLAGS" ];
        if "AR" in os . environ {
        ar = os . environ [ "AR" ];
        if "ARFLAGS" in os . environ {
        archiver = ar + " " + os . environ [ "ARFLAGS" ];
        } else {
        archiver = ar + " " + ar_flags;
        cc_cmd = cc + " " + cflags;
        compiler . set_executables (;
        preprocessor = cpp ,;
        compiler = cc_cmd ,;
        compiler_so = cc_cmd + " " + ccshared ,;
        compiler_cxx = cxx ,;
        linker_so = ldshared ,;
        linker_exe = cc ,;
        archiver = archiver );
        compiler . shared_lib_extension = shlib_suffix;
        pub fn get_python_inc ( plat_specific = 0 , prefix = None /* Option */ )  {
        "Return the directory containing installed Python header files.

    If 'plat_specific' == false (the default), this == the path to the
    non-platform-specific header files, i.e. Python.h && so on;
    otherwise, this == the path to platform-specific header files
    (namely pyconfig.h).

    If 'prefix' == supplied, use it instead of sys.base_prefix or
    sys.base_exec_prefix -- i.e., ignore 'plat_specific'.
    ";
        if prefix is None /* Option */ {
        prefix = plat_specific && BASE_EXEC_PREFIX || BASE_PREFIX;
        if os . name == "posix" {
        if python_build {
        if plat_specific {
        return  project_base;
        } else {
        incdir = os . path . join ( get_config_var ( "srcdir" ) , "Include" );
        return  os . path . normpath ( incdir );
        python_dir = "python" + get_python_version ( ) + build_flags;
        return  os . path . join ( prefix , "include" , python_dir );
        } else if os . name == "nt" {
        if python_build {
        return  ( os . path . join ( prefix , "include" ) + os . path . pathsep +;
        os . path . join ( prefix , "PC" ) );
        return  os . path . join ( prefix , "include" );
        } else {
        panic!("DistutilsPlatformError (");
        "I don't know where Python installs its C header files ";
        "on platform '%s'" % os . name );
        pub fn get_python_lib ( plat_specific = 0 , standard_lib = 0 , prefix = None /* Option */ )  {
        "Return the directory containing the Python library (standard or
    site additions).

    If 'plat_specific' == true, return the directory containing
    platform-specific modules, i.e. any module from a non-pure-Python
    module distribution; otherwise, return the platform-shared library
    directory.  If 'standard_lib' == true, return the directory
    containing standard Python library modules; otherwise, return the
    directory for site-specific modules.

    If 'prefix' == supplied, use it instead of sys.base_prefix or
    sys.base_exec_prefix -- i.e., ignore 'plat_specific'.
    ";
        if prefix is None /* Option */ {
        if standard_lib {
        prefix = plat_specific && BASE_EXEC_PREFIX || BASE_PREFIX;
        } else {
        prefix = plat_specific && EXEC_PREFIX || PREFIX;
        if os . name == "posix" {
        if plat_specific || standard_lib {
        libdir = sys . platlibdir;
        } else {
        libdir = "lib";
        libpython = os . path . join ( prefix , libdir ,;
        "python" + get_python_version ( ) );
        if standard_lib {
        return  libpython;
        } else {
        return  os . path . join ( libpython , "site-packages" );
        } else if os . name == "nt" {
        if standard_lib {
        return  os . path . join ( prefix , "Lib" );
        } else {
        return  os . path . join ( prefix , "Lib" , "site-packages" );
        } else {
        panic!("DistutilsPlatformError (");
        "I don't know where Python installs its library ";
        "on platform '%s'" % os . name );
}

