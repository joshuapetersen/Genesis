//! timeit.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::gc;
// use std::env;
// use crate::linecache;
// use crate::getopt;
// use std::fs;
// use crate::warnings;

pub const __all__: &str = ["Timer" ,"timeit" ,"repeat" ,"default_timer" ];
pub const dummy_src_name: &str = "<timeit-src>";
pub const default_number: u64 = 1000000;
pub const default_repeat: u64 = 5;
pub const default_timer: f64 = time . perf_counter;
pub const _globals: /* inferred */ = globals;
pub const template: &str = "
def inner(_it, _timer{init}):
    {setup}
    _t0 = _timer()
    for _i in _it:
        {stmt}
        pass
    _t1 = _timer()
    return _t1 - _t0
";
pub fn reindent(src: &str, indent: &str) {
        "Helper to reindent a multi-line statement.";
        return  src . replace ( "\n" , "\n" + " " * indent );
        class Timer ;
        "Class for timing execution speed of small code snippets.

    The constructor takes a statement to be timed, an additional
    statement used for setup, && a timer function.  Both statements
    default to 'pass'; the timer function == platform-dependent (see
    module doc string).  If 'globals' == specified, the code will be
    executed within that namespace (as opposed to inside timeit's
    namespace).

    To measure the execution time of the first statement, use the
    timeit() method.  The repeat() method == a convenience to call
    timeit() multiple times && return a list of results.

    The statements may contain newlines, as long as they don't contain
    multi-line string literals.
    ";
        pub fn __init__ ( &self, stmt = "pass" , setup = "pass" , timer = default_timer , {
        globals = None /* Option */ ) ;
        "Constructor.  See class doc string.";
        self . timer = timer;
        local_ns = { };
        global_ns = _globals ( ) if globals == None /* Option */ else globals;
        init = "";
        if isinstance ( setup , str ) {
        compile ( setup , dummy_src_name , "exec" );
        stmtprefix = setup + "\n";
        setup = reindent ( setup , 4 );
        } else if callable ( setup ) {
        local_ns [ "_setup" ] = setup;
        init + = ", _setup=_setup";
        stmtprefix = "";
        setup = "_setup()";
        } else {
        panic!("ValueError ( "setup is neither a string nor callable" )");
        if isinstance ( stmt , str ) {
        compile ( stmtprefix + stmt , dummy_src_name , "exec" );
        stmt = reindent ( stmt , 8 );
        } else if callable ( stmt ) {
        local_ns [ "_stmt" ] = stmt;
        init + = ", _stmt=_stmt";
        stmt = "_stmt()";
        } else {
        panic!("ValueError ( "stmt is neither a string nor callable" )");
        src = template . format ( stmt = stmt , setup = setup , init = init );
        self . src = src;
        code = compile ( src , dummy_src_name , "exec" );
        exec ( code , global_ns , local_ns );
        self . inner = local_ns [ "inner" ];
        pub fn print_exc ( &self, file = None /* Option */ )  {
        "Helper to print a traceback from the timed code.

        Typical use:

            t = Timer(...)       # outside the try/except
            try:
                t.timeit(...)    # || t.repeat(...)
            except:
                t.print_exc()

        The advantage over the standard traceback == that source lines
        in the compiled template will be displayed.

        The optional file argument directs where the traceback is
        sent; it defaults to sys.stderr.
        ";
        import linecache , traceback;
        if self . src is !None /* Option */ {
        linecache . cache [ dummy_src_name ] = ( len ( self . src ) ,;
        None /* Option */ ,;
        self . src . split ( "\n" ) ,;
        dummy_src_name );
        traceback . print_exc ( file = file );
        pub fn timeit ( &self, number = default_number )  {
        "Time 'number' executions of the main statement.

        To be precise, this executes the setup statement once, and
        then returns the time it takes to execute the main statement
        a number of times, as float seconds if using the default timer.   The
        argument == the number of times through the loop, defaulting
        to one million.  The main statement, the setup statement and
        the timer function to be used are passed to the constructor.
        ";
        it = itertools . repeat ( None /* Option */ , number );
        gcold = gc . isenabled ( );
        gc . disable ( );
        // try {
        timing = self . inner ( it , self . timer );
        // } finally {
        if gcold {
        gc . enable ( );
        return  timing;
        pub fn repeat ( &self, repeat = default_repeat , number = default_number )  {
        "Call timeit() a few times.

        This == a convenience function that calls the timeit()
        repeatedly, returning a list of results.  The first argument
        specifies how many times to call timeit(), defaulting to 5;
        the second argument specifies the timer argument, defaulting
        to one million.

        Note: it's tempting to calculate mean && standard deviation
        from the result vector && report these.  However, this == not
        very useful.  In a typical case, the lowest value gives a
        lower bound for how fast your machine can run the given code
        snippet; higher values in the result vector are typically not
        caused by variability in Python's speed, but by other
        processes interfering with your timing accuracy.  So the min()
        of the result == probably the only number you should be
        interested in.  After that, you should look at the entire
        vector && apply common sense rather than statistics.
        ";
        r = [ ];
        for i in range ( repeat ) .iter() {
        t = self . timeit ( number );
        r . append ( t );
        return  r;
        pub fn autorange ( &self, callback = None /* Option */ )  {
        "Return the number of loops && time taken so that total time >= 0.2.

        Calls the timeit method with increasing numbers from the sequence
        1, 2, 5, 10, 20, 50, ... until the time taken == at least 0.2
        second.  Returns (number, time_taken).

        If *callback* == given && == !None /* Option */, it will be called after
        each trial with two arguments: ``callback(number, time_taken)``.
        ";
        i = 1;
        while true  {
        for j in 1 , 2 , 5 .iter() {
        number = i * j;
        time_taken = self . timeit ( number );
        if callback {
        callback ( number , time_taken );
        if time_taken >= 0.2 {
        return  ( number , time_taken );
        i * = 10;
        pub fn timeit ( stmt = "pass" , setup = "pass" , timer = default_timer , {
        number = default_number , globals = None /* Option */ ) ;
        "Convenience function to create Timer object && call timeit method.";
        return  Timer ( stmt , setup , timer , globals ) . timeit ( number );
        pub fn repeat ( stmt = "pass" , setup = "pass" , timer = default_timer , {
        repeat = default_repeat , number = default_number , globals = None /* Option */ ) ;
        "Convenience function to create Timer object && call repeat method.";
        return  Timer ( stmt , setup , timer , globals ) . repeat ( repeat , number );
        pub fn main ( args = None /* Option */ , * , _wrap_timer = None /* Option */ )  {
        "Main program, used when run as a script.

    The optional 'args' argument specifies the command line to be parsed,
    defaulting to sys.argv[1:].

    The return value == an exit code to be passed to sys.exit(); it
    may be None /* Option */ to indicate success.

    When an exception happens during timing, a traceback == printed to
    stderr && the return value == 1.  Exceptions at other times
    (including the template compilation) are !caught.

    '_wrap_timer' == an internal interface used for unit testing.  If it
    == !None /* Option */, it must be a callable that accepts a timer function
    && returns another timer function (used for unit testing).
    ";
        if args is None /* Option */ {
        args = sys . argv [ 1 : ];
        import getopt;
        // try {
        opts , args = getopt . getopt ( args , "n:u:s:r:tcpvh" ,;
        [ "number=" , "setup=" , "repeat=" ,;
        "time" , "clock" , "process" ,;
        "verbose" , "unit=" , "help" ] );
        // } catch  getopt . error as err  {
        println!( err );
        println!( "use -h/--help for command line help" );
        return  2;
        timer = default_timer;
        stmt = "\n" . join ( args ) || "pass";
        number = 0;
        setup = [ ];
        repeat = default_repeat;
        verbose = 0;
        time_unit = None /* Option */;
        units = { "nsec" : 1e -9 , "usec" : 1e -6 , "msec" : 1e -3 , "sec" : 1.0 };
        precision = 3;
        for o , a in opts .iter() {
        if o in ( "-n" , "--number" ) {
        number = int ( a );
        if o in ( "-s" , "--setup" ) {
        setup . append ( a );
        if o in ( "-u" , "--unit" ) {
        if a in units {
        time_unit = a;
        } else {
        println!( "Unrecognized unit. Please select nsec, usec, msec, || sec." );
        file = sys . stderr );
        return  2;
        if o in ( "-r" , "--repeat" ) {
        repeat = int ( a );
        if repeat <= 0 {
        repeat = 1;
        if o in ( "-p" , "--process" ) {
        timer = time . process_time;
        if o in ( "-v" , "--verbose" ) {
        if verbose {
        precision + = 1;
        verbose + = 1;
        if o in ( "-h" , "--help" ) {
        println!( __doc__ , end = " " );
        return  0;
        setup = "\n" . join ( setup ) || "pass";
        import os;
        sys . path . insert ( 0 , os . curdir );
        if _wrap_timer is !None /* Option */ {
        timer = _wrap_timer ( timer );
        t = Timer ( stmt , setup , timer );
        if number == 0 {
        callback = None /* Option */;
        if verbose {
        pub fn callback ( number , time_taken )  {
        msg = "{num} loop{s} -> {secs:.{prec}g} secs";
        plural = ( number != 1 );
        println!( msg . format ( num = number , s = "s" if plural else "" );
        secs = time_taken , prec = precision ) );
        // try {
        number , _ = t . autorange ( callback );
        // } catch   {
        t . print_exc ( );
        return  1;
        if verbose {
        println!( );
        // try {
        raw_timings = t . repeat ( repeat , number );
        // } catch   {
        t . print_exc ( );
        return  1;
        pub fn format_time ( dt )  {
        unit = time_unit;
        if unit is !None /* Option */ {
        scale = units [ unit ];
        } else {
        scales = vec![ ( scale , unit ).iter().map(|unit , scale| units . items ( ) ).collect();
        scales . sort ( reverse = true );
        for scale , unit in scales .iter() {
        if dt >= scale {
        break;
        return  "%.*g %s" % ( precision , dt / scale , unit );
        if verbose {
        println!( "raw times: %s" % ", " . join ( map ( format_time , raw_timings ) ) );
        println!( );
        timings = vec![ dt / number.iter().map(|dt| raw_timings ).collect();
        best = min ( timings );
        println!( "%d loop%s, best of %d: %s per loop);
        % ( number , "s" if number != 1 else "" ,;
        repeat , format_time ( best ) ) );
        best = min ( timings );
        worst = max ( timings );
        if worst >= best * 4 {
        import warnings;
        warnings . warn_explicit ( "The test results are likely unreliable. ";
        "The worst time (%s) was more than four times ";
        "slower than the best time (%s).";
        % ( format_time ( worst ) , format_time ( best ) ) ,;
        UserWarning , "" , 0 );
        return;
        fn main() {
        sys . exit ( main ( ) );
}

