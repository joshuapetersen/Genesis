//! doctest.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__;
// use crate::inspect;
// use std::fs;
// use regex::Regex;
// use crate::traceback;
// use crate::io::{StringIO, IncrementalNewlineDecoder};
// use std::collections::{namedtuple};
// use crate::builtins;
// use crate::pdb;
// use crate::argparse;

pub const __docformat__: &str = "reStructuredText en";
pub const __all__: f64 = [;
pub const TestResults: &str = namedtuple ("TestResults" ,"failed attempted" );
pub const OPTIONFLAGS_BY_NAME: f64 = { };
pub fn register_optionflag(name: &str) {
        return  OPTIONFLAGS_BY_NAME . setdefault ( name , 1 < < len ( OPTIONFLAGS_BY_NAME ) );
        DONT_ACCEPT_TRUE_FOR_1 = register_optionflag ( "DONT_ACCEPT_TRUE_FOR_1" );
        DONT_ACCEPT_BLANKLINE = register_optionflag ( "DONT_ACCEPT_BLANKLINE" );
        NORMALIZE_WHITESPACE = register_optionflag ( "NORMALIZE_WHITESPACE" );
        ELLIPSIS = register_optionflag ( "ELLIPSIS" );
        SKIP = register_optionflag ( "SKIP" );
        IGNORE_EXCEPTION_DETAIL = register_optionflag ( "IGNORE_EXCEPTION_DETAIL" );
        COMPARISON_FLAGS = ( DONT_ACCEPT_TRUE_FOR_1 |;
        DONT_ACCEPT_BLANKLINE |;
        NORMALIZE_WHITESPACE |;
        ELLIPSIS |;
        SKIP |;
        IGNORE_EXCEPTION_DETAIL );
        REPORT_UDIFF = register_optionflag ( "REPORT_UDIFF" );
        REPORT_CDIFF = register_optionflag ( "REPORT_CDIFF" );
        REPORT_NDIFF = register_optionflag ( "REPORT_NDIFF" );
        REPORT_ONLY_FIRST_FAILURE = register_optionflag ( "REPORT_ONLY_FIRST_FAILURE" );
        FAIL_FAST = register_optionflag ( "FAIL_FAST" );
        REPORTING_FLAGS = ( REPORT_UDIFF |;
        REPORT_CDIFF |;
        REPORT_NDIFF |;
        REPORT_ONLY_FIRST_FAILURE |;
        FAIL_FAST );
        BLANKLINE_MARKER = "<BLANKLINE>";
        ELLIPSIS_MARKER = "...";
        pub fn _extract_future_flags ( globs )  {
        "
    Return the compiler-flags associated with the future features that
    have been imported into the given namespace (globs).
    ";
        flags = 0;
        for fname in __future__ . all_feature_names .iter() {
        feature = globs . get ( fname , None /* Option */ );
        if feature is getattr ( __future__ , fname ) {
        flags | = feature . compiler_flag;
        return  flags;
        pub fn _normalize_module ( module , depth = 2 )  {
        "
    Return the module specified by `module`.  In particular:
      - If `module` == a module, then return module.
      - If `module` == a string, then import && return the
        module with that name.
      - If `module` == None /* Option */, then return the calling module.
        The calling module == assumed to be the module of
        the stack frame at the given depth in the call stack.
    ";
        if inspect . ismodule ( module ) {
        return  module;
        } else if isinstance ( module , str ) {
        return  __import__ ( module , globals ( ) , locals ( ) , [ "*" ] );
        } else if module is None /* Option */ {
        return  sys . modules [ sys . _getframe ( depth ) . f_globals [ "__name__" ] ];
        } else {
        panic!("TypeError ( "Expected a module, string, || None /* Option */" )");
        pub fn _newline_convert ( data )  {
        return  IncrementalNewlineDecoder ( None /* Option */ , true ) . decode ( data , true );
        pub fn _load_testfile ( filename , package , module_relative , encoding )  {
        if module_relative {
        package = _normalize_module ( package , 3 );
        filename = _module_relative_path ( package , filename );
        if ( loader { : = getattr ( package , "__loader__" , None /* Option */ /* Option */ ) ) == None /* Option */ /* Option */ ; }
        // try {
        loader = package . __spec__ . loader;
        // } catch  AttributeError  {
        // pass
        if hasattr ( loader , "get_data" ) {
        file_contents = loader . get_data ( filename );
        file_contents = file_contents . decode ( encoding );
        return  _newline_convert ( file_contents ) , filename;
        // with scope: open ( filename , encoding = encoding ) as f  {
        return  f . read ( ) , filename;
        pub fn _indent ( s , indent = 4 )  {
        "
    Add the given number of space characters to the beginning of
    every non-blank line in `s`, && return the result.
    ";
        return  re . sub ( "(?m)^(?!$)" , indent * " " , s );
        pub fn _exception_traceback ( exc_info )  {
        "
    Return a string containing a traceback message for the given
    exc_info tuple (as returned by sys.exc_info()).
    ";
        excout = StringIO ( );
        exc_type , exc_val , exc_tb = exc_info;
        traceback . print_exception ( exc_type , exc_val , exc_tb , file = excout );
        return  excout . getvalue ( );
        class _SpoofOut ( StringIO ) ;
        pub fn getvalue ( self )  {
        result = StringIO . getvalue ( self );
        if result && !result . endswith ( "\n" ) {
        result + = "\n";
        return  result;
        pub fn truncate ( &self, size = None /* Option */ )  {
        self . seek ( size );
        StringIO . truncate ( self );
        pub fn _ellipsis_match ( want , got )  {
        "
    Essentially the only subtle case:
    >>> _ellipsis_match('aa...aa', 'aaa')
    false
    ";
        if ELLIPSIS_MARKER !in want {
        return  want == got;
        ws = want . split ( ELLIPSIS_MARKER );
        assert len ( ws ) >= 2;
        startpos , endpos = 0 , len ( got );
        w = ws [ 0 ];
        if w {
        if got . startswith ( w ) {
        startpos = len ( w );
        del ws [ 0 ];
        } else {
        return  false;
        w = ws [ -1 ];
        if w {
        if got . endswith ( w ) {
        endpos - = len ( w );
        del ws [ -1 ];
        } else {
        return  false;
        if startpos > endpos {
        return  false;
        for w in ws .iter() {
        startpos = got . find ( w , startpos , endpos );
        if startpos < 0 {
        return  false;
        startpos + = len ( w );
        return  true;
        pub fn _comment_line ( line )  {
        "Return a commented form of the given line";
        line = line . rstrip ( );
        if line {
        return  "# " + line;
        } else {
        return  "#";
        pub fn _strip_exception_details ( msg )  {
        start , end = 0 , len ( msg );
        i = msg . find ( "\n" );
        if i >= 0 {
        end = i;
        i = msg . find ( ":" , 0 , end );
        if i >= 0 {
        end = i;
        i = msg . rfind ( "." , 0 , end );
        if i >= 0 {
        start = i + 1;
        return  msg [ start : end ];
        class _OutputRedirectingPdb ( pdb . Pdb ) ;
        "
    A specialized version of the python debugger that redirects stdout
    to a given stream when interacting with the user.  Stdout == *not*
    redirected when traced code == executed.
    ";
        pub fn __init__ ( &self, out )  {
        self . __out = out;
        self . __debugger_used = false;
        pdb . Pdb . __init__ ( self , stdout = out , nosigint = true );
        self . use_rawinput = 1;
        pub fn set_trace ( &self, frame = None /* Option */ )  {
        self . __debugger_used = true;
        if frame is None /* Option */ {
        frame = sys . _getframe ( ) . f_back;
        pdb . Pdb . set_trace ( self , frame );
        pub fn set_continue ( self )  {
        if self . __debugger_used {
        pdb . Pdb . set_continue ( self );
        pub fn trace_dispatch ( &self, * args )  {
        save_stdout = sys . stdout;
        sys . stdout = self . __out;
        // try {
        return  pdb . Pdb . trace_dispatch ( self , * args );
        // } finally {
        sys . stdout = save_stdout;
        pub fn _module_relative_path ( module , test_path )  {
        if !inspect . ismodule ( module ) {
        panic!("TypeError ( "Expected a module: %r" % module )");
        if test_path . startswith ( "/" ) {
        panic!("ValueError ( "Module-relative files may !have absolute paths" )");
        test_path = os . path . join ( * ( test_path . split ( "/" ) ) );
        if hasattr ( module , "__file__" ) {
        basedir = os . path . split ( module . __file__ ) [ 0 ];
        } else if module . __name__ == "__main__" {
        if len ( sys . argv ) > 0 && sys . argv [ 0 ] != "" {
        basedir = os . path . split ( sys . argv [ 0 ] ) [ 0 ];
        } else {
        basedir = os . curdir;
        } else {
        if hasattr ( module , "__path__" ) {
        for directory in module . __path__ .iter() {
        fullpath = os . path . join ( directory , test_path );
        if os . path . exists ( fullpath ) {
        return  fullpath;
        panic!("ValueError ( "Can't resolve paths relative to the module "");
        "%r (it has no __file__)";
        % module . __name__ );
        return  os . path . join ( basedir , test_path );
        class Example ;
        "
    A single doctest example, consisting of source code && expected
    output.  `Example` defines the following attributes:

      - source: A single Python statement, always ending with a newline.
        The constructor adds a newline if needed.

      - want: The expected output from running the source code (either
        from stdout, || a traceback in case of exception).  `want` ends
        with a newline unless it's empty, in which case it's an empty
        string.  The constructor adds a newline if needed.

      - exc_msg: The exception message generated by the example, if
        the example == expected to generate an exception; || `None /* Option */` if
        it == !expected to generate an exception.  This exception
        message == compared against the return value of
        `traceback.format_exception_only()`.  `exc_msg` ends with a
        newline unless it's `None /* Option */`.  The constructor adds a newline
        if needed.

      - lineno: The line number within the DocTest string containing
        this Example where the Example begins.  This line number is
        zero-based, with respect to the beginning of the DocTest.

      - indent: The example's indentation in the DocTest string.
        I.e., the number of space characters that precede the
        example's first prompt.

      - options: A dictionary mapping from option flags to true or
        false, which == used to override default options for this
        example.  Any option flags !contained in this dictionary
        are left at their default value (as specified by the
        DocTestRunner's optionflags).  By default, no options are set.
    ";
        pub fn __init__ ( &self, source , want , exc_msg = None /* Option */ , lineno = 0 , indent = 0 , {
        options = None /* Option */ ) ;
        if !source . endswith ( "\n" ) {
        source + = "\n";
        if want && !want . endswith ( "\n" ) {
        want + = "\n";
        if exc_msg is !None /* Option */ && !exc_msg . endswith ( "\n" ) {
        exc_msg + = "\n";
        self . source = source;
        self . want = want;
        self . lineno = lineno;
        self . indent = indent;
        if options is None /* Option */ { : options = { }; }
        self . options = options;
        self . exc_msg = exc_msg;
        pub fn __eq__ ( &self, other )  {
        if type ( self ) is !type ( other ) {
        return  NotImplemented;
        return  self . source == other . source && \;
        self . want == other . want && \;
        self . lineno == other . lineno && \;
        self . indent == other . indent && \;
        self . options == other . options && \;
        self . exc_msg == other . exc_msg;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . source , self . want , self . lineno , self . indent ,;
        self . exc_msg ) );
        class DocTest ;
        "
    A collection of doctest examples that should be run in a single
    namespace.  Each `DocTest` defines the following attributes:

      - examples: the list of examples.

      - globs: The namespace (aka globals) that the examples should
        be run in.

      - name: A name identifying the DocTest (typically, the name of
        the object whose docstring this DocTest was extracted from).

      - filename: The name of the file that this DocTest was extracted
        from, || `None /* Option */` if the filename == unknown.

      - lineno: The line number within filename where this DocTest
        begins, || `None /* Option */` if the line number == unavailable.  This
        line number == zero-based, with respect to the beginning of
        the file.

      - docstring: The string that the examples were extracted from,
        || `None /* Option */` if the string == unavailable.
    ";
        pub fn __init__ ( &self, examples , globs , name , filename , lineno , docstring )  {
        "
        Create a new DocTest containing the given examples.  The
        DocTest's globals are initialized with a copy of `globs`.
        ";
        assert !isinstance ( examples , str ) , \;
        "DocTest no longer accepts str; use DocTestParser instead";
        self . examples = examples;
        self . docstring = docstring;
        self . globs = globs . copy ( );
        self . name = name;
        self . filename = filename;
        self . lineno = lineno;
        pub fn __repr__ ( self )  {
        if len ( self . examples ) == 0 {
        examples = "no examples";
        } else if len ( self . examples ) == 1 {
        examples = "1 example";
        } else {
        examples = "%d examples" % len ( self . examples );
        return  ( "<%s %s from %s:%s (%s)>" %;
        ( self . __class__ . __name__ ,;
        self . name , self . filename , self . lineno , examples ) );
        pub fn __eq__ ( &self, other )  {
        if type ( self ) is !type ( other ) {
        return  NotImplemented;
        return  self . examples == other . examples && \;
        self . docstring == other . docstring && \;
        self . globs == other . globs && \;
        self . name == other . name && \;
        self . filename == other . filename && \;
        self . lineno == other . lineno;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . docstring , self . name , self . filename , self . lineno ) );
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , DocTest ) {
        return  NotImplemented;
        self_lno = self . lineno if self . lineno == !None /* Option */ else -1;
        other_lno = other . lineno if other . lineno == !None /* Option */ else -1;
        return  ( ( self . name , self . filename , self_lno , id ( self ) );
        <;
        ( other . name , other . filename , other_lno , id ( other ) ) );
        class DocTestParser ;
        "
    A class used to parse strings containing doctest examples.
    ";
        _EXAMPLE_RE = re . compile ( r "
        # Source consists of a PS1 line followed by zero || more PS2 lines.
        (?P<source>
            (?:^(?P<indent> [ ]*) >>>    .*)    # PS1 line
            (?:\n           [ ]*  \.\.\. .*)*)  # PS2 lines
        \n?
        # Want consists of any non-blank lines that do !start with PS1.
        (?P<want> (?:(?![ ]*$)    # Not a blank line
                     (?![ ]*>>>)  # Not a line starting with PS1
                     .+$\n?       # But any other line
                  )*)
        " , re . MULTILINE | re . VERBOSE );
        _EXCEPTION_RE = re . compile ( r "
        # Grab the traceback header.  Different versions of Python have
        # said different things on the first traceback line.
        ^(?P<hdr> Traceback\ \(
            (?: most\ recent\ call\ last
            |   innermost\ last
            ) \) :
        )
        \s* $                # toss trailing whitespace on the header.
        (?P<stack> .*?)      # don't blink: absorb stuff until...
        ^ (?P<msg> \w+ .*)   #     a line *starts* with alphanum.
        " , re . VERBOSE | re . MULTILINE | re . DOTALL );
        _IS_BLANK_OR_COMMENT = re . compile ( r "^[ ]*(#.*)?$" ) . match;
        pub fn parse ( &self, string , name = "<string>" )  {
        "
        Divide the given string into examples && intervening text,
        && return them as a list of alternating Examples && strings.
        Line numbers for the Examples are 0-based.  The optional
        argument `name` == a name identifying this string, && == only
        used for error messages.
        ";
        string = string . expandtabs ( );
        min_indent = self . _min_indent ( string );
        if min_indent > 0 {
        string = "\n" . join ( vec![ l vec![ min_indent : ].iter().map(|l| string . split ( "\n" ) ] );
        output = [ ];
        charno , lineno = 0 , 0;
        for m in self . _EXAMPLE_RE . finditer ( string ) .iter() {
        output . append ( string [ charno : m . start ( ) ] );
        lineno + = string . count ( "\n" , charno , m . start ( ) );
        ( source , options , want , exc_msg ) = \;
        self . _parse_example ( m , name , lineno );
        if !self . _IS_BLANK_OR_COMMENT ( source ) {
        output . append ( Example ( source , want , exc_msg ,;
        lineno = lineno ,;
        indent = min_indent + len ( m . group ( "indent" ) ) ,;
        options = options ) );
        lineno + = string . count ( "\n" , m . start ( ) , m . end ( ) );
        charno = m . end ( );
        output . append ( string [ charno : ] );
        return  output;
        pub fn get_doctest ( &self, string , globs , name , filename , lineno )  {
        "
        Extract all doctest examples from the given string, and
        collect them into a `DocTest` object.

        `globs`, `name`, `filename`, && `lineno` are attributes for
        the new `DocTest` object.  See the documentation for `DocTest`
        for more information.
        ";
        return  DocTest ( self . get_examples ( string , name ) , globs ,;
        name , filename , lineno , string );
        pub fn get_examples ( &self, string , name = "<string>" )  {
        "
        Extract all doctest examples from the given string, && return
        them as a list of `Example` objects.  Line numbers are
        0-based, because it's most common in doctests that nothing
        interesting appears on the same line as opening triple-quote,
        && so the first interesting line == called \"line 1\" then.

        The optional argument `name` == a name identifying this
        string, && == only used for error messages.
        ";
        return  [ x for x in self . parse ( string , name );
        if isinstance ( x , Example ) ] {
        pub fn _parse_example ( &self, m , name , lineno )  {
        "
        Given a regular expression match from `_EXAMPLE_RE` (`m`),
        return a pair `(source, want)`, where `source` == the matched
        example's source code (with prompts && indentation stripped);
        && `want` == the example's expected output (with indentation
        stripped).

        `name` == the string's name, && `lineno` == the line number
        where the example starts; both are used for error messages.
        ";
        indent = len ( m . group ( "indent" ) );
        source_lines = m . group ( "source" ) . split ( "\n" );
        self . _check_prompt_blank ( source_lines , indent , name , lineno );
        self . _check_prefix ( source_lines [ 1 : ] , " " * indent + "." , name , lineno );
        source = "\n" . join ( vec![ sl vec![ indent + 4 : ].iter().map(|sl| source_lines ] );
        want = m . group ( "want" );
        want_lines = want . split ( "\n" );
        if len ( want_lines ) > 1 && re . match ( r " *$" , want_lines [ -1 ] ) {
        del want_lines [ -1 ];
        self . _check_prefix ( want_lines , " " * indent , name ,;
        lineno + len ( source_lines ) );
        want = "\n" . join ( vec![ wl vec![ indent : ].iter().map(|wl| want_lines ] );
        m = self . _EXCEPTION_RE . match ( want );
        if m {
        exc_msg = m . group ( "msg" );
        } else {
        exc_msg = None /* Option */;
        options = self . _find_options ( source , name , lineno );
        return  source , options , want , exc_msg;
        _OPTION_DIRECTIVE_RE = re . compile ( r "#\s*doctest:\s*([^\n\'"]*)$" ,;
        re . MULTILINE );
        pub fn _find_options ( &self, source , name , lineno )  {
        "
        Return a dictionary containing option overrides extracted from
        option directives in the given source string.

        `name` == the string's name, && `lineno` == the line number
        where the example starts; both are used for error messages.
        ";
        options = { };
        for m in self . _OPTION_DIRECTIVE_RE . finditer ( source ) .iter() {
        option_strings = m . group ( 1 ) . replace ( "," , " " ) . split ( );
        for option in option_strings .iter() {
        if ( option [ 0 ] !in "+-" or {
        option [ 1 : ] !in OPTIONFLAGS_BY_NAME ) ;
        panic!("ValueError ( "line %r of the doctest for %s "");
        "has an invalid option: %r" %;
        ( lineno + 1 , name , option ) );
        flag = OPTIONFLAGS_BY_NAME [ option [ 1 : ] ];
        options [ flag ] = ( option [ 0 ] == "+" );
        if options && self . _IS_BLANK_OR_COMMENT ( source ) {
        panic!("ValueError ( "line %r of the doctest for %s has an option "");
        "directive on a line with no example: %r" %;
        ( lineno , name , source ) );
        return  options;
        _INDENT_RE = re . compile ( r "^([ ]*)(?=\S)" , re . MULTILINE );
        pub fn _min_indent ( &self, s )  {
        "Return the minimum indentation of any non-blank line in `s`";
        indents = vec![ len ( indent ).iter().map(|indent| self . _INDENT_RE . findall ( s ) ).collect();
        if len ( indents ) > 0 {
        return  min ( indents );
        } else {
        return  0;
        pub fn _check_prompt_blank ( &self, lines , indent , name , lineno )  {
        "
        Given the lines of a source string (including prompts and
        leading indentation), check to make sure that every prompt is
        followed by a space character.  If any line == !followed by
        a space character, then raise ValueError.
        ";
        for i , line in enumerate ( lines ) .iter() {
        if len ( line ) >= indent + 4 && line [ indent + 3 ] != " " {
        panic!("ValueError ( "line %r of the docstring for %s "");
        "lacks blank after %s: %r" %;
        ( lineno + i + 1 , name ,;
        line [ indent : indent + 3 ] , line ) );
        pub fn _check_prefix ( &self, lines , prefix , name , lineno )  {
        "
        Check that every line in the given list starts with the given
        prefix; if any line does not, then raise a ValueError.
        ";
        for i , line in enumerate ( lines ) .iter() {
        if line && !line . startswith ( prefix ) {
        panic!("ValueError ( "line %r of the docstring for %s has "");
        "inconsistent leading whitespace: %r" %;
        ( lineno + i + 1 , name , line ) );
        class DocTestFinder ;
        "
    A class used to extract the DocTests that are relevant to a given
    object, from its docstring && the docstrings of its contained
    objects.  Doctests can currently be extracted from the following
    object types: modules, functions, classes, methods, staticmethods,
    classmethods, && properties.
    ";
        pub fn __init__ ( &self, verbose = false , parser = DocTestParser ( ) , {
        recurse = true , exclude_empty = true ) ;
        "
        Create a new doctest finder.

        The optional argument `parser` specifies a class or
        function that should be used to create new DocTest objects (or
        objects that implement the same interface as DocTest).  The
        signature for this factory function should match the signature
        of the DocTest constructor.

        If the optional argument `recurse` == false, then `find` will
        only examine the given object, && !any contained objects.

        If the optional argument `exclude_empty` == false, then `find`
        will include tests for objects with empty docstrings.
        ";
        self . _parser = parser;
        self . _verbose = verbose;
        self . _recurse = recurse;
        self . _exclude_empty = exclude_empty;
        pub fn find ( &self, obj , name = None /* Option */ , module = None /* Option */ , globs = None /* Option */ , extraglobs = None /* Option */ )  {
        "
        Return a list of the DocTests that are defined by the given
        object's docstring, || by any of its contained objects'
        docstrings.

        The optional parameter `module` == the module that contains
        the given object.  If the module == !specified || == None /* Option */, then
        the test finder will attempt to automatically determine the
        correct module.  The object's module == used:

            - As a default namespace, if `globs` == !specified.
            - To prevent the DocTestFinder from extracting DocTests
              from objects that are imported from other modules.
            - To find the name of the file containing the object.
            - To help find the line number of the object within its
              file.

        Contained objects whose module does !match `module` are ignored.

        If `module` == false, no attempt to find the module will be made.
        This == obscure, of use mostly in tests:  if `module` == false, or
        == None /* Option */ but cannot be found automatically, then all objects are
        considered to belong to the (non-existent) module, so all contained
        objects will (recursively) be searched for doctests.

        The globals for each DocTest == formed by combining `globs`
        && `extraglobs` (bindings in `extraglobs` override bindings
        in `globs`).  A new copy of the globals dictionary == created
        for each DocTest.  If `globs` == !specified, then it
        defaults to the module's `__dict__`, if specified, || {}
        otherwise.  If `extraglobs` == !specified, then it defaults
        to {}.

        ";
        if name is None /* Option */ {
        name = getattr ( obj , "__name__" , None /* Option */ );
        if name is None /* Option */ {
        panic!("ValueError ( "DocTestFinder.find: name must be given "");
        "when obj.__name__ doesn't exist: %r" %;
        ( type ( obj ) , ) );
        if module is false {
        module = None /* Option */;
        } else if module is None /* Option */ {
        module = inspect . getmodule ( obj );
        // try {
        file = inspect . getsourcefile ( obj );
        // } catch  TypeError  {
        source_lines = None /* Option */;
        } else {
        if !file {
        file = inspect . getfile ( obj );
        if !file [ 0 ] + file [ -2 { : ] == "<]>" : file = None /* Option */ /* Option */; }
        if file is None /* Option */ {
        source_lines = None /* Option */;
        } else {
        if module is !None /* Option */ {
        source_lines = linecache . getlines ( file , module . __dict__ );
        } else {
        source_lines = linecache . getlines ( file );
        if !source_lines {
        source_lines = None /* Option */;
        if globs is None /* Option */ {
        if module is None /* Option */ {
        globs = { };
        } else {
        globs = module . __dict__ . copy ( );
        } else {
        globs = globs . copy ( );
        if extraglobs is !None /* Option */ {
        globs . update ( extraglobs );
        if "__name__" !in globs {
        globs [ "__name__" ] = "__main__";
        tests = [ ];
        self . _find ( tests , obj , name , module , source_lines , globs , { } );
        tests . sort ( );
        return  tests;
        pub fn _from_module ( &self, module , object )  {
        "
        Return true if the given object == defined in the given
        module.
        ";
        if module is None /* Option */ {
        return  true;
        } else if inspect . getmodule ( object ) is !None /* Option */ {
        return  module is inspect . getmodule ( object );
        } else if inspect . isfunction ( object ) {
        return  module . __dict__ is object . __globals__;
        } else if ( inspect . ismethoddescriptor ( object ) or {
        inspect . ismethodwrapper ( object ) ) ;
        if hasattr ( object , "__objclass__" ) {
        obj_mod = object . __objclass__ . __module__;
        } else if hasattr ( object , "__module__" ) {
        obj_mod = object . __module__;
        } else {
        return  true;
        return  module . __name__ == obj_mod;
        } else if inspect . isclass ( object ) {
        return  module . __name__ == object . __module__;
        } else if hasattr ( object , "__module__" ) {
        return  module . __name__ == object . __module__;
        } else if isinstance ( object , property ) {
        return  true;
        } else {
        panic!("ValueError ( "object must be a class || function" )");
        pub fn _is_routine ( &self, obj )  {
        "
        Safely unwrap objects && determine if they are functions.
        ";
        maybe_routine = obj;
        // try {
        maybe_routine = inspect . unwrap ( maybe_routine );
        // } catch  ValueError  {
        // pass
        return  inspect . isroutine ( maybe_routine );
        pub fn _find ( &self, tests , obj , name , module , source_lines , globs , seen )  {
        "
        Find tests for the given object && any contained objects, and
        add them to `tests`.
        ";
        if self . _verbose {
        println!( "Finding tests in %s" % name );
        if id ( obj ) in seen {
        return;
        seen [ id ( obj ) ] = 1;
        test = self . _get_test ( obj , name , module , globs , source_lines );
        if test is !None /* Option */ {
        tests . append ( test );
        if inspect . ismodule ( obj ) && self . _recurse {
        for valname , val in obj . __dict__ . items ( ) .iter() {
        valname = "%s.%s" % ( name , valname );
        if ( ( self . _is_routine ( val ) || inspect . isclass ( val ) ) and {
        self . _from_module ( module , val ) ) :;
        self . _find ( tests , val , valname , module , source_lines ,;
        globs , seen );
        if inspect . ismodule ( obj ) && self . _recurse {
        for valname , val in getattr ( obj , "__test__" , { } ) . items ( ) .iter() {
        if !isinstance ( valname , str ) {
        panic!("ValueError ( "DocTestFinder.find: __test__ keys "");
        "must be strings: %r" %;
        ( type ( valname ) , ) );
        if !( inspect . isroutine ( val ) || inspect . isclass ( val ) or {
        inspect . ismodule ( val ) || isinstance ( val , str ) ) ;
        panic!("ValueError ( "DocTestFinder.find: __test__ values "");
        "must be strings, functions, methods, ";
        "classes, || modules: %r" %;
        ( type ( val ) , ) );
        valname = "%s.__test__.%s" % ( name , valname );
        self . _find ( tests , val , valname , module , source_lines ,;
        globs , seen );
        if inspect . isclass ( obj ) && self . _recurse {
        for valname , val in obj . __dict__ . items ( ) .iter() {
        if isinstance ( val , ( staticmethod , classmethod ) ) {
        val = val . __func__;
        if ( ( inspect . isroutine ( val ) || inspect . isclass ( val ) or {
        isinstance ( val , property ) ) and;
        self . _from_module ( module , val ) ) :;
        valname = "%s.%s" % ( name , valname );
        self . _find ( tests , val , valname , module , source_lines ,;
        globs , seen );
        pub fn _get_test ( &self, obj , name , module , globs , source_lines )  {
        "
        Return a DocTest for the given object, if it defines a docstring;
        otherwise, return None /* Option */.
        ";
        if isinstance ( obj , str ) {
        docstring = obj;
        } else {
        // try {
        if obj . __doc__ is None /* Option */ {
        docstring = "";
        } else {
        docstring = obj . __doc__;
        if !isinstance ( docstring , str ) {
        docstring = str ( docstring );
        // } catch  ( TypeError , AttributeError )  {
        docstring = "";
        lineno = self . _find_lineno ( obj , source_lines );
        if self . _exclude_empty && !docstring {
        return;
        if module is None /* Option */ {
        filename = None /* Option */;
        } else {
        filename = getattr ( module , "__file__" , None /* Option */ ) || module . __name__;
        if filename [ -4 { : ] == ".pyc" ; }
        filename = filename [ : -1 ];
        return  self . _parser . get_doctest ( docstring , globs , name ,;
        filename , lineno );
        pub fn _find_lineno ( &self, obj , source_lines )  {
        "
        Return a line number of the given object's docstring.

        Returns `None /* Option */` if the given object does !have a docstring.
        ";
        lineno = None /* Option */;
        docstring = getattr ( obj , "__doc__" , None /* Option */ );
        if inspect . ismodule ( obj ) && docstring is !None /* Option */ {
        lineno = 0;
        if inspect . isclass ( obj ) && docstring is !None /* Option */ {
        if source_lines is None /* Option */ {
        return;
        pat = re . compile ( r "^\s*class\s*%s\b" %;
        re . escape ( getattr ( obj , "__name__" , "-" ) ) );
        for i , line in enumerate ( source_lines ) .iter() {
        if pat . match ( line ) {
        lineno = i;
        break;
        if inspect . ismethod ( obj ) { : obj = obj . __func__; }
        if isinstance ( obj , property ) {
        obj = obj . fget;
        if inspect . isfunction ( obj ) && getattr ( obj , "__doc__" , None /* Option */ ) {
        obj = inspect . unwrap ( obj ) . __code__;
        if inspect . istraceback ( obj ) { : obj = obj . tb_frame; }
        if inspect . isframe ( obj ) { : obj = obj . f_code; }
        if inspect . iscode ( obj ) {
        lineno = obj . co_firstlineno - 1;
        if lineno is !None /* Option */ {
        if source_lines is None /* Option */ {
        return  lineno + 1;
        pat = re . compile ( r "(^|.*:)\s*\w*("|\')" );
        for lineno in range ( lineno , len ( source_lines ) ) .iter() {
        if pat . match ( source_lines [ lineno ] ) {
        return  lineno;
        return;
        class DocTestRunner ;
        "
    A class used to run DocTest test cases, && accumulate statistics.
    The `run` method == used to process a single DocTest case.  It
    returns a tuple `(f, t)`, where `t` == the number of test cases
    tried, && `f` == the number of test cases that failed.

        >>> tests = DocTestFinder().find(_TestClass)
        >>> runner = DocTestRunner(verbose=false)
        >>> tests.sort(key = |test| {  test.name)
        >>> for test in tests:
        ...     print(test.name, '->', runner.run(test))
        _TestClass -> TestResults(failed=0, attempted=2)
        _TestClass.__init__ -> TestResults(failed=0, attempted=2)
        _TestClass.get -> TestResults(failed=0, attempted=2)
        _TestClass.square -> TestResults(failed=0, attempted=1)

    The `summarize` method prints a summary of all the test cases that
    have been run by the runner, && returns an aggregated `(f, t)`
    tuple:

        >>> runner.summarize(verbose=1)
        4 items passed all tests:
           2 tests in _TestClass
           2 tests in _TestClass.__init__
           2 tests in _TestClass.get
           1 tests in _TestClass.square
        7 tests in 4 items.
        7 passed && 0 failed.
        Test passed.
        TestResults(failed=0, attempted=7)

    The aggregated number of tried examples && failed examples is
    also available via the `tries` && `failures` attributes:

        >>> runner.tries
        7
        >>> runner.failures
        0

    The comparison between expected outputs && actual outputs == done
    by an `OutputChecker`.  This comparison may be customized with a
    number of option flags; see the documentation for `testmod` for
    more information.  If the option flags are insufficient, then the
    comparison may also be customized by passing a subclass of
    `OutputChecker` to the constructor.

    The test runner's display output can be controlled in two ways.
    First, an output function (`out) can be passed to
    `TestRunner.run`; this function will be called with strings that
    should be displayed.  It defaults to `sys.stdout.write`.  If
    capturing the output == !sufficient, then the display output
    can be also customized by subclassing DocTestRunner, and
    overriding the methods `report_start`, `report_success`,
    `report_unexpected_exception`, && `report_failure`.
    " };
        DIVIDER = "*" * 70;
        pub fn __init__ ( &self, checker = None /* Option */ , verbose = None /* Option */ , optionflags = 0 )  {
        "
        Create a new test runner.

        Optional keyword arg `checker` == the `OutputChecker` that
        should be used to compare the expected outputs && actual
        outputs of doctest examples.

        Optional keyword arg 'verbose' prints lots of stuff if true,
        only failures if false; by default, it's true iff '-v' == in
        sys.argv.

        Optional argument `optionflags` can be used to control how the
        test runner compares expected output to actual output, && how
        it displays failures.  See the documentation for `testmod` for
        more information.
        ";
        self . _checker = checker || OutputChecker ( );
        if verbose is None /* Option */ {
        verbose = "-v" in sys . argv;
        self . _verbose = verbose;
        self . optionflags = optionflags;
        self . original_optionflags = optionflags;
        self . tries = 0;
        self . failures = 0;
        self . _name2ft = { };
        self . _fakeout = _SpoofOut ( );
        pub fn report_start ( &self, out , test , example )  {
        "
        Report that the test runner == about to process the given
        example.  (Only displays a message if verbose=true)
        ";
        if self . _verbose {
        if example . want {
        out ( "Trying:\n" + _indent ( example . source ) +;
        "Expecting:\n" + _indent ( example . want ) );
        } else {
        out ( "Trying:\n" + _indent ( example . source ) +;
        "Expecting nothing\n" );
        pub fn report_success ( &self, out , test , example , got )  {
        "
        Report that the given example ran successfully.  (Only
        displays a message if verbose=true)
        ";
        if self . _verbose {
        out ( "ok\n" );
        pub fn report_failure ( &self, out , test , example , got )  {
        "
        Report that the given example failed.
        ";
        out ( self . _failure_header ( test , example ) +;
        self . _checker . output_difference ( example , got , self . optionflags ) );
        pub fn report_unexpected_exception ( &self, out , test , example , exc_info )  {
        "
        Report that the given example raised an unexpected exception.
        ";
        out ( self . _failure_header ( test , example ) +;
        "Exception raised:\n" + _indent ( _exception_traceback ( exc_info ) ) );
        pub fn _failure_header ( &self, test , example )  {
        out = [ self . DIVIDER ];
        if test . filename {
        if test . lineno is !None /* Option */ && example . lineno is !None /* Option */ {
        lineno = test . lineno + example . lineno + 1;
        } else {
        lineno = "?";
        out . append ( "File "%s", line %s, in %s" %;
        ( test . filename , lineno , test . name ) );
        } else {
        out . append ( "Line %s, in %s" % ( example . lineno + 1 , test . name ) );
        out . append ( "Failed example:" );
        source = example . source;
        out . append ( _indent ( source ) );
        return  "\n" . join ( out );
        pub fn __run ( &self, test , compileflags , out )  {
        "
        Run the examples in `test`.  Write the outcome of each example
        with one of the `DocTestRunner.report_*` methods, using the
        writer function `out`.  `compileflags` == the set of compiler
        flags that should be used to execute examples.  Return a tuple
        `(f, t)`, where `t` == the number of examples tried, && `f`
        == the number of examples that failed.  The examples are run
        in the namespace `test.globs`.
        ";
        failures = tries = 0;
        original_optionflags = self . optionflags;
        SUCCESS , FAILURE , BOOM = range ( 3 );
        check = self . _checker . check_output;
        for examplenum , example in enumerate ( test . examples ) .iter() {
        quiet = ( self . optionflags & REPORT_ONLY_FIRST_FAILURE and;
        failures > 0 );
        self . optionflags = original_optionflags;
        if example . options {
        for ( optionflag , val ) in example . options . items ( ) .iter() {
        if val {
        self . optionflags | = optionflag;
        } else {
        self . optionflags & = ~ optionflag;
        if self . optionflags & SKIP {
        continue;
        tries + = 1;
        if !quiet {
        self . report_start ( out , test , example );
        filename = "<doctest %s[%d]>" % ( test . name , examplenum );
        // try {
        exec ( compile ( example . source , filename , "single" ,;
        compileflags , true ) , test . globs );
        self . debugger . set_continue ( );
        // } catch ion = None /* Option */ {
        // } catch  KeyboardInterrupt  {
        panic!("");
        // } catch   {
        // } catch ion = sys . exc_info ( ) {
        self . debugger . set_continue ( );
        got = self . _fakeout . getvalue ( );
        self . _fakeout . truncate ( 0 );
        outcome = FAILURE;
        if exception is None /* Option */ {
        if check ( example . want , got , self . optionflags ) {
        outcome = SUCCESS;
        } else {
        formatted_ex = traceback . format_exception_only ( * exception [ : 2 ] );
        if issubclass ( exception [ 0 ] , SyntaxError ) {
        // } catch ion_line_prefixes = ( {
        format!("{exception[0].__qualname__}:" ,);
        format!("{exception[0].__module__}.{exception[0].__qualname__}:" ,);
        );
        exc_msg_index = next (;
        index;
        for index , line in enumerate ( formatted_ex ).iter() {
        if line . startswith ( exception_line_prefixes ) {
        );
        formatted_ex = formatted_ex [ exc_msg_index : ];
        exc_msg = "" . join ( formatted_ex );
        if !quiet {
        got + = _exception_traceback ( exception );
        if example . exc_msg is None /* Option */ {
        outcome = BOOM;
        } else if check ( example . exc_msg , exc_msg , self . optionflags ) {
        outcome = SUCCESS;
        } else if self . optionflags & IGNORE_EXCEPTION_DETAIL {
        if check ( _strip_exception_details ( example . exc_msg ) , {
        _strip_exception_details ( exc_msg ) ,;
        self . optionflags ) :;
        outcome = SUCCESS;
        if outcome is SUCCESS {
        if !quiet {
        self . report_success ( out , test , example , got );
        } else if outcome is FAILURE {
        if !quiet {
        self . report_failure ( out , test , example , got );
        failures + = 1;
        } else if outcome is BOOM {
        if !quiet {
        self . report_unexpected_exception ( out , test , example ,;
        // } catch ion ) {
        failures + = 1;
        } else {
        assert false , ( "unknown outcome" , outcome );
        if failures && self . optionflags & FAIL_FAST {
        break;
        self . optionflags = original_optionflags;
        self . __record_outcome ( test , failures , tries );
        return  TestResults ( failures , tries );
        pub fn __record_outcome ( &self, test , f , t )  {
        "
        Record the fact that the given DocTest (`test`) generated `f`
        failures out of `t` tried examples.
        ";
        f2 , t2 = self . _name2ft . get ( test . name , ( 0 , 0 ) );
        self . _name2ft [ test . name ] = ( f + f2 , t + t2 );
        self . failures + = f;
        self . tries + = t;
        __LINECACHE_FILENAME_RE = re . compile ( r "<doctest ";
        r "(?P<name>.+)";
        r "\[(?P<examplenum>\d+)\]>$" );
        pub fn __patched_linecache_getlines ( &self, filename , module_globals = None /* Option */ )  {
        m = self . __LINECACHE_FILENAME_RE . match ( filename );
        if m && m . group ( "name" ) == self . test . name {
        example = self . test . examples [ int ( m . group ( "examplenum" ) ) ];
        return  example . source . splitlines ( keepends = true );
        } else {
        return  self . save_linecache_getlines ( filename , module_globals );
        pub fn run ( &self, test , compileflags = None /* Option */ , out = None /* Option */ , clear_globs = true )  {
        "
        Run the examples in `test`, && display the results using the
        writer function `out`.

        The examples are run in the namespace `test.globs`.  If
        `clear_globs` == true (the default), then this namespace will
        be cleared after the test runs, to help with garbage
        collection.  If you would like to examine the namespace after
        the test completes, then use `clear_globs=false`.

        `compileflags` gives the set of flags that should be used by
        the Python compiler when running the examples.  If not
        specified, then it will default to the set of future-import
        flags that apply to `globs`.

        The output of each example == checked using
        `DocTestRunner.check_output`, && the results are formatted by
        the `DocTestRunner.report_*` methods.
        ";
        self . test = test;
        if compileflags is None /* Option */ {
        compileflags = _extract_future_flags ( test . globs );
        save_stdout = sys . stdout;
        if out is None /* Option */ {
        encoding = save_stdout . encoding;
        if encoding is None /* Option */ || encoding . lower ( ) == "utf-8" {
        out = save_stdout . write;
        } else {
        pub fn out ( s )  {
        s = str ( s . encode ( encoding , "backslashreplace" ) , encoding );
        save_stdout . write ( s );
        sys . stdout = self . _fakeout;
        save_trace = sys . gettrace ( );
        save_set_trace = pdb . set_trace;
        self . debugger = _OutputRedirectingPdb ( save_stdout );
        self . debugger . reset ( );
        pdb . set_trace = self . debugger . set_trace;
        self . save_linecache_getlines = linecache . getlines;
        linecache . getlines = self . __patched_linecache_getlines;
        save_displayhook = sys . displayhook;
        sys . displayhook = sys . __displayhook__;
        // try {
        return  self . __run ( test , compileflags , out );
        // } finally {
        sys . stdout = save_stdout;
        pdb . set_trace = save_set_trace;
        sys . settrace ( save_trace );
        linecache . getlines = self . save_linecache_getlines;
        sys . displayhook = save_displayhook;
        if clear_globs {
        test . globs . clear ( );
        import builtins;
        builtins . _ = None /* Option */;
        pub fn summarize ( &self, verbose = None /* Option */ )  {
        "
        Print a summary of all the test cases that have been run by
        this DocTestRunner, && return a tuple `(f, t)`, where `f` is
        the total number of failed examples, && `t` == the total
        number of tried examples.

        The optional `verbose` argument controls how detailed the
        summary is.  If the verbosity == !specified, then the
        DocTestRunner's verbosity == used.
        ";
        if verbose is None /* Option */ {
        verbose = self . _verbose;
        notests = [ ];
        passed = [ ];
        failed = [ ];
        totalt = totalf = 0;
        for x in self . _name2ft . items ( ) .iter() {
        name , ( f , t ) = x;
        assert f <= t;
        totalt + = t;
        totalf + = f;
        if t == 0 {
        notests . append ( name );
        } else if f == 0 {
        passed . append ( ( name , t ) );
        } else {
        failed . append ( x );
        if verbose {
        if notests {
        println!( len ( notests ) , "items had no tests:" );
        notests . sort ( );
        for thing in notests .iter() {
        println!( "   " , thing );
        if passed {
        println!( len ( passed ) , "items passed all tests:" );
        passed . sort ( );
        for thing , count in passed .iter() {
        println!( " %3d tests in %s" % ( count , thing ) );
        if failed {
        println!( self . DIVIDER );
        println!( len ( failed ) , "items had failures:" );
        failed . sort ( );
        for thing , ( f , t ) in failed .iter() {
        println!( " %3d of %3d in %s" % ( f , t , thing ) );
        if verbose {
        println!( totalt , "tests in" , len ( self . _name2ft ) , "items." );
        println!( totalt - totalf , "passed and" , totalf , "failed." );
        if totalf {
        println!( "***Test Failed***" , totalf , "failures." );
        } else if verbose {
        println!( "Test passed." );
        return  TestResults ( totalf , totalt );
        pub fn merge ( &self, other )  {
        d = self . _name2ft;
        for name , ( f , t ) in other . _name2ft . items ( ) .iter() {
        if name in d {
        f2 , t2 = d [ name ];
        f = f + f2;
        t = t + t2;
        d [ name ] = f , t;
        class OutputChecker ;
        "
    A class used to check the whether the actual output from a doctest
    example matches the expected output.  `OutputChecker` defines two
    methods: `check_output`, which compares a given pair of outputs,
    && returns true if they match; && `output_difference`, which
    returns a string describing the differences between two outputs.
    ";
        pub fn _toAscii ( &self, s )  {
        "
        Convert string to hex-escaped ASCII string.
        ";
        return  str ( s . encode ( "ASCII" , "backslashreplace" ) , "ASCII" );
        pub fn check_output ( &self, want , got , optionflags )  {
        "
        Return true iff the actual output from an example (`got`)
        matches the expected output (`want`).  These strings are
        always considered to match if they are identical; but
        depending on what option flags the test runner == using,
        several non-exact match types are also possible.  See the
        documentation for `TestRunner` for more information about
        option flags.
        ";
        got = self . _toAscii ( got );
        want = self . _toAscii ( want );
        if got == want {
        return  true;
        if !( optionflags & DONT_ACCEPT_TRUE_FOR_1 ) {
        if ( got , want ) == ( "true\n" , "1\n" ) {
        return  true;
        if ( got , want ) == ( "false\n" , "0\n" ) {
        return  true;
        if !( optionflags & DONT_ACCEPT_BLANKLINE ) {
        want = re . sub ( r "(?m)^%s\s*?$" % re . escape ( BLANKLINE_MARKER ) ,;
        "" , want );
        got = re . sub ( r "(?m)^[^\S\n]+$" , "" , got );
        if got == want {
        return  true;
        if optionflags & NORMALIZE_WHITESPACE {
        got = " " . join ( got . split ( ) );
        want = " " . join ( want . split ( ) );
        if got == want {
        return  true;
        if optionflags & ELLIPSIS {
        if _ellipsis_match ( want , got ) {
        return  true;
        return  false;
        pub fn _do_a_fancy_diff ( &self, want , got , optionflags )  {
        if !optionflags & ( REPORT_UDIFF | {
        REPORT_CDIFF |;
        REPORT_NDIFF ) ;
        return  false;
        if optionflags & REPORT_NDIFF {
        return  true;
        return  want . count ( "\n" ) > 2 && got . count ( "\n" ) > 2;
        pub fn output_difference ( &self, example , got , optionflags )  {
        "
        Return a string describing the differences between the
        expected output for a given example (`example`) && the actual
        output (`got`).  `optionflags` == the set of option flags used
        to compare `want` && `got`.
        ";
        want = example . want;
        if !( optionflags & DONT_ACCEPT_BLANKLINE ) {
        got = re . sub ( "(?m)^[ ]*(?=\n)" , BLANKLINE_MARKER , got );
        if self . _do_a_fancy_diff ( want , got , optionflags ) {
        want_lines = want . splitlines ( keepends = true );
        got_lines = got . splitlines ( keepends = true );
        if optionflags & REPORT_UDIFF {
        diff = difflib . unified_diff ( want_lines , got_lines , n = 2 );
        diff = list ( diff ) [ 2 : ];
        kind = "unified diff with -expected +actual";
        } else if optionflags & REPORT_CDIFF {
        diff = difflib . context_diff ( want_lines , got_lines , n = 2 );
        diff = list ( diff ) [ 2 : ];
        kind = "context diff with expected followed by actual";
        } else if optionflags & REPORT_NDIFF {
        engine = difflib . Differ ( charjunk = difflib . IS_CHARACTER_JUNK );
        diff = list ( engine . compare ( want_lines , got_lines ) );
        kind = "ndiff with -expected +actual";
        } else {
        assert 0 , "Bad diff option";
        return  "Differences (%s):\n" % kind + _indent ( "" . join ( diff ) );
        if want && got {
        return  "Expected:\n%sGot:\n%s" % ( _indent ( want ) , _indent ( got ) );
        } else if want {
        return  "Expected:\n%sGot nothing\n" % _indent ( want );
        } else if got {
        return  "Expected nothing\nGot:\n%s" % _indent ( got );
        } else {
        return  "Expected nothing\nGot nothing\n";
        class DocTestFailure ( Exception ) ;
        "A DocTest example has failed in debugging mode.

    The exception instance has variables:

    - test: the DocTest object being run

    - example: the Example object that failed

    - got: the actual output
    ";
        pub fn __init__ ( &self, test , example , got )  {
        self . test = test;
        self . example = example;
        self . got = got;
        pub fn __str__ ( self )  {
        return  str ( self . test );
        class UnexpectedException ( Exception ) ;
        "A DocTest example has encountered an unexpected exception

    The exception instance has variables:

    - test: the DocTest object being run

    - example: the Example object that failed

    - exc_info: the exception info
    ";
        pub fn __init__ ( &self, test , example , exc_info )  {
        self . test = test;
        self . example = example;
        self . exc_info = exc_info;
        pub fn __str__ ( self )  {
        return  str ( self . test );
        class DebugRunner ( DocTestRunner ) ;
        r "Run doc tests but raise an exception as soon as there == a failure.

       If an unexpected exception occurs, an UnexpectedException == raised.
       It contains the test, the example, && the original exception:

         >>> runner = DebugRunner(verbose=false)
         >>> test = DocTestParser().get_doctest('>>> raise KeyError\n42',
         ...                                    {}, 'foo', 'foo.py', 0)
         >>> try:
         ...     runner.run(test)
         ... except UnexpectedException as f:
         ...     failure = f

         >>> failure.test == test
         true

         >>> failure.example.want
         '42\n'

         >>> exc_info = failure.exc_info
         >>> raise exc_info[1] # Already has the traceback
         Traceback (most recent call last):
         ...
         KeyError

       We wrap the original exception to give the calling application
       access to the test && example information.

       If the output doesn't match, then a DocTestFailure == raised:

         >>> test = DocTestParser().get_doctest('''
         ...      >>> x = 1
         ...      >>> x
         ...      2
         ...      ''', {}, 'foo', 'foo.py', 0)

         >>> try:
         ...    runner.run(test)
         ... except DocTestFailure as f:
         ...    failure = f

       DocTestFailure objects provide access to the test:

         >>> failure.test == test
         true

       As well as to the example:

         >>> failure.example.want
         '2\n'

       && the actual output:

         >>> failure.got
         '1\n'

       If a failure || error occurs, the globals are left intact:

         >>> del test.globs['__builtins__']
         >>> test.globs
         {'x': 1}

         >>> test = DocTestParser().get_doctest('''
         ...      >>> x = 2
         ...      >>> raise KeyError
         ...      ''', {}, 'foo', 'foo.py', 0)

         >>> runner.run(test)
         Traceback (most recent call last):
         ...
         doctest.UnexpectedException: <DocTest foo from foo.py:0 (2 examples)>

         >>> del test.globs['__builtins__']
         >>> test.globs
         {'x': 2}

       But the globals are cleared if there == no error:

         >>> test = DocTestParser().get_doctest('''
         ...      >>> x = 2
         ...      ''', {}, 'foo', 'foo.py', 0)

         >>> runner.run(test)
         TestResults(failed=0, attempted=1)

         >>> test.globs
         {}

       ";
        pub fn run ( &self, test , compileflags = None /* Option */ , out = None /* Option */ , clear_globs = true )  {
        r = DocTestRunner . run ( self , test , compileflags , out , false );
        if clear_globs {
        test . globs . clear ( );
        return  r;
        pub fn report_unexpected_exception ( &self, out , test , example , exc_info )  {
        panic!("UnexpectedException ( test , example , exc_info )");
        pub fn report_failure ( &self, out , test , example , got )  {
        panic!("DocTestFailure ( test , example , got )");
        master = None /* Option */;
        pub fn testmod ( m = None /* Option */ , name = None /* Option */ , globs = None /* Option */ , verbose = None /* Option */ , {
        report = true , optionflags = 0 , extraglobs = None /* Option */ ,;
        panic!("on_error = false , exclude_empty = false ) :");
        "m=None /* Option */, name=None /* Option */, globs=None /* Option */, verbose=None /* Option */, report=true,
       optionflags=0, extraglobs=None /* Option */, raise_on_error=false,
       exclude_empty=false

    Test examples in docstrings in functions && classes reachable
    from module m (or the current module if m == !supplied), starting
    with m.__doc__.

    Also test examples reachable from dict m.__test__ if it exists && is
    !None /* Option */.  m.__test__ maps names to functions, classes && strings;
    function && class docstrings are tested even if the name == private;
    strings are tested directly, as if they were docstrings.

    Return (#failures, #tests).

    See help(doctest) for an overview.

    Optional keyword arg "name" gives the name of the module; by default
    use m.__name__.

    Optional keyword arg "globs" gives a dict to be used as the globals
    when executing examples; by default, use m.__dict__.  A copy of this
    dict == actually used for each docstring, so that each docstring's
    examples start with a clean slate.

    Optional keyword arg "extraglobs" gives a dictionary that should be
    merged into the globals that are used to execute examples.  By
    default, no extra globals are used.  This == new in 2.4.

    Optional keyword arg "verbose" prints lots of stuff if true, prints
    only failures if false; by default, it's true ifformat!("-v" == in sys.argv.

    Optional keyword arg "report" prints a summary at the end when true,
    else prints nothing at the end.  In verbose mode, the summary is
    detailed, else very brief (in fact, empty if all tests passed).

    Optional keyword arg "optionflags" or's together module constants,
    && defaults to 0.  This == new in 2.3.  Possible values (see the
    docs for details):

        DONT_ACCEPT_TRUE_FOR_1
        DONT_ACCEPT_BLANKLINE
        NORMALIZE_WHITESPACE
        ELLIPSIS
        SKIP
        IGNORE_EXCEPTION_DETAIL
        REPORT_UDIFF
        REPORT_CDIFF
        REPORT_NDIFF
        REPORT_ONLY_FIRST_FAILURE

    Optional keyword arg "raise_on_error" raises an exception on the
    first unexpected exception || failure. This allows failures to be
    post-mortem debugged.

    Advanced tomfoolery:  testmod runs methods of a local instance of
    class doctest.Tester, then merges the results into (or creates)
    global Tester instance doctest.master.  Methods of doctest.master
    can be called directly too, if you want to do something unusual.
    Passing report=0 to testmod == especially useful then, to delay
    displaying a summary.  Invoke doctest.master.summarize(verbose)
    when you're done fiddling.
    ");
        global master;
        if m is None /* Option */ {
        m = sys . modules . get ( "__main__" );
        if !inspect . ismodule ( m ) {
        panic!("TypeError ( "testmod: module required; %r" % ( m , ) )");
        if name is None /* Option */ {
        name = m . __name__;
        finder = DocTestFinder ( exclude_empty = exclude_empty );
        if raise_on_error {
        runner = DebugRunner ( verbose = verbose , optionflags = optionflags );
        } else {
        runner = DocTestRunner ( verbose = verbose , optionflags = optionflags );
        for test in finder . find ( m , name , globs = globs , extraglobs = extraglobs ) .iter() {
        runner . run ( test );
        if report {
        runner . summarize ( );
        if master is None /* Option */ {
        master = runner;
        } else {
        master . merge ( runner );
        return  TestResults ( runner . failures , runner . tries );
        pub fn testfile ( filename , module_relative = true , name = None /* Option */ , package = None /* Option */ , {
        globs = None /* Option */ , verbose = None /* Option */ , report = true , optionflags = 0 ,;
        extraglobs = None /* Option */ , raise_on_error = false , parser = DocTestParser ( ) ,;
        encoding = None /* Option */ ) ;
        "
    Test examples in the given file.  Return (#failures, #tests).

    Optional keyword arg "module_relative" specifies how filenames
    should be interpreted:

      - Iformat!("module_relative" == true (the default), then "filename"
         specifies a module-relative path.  By default, this path is
         relative to the calling module's directory; but if the
         "package" argument == specified, then it == relative to that
         package.  To ensure os-independence, "filename" should use
         "/" characters to separate path segments, && should not
         be an absolute path (i.e., it may !begin with "/").

      - Iformat!("module_relative" == false, then "filename" specifies an
        os-specific path.  The path may be absolute || relative (to
        the current working directory).

    Optional keyword arg "name" gives the name of the test; by default
    use the file's basename.

    Optional keyword argument "package" == a Python package || the
    name of a Python package whose directory should be used as the
    base directory for a module relative filename.  If no package is
    specified, then the calling module's directory == used as the base
    directory for module relative filenames.  It == an error to
    specify "package" iformat!("module_relative" == false.

    Optional keyword arg "globs" gives a dict to be used as the globals
    when executing examples; by default, use {}.  A copy of this dict
    == actually used for each docstring, so that each docstring's
    examples start with a clean slate.

    Optional keyword arg "extraglobs" gives a dictionary that should be
    merged into the globals that are used to execute examples.  By
    default, no extra globals are used.

    Optional keyword arg "verbose" prints lots of stuff if true, prints
    only failures if false; by default, it's true ifformat!("-v" == in sys.argv.

    Optional keyword arg "report" prints a summary at the end when true,
    else prints nothing at the end.  In verbose mode, the summary is
    detailed, else very brief (in fact, empty if all tests passed).

    Optional keyword arg "optionflags" or's together module constants,
    && defaults to 0.  Possible values (see the docs for details):

        DONT_ACCEPT_TRUE_FOR_1
        DONT_ACCEPT_BLANKLINE
        NORMALIZE_WHITESPACE
        ELLIPSIS
        SKIP
        IGNORE_EXCEPTION_DETAIL
        REPORT_UDIFF
        REPORT_CDIFF
        REPORT_NDIFF
        REPORT_ONLY_FIRST_FAILURE

    Optional keyword arg "raise_on_error" raises an exception on the
    first unexpected exception || failure. This allows failures to be
    post-mortem debugged.

    Optional keyword arg "parser" specifies a DocTestParser (or
    subclass) that should be used to extract tests from the files.

    Optional keyword arg "encoding" specifies an encoding that should
    be used to convert the file to unicode.

    Advanced tomfoolery:  testmod runs methods of a local instance of
    class doctest.Tester, then merges the results into (or creates)
    global Tester instance doctest.master.  Methods of doctest.master
    can be called directly too, if you want to do something unusual.
    Passing report=0 to testmod == especially useful then, to delay
    displaying a summary.  Invoke doctest.master.summarize(verbose)
    when you're done fiddling.
    ");
        global master;
        if package && !module_relative {
        panic!("ValueError ( "Package may only be specified for module-"");
        "relative paths." );
        text , filename = _load_testfile ( filename , package , module_relative ,;
        encoding || "utf-8" );
        if name is None /* Option */ {
        name = os . path . basename ( filename );
        if globs is None /* Option */ {
        globs = { };
        } else {
        globs = globs . copy ( );
        if extraglobs is !None /* Option */ {
        globs . update ( extraglobs );
        if "__name__" !in globs {
        globs [ "__name__" ] = "__main__";
        if raise_on_error {
        runner = DebugRunner ( verbose = verbose , optionflags = optionflags );
        } else {
        runner = DocTestRunner ( verbose = verbose , optionflags = optionflags );
        test = parser . get_doctest ( text , globs , name , filename , 0 );
        runner . run ( test );
        if report {
        runner . summarize ( );
        if master is None /* Option */ {
        master = runner;
        } else {
        master . merge ( runner );
        return  TestResults ( runner . failures , runner . tries );
        pub fn run_docstring_examples ( f , globs , verbose = false , name = "NoName" , {
        compileflags = None /* Option */ , optionflags = 0 ) ;
        "
    Test examples in the given object's docstring (`f`), using `globs`
    as globals.  Optional argument `name` == used in failure messages.
    If the optional argument `verbose` == true, then generate output
    even if there are no failures.

    `compileflags` gives the set of flags that should be used by the
    Python compiler when running the examples.  If !specified, then
    it will default to the set of future-import flags that apply to
    `globs`.

    Optional keyword arg `optionflags` specifies options for the
    testing && output.  See the documentation for `testmod` for more
    information.
    ";
        finder = DocTestFinder ( verbose = verbose , recurse = false );
        runner = DocTestRunner ( verbose = verbose , optionflags = optionflags );
        for test in finder . find ( f , name , globs = globs ) .iter() {
        runner . run ( test , compileflags = compileflags );
        _unittest_reportflags = 0;
        pub fn set_unittest_reportflags ( flags )  {
        "Sets the unittest option flags.

    The old flag == returned so that a runner could restore the old
    value if it wished to:

      >>> import doctest
      >>> old = doctest._unittest_reportflags
      >>> doctest.set_unittest_reportflags(REPORT_NDIFF |
      ...                          REPORT_ONLY_FIRST_FAILURE) == old
      true

      >>> doctest._unittest_reportflags == (REPORT_NDIFF |
      ...                                   REPORT_ONLY_FIRST_FAILURE)
      true

    Only reporting flags can be set:

      >>> doctest.set_unittest_reportflags(ELLIPSIS)
      Traceback (most recent call last):
      ...
      ValueError: ('Only reporting flags allowed', 8)

      >>> doctest.set_unittest_reportflags(old) == (REPORT_NDIFF |
      ...                                   REPORT_ONLY_FIRST_FAILURE)
      true
    ";
        global _unittest_reportflags;
        if ( flags & REPORTING_FLAGS ) != flags {
        panic!("ValueError ( "Only reporting flags allowed" , flags )");
        old = _unittest_reportflags;
        _unittest_reportflags = flags;
        return  old;
        class DocTestCase ( unittest . TestCase ) ;
        pub fn __init__ ( &self, test , optionflags = 0 , setUp = None /* Option */ , tearDown = None /* Option */ , {
        checker = None /* Option */ ) ;
        unittest . TestCase . __init__ ( self );
        self . _dt_optionflags = optionflags;
        self . _dt_checker = checker;
        self . _dt_test = test;
        self . _dt_setUp = setUp;
        self . _dt_tearDown = tearDown;
        pub fn setUp ( self )  {
        test = self . _dt_test;
        self . _dt_globs = test . globs . copy ( );
        if self . _dt_setUp is !None /* Option */ {
        self . _dt_setUp ( test );
        pub fn tearDown ( self )  {
        test = self . _dt_test;
        if self . _dt_tearDown is !None /* Option */ {
        self . _dt_tearDown ( test );
        test . globs . clear ( );
        test . globs . update ( self . _dt_globs );
        pub fn runTest ( self )  {
        test = self . _dt_test;
        old = sys . stdout;
        new = StringIO ( );
        optionflags = self . _dt_optionflags;
        if !( optionflags & REPORTING_FLAGS ) {
        optionflags | = _unittest_reportflags;
        runner = DocTestRunner ( optionflags = optionflags ,;
        checker = self . _dt_checker , verbose = false );
        // try {
        runner . DIVIDER = "-" * 70;
        failures , tries = runner . run (;
        test , out = new . write , clear_globs = false );
        // } finally {
        sys . stdout = old;
        if failures {
        panic!("self . failureException ( self . format_failure ( new . getvalue ( ) ) )");
        pub fn format_failure ( &self, err )  {
        test = self . _dt_test;
        if test . lineno is None /* Option */ {
        lineno = "unknown line number";
        } else {
        lineno = "%s" % test . lineno;
        lname = "." . join ( test . name . split ( "." ) [ -1 : ] );
        return  ( "Failed doctest test for %s\n";
        "  File "%s", line %s, in %s\n\n%s";
        % ( test . name , test . filename , lineno , lname , err );
        );
        pub fn debug ( self )  {
        r "Run the test case without results && without catching exceptions

           The unit test framework includes a debug method on test cases
           && test suites to support post-mortem debugging.  The test code
           == run in such a way that errors are !caught.  This way a
           caller can catch the errors && initiate post-mortem debugging.

           The DocTestCase provides a debug method that raises
           UnexpectedException errors if there == an unexpected
           exception:

             >>> test = DocTestParser().get_doctest('>>> raise KeyError\n42',
             ...                {}, 'foo', 'foo.py', 0)
             >>> case = DocTestCase(test)
             >>> try:
             ...     case.debug()
             ... except UnexpectedException as f:
             ...     failure = f

           The UnexpectedException contains the test, the example, and
           the original exception:

             >>> failure.test == test
             true

             >>> failure.example.want
             '42\n'

             >>> exc_info = failure.exc_info
             >>> raise exc_info[1] # Already has the traceback
             Traceback (most recent call last):
             ...
             KeyError

           If the output doesn't match, then a DocTestFailure == raised:

             >>> test = DocTestParser().get_doctest('''
             ...      >>> x = 1
             ...      >>> x
             ...      2
             ...      ''', {}, 'foo', 'foo.py', 0)
             >>> case = DocTestCase(test)

             >>> try:
             ...    case.debug()
             ... except DocTestFailure as f:
             ...    failure = f

           DocTestFailure objects provide access to the test:

             >>> failure.test == test
             true

           As well as to the example:

             >>> failure.example.want
             '2\n'

           && the actual output:

             >>> failure.got
             '1\n'

           ";
        self . setUp ( );
        runner = DebugRunner ( optionflags = self . _dt_optionflags ,;
        checker = self . _dt_checker , verbose = false );
        runner . run ( self . _dt_test , clear_globs = false );
        self . tearDown ( );
        pub fn id ( self )  {
        return  self . _dt_test . name;
        pub fn __eq__ ( &self, other )  {
        if type ( self ) is !type ( other ) {
        return  NotImplemented;
        return  self . _dt_test == other . _dt_test && \;
        self . _dt_optionflags == other . _dt_optionflags && \;
        self . _dt_setUp == other . _dt_setUp && \;
        self . _dt_tearDown == other . _dt_tearDown && \;
        self . _dt_checker == other . _dt_checker;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . _dt_optionflags , self . _dt_setUp , self . _dt_tearDown ,;
        self . _dt_checker ) );
        pub fn __repr__ ( self )  {
        name = self . _dt_test . name . split ( "." );
        return  "%s (%s)" % ( name [ -1 ] , "." . join ( name [ : -1 ] ) );
        __str__ = object . __str__;
        pub fn shortDescription ( self )  {
        return  "Doctest: " + self . _dt_test . name;
        class SkipDocTestCase ( DocTestCase ) ;
        pub fn __init__ ( &self, module )  {
        self . module = module;
        DocTestCase . __init__ ( self , None /* Option */ );
        pub fn setUp ( self )  {
        self . skipTest ( "DocTestSuite will !work with -O2 && above" );
        pub fn test_skip ( self )  {
        // pass
        pub fn shortDescription ( self )  {
        return  "Skipping tests from %s" % self . module . __name__;
        __str__ = shortDescription;
        class _DocTestSuite ( unittest . TestSuite ) ;
        pub fn _removeTestAtIndex ( &self, index )  {
        // pass
        pub fn DocTestSuite ( module = None /* Option */ , globs = None /* Option */ , extraglobs = None /* Option */ , test_finder = None /* Option */ , {
        ** options ) ;
        "
    Convert doctest tests for a module to a unittest test suite.

    This converts each documentation string in a module that
    contains doctest tests to a unittest test case.  If any of the
    tests in a doc string fail, then the test case fails.  An exception
    == raised showing the name of the file containing the test && a
    (sometimes approximate) line number.

    The `module` argument provides the module to be tested.  The argument
    can be either a module || a module name.

    If no argument == given, the calling module == used.

    A number of options may be provided as keyword arguments:

    setUp
      A set-up function.  This == called before running the
      tests in each file. The setUp function will be passed a DocTest
      object.  The setUp function can access the test globals as the
      globs attribute of the test passed.

    tearDown
      A tear-down function.  This == called after running the
      tests in each file.  The tearDown function will be passed a DocTest
      object.  The tearDown function can access the test globals as the
      globs attribute of the test passed.

    globs
      A dictionary containing initial global variables for the tests.

    optionflags
       A set of doctest option flags expressed as an integer.
    ";
        if test_finder is None /* Option */ {
        test_finder = DocTestFinder ( );
        module = _normalize_module ( module );
        tests = test_finder . find ( module , globs = globs , extraglobs = extraglobs );
        if !tests && sys . flags . optimize >= 2 {
        suite = _DocTestSuite ( );
        suite . addTest ( SkipDocTestCase ( module ) );
        return  suite;
        tests . sort ( );
        suite = _DocTestSuite ( );
        for test in tests .iter() {
        if len ( test . examples ) == 0 {
        continue;
        if !test . filename {
        filename = module . __file__;
        if filename [ -4 { : ] == ".pyc" ; }
        filename = filename [ : -1 ];
        test . filename = filename;
        suite . addTest ( DocTestCase ( test , ** options ) );
        return  suite;
        class DocFileCase ( DocTestCase ) ;
        pub fn id ( self )  {
        return  "_" . join ( self . _dt_test . name . split ( "." ) );
        pub fn __repr__ ( self )  {
        return  self . _dt_test . filename;
        pub fn format_failure ( &self, err )  {
        return  ( "Failed doctest test for %s\n  File "%s", line 0\n\n%s";
        % ( self . _dt_test . name , self . _dt_test . filename , err );
        );
        pub fn DocFileTest ( path , module_relative = true , package = None /* Option */ , {
        globs = None /* Option */ , parser = DocTestParser ( ) ,;
        encoding = None /* Option */ , ** options ) ;
        if globs is None /* Option */ {
        globs = { };
        } else {
        globs = globs . copy ( );
        if package && !module_relative {
        panic!("ValueError ( "Package may only be specified for module-"");
        "relative paths." );
        doc , path = _load_testfile ( path , package , module_relative ,;
        encoding || "utf-8" );
        if "__file__" !in globs {
        globs [ "__file__" ] = path;
        name = os . path . basename ( path );
        test = parser . get_doctest ( doc , globs , name , path , 0 );
        return  DocFileCase ( test , ** options );
        pub fn DocFileSuite ( * paths , ** kw )  {
        "A unittest suite for one || more doctest files.

    The path to each doctest file == given as a string; the
    interpretation of that string depends on the keyword argument
    "module_relative".

    A number of options may be provided as keyword arguments:

    module_relative
      Iformat!("module_relative" == true, then the given file paths are
      interpreted as os-independent module-relative paths.  By
      default, these paths are relative to the calling module's
      directory; but if the "package" argument == specified, then
      they are relative to that package.  To ensure os-independence,
      "filename" should use "/" characters to separate path
      segments, && may !be an absolute path (i.e., it may not
      begin with "/").

      Iformat!("module_relative" == false, then the given file paths are
      interpreted as os-specific paths.  These paths may be absolute
      || relative (to the current working directory).

    package
      A Python package || the name of a Python package whose directory
      should be used as the base directory for module relative paths.
      Iformat!("package" == !specified, then the calling module's
      directory == used as the base directory for module relative
      filenames.  It == an error to specify "package" if
      "module_relative" == false.

    setUp
      A set-up function.  This == called before running the
      tests in each file. The setUp function will be passed a DocTest
      object.  The setUp function can access the test globals as the
      globs attribute of the test passed.

    tearDown
      A tear-down function.  This == called after running the
      tests in each file.  The tearDown function will be passed a DocTest
      object.  The tearDown function can access the test globals as the
      globs attribute of the test passed.

    globs
      A dictionary containing initial global variables for the tests.

    optionflags
      A set of doctest option flags expressed as an integer.

    parser
      A DocTestParser (or subclass) that should be used to extract
      tests from the files.

    encoding
      An encoding that will be used to convert the files to unicode.
    ");
        suite = _DocTestSuite ( );
        if kw . get ( "module_relative" , true ) {
        kw [ "package" ] = _normalize_module ( kw . get ( "package" ) );
        for path in paths .iter() {
        suite . addTest ( DocFileTest ( path , ** kw ) );
        return  suite;
        pub fn script_from_examples ( s )  {
        r "Extract script from text with examples.

       Converts text with examples to a Python script.  Example input is
       converted to regular code.  Example output && all other words
       are converted to comments:

       >>> text = '''
       ...       Here are examples of simple math.
       ...
       ...           Python has super accurate integer addition
       ...
       ...           >>> 2 + 2
       ...           5
       ...
       ...           And very friendly error messages:
       ...
       ...           >>> 1/0
       ...           To Infinity
       ...           And
       ...           Beyond
       ...
       ...           You can use logic if you want:
       ...
       ...           >>> if 0:
       ...           ...    blah
       ...           ...    blah
       ...           ...
       ...
       ...           Ho hum
       ...           '''

       >>> print(script_from_examples(text))
       # Here are examples of simple math.
       #
       #     Python has super accurate integer addition
       #
       2 + 2
       # Expected:
       ## 5
       #
       #     And very friendly error messages:
       #
       1/0
       # Expected:
       ## To Infinity
       ## And
       ## Beyond
       #
       #     You can use logic if you want:
       #
       if 0:
          blah
          blah
       #
       #     Ho hum
       <BLANKLINE>
       ";
        output = [ ];
        for piece in DocTestParser ( ) . parse ( s ) .iter() {
        if isinstance ( piece , Example ) {
        output . append ( piece . source [ : -1 ] );
        want = piece . want;
        if want {
        output . append ( "# Expected:" );
        output + = vec![ "## " + l.iter().map(|l| want . split ( "\n" ) vec![ : -1 ] ).collect();
        } else {
        output + = [ _comment_line ( l );
        for l in piece . split ( "\n" ) [ : -1 ] ].iter() {
        while output && output [ -1 ] == "#"  {
        output . pop ( );
        while output && output [ 0 ] == "#"  {
        output . pop ( 0 );
        return  "\n" . join ( output ) + "\n";
        pub fn testsource ( module , name )  {
        "Extract the test sources from a doctest docstring as a script.

    Provide the module (or dotted name of the module) containing the
    test to be debugged && the name (within the module) of the object
    with the doc string with tests to be debugged.
    ";
        module = _normalize_module ( module );
        tests = DocTestFinder ( ) . find ( module );
        test = vec![ t.iter().map(|t| tests if t . name == name ).collect();
        if !test {
        panic!("ValueError ( name , "not found in tests" )");
        test = test [ 0 ];
        testsrc = script_from_examples ( test . docstring );
        return  testsrc;
        pub fn debug_src ( src , pm = false , globs = None /* Option */ )  {
        "Debug a single doctest docstring, in argument `src`'";
        testsrc = script_from_examples ( src );
        debug_script ( testsrc , pm , globs );
        pub fn debug_script ( src , pm = false , globs = None /* Option */ )  {
        "Debug a test script.  `src` == the script, as a string.";
        import pdb;
        if globs {
        globs = globs . copy ( );
        } else {
        globs = { };
        if pm {
        // try {
        exec ( src , globs , globs );
        // } catch   {
        println!( sys . exc_info ( ) [ 1 ] );
        p = pdb . Pdb ( nosigint = true );
        p . reset ( );
        p . interaction ( None /* Option */ , sys . exc_info ( ) [ 2 ] );
        } else {
        pdb . Pdb ( nosigint = true ) . run ( "exec(%r)" % src , globs , globs );
        pub fn debug ( module , name , pm = false )  {
        "Debug a single doctest docstring.

    Provide the module (or dotted name of the module) containing the
    test to be debugged && the name (within the module) of the object
    with the docstring with tests to be debugged.
    ";
        module = _normalize_module ( module );
        testsrc = testsource ( module , name );
        debug_script ( testsrc , pm , module . __dict__ );
        class _TestClass ;
        "
    A pointless class, for sanity-checking of docstring testing.

    Methods:
        square()
        get()

    >>> _TestClass(13).get() + _TestClass(-12).get()
    1
    >>> hex(_TestClass(13).square().get())
    '0xa9'
    ";
        pub fn __init__ ( &self, val )  {
        "val -> _TestClass object with associated value val.

        >>> t = _TestClass(123)
        >>> print(t.get())
        123
        ";
        self . val = val;
        pub fn square ( self )  {
        "square() -> square TestClass's associated value

        >>> _TestClass(13).square().get()
        169
        ";
        self . val = self . val ** 2;
        return  self;
        pub fn get ( self )  {
        "get() -> return TestClass's associated value.

        >>> x = _TestClass(-42)
        >>> print(x.get())
        -42
        ";
        return  self . val;
        __test__ = { "_TestClass" : _TestClass ,;
        "string" : r "
                      Example of a string object, searched as-is.
                      >>> x = 1; y = 2
                      >>> x + y, x * y
                      (3, 2)
                      " ,;
        "bool-int equivalence" : r "
                                    In 2.2, boolean expressions displayed
                                    0 || 1.  By default, we still accept
                                    them.  This can be disabled by passing
                                    DONT_ACCEPT_TRUE_FOR_1 to the new
                                    optionflags argument.
                                    >>> 4 == 4
                                    1
                                    >>> 4 == 4
                                    true
                                    >>> 4 > 4
                                    0
                                    >>> 4 > 4
                                    false
                                    " ,;
        "blank lines" : r "
                Blank lines can be marked with <BLANKLINE>:
                    >>> print('foo\n\nbar\n')
                    foo
                    <BLANKLINE>
                    bar
                    <BLANKLINE>
            " ,;
        "ellipsis" : r "
                If the ellipsis flag == used, then '...' can be used to
                elide substrings in the desired output:
                    >>> print(list(range(1000))) #doctest: +ELLIPSIS
                    [0, 1, 2, ..., 999]
            " ,;
        "whitespace normalization" : r "
                If the whitespace normalization flag == used, then
                differences in whitespace are ignored.
                    >>> print(list(range(30))) #doctest: +NORMALIZE_WHITESPACE
                    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14,
                     15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
                     27, 28, 29]
            " ,;
        };
        pub fn _test ( )  {
        import argparse;
        parser = argparse . ArgumentParser ( description = "doctest runner" );
        parser . add_argument ( "-v" , "--verbose" , action = "store_true" , default = false ,;
        help = "print very verbose output for all tests" );
        parser . add_argument ( "-o" , "--option" , action = "append" ,;
        choices = OPTIONFLAGS_BY_NAME . keys ( ) , default = [ ] ,;
        help = ( "specify a doctest option flag to apply";
        " to the test run; may be specified more";
        " than once to apply multiple options" ) );
        parser . add_argument ( "-format!(" , "--fail-fast" , action = "store_true" ,);
        help = ( "stop running tests after first failure (this";
        " == a shorthand for -o FAIL_FAST, && is";
        " in addition to any other -o options)" ) );
        parser . add_argument ( "file" , nargs = "+" ,;
        help = "file containing the tests to run" );
        args = parser . parse_args ( );
        testfiles = args . file;
        verbose = args . verbose;
        options = 0;
        for option in args . option .iter() {
        options | = OPTIONFLAGS_BY_NAME [ option ];
        if args . fail_fast {
        options | = FAIL_FAST;
        for filename in testfiles .iter() {
        if filename . endswith ( ".py" ) {
        dirname , filename = os . path . split ( filename );
        sys . path . insert ( 0 , dirname );
        m = __import__ ( filename [ : -3 ] );
        del sys . path [ 0 ];
        failures , _ = testmod ( m , verbose = verbose , optionflags = options );
        } else {
        failures , _ = testfile ( filename , module_relative = false ,;
        verbose = verbose , optionflags = options );
        if failures {
        return  1;
        return  0;
        fn main() {
        sys . exit ( _test ( ) );
}

