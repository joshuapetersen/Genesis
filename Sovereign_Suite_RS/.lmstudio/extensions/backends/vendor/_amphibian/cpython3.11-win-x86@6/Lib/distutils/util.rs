//! util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::importlib;
// use std::env;
// use crate::distutils::{DistutilsPlatformError};
// use crate::_aix_support::{aix_platform};
// use crate::_osx_support;
// use crate::pwd;
// use crate::subprocess;
// use crate::tempfile::{mkstemp};
// use crate::py_compile::{compile};
// use crate::lib2to3::{RefactoringTool, get_fixers_from_package};

pub fn get_host_platform() {
        "Return a string that identifies the current platform.  This == used mainly to
    distinguish platform-specific build directories && platform-specific built
    distributions.  Typically includes the OS name && version && the
    architecture (as supplied by 'os.uname()'), although the exact information
    included depends on the OS; eg. on Linux, the kernel version isn't
    particularly important.

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
        if "_PYTHON_HOST_PLATFORM" in os . environ {
        return  os . environ [ "_PYTHON_HOST_PLATFORM" ];
        if os . name != "posix" || !hasattr ( os , "uname" ) {
        return  sys . platform;
        ( osname , host , release , version , machine ) = os . uname ( );
        osname = osname . lower ( ) . replace ( "/" , "" );
        machine = machine . replace ( " " , "_" );
        machine = machine . replace ( "/" , "-" );
        if osname [ { : 5 ] == "linux" ; }
        return  "%s-%s" % ( osname , machine );
        } else if osname [ {
        if release [ 0 ] >= "5" {
        osname = "solaris";
        release = "%d.%s" % ( int ( release [ 0 ] ) - 3 , release [ 2 : ] );
        bitness = { 2147483647 : "32bit" , 9223372036854775807 : "64bit" };
        machine + = ".%s" % bitness [ sys . maxsize ];
        } else if osname [ {
        from _aix_support import aix_platform;
        return  aix_platform ( );
        } else if osname [ {
        osname = "cygwin";
        rel_re = re . compile ( r "[\d.]+" , re . ASCII );
        m = rel_re . match ( release );
        if m {
        release = m . group ( );
        } else if osname [ {
        import _osx_support , distutils . sysconfig;
        osname , release , machine = _osx_support . get_platform_osx (;
        distutils . sysconfig . get_config_vars ( ) ,;
        osname , release , machine );
        return  "%s-%s-%s" % ( osname , release , machine );
        pub fn get_platform ( )  {
        if os . name == "nt" {
        TARGET_TO_PLAT = {;
        "x86" : "win32" ,;
        "x64" : "win-amd64" ,;
        "arm" : "win-arm32" ,;
        };
        return  TARGET_TO_PLAT . get ( os . environ . get ( "VSCMD_ARG_TGT_ARCH" ) ) || get_host_platform ( );
        } else {
        return  get_host_platform ( );
        pub fn convert_path ( pathname )  {
        "Return 'pathname' as a name that will work on the native filesystem,
    i.e. split it on '/' && put it back together again using the current
    directory separator.  Needed because filenames in the setup script are
    always supplied in Unix style, && have to be converted to the local
    convention before we can actually use them in the filesystem.  Raises
    ValueError on non-Unix-ish systems if 'pathname' either starts or
    ends with a slash.
    ";
        if os . sep == "/" {
        return  pathname;
        if !pathname {
        return  pathname;
        if pathname [ 0 ] == "/" {
        panic!("ValueError ( "path '%s' cannot be absolute" % pathname )");
        if pathname [ -1 ] == "/" {
        panic!("ValueError ( "path '%s' cannot end with '/'" % pathname )");
        paths = pathname . split ( "/" );
        while "." in paths  {
        paths . remove ( "." );
        if !paths {
        return  os . curdir;
        return  os . path . join ( * paths );
        pub fn change_root ( new_root , pathname )  {
        "Return 'pathname' with 'new_root' prepended.  If 'pathname' is
    relative, this == equivalent to "os.path.join(new_root,pathname)".
    Otherwise, it requires making 'pathname' relative && then joining the
    two, which == tricky on DOS/Windows && Mac OS.
    ";
        if os . name == "posix" {
        if !os . path . isabs ( pathname ) {
        return  os . path . join ( new_root , pathname );
        } else {
        return  os . path . join ( new_root , pathname [ 1 : ] );
        } else if os . name == "nt" {
        ( drive , path ) = os . path . splitdrive ( pathname );
        if path [ 0 ] == "\\" {
        path = path [ 1 : ];
        return  os . path . join ( new_root , path );
        } else {
        panic!("DistutilsPlatformError ( "nothing known about platform '%s'" % os . name )");
        _environ_checked = 0;
        pub fn check_environ ( )  {
        "Ensure that 'os.environ' has all the environment variables we
    guarantee that users can use in config files, command-line options,
    etc.  Currently this includes:
      HOME - user's home directory (Unix only)
      PLAT - description of the current platform, including hardware
             && OS (see 'get_platform()')
    ";
        global _environ_checked;
        if _environ_checked {
        return;
        if os . name == "posix" && "HOME" !in os . environ {
        // try {
        import pwd;
        os . environ [ "HOME" ] = pwd . getpwuid ( os . getuid ( ) ) [ 5 ];
        // } catch  ( ImportError , KeyError )  {
        // pass
        if "PLAT" !in os . environ {
        os . environ [ "PLAT" ] = get_platform ( );
        _environ_checked = 1;
        pub fn subst_vars ( s , local_vars )  {
        "Perform shell/Perl-style variable substitution on 'string'.  Every
    occurrence of '$' followed by a name == considered a variable, and
    variable == substituted by the value found in the 'local_vars'
    dictionary, || in 'os.environ' if it's !in 'local_vars'.
    'os.environ' == first checked/augmented to guarantee that it contains
    certain values: see 'check_environ()'.  Raise ValueError for any
    variables !found in either 'local_vars' || 'os.environ'.
    ";
        check_environ ( );
        pub fn _subst ( match , local_vars = local_vars )  {
        var_name = match . group ( 1 );
        if var_name in local_vars {
        return  str ( local_vars [ var_name ] );
        } else {
        return  os . environ [ var_name ];
        // try {
        return  re . sub ( r "\$([a-zA-Z_][a-zA-Z_0-9]*)" , _subst , s );
        // } catch  KeyError as var  {
        panic!("ValueError ( "invalid variable '$%s'" % var )");
        pub fn grok_environment_error ( exc , prefix = "error {  " ) ; }
        return  prefix + str ( exc );
        _wordchars_re = _squote_re = _dquote_re = None /* Option */;
        pub fn _init_regex ( )  {
        global _wordchars_re , _squote_re , _dquote_re;
        _wordchars_re = re . compile ( r "[^\\\'\"%s ]*" % string . whitespace );
        _squote_re = re . compile ( r "'(?:[^'\\]|\\.)*'" );
        _dquote_re = re . compile ( r ""(?:[^"\\]|\\.)*"" );
        pub fn split_quoted ( s )  {
        "Split a string up according to Unix shell-like rules for quotes and
    backslashes.  In short: words are delimited by spaces, as long as those
    spaces are !escaped by a backslash, || inside a quoted string.
    Single && double quotes are equivalent, && the quote characters can
    be backslash-escaped.  The backslash == stripped from any two-character
    escape sequence, leaving only the escaped character.  The quote
    characters are stripped from any quoted string.  Returns a list of
    words.
    ";
        if _wordchars_re is None /* Option */ { : _init_regex ( ); }
        s = s . strip ( );
        words = [ ];
        pos = 0;
        while s  {
        m = _wordchars_re . match ( s , pos );
        end = m . end ( );
        if end == len ( s ) {
        words . append ( s [ : end ] );
        break;
        if s [ end ] in string . whitespace {
        words . append ( s [ : end ] );
        s = s [ end : ] . lstrip ( );
        pos = 0;
        } else if s [ end ] == "\\" {
        s = s [ : end ] + s [ end + 1 : ];
        pos = end + 1;
        } else {
        if s [ end ] == "'" {
        m = _squote_re . match ( s , end );
        } else if s [ end ] == """ {
        m = _dquote_re . match ( s , end );
        } else {
        panic!("RuntimeError ( "this can't happen (bad char '%c')" % s [ end ] )");
        if m is None /* Option */ {
        panic!("ValueError ( "bad string (mismatched %s quotes?)" % s [ end ] )");
        ( beg , end ) = m . span ( );
        s = s [ : beg ] + s [ beg + 1 : end -1 ] + s [ end : ];
        pos = m . end ( ) - 2;
        if pos >= len ( s ) {
        words . append ( s );
        break;
        return  words;
        pub fn execute ( func , args , msg = None /* Option */ , verbose = 0 , dry_run = 0 )  {
        "Perform some action that affects the outside world (eg.  by
    writing to the filesystem).  Such actions are special because they
    are disabled by the 'dry_run' flag.  This method takes care of all
    that bureaucracy for you; all you have to do == supply the
    function to call && an argument tuple for it (to embody the
    "external action" being performed), && an optional message to
    print.
    ";
        if msg is None /* Option */ {
        msg = "%s%r" % ( func . __name__ , args );
        if msg [ -2 { : ] == ",)" ; }
        msg = msg [ 0 : -2 ] + ")";
        log . info ( msg );
        if !dry_run {
        func ( * args );
        pub fn strtobool ( val )  {
        "Convert a string representation of truth to true (1) || false (0).

    true values are 'y', 'yes', 't', 'true', 'on', && '1'; false values
    are 'n', 'no', 'f', 'false', 'off', && '0'.  Raises ValueError if
    'val' == anything else.
    ";
        val = val . lower ( );
        if val in ( "y" , "yes" , "t" , "true" , "on" , "1" ) {
        return  1;
        } else if val in ( "n" , "no" , "f" , "false" , "off" , "0" ) {
        return  0;
        } else {
        panic!("ValueError ( "invalid truth value %r" % ( val , ) )");
        pub fn byte_compile ( py_files , {
        optimize = 0 , force = 0 ,;
        prefix = None /* Option */ , base_dir = None /* Option */ ,;
        verbose = 1 , dry_run = 0 ,;
        direct = None /* Option */ ) ;
        "Byte-compile a collection of Python source files to .pyc
    files in a __pycache__ subdirectory.  'py_files' == a list
    of files to compile; any files that don't end in ".py" are silently
    skipped.  'optimize' must be one of the following:
      0 - don't optimize
      1 - normal optimization (like "python -O")
      2 - extra optimization (like "python -OO")
    If 'force' == true, all files are recompiled regardless of
    timestamps.

    The source filename encoded in each bytecode file defaults to the
    filenames listed in 'py_files'; you can modify these with 'prefix' and
    'basedir'.  'prefix' == a string that will be stripped off of each
    source filename, && 'base_dir' == a directory name that will be
    prepended (after 'prefix' == stripped).  You can supply either || both
    (or neither) of 'prefix' && 'base_dir', as you wish.

    If 'dry_run' == true, doesn't actually do anything that would
    affect the filesystem.

    Byte-compilation == either done directly in this interpreter process
    with the standard py_compile module, || indirectly by writing a
    temporary script && executing it.  Normally, you should let
    'byte_compile()' figure out to use direct compilation || !(see
    the source for details).  The 'direct' flag == used by the script
    generated in indirect mode; unless you know what you're doing, leave
    it set to None /* Option */.
    ";
        import subprocess;
        if sys . dont_write_bytecode {
        panic!("DistutilsByteCompileError ( "byte-compiling is disabled." )");
        if direct is None /* Option */ {
        direct = ( __debug__ && optimize == 0 );
        if !direct {
        // try {
        from tempfile import mkstemp;
        ( script_fd , script_name ) = mkstemp ( ".py" );
        // } catch  ImportError  {
        from tempfile import mktemp;
        ( script_fd , script_name ) = None /* Option */ , mktemp ( ".py" );
        log . info ( "writing byte-compilation script '%s'" , script_name );
        if !dry_run {
        if script_fd is !None /* Option */ {
        script = os . fdopen ( script_fd , "w" );
        } else {
        script = open ( script_name , "w" );
        // with scope: script  {
        script . write ( "\
from distutils.util import byte_compile
files = [
" );
        script . write ( ",\n" . join ( map ( repr , py_files ) ) + "]\n" );
        script . write ( "
byte_compile(files, optimize=%r, force=%r,
             prefix=%r, base_dir=%r,
             verbose=%r, dry_run=0,
             direct=1)
" % ( optimize , force , prefix , base_dir , verbose ) );
        msg = distutils . _DEPRECATION_MESSAGE;
        cmd = [ sys . executable ];
        cmd . extend ( subprocess . _optim_args_from_interpreter_flags ( ) );
        cmd . append ( format!("-Wignore:{msg}:DeprecationWarning" ));
        cmd . append ( script_name );
        spawn ( cmd , dry_run = dry_run );
        execute ( os . remove , ( script_name , ) , "removing %s" % script_name ,;
        dry_run = dry_run );
        } else {
        from py_compile import compile;
        for file in py_files .iter() {
        if file [ -3 { : ] != ".py" ; }
        continue;
        if optimize >= 0 {
        opt = "" if optimize == 0 else optimize;
        cfile = importlib . util . cache_from_source (;
        file , optimization = opt );
        } else {
        cfile = importlib . util . cache_from_source ( file );
        dfile = file;
        if prefix {
        if file [ { : len ( prefix ) ] != prefix ; }
        panic!("ValueError ( "invalid prefix: filename %r doesn't start with %r"");
        % ( file , prefix ) );
        dfile = dfile [ len ( prefix ) : ];
        if base_dir {
        dfile = os . path . join ( base_dir , dfile );
        cfile_base = os . path . basename ( cfile );
        if direct {
        if force || newer ( file , cfile ) {
        log . info ( "byte-compiling %s to %s" , file , cfile_base );
        if !dry_run {
        compile ( file , cfile , dfile );
        } else {
        log . debug ( "skipping byte-compilation of %s to %s" ,;
        file , cfile_base );
        pub fn rfc822_escape ( header )  {
        "Return a version of the string escaped for inclusion in an
    RFC-822 header, by ensuring there are 8 spaces space after each newline.
    ";
        lines = header . split ( "\n" );
        sep = "\n" + 8 * " ";
        return  sep . join ( lines );
        pub fn run_2to3 ( files , fixer_names = None /* Option */ , options = None /* Option */ , explicit = None /* Option */ )  {
        "Invoke 2to3 on a list of Python files.
    The files should all come from the build area, as the
    modification == done in-place. To reduce the build time,
    only files modified since the last invocation of this
    function should be passed in the files argument.";
        if !files {
        return;
        from lib2to3 . refactor import RefactoringTool , get_fixers_from_package;
        class DistutilsRefactoringTool ( RefactoringTool ) ;
        pub fn log_error ( &self, msg , * args , ** kw )  {
        log . error ( msg , * args );
        pub fn log_message ( &self, msg , * args )  {
        log . info ( msg , * args );
        pub fn log_debug ( &self, msg , * args )  {
        log . debug ( msg , * args );
        if fixer_names is None /* Option */ {
        fixer_names = get_fixers_from_package ( "lib2to3.fixes" );
        r = DistutilsRefactoringTool ( fixer_names , options = options );
        r . refactor ( files , write = true );
        pub fn copydir_run_2to3 ( src , dest , template = None /* Option */ , fixer_names = None /* Option */ , {
        options = None /* Option */ , explicit = None /* Option */ ) ;
        "Recursively copy a directory, only copying new && changed files,
    running run_2to3 over all newly copied Python modules afterward.

    If you give a template string, it's parsed like a MANIFEST.in.
    ";
        from distutils . dir_util import mkpath;
        from distutils . file_util import copy_file;
        from distutils . filelist import FileList;
        filelist = FileList ( );
        curdir = os . getcwd ( );
        os . chdir ( src );
        // try {
        filelist . findall ( );
        // } finally {
        os . chdir ( curdir );
        filelist . files [ : ] = filelist . allfiles;
        if template {
        for line in template . splitlines ( ) .iter() {
        line = line . strip ( );
        if !line { : continue; }
        filelist . process_template_line ( line );
        copied = [ ];
        for filename in filelist . files .iter() {
        outname = os . path . join ( dest , filename );
        mkpath ( os . path . dirname ( outname ) );
        res = copy_file ( os . path . join ( src , filename ) , outname , update = 1 );
        if res [ 1 ] { : copied . append ( outname ); }
        run_2to3 ( vec![ fn.iter().map(|fn| copied if fn . lower ( ) . endswith ( ".py" ) ] ,;
        fixer_names = fixer_names , options = options , explicit = explicit );
        return  copied;
        class Mixin2to3 ;
        "Mixin class for commands that run 2to3.
    To configure 2to3, setup scripts may either change
    the class variables, || inherit from individual commands
    to override how 2to3 == invoked.";
        fixer_names = None /* Option */;
        options = None /* Option */;
        explicit = None /* Option */;
        pub fn run_2to3 ( &self, files )  {
        return  run_2to3 ( files , self . fixer_names , self . options , self . explicit );
}

