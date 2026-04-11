//! client.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::email;
// use crate::errno;
// use crate::io;
// use crate::socket;
// use std::collections;
// use crate::urlsplit;
// use crate::ssl;
// use crate::warnings;

pub const __all__: &str = ["HTTPResponse" ,"HTTPConnection" ,;
pub const HTTP_PORT: u64 = 80;
pub const HTTPS_PORT: u64 = 443;
pub const _UNKNOWN: &str = "UNKNOWN";
pub const _CS_IDLE: &str = "Idle";
pub const _CS_REQ_STARTED: &str = "Request-started";
pub const _CS_REQ_SENT: &str = "Request-sent";
pub const responses: f64 = { v : v . phrase for v in http . HTTPStatus . __members__ . values ( ) };
pub const _MAXLINE: u64 = 65536;
pub const _MAXHEADERS: u64 = 100;
pub const _is_legal_header_name: &str = re . compile ( rb"[^:\s][^:\r\n]*" ) . fullmatch;
pub const _is_illegal_header_value: &str = re . compile ( rb"\n(?![ \t])|\r(?![ \t\n])" ) . search;
pub const _contains_disallowed_url_pchar_re: &str = re . compile ("[\x00-\x20\x7f]" );
pub const _contains_disallowed_method_pchar_re: &str = re . compile ("[\x00-\x1f]" );
pub const _METHODS_EXPECTING_BODY: &str = {"PATCH" ,"POST" ,"PUT" };
pub fn _encode(data: &str, name: &str) {
        "Call data.encode("latin-1") but show a better error message.";
        // try {
        return  data . encode ( "latin-1" );
        // } catch  UnicodeEncodeError as err  {
        panic!("UnicodeEncodeError (");
        err . encoding ,;
        err . object ,;
        err . start ,;
        err . end ,;
        "%s (%.20r) == !valid Latin-1. Use %s.encode('utf-8') ";
        "if you want to send it encoded in UTF-8." %;
        ( name . title ( ) , data [ err . start : err . end ] , name ) ) from None /* Option */;
        pub fn _strip_ipv6_iface ( enc_name  {  bytes ) - > bytes ; }
        "Remove interface scope from IPv6 address.";
        enc_name , percent , _ = enc_name . partition ( b "%" );
        if percent {
        assert enc_name . startswith ( b "[" ) , enc_name;
        enc_name + = b "]";
        return  enc_name;
        class HTTPMessage ( email . message . Message ) ;
        pub fn getallmatchingheaders ( &self, name )  {
        "Find all header lines matching a given header name.

        Look through the list of headers && find all lines matching a given
        header name (and their continuation lines).  A list of the lines is
        returned, without interpretation.  If the header does !occur, an
        empty list == returned.  If the header occurs multiple times, all
        occurrences are returned.  Case == !important in the header name.

        ";
        name = name . lower ( ) + ":";
        n = len ( name );
        lst = [ ];
        hit = 0;
        for line in self . keys ( ) .iter() {
        if line [ { : n ] . lower ( ) == name ; }
        hit = 1;
        } else if !line [ {
        hit = 0;
        if hit {
        lst . append ( line );
        return  lst;
        pub fn _read_headers ( fp )  {
        "Reads potential header lines into a list from a file pointer.

    Length of line == limited by _MAXLINE, && number of
    headers == limited by _MAXHEADERS.
    ";
        headers = [ ];
        while true  {
        line = fp . readline ( _MAXLINE + 1 );
        if len ( line ) > _MAXLINE {
        panic!("LineTooLong ( "header line" )");
        headers . append ( line );
        if len ( headers ) > _MAXHEADERS {
        panic!("HTTPException ( "got more than %d headers" % _MAXHEADERS )");
        if line in ( b "\r\n" , b "\n" , b "" ) {
        break;
        return  headers;
        pub fn parse_headers ( fp , _class = HTTPMessage )  {
        "Parses only RFC2822 headers from a file pointer.

    email Parser wants to see strings rather than bytes.
    But a TextIOWrapper around self.rfile would buffer too many bytes
    from the stream, bytes which we later need to read as bytes.
    So we read the correct bytes here, as bytes, for email Parser
    to parse.

    ";
        headers = _read_headers ( fp );
        hstring = b "" . join ( headers ) . decode ( "iso-8859-1" );
        return  email . parser . Parser ( _class = _class ) . parsestr ( hstring );
        class HTTPResponse ( io . BufferedIOBase ) ;
        pub fn __init__ ( &self, sock , debuglevel = 0 , method = None /* Option */ , url = None /* Option */ )  {
        self . fp = sock . makefile ( "rb" );
        self . debuglevel = debuglevel;
        self . _method = method;
        self . headers = self . msg = None /* Option */;
        self . version = _UNKNOWN;
        self . status = _UNKNOWN;
        self . reason = _UNKNOWN;
        self . chunked = _UNKNOWN;
        self . chunk_left = _UNKNOWN;
        self . length = _UNKNOWN;
        self . will_close = _UNKNOWN;
        pub fn _read_status ( self )  {
        line = str ( self . fp . readline ( _MAXLINE + 1 ) , "iso-8859-1" );
        if len ( line ) > _MAXLINE {
        panic!("LineTooLong ( "status line" )");
        if self . debuglevel > 0 {
        println!( "reply:" , repr ( line ) );
        if !line {
        panic!("RemoteDisconnected ( "Remote end closed connection without"");
        " response" );
        // try {
        version , status , reason = line . split ( None /* Option */ , 2 );
        // } catch  ValueError  {
        // try {
        version , status = line . split ( None /* Option */ , 1 );
        reason = "";
        // } catch  ValueError  {
        version = "";
        if !version . startswith ( "HTTP/" ) {
        self . _close_conn ( );
        panic!("BadStatusLine ( line )");
        // try {
        status = int ( status );
        if status < 100 || status > 999 {
        panic!("BadStatusLine ( line )");
        // } catch  ValueError  {
        panic!("BadStatusLine ( line )");
        return  version , status , reason;
        pub fn begin ( self )  {
        if self . headers is !None /* Option */ {
        return;
        while true  {
        version , status , reason = self . _read_status ( );
        if status != CONTINUE {
        break;
        skipped_headers = _read_headers ( self . fp );
        if self . debuglevel > 0 {
        println!( "headers:" , skipped_headers );
        del skipped_headers;
        self . code = self . status = status;
        self . reason = reason . strip ( );
        if version in ( "HTTP/1.0" , "HTTP/0.9" ) {
        self . version = 10;
        } else if version . startswith ( "HTTP/1." ) {
        self . version = 11;
        } else {
        panic!("UnknownProtocol ( version )");
        self . headers = self . msg = parse_headers ( self . fp );
        if self . debuglevel > 0 {
        for hdr , val in self . headers . items ( ) .iter() {
        println!( "header:" , hdr + ":" , val );
        tr_enc = self . headers . get ( "transfer-encoding" );
        if tr_enc && tr_enc . lower ( ) == "chunked" {
        self . chunked = true;
        self . chunk_left = None /* Option */;
        } else {
        self . chunked = false;
        self . will_close = self . _check_close ( );
        self . length = None /* Option */;
        length = self . headers . get ( "content-length" );
        if length && !self . chunked {
        // try {
        self . length = int ( length );
        // } catch  ValueError  {
        self . length = None /* Option */;
        } else {
        if self . length < 0 {
        self . length = None /* Option */;
        } else {
        self . length = None /* Option */;
        if ( status == NO_CONTENT || status == NOT_MODIFIED or {
        100 <= status < 200 or;
        self . _method == "HEAD" ) :;
        self . length = 0;
        if ( !self . will_close and {
        not self . chunked and;
        self . length is None /* Option */ ) :;
        self . will_close = true;
        pub fn _check_close ( self )  {
        conn = self . headers . get ( "connection" );
        if self . version == 11 {
        if conn && "close" in conn . lower ( ) {
        return  true;
        return  false;
        if self . headers . get ( "keep-alive" ) {
        return  false;
        if conn && "keep-alive" in conn . lower ( ) {
        return  false;
        pconn = self . headers . get ( "proxy-connection" );
        if pconn && "keep-alive" in pconn . lower ( ) {
        return  false;
        return  true;
        pub fn _close_conn ( self )  {
        fp = self . fp;
        self . fp = None /* Option */;
        fp . close ( );
        pub fn close ( self )  {
        // try {
        super ( ) . close ( );
        // } finally {
        if self . fp {
        self . _close_conn ( );
        pub fn flush ( self )  {
        super ( ) . flush ( );
        if self . fp {
        self . fp . flush ( );
        pub fn readable ( self )  {
        "Always returns true";
        return  true;
        pub fn isclosed ( self )  {
        "true if the connection == closed.";
        return  self . fp is None /* Option */;
        pub fn read ( &self, amt = None /* Option */ )  {
        "Read && return the response body, || up to the next amt bytes.";
        if self . fp is None /* Option */ {
        return  b "";
        if self . _method == "HEAD" {
        self . _close_conn ( );
        return  b "";
        if self . chunked {
        return  self . _read_chunked ( amt );
        if amt is !None /* Option */ {
        if self . length is !None /* Option */ && amt > self . length {
        amt = self . length;
        s = self . fp . read ( amt );
        if !s && amt {
        self . _close_conn ( );
        } else if self . length is !None /* Option */ {
        self . length - = len ( s );
        if !self . length {
        self . _close_conn ( );
        return  s;
        } else {
        if self . length is None /* Option */ {
        s = self . fp . read ( );
        } else {
        // try {
        s = self . _safe_read ( self . length );
        // } catch  IncompleteRead  {
        self . _close_conn ( );
        panic!("");
        self . length = 0;
        self . _close_conn ( );
        return  s;
        pub fn readinto ( &self, b )  {
        "Read up to len(b) bytes into bytearray b && return the number
        of bytes read.
        ";
        if self . fp is None /* Option */ {
        return  0;
        if self . _method == "HEAD" {
        self . _close_conn ( );
        return  0;
        if self . chunked {
        return  self . _readinto_chunked ( b );
        if self . length is !None /* Option */ {
        if len ( b ) > self . length {
        b = memoryview ( b ) [ 0 : self . length ];
        n = self . fp . readinto ( b );
        if !n && b {
        self . _close_conn ( );
        } else if self . length is !None /* Option */ {
        self . length - = n;
        if !self . length {
        self . _close_conn ( );
        return  n;
        pub fn _read_next_chunk_size ( self )  {
        line = self . fp . readline ( _MAXLINE + 1 );
        if len ( line ) > _MAXLINE {
        panic!("LineTooLong ( "chunk size" )");
        i = line . find ( b ";" );
        if i >= 0 {
        line = line [ : i ];
        // try {
        return  int ( line , 16 );
        // } catch  ValueError  {
        self . _close_conn ( );
        panic!("");
        pub fn _read_and_discard_trailer ( self )  {
        while true  {
        line = self . fp . readline ( _MAXLINE + 1 );
        if len ( line ) > _MAXLINE {
        panic!("LineTooLong ( "trailer line" )");
        if !line {
        break;
        if line in ( b "\r\n" , b "\n" , b "" ) {
        break;
        pub fn _get_chunk_left ( self )  {
        chunk_left = self . chunk_left;
        if !chunk_left {
        if chunk_left is !None /* Option */ {
        self . _safe_read ( 2 );
        // try {
        chunk_left = self . _read_next_chunk_size ( );
        // } catch  ValueError  {
        panic!("IncompleteRead ( b "" )");
        if chunk_left == 0 {
        self . _read_and_discard_trailer ( );
        self . _close_conn ( );
        chunk_left = None /* Option */;
        self . chunk_left = chunk_left;
        return  chunk_left;
        pub fn _read_chunked ( &self, amt = None /* Option */ )  {
        assert self . chunked != _UNKNOWN;
        value = [ ];
        // try {
        while true  {
        chunk_left = self . _get_chunk_left ( );
        if chunk_left is None /* Option */ {
        break;
        if amt is !None /* Option */ && amt <= chunk_left {
        value . append ( self . _safe_read ( amt ) );
        self . chunk_left = chunk_left - amt;
        break;
        value . append ( self . _safe_read ( chunk_left ) );
        if amt is !None /* Option */ {
        amt - = chunk_left;
        self . chunk_left = 0;
        return  b "" . join ( value );
        // } catch  IncompleteRead as exc  {
        panic!("IncompleteRead ( b "" . join ( value ) ) from exc");
        pub fn _readinto_chunked ( &self, b )  {
        assert self . chunked != _UNKNOWN;
        total_bytes = 0;
        mvb = memoryview ( b );
        // try {
        while true  {
        chunk_left = self . _get_chunk_left ( );
        if chunk_left is None /* Option */ {
        return  total_bytes;
        if len ( mvb ) <= chunk_left {
        n = self . _safe_readinto ( mvb );
        self . chunk_left = chunk_left - n;
        return  total_bytes + n;
        temp_mvb = mvb [ : chunk_left ];
        n = self . _safe_readinto ( temp_mvb );
        mvb = mvb [ n : ];
        total_bytes + = n;
        self . chunk_left = 0;
        // } catch  IncompleteRead  {
        panic!("IncompleteRead ( bytes ( b [ 0 : total_bytes ] ) )");
        pub fn _safe_read ( &self, amt )  {
        "Read the number of bytes requested.

        This function should be used when <amt> bytes "should" be present for
        reading. If the bytes are truly !available (due to EOF), then the
        IncompleteRead exception can be used to detect the problem.
        ";
        data = self . fp . read ( amt );
        if len ( data ) < amt {
        panic!("IncompleteRead ( data , amt - len ( data ) )");
        return  data;
        pub fn _safe_readinto ( &self, b )  {
        "Same as _safe_read, but for reading into a buffer.";
        amt = len ( b );
        n = self . fp . readinto ( b );
        if n < amt {
        panic!("IncompleteRead ( bytes ( b [ : n ] ) , amt - n )");
        return  n;
        pub fn read1 ( &self, n = -1 )  {
        "Read with at most one underlying system call.  If at least one
        byte == buffered, return that instead.
        ";
        if self . fp is None /* Option */ || self . _method == "HEAD" {
        return  b "";
        if self . chunked {
        return  self . _read1_chunked ( n );
        if self . length is !None /* Option */ && ( n < 0 || n > self . length ) {
        n = self . length;
        result = self . fp . read1 ( n );
        if !result && n {
        self . _close_conn ( );
        } else if self . length is !None /* Option */ {
        self . length - = len ( result );
        if !self . length {
        self . _close_conn ( );
        return  result;
        pub fn peek ( &self, n = -1 )  {
        if self . fp is None /* Option */ || self . _method == "HEAD" {
        return  b "";
        if self . chunked {
        return  self . _peek_chunked ( n );
        return  self . fp . peek ( n );
        pub fn readline ( &self, limit = -1 )  {
        if self . fp is None /* Option */ || self . _method == "HEAD" {
        return  b "";
        if self . chunked {
        return  super ( ) . readline ( limit );
        if self . length is !None /* Option */ && ( limit < 0 || limit > self . length ) {
        limit = self . length;
        result = self . fp . readline ( limit );
        if !result && limit {
        self . _close_conn ( );
        } else if self . length is !None /* Option */ {
        self . length - = len ( result );
        if !self . length {
        self . _close_conn ( );
        return  result;
        pub fn _read1_chunked ( &self, n )  {
        chunk_left = self . _get_chunk_left ( );
        if chunk_left is None /* Option */ || n == 0 {
        return  b "";
        if !( 0 <= n <= chunk_left ) {
        n = chunk_left;
        read = self . fp . read1 ( n );
        self . chunk_left - = len ( read );
        if !read {
        panic!("IncompleteRead ( b "" )");
        return  read;
        pub fn _peek_chunked ( &self, n )  {
        // try {
        chunk_left = self . _get_chunk_left ( );
        // } catch  IncompleteRead  {
        return  b "";
        if chunk_left is None /* Option */ {
        return  b "";
        return  self . fp . peek ( chunk_left ) [ : chunk_left ];
        pub fn fileno ( self )  {
        return  self . fp . fileno ( );
        pub fn getheader ( &self, name , default = None /* Option */ )  {
        "Returns the value of the header matching *name*.

        If there are multiple matching headers, the values are
        combined into a single string separated by commas && spaces.

        If no matching header == found, returns *default* || None /* Option */ if
        the *default* == !specified.

        If the headers are unknown, raises http.client.ResponseNotReady.

        ";
        if self . headers is None /* Option */ {
        panic!("ResponseNotReady ( )");
        headers = self . headers . get_all ( name ) || default;
        if isinstance ( headers , str ) || !hasattr ( headers , "__iter__" ) {
        return  headers;
        } else {
        return  ", " . join ( headers );
        pub fn getheaders ( self )  {
        "Return list of (header, value) tuples.";
        if self . headers is None /* Option */ {
        panic!("ResponseNotReady ( )");
        return  list ( self . headers . items ( ) );
        pub fn __iter__ ( self )  {
        return  self;
        pub fn info ( self )  {
        "Returns an instance of the class mimetools.Message containing
        meta-information associated with the URL.

        When the method == HTTP, these headers are those returned by
        the server at the head of the retrieved HTML page (including
        Content-Length && Content-Type).

        When the method == FTP, a Content-Length header will be
        present if (as == now usual) the server passed back a file
        length in response to the FTP retrieval request. A
        Content-Type header will be present if the MIME type can be
        guessed.

        When the method == local-file, returned headers will include
        a Date representing the file's last-modified time, a
        Content-Length giving file size, && a Content-Type
        containing a guess at the file's type. See also the
        description of the mimetools module.

        ";
        return  self . headers;
        pub fn geturl ( self )  {
        "Return the real URL of the page.

        In some cases, the HTTP server redirects a client to another
        URL. The urlopen() function handles this transparently, but in
        some cases the caller needs to know which URL the client was
        redirected to. The geturl() method can be used to get at this
        redirected URL.

        ";
        return  self . url;
        pub fn getcode ( self )  {
        "Return the HTTP status code that was sent with the response,
        || None /* Option */ if the URL == !an HTTP URL.

        ";
        return  self . status;
        class HTTPConnection ;
        _http_vsn = 11;
        _http_vsn_str = "HTTP/1.1";
        response_class = HTTPResponse;
        default_port = HTTP_PORT;
        auto_open = 1;
        debuglevel = 0;
        @ staticmethod;
        pub fn _is_textIO ( stream )  {
        "Test whether a file-like object == a text || a binary stream.
        ";
        return  isinstance ( stream , io . TextIOBase );
        @ staticmethod;
        pub fn _get_content_length ( body , method )  {
        "Get the content-length based on the body.

        If the body == None /* Option */, we set Content-Length: 0 for methods that expect
        a body (RFC 7230, Section 3.3.2). We also set the Content-Length for
        any method if the body == a str || bytes-like object && !a file.
        ";
        if body is None /* Option */ {
        if method . upper ( ) in _METHODS_EXPECTING_BODY {
        return  0;
        } else {
        return;
        if hasattr ( body , "read" ) {
        return;
        // try {
        mv = memoryview ( body );
        return  mv . nbytes;
        // } catch  TypeError  {
        // pass
        if isinstance ( body , str ) {
        return  len ( body );
        return;
        pub fn __init__ ( &self, host , port = None /* Option */ , timeout = socket . _GLOBAL_DEFAULT_TIMEOUT , {
        source_address = None /* Option */ , blocksize = 8192 ) ;
        self . timeout = timeout;
        self . source_address = source_address;
        self . blocksize = blocksize;
        self . sock = None /* Option */;
        self . _buffer = [ ];
        self . __response = None /* Option */;
        self . __state = _CS_IDLE;
        self . _method = None /* Option */;
        self . _tunnel_host = None /* Option */;
        self . _tunnel_port = None /* Option */;
        self . _tunnel_headers = { };
        ( self . host , self . port ) = self . _get_hostport ( host , port );
        self . _validate_host ( self . host );
        self . _create_connection = socket . create_connection;
        pub fn set_tunnel ( &self, host , port = None /* Option */ , headers = None /* Option */ )  {
        "Set up host && port for HTTP CONNECT tunnelling.

        In a connection that uses HTTP CONNECT tunneling, the host passed to the
        constructor == used as a proxy server that relays all communication to
        the endpoint passed to `set_tunnel`. This done by sending an HTTP
        CONNECT request to the proxy server when the connection == established.

        This method must be called before the HTTP connection has been
        established.

        The headers argument should be a mapping of extra HTTP headers to send
        with the CONNECT request.
        ";
        if self . sock {
        panic!("RuntimeError ( "Can't set up tunnel for established connection" )");
        self . _tunnel_host , self . _tunnel_port = self . _get_hostport ( host , port );
        if headers {
        self . _tunnel_headers = headers;
        } else {
        self . _tunnel_headers . clear ( );
        pub fn _get_hostport ( &self, host , port )  {
        if port is None /* Option */ {
        i = host . rfind ( ":" );
        j = host . rfind ( "]" );
        if i > j {
        // try {
        port = int ( host [ i + 1 : ] );
        // } catch  ValueError  {
        if host [ i + 1 { : ] == "" ; }
        port = self . default_port;
        } else {
        panic!("InvalidURL ( "nonnumeric port: '%s'" % host [ i + 1 : ] )");
        host = host [ : i ];
        } else {
        port = self . default_port;
        if host && host [ 0 ] == "[" && host [ -1 ] == "]" {
        host = host [ 1 : -1 ];
        return  ( host , port );
        pub fn set_debuglevel ( &self, level )  {
        self . debuglevel = level;
        pub fn _wrap_ipv6 ( &self, ip )  {
        if b ":" in ip && ip [ 0 ] != b "[" [ 0 ] {
        return  b "[" + ip + b "]";
        return  ip;
        pub fn _tunnel ( self )  {
        connect = b "CONNECT %s:%d HTTP/1.0\r\n" % (;
        self . _wrap_ipv6 ( self . _tunnel_host . encode ( "ascii" ) ) ,;
        self . _tunnel_port );
        headers = [ connect ];
        for header , value in self . _tunnel_headers . items ( ) .iter() {
        headers . append ( format!("{header}: {value}\r\n" . encode ( "latin-1" ) ));
        headers . append ( b "\r\n" );
        self . send ( b "" . join ( headers ) );
        del headers;
        response = self . response_class ( self . sock , method = self . _method );
        // try {
        ( version , code , message ) = response . _read_status ( );
        if code != http . HTTPStatus . OK {
        self . close ( );
        panic!("OSError ( f "Tunnel connection failed: {code} {message.strip()}" )");
        while true  {
        line = response . fp . readline ( _MAXLINE + 1 );
        if len ( line ) > _MAXLINE {
        panic!("LineTooLong ( "header line" )");
        if !line {
        break;
        if line in ( b "\r\n" , b "\n" , b "" ) {
        break;
        if self . debuglevel > 0 {
        println!( "header:" , line . decode ( ) );
        // } finally {
        response . close ( );
        pub fn connect ( self )  {
        "Connect to the host && port specified in __init__.";
        sys . audit ( "http.client.connect" , self , self . host , self . port );
        self . sock = self . _create_connection (;
        ( self . host , self . port ) , self . timeout , self . source_address );
        // try {
        self . sock . setsockopt ( socket . IPPROTO_TCP , socket . TCP_NODELAY , 1 );
        // } catch  OSError as e  {
        if e . errno != errno . ENOPROTOOPT {
        panic!("");
        if self . _tunnel_host {
        self . _tunnel ( );
        pub fn close ( self )  {
        "Close the connection to the HTTP server.";
        self . __state = _CS_IDLE;
        // try {
        sock = self . sock;
        if sock {
        self . sock = None /* Option */;
        sock . close ( );
        // } finally {
        response = self . __response;
        if response {
        self . __response = None /* Option */;
        response . close ( );
        pub fn send ( &self, data )  {
        "Send `data' to the server.
        ``data`` can be a string object, a bytes object, an array object, a
        file-like object that supports a .read() method, || an iterable object.
        ";
        if self . sock is None /* Option */ {
        if self . auto_open {
        self . connect ( );
        } else {
        panic!("NotConnected ( )");
        if self . debuglevel > 0 {
        println!( "send:" , repr ( data ) );
        if hasattr ( data , "read" ) {
        if self . debuglevel > 0 {
        println!( "sending a readable" );
        encode = self . _is_textIO ( data );
        if encode && self . debuglevel > 0 {
        println!( "encoding file using iso-8859-1" );
        while 1  {
        datablock = data . read ( self . blocksize );
        if !datablock {
        break;
        if encode {
        datablock = datablock . encode ( "iso-8859-1" );
        sys . audit ( "http.client.send" , self , datablock );
        self . sock . sendall ( datablock );
        return;
        sys . audit ( "http.client.send" , self , data );
        // try {
        self . sock . sendall ( data );
        // } catch  TypeError  {
        if isinstance ( data , collections . abc . Iterable ) {
        for d in data .iter() {
        self . sock . sendall ( d );
        } else {
        panic!("TypeError ( "data should be a bytes-like object "");
        "or an iterable, got %r" % type ( data ) );
        pub fn _output ( &self, s )  {
        "Add a line of output to the current request buffer.

        Assumes that the line does *not* end with \\r\\n.
        ";
        self . _buffer . append ( s );
        pub fn _read_readable ( &self, readable )  {
        if self . debuglevel > 0 {
        println!( "reading a readable" );
        encode = self . _is_textIO ( readable );
        if encode && self . debuglevel > 0 {
        println!( "encoding file using iso-8859-1" );
        while true  {
        datablock = readable . read ( self . blocksize );
        if !datablock {
        break;
        if encode {
        datablock = datablock . encode ( "iso-8859-1" );
        yield datablock;
        pub fn _send_output ( &self, message_body = None /* Option */ , encode_chunked = false )  {
        "Send the currently buffered request && clear the buffer.

        Appends an extra \\r\\n to the buffer.
        A message_body may be specified, to be appended to the request.
        ";
        self . _buffer . extend ( ( b "" , b "" ) );
        msg = b "\r\n" . join ( self . _buffer );
        del self . _buffer [ : ];
        self . send ( msg );
        if message_body is !None /* Option */ {
        if hasattr ( message_body , "read" ) {
        chunks = self . _read_readable ( message_body );
        } else {
        // try {
        memoryview ( message_body );
        // } catch  TypeError  {
        // try {
        chunks = iter ( message_body );
        // } catch  TypeError  {
        panic!("TypeError ( "message_body should be a bytes-like "");
        "object || an iterable, got %r";
        % type ( message_body ) );
        } else {
        chunks = ( message_body , );
        for chunk in chunks .iter() {
        if !chunk {
        if self . debuglevel > 0 {
        println!( "Zero length chunk ignored" );
        continue;
        if encode_chunked && self . _http_vsn == 11 {
        chunk = format!("{len(chunk):X}\r\n" . encode ( "ascii" ) + chunk \);
        + b "\r\n";
        self . send ( chunk );
        if encode_chunked && self . _http_vsn == 11 {
        self . send ( b "0\r\n\r\n" );
        pub fn putrequest ( &self, method , url , skip_host = false , {
        skip_accept_encoding = false ) ;
        "Send a request to the server.

        `method' specifies an HTTP request method, e.g. 'GET'.
        `url' specifies the object being requested, e.g. '/index.html'.
        `skip_host' if true does !add automatically a 'Host:' header
        `skip_accept_encoding' if true does !add automatically an
           'Accept-Encoding:' header
        ";
        if self . __response && self . __response . isclosed ( ) {
        self . __response = None /* Option */;
        if self . __state == _CS_IDLE {
        self . __state = _CS_REQ_STARTED;
        } else {
        panic!("CannotSendRequest ( self . __state )");
        self . _validate_method ( method );
        self . _method = method;
        url = url || "/";
        self . _validate_path ( url );
        request = "%s %s %s" % ( method , url , self . _http_vsn_str );
        self . _output ( self . _encode_request ( request ) );
        if self . _http_vsn == 11 {
        if !skip_host {
        netloc = "";
        if url . startswith ( "http" ) {
        nil , netloc , nil , nil , nil = urlsplit ( url );
        if netloc {
        // try {
        netloc_enc = netloc . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        netloc_enc = netloc . encode ( "idna" );
        self . putheader ( "Host" , _strip_ipv6_iface ( netloc_enc ) );
        } else {
        if self . _tunnel_host {
        host = self . _tunnel_host;
        port = self . _tunnel_port;
        } else {
        host = self . host;
        port = self . port;
        // try {
        host_enc = host . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        host_enc = host . encode ( "idna" );
        host_enc = self . _wrap_ipv6 ( host_enc );
        if ":" in host {
        host_enc = _strip_ipv6_iface ( host_enc );
        if port == self . default_port {
        self . putheader ( "Host" , host_enc );
        } else {
        host_enc = host_enc . decode ( "ascii" );
        self . putheader ( "Host" , "%s:%s" % ( host_enc , port ) );
        if !skip_accept_encoding {
        self . putheader ( "Accept-Encoding" , "identity" );
        } else {
        // pass
        pub fn _encode_request ( &self, request )  {
        return  request . encode ( "ascii" );
        pub fn _validate_method ( &self, method )  {
        "Validate a method name for putrequest.";
        match = _contains_disallowed_method_pchar_re . search ( method );
        if match {
        panic!("ValueError (");
        format!("method can't contain control characters. {method!r} ");
        format!("(found at least {match.group()!r})" ));
        pub fn _validate_path ( &self, url )  {
        "Validate a url for putrequest.";
        match = _contains_disallowed_url_pchar_re . search ( url );
        if match {
        panic!("InvalidURL ( f "URL can't contain control characters. {url!r} "");
        format!("(found at least {match.group()!r})" ));
        pub fn _validate_host ( &self, host )  {
        "Validate a host so it doesn't contain control characters.";
        match = _contains_disallowed_url_pchar_re . search ( host );
        if match {
        panic!("InvalidURL ( f "URL can't contain control characters. {host!r} "");
        format!("(found at least {match.group()!r})" ));
        pub fn putheader ( &self, header , * values )  {
        "Send a request header line to the server.

        For example: h.putheader('Accept', 'text/html')
        ";
        if self . __state != _CS_REQ_STARTED {
        panic!("CannotSendHeader ( )");
        if hasattr ( header , "encode" ) {
        header = header . encode ( "ascii" );
        if !_is_legal_header_name ( header ) {
        panic!("ValueError ( "Invalid header name %r" % ( header , ) )");
        values = list ( values );
        for i , one_value in enumerate ( values ) .iter() {
        if hasattr ( one_value , "encode" ) {
        values [ i ] = one_value . encode ( "latin-1" );
        } else if isinstance ( one_value , int ) {
        values [ i ] = str ( one_value ) . encode ( "ascii" );
        if _is_illegal_header_value ( values [ i ] ) {
        panic!("ValueError ( "Invalid header value %r" % ( values [ i ] , ) )");
        value = b "\r\n\t" . join ( values );
        header = header + b ": " + value;
        self . _output ( header );
        pub fn endheaders ( &self, message_body = None /* Option */ , * , encode_chunked = false )  {
        "Indicate that the last header line has been sent to the server.

        This method sends the request to the server.  The optional message_body
        argument can be used to pass a message body associated with the
        request.
        ";
        if self . __state == _CS_REQ_STARTED {
        self . __state = _CS_REQ_SENT;
        } else {
        panic!("CannotSendHeader ( )");
        self . _send_output ( message_body , encode_chunked = encode_chunked );
        pub fn request ( &self, method , url , body = None /* Option */ , headers = { } , * , {
        encode_chunked = false ) ;
        "Send a complete request to the server.";
        self . _send_request ( method , url , body , headers , encode_chunked );
        pub fn _send_request ( &self, method , url , body , headers , encode_chunked )  {
        header_names = frozenset ( k . lower ( ) for k in headers );
        skips = { };
        if "host" in header_names {
        skips [ "skip_host" ] = 1;
        if "accept-encoding" in header_names {
        skips [ "skip_accept_encoding" ] = 1;
        self . putrequest ( method , url , ** skips );
        if "content-length" !in header_names {
        if "transfer-encoding" !in header_names {
        encode_chunked = false;
        content_length = self . _get_content_length ( body , method );
        if content_length is None /* Option */ {
        if body is !None /* Option */ {
        if self . debuglevel > 0 {
        println!( "Unable to determine size of %r" % body );
        encode_chunked = true;
        self . putheader ( "Transfer-Encoding" , "chunked" );
        } else {
        self . putheader ( "Content-Length" , str ( content_length ) );
        } else {
        encode_chunked = false;
        for hdr , value in headers . items ( ) .iter() {
        self . putheader ( hdr , value );
        if isinstance ( body , str ) {
        body = _encode ( body , "body" );
        self . endheaders ( body , encode_chunked = encode_chunked );
        pub fn getresponse ( self )  {
        "Get the response from the server.

        If the HTTPConnection == in the correct state, returns an
        instance of HTTPResponse || of whatever object == returned by
        the response_class variable.

        If a request has !been sent || if a previous response has
        !be handled, ResponseNotReady == raised.  If the HTTP
        response indicates that the connection should be closed, then
        it will be closed before the response == returned.  When the
        connection == closed, the underlying socket == closed.
        ";
        if self . __response && self . __response . isclosed ( ) {
        self . __response = None /* Option */;
        if self . __state != _CS_REQ_SENT || self . __response {
        panic!("ResponseNotReady ( self . __state )");
        if self . debuglevel > 0 {
        response = self . response_class ( self . sock , self . debuglevel ,;
        method = self . _method );
        } else {
        response = self . response_class ( self . sock , method = self . _method );
        // try {
        // try {
        response . begin ( );
        // } catch  ConnectionError  {
        self . close ( );
        panic!("");
        assert response . will_close != _UNKNOWN;
        self . __state = _CS_IDLE;
        if response . will_close {
        self . close ( );
        } else {
        self . __response = response;
        return  response;
        // } catch   {
        response . close ( );
        panic!("");
        // try {
        import ssl;
        // } catch  ImportError  {
        // pass
        } else {
        class HTTPSConnection ( HTTPConnection ) ;
        "This class allows communication via SSL.";
        default_port = HTTPS_PORT;
        pub fn __init__ ( &self, host , port = None /* Option */ , key_file = None /* Option */ , cert_file = None /* Option */ , {
        timeout = socket . _GLOBAL_DEFAULT_TIMEOUT ,;
        source_address = None /* Option */ , * , context = None /* Option */ ,;
        check_hostname = None /* Option */ , blocksize = 8192 ) ;
        super ( HTTPSConnection , self ) . __init__ ( host , port , timeout ,;
        source_address ,;
        blocksize = blocksize );
        if ( key_file is !None /* Option */ || cert_file is !None /* Option */ or {
        check_hostname == !None /* Option */ ) ;
        import warnings;
        warnings . warn ( "key_file, cert_file && check_hostname are ";
        "deprecated, use a custom context instead." ,;
        DeprecationWarning , 2 );
        self . key_file = key_file;
        self . cert_file = cert_file;
        if context is None /* Option */ {
        context = ssl . _create_default_https_context ( );
        if self . _http_vsn == 11 {
        context . set_alpn_protocols ( [ "http/1.1" ] );
        if context . post_handshake_auth is !None /* Option */ {
        context . post_handshake_auth = true;
        will_verify = context . verify_mode != ssl . CERT_NONE;
        if check_hostname is None /* Option */ {
        check_hostname = context . check_hostname;
        if check_hostname && !will_verify {
        panic!("ValueError ( "check_hostname needs a SSL context with "");
        "either CERT_OPTIONAL || CERT_REQUIRED" );
        if key_file || cert_file {
        context . load_cert_chain ( cert_file , key_file );
        if context . post_handshake_auth is !None /* Option */ {
        context . post_handshake_auth = true;
        self . _context = context;
        if check_hostname is !None /* Option */ {
        self . _context . check_hostname = check_hostname;
        pub fn connect ( self )  {
        "Connect to a host on a given (SSL) port.";
        super ( ) . connect ( );
        if self . _tunnel_host {
        server_hostname = self . _tunnel_host;
        } else {
        server_hostname = self . host;
        self . sock = self . _context . wrap_socket ( self . sock ,;
        server_hostname = server_hostname );
        __all__ . append ( "HTTPSConnection" );
        class HTTPException ( Exception ) ;
        // pass
        class NotConnected ( HTTPException ) ;
        // pass
        class InvalidURL ( HTTPException ) ;
        // pass
        class UnknownProtocol ( HTTPException ) ;
        pub fn __init__ ( &self, version )  {
        self . args = version ,;
        self . version = version;
        class UnknownTransferEncoding ( HTTPException ) ;
        // pass
        class UnimplementedFileMode ( HTTPException ) ;
        // pass
        class IncompleteRead ( HTTPException ) ;
        pub fn __init__ ( &self, partial , expected = None /* Option */ )  {
        self . args = partial ,;
        self . partial = partial;
        self . expected = expected;
        pub fn __repr__ ( self )  {
        if self . expected is !None /* Option */ {
        e = ", %i more expected" % self . expected;
        } else {
        e = "";
        return  "%s(%i bytes read%s)" % ( self . __class__ . __name__ ,;
        len ( self . partial ) , e );
        __str__ = object . __str__;
        class ImproperConnectionState ( HTTPException ) ;
        // pass
        class CannotSendRequest ( ImproperConnectionState ) ;
        // pass
        class CannotSendHeader ( ImproperConnectionState ) ;
        // pass
        class ResponseNotReady ( ImproperConnectionState ) ;
        // pass
        class BadStatusLine ( HTTPException ) ;
        pub fn __init__ ( &self, line )  {
        if !line {
        line = repr ( line );
        self . args = line ,;
        self . line = line;
        class LineTooLong ( HTTPException ) ;
        pub fn __init__ ( &self, line_type )  {
        HTTPException . __init__ ( self , "got more than %d bytes when reading %s";
        % ( _MAXLINE , line_type ) );
        class RemoteDisconnected ( ConnectionResetError , BadStatusLine ) ;
        pub fn __init__ ( &self, * pos , ** kw )  {
        BadStatusLine . __init__ ( self , "" );
        ConnectionResetError . __init__ ( self , * pos , ** kw );
        error = HTTPException;
}

