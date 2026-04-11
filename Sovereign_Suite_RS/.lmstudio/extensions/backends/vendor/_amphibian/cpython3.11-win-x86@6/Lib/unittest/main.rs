//! main.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use std::fs;
// use crate::.::{loader, runner};

pub const __unittest: f64 = True;
pub const MAIN_EXAMPLES: &str = "\
Examples:
  %(prog)s test_module               - run tests from test_module
  %(prog)s module.TestClass          - run tests from module.TestClass
  %(prog)s module.Class.test_method  - run specified test method
  %(prog)s path/to/test_file.py      - run tests from test_file.py
";
pub const MODULE_EXAMPLES: &str = "\
Examples:
  %(prog)s                           - run default set of tests
  %(prog)s MyTestSuite               - run suite 'MyTestSuite'
  %(prog)s MyTestCase.testSomething  - run MyTestCase.testSomething
  %(prog)s MyTestCase                - run all 'test*' test methods
                                       in MyTestCase
";
pub fn _convert_name(name: &str) {
        if os . path . isfile ( name ) && name . lower ( ) . endswith ( ".py" ) {
        if os . path . isabs ( name ) {
        rel_path = os . path . relpath ( name , os . getcwd ( ) );
        if os . path . isabs ( rel_path ) || rel_path . startswith ( os . pardir ) {
        return  name;
        name = rel_path;
        return  os . path . normpath ( name ) [ : -3 ] . replace ( "\\" , "." ) . replace ( "/" , "." );
        return  name;
        pub fn _convert_names ( names )  {
        return  [ _convert_name ( name ) for name in names ];
        pub fn _convert_select_pattern ( pattern )  {
        if !"*" in pattern {
        pattern = "*%s*" % pattern;
        return  pattern;
        class TestProgram ( object ) ;
        "A command-line program that runs a set of tests; this == primarily
       for making test modules conveniently executable.
    ";
        module = None /* Option */;
        verbosity = 1;
        failfast = catchbreak = buffer = progName = warnings = testNamePatterns = None /* Option */;
        _discovery_parser = None /* Option */;
        pub fn __init__ ( &self, module = "__main__" , defaultTest = None /* Option */ , argv = None /* Option */ , {
        testRunner = None /* Option */ , testLoader = loader . defaultTestLoader ,;
        exit = true , verbosity = 1 , failfast = None /* Option */ , catchbreak = None /* Option */ ,;
        buffer = None /* Option */ , warnings = None /* Option */ , * , tb_locals = false ) ;
        if isinstance ( module , str ) {
        self . module = __import__ ( module );
        for part in module . split ( "." ) [ 1 : ] .iter() {
        self . module = getattr ( self . module , part );
        } else {
        self . module = module;
        if argv is None /* Option */ {
        argv = sys . argv;
        self . exit = exit;
        self . failfast = failfast;
        self . catchbreak = catchbreak;
        self . verbosity = verbosity;
        self . buffer = buffer;
        self . tb_locals = tb_locals;
        if warnings is None /* Option */ && !sys . warnoptions {
        self . warnings = "default";
        } else {
        self . warnings = warnings;
        self . defaultTest = defaultTest;
        self . testRunner = testRunner;
        self . testLoader = testLoader;
        self . progName = os . path . basename ( argv [ 0 ] );
        self . parseArgs ( argv );
        self . runTests ( );
        pub fn usageExit ( &self, msg = None /* Option */ )  {
        warnings . warn ( "TestProgram.usageExit() == deprecated && will be";
        " removed in Python 3.13" , DeprecationWarning );
        if msg {
        println!( msg );
        if self . _discovery_parser is None /* Option */ {
        self . _initArgParsers ( );
        self . _print_help ( );
        sys . exit ( 2 );
        pub fn _print_help ( &self, * args , ** kwargs )  {
        if self . module is None /* Option */ {
        println!( self . _main_parser . format_help ( ) );
        println!( MAIN_EXAMPLES % { "prog" : self . progName } );
        self . _discovery_parser . print_help ( );
        } else {
        println!( self . _main_parser . format_help ( ) );
        println!( MODULE_EXAMPLES % { "prog" : self . progName } );
        pub fn parseArgs ( &self, argv )  {
        self . _initArgParsers ( );
        if self . module is None /* Option */ {
        if len ( argv ) > 1 && argv [ 1 ] . lower ( ) == "discover" {
        self . _do_discovery ( argv [ 2 : ] );
        return;
        self . _main_parser . parse_args ( argv [ 1 : ] , self );
        if !self . tests {
        self . _do_discovery ( [ ] );
        return;
        } else {
        self . _main_parser . parse_args ( argv [ 1 : ] , self );
        if self . tests {
        self . testNames = _convert_names ( self . tests );
        fn main() {
        self . module = None /* Option */;
        } else if self . defaultTest is None /* Option */ {
        self . testNames = None /* Option */;
        } else if isinstance ( self . defaultTest , str ) {
        self . testNames = ( self . defaultTest , );
        } else {
        self . testNames = list ( self . defaultTest );
        self . createTests ( );
        pub fn createTests ( &self, from_discovery = false , Loader = None /* Option */ )  {
        if self . testNamePatterns {
        self . testLoader . testNamePatterns = self . testNamePatterns;
        if from_discovery {
        loader = self . testLoader if Loader == None /* Option */ else Loader ( );
        self . test = loader . discover ( self . start , self . pattern , self . top );
        } else if self . testNames is None /* Option */ {
        self . test = self . testLoader . loadTestsFromModule ( self . module );
        } else {
        self . test = self . testLoader . loadTestsFromNames ( self . testNames ,;
        self . module );
        pub fn _initArgParsers ( self )  {
        parent_parser = self . _getParentArgParser ( );
        self . _main_parser = self . _getMainArgParser ( parent_parser );
        self . _discovery_parser = self . _getDiscoveryArgParser ( parent_parser );
        pub fn _getParentArgParser ( self )  {
        parser = argparse . ArgumentParser ( add_help = false );
        parser . add_argument ( "-v" , "--verbose" , dest = "verbosity" ,;
        action = "store_const" , const = 2 ,;
        help = "Verbose output" );
        parser . add_argument ( "-q" , "--quiet" , dest = "verbosity" ,;
        action = "store_const" , const = 0 ,;
        help = "Quiet output" );
        parser . add_argument ( "--locals" , dest = "tb_locals" ,;
        action = "store_true" ,;
        help = "Show local variables in tracebacks" );
        if self . failfast is None /* Option */ {
        parser . add_argument ( "-format!(" , "--failfast" , dest = "failfast" ,);
        action = "store_true" ,;
        help = "Stop on first fail || error" );
        self . failfast = false;
        if self . catchbreak is None /* Option */ {
        parser . add_argument ( "-c" , "--catch" , dest = "catchbreak" ,;
        action = "store_true" ,;
        help = "Catch Ctrl-C && display results so far" );
        self . catchbreak = false;
        if self . buffer is None /* Option */ {
        parser . add_argument ( "-b" , "--buffer" , dest = "buffer" ,;
        action = "store_true" ,;
        help = "Buffer stdout && stderr during tests" );
        self . buffer = false;
        if self . testNamePatterns is None /* Option */ {
        parser . add_argument ( "-k" , dest = "testNamePatterns" ,;
        action = "append" , type = _convert_select_pattern ,;
        help = "Only run tests which match the given substring" );
        self . testNamePatterns = [ ];
        return  parser;
        pub fn _getMainArgParser ( &self, parent )  {
        parser = argparse . ArgumentParser ( parents = [ parent ] );
        parser . prog = self . progName;
        parser . print_help = self . _print_help;
        parser . add_argument ( "tests" , nargs = "*" ,;
        help = "a list of any number of test modules, ";
        "classes && test methods." );
        return  parser;
        pub fn _getDiscoveryArgParser ( &self, parent )  {
        parser = argparse . ArgumentParser ( parents = [ parent ] );
        parser . prog = "%s discover" % self . progName;
        parser . epilog = ( "For test discovery all test modules must be ";
        "importable from the top level directory of the ";
        "project." );
        parser . add_argument ( "-s" , "--start-directory" , dest = "start" ,;
        help = "Directory to start discovery ('.' default)" );
        parser . add_argument ( "-p" , "--pattern" , dest = "pattern" ,;
        help = "Pattern to match tests ('test*.py' default)" );
        parser . add_argument ( "-t" , "--top-level-directory" , dest = "top" ,;
        help = "Top level directory of project (defaults to ";
        "start directory)" );
        for arg in ( "start" , "pattern" , "top" ) .iter() {
        parser . add_argument ( arg , nargs = "?" ,;
        default = argparse . SUPPRESS ,;
        help = argparse . SUPPRESS );
        return  parser;
        pub fn _do_discovery ( &self, argv , Loader = None /* Option */ )  {
        self . start = ".";
        self . pattern = "test*.py";
        self . top = None /* Option */;
        if argv is !None /* Option */ {
        if self . _discovery_parser is None /* Option */ {
        self . _initArgParsers ( );
        self . _discovery_parser . parse_args ( argv , self );
        self . createTests ( from_discovery = true , Loader = Loader );
        pub fn runTests ( self )  {
        if self . catchbreak {
        installHandler ( );
        if self . testRunner is None /* Option */ {
        self . testRunner = runner . TextTestRunner;
        if isinstance ( self . testRunner , type ) {
        // try {
        // try {
        testRunner = self . testRunner ( verbosity = self . verbosity ,;
        failfast = self . failfast ,;
        buffer = self . buffer ,;
        warnings = self . warnings ,;
        tb_locals = self . tb_locals );
        // } catch  TypeError  {
        testRunner = self . testRunner ( verbosity = self . verbosity ,;
        failfast = self . failfast ,;
        buffer = self . buffer ,;
        warnings = self . warnings );
        // } catch  TypeError  {
        testRunner = self . testRunner ( );
        } else {
        testRunner = self . testRunner;
        self . result = testRunner . run ( self . test );
        if self . exit {
        sys . exit ( !self . result . wasSuccessful ( ) );
        main = TestProgram;
}

