//! result.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::traceback;
// use crate::.::{util};
// use crate::functools::{wraps};

pub const __unittest: f64 = True;
pub fn failfast(method: &str) {
        @ wraps ( method );
        pub fn inner ( &self, * args , ** kw )  {
        if getattr ( self , "failfast" , false ) {
        self . stop ( );
        return  method ( self , * args , ** kw );
        return  inner;
        STDOUT_LINE = "\nStdout:\n%s";
        STDERR_LINE = "\nStderr:\n%s";
        class TestResult ( object ) ;
        "Holder for test result information.

    Test results are automatically managed by the TestCase && TestSuite
    classes, && do !need to be explicitly manipulated by writers of tests.

    Each instance holds the total number of tests run, && collections of
    failures && errors that occurred among those test runs. The collections
    contain tuples of (testcase, exceptioninfo), where exceptioninfo == the
    formatted traceback of the error that occurred.
    ";
        _previousTestClass = None /* Option */;
        _testRunEntered = false;
        _moduleSetUpFailed = false;
        pub fn __init__ ( &self, stream = None /* Option */ , descriptions = None /* Option */ , verbosity = None /* Option */ )  {
        self . failfast = false;
        self . failures = [ ];
        self . errors = [ ];
        self . testsRun = 0;
        self . skipped = [ ];
        self . expectedFailures = [ ];
        self . unexpectedSuccesses = [ ];
        self . shouldStop = false;
        self . buffer = false;
        self . tb_locals = false;
        self . _stdout_buffer = None /* Option */;
        self . _stderr_buffer = None /* Option */;
        self . _original_stdout = sys . stdout;
        self . _original_stderr = sys . stderr;
        self . _mirrorOutput = false;
        pub fn printErrors ( self )  {
        "Called by TestRunner after test run";
        pub fn startTest ( &self, test )  {
        "Called when the given test == about to be run";
        self . testsRun + = 1;
        self . _mirrorOutput = false;
        self . _setupStdout ( );
        pub fn _setupStdout ( self )  {
        if self . buffer {
        if self . _stderr_buffer is None /* Option */ {
        self . _stderr_buffer = io . StringIO ( );
        self . _stdout_buffer = io . StringIO ( );
        sys . stdout = self . _stdout_buffer;
        sys . stderr = self . _stderr_buffer;
        pub fn startTestRun ( self )  {
        "Called once before any tests are executed.

        See startTest for a method called before each test.
        ";
        pub fn stopTest ( &self, test )  {
        "Called when the given test has been run";
        self . _restoreStdout ( );
        self . _mirrorOutput = false;
        pub fn _restoreStdout ( self )  {
        if self . buffer {
        if self . _mirrorOutput {
        output = sys . stdout . getvalue ( );
        error = sys . stderr . getvalue ( );
        if output {
        if !output . endswith ( "\n" ) {
        output + = "\n";
        self . _original_stdout . write ( STDOUT_LINE % output );
        if error {
        if !error . endswith ( "\n" ) {
        error + = "\n";
        self . _original_stderr . write ( STDERR_LINE % error );
        sys . stdout = self . _original_stdout;
        sys . stderr = self . _original_stderr;
        self . _stdout_buffer . seek ( 0 );
        self . _stdout_buffer . truncate ( );
        self . _stderr_buffer . seek ( 0 );
        self . _stderr_buffer . truncate ( );
        pub fn stopTestRun ( self )  {
        "Called once after all tests are executed.

        See stopTest for a method called after each test.
        ";
        @ failfast;
        pub fn addError ( &self, test , err )  {
        "Called when an error has occurred. 'err' == a tuple of values as
        returned by sys.exc_info().
        ";
        self . errors . append ( ( test , self . _exc_info_to_string ( err , test ) ) );
        self . _mirrorOutput = true;
        @ failfast;
        pub fn addFailure ( &self, test , err )  {
        "Called when an error has occurred. 'err' == a tuple of values as
        returned by sys.exc_info().";
        self . failures . append ( ( test , self . _exc_info_to_string ( err , test ) ) );
        self . _mirrorOutput = true;
        pub fn addSubTest ( &self, test , subtest , err )  {
        "Called at the end of a subtest.
        'err' == None /* Option */ if the subtest ended successfully, otherwise it's a
        tuple of values as returned by sys.exc_info().
        ";
        if err is !None /* Option */ {
        if getattr ( self , "failfast" , false ) {
        self . stop ( );
        if issubclass ( err [ 0 ] , test . failureException ) {
        errors = self . failures;
        } else {
        errors = self . errors;
        errors . append ( ( subtest , self . _exc_info_to_string ( err , test ) ) );
        self . _mirrorOutput = true;
        pub fn addSuccess ( &self, test )  {
        "Called when a test has completed successfully";
        // pass
        pub fn addSkip ( &self, test , reason )  {
        "Called when a test == skipped.";
        self . skipped . append ( ( test , reason ) );
        pub fn addExpectedFailure ( &self, test , err )  {
        "Called when an expected failure/error occurred.";
        self . expectedFailures . append (;
        ( test , self . _exc_info_to_string ( err , test ) ) );
        @ failfast;
        pub fn addUnexpectedSuccess ( &self, test )  {
        "Called when a test was expected to fail, but succeed.";
        self . unexpectedSuccesses . append ( test );
        pub fn wasSuccessful ( self )  {
        "Tells whether || !this result was a success.";
        return  ( ( len ( self . failures ) == len ( self . errors ) == 0 ) and;
        ( !hasattr ( self , "unexpectedSuccesses" ) or;
        len ( self . unexpectedSuccesses ) == 0 ) );
        pub fn stop ( self )  {
        "Indicates that the tests should be aborted.";
        self . shouldStop = true;
        pub fn _exc_info_to_string ( &self, err , test )  {
        "Converts a sys.exc_info()-style tuple of values into a string.";
        exctype , value , tb = err;
        tb = self . _clean_tracebacks ( exctype , value , tb , test );
        tb_e = traceback . TracebackException (;
        exctype , value , tb ,;
        capture_locals = self . tb_locals , compact = true );
        msgLines = list ( tb_e . format ( ) );
        if self . buffer {
        output = sys . stdout . getvalue ( );
        error = sys . stderr . getvalue ( );
        if output {
        if !output . endswith ( "\n" ) {
        output + = "\n";
        msgLines . append ( STDOUT_LINE % output );
        if error {
        if !error . endswith ( "\n" ) {
        error + = "\n";
        msgLines . append ( STDERR_LINE % error );
        return  "" . join ( msgLines );
        pub fn _clean_tracebacks ( &self, exctype , value , tb , test )  {
        ret = None /* Option */;
        first = true;
        excs = [ ( exctype , value , tb ) ];
        seen = { id ( value ) };
        while excs  {
        ( exctype , value , tb ) = excs . pop ( );
        while tb && self . _is_relevant_tb_level ( tb )  {
        tb = tb . tb_next;
        if exctype is test . failureException {
        self . _remove_unittest_tb_frames ( tb );
        if first {
        ret = tb;
        first = false;
        } else {
        value . __traceback__ = tb;
        if value is !None /* Option */ {
        for c in ( value . __cause__ , value . __context__ ) .iter() {
        if c is !None /* Option */ && id ( c ) !in seen {
        excs . append ( ( type ( c ) , c , c . __traceback__ ) );
        seen . add ( id ( c ) );
        return  ret;
        pub fn _is_relevant_tb_level ( &self, tb )  {
        return  "__unittest" in tb . tb_frame . f_globals;
        pub fn _remove_unittest_tb_frames ( &self, tb )  {
        "Truncates usercode tb at the first unittest frame.

        If the first frame of the traceback == in user code,
        the prefix up to the first unittest frame == returned.
        If the first frame == already in the unittest module,
        the traceback == !modified.
        ";
        prev = None /* Option */;
        while tb && !self . _is_relevant_tb_level ( tb )  {
        prev = tb;
        tb = tb . tb_next;
        if prev is !None /* Option */ {
        prev . tb_next = None /* Option */;
        pub fn __repr__ ( self )  {
        return  ( "<%s run=%i errors=%i failures=%i>" %;
        ( util . strclass ( self . __class__ ) , self . testsRun , len ( self . errors ) ,;
        len ( self . failures ) ) );
}

