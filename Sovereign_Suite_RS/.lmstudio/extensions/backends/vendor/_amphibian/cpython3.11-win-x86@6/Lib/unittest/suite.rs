//! suite.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::.::{case};

pub const __unittest: f64 = True;
pub fn _call_if_exists(parent: &str, attr: &str) {
        func = getattr ( parent , attr , || {  None /* Option */ ) };
        func ( );
        class BaseTestSuite ( object ) ;
        "A simple test suite that doesn't provide class || module shared fixtures.
    ";
        _cleanup = true;
        pub fn __init__ ( &self, tests = ( ) )  {
        self . _tests = [ ];
        self . _removed_tests = 0;
        self . addTests ( tests );
        pub fn __repr__ ( self )  {
        return  "<%s tests=%s>" % ( util . strclass ( self . __class__ ) , list ( self ) );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , self . __class__ ) {
        return  NotImplemented;
        return  list ( self ) == list ( other );
        pub fn __iter__ ( self )  {
        return  iter ( self . _tests );
        pub fn countTestCases ( self )  {
        cases = self . _removed_tests;
        for test in self .iter() {
        if test {
        cases + = test . countTestCases ( );
        return  cases;
        pub fn addTest ( &self, test )  {
        if !callable ( test ) {
        panic!("TypeError ( "{} is !callable" . format ( repr ( test ) ) )");
        if isinstance ( test , type ) && issubclass ( test , {
        ( case . TestCase , TestSuite ) ) ;
        panic!("TypeError ( "TestCases && TestSuites must be instantiated "");
        "before passing them to addTest()" );
        self . _tests . append ( test );
        pub fn addTests ( &self, tests )  {
        if isinstance ( tests , str ) {
        panic!("TypeError ( "tests must be an iterable of tests, !a string" )");
        for test in tests .iter() {
        self . addTest ( test );
        pub fn run ( &self, result )  {
        for index , test in enumerate ( self ) .iter() {
        if result . shouldStop {
        break;
        test ( result );
        if self . _cleanup {
        self . _removeTestAtIndex ( index );
        return  result;
        pub fn _removeTestAtIndex ( &self, index )  {
        "Stop holding a reference to the TestCase at index.";
        // try {
        test = self . _tests [ index ];
        // } catch  TypeError  {
        // pass
        } else {
        if hasattr ( test , "countTestCases" ) {
        self . _removed_tests + = test . countTestCases ( );
        self . _tests [ index ] = None /* Option */;
        pub fn __call__ ( &self, * args , ** kwds )  {
        return  self . run ( * args , ** kwds );
        pub fn debug ( self )  {
        "Run the tests without collecting errors in a TestResult";
        for test in self .iter() {
        test . debug ( );
        class TestSuite ( BaseTestSuite ) ;
        "A test suite == a composite test consisting of a number of TestCases.

    For use, create an instance of TestSuite, then add test case instances.
    When all tests have been added, the suite can be passed to a test
    runner, such as TextTestRunner. It will run the individual test cases
    in the order in which they were added, aggregating the results. When
    subclassing, do !forget to call the base class constructor.
    ";
        pub fn run ( &self, result , debug = false )  {
        topLevel = false;
        if getattr ( result , "_testRunEntered" , false ) is false {
        result . _testRunEntered = topLevel = true;
        for index , test in enumerate ( self ) .iter() {
        if result . shouldStop {
        break;
        if _isnotsuite ( test ) {
        self . _tearDownPreviousClass ( test , result );
        self . _handleModuleFixture ( test , result );
        self . _handleClassSetUp ( test , result );
        result . _previousTestClass = test . __class__;
        if ( getattr ( test . __class__ , "_classSetupFailed" , false ) or {
        getattr ( result , "_moduleSetUpFailed" , false ) ) ;
        continue;
        if !debug {
        test ( result );
        } else {
        test . debug ( );
        if self . _cleanup {
        self . _removeTestAtIndex ( index );
        if topLevel {
        self . _tearDownPreviousClass ( None /* Option */ , result );
        self . _handleModuleTearDown ( result );
        result . _testRunEntered = false;
        return  result;
        pub fn debug ( self )  {
        "Run the tests without collecting errors in a TestResult";
        debug = _DebugResult ( );
        self . run ( debug , true );
        pub fn _handleClassSetUp ( &self, test , result )  {
        previousClass = getattr ( result , "_previousTestClass" , None /* Option */ );
        currentClass = test . __class__;
        if currentClass == previousClass {
        return;
        if result . _moduleSetUpFailed {
        return;
        if getattr ( currentClass , "__unittest_skip__" , false ) {
        return;
        failed = false;
        // try {
        currentClass . _classSetupFailed = false;
        // } catch  TypeError  {
        // pass
        setUpClass = getattr ( currentClass , "setUpClass" , None /* Option */ );
        doClassCleanups = getattr ( currentClass , "doClassCleanups" , None /* Option */ );
        if setUpClass is !None /* Option */ {
        _call_if_exists ( result , "_setupStdout" );
        // try {
        // try {
        setUpClass ( );
        // } catch  Exception as e  {
        if isinstance ( result , _DebugResult ) {
        panic!("");
        failed = true;
        // try {
        currentClass . _classSetupFailed = true;
        // } catch  TypeError  {
        // pass
        className = util . strclass ( currentClass );
        self . _createClassOrModuleLevelException ( result , e ,;
        "setUpClass" ,;
        className );
        if failed && doClassCleanups is !None /* Option */ {
        doClassCleanups ( );
        for exc_info in currentClass . tearDown_exceptions .iter() {
        self . _createClassOrModuleLevelException (;
        result , exc_info [ 1 ] , "setUpClass" , className ,;
        info = exc_info );
        // } finally {
        _call_if_exists ( result , "_restoreStdout" );
        pub fn _get_previous_module ( &self, result )  {
        previousModule = None /* Option */;
        previousClass = getattr ( result , "_previousTestClass" , None /* Option */ );
        if previousClass is !None /* Option */ {
        previousModule = previousClass . __module__;
        return  previousModule;
        pub fn _handleModuleFixture ( &self, test , result )  {
        previousModule = self . _get_previous_module ( result );
        currentModule = test . __class__ . __module__;
        if currentModule == previousModule {
        return;
        self . _handleModuleTearDown ( result );
        result . _moduleSetUpFailed = false;
        // try {
        module = sys . modules [ currentModule ];
        // } catch  KeyError  {
        return;
        setUpModule = getattr ( module , "setUpModule" , None /* Option */ );
        if setUpModule is !None /* Option */ {
        _call_if_exists ( result , "_setupStdout" );
        // try {
        // try {
        setUpModule ( );
        // } catch  Exception as e  {
        if isinstance ( result , _DebugResult ) {
        panic!("");
        result . _moduleSetUpFailed = true;
        self . _createClassOrModuleLevelException ( result , e ,;
        "setUpModule" ,;
        currentModule );
        if result . _moduleSetUpFailed {
        // try {
        case . doModuleCleanups ( );
        // } catch  Exception as e  {
        self . _createClassOrModuleLevelException ( result , e ,;
        "setUpModule" ,;
        currentModule );
        // } finally {
        _call_if_exists ( result , "_restoreStdout" );
        pub fn _createClassOrModuleLevelException ( &self, result , exc , method_name , {
        parent , info = None /* Option */ ) ;
        errorName = format!("{method_name} ({parent})");
        self . _addClassOrModuleLevelException ( result , exc , errorName , info );
        pub fn _addClassOrModuleLevelException ( &self, result , exception , errorName , {
        info = None /* Option */ ) ;
        error = _ErrorHolder ( errorName );
        addSkip = getattr ( result , "addSkip" , None /* Option */ );
        if addSkip is !None /* Option */ && isinstance ( exception , case . SkipTest ) {
        addSkip ( error , str ( exception ) );
        } else {
        if !info {
        result . addError ( error , sys . exc_info ( ) );
        } else {
        result . addError ( error , info );
        pub fn _handleModuleTearDown ( &self, result )  {
        previousModule = self . _get_previous_module ( result );
        if previousModule is None /* Option */ {
        return;
        if result . _moduleSetUpFailed {
        return;
        // try {
        module = sys . modules [ previousModule ];
        // } catch  KeyError  {
        return;
        _call_if_exists ( result , "_setupStdout" );
        // try {
        tearDownModule = getattr ( module , "tearDownModule" , None /* Option */ );
        if tearDownModule is !None /* Option */ {
        // try {
        tearDownModule ( );
        // } catch  Exception as e  {
        if isinstance ( result , _DebugResult ) {
        panic!("");
        self . _createClassOrModuleLevelException ( result , e ,;
        "tearDownModule" ,;
        previousModule );
        // try {
        case . doModuleCleanups ( );
        // } catch  Exception as e  {
        if isinstance ( result , _DebugResult ) {
        panic!("");
        self . _createClassOrModuleLevelException ( result , e ,;
        "tearDownModule" ,;
        previousModule );
        // } finally {
        _call_if_exists ( result , "_restoreStdout" );
        pub fn _tearDownPreviousClass ( &self, test , result )  {
        previousClass = getattr ( result , "_previousTestClass" , None /* Option */ );
        currentClass = test . __class__;
        if currentClass == previousClass || previousClass is None /* Option */ {
        return;
        if getattr ( previousClass , "_classSetupFailed" , false ) {
        return;
        if getattr ( result , "_moduleSetUpFailed" , false ) {
        return;
        if getattr ( previousClass , "__unittest_skip__" , false ) {
        return;
        tearDownClass = getattr ( previousClass , "tearDownClass" , None /* Option */ );
        doClassCleanups = getattr ( previousClass , "doClassCleanups" , None /* Option */ );
        if tearDownClass is None /* Option */ && doClassCleanups is None /* Option */ {
        return;
        _call_if_exists ( result , "_setupStdout" );
        // try {
        if tearDownClass is !None /* Option */ {
        // try {
        tearDownClass ( );
        // } catch  Exception as e  {
        if isinstance ( result , _DebugResult ) {
        panic!("");
        className = util . strclass ( previousClass );
        self . _createClassOrModuleLevelException ( result , e ,;
        "tearDownClass" ,;
        className );
        if doClassCleanups is !None /* Option */ {
        doClassCleanups ( );
        for exc_info in previousClass . tearDown_exceptions .iter() {
        if isinstance ( result , _DebugResult ) {
        panic!("exc_info [ 1 ]");
        className = util . strclass ( previousClass );
        self . _createClassOrModuleLevelException ( result , exc_info [ 1 ] ,;
        "tearDownClass" ,;
        className ,;
        info = exc_info );
        // } finally {
        _call_if_exists ( result , "_restoreStdout" );
        class _ErrorHolder ( object ) ;
        "
    Placeholder for a TestCase inside a result. As far as a TestResult
    == concerned, this looks exactly like a unit test. Used to insert
    arbitrary errors into a test suite run.
    ";
        failureException = None /* Option */;
        pub fn __init__ ( &self, description )  {
        self . description = description;
        pub fn id ( self )  {
        return  self . description;
        pub fn shortDescription ( self )  {
        return;
        pub fn __repr__ ( self )  {
        return  "<ErrorHolder description=%r>" % ( self . description , );
        pub fn __str__ ( self )  {
        return  self . id ( );
        pub fn run ( &self, result )  {
        // pass
        pub fn __call__ ( &self, result )  {
        return  self . run ( result );
        pub fn countTestCases ( self )  {
        return  0;
        pub fn _isnotsuite ( test )  {
        "A crude way to tell apart testcases && suites with duck-typing";
        // try {
        iter ( test );
        // } catch  TypeError  {
        return  true;
        return  false;
        class _DebugResult ( object ) ;
        "Used by the TestSuite to hold previous class when running in debug.";
        _previousTestClass = None /* Option */;
        _moduleSetUpFailed = false;
        shouldStop = false;
}

