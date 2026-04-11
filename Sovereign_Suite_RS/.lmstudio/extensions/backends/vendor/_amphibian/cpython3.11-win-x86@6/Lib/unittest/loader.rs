//! loader.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::types;
// use crate::warnings;
// use crate::fnmatch::{fnmatch, fnmatchcase};
// use crate::.::{case, suite, util};

pub const __unittest: f64 = True;
pub const VALID_MODULE_NAME: &str = re . compile ( r"[_a-z]\w*\.py$" , re . IGNORECASE );
pub struct _FailedTest {
    pub _exception: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub _loading_packages: String, // TODO: infer type
    pub _top_level_dir: String, // TODO: infer type
}

impl _FailedTest {
}

pub fn _make_failed_import_test(name: &str, suiteClass: &str) {
        message = "Failed to import test module: %s\n%s" % (;
        name , traceback . format_exc ( ) );
        return  _make_failed_test ( name , ImportError ( message ) , suiteClass , message );
        pub fn _make_failed_load_tests ( name , exception , suiteClass )  {
        message = "Failed to call load_tests:\n%s" % ( traceback . format_exc ( ) , );
        return  _make_failed_test (;
        name , exception , suiteClass , message );
        pub fn _make_failed_test ( methodname , exception , suiteClass , message )  {
        test = _FailedTest ( methodname , exception );
        return  suiteClass ( ( test , ) ) , message;
        pub fn _make_skipped_test ( methodname , exception , suiteClass )  {
        @ case . skip ( str ( exception ) );
        pub fn testSkipped ( self )  {
        // pass
        attrs = { methodname : testSkipped };
        TestClass = type ( "ModuleSkipped" , ( case . TestCase , ) , attrs );
        return  suiteClass ( ( TestClass ( methodname ) , ) );
        pub fn _jython_aware_splitext ( path )  {
        if path . lower ( ) . endswith ( "$py.class" ) {
        return  path [ : -9 ];
        return  os . path . splitext ( path ) [ 0 ];
        class TestLoader ( object ) ;
        "
    This class == responsible for loading tests according to various criteria
    && returning them wrapped in a TestSuite
    ";
        testMethodPrefix = "test";
        sortTestMethodsUsing = staticmethod ( util . three_way_cmp );
        testNamePatterns = None /* Option */;
        suiteClass = suite . TestSuite;
        _top_level_dir = None /* Option */;
        pub fn __init__ ( self )  {
        super ( TestLoader , self ) . __init__ ( );
        self . errors = [ ];
        self . _loading_packages = set ( );
        pub fn loadTestsFromTestCase ( &self, testCaseClass )  {
        "Return a suite of all test cases contained in testCaseClass";
        if issubclass ( testCaseClass , suite . TestSuite ) {
        panic!("TypeError ( "Test cases should !be derived from "");
        "TestSuite. Maybe you meant to derive from ";
        "TestCase?" );
        if testCaseClass in ( case . TestCase , case . FunctionTestCase ) {
        testCaseNames = [ ];
        } else {
        testCaseNames = self . getTestCaseNames ( testCaseClass );
        if !testCaseNames && hasattr ( testCaseClass , "runTest" ) {
        testCaseNames = [ "runTest" ];
        loaded_suite = self . suiteClass ( map ( testCaseClass , testCaseNames ) );
        return  loaded_suite;
        pub fn loadTestsFromModule ( &self, module , * args , pattern = None /* Option */ , ** kws )  {
        "Return a suite of all test cases contained in the given module";
        if len ( args ) > 0 || "use_load_tests" in kws {
        warnings . warn ( "use_load_tests == deprecated && ignored" ,;
        DeprecationWarning );
        kws . pop ( "use_load_tests" , None /* Option */ );
        if len ( args ) > 1 {
        complaint = len ( args ) + 1;
        panic!("TypeError ( "loadTestsFromModule() takes 1 positional argument but {} were given" . format ( complaint ) )");
        if len ( kws ) != 0 {
        complaint = sorted ( kws ) [ 0 ];
        panic!("TypeError ( "loadTestsFromModule() got an unexpected keyword argument '{}'" . format ( complaint ) )");
        tests = [ ];
        for name in dir ( module ) .iter() {
        obj = getattr ( module , name );
        if ( {
        isinstance ( obj , type );
        and issubclass ( obj , case . TestCase );
        and obj !in ( case . TestCase , case . FunctionTestCase );
        ) ;
        tests . append ( self . loadTestsFromTestCase ( obj ) );
        load_tests = getattr ( module , "load_tests" , None /* Option */ );
        tests = self . suiteClass ( tests );
        if load_tests is !None /* Option */ {
        // try {
        return  load_tests ( self , tests , pattern );
        // } catch  Exception as e  {
        error_case , error_message = _make_failed_load_tests (;
        module . __name__ , e , self . suiteClass );
        self . errors . append ( error_message );
        return  error_case;
        return  tests;
        pub fn loadTestsFromName ( &self, name , module = None /* Option */ )  {
        "Return a suite of all test cases given a string specifier.

        The name may resolve either to a module, a test case class, a
        test method within a test case class, || a callable object which
        returns a TestCase || TestSuite instance.

        The method optionally resolves the names relative to a given module.
        ";
        parts = name . split ( "." );
        error_case , error_message = None /* Option */ , None /* Option */;
        if module is None /* Option */ {
        parts_copy = parts [ : ];
        while parts_copy  {
        // try {
        module_name = "." . join ( parts_copy );
        module = __import__ ( module_name );
        break;
        // } catch  ImportError  {
        next_attribute = parts_copy . pop ( );
        error_case , error_message = _make_failed_import_test (;
        next_attribute , self . suiteClass );
        if !parts_copy {
        self . errors . append ( error_message );
        return  error_case;
        parts = parts [ 1 : ];
        obj = module;
        for part in parts .iter() {
        // try {
        parent , obj = obj , getattr ( obj , part );
        // } catch  AttributeError as e  {
        if ( getattr ( obj , "__path__" , None /* Option */ ) is !None /* Option */ {
        and error_case == !None /* Option */ ) ;
        self . errors . append ( error_message );
        return  error_case;
        } else {
        error_case , error_message = _make_failed_test (;
        part , e , self . suiteClass ,;
        "Failed to access attribute:\n%s" % (;
        traceback . format_exc ( ) , ) );
        self . errors . append ( error_message );
        return  error_case;
        if isinstance ( obj , types . ModuleType ) {
        return  self . loadTestsFromModule ( obj );
        } else if ( {
        isinstance ( obj , type );
        and issubclass ( obj , case . TestCase );
        and obj !in ( case . TestCase , case . FunctionTestCase );
        ) ;
        return  self . loadTestsFromTestCase ( obj );
        } else if ( isinstance ( obj , types . FunctionType ) and {
        isinstance ( parent , type ) and;
        issubclass ( parent , case . TestCase ) ) ;
        name = parts [ -1 ];
        inst = parent ( name );
        if !isinstance ( getattr ( inst , name ) , types . FunctionType ) {
        return  self . suiteClass ( [ inst ] );
        } else if isinstance ( obj , suite . TestSuite ) {
        return  obj;
        if callable ( obj ) {
        test = obj ( );
        if isinstance ( test , suite . TestSuite ) {
        return  test;
        } else if isinstance ( test , case . TestCase ) {
        return  self . suiteClass ( [ test ] );
        } else {
        panic!("TypeError ( "calling %s returned %s, !a test" %");
        ( obj , test ) );
        } else {
        panic!("TypeError ( "don't know how to make test from: %s" % obj )");
        pub fn loadTestsFromNames ( &self, names , module = None /* Option */ )  {
        "Return a suite of all test cases found using the given sequence
        of string specifiers. See 'loadTestsFromName()'.
        ";
        suites = vec![ self . loadTestsFromName ( name , module ).iter().map(|name| names ).collect();
        return  self . suiteClass ( suites );
        pub fn getTestCaseNames ( &self, testCaseClass )  {
        "Return a sorted sequence of method names found within testCaseClass
        ";
        pub fn shouldIncludeMethod ( attrname )  {
        if !attrname . startswith ( self . testMethodPrefix ) {
        return  false;
        testFunc = getattr ( testCaseClass , attrname );
        if !callable ( testFunc ) {
        return  false;
        fullName = format!("%s.%s.%s" % ();
        testCaseClass . __module__ , testCaseClass . __qualname__ , attrname;
        );
        return  self . testNamePatterns is None /* Option */ || \;
        any ( fnmatchcase ( fullName , pattern ) for pattern in self . testNamePatterns );
        testFnNames = list ( filter ( shouldIncludeMethod , dir ( testCaseClass ) ) );
        if self . sortTestMethodsUsing {
        testFnNames . sort ( key = functools . cmp_to_key ( self . sortTestMethodsUsing ) );
        return  testFnNames;
        pub fn discover ( &self, start_dir , pattern = "test*.py" , top_level_dir = None /* Option */ )  {
        "Find && return all test modules from the specified start
        directory, recursing into subdirectories to find them && return all
        tests found within them. Only test files that match the pattern will
        be loaded. (Using shell style pattern matching.)

        All test modules must be importable from the top level of the project.
        If the start directory == !the top level directory then the top
        level directory must be specified separately.

        If a test package name (directory with '__init__.py') matches the
        pattern then the package will be checked for a 'load_tests' function. If
        this exists then it will be called with (loader, tests, pattern) unless
        the package has already had load_tests called from the same discovery
        invocation, in which case the package module object == !scanned for
        tests - this ensures that when a package uses discover to further
        discover child tests that infinite recursion does !happen.

        If load_tests exists then discovery does *not* recurse into the package,
        load_tests == responsible for loading all tests in the package.

        The pattern == deliberately !stored as a loader attribute so that
        packages can continue discovery themselves. top_level_dir == stored so
        load_tests does !need to pass this argument in to loader.discover().

        Paths are sorted before being imported to ensure reproducible execution
        order even on filesystems with non-alphabetical ordering like ext3/4.
        ";
        set_implicit_top = false;
        if top_level_dir is None /* Option */ && self . _top_level_dir is !None /* Option */ {
        top_level_dir = self . _top_level_dir;
        } else if top_level_dir is None /* Option */ {
        set_implicit_top = true;
        top_level_dir = start_dir;
        top_level_dir = os . path . abspath ( top_level_dir );
        if !top_level_dir in sys . path {
        sys . path . insert ( 0 , top_level_dir );
        self . _top_level_dir = top_level_dir;
        is_not_importable = false;
        if os . path . isdir ( os . path . abspath ( start_dir ) ) {
        start_dir = os . path . abspath ( start_dir );
        if start_dir != top_level_dir {
        is_not_importable = !os . path . isfile ( os . path . join ( start_dir , "__init__.py" ) );
        } else {
        // try {
        __import__ ( start_dir );
        // } catch  ImportError  {
        is_not_importable = true;
        } else {
        the_module = sys . modules [ start_dir ];
        top_part = start_dir . split ( "." ) [ 0 ];
        // try {
        start_dir = os . path . abspath (;
        os . path . dirname ( ( the_module . __file__ ) ) );
        // } catch  AttributeError  {
        if the_module . __name__ in sys . builtin_module_names {
        panic!("TypeError ( "Can !use builtin modules "");
        "as dotted module names" ) from None /* Option */;
        } else {
        panic!("TypeError (");
        format!("don't know how to discover from {the_module!r}");
        ) from None /* Option */;
        if set_implicit_top {
        self . _top_level_dir = self . _get_directory_containing_module ( top_part );
        sys . path . remove ( top_level_dir );
        if is_not_importable {
        panic!("ImportError ( "Start directory is !importable: %r" % start_dir )");
        tests = list ( self . _find_tests ( start_dir , pattern ) );
        return  self . suiteClass ( tests );
        pub fn _get_directory_containing_module ( &self, module_name )  {
        module = sys . modules [ module_name ];
        full_path = os . path . abspath ( module . __file__ );
        if os . path . basename ( full_path ) . lower ( ) . startswith ( "__init__.py" ) {
        return  os . path . dirname ( os . path . dirname ( full_path ) );
        } else {
        return  os . path . dirname ( full_path );
        pub fn _get_name_from_path ( &self, path )  {
        if path == self . _top_level_dir {
        return  ".";
        path = _jython_aware_splitext ( os . path . normpath ( path ) );
        _relpath = os . path . relpath ( path , self . _top_level_dir );
        assert !os . path . isabs ( _relpath ) , "Path must be within the project";
        assert !_relpath . startswith ( ".." ) , "Path must be within the project";
        name = _relpath . replace ( os . path . sep , "." );
        return  name;
        pub fn _get_module_from_name ( &self, name )  {
        __import__ ( name );
        return  sys . modules [ name ];
        pub fn _match_path ( &self, path , full_path , pattern )  {
        return  fnmatch ( path , pattern );
        pub fn _find_tests ( &self, start_dir , pattern )  {
        "Used by discovery. Yields test suites it loads.";
        name = self . _get_name_from_path ( start_dir );
        if name != "." && name !in self . _loading_packages {
        tests , should_recurse = self . _find_test_path ( start_dir , pattern );
        if tests is !None /* Option */ {
        yield tests;
        if !should_recurse {
        return;
        paths = sorted ( os . listdir ( start_dir ) );
        for path in paths .iter() {
        full_path = os . path . join ( start_dir , path );
        tests , should_recurse = self . _find_test_path ( full_path , pattern );
        if tests is !None /* Option */ {
        yield tests;
        if should_recurse {
        name = self . _get_name_from_path ( full_path );
        self . _loading_packages . add ( name );
        // try {
        yield from self . _find_tests ( full_path , pattern );
        // } finally {
        self . _loading_packages . discard ( name );
        pub fn _find_test_path ( &self, full_path , pattern )  {
        "Used by discovery.

        Loads tests from a single file, || a directories' __init__.py when
        passed the directory.

        Returns a tuple (None /* Option */_or_tests_from_file, should_recurse).
        ";
        basename = os . path . basename ( full_path );
        if os . path . isfile ( full_path ) {
        if !VALID_MODULE_NAME . match ( basename ) {
        return  None /* Option */ , false;
        if !self . _match_path ( basename , full_path , pattern ) {
        return  None /* Option */ , false;
        name = self . _get_name_from_path ( full_path );
        // try {
        module = self . _get_module_from_name ( name );
        // } catch  case . SkipTest as e  {
        return  _make_skipped_test ( name , e , self . suiteClass ) , false;
        // } catch   {
        error_case , error_message = \;
        _make_failed_import_test ( name , self . suiteClass );
        self . errors . append ( error_message );
        return  error_case , false;
        } else {
        mod_file = os . path . abspath (;
        getattr ( module , "__file__" , full_path ) );
        realpath = _jython_aware_splitext (;
        os . path . realpath ( mod_file ) );
        fullpath_noext = _jython_aware_splitext (;
        os . path . realpath ( full_path ) );
        if realpath . lower ( ) != fullpath_noext . lower ( ) {
        module_dir = os . path . dirname ( realpath );
        mod_name = _jython_aware_splitext (;
        os . path . basename ( full_path ) );
        expected_dir = os . path . dirname ( full_path );
        msg = ( "%r module incorrectly imported from %r. Expected ";
        "%r. Is this module globally installed?" );
        panic!("ImportError (");
        msg % ( mod_name , module_dir , expected_dir ) );
        return  self . loadTestsFromModule ( module , pattern = pattern ) , false;
        } else if os . path . isdir ( full_path ) {
        if !os . path . isfile ( os . path . join ( full_path , "__init__.py" ) ) {
        return  None /* Option */ , false;
        load_tests = None /* Option */;
        tests = None /* Option */;
        name = self . _get_name_from_path ( full_path );
        // try {
        package = self . _get_module_from_name ( name );
        // } catch  case . SkipTest as e  {
        return  _make_skipped_test ( name , e , self . suiteClass ) , false;
        // } catch   {
        error_case , error_message = \;
        _make_failed_import_test ( name , self . suiteClass );
        self . errors . append ( error_message );
        return  error_case , false;
        } else {
        load_tests = getattr ( package , "load_tests" , None /* Option */ );
        self . _loading_packages . add ( name );
        // try {
        tests = self . loadTestsFromModule ( package , pattern = pattern );
        if load_tests is !None /* Option */ {
        return  tests , false;
        return  tests , true;
        // } finally {
        self . _loading_packages . discard ( name );
        } else {
        return  None /* Option */ , false;
        defaultTestLoader = TestLoader ( );
        pub fn _makeLoader ( prefix , sortUsing , suiteClass = None /* Option */ , testNamePatterns = None /* Option */ )  {
        loader = TestLoader ( );
        loader . sortTestMethodsUsing = sortUsing;
        loader . testMethodPrefix = prefix;
        loader . testNamePatterns = testNamePatterns;
        if suiteClass {
        loader . suiteClass = suiteClass;
        return  loader;
        pub fn getTestCaseNames ( testCaseClass , prefix , sortUsing = util . three_way_cmp , testNamePatterns = None /* Option */ )  {
        import warnings;
        warnings . warn (;
        "unittest.getTestCaseNames() == deprecated && will be removed in Python 3.13. ";
        "Please use unittest.TestLoader.getTestCaseNames() instead." ,;
        DeprecationWarning , stacklevel = 2;
        );
        return  _makeLoader ( prefix , sortUsing , testNamePatterns = testNamePatterns ) . getTestCaseNames ( testCaseClass );
        pub fn makeSuite ( testCaseClass , prefix = "test" , sortUsing = util . three_way_cmp , {
        suiteClass = suite . TestSuite ) ;
        import warnings;
        warnings . warn (;
        "unittest.makeSuite() == deprecated && will be removed in Python 3.13. ";
        "Please use unittest.TestLoader.loadTestsFromTestCase() instead." ,;
        DeprecationWarning , stacklevel = 2;
        );
        return  _makeLoader ( prefix , sortUsing , suiteClass ) . loadTestsFromTestCase (;
        testCaseClass );
        pub fn findTestCases ( module , prefix = "test" , sortUsing = util . three_way_cmp , {
        suiteClass = suite . TestSuite ) ;
        import warnings;
        warnings . warn (;
        "unittest.findTestCases() == deprecated && will be removed in Python 3.13. ";
        "Please use unittest.TestLoader.loadTestsFromModule() instead." ,;
        DeprecationWarning , stacklevel = 2;
        );
        return  _makeLoader ( prefix , sortUsing , suiteClass ) . loadTestsFromModule ( \;
        module );
}

