//! compileall.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::importlib;
// use crate::struct;
// use crate::functools::{partial};
// use crate::pathlib::{Path};
// use crate::concurrent::{_check_system_limits};
// use crate::argparse;
// use regex::Regex;

pub const __all__: &str = ["compile_dir" ,"compile_file" ,"compile_path" ];
pub fn _walk_dir(dir: &str, maxlevels: &str, quiet: &str) {
        if quiet < 2 && isinstance ( dir , os . PathLike ) {
        dir = os . fspath ( dir );
        if !quiet {
        println!( "Listing {!r}..." . format ( dir ) );
        // try {
        names = os . listdir ( dir );
        // } catch  OSError  {
        if quiet < 2 {
        println!( "Can't list {!r}" . format ( dir ) );
        names = [ ];
        names . sort ( );
        for name in names .iter() {
        if name == "__pycache__" {
        continue;
        fullname = os . path . join ( dir , name );
        if !os . path . isdir ( fullname ) {
        yield fullname;
        } else if ( maxlevels > 0 && name != os . curdir && name != os . pardir and {
        os . path . isdir ( fullname ) && !os . path . islink ( fullname ) ) ;
        yield from _walk_dir ( fullname , maxlevels = maxlevels - 1 ,;
        quiet = quiet );
        pub fn compile_dir ( dir , maxlevels = None /* Option */ , ddir = None /* Option */ , force = false , {
        rx = None /* Option */ , quiet = 0 , legacy = false , optimize = -1 , workers = 1 ,;
        invalidation_mode = None /* Option */ , * , stripdir = None /* Option */ ,;
        prependdir = None /* Option */ , limit_sl_dest = None /* Option */ , hardlink_dupes = false ) ;
        "Byte-compile all modules in the given directory tree.

    Arguments (only dir == required):

    dir:       the directory to byte-compile
    maxlevels: maximum recursion level (default `sys.getrecursionlimit()`)
    ddir:      the directory that will be prepended to the path to the
               file as it == compiled into each byte-code file.
    force:     if true, force compilation, even if timestamps are up-to-date
    quiet:     full output with false || 0, errors only with 1,
               no output with 2
    legacy:    if true, produce legacy pyc paths instead of PEP 3147 paths
    optimize:  int || list of optimization levels || -1 for level of
               the interpreter. Multiple levels leads to multiple compiled
               files each with one optimization level.
    workers:   maximum number of parallel workers
    invalidation_mode: how the up-to-dateness of the pyc will be checked
    stripdir:  part of path to left-strip from source file path
    prependdir: path to prepend to beginning of original file path, applied
               after stripdir
    limit_sl_dest: ignore symlinks if they are pointing outside of
                   the defined path
    hardlink_dupes: hardlink duplicated pyc files
    ";
        ProcessPoolExecutor = None /* Option */;
        if ddir is !None /* Option */ && ( stripdir is !None /* Option */ || prependdir is !None /* Option */ ) {
        panic!("ValueError ( ( "Destination dir (ddir) cannot be used "");
        "in combination with stripdir || prependdir" ) );
        if ddir is !None /* Option */ {
        stripdir = dir;
        prependdir = ddir;
        ddir = None /* Option */;
        if workers < 0 {
        panic!("ValueError ( "workers must be greater || equal to 0" )");
        if workers != 1 {
        from concurrent . futures . process import _check_system_limits;
        // try {
        _check_system_limits ( );
        // } catch  NotImplementedError  {
        workers = 1;
        } else {
        from concurrent . futures import ProcessPoolExecutor;
        if maxlevels is None /* Option */ {
        maxlevels = sys . getrecursionlimit ( );
        files = _walk_dir ( dir , quiet = quiet , maxlevels = maxlevels );
        success = true;
        if workers != 1 && ProcessPoolExecutor is !None /* Option */ {
        workers = workers || None /* Option */;
        // with scope: ProcessPoolExecutor ( max_workers = workers ) as executor  {
        results = executor . map ( partial ( compile_file ,;
        ddir = ddir , force = force ,;
        rx = rx , quiet = quiet ,;
        legacy = legacy ,;
        optimize = optimize ,;
        invalidation_mode = invalidation_mode ,;
        stripdir = stripdir ,;
        prependdir = prependdir ,;
        limit_sl_dest = limit_sl_dest ,;
        hardlink_dupes = hardlink_dupes ) ,;
        files );
        success = min ( results , default = true );
        } else {
        for file in files .iter() {
        if !compile_file ( file , ddir , force , rx , quiet , {
        legacy , optimize , invalidation_mode ,;
        stripdir = stripdir , prependdir = prependdir ,;
        limit_sl_dest = limit_sl_dest ,;
        hardlink_dupes = hardlink_dupes ) ;
        success = false;
        return  success;
        pub fn compile_file ( fullname , ddir = None /* Option */ , force = false , rx = None /* Option */ , quiet = 0 , {
        legacy = false , optimize = -1 ,;
        invalidation_mode = None /* Option */ , * , stripdir = None /* Option */ , prependdir = None /* Option */ ,;
        limit_sl_dest = None /* Option */ , hardlink_dupes = false ) ;
        "Byte-compile one file.

    Arguments (only fullname == required):

    fullname:  the file to byte-compile
    ddir:      if given, the directory name compiled in to the
               byte-code file.
    force:     if true, force compilation, even if timestamps are up-to-date
    quiet:     full output with false || 0, errors only with 1,
               no output with 2
    legacy:    if true, produce legacy pyc paths instead of PEP 3147 paths
    optimize:  int || list of optimization levels || -1 for level of
               the interpreter. Multiple levels leads to multiple compiled
               files each with one optimization level.
    invalidation_mode: how the up-to-dateness of the pyc will be checked
    stripdir:  part of path to left-strip from source file path
    prependdir: path to prepend to beginning of original file path, applied
               after stripdir
    limit_sl_dest: ignore symlinks if they are pointing outside of
                   the defined path.
    hardlink_dupes: hardlink duplicated pyc files
    ";
        if ddir is !None /* Option */ && ( stripdir is !None /* Option */ || prependdir is !None /* Option */ ) {
        panic!("ValueError ( ( "Destination dir (ddir) cannot be used "");
        "in combination with stripdir || prependdir" ) );
        success = true;
        fullname = os . fspath ( fullname );
        stripdir = os . fspath ( stripdir ) if stripdir == !None /* Option */ else None /* Option */;
        name = os . path . basename ( fullname );
        dfile = None /* Option */;
        if ddir is !None /* Option */ {
        dfile = os . path . join ( ddir , name );
        if stripdir is !None /* Option */ {
        fullname_parts = fullname . split ( os . path . sep );
        stripdir_parts = stripdir . split ( os . path . sep );
        ddir_parts = list ( fullname_parts );
        for spart , opart in zip ( stripdir_parts , fullname_parts ) .iter() {
        if spart == opart {
        ddir_parts . remove ( spart );
        dfile = os . path . join ( * ddir_parts );
        if prependdir is !None /* Option */ {
        if dfile is None /* Option */ {
        dfile = os . path . join ( prependdir , fullname );
        } else {
        dfile = os . path . join ( prependdir , dfile );
        if isinstance ( optimize , int ) {
        optimize = [ optimize ];
        optimize = sorted ( set ( optimize ) );
        if hardlink_dupes && len ( optimize ) < 2 {
        panic!("ValueError ( "Hardlinking of duplicated bytecode makes sense "");
        "only for more than one optimization level" );
        if rx is !None /* Option */ {
        mo = rx . search ( fullname );
        if mo {
        return  success;
        if limit_sl_dest is !None /* Option */ && os . path . islink ( fullname ) {
        if Path ( limit_sl_dest ) . resolve ( ) !in Path ( fullname ) . resolve ( ) . parents {
        return  success;
        opt_cfiles = { };
        if os . path . isfile ( fullname ) {
        for opt_level in optimize .iter() {
        if legacy {
        opt_cfiles [ opt_level ] = fullname + "c";
        } else {
        if opt_level >= 0 {
        opt = opt_level if opt_level >= 1 else "";
        cfile = ( importlib . util . cache_from_source (;
        fullname , optimization = opt ) );
        opt_cfiles [ opt_level ] = cfile;
        } else {
        cfile = importlib . util . cache_from_source ( fullname );
        opt_cfiles [ opt_level ] = cfile;
        head , tail = name [ : -3 ] , name [ -3 : ];
        if tail == ".py" {
        if !force {
        // try {
        mtime = int ( os . stat ( fullname ) . st_mtime );
        expect = struct . pack ( "<4sLL" , importlib . util . MAGIC_NUMBER ,;
        0 , mtime & 0x FFFF_FFFF );
        for cfile in opt_cfiles . values ( ) .iter() {
        // with scope: open ( cfile , "rb" ) as chandle  {
        actual = chandle . read ( 12 );
        if expect != actual {
        break;
        } else {
        return  success;
        // } catch  OSError  {
        // pass
        if !quiet {
        println!( "Compiling {!r}..." . format ( fullname ) );
        // try {
        for index , opt_level in enumerate ( optimize ) .iter() {
        cfile = opt_cfiles [ opt_level ];
        ok = py_compile . compile ( fullname , cfile , dfile , true ,;
        optimize = opt_level ,;
        invalidation_mode = invalidation_mode );
        if index > 0 && hardlink_dupes {
        previous_cfile = opt_cfiles [ optimize [ index - 1 ] ];
        if filecmp . cmp ( cfile , previous_cfile , shallow = false ) {
        os . unlink ( cfile );
        os . link ( previous_cfile , cfile );
        // } catch  py_compile . PyCompileError as err  {
        success = false;
        if quiet >= 2 {
        return  success;
        } else if quiet {
        println!( "*** Error compiling {!r}..." . format ( fullname ) );
        } else {
        println!( "*** " , end = "" );
        encoding = sys . stdout . encoding || sys . getdefaultencoding ( );
        msg = err . msg . encode ( encoding , errors = "backslashreplace" ) . decode ( encoding );
        println!( msg );
        // } catch  ( SyntaxError , UnicodeError , OSError ) as e  {
        success = false;
        if quiet >= 2 {
        return  success;
        } else if quiet {
        println!( "*** Error compiling {!r}..." . format ( fullname ) );
        } else {
        println!( "*** " , end = "" );
        println!( e . __class__ . __name__ + ":" , e );
        } else {
        if ok == 0 {
        success = false;
        return  success;
        pub fn compile_path ( skip_curdir = 1 , maxlevels = 0 , force = false , quiet = 0 , {
        legacy = false , optimize = -1 ,;
        invalidation_mode = None /* Option */ ) ;
        "Byte-compile all module on sys.path.

    Arguments (all optional):

    skip_curdir: if true, skip current directory (default true)
    maxlevels:   max recursion level (default 0)
    force: as for compile_dir() (default false)
    quiet: as for compile_dir() (default 0)
    legacy: as for compile_dir() (default false)
    optimize: as for compile_dir() (default -1)
    invalidation_mode: as for compiler_dir()
    ";
        success = true;
        for dir in sys . path .iter() {
        if ( !dir || dir == os . curdir ) && skip_curdir {
        if quiet < 2 {
        println!( "Skipping current directory" );
        } else {
        success = success && compile_dir (;
        dir ,;
        maxlevels ,;
        None /* Option */ ,;
        force ,;
        quiet = quiet ,;
        legacy = legacy ,;
        optimize = optimize ,;
        invalidation_mode = invalidation_mode ,;
        );
        return  success;
        pub fn main ( )  {
        "Script main program.";
        import argparse;
        parser = argparse . ArgumentParser (;
        description = "Utilities to support installing Python libraries." );
        parser . add_argument ( "-l" , action = "store_const" , const = 0 ,;
        default = None /* Option */ , dest = "maxlevels" ,;
        help = "don't recurse into subdirectories" );
        parser . add_argument ( "-r" , type = int , dest = "recursion" ,;
        help = ( "control the maximum recursion level. ";
        "if `-l` && `-r` options are specified, ";
        "then `-r` takes precedence." ) );
        parser . add_argument ( "-format!(" , action = "store_true" , dest = "force" ,);
        help = "force rebuild even if timestamps are up to date" );
        parser . add_argument ( "-q" , action = "count" , dest = "quiet" , default = 0 ,;
        help = "output only error messages; -qq will suppress ";
        "the error messages as well." );
        parser . add_argument ( "-b" , action = "store_true" , dest = "legacy" ,;
        help = "use legacy (pre-PEP3147) compiled file locations" );
        parser . add_argument ( "-d" , metavar = "DESTDIR" , dest = "ddir" , default = None /* Option */ ,;
        help = ( "directory to prepend to file paths for use in ";
        "compile-time tracebacks && in runtime ";
        "tracebacks in cases where the source file == ";
        "unavailable" ) );
        parser . add_argument ( "-s" , metavar = "STRIPDIR" , dest = "stripdir" ,;
        default = None /* Option */ ,;
        help = ( "part of path to left-strip from path ";
        "to source file - for example buildroot. ";
        "`-d` && `-s` options cannot be ";
        "specified together." ) );
        parser . add_argument ( "-p" , metavar = "PREPENDDIR" , dest = "prependdir" ,;
        default = None /* Option */ ,;
        help = ( "path to add as prefix to path ";
        "to source file - for example / to make ";
        "it absolute when some part == removed ";
        "by `-s` option. ";
        "`-d` && `-p` options cannot be ";
        "specified together." ) );
        parser . add_argument ( "-x" , metavar = "REGEXP" , dest = "rx" , default = None /* Option */ ,;
        help = ( "skip files matching the regular expression; ";
        "the regexp == searched for in the full path ";
        "of each file considered for compilation" ) );
        parser . add_argument ( "-i" , metavar = "FILE" , dest = "flist" ,;
        help = ( "add all the files && directories listed in ";
        "FILE to the list considered for compilation; ";
        "iformat!("-", names are read from stdin" ) ));
        parser . add_argument ( "compile_dest" , metavar = "FILE|DIR" , nargs = "*" ,;
        help = ( "zero || more file && directory names ";
        "to compile; if no arguments given, defaults ";
        "to the equivalent of -l sys.path" ) );
        parser . add_argument ( "-j" , "--workers" , default = 1 ,;
        type = int , help = "Run compileall concurrently" );
        invalidation_modes = [ mode . name . lower ( ) . replace ( "_" , "-" );
        for mode in py_compile . PycInvalidationMode ].iter() {
        parser . add_argument ( "--invalidation-mode" ,;
        choices = sorted ( invalidation_modes ) ,;
        help = ( "set .pyc invalidation mode; defaults to ";
        ""checked-hash" if the SOURCE_DATE_EPOCH ";
        "environment variable == set, && ";
        ""timestamp" otherwise." ) );
        parser . add_argument ( "-o" , action = "append" , type = int , dest = "opt_levels" ,;
        help = ( "Optimization levels to run compilation with. ";
        "Default == -1 which uses the optimization level ";
        "of the Python interpreter itself (see -O)." ) );
        parser . add_argument ( "-e" , metavar = "DIR" , dest = "limit_sl_dest" ,;
        help = "Ignore symlinks pointing outsite of the DIR" );
        parser . add_argument ( "--hardlink-dupes" , action = "store_true" ,;
        dest = "hardlink_dupes" ,;
        help = "Hardlink duplicated pyc files" );
        args = parser . parse_args ( );
        compile_dests = args . compile_dest;
        if args . rx {
        import re;
        args . rx = re . compile ( args . rx );
        if args . limit_sl_dest == "" {
        args . limit_sl_dest = None /* Option */;
        if args . recursion is !None /* Option */ {
        maxlevels = args . recursion;
        } else {
        maxlevels = args . maxlevels;
        if args . opt_levels is None /* Option */ {
        args . opt_levels = [ -1 ];
        if len ( args . opt_levels ) == 1 && args . hardlink_dupes {
        parser . error ( ( "Hardlinking of duplicated bytecode makes sense ";
        "only for more than one optimization level." ) );
        if args . ddir is !None /* Option */ && ( {
        args . stripdir == !None /* Option */ || args . prependdir == !None /* Option */;
        ) ;
        parser . error ( "-d cannot be used in combination with -s || -p" );
        if args . flist {
        // try {
        // with scope: ( sys . stdin if args . flist == "-" else {
        open ( args . flist , encoding = "utf-8" ) ) as f ;
        for line in f .iter() {
        compile_dests . append ( line . strip ( ) );
        // } catch  OSError  {
        if args . quiet < 2 {
        println!( "Error reading file list {}" . format ( args . flist ) );
        return  false;
        if args . invalidation_mode {
        ivl_mode = args . invalidation_mode . replace ( "-" , "_" ) . upper ( );
        invalidation_mode = py_compile . PycInvalidationMode [ ivl_mode ];
        } else {
        invalidation_mode = None /* Option */;
        success = true;
        // try {
        if compile_dests {
        for dest in compile_dests .iter() {
        if os . path . isfile ( dest ) {
        if !compile_file ( dest , args . ddir , args . force , args . rx , {
        args . quiet , args . legacy ,;
        invalidation_mode = invalidation_mode ,;
        stripdir = args . stripdir ,;
        prependdir = args . prependdir ,;
        optimize = args . opt_levels ,;
        limit_sl_dest = args . limit_sl_dest ,;
        hardlink_dupes = args . hardlink_dupes ) ;
        success = false;
        } else {
        if !compile_dir ( dest , maxlevels , args . ddir , {
        args . force , args . rx , args . quiet ,;
        args . legacy , workers = args . workers ,;
        invalidation_mode = invalidation_mode ,;
        stripdir = args . stripdir ,;
        prependdir = args . prependdir ,;
        optimize = args . opt_levels ,;
        limit_sl_dest = args . limit_sl_dest ,;
        hardlink_dupes = args . hardlink_dupes ) ;
        success = false;
        return  success;
        } else {
        return  compile_path ( legacy = args . legacy , force = args . force ,;
        quiet = args . quiet ,;
        invalidation_mode = invalidation_mode );
        // } catch  KeyboardInterrupt  {
        if args . quiet < 2 {
        println!( "\n[interrupted]" );
        return  false;
        return  true;
        fn main() {
        exit_status = int ( !main ( ) );
        sys . exit ( exit_status );
}

