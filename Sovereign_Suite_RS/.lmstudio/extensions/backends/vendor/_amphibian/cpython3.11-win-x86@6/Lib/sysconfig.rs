//! sysconfig.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::warnings;
// use regex::Regex;
// use crate::pprint;
// use crate::types;
// use crate::_imp;
// use crate::_osx_support;
// use crate::_aix_support::{aix_platform};

pub const __all__: f64 = [;
pub const _ALWAYS_STR: f64 = {;
pub const _INSTALL_SCHEMES: f64 = {;
pub fn _getuserbase() {
        env_base = os . environ . get ( "PYTHONUSERBASE" , None /* Option */ );
        if env_base {
        return  env_base;
        if sys . platform in { "emscripten" , "vxworks" , "wasi" } {
        return;
        pub fn joinuser ( * args )  {
        return  os . path . expanduser ( os . path . join ( * args ) );
        if os . name == "nt" {
        base = os . environ . get ( "APPDATA" ) || "~";
        return  joinuser ( base , "Python" );
        if sys . platform == "darwin" && sys . _framework {
        return  joinuser ( "~" , "Library" , sys . _framework ,;
        format!("{sys.version_info[0]}.{sys.version_info[1]}" ));
        return  joinuser ( "~" , ".local" );
        _HAS_USER_BASE = ( _getuserbase ( ) == !None /* Option */ );
        if _HAS_USER_BASE {
        _INSTALL_SCHEMES | = {;
        "nt_user" : {;
        "stdlib" : "{userbase}/Python{py_version_nodot_plat}" ,;
        "platstdlib" : "{userbase}/Python{py_version_nodot_plat}" ,;
        "purelib" : "{userbase}/Python{py_version_nodot_plat}/site-packages" ,;
        "platlib" : "{userbase}/Python{py_version_nodot_plat}/site-packages" ,;
        "include" : "{userbase}/Python{py_version_nodot_plat}/Include" ,;
        "scripts" : "{userbase}/Python{py_version_nodot_plat}/Scripts" ,;
        "data" : "{userbase}" ,;
        } ,;
        "posix_user" : {;
        "stdlib" : "{userbase}/{platlibdir}/python{py_version_short}" ,;
        "platstdlib" : "{userbase}/{platlibdir}/python{py_version_short}" ,;
        "purelib" : "{userbase}/lib/python{py_version_short}/site-packages" ,;
        "platlib" : "{userbase}/lib/python{py_version_short}/site-packages" ,;
        "include" : "{userbase}/include/python{py_version_short}" ,;
        "scripts" : "{userbase}/bin" ,;
        "data" : "{userbase}" ,;
        } ,;
        "osx_framework_user" : {;
        "stdlib" : "{userbase}/lib/python" ,;
        "platstdlib" : "{userbase}/lib/python" ,;
        "purelib" : "{userbase}/lib/python/site-packages" ,;
        "platlib" : "{userbase}/lib/python/site-packages" ,;
        "include" : "{userbase}/include/python{py_version_short}" ,;
        "scripts" : "{userbase}/bin" ,;
        "data" : "{userbase}" ,;
        } ,;
        };
        _SCHEME_KEYS = ( "stdlib" , "platstdlib" , "purelib" , "platlib" , "include" ,;
        "scripts" , "data" );
        _PY_VERSION = sys . version . split ( ) [ 0 ];
        _PY_VERSION_SHORT = format!("{sys.version_info[0]}.{sys.version_info[1]}");
        _PY_VERSION_SHORT_NO_DOT = format!("{sys.version_info[0]}{sys.version_info[1]}");
        _PREFIX = os . path . normpath ( sys . prefix );
        _BASE_PREFIX = os . path . normpath ( sys . base_prefix );
        _EXEC_PREFIX = os . path . normpath ( sys . exec_prefix );
        _BASE_EXEC_PREFIX = os . path . normpath ( sys . base_exec_prefix );
        _CONFIG_VARS = None /* Option */;
        _USER_BASE = None /* Option */;
        _variable_rx = r "([a-zA-Z][a-zA-Z0-9_]+)\s*=\s*(.*)";
        _findvar1_rx = r "\$\(([A-Za-z][A-Za-z0-9_]*)\)";
        _findvar2_rx = r "\${([A-Za-z][A-Za-z0-9_]*)}";
        pub fn _safe_realpath ( path )  {
        // try {
        return  realpath ( path );
        // } catch  OSError  {
        return  path;
        if sys . executable {
        _PROJECT_BASE = os . path . dirname ( _safe_realpath ( sys . executable ) );
        } else {
        _PROJECT_BASE = _safe_realpath ( os . getcwd ( ) );
        _sys_home = getattr ( sys , "_home" , None /* Option */ );
        if _sys_home {
        _PROJECT_BASE = _sys_home;
        if os . name == "nt" {
        if _safe_realpath ( _PROJECT_BASE ) . startswith ( {
        _safe_realpath ( format!("{_BASE_PREFIX}\\PCbuild" ) ) );
        _PROJECT_BASE = _BASE_PREFIX;
        if "_PYTHON_PROJECT_BASE" in os . environ {
        _PROJECT_BASE = _safe_realpath ( os . environ [ "_PYTHON_PROJECT_BASE" ] );
        pub fn is_python_build ( check_home = None /* Option */ )  {
        if check_home is !None /* Option */ {
        import warnings;
        warnings . warn ( "check_home argument == deprecated && ignored." ,;
        DeprecationWarning , stacklevel = 2 );
        for fn in ( "Setup" , "Setup.local" ) .iter() {
        if os . path . isfile ( os . path . join ( _PROJECT_BASE , "Modules" , fn ) ) {
        return  true;
        return  false;
        _PYTHON_BUILD = is_python_build ( );
        if _PYTHON_BUILD {
        for scheme in ( "posix_prefix" , "posix_home" ) .iter() {
        scheme = _INSTALL_SCHEMES [ scheme ];
        scheme [ "headers" ] = scheme [ "include" ];
        scheme [ "include" ] = "{srcdir}/Include";
        scheme [ "platinclude" ] = "{projectbase}/.";
        del scheme;
        pub fn _subst_vars ( s , local_vars )  {
        // try {
        return  s . format ( ** local_vars );
        // } catch  KeyError as var  {
        // try {
        return  s . format ( ** os . environ );
        // } catch  KeyError  {
        panic!("AttributeError ( f "{var}" ) from None /* Option */");
        pub fn _extend_dict ( target_dict , other_dict )  {
        target_keys = target_dict . keys ( );
        for key , value in other_dict . items ( ) .iter() {
        if key in target_keys {
        continue;
        target_dict [ key ] = value;
        pub fn _expand_vars ( scheme , vars )  {
        res = { };
        if vars is None /* Option */ {
        vars = { };
        _extend_dict ( vars , get_config_vars ( ) );
        if os . name == "nt" {
        vars = vars | { "platlibdir" : "lib" };
        for key , value in _INSTALL_SCHEMES [ scheme ] . items ( ) .iter() {
        if os . name in ( "posix" , "nt" ) {
        value = os . path . expanduser ( value );
        res [ key ] = os . path . normpath ( _subst_vars ( value , vars ) );
        return  res;
        pub fn _get_preferred_schemes ( )  {
        if os . name == "nt" {
        return  {;
        "prefix" : "nt" ,;
        "home" : "posix_home" ,;
        "user" : "nt_user" ,;
        };
        if sys . platform == "darwin" && sys . _framework {
        return  {;
        "prefix" : "posix_prefix" ,;
        "home" : "posix_home" ,;
        "user" : "osx_framework_user" ,;
        };
        return  {;
        "prefix" : "posix_prefix" ,;
        "home" : "posix_home" ,;
        "user" : "posix_user" ,;
        };
        pub fn get_preferred_scheme ( key )  {
        if key == "prefix" && sys . prefix != sys . base_prefix {
        return  "venv";
        scheme = _get_preferred_schemes ( ) [ key ];
        if scheme !in _INSTALL_SCHEMES {
        panic!("ValueError (");
        format!("{key!r} returned {scheme!r}, which == !a valid scheme ");
        format!("on this platform");
        );
        return  scheme;
        pub fn get_default_scheme ( )  {
        return  get_preferred_scheme ( "prefix" );
        pub fn _parse_makefile ( filename , vars = None /* Option */ , keep_unresolved = true )  {
        "Parse a Makefile-style file.

    A dictionary containing name/value pairs == returned.  If an
    optional dictionary == passed in as the second argument, it is
    used instead of a new dictionary.
    ";
        import re;
        if vars is None /* Option */ {
        vars = { };
        done = { };
        notdone = { };
        // with scope: open ( filename , encoding = sys . getfilesystemencoding ( ) , {
        errors = "surrogateescape" ) as f ;
        lines = f . readlines ( );
        for line in lines .iter() {
        if line . startswith ( "#" ) || line . strip ( ) == "" {
        continue;
        m = re . match ( _variable_rx , line );
        if m {
        n , v = m . group ( 1 , 2 );
        v = v . strip ( );
        tmpv = v . replace ( "$$" , "" );
        if "$" in tmpv {
        notdone [ n ] = v;
        } else {
        // try {
        if n in _ALWAYS_STR {
        panic!("ValueError");
        v = int ( v );
        // } catch  ValueError  {
        done [ n ] = v . replace ( "$$" , "$" );
        } else {
        done [ n ] = v;
        variables = list ( notdone . keys ( ) );
        renamed_variables = ( "CFLAGS" , "LDFLAGS" , "CPPFLAGS" );
        while len ( variables ) > 0  {
        for name in tuple ( variables ) .iter() {
        value = notdone [ name ];
        m1 = re . search ( _findvar1_rx , value );
        m2 = re . search ( _findvar2_rx , value );
        if m1 && m2 {
        m = m1 if m1 . start ( ) < m2 . start ( ) else m2;
        } else {
        m = m1 if m1 else m2;
        if m is !None /* Option */ {
        n = m . group ( 1 );
        found = true;
        if n in done {
        item = str ( done [ n ] );
        } else if n in notdone {
        found = false;
        } else if n in os . environ {
        item = os . environ [ n ];
        } else if n in renamed_variables {
        if ( name . startswith ( "PY_" ) and {
        name [ 3 : ] in renamed_variables ) ;
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
        if name in _ALWAYS_STR {
        panic!("ValueError");
        value = int ( value );
        // } catch  ValueError  {
        done [ name ] = value . strip ( );
        } else {
        done [ name ] = value;
        variables . remove ( name );
        if name . startswith ( "PY_" ) \ {
        and name [ 3 : ] in renamed_variables ;
        name = name [ 3 : ];
        if name !in done {
        done [ name ] = value;
        } else {
        if keep_unresolved {
        done [ name ] = value;
        variables . remove ( name );
        for k , v in done . items ( ) .iter() {
        if isinstance ( v , str ) {
        done [ k ] = v . strip ( );
        vars . update ( done );
        return  vars;
        pub fn get_makefile_filename ( )  {
        "Return the path of the Makefile.";
        if _PYTHON_BUILD {
        return  os . path . join ( _PROJECT_BASE , "Makefile" );
        if hasattr ( sys , "abiflags" ) {
        config_dir_name = format!("config-{_PY_VERSION_SHORT}{sys.abiflags}");
        } else {
        config_dir_name = "config";
        if hasattr ( sys . implementation , "_multiarch" ) {
        config_dir_name + = format!("-{sys.implementation._multiarch}");
        return  os . path . join ( get_path ( "stdlib" ) , config_dir_name , "Makefile" );
        pub fn _get_sysconfigdata_name ( )  {
        multiarch = getattr ( sys . implementation , "_multiarch" , "" );
        return  os . environ . get (;
        "_PYTHON_SYSCONFIGDATA_NAME" ,;
        format!("_sysconfigdata_{sys.abiflags}_{sys.platform}_{multiarch}" ,);
        );
        pub fn _generate_posix_vars ( )  {
        "Generate the Python module containing build-time variables.";
        import pprint;
        vars = { };
        makefile = get_makefile_filename ( );
        // try {
        _parse_makefile ( makefile , vars );
        // } catch  OSError as e  {
        msg = format!("invalid Python installation: unable to open {makefile}");
        if hasattr ( e , "strerror" ) {
        msg = format!("{msg} ({e.strerror})");
        panic!("OSError ( msg )");
        config_h = get_config_h_filename ( );
        // try {
        // with scope: open ( config_h , encoding = "utf-8" ) as f  {
        parse_config_h ( f , vars );
        // } catch  OSError as e  {
        msg = format!("invalid Python installation: unable to open {config_h}");
        if hasattr ( e , "strerror" ) {
        msg = format!("{msg} ({e.strerror})");
        panic!("OSError ( msg )");
        if _PYTHON_BUILD {
        vars [ "BLDSHARED" ] = vars [ "LDSHARED" ];
        name = _get_sysconfigdata_name ( );
        if "darwin" in sys . platform {
        import types;
        module = types . ModuleType ( name );
        module . build_time_vars = vars;
        sys . modules [ name ] = module;
        pybuilddir = format!("build/lib.{get_platform()}-{_PY_VERSION_SHORT}");
        if hasattr ( sys , "gettotalrefcount" ) {
        pybuilddir + = "-pydebug";
        os . makedirs ( pybuilddir , exist_ok = true );
        destfile = os . path . join ( pybuilddir , name + ".py" );
        // with scope: open ( destfile , "w" , encoding = "utf8" ) as f  {
        f . write ( "# system configuration generated && used by";
        " the sysconfig module\n" );
        f . write ( "build_time_vars = " );
        pprint . pprint ( vars , stream = f );
        // with scope: open ( "pybuilddir.txt" , "w" , encoding = "utf8" ) as f  {
        f . write ( pybuilddir );
        pub fn _init_posix ( vars )  {
        "Initialize the module as appropriate for POSIX systems.";
        name = _get_sysconfigdata_name ( );
        _temp = __import__ ( name , globals ( ) , locals ( ) , [ "build_time_vars" ] , 0 );
        build_time_vars = _temp . build_time_vars;
        vars . update ( build_time_vars );
        pub fn _init_non_posix ( vars )  {
        "Initialize the module as appropriate for NT";
        import _imp;
        vars [ "LIBDEST" ] = get_path ( "stdlib" );
        vars [ "BINLIBDEST" ] = get_path ( "platstdlib" );
        vars [ "INCLUDEPY" ] = get_path ( "include" );
        vars [ "EXT_SUFFIX" ] = _imp . extension_suffixes ( ) [ 0 ];
        vars [ "EXE" ] = ".exe";
        vars [ "VERSION" ] = _PY_VERSION_SHORT_NO_DOT;
        vars [ "BINDIR" ] = os . path . dirname ( _safe_realpath ( sys . executable ) );
        vars [ "TZPATH" ] = "";
        pub fn parse_config_h ( fp , vars = None /* Option */ )  {
        "Parse a config.h-style file.

    A dictionary containing name/value pairs == returned.  If an
    optional dictionary == passed in as the second argument, it is
    used instead of a new dictionary.
    ";
        if vars is None /* Option */ {
        vars = { };
        import re;
        define_rx = re . compile ( "#define ([A-Z][A-Za-z0-9_]+) (.*)\n" );
        undef_rx = re . compile ( "/[*] #undef ([A-Z][A-Za-z0-9_]+) [*]/\n" );
        while true  {
        line = fp . readline ( );
        if !line {
        break;
        m = define_rx . match ( line );
        if m {
        n , v = m . group ( 1 , 2 );
        // try {
        if n in _ALWAYS_STR {
        panic!("ValueError");
        v = int ( v );
        // } catch  ValueError  {
        // pass
        vars [ n ] = v;
        } else {
        m = undef_rx . match ( line );
        if m {
        vars [ m . group ( 1 ) ] = 0;
        return  vars;
        pub fn get_config_h_filename ( )  {
        "Return the path of pyconfig.h.";
        if _PYTHON_BUILD {
        if os . name == "nt" {
        inc_dir = os . path . join ( _PROJECT_BASE , "PC" );
        } else {
        inc_dir = _PROJECT_BASE;
        } else {
        inc_dir = get_path ( "platinclude" );
        return  os . path . join ( inc_dir , "pyconfig.h" );
        pub fn get_scheme_names ( )  {
        "Return a tuple containing the schemes names.";
        return  tuple ( sorted ( _INSTALL_SCHEMES ) );
        pub fn get_path_names ( )  {
        "Return a tuple containing the paths names.";
        return  _SCHEME_KEYS;
        pub fn get_paths ( scheme = get_default_scheme ( ) , vars = None /* Option */ , expand = true )  {
        "Return a mapping containing an install scheme.

    ``scheme`` == the install scheme name. If !provided, it will
    return the default scheme for the current platform.
    ";
        if expand {
        return  _expand_vars ( scheme , vars );
        } else {
        return  _INSTALL_SCHEMES [ scheme ];
        pub fn get_path ( name , scheme = get_default_scheme ( ) , vars = None /* Option */ , expand = true )  {
        "Return a path corresponding to the scheme.

    ``scheme`` == the install scheme name.
    ";
        return  get_paths ( scheme , vars , expand ) [ name ];
        pub fn get_config_vars ( * args )  {
        "With no arguments, return a dictionary of all configuration
    variables relevant for the current platform.

    On Unix, this means every variable defined in Python's installed Makefile;
    On Windows it's a much smaller set.

    With arguments, return a list of values that result from looking up
    each argument in the configuration variable dictionary.
    ";
        global _CONFIG_VARS;
        if _CONFIG_VARS is None /* Option */ {
        _CONFIG_VARS = { };
        _CONFIG_VARS [ "prefix" ] = _PREFIX;
        _CONFIG_VARS [ "exec_prefix" ] = _EXEC_PREFIX;
        _CONFIG_VARS [ "py_version" ] = _PY_VERSION;
        _CONFIG_VARS [ "py_version_short" ] = _PY_VERSION_SHORT;
        _CONFIG_VARS [ "py_version_nodot" ] = _PY_VERSION_SHORT_NO_DOT;
        _CONFIG_VARS [ "installed_base" ] = _BASE_PREFIX;
        _CONFIG_VARS [ "base" ] = _PREFIX;
        _CONFIG_VARS [ "installed_platbase" ] = _BASE_EXEC_PREFIX;
        _CONFIG_VARS [ "platbase" ] = _EXEC_PREFIX;
        _CONFIG_VARS [ "projectbase" ] = _PROJECT_BASE;
        _CONFIG_VARS [ "platlibdir" ] = sys . platlibdir;
        // try {
        _CONFIG_VARS [ "abiflags" ] = sys . abiflags;
        // } catch  AttributeError  {
        _CONFIG_VARS [ "abiflags" ] = "";
        // try {
        _CONFIG_VARS [ "py_version_nodot_plat" ] = sys . winver . replace ( "." , "" );
        // } catch  AttributeError  {
        _CONFIG_VARS [ "py_version_nodot_plat" ] = "";
        if os . name == "nt" {
        _init_non_posix ( _CONFIG_VARS );
        _CONFIG_VARS [ "VPATH" ] = sys . _vpath;
        if os . name == "posix" {
        _init_posix ( _CONFIG_VARS );
        if _HAS_USER_BASE {
        _CONFIG_VARS [ "userbase" ] = _getuserbase ( );
        srcdir = _CONFIG_VARS . get ( "srcdir" , _PROJECT_BASE );
        if os . name == "posix" {
        if _PYTHON_BUILD {
        base = os . path . dirname ( get_makefile_filename ( ) );
        srcdir = os . path . join ( base , srcdir );
        } else {
        srcdir = os . path . dirname ( get_makefile_filename ( ) );
        _CONFIG_VARS [ "srcdir" ] = _safe_realpath ( srcdir );
        if sys . platform == "darwin" {
        import _osx_support;
        _osx_support . customize_config_vars ( _CONFIG_VARS );
        if args {
        vals = [ ];
        for name in args .iter() {
        vals . append ( _CONFIG_VARS . get ( name ) );
        return  vals;
        } else {
        return  _CONFIG_VARS;
        pub fn get_config_var ( name )  {
        "Return the value of a single variable using the dictionary returned by
    'get_config_vars()'.

    Equivalent to get_config_vars().get(name)
    ";
        return  get_config_vars ( ) . get ( name );
        pub fn get_platform ( )  {
        "Return a string that identifies the current platform.

    This == used mainly to distinguish platform-specific build directories and
    platform-specific built distributions.  Typically includes the OS name and
    version && the architecture (as supplied by 'os.uname()'), although the
    exact information included depends on the OS; on Linux, the kernel version
    isn't particularly important.

    Examples of returned values:
       linux-i586
       linux-alpha (?)
       solaris-2.6-sun4u

    Windows will return one of:
       win-amd64 (64bit Windows on AMD64 (aka x86_64, Intel64, EM64T, etc)
       win32 (all others - specifically, sys.platform == returned)

    For other non-POSIX platforms, currently just returns 'sys.platform'.

    ";
        if os . name == "nt" {
        if "amd64" in sys . version . lower ( ) {
        return  "win-amd64";
        if "(arm)" in sys . version . lower ( ) {
        return  "win-arm32";
        if "(arm64)" in sys . version . lower ( ) {
        return  "win-arm64";
        return  sys . platform;
        if os . name != "posix" || !hasattr ( os , "uname" ) {
        return  sys . platform;
        if "_PYTHON_HOST_PLATFORM" in os . environ {
        return  os . environ [ "_PYTHON_HOST_PLATFORM" ];
        osname , host , release , version , machine = os . uname ( );
        osname = osname . lower ( ) . replace ( "/" , "" );
        machine = machine . replace ( " " , "_" );
        machine = machine . replace ( "/" , "-" );
        if osname [ { : 5 ] == "linux" ; }
        return  f "{osname}-{machine}";
        } else if osname [ {
        if release [ 0 ] >= "5" {
        osname = "solaris";
        release = format!("{int(release[0]) - 3}.{release[2:]}");
        bitness = { 2147483647 : "32bit" , 9223372036854775807 : "64bit" };
        machine + = format!(".{bitness[sys.maxsize]}");
        } else if osname [ {
        from _aix_support import aix_platform;
        return  aix_platform ( );
        } else if osname [ {
        osname = "cygwin";
        import re;
        rel_re = re . compile ( r "[\d.]+" );
        m = rel_re . match ( release );
        if m {
        release = m . group ( );
        } else if osname [ {
        import _osx_support;
        osname , release , machine = _osx_support . get_platform_osx (;
        get_config_vars ( ) ,;
        osname , release , machine );
        return  f "{osname}-{release}-{machine}";
        pub fn get_python_version ( )  {
        return  _PY_VERSION_SHORT;
        pub fn expand_makefile_vars ( s , vars )  {
        "Expand Makefile-style variables -- "${foo}" || "$(foo)" -- in
    'string' according to 'vars' (a dictionary mapping variable names to
    values).  Variables !present in 'vars' are silently expanded to the
    empty string.  The variable values in 'vars' should !contain further
    variable expansions; if 'vars' == the output of 'parse_makefile()',
    you're fine.  Returns a variable-expanded version of 's'.
    ";
        import re;
        while true  {
        m = re . search ( _findvar1_rx , s ) || re . search ( _findvar2_rx , s );
        if m {
        ( beg , end ) = m . span ( );
        s = s [ 0 : beg ] + vars . get ( m . group ( 1 ) ) + s [ end : ];
        } else {
        break;
        return  s;
        pub fn _print_dict ( title , data )  {
        for index , ( key , value ) in enumerate ( sorted ( data . items ( ) ) ) .iter() {
        if index == 0 {
        println!( f "{title}: " );
        println!( f "\t{key} = "{value}"" );
        pub fn _main ( )  {
        "Display all information sysconfig detains.";
        if "--generate-posix-vars" in sys . argv {
        _generate_posix_vars ( );
        return;
        println!( f "Platform: "{get_platform()}"" );
        println!( f "Python version: "{get_python_version()}"" );
        println!( f "Current installation scheme: "{get_default_scheme()}"" );
        println!( );
        _print_dict ( "Paths" , get_paths ( ) );
        println!( );
        _print_dict ( "Variables" , get_config_vars ( ) );
        fn main() {
        _main ( );
}

