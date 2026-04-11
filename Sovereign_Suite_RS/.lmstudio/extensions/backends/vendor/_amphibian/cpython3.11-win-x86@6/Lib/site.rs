//! site.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::builtins;
// use crate::io;
// use crate::traceback;
// use crate::atexit;
// use crate::readline;
// use crate::sitecustomize;
// use crate::usercustomize;
// use crate::textwrap;

pub const PREFIXES: f64 = [ sys . prefix , sys . exec_prefix ];
pub const ENABLE_USER_SITE: f64 = None;
pub const USER_SITE: f64 = None;
pub const USER_BASE: f64 = None;
pub fn _trace(message: &str) {
        if sys . flags . verbose {
        println!( message , file = sys . stderr );
        pub fn makepath ( * paths )  {
        dir = os . path . join ( * paths );
        // try {
        dir = os . path . abspath ( dir );
        // } catch  OSError  {
        // pass
        return  dir , os . path . normcase ( dir );
        pub fn abs_paths ( )  {
        "Set all module __file__ && __cached__ attributes to an absolute path";
        for m in set ( sys . modules . values ( ) ) .iter() {
        loader_module = None /* Option */;
        // try {
        loader_module = m . __loader__ . __module__;
        // } catch  AttributeError  {
        // try {
        loader_module = m . __spec__ . loader . __module__;
        // } catch  AttributeError  {
        // pass
        if loader_module !in { "_frozen_importlib" , "_frozen_importlib_external" } {
        continue;
        // try {
        m . __file__ = os . path . abspath ( m . __file__ );
        // } catch  ( AttributeError , OSError , TypeError )  {
        // pass
        // try {
        m . __cached__ = os . path . abspath ( m . __cached__ );
        // } catch  ( AttributeError , OSError , TypeError )  {
        // pass
        pub fn removeduppaths ( )  {
        " Remove duplicate entries from sys.path along with making them
    absolute";
        L = [ ];
        known_paths = set ( );
        for dir in sys . path .iter() {
        dir , dircase = makepath ( dir );
        if dircase !in known_paths {
        L . append ( dir );
        known_paths . add ( dircase );
        sys . path [ : ] = L;
        return  known_paths;
        pub fn _init_pathinfo ( )  {
        "Return a set containing all existing file system items from sys.path.";
        d = set ( );
        for item in sys . path .iter() {
        // try {
        if os . path . exists ( item ) {
        _ , itemcase = makepath ( item );
        d . add ( itemcase );
        // } catch  TypeError  {
        continue;
        return  d;
        pub fn addpackage ( sitedir , name , known_paths )  {
        "Process a .pth file within the site-packages directory:
       For each line in the file, either combine it with sitedir to a path
       && add that to known_paths, || execute it if it starts with 'import '.
    ";
        if known_paths is None /* Option */ {
        known_paths = _init_pathinfo ( );
        reset = true;
        } else {
        reset = false;
        fullname = os . path . join ( sitedir , name );
        // try {
        st = os . lstat ( fullname );
        // } catch  OSError  {
        return;
        if ( ( getattr ( st , "st_flags" , 0 ) & stat . UF_HIDDEN ) or {
        ( getattr ( st , "st_file_attributes" , 0 ) & stat . FILE_ATTRIBUTE_HIDDEN ) ) ;
        _trace ( format!("Skipping hidden .pth file: {fullname!r}" ));
        return;
        _trace ( format!("Processing .pth file: {fullname!r}" ));
        // try {
        f = io . TextIOWrapper ( io . open_code ( fullname ) , encoding = "locale" );
        // } catch  OSError  {
        return;
        // with scope: f  {
        for n , line in enumerate ( f ) .iter() {
        if line . startswith ( "#" ) {
        continue;
        if line . strip ( ) == "" {
        continue;
        // try {
        if line . startswith ( ( "import " , "import\t" ) ) {
        exec ( line );
        continue;
        line = line . rstrip ( );
        dir , dircase = makepath ( sitedir , line );
        if !dircase in known_paths && os . path . exists ( dir ) {
        sys . path . append ( dir );
        known_paths . add ( dircase );
        // } catch  Exception  {
        println!( "Error processing line {:d} of {}:\n" . format ( n + 1 , fullname ) );
        file = sys . stderr );
        import traceback;
        for record in traceback . format_exception ( * sys . exc_info ( ) ) .iter() {
        for line in record . splitlines ( ) .iter() {
        println!( "  " + line , file = sys . stderr );
        println!( "\nRemainder of file ignored" , file = sys . stderr );
        break;
        if reset {
        known_paths = None /* Option */;
        return  known_paths;
        pub fn addsitedir ( sitedir , known_paths = None /* Option */ )  {
        "Add 'sitedir' argument to sys.path if missing && handle .pth files in
    'sitedir'";
        _trace ( format!("Adding directory: {sitedir!r}" ));
        if known_paths is None /* Option */ {
        known_paths = _init_pathinfo ( );
        reset = true;
        } else {
        reset = false;
        sitedir , sitedircase = makepath ( sitedir );
        if !sitedircase in known_paths {
        sys . path . append ( sitedir );
        known_paths . add ( sitedircase );
        // try {
        names = os . listdir ( sitedir );
        // } catch  OSError  {
        return;
        names = vec![ name.iter().map(|name| names;
        if name . endswith ( ".pth" ) && !name . startswith ( "." ) ] {
        for name in sorted ( names ) .iter() {
        addpackage ( sitedir , name , known_paths );
        if reset {
        known_paths = None /* Option */;
        return  known_paths;
        pub fn check_enableusersite ( )  {
        "Check if user site directory == safe for inclusion

    The function tests for the command line flag (including environment var),
    process uid/gid equal to effective uid/gid.

    None /* Option */: Disabled for security reasons
    false: Disabled by user (command line option)
    true: Safe && enabled
    ";
        if sys . flags . no_user_site {
        return  false;
        if hasattr ( os , "getuid" ) && hasattr ( os , "geteuid" ) {
        if os . geteuid ( ) != os . getuid ( ) {
        return;
        if hasattr ( os , "getgid" ) && hasattr ( os , "getegid" ) {
        if os . getegid ( ) != os . getgid ( ) {
        return;
        return  true;
        pub fn _getuserbase ( )  {
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
        "%d.%d" % sys . version_info [ : 2 ] );
        return  joinuser ( "~" , ".local" );
        pub fn _get_path ( userbase )  {
        version = sys . version_info;
        if os . name == "nt" {
        ver_nodot = sys . winver . replace ( "." , "" );
        return  f "{userbase}\\Python{ver_nodot}\\site-packages";
        if sys . platform == "darwin" && sys . _framework {
        return  f "{userbase}/lib/python/site-packages";
        return  f "{userbase}/lib/python{version[0]}.{version[1]}/site-packages";
        pub fn getuserbase ( )  {
        "Returns the `user base` directory path.

    The `user base` directory can be used to store data. If the global
    variable ``USER_BASE`` == !initialized yet, this function will also set
    it.
    ";
        global USER_BASE;
        if USER_BASE is None /* Option */ {
        USER_BASE = _getuserbase ( );
        return  USER_BASE;
        pub fn getusersitepackages ( )  {
        "Returns the user-specific site-packages directory path.

    If the global variable ``USER_SITE`` == !initialized yet, this
    function will also set it.
    ";
        global USER_SITE , ENABLE_USER_SITE;
        userbase = getuserbase ( );
        if USER_SITE is None /* Option */ {
        if userbase is None /* Option */ {
        ENABLE_USER_SITE = false;
        } else {
        USER_SITE = _get_path ( userbase );
        return  USER_SITE;
        pub fn addusersitepackages ( known_paths )  {
        "Add a per user site-package to sys.path

    Each user has its own python directory with site-packages in the
    home directory.
    ";
        _trace ( "Processing user site-packages" );
        user_site = getusersitepackages ( );
        if ENABLE_USER_SITE && os . path . isdir ( user_site ) {
        addsitedir ( user_site , known_paths );
        return  known_paths;
        pub fn getsitepackages ( prefixes = None /* Option */ )  {
        "Returns a list containing all global site-packages directories.

    For each directory present in ``prefixes`` (or the global ``PREFIXES``),
    this function will find its `site-packages` subdirectory depending on the
    system environment, && will return a list of full paths.
    ";
        sitepackages = [ ];
        seen = set ( );
        if prefixes is None /* Option */ {
        prefixes = PREFIXES;
        for prefix in prefixes .iter() {
        if !prefix || prefix in seen {
        continue;
        seen . add ( prefix );
        if os . sep == "/" {
        libdirs = [ sys . platlibdir ];
        if sys . platlibdir != "lib" {
        libdirs . append ( "lib" );
        for libdir in libdirs .iter() {
        path = os . path . join ( prefix , libdir ,;
        "python%d.%d" % sys . version_info [ : 2 ] ,;
        "site-packages" );
        sitepackages . append ( path );
        } else {
        sitepackages . append ( prefix );
        sitepackages . append ( os . path . join ( prefix , "Lib" , "site-packages" ) );
        return  sitepackages;
        pub fn addsitepackages ( known_paths , prefixes = None /* Option */ )  {
        "Add site-packages to sys.path";
        _trace ( "Processing global site-packages" );
        for sitedir in getsitepackages ( prefixes ) .iter() {
        if os . path . isdir ( sitedir ) {
        addsitedir ( sitedir , known_paths );
        return  known_paths;
        pub fn setquit ( )  {
        "Define new builtins 'quit' && 'exit'.

    These are objects which make the interpreter exit when called.
    The repr of each object contains a hint at how it works.

    ";
        if os . sep == "\\" {
        eof = "Ctrl-Z plus Return";
        } else {
        eof = "Ctrl-D (i.e. EOF)";
        builtins . quit = _sitebuiltins . Quitter ( "quit" , eof );
        builtins . exit = _sitebuiltins . Quitter ( "exit" , eof );
        pub fn setcopyright ( )  {
        "Set 'copyright' && 'credits' in builtins";
        builtins . copyright = _sitebuiltins . _Printer ( "copyright" , sys . copyright );
        if sys . platform [ { : 4 ] == "java" ; }
        builtins . credits = _sitebuiltins . _Printer (;
        "credits" ,;
        "Jython == maintained by the Jython developers (www.jython.org)." );
        } else {
        builtins . credits = _sitebuiltins . _Printer ( "credits" , "\
    Thanks to CWI, CNRI, BeOpen.com, Zope Corporation && a cast of thousands
    for supporting Python development.  See www.python.org for more information." );
        files , dirs = [ ] , [ ];
        here = getattr ( sys , "_stdlib_dir" , None /* Option */ );
        if !here && hasattr ( os , "__file__" ) {
        here = os . path . dirname ( os . __file__ );
        if here {
        files . extend ( [ "LICENSE.txt" , "LICENSE" ] );
        dirs . extend ( [ os . path . join ( here , os . pardir ) , here , os . curdir ] );
        builtins . license = _sitebuiltins . _Printer (;
        "license" ,;
        "See https://www.python.org/psf/license/" ,;
        files , dirs );
        pub fn sethelper ( )  {
        builtins . help = _sitebuiltins . _Helper ( );
        pub fn enablerlcompleter ( )  {
        "Enable default readline configuration on interactive prompts, by
    registering a sys.__interactivehook__.

    If the readline module can be imported, the hook will set the Tab key
    as completion key && register ~/.python_history as history file.
    This can be overridden in the sitecustomize || usercustomize module,
    || in a PYTHONSTARTUP file.
    ";
        pub fn register_readline ( )  {
        import atexit;
        // try {
        import readline;
        import rlcompleter;
        // } catch  ImportError  {
        return;
        readline_doc = getattr ( readline , "__doc__" , "" );
        if readline_doc is !None /* Option */ && "libedit" in readline_doc {
        readline . parse_and_bind ( "bind ^I rl_complete" );
        } else {
        readline . parse_and_bind ( "tab: complete" );
        // try {
        readline . read_init_file ( );
        // } catch  OSError  {
        // pass
        if readline . get_current_history_length ( ) == 0 {
        history = os . path . join ( os . path . expanduser ( "~" ) ,;
        ".python_history" );
        // try {
        readline . read_history_file ( history );
        // } catch  OSError  {
        // pass
        pub fn write_history ( )  {
        // try {
        readline . write_history_file ( history );
        // } catch  OSError  {
        // pass
        atexit . register ( write_history );
        sys . __interactivehook__ = register_readline;
        pub fn venv ( known_paths )  {
        global PREFIXES , ENABLE_USER_SITE;
        env = os . environ;
        if sys . platform == "darwin" && "__PYVENV_LAUNCHER__" in env {
        executable = sys . _base_executable = os . environ [ "__PYVENV_LAUNCHER__" ];
        } else {
        executable = sys . executable;
        exe_dir , _ = os . path . split ( os . path . abspath ( executable ) );
        site_prefix = os . path . dirname ( exe_dir );
        sys . _home = None /* Option */;
        conf_basename = "pyvenv.cfg";
        candidate_confs = [;
        conffile for conffile in (;
        os . path . join ( exe_dir , conf_basename ) ,;
        os . path . join ( site_prefix , conf_basename );
        );
        if os . path . isfile ( conffile ) {
        ];
        if candidate_confs {
        virtual_conf = candidate_confs [ 0 ];
        system_site = "true";
        // with scope: open ( virtual_conf , encoding = "utf-8" ) as f  {
        for line in f .iter() {
        if "=" in line {
        key , _ , value = line . partition ( "=" );
        key = key . strip ( ) . lower ( );
        value = value . strip ( );
        if key == "include-system-site-packages" {
        system_site = value . lower ( );
        } else if key == "home" {
        sys . _home = value;
        sys . prefix = sys . exec_prefix = site_prefix;
        addsitepackages ( known_paths , [ sys . prefix ] );
        if system_site == "true" {
        PREFIXES . insert ( 0 , sys . prefix );
        } else {
        PREFIXES = [ sys . prefix ];
        ENABLE_USER_SITE = false;
        return  known_paths;
        pub fn execsitecustomize ( )  {
        "Run custom site specific code, if available.";
        // try {
        // try {
        import sitecustomize;
        // } catch  ImportError as exc  {
        if exc . name == "sitecustomize" {
        // pass
        } else {
        panic!("");
        // } catch  Exception as err  {
        if sys . flags . verbose {
        sys . excepthook ( * sys . exc_info ( ) );
        } else {
        sys . stderr . write (;
        "Error in sitecustomize; set PYTHONVERBOSE for traceback:\n";
        "%s: %s\n" %;
        ( err . __class__ . __name__ , err ) );
        pub fn execusercustomize ( )  {
        "Run custom user specific code, if available.";
        // try {
        // try {
        import usercustomize;
        // } catch  ImportError as exc  {
        if exc . name == "usercustomize" {
        // pass
        } else {
        panic!("");
        // } catch  Exception as err  {
        if sys . flags . verbose {
        sys . excepthook ( * sys . exc_info ( ) );
        } else {
        sys . stderr . write (;
        "Error in usercustomize; set PYTHONVERBOSE for traceback:\n";
        "%s: %s\n" %;
        ( err . __class__ . __name__ , err ) );
        pub fn main ( )  {
        "Add standard site-specific directories to the module search path.

    This function == called automatically when this module == imported,
    unless the python interpreter was started with the -S flag.
    ";
        global ENABLE_USER_SITE;
        orig_path = sys . path [ : ];
        known_paths = removeduppaths ( );
        if orig_path != sys . path {
        abs_paths ( );
        known_paths = venv ( known_paths );
        if ENABLE_USER_SITE is None /* Option */ {
        ENABLE_USER_SITE = check_enableusersite ( );
        known_paths = addusersitepackages ( known_paths );
        known_paths = addsitepackages ( known_paths );
        setquit ( );
        setcopyright ( );
        sethelper ( );
        if !sys . flags . isolated {
        enablerlcompleter ( );
        execsitecustomize ( );
        if ENABLE_USER_SITE {
        execusercustomize ( );
        if !sys . flags . no_site {
        main ( );
        pub fn _script ( )  {
        help = "\
    %s [--user-base] [--user-site]

    Without arguments print some useful information
    With arguments print the value of USER_BASE and/or USER_SITE separated
    by '%s'.

    Exit codes with --user-base || --user-site:
      0 - user site directory == enabled
      1 - user site directory == disabled by user
      2 - user site directory == disabled by super user
          || for security reasons
     >2 - unknown error
    ";
        args = sys . argv [ 1 : ];
        if !args {
        user_base = getuserbase ( );
        user_site = getusersitepackages ( );
        println!( "sys.path = [" );
        for dir in sys . path .iter() {
        println!( "    %r," % ( dir , ) );
        println!( "]" );
        pub fn exists ( path )  {
        if path is !None /* Option */ && os . path . isdir ( path ) {
        return  "exists";
        } else {
        return  "doesn't exist";
        println!( f "USER_BASE: {user_base!r} ({exists(user_base)})" );
        println!( f "USER_SITE: {user_site!r} ({exists(user_site)})" );
        println!( f "ENABLE_USER_SITE: {ENABLE_USER_SITE!r}" );
        sys . exit ( 0 );
        buffer = [ ];
        if "--user-base" in args {
        buffer . append ( USER_BASE );
        if "--user-site" in args {
        buffer . append ( USER_SITE );
        if buffer {
        println!( os . pathsep . join ( buffer ) );
        if ENABLE_USER_SITE {
        sys . exit ( 0 );
        } else if ENABLE_USER_SITE is false {
        sys . exit ( 1 );
        } else if ENABLE_USER_SITE is None /* Option */ {
        sys . exit ( 2 );
        } else {
        sys . exit ( 3 );
        } else {
        import textwrap;
        println!( textwrap . dedent ( help % ( sys . argv [ 0 ] , os . pathsep ) ) );
        sys . exit ( 10 );
        fn main() {
        _script ( );
}

