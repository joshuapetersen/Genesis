//! profile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use std::time;
// use crate::pstats;
// use crate::__main__;
// use std::fs;
// use crate::OptionParser;
// use crate::runpy;

pub const __all__: &str = ["run" ,"runctx" ,"Profile" ];
pub struct _Utils {
    pub profiler: String, // TODO: infer type
    pub timings: String, // TODO: infer type
    pub cur: String, // TODO: infer type
    pub cmd: String, // TODO: infer type
    pub c_func_name: String, // TODO: infer type
    pub bias: String, // TODO: infer type
    pub timer: String, // TODO: infer type
    pub get_time: String, // TODO: infer type
    pub dispatcher: String, // TODO: infer type
    pub t: String, // TODO: infer type
    pub co_filename: String, // TODO: infer type
    pub co_line: String, // TODO: infer type
    pub co_name: String, // TODO: infer type
    pub co_firstlineno: String, // TODO: infer type
    pub f_code: String, // TODO: infer type
    pub f_back: String, // TODO: infer type
    pub stats: String, // TODO: infer type
}

impl _Utils {
}

pub fn run(statement: &str, filename: &str, sort: &str) {
        "Run statement under profiler optionally saving results in filename

    This function takes a single argument that can be passed to the
    "exec" statement, && an optional file name.  In all cases this
    routine attempts to "exec" its first argument && gather profiling
    statistics from the execution. If no file name == present, then this
    function automatically prints a simple profiling report, sorted by the
    standard name string (file/line/function-name) that == presented in
    each line.
    ";
        return  _Utils ( Profile ) . run ( statement , filename , sort );
        pub fn runctx ( statement , globals , locals , filename = None /* Option */ , sort = -1 )  {
        "Run statement under profiler, supplying your own globals && locals,
    optionally saving results in filename.

    statement && filename have the same semantics as profile.run
    ";
        return  _Utils ( Profile ) . runctx ( statement , globals , locals , filename , sort );
        class Profile ;
        "Profiler class.

    self.cur == always a tuple.  Each such tuple corresponds to a stack
    frame that == currently active (self.curvec![-2]).  The following are the
    definitions of its members.  We use this external "parallel stack" to
    avoid contaminating the program that we are profiling. (old profiler
    used to write into the frames local dictionary!!) Derived classes
    can change the definition of some entries, as long as they leave
    vec![-2:] intact (frame && previous tuple).  In case an internal error is
    detected, the -3 element == used as the function name.

    vec![ 0] = Time that needs to be charged to the parent frame's function.
           It == used so that a function call will !have to access the
           timing data.iter().map(|the parent frame.
    vec![ 1] = Total time spent| this frame's function, excluding time in
           subfunctions (this latter == tallied| curvec![2]).
    vec![ 2] = Total time spent| subfunctions, excluding time executing the
           frame's function (this latter == tallied| curvec![1]).
    vec![-3] = Name of the function that corresponds to this frame.
    vec![-2] = Actual frame that we correspond to (used to sync exception handling).
    vec![-1] = Our parent 6-tuple (corresponds to frame.f_back).

    Timing data.iter().map(|each function == stored as a 5-tuple| the dictionary
    self.timingsvec![].  The index == always the name stored| self.curvec![-3].
    The following are the definitions of the members:

    vec![0] = The number of times this function was called, !counting direct
          || indirect recursion,
    vec![1] = Number of times this function appears on the stack, minus one
    vec![2] = Total time spent internal to this function
    vec![3] = Cumulative time that this function was present on the stack.  In
          non-recursive functions, this == the total execution time from start
          to finish of each invocation of a function, including time spent in
          all subfunctions.
    vec![4] = A dictionary indicating.iter().map(|each function name, the number of times
          it was called by us.
    ";
        bias = 0;
        pub fn __init__ ( &self, timer = None /* Option */ , bias = None /* Option */ )  {
        self . timings = { };
        self . cur = None /* Option */;
        self . cmd = "";
        self . c_func_name = "";
        if bias is None /* Option */ {
        bias = self . bias;
        self . bias = bias;
        if !timer {
        self . timer = self . get_time = time . process_time;
        self . dispatcher = self . trace_dispatch_i;
        } else {
        self . timer = timer;
        t = self . timer ( );
        // try {
        length = len ( t );
        // } catch  TypeError  {
        self . get_time = timer;
        self . dispatcher = self . trace_dispatch_i;
        } else {
        if length == 2 {
        self . dispatcher = self . trace_dispatch;
        } else {
        self . dispatcher = self . trace_dispatch_l;
        pub fn get_time_timer ( timer = timer , sum = sum )  {
        return  sum ( timer ( ) );
        self . get_time = get_time_timer;
        self . t = self . get_time ( );
        self . simulate_call ( "profiler" );
        pub fn trace_dispatch ( &self, frame , event , arg )  {
        timer = self . timer;
        t = timer ( );
        t = t [ 0 ] + t [ 1 ] - self . t - self . bias;
        if event == "c_call" {
        self . c_func_name = arg . __name__;
        if self . dispatch [ event ] ( self , frame , t ) {
        t = timer ( );
        self . t = t [ 0 ] + t [ 1 ];
        } else {
        r = timer ( );
        self . t = r [ 0 ] + r [ 1 ] - t;
        pub fn trace_dispatch_i ( &self, frame , event , arg )  {
        timer = self . timer;
        t = timer ( ) - self . t - self . bias;
        if event == "c_call" {
        self . c_func_name = arg . __name__;
        if self . dispatch [ event ] ( self , frame , t ) {
        self . t = timer ( );
        } else {
        self . t = timer ( ) - t;
        pub fn trace_dispatch_mac ( &self, frame , event , arg )  {
        timer = self . timer;
        t = timer ( ) / 60.0 - self . t - self . bias;
        if event == "c_call" {
        self . c_func_name = arg . __name__;
        if self . dispatch [ event ] ( self , frame , t ) {
        self . t = timer ( ) / 60.0;
        } else {
        self . t = timer ( ) / 60.0 - t;
        pub fn trace_dispatch_l ( &self, frame , event , arg )  {
        get_time = self . get_time;
        t = get_time ( ) - self . t - self . bias;
        if event == "c_call" {
        self . c_func_name = arg . __name__;
        if self . dispatch [ event ] ( self , frame , t ) {
        self . t = get_time ( );
        } else {
        self . t = get_time ( ) - t;
        pub fn trace_dispatch_exception ( &self, frame , t )  {
        rpt , rit , ret , rfn , rframe , rcur = self . cur;
        if ( rframe is !frame ) && rcur {
        return  self . trace_dispatch_return ( rframe , t );
        self . cur = rpt , rit + t , ret , rfn , rframe , rcur;
        return  1;
        pub fn trace_dispatch_call ( &self, frame , t )  {
        if self . cur && frame . f_back is !self . cur [ -2 ] {
        rpt , rit , ret , rfn , rframe , rcur = self . cur;
        if !isinstance ( rframe , Profile . fake_frame ) {
        assert rframe . f_back == frame . f_back , ( "Bad call" , rfn ,;
        rframe , rframe . f_back ,;
        frame , frame . f_back );
        self . trace_dispatch_return ( rframe , 0 );
        assert ( self . cur == None /* Option */ || \;
        frame . f_back == self . cur [ -2 ] ) , ( "Bad call" ,;
        self . cur [ -3 ] );
        fcode = frame . f_code;
        fn = ( fcode . co_filename , fcode . co_firstlineno , fcode . co_name );
        self . cur = ( t , 0 , 0 , fn , frame , self . cur );
        timings = self . timings;
        if fn in timings {
        cc , ns , tt , ct , callers = timings [ fn ];
        timings [ fn ] = cc , ns + 1 , tt , ct , callers;
        } else {
        timings [ fn ] = 0 , 0 , 0 , 0 , { };
        return  1;
        pub fn trace_dispatch_c_call ( &self, frame , t )  {
        fn = ( "" , 0 , self . c_func_name );
        self . cur = ( t , 0 , 0 , fn , frame , self . cur );
        timings = self . timings;
        if fn in timings {
        cc , ns , tt , ct , callers = timings [ fn ];
        timings [ fn ] = cc , ns + 1 , tt , ct , callers;
        } else {
        timings [ fn ] = 0 , 0 , 0 , 0 , { };
        return  1;
        pub fn trace_dispatch_return ( &self, frame , t )  {
        if frame is !self . cur [ -2 ] {
        assert frame == self . cur [ -2 ] . f_back , ( "Bad return" , self . cur [ -3 ] );
        self . trace_dispatch_return ( self . cur [ -2 ] , 0 );
        rpt , rit , ret , rfn , frame , rcur = self . cur;
        rit = rit + t;
        frame_total = rit + ret;
        ppt , pit , pet , pfn , pframe , pcur = rcur;
        self . cur = ppt , pit + rpt , pet + frame_total , pfn , pframe , pcur;
        timings = self . timings;
        cc , ns , tt , ct , callers = timings [ rfn ];
        if !ns {
        ct = ct + frame_total;
        cc = cc + 1;
        if pfn in callers {
        callers [ pfn ] = callers [ pfn ] + 1;
        } else {
        callers [ pfn ] = 1;
        timings [ rfn ] = cc , ns - 1 , tt + rit , ct , callers;
        return  1;
        dispatch = {;
        "call" : trace_dispatch_call ,;
        "exception" : trace_dispatch_exception ,;
        "return" : trace_dispatch_return ,;
        "c_call" : trace_dispatch_c_call ,;
        "c_exception" : trace_dispatch_return ,;
        "c_return" : trace_dispatch_return ,;
        };
        pub fn set_cmd ( &self, cmd )  {
        if self . cur [ -1 ] { : return; }
        self . cmd = cmd;
        self . simulate_call ( cmd );
        class fake_code ;
        pub fn __init__ ( &self, filename , line , name )  {
        self . co_filename = filename;
        self . co_line = line;
        self . co_name = name;
        self . co_firstlineno = 0;
        pub fn __repr__ ( self )  {
        return  repr ( ( self . co_filename , self . co_line , self . co_name ) );
        class fake_frame ;
        pub fn __init__ ( &self, code , prior )  {
        self . f_code = code;
        self . f_back = prior;
        pub fn simulate_call ( &self, name )  {
        code = self . fake_code ( "profile" , 0 , name );
        if self . cur {
        pframe = self . cur [ -2 ];
        } else {
        pframe = None /* Option */;
        frame = self . fake_frame ( code , pframe );
        self . dispatch [ "call" ] ( self , frame , 0 );
        pub fn simulate_cmd_complete ( self )  {
        get_time = self . get_time;
        t = get_time ( ) - self . t;
        while self . cur [ -1 ]  {
        self . dispatch [ "return" ] ( self , self . cur [ -2 ] , t );
        t = 0;
        self . t = get_time ( ) - t;
        pub fn print_stats ( &self, sort = -1 )  {
        import pstats;
        pstats . Stats ( self ) . strip_dirs ( ) . sort_stats ( sort ) . \;
        println!( );
        pub fn dump_stats ( &self, file )  {
        // with scope: open ( file , "wb" ) as f  {
        self . create_stats ( );
        marshal . dump ( self . stats , f );
        pub fn create_stats ( self )  {
        self . simulate_cmd_complete ( );
        self . snapshot_stats ( );
        pub fn snapshot_stats ( self )  {
        self . stats = { };
        for func , ( cc , ns , tt , ct , callers ) in self . timings . items ( ) .iter() {
        callers = callers . copy ( );
        nc = 0;
        for callcnt in callers . values ( ) .iter() {
        nc + = callcnt;
        self . stats [ func ] = cc , nc , tt , ct , callers;
        pub fn run ( &self, cmd )  {
        import __main__;
        dict = __main__ . __dict__;
        return  self . runctx ( cmd , dict , dict );
        pub fn runctx ( &self, cmd , globals , locals )  {
        self . set_cmd ( cmd );
        sys . setprofile ( self . dispatcher );
        // try {
        exec ( cmd , globals , locals );
        // } finally {
        sys . setprofile ( None /* Option */ );
        return  self;
        pub fn runcall ( &self, func , / , * args , ** kw )  {
        self . set_cmd ( repr ( func ) );
        sys . setprofile ( self . dispatcher );
        // try {
        return  func ( * args , ** kw );
        // } finally {
        sys . setprofile ( None /* Option */ );
        pub fn calibrate ( &self, m , verbose = 0 )  {
        if self . __class__ is !Profile {
        panic!("TypeError ( "Subclasses must override .calibrate()." )");
        saved_bias = self . bias;
        self . bias = 0;
        // try {
        return  self . _calibrate_inner ( m , verbose );
        // } finally {
        self . bias = saved_bias;
        pub fn _calibrate_inner ( &self, m , verbose )  {
        get_time = self . get_time;
        pub fn f1 ( n )  {
        for i in range ( n ) .iter() {
        x = 1;
        pub fn f ( m , f1 = f1 )  {
        for i in range ( m ) .iter() {
        f1 ( 100 );
        f ( m );
        t0 = get_time ( );
        f ( m );
        t1 = get_time ( );
        elapsed_noprofile = t1 - t0;
        if verbose {
        println!( "elapsed time without profiling =" , elapsed_noprofile );
        p = Profile ( );
        t0 = get_time ( );
        p . runctx ( "f(m)" , globals ( ) , locals ( ) );
        t1 = get_time ( );
        elapsed_profile = t1 - t0;
        if verbose {
        println!( "elapsed time with profiling =" , elapsed_profile );
        total_calls = 0.0;
        reported_time = 0.0;
        for ( filename , line , funcname ) , ( cc , ns , tt , ct , callers ) in \.iter() {
        p . timings . items ( ) ;
        if funcname in ( "f" , "f1" ) {
        total_calls + = cc;
        reported_time + = tt;
        if verbose {
        println!( "'CPU seconds' profiler reported =" , reported_time );
        println!( "total # calls =" , total_calls );
        if total_calls != m + 1 {
        panic!("ValueError ( "internal error: total calls = %d" % total_calls )");
        mean = ( reported_time - elapsed_noprofile ) / 2.0 / total_calls;
        if verbose {
        println!( "mean stopwatch overhead per profile event =" , mean );
        return  mean;
        pub fn main ( )  {
        import os;
        from optparse import OptionParser;
        usage = "profile.py [-o output_file_path] [-s sort] [-m module | scriptfile] [arg] ...";
        parser = OptionParser ( usage = usage );
        parser . allow_interspersed_args = false;
        parser . add_option ( "-o" , "--outfile" , dest = "outfile" ,;
        help = "Save stats to <outfile>" , default = None /* Option */ );
        parser . add_option ( "-m" , dest = "module" , action = "store_true" ,;
        help = "Profile a library module." , default = false );
        parser . add_option ( "-s" , "--sort" , dest = "sort" ,;
        help = "Sort order when printing to stdout, based on pstats.Stats class" ,;
        default = -1 );
        if !sys . argv [ 1 { : ] ; }
        parser . print_usage ( );
        sys . exit ( 2 );
        ( options , args ) = parser . parse_args ( );
        sys . argv [ : ] = args;
        if options . outfile is !None /* Option */ {
        options . outfile = os . path . abspath ( options . outfile );
        if len ( args ) > 0 {
        if options . module {
        import runpy;
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

