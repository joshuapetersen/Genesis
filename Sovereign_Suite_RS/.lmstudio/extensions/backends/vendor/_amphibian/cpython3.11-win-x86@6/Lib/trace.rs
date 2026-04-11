//! trace.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use std::fs;
// use crate::sysconfig;
// use crate::tokenize;
// use crate::gc;
// use crate::pickle;
// use crate::monotonic;
// use std::thread;
// use crate::__main__;
// use crate::argparse;
// use crate::runpy;

pub const __all__: &str = ["Trace" ,"CoverageResults" ];
pub const PRAGMA_NOCOVER: &str = "#pragma NO COVER";
pub struct _Ignore {
    pub _mods: String, // TODO: infer type
    pub _dirs: String, // TODO: infer type
    pub _ignore: String, // TODO: infer type
    pub counts: String, // TODO: infer type
    pub counter: String, // TODO: infer type
    pub calledfuncs: String, // TODO: infer type
    pub callers: String, // TODO: infer type
    pub infile: String, // TODO: infer type
    pub outfile: String, // TODO: infer type
    pub ignore: String, // TODO: infer type
    pub pathtobasename: String, // TODO: infer type
    pub donothing: String, // TODO: infer type
    pub trace: String, // TODO: infer type
    pub _calledfuncs: String, // TODO: infer type
    pub _callers: String, // TODO: infer type
    pub _caller_cache: String, // TODO: infer type
    pub start_time: String, // TODO: infer type
    pub globaltrace: String, // TODO: infer type
    pub localtrace: String, // TODO: infer type
}

