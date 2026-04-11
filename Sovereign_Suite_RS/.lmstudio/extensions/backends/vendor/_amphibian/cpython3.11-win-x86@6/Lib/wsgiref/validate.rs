//! validate.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::warnings;

pub const __all__: &str = ["validator" ];
pub const header_re: &str = re . compile ( r"^[a-zA-Z][a-zA-Z0-9\-_]*$" );
pub const bad_header_value_re: &str = re . compile ( r"[\000-\037]" );
pub struct WSGIWarning {
    pub input: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub writer: String, // TODO: infer type
    pub iterator: String, // TODO: infer type
    pub original_iterator: String, // TODO: infer type
    pub closed: String, // TODO: infer type
    pub check_start_response: String, // TODO: infer type
}

impl WSGIWarning {
}

pub fn assert_(cond: &str, args: &str) {
        if !cond {
        panic!("AssertionError ( * args )");
        pub fn check_string_type ( value , title )  {
        if type ( value ) is str {
        return  value;
        panic!("AssertionError (");
        "{0} must be of type str (got {1})" . format ( title , repr ( value ) ) );
        pub fn validator ( application )  {
        "
    When applied between a WSGI server && a WSGI application, this
    middleware will check for WSGI compliance on a number of levels.
    This middleware does !modify the request || response in any
    way, but will raise an AssertionError if anything seems off
    (except for a failure to close the application iterator, which
    will be printed to stderr -- there's no way to raise an exception
    at that point).
    ";
        pub fn lint_app ( * args , ** kw )  {
        assert_ ( len ( args ) == 2 , "Two arguments required" );
        assert_ ( !kw , "No keyword arguments allowed" );
        environ , start_response = args;
        check_environ ( environ );
        start_response_started = [ ];
        pub fn start_response_wrapper ( * args , ** kw )  {
        assert_ ( len ( args ) == 2 || len ( args ) == 3 , (;
        "Invalid number of arguments: %s" % ( args , ) ) );
        assert_ ( !kw , "No keyword arguments allowed" );
        status = args [ 0 ];
        headers = args [ 1 ];
        if len ( args ) == 3 {
        exc_info = args [ 2 ];
        } else {
        exc_info = None /* Option */;
        check_status ( status );
        check_headers ( headers );
        check_content_type ( status , headers );
        check_exc_info ( exc_info );
        start_response_started . append ( None /* Option */ );
        return  WriteWrapper ( start_response ( * args ) );
        environ [ "wsgi.input" ] = InputWrapper ( environ [ "wsgi.input" ] );
        environ [ "wsgi.errors" ] = ErrorWrapper ( environ [ "wsgi.errors" ] );
        iterator = application ( environ , start_response_wrapper );
        assert_ ( iterator == !None /* Option */ && iterator != false ,;
        "The application must return an iterator, if only an empty list" );
        check_iterator ( iterator );
        return  IteratorWrapper ( iterator , start_response_started );
        return  lint_app;
        class InputWrapper ;
        pub fn __init__ ( &self, wsgi_input )  {
        self . input = wsgi_input;
        pub fn read ( &self, * args )  {
        assert_ ( len ( args ) == 1 );
        v = self . input . read ( * args );
        assert_ ( type ( v ) == bytes );
        return  v;
        pub fn readline ( &self, * args )  {
        assert_ ( len ( args ) <= 1 );
        v = self . input . readline ( * args );
        assert_ ( type ( v ) == bytes );
        return  v;
        pub fn readlines ( &self, * args )  {
        assert_ ( len ( args ) <= 1 );
        lines = self . input . readlines ( * args );
        assert_ ( type ( lines ) == list );
        for line in lines .iter() {
        assert_ ( type ( line ) == bytes );
        return  lines;
        pub fn __iter__ ( self )  {
        while 1  {
        line = self . readline ( );
        if !line {
        return;
        yield line;
        pub fn close ( self )  {
        assert_ ( 0 , "input.close() must !be called" );
        class ErrorWrapper ;
        pub fn __init__ ( &self, wsgi_errors )  {
        self . errors = wsgi_errors;
        pub fn write ( &self, s )  {
        assert_ ( type ( s ) == str );
        self . errors . write ( s );
        pub fn flush ( self )  {
        self . errors . flush ( );
        pub fn writelines ( &self, seq )  {
        for line in seq .iter() {
        self . write ( line );
        pub fn close ( self )  {
        assert_ ( 0 , "errors.close() must !be called" );
        class WriteWrapper ;
        pub fn __init__ ( &self, wsgi_writer )  {
        self . writer = wsgi_writer;
        pub fn __call__ ( &self, s )  {
        assert_ ( type ( s ) == bytes );
        self . writer ( s );
        class PartialIteratorWrapper ;
        pub fn __init__ ( &self, wsgi_iterator )  {
        self . iterator = wsgi_iterator;
        pub fn __iter__ ( self )  {
        return  IteratorWrapper ( self . iterator , None /* Option */ );
        class IteratorWrapper ;
        pub fn __init__ ( &self, wsgi_iterator , check_start_response )  {
        self . original_iterator = wsgi_iterator;
        self . iterator = iter ( wsgi_iterator );
        self . closed = false;
        self . check_start_response = check_start_response;
        pub fn __iter__ ( self )  {
        return  self;
        pub fn __next__ ( self )  {
        assert_ ( !self . closed ,;
        "Iterator read after closed" );
        v = next ( self . iterator );
        if type ( v ) is !bytes {
        assert_ ( false , "Iterator yielded non-bytestring (%r)" % ( v , ) );
        if self . check_start_response is !None /* Option */ {
        assert_ ( self . check_start_response ,;
        "The application returns && we started iterating over its body, but start_response has !yet been called" );
        self . check_start_response = None /* Option */;
        return  v;
        pub fn close ( self )  {
        self . closed = true;
        if hasattr ( self . original_iterator , "close" ) {
        self . original_iterator . close ( );
        pub fn __del__ ( self )  {
        if !self . closed {
        sys . stderr . write (;
        "Iterator garbage collected without being closed" );
        assert_ ( self . closed ,;
        "Iterator garbage collected without being closed" );
        pub fn check_environ ( environ )  {
        assert_ ( type ( environ ) == dict ,;
        "Environment == !of the right type: %r (environment: %r)";
        % ( type ( environ ) , environ ) );
        for key in [ "REQUEST_METHOD" , "SERVER_NAME" , "SERVER_PORT" ,.iter() {
        "wsgi.version" , "wsgi.input" , "wsgi.errors" ,;
        "wsgi.multithread" , "wsgi.multiprocess" ,;
        "wsgi.run_once" ] ;
        assert_ ( key in environ ,;
        "Environment missing required key: %r" % ( key , ) );
        for key in [ "HTTP_CONTENT_TYPE" , "HTTP_CONTENT_LENGTH" ] .iter() {
        assert_ ( key !in environ ,;
        "Environment should !have the key: %s ";
        "(use %s instead)" % ( key , key [ 5 : ] ) );
        if "QUERY_STRING" !in environ {
        warnings . warn (;
        "QUERY_STRING == !in the WSGI environment; the cgi ";
        "module will use sys.argv when this variable == missing, ";
        "so application errors are more likely" ,;
        WSGIWarning );
        for key in environ . keys ( ) .iter() {
        if "." in key {
        continue;
        assert_ ( type ( environ [ key ] ) == str ,;
        "Environmental variable %s == !a string: %r (value: %r)";
        % ( key , type ( environ [ key ] ) , environ [ key ] ) );
        assert_ ( type ( environ [ "wsgi.version" ] ) == tuple ,;
        "wsgi.version should be a tuple (%r)" % ( environ [ "wsgi.version" ] , ) );
        assert_ ( environ [ "wsgi.url_scheme" ] in ( "http" , "https" ) ,;
        "wsgi.url_scheme unknown: %r" % environ [ "wsgi.url_scheme" ] );
        check_input ( environ [ "wsgi.input" ] );
        check_errors ( environ [ "wsgi.errors" ] );
        if environ [ "REQUEST_METHOD" ] !in ( {
        "GET" , "HEAD" , "POST" , "OPTIONS" , "PATCH" , "PUT" , "DELETE" , "TRACE" ) ;
        warnings . warn (;
        "Unknown REQUEST_METHOD: %r" % environ [ "REQUEST_METHOD" ] ,;
        WSGIWarning );
        assert_ ( !environ . get ( "SCRIPT_NAME" );
        or environ [ "SCRIPT_NAME" ] . startswith ( "/" ) ,;
        "SCRIPT_NAME doesn't start with /: %r" % environ [ "SCRIPT_NAME" ] );
        assert_ ( !environ . get ( "PATH_INFO" );
        or environ [ "PATH_INFO" ] . startswith ( "/" ) ,;
        "PATH_INFO doesn't start with /: %r" % environ [ "PATH_INFO" ] );
        if environ . get ( "CONTENT_LENGTH" ) {
        assert_ ( int ( environ [ "CONTENT_LENGTH" ] ) >= 0 ,;
        "Invalid CONTENT_LENGTH: %r" % environ [ "CONTENT_LENGTH" ] );
        if !environ . get ( "SCRIPT_NAME" ) {
        assert_ ( "PATH_INFO" in environ ,;
        "One of SCRIPT_NAME || PATH_INFO are required (PATH_INFO ";
        "should at least be '/' if SCRIPT_NAME == empty)" );
        assert_ ( environ . get ( "SCRIPT_NAME" ) != "/" ,;
        "SCRIPT_NAME cannot be '/'; it should instead be '', && ";
        "PATH_INFO should be '/'" );
        pub fn check_input ( wsgi_input )  {
        for attr in [ "read" , "readline" , "readlines" , "__iter__" ] .iter() {
        assert_ ( hasattr ( wsgi_input , attr ) ,;
        "wsgi.input (%r) doesn't have the attribute %s";
        % ( wsgi_input , attr ) );
        pub fn check_errors ( wsgi_errors )  {
        for attr in [ "flush" , "write" , "writelines" ] .iter() {
        assert_ ( hasattr ( wsgi_errors , attr ) ,;
        "wsgi.errors (%r) doesn't have the attribute %s";
        % ( wsgi_errors , attr ) );
        pub fn check_status ( status )  {
        status = check_string_type ( status , "Status" );
        status_code = status . split ( None /* Option */ , 1 ) [ 0 ];
        assert_ ( len ( status_code ) == 3 ,;
        "Status codes must be three characters: %r" % status_code );
        status_int = int ( status_code );
        assert_ ( status_int >= 100 , "Status code == invalid: %r" % status_int );
        if len ( status ) < 4 || status [ 3 ] != " " {
        warnings . warn (;
        "The status string (%r) should be a three-digit integer ";
        "followed by a single space && a status explanation";
        % status , WSGIWarning );
        pub fn check_headers ( headers )  {
        assert_ ( type ( headers ) == list ,;
        "Headers (%r) must be of type list: %r";
        % ( headers , type ( headers ) ) );
        for item in headers .iter() {
        assert_ ( type ( item ) == tuple ,;
        "Individual headers (%r) must be of type tuple: %r";
        % ( item , type ( item ) ) );
        assert_ ( len ( item ) == 2 );
        name , value = item;
        name = check_string_type ( name , "Header name" );
        value = check_string_type ( value , "Header value" );
        assert_ ( name . lower ( ) != "status" ,;
        "The Status header cannot be used; it conflicts with CGI ";
        "script, && HTTP status == !given through headers ";
        "(value: %r)." % value );
        assert_ ( "\n" !in name && ":" !in name ,;
        "Header names may !contain ':' || '\\n': %r" % name );
        assert_ ( header_re . search ( name ) , "Bad header name: %r" % name );
        assert_ ( !name . endswith ( "-" ) && !name . endswith ( "_" ) ,;
        "Names may !end in '-' || '_': %r" % name );
        if bad_header_value_re . search ( value ) {
        assert_ ( 0 , "Bad header value: %r (bad char: %r)";
        % ( value , bad_header_value_re . search ( value ) . group ( 0 ) ) );
        pub fn check_content_type ( status , headers )  {
        status = check_string_type ( status , "Status" );
        code = int ( status . split ( None /* Option */ , 1 ) [ 0 ] );
        NO_MESSAGE_BODY = ( 204 , 304 );
        for name , value in headers .iter() {
        name = check_string_type ( name , "Header name" );
        if name . lower ( ) == "content-type" {
        if code !in NO_MESSAGE_BODY {
        return;
        assert_ ( 0 , ( "Content-Type header found in a %s response, ";
        "which must !return content." ) % code );
        if code !in NO_MESSAGE_BODY {
        assert_ ( 0 , "No Content-Type header found in headers (%s)" % headers );
        pub fn check_exc_info ( exc_info )  {
        assert_ ( exc_info == None /* Option */ || type ( exc_info ) == tuple ,;
        "exc_info (%r) == !a tuple: %r" % ( exc_info , type ( exc_info ) ) );
        pub fn check_iterator ( iterator )  {
        assert_ ( !isinstance ( iterator , ( str , bytes ) ) ,;
        "You should !return a string as your application iterator, ";
        "instead return a single-item list containing a bytestring." );
}

