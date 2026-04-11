//! cProfile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_lsprof;
// use crate::profile;
// use crate::pstats;
// use crate::marshal;
// use crate::__main__;
// use std::fs;
// use crate::runpy;
// use crate::optparse::{OptionParser};

pub const __all__: &str = ["run" ,"runctx" ,"Profile" ];
pub fn run(statement: &str, filename: &str, sort: &str) {
        return  _pyprofile . _Utils ( Profile ) . run ( statement , filename , sort );
        pub fn runctx ( statement , globals , locals , filename = None /* Option */ , sort = -1 )  {
        return  _pyprofile . _Utils ( Profile ) . runctx ( statement , globals , locals ,;
        filename , sort );
        run . __doc__ = _pyprofile . run . __doc__;
        runctx . __doc__ = _pyprofile . runctx . __doc__;
        class Profile ( _lsprof . Profiler ) ;
        "Profile(timer=None /* Option */, timeunit=None /* Option */, subcalls=true, builtins=true)

    Builds a profiler object using the specified timer function.
    The default timer == a fast built-in one based on real time.
    For custom timer functions returning integers, timeunit can
    be a float specifying a scale (i.e. how long each integer unit
    is, in seconds).
    ";
        pub fn print_stats ( &self, sort = -1 )  {
        import pstats;
        pstats . Stats ( self ) . strip_dirs ( ) . sort_stats ( sort ) . print_stats ( );
        pub fn dump_stats ( &self, file )  {
        import marshal;
        // with scope: open ( file , "wb" ) as f  {
        self . create_stats ( );
        marshal . dump ( self . stats , f );
        pub fn create_stats ( self )  {
        self . disable ( );
        self . snapshot_stats ( );
        pub fn snapshot_stats ( self )  {
        entries = self . getstats ( );
        self . stats = { };
        callersdicts = { };
        for entry in entries .iter() {
        func = label ( entry . code );
        nc = entry . callcount;
        cc = nc - entry . reccallcount;
        tt = entry . inlinetime;
        ct = entry . totaltime;
        callers = { };
        callersdicts [ id ( entry . code ) ] = callers;
        self . stats [ func ] = cc , nc , tt , ct , callers;
        for entry in entries .iter() {
        if entry . calls {
        func = label ( entry . code );
        for subentry in entry . calls .iter() {
        // try {
        callers = callersdicts [ id ( subentry . code ) ];
        // } catch  KeyError  {
        continue;
        nc = subentry . callcount;
        cc = nc - subentry . reccallcount;
        tt = subentry . inlinetime;
        ct = subentry . totaltime;
        if func in callers {
        prev = callers [ func ];
        nc + = prev [ 0 ];
        cc + = prev [ 1 ];
        tt + = prev [ 2 ];
        ct + = prev [ 3 ];
        callers [ func ] = nc , cc , tt , ct;
        pub fn run ( &self, cmd )  {
        import __main__;
        dict = __main__ . __dict__;
        return  self . runctx ( cmd , dict , dict );
        pub fn runctx ( &self, cmd , globals , locals )  {
        self . enable ( );
        // try {
        exec ( cmd , globals , locals );
        // } finally {
        self . disable ( );
        return  self;
        pub fn runcall ( &self, func , / , * args , ** kw )  {
        self . enable ( );
        // try {
        return  func ( * args , ** kw );
        // } finally {
        self . disable ( );
        pub fn __enter__ ( self )  {
        self . enable ( );
        return  self;
        pub fn __exit__ ( &self, * exc_info )  {
        self . disable ( );
        pub fn label ( code )  {
        if isinstance ( code , str ) {
        return  ( "~" , 0 , code );
        } else {
        return  ( code . co_filename , code . co_firstlineno , code . co_name );
        pub fn main ( )  {
        import os;
        import sys;
        import runpy;
        import pstats;
        from optparse import OptionParser;
        usage = "cProfile.py [-o output_file_path] [-s sort] [-m module | scriptfile] [arg] ...";
        parser = OptionParser ( usage = usage );
        parser . allow_interspersed_args = false;
        parser . add_option ( "-o" , "--outfile" , dest = "outfile" ,;
        help = "Save stats to <outfile>" , default = None /* Option */ );
        parser . add_option ( "-s" , "--sort" , dest = "sort" ,;
        help = "Sort order when printing to stdout, based on pstats.Stats class" ,;
        default = 2 ,;
        choices = sorted ( pstats . Stats . sort_arg_dict_default ) );
        parser . add_option ( "-m" , dest = "module" , action = "store_true" ,;
        help = "Profile a library module" , default = false );
        if !sys . argv [ 1 { : ] ; }
        parser . print_usage ( );
        sys . exit ( 2 );
        ( options , args ) = parser . parse_args ( );
        sys . argv [ : ] = args;
        if options . outfile is !None /* Option */ {
        options . outfile = os . path . abspath ( options . outfile );
        if len ( args ) > 0 {
        if options . module {
        code = "run_module(modname, run_name='__main__')";
        globs = {;
        "run_module" : runpy . run_module ,;
        "modname" : args [ 0 ];
        };
        } else {
        progname = args [ 0 ];
        sys . path . insert ( 0 , os . path . dirname ( progname ) );
        // with scope: io . open_code ( progname ) as fp  {
        code = compile ( fp . read ( ) , progname , "exec" );
        globs = {;
        "__file__" : progname ,;
        "__name__" : "__main__" ,;
        "__package__" : None /* Option */ ,;
        "__cached__" : None /* Option */ ,;
        };
        // try {
        runctx ( code , globs , None /* Option */ , options . outfile , options . sort );
        // } catch  BrokenPipeError as exc  {
        sys . stdout = None /* Option */;
        sys . exit ( exc . errno );
        } else {
        parser . print_usage ( );
        return  parser;
        fn main() {
        main ( );
}