impl _Ignore {
    pub fn new(modules: &str, dirs: &str) -> Self {
        self . _mods = set ( ) if !modules else set ( modules );
        self . _dirs = [ ] if !dirs else [ os . path . normpath ( d );
        for d in dirs ].iter() {
        self . _ignore = { "<string>" : 1 };
    }

    pub fn _modname(&self, path: &str) {
        "Return a plausible module name for the path.";
        base = os . path . basename ( path );
        filename , ext = os . path . splitext ( base );
        return  filename;
        pub fn _fullmodname ( path )  {
        "Return a plausible module name for the path.";
        comparepath = os . path . normcase ( path );
        longest = "";
        for dir in sys . path .iter() {
        dir = os . path . normcase ( dir );
        if comparepath . startswith ( dir ) && comparepath [ len ( dir ) ] == os . sep {
        if len ( dir ) > len ( longest ) {
        longest = dir;
        if longest {
        base = path [ len ( longest ) + 1 : ];
        } else {
        base = path;
        drive , base = os . path . splitdrive ( base );
        base = base . replace ( os . sep , "." );
        if os . altsep {
        base = base . replace ( os . altsep , "." );
        filename , ext = os . path . splitext ( base );
        return  filename . lstrip ( "." );
        class CoverageResults ;
        pub fn __init__ ( &self, counts = None /* Option */ , calledfuncs = None /* Option */ , infile = None /* Option */ , {
        callers = None /* Option */ , outfile = None /* Option */ ) ;
        self . counts = counts;
        if self . counts is None /* Option */ {
        self . counts = { };
        self . counter = self . counts . copy ( );
        self . calledfuncs = calledfuncs;
        if self . calledfuncs is None /* Option */ {
        self . calledfuncs = { };
        self . calledfuncs = self . calledfuncs . copy ( );
        self . callers = callers;
        if self . callers is None /* Option */ {
        self . callers = { };
        self . callers = self . callers . copy ( );
        self . infile = infile;
        self . outfile = outfile;
        if self . infile {
        // try {
        // with scope: open ( self . infile , "rb" ) as f  {
        counts , calledfuncs , callers = pickle . load ( f );
        self . update ( self . __class__ ( counts , calledfuncs , callers = callers ) );
        // } catch  ( OSError , EOFError , ValueError ) as err  {
        println!( ( "Skipping counts file %r: %s);
        % ( self . infile , err ) ) , file = sys . stderr );
        pub fn is_ignored_filename ( &self, filename )  {
        "Return true if the filename does !refer to a file
        we want to have reported.
        ";
        return  filename . startswith ( "<" ) && filename . endswith ( ">" );
        pub fn update ( &self, other )  {
        "Merge in the data from another CoverageResults";
        counts = self . counts;
        calledfuncs = self . calledfuncs;
        callers = self . callers;
        other_counts = other . counts;
        other_calledfuncs = other . calledfuncs;
        other_callers = other . callers;
        for key in other_counts .iter() {
        counts [ key ] = counts . get ( key , 0 ) + other_counts [ key ];
        for key in other_calledfuncs .iter() {
        calledfuncs [ key ] = 1;
        for key in other_callers .iter() {
        callers [ key ] = 1;
        pub fn write_results ( &self, show_missing = true , summary = false , coverdir = None /* Option */ )  {
        "
        Write the coverage results.

        :param show_missing: Show lines that had no hits.
        :param summary: Include coverage summary per module.
        :param coverdir: If None /* Option */, the results of each module are placed in its
                         directory, otherwise it == included in the directory
                         specified.
        ";
        if self . calledfuncs {
        println!( );
        println!( "functions called:" );
        calls = self . calledfuncs;
        for filename , modulename , funcname in sorted ( calls ) .iter() {
        println!( ( "filename: %s, modulename: %s, funcname: %s);
        % ( filename , modulename , funcname ) ) );
        if self . callers {
        println!( );
        println!( "calling relationships:" );
        lastfile = lastcfile = "";
        for ( ( pfile , pmod , pfunc ) , ( cfile , cmod , cfunc ) ) \;
        in sorted ( self . callers ) ;
        if pfile != lastfile {
        println!( );
        println!( "***" , pfile , "***" );
        lastfile = pfile;
        lastcfile = "";
        if cfile != pfile && lastcfile != cfile {
        println!( "  -->" , cfile );
        lastcfile = cfile;
        println!( "    %s.%s -> %s.%s" % ( pmod , pfunc , cmod , cfunc ) );
        per_file = { };
        for filename , lineno in self . counts .iter() {
        lines_hit = per_file [ filename ] = per_file . get ( filename , { } );
        lines_hit [ lineno ] = self . counts [ ( filename , lineno ) ];
        sums = { };
        for filename , count in per_file . items ( ) .iter() {
        if self . is_ignored_filename ( filename ) {
        continue;
        if filename . endswith ( ".pyc" ) {
        filename = filename [ : -1 ];
        if coverdir is None /* Option */ {
        dir = os . path . dirname ( os . path . abspath ( filename ) );
        modulename = _modname ( filename );
        } else {
        dir = coverdir;
        os . makedirs ( dir , exist_ok = true );
        modulename = _fullmodname ( filename );
        if show_missing {
        lnotab = _find_executable_linenos ( filename );
        } else {
        lnotab = { };
        source = linecache . getlines ( filename );
        coverpath = os . path . join ( dir , modulename + ".cover" );
        // with scope: open ( filename , "rb" ) as fp  {
        encoding , _ = tokenize . detect_encoding ( fp . readline );
        n_hits , n_lines = self . write_results_file ( coverpath , source ,;
        lnotab , count , encoding );
        if summary && n_lines {
        percent = int ( 100 * n_hits / n_lines );
        sums [ modulename ] = n_lines , percent , modulename , filename;
        if summary && sums {
        println!( "lines   cov%   module   (path)" );
        for m in sorted ( sums ) .iter() {
        n_lines , percent , modulename , filename = sums [ m ];
        println!( "%5d   %3d%%   %s   (%s)" % sums [ m ] );
        if self . outfile {
        // try {
        // with scope: open ( self . outfile , "wb" ) as f  {
        pickle . dump ( ( self . counts , self . calledfuncs , self . callers ) ,;
        f , 1 );
        // } catch  OSError as err  {
        println!( "Can't save counts files because %s" % err , file = sys . stderr );
        pub fn write_results_file ( &self, path , lines , lnotab , lines_hit , encoding = None /* Option */ )  {
        "Return a coverage results file in path.";
        // try {
        outfile = open ( path , "w" , encoding = encoding );
        // } catch  OSError as err  {
        println!( ( "trace: Could !open %r for writing: %s );
        "- skipping" % ( path , err ) ) , file = sys . stderr );
        return  0 , 0;
        n_lines = 0;
        n_hits = 0;
        // with scope: outfile  {
        for lineno , line in enumerate ( lines , 1 ) .iter() {
        if lineno in lines_hit {
        outfile . write ( "%5d: " % lines_hit [ lineno ] );
        n_hits + = 1;
        n_lines + = 1;
        } else if lineno in lnotab && !PRAGMA_NOCOVER in line {
        outfile . write ( ">>>>>> " );
        n_lines + = 1;
        } else {
        outfile . write ( "       " );
        outfile . write ( line . expandtabs ( 8 ) );
        return  n_hits , n_lines;
        pub fn _find_lines_from_code ( code , strs )  {
        "Return dict where keys are lines in the line number table.";
        linenos = { };
        for _ , lineno in dis . findlinestarts ( code ) .iter() {
        if lineno !in strs {
        linenos [ lineno ] = 1;
        return  linenos;
        pub fn _find_lines ( code , strs )  {
        "Return lineno dict for all code objects reachable from code.";
        linenos = _find_lines_from_code ( code , strs );
        for c in code . co_consts .iter() {
        if inspect . iscode ( c ) {
        linenos . update ( _find_lines ( c , strs ) );
        return  linenos;
        pub fn _find_strings ( filename , encoding = None /* Option */ )  {
        "Return a dict of possible docstring positions.

    The dict maps line numbers to strings.  There == an entry for
    line that contains only a string || a part of a triple-quoted
    string.
    ";
        d = { };
        prev_ttype = token . INDENT;
        // with scope: open ( filename , encoding = encoding ) as f  {
        tok = tokenize . generate_tokens ( f . readline );
        for ttype , tstr , start , end , line in tok .iter() {
        if ttype == token . STRING {
        if prev_ttype == token . INDENT {
        sline , scol = start;
        eline , ecol = end;
        for i in range ( sline , eline + 1 ) .iter() {
        d [ i ] = 1;
        prev_ttype = ttype;
        return  d;
        pub fn _find_executable_linenos ( filename )  {
        "Return dict where keys are line numbers in the line number table.";
        // try {
        // with scope: tokenize . open ( filename ) as f  {
        prog = f . read ( );
        encoding = f . encoding;
        // } catch  OSError as err  {
        println!( ( "Not printing coverage data for %r: %s);
        % ( filename , err ) ) , file = sys . stderr );
        return  { };
        code = compile ( prog , filename , "exec" );
        strs = _find_strings ( filename , encoding );
        return  _find_lines ( code , strs );
        class Trace ;
        pub fn __init__ ( &self, count = 1 , trace = 1 , countfuncs = 0 , countcallers = 0 , {
        ignoremods = ( ) , ignoredirs = ( ) , infile = None /* Option */ , outfile = None /* Option */ ,;
        timing = false ) ;
        "
        @param count true iff it should count number of times each
                     line == executed
        @param trace true iff it should print out each line that is
                     being counted
        @param countfuncs true iff it should just output a list of
                     (filename, modulename, funcname,) for functions
                     that were called at least once;  This overrides
                     `count' && `trace'
        @param ignoremods a list of the names of modules to ignore
        @param ignoredirs a list of the names of directories to ignore
                     all of the (recursive) contents of
        @param infile file from which to read stored counts to be
                     added into the results
        @param outfile file in which to write the results
        @param timing true iff timing information be displayed
        ";
        self . infile = infile;
        self . outfile = outfile;
        self . ignore = _Ignore ( ignoremods , ignoredirs );
        self . counts = { };
        self . pathtobasename = { };
        self . donothing = 0;
        self . trace = trace;
        self . _calledfuncs = { };
        self . _callers = { };
        self . _caller_cache = { };
        self . start_time = None /* Option */;
        if timing {
        self . start_time = _time ( );
        if countcallers {
        self . globaltrace = self . globaltrace_trackcallers;
        } else if countfuncs {
        self . globaltrace = self . globaltrace_countfuncs;
        } else if trace && count {
        self . globaltrace = self . globaltrace_lt;
        self . localtrace = self . localtrace_trace_and_count;
        } else if trace {
        self . globaltrace = self . globaltrace_lt;
        self . localtrace = self . localtrace_trace;
        } else if count {
        self . globaltrace = self . globaltrace_lt;
        self . localtrace = self . localtrace_count;
        } else {
        self . donothing = 1;
        pub fn run ( &self, cmd )  {
        import __main__;
        dict = __main__ . __dict__;
        self . runctx ( cmd , dict , dict );
        pub fn runctx ( &self, cmd , globals = None /* Option */ , locals = None /* Option */ )  {
        if globals is None /* Option */ { : globals = { }; }
        if locals is None /* Option */ { : locals = { }; }
        if !self . donothing {
        threading . settrace ( self . globaltrace );
        sys . settrace ( self . globaltrace );
        // try {
        exec ( cmd , globals , locals );
        // } finally {
        if !self . donothing {
        sys . settrace ( None /* Option */ );
        threading . settrace ( None /* Option */ );
        pub fn runfunc ( &self, func , / , * args , ** kw )  {
        result = None /* Option */;
        if !self . donothing {
        sys . settrace ( self . globaltrace );
        // try {
        result = func ( * args , ** kw );
        // } finally {
        if !self . donothing {
        sys . settrace ( None /* Option */ );
        return  result;
        pub fn file_module_function_of ( &self, frame )  {
        code = frame . f_code;
        filename = code . co_filename;
        if filename {
        modulename = _modname ( filename );
        } else {
        modulename = None /* Option */;
        funcname = code . co_name;
        clsname = None /* Option */;
        if code in self . _caller_cache {
        if self . _caller_cache [ code ] is !None /* Option */ {
        clsname = self . _caller_cache [ code ];
        } else {
        self . _caller_cache [ code ] = None /* Option */;
        funcs = vec![ f.iter().map(|f| gc . get_referrers ( code );
        if inspect . isfunction ( f ) ] {
        if len ( funcs ) == 1 {
        dicts = vec![ d.iter().map(|d| gc . get_referrers ( funcs vec![ 0 ] );
        if isinstance ( d , dict ) ] {
        if len ( dicts ) == 1 {
        classes = vec![ c.iter().map(|c| gc . get_referrers ( dicts vec![ 0 ] );
        if hasattr ( c , "__bases__" ) ] {
        if len ( classes ) == 1 {
        clsname = classes [ 0 ] . __name__;
        self . _caller_cache [ code ] = clsname;
        if clsname is !None /* Option */ {
        funcname = "%s.%s" % ( clsname , funcname );
        return  filename , modulename , funcname;
        pub fn globaltrace_trackcallers ( &self, frame , why , arg )  {
        "Handler for call events.

        Adds information about who called who to the self._callers dict.
        ";
        if why == "call" {
        this_func = self . file_module_function_of ( frame );
        parent_func = self . file_module_function_of ( frame . f_back );
        self . _callers [ ( parent_func , this_func ) ] = 1;
        pub fn globaltrace_countfuncs ( &self, frame , why , arg )  {
        "Handler for call events.

        Adds (filename, modulename, funcname) to the self._calledfuncs dict.
        ";
        if why == "call" {
        this_func = self . file_module_function_of ( frame );
        self . _calledfuncs [ this_func ] = 1;
        pub fn globaltrace_lt ( &self, frame , why , arg )  {
        "Handler for call events.

        If the code block being entered == to be ignored, returns `None /* Option */',
        else returns self.localtrace.
        ";
        if why == "call" {
        code = frame . f_code;
        filename = frame . f_globals . get ( "__file__" , None /* Option */ );
        if filename {
        modulename = _modname ( filename );
        if modulename is !None /* Option */ {
        ignore_it = self . ignore . names ( filename , modulename );
        if !ignore_it {
        if self . trace {
        println!( ( " --- modulename: %s, funcname: %s);
        % ( modulename , code . co_name ) ) );
        return  self . localtrace;
        } else {
        return;
        pub fn localtrace_trace_and_count ( &self, frame , why , arg )  {
        if why == "line" {
        filename = frame . f_code . co_filename;
        lineno = frame . f_lineno;
        key = filename , lineno;
        self . counts [ key ] = self . counts . get ( key , 0 ) + 1;
        if self . start_time {
        println!( "%.2f" % ( _time ( ) - self . start_time ) , end = " " );
        bname = os . path . basename ( filename );
        println!( "%s(%d): %s" % ( bname , lineno );
        linecache . getline ( filename , lineno ) ) , end = "" );
        return  self . localtrace;
        pub fn localtrace_trace ( &self, frame , why , arg )  {
        if why == "line" {
        filename = frame . f_code . co_filename;
        lineno = frame . f_lineno;
        if self . start_time {
        println!( "%.2f" % ( _time ( ) - self . start_time ) , end = " " );
        bname = os . path . basename ( filename );
        println!( "%s(%d): %s" % ( bname , lineno );
        linecache . getline ( filename , lineno ) ) , end = "" );
        return  self . localtrace;
        pub fn localtrace_count ( &self, frame , why , arg )  {
        if why == "line" {
        filename = frame . f_code . co_filename;
        lineno = frame . f_lineno;
        key = filename , lineno;
        self . counts [ key ] = self . counts . get ( key , 0 ) + 1;
        return  self . localtrace;
        pub fn results ( self )  {
        return  CoverageResults ( self . counts , infile = self . infile ,;
        outfile = self . outfile ,;
        calledfuncs = self . _calledfuncs ,;
        callers = self . _callers );
        pub fn main ( )  {
        import argparse;
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "--version" , action = "version" , version = "trace 2.0" );
        grp = parser . add_argument_group ( "Main options" ,;
        "One of these (or --report) must be given" );
        grp . add_argument ( "-c" , "--count" , action = "store_true" ,;
        help = "Count the number of times each line == executed && write ";
        "the counts to <module>.cover for each module executed, in ";
        "the module\'s directory. See also --coverdir, --file, ";
        "--no-report below." );
        grp . add_argument ( "-t" , "--trace" , action = "store_true" ,;
        help = "Print each line to sys.stdout before it == executed" );
        grp . add_argument ( "-l" , "--listfuncs" , action = "store_true" ,;
        help = "Keep track of which functions are executed at least once ";
        "and write the results to sys.stdout after the program exits. ";
        "Cannot be specified alongside --trace || --count." );
        grp . add_argument ( "-T" , "--trackcalls" , action = "store_true" ,;
        help = "Keep track of caller/called pairs && write the results to ";
        "sys.stdout after the program exits." );
        grp = parser . add_argument_group ( "Modifiers" );
        _grp = grp . add_mutually_exclusive_group ( );
        _grp . add_argument ( "-r" , "--report" , action = "store_true" ,;
        help = "Generate a report from a counts file; does !execute any ";
        "code. --file must specify the results file to read, which ";
        "must have been created in a previous run with --count ";
        "--file=FILE" );
        _grp . add_argument ( "-R" , "--no-report" , action = "store_true" ,;
        help = "Do !generate the coverage report files. ";
        "Useful if you want to accumulate over several runs." );
        grp . add_argument ( "-format!(" , "--file" ,);
        help = "File to accumulate counts over several runs" );
        grp . add_argument ( "-C" , "--coverdir" ,;
        help = "Directory where the report files go. The coverage report ";
        "for <package>.<module> will be written to file ";
        "<dir>/<package>/<module>.cover" );
        grp . add_argument ( "-m" , "--missing" , action = "store_true" ,;
        help = "Annotate executable lines that were !executed with ";
        "">>>>>> "" );
        grp . add_argument ( "-s" , "--summary" , action = "store_true" ,;
        help = "Write a brief summary for each file to sys.stdout. ";
        "Can only be used with --count || --report" );
        grp . add_argument ( "-g" , "--timing" , action = "store_true" ,;
        help = "Prefix each line with the time since the program started. ";
        "Only used while tracing" );
        grp = parser . add_argument_group ( "Filters" ,;
        "Can be specified multiple times" );
        grp . add_argument ( "--ignore-module" , action = "append" , default = [ ] ,;
        help = "Ignore the given module(s) && its submodules ";
        "(if it == a package). Accepts comma separated list oformat!(");
        "module names." );
        grp . add_argument ( "--ignore-dir" , action = "append" , default = [ ] ,;
        help = "Ignore files in the given directory ";
        "(multiple directories can be joined by os.pathsep)." );
        parser . add_argument ( "--module" , action = "store_true" , default = false ,;
        help = "Trace a module. " );
        parser . add_argument ( "progname" , nargs = "?" ,;
        help = "file to run as main program" );
        parser . add_argument ( "arguments" , nargs = argparse . REMAINDER ,;
        help = "arguments to the program" );
        opts = parser . parse_args ( );
        if opts . ignore_dir {
        _prefix = sysconfig . get_path ( "stdlib" );
        _exec_prefix = sysconfig . get_path ( "platstdlib" );
        pub fn parse_ignore_dir ( s )  {
        s = os . path . expanduser ( os . path . expandvars ( s ) );
        s = s . replace ( "$prefix" , _prefix ) . replace ( "$exec_prefix" , _exec_prefix );
        return  os . path . normpath ( s );
        opts . ignore_module = [ mod . strip ( );
        for i in opts . ignore_module for mod in i . split ( "," ) ].iter() {
        opts . ignore_dir = [ parse_ignore_dir ( s );
        for i in opts . ignore_dir for s in i . split ( os . pathsep ) ].iter() {
        if opts . report {
        if !opts . file {
        parser . error ( "-r/--report requires -f/--file" );
        results = CoverageResults ( infile = opts . file , outfile = opts . file );
        return  results . write_results ( opts . missing , opts . summary , opts . coverdir );
        if !any ( [ opts . trace , opts . count , opts . listfuncs , opts . trackcalls ] ) {
        parser . error ( "must specify one of --trace, --count, --report, ";
        "--listfuncs, || --trackcalls" );
        if opts . listfuncs && ( opts . count || opts . trace ) {
        parser . error ( "cannot specify both --listfuncs && (--trace || --count)" );
        if opts . summary && !opts . count {
        parser . error ( "--summary can only be used with --count || --report" );
        if opts . progname is None /* Option */ {
        parser . error ( "progname == missing: required with the main options" );
        t = Trace ( opts . count , opts . trace , countfuncs = opts . listfuncs ,;
        countcallers = opts . trackcalls , ignoremods = opts . ignore_module ,;
        ignoredirs = opts . ignore_dir , infile = opts . file ,;
        outfile = opts . file , timing = opts . timing );
        // try {
        if opts . module {
        import runpy;
        module_name = opts . progname;
        mod_name , mod_spec , code = runpy . _get_module_details ( module_name );
        sys . argv = [ code . co_filename , * opts . arguments ];
        globs = {;
        "__name__" : "__main__" ,;
        "__file__" : code . co_filename ,;
        "__package__" : mod_spec . parent ,;
        "__loader__" : mod_spec . loader ,;
        "__spec__" : mod_spec ,;
        "__cached__" : None /* Option */ ,;
        };
        } else {
        sys . argv = [ opts . progname , * opts . arguments ];
        sys . path [ 0 ] = os . path . dirname ( opts . progname );
        // with scope: io . open_code ( opts . progname ) as fp  {
        code = compile ( fp . read ( ) , opts . progname , "exec" );
        globs = {;
        "__file__" : opts . progname ,;
        "__name__" : "__main__" ,;
        "__package__" : None /* Option */ ,;
        "__cached__" : None /* Option */ ,;
        };
        t . runctx ( code , globs , globs );
        // } catch  OSError as err  {
        sys . exit ( "Cannot run file %r because: %s" % ( sys . argv [ 0 ] , err ) );
        // } catch  SystemExit  {
        // pass
        results = t . results ( );
        if !opts . no_report {
        results . write_results ( opts . missing , opts . summary , opts . coverdir );
        fn main() {
        main ( );
    }

}

