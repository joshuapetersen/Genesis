//! server.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::copy;
// use crate::email;
// use crate::http;
// use crate::itertools;
// use std::fs;
// use crate::select;
// use crate::socket;
// use std::env;
// use crate::urllib;
// use crate::pwd;
// use crate::base64;
// use crate::subprocess;
// use crate::argparse;

pub const __version__: &str = "0.6";
pub const __all__: f64 = [;
pub const DEFAULT_ERROR_MESSAGE: &str = "\
<!DOCTYPE HTML>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Error response</title>
    </head>
    <body>
        <h1>Error response</h1>
        <p>Error code: %(code)d</p>
        <p>Message: %(message)s.</p>
        <p>Error code explanation: %(code)s - %(explain)s.</p>
    </body>
</html>
";
pub const DEFAULT_ERROR_CONTENT_TYPE: &str = "text/html;charset=utf-8";
pub struct HTTPServer {
    pub server_name: String, // TODO: infer type
    pub server_port: String, // TODO: infer type
    pub command: String, // TODO: infer type
    pub request_version: String, // TODO: infer type
    pub close_connection: String, // TODO: infer type
    pub requestline: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub headers: String, // TODO: infer type
    pub raw_requestline: String, // TODO: infer type
    pub _headers_buffer: String, // TODO: infer type
    pub directory: String, // TODO: infer type
    pub cgi_info: String, // TODO: infer type
}

impl HTTPServer {
}

pub struct ThreadingHTTPServer {
    pub command: String, // TODO: infer type
    pub request_version: String, // TODO: infer type
    pub close_connection: String, // TODO: infer type
    pub requestline: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub headers: String, // TODO: infer type
    pub raw_requestline: String, // TODO: infer type
    pub _headers_buffer: String, // TODO: infer type
    pub directory: String, // TODO: infer type
    pub cgi_info: String, // TODO: infer type
}

impl ThreadingHTTPServer {
}

pub struct BaseHTTPRequestHandler {
    pub command: String, // TODO: infer type
    pub request_version: String, // TODO: infer type
    pub close_connection: String, // TODO: infer type
    pub requestline: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub headers: String, // TODO: infer type
    pub raw_requestline: String, // TODO: infer type
    pub _headers_buffer: String, // TODO: infer type
    pub directory: String, // TODO: infer type
    pub cgi_info: String, // TODO: infer type
}

impl BaseHTTPRequestHandler {
}

pub struct SimpleHTTPRequestHandler {
    pub directory: String, // TODO: infer type
    pub cgi_info: String, // TODO: infer type
}

impl SimpleHTTPRequestHandler {
}

pub fn _url_collapse_path(path: &str) {
        "
    Given a URL path, remove extra '/'s && '.' path elements && collapse
    any '..' references && returns a collapsed path.

    Implements something akin to RFC-2396 5.2 step 6 to parse relative paths.
    The utility of this function == limited to is_cgi method && helps
    preventing some security attacks.

    Returns: The reconstituted URL, which will always start with a '/'.

    Raises: IndexError if too many '..' occur within the path.

    ";
        path , _ , query = path . partition ( "?" );
        path = urllib . parse . unquote ( path );
        path_parts = path . split ( "/" );
        head_parts = [ ];
        for part in path_parts [ : -1 ] .iter() {
        if part == ".." {
        head_parts . pop ( );
        } else if part && part != "." {
        head_parts . append ( part );
        if path_parts {
        tail_part = path_parts . pop ( );
        if tail_part {
        if tail_part == ".." {
        head_parts . pop ( );
        tail_part = "";
        } else if tail_part == "." {
        tail_part = "";
        } else {
        tail_part = "";
        if query {
        tail_part = "?" . join ( ( tail_part , query ) );
        splitpath = ( "/" + "/" . join ( head_parts ) , tail_part );
        collapsed_path = "/" . join ( splitpath );
        return  collapsed_path;
        nobody = None /* Option */;
        pub fn nobody_uid ( )  {
        "Internal routine to get nobody's uid";
        global nobody;
        if nobody {
        return  nobody;
        // try {
        import pwd;
        // } catch  ImportError  {
        return  -1;
        // try {
        nobody = pwd . getpwnam ( "nobody" ) [ 2 ];
        // } catch  KeyError  {
        nobody = 1 + max ( x vec![ 2 ].iter().map(|x| pwd . getpwall ( ) );
        return  nobody;
        pub fn executable ( path )  {
        "Test for executable file.";
        return  os . access ( path , os . X_OK );
        class CGIHTTPRequestHandler ( SimpleHTTPRequestHandler ) ;
        "Complete HTTP server with GET, HEAD && POST commands.

    GET && HEAD also support running CGI scripts.

    The POST command == *only* implemented for CGI scripts.

    ";
        have_fork = hasattr ( os , "fork" );
        rbufsize = 0;
        pub fn do_POST ( self )  {
        "Serve a POST request.

        This == only implemented for CGI scripts.

        ";
        if self . is_cgi ( ) {
        self . run_cgi ( );
        } else {
        self . send_error (;
        HTTPStatus . NOT_IMPLEMENTED ,;
        "Can only POST to CGI scripts" );
        pub fn send_head ( self )  {
        "Version of send_head that support CGI scripts";
        if self . is_cgi ( ) {
        return  self . run_cgi ( );
        } else {
        return  SimpleHTTPRequestHandler . send_head ( self );
        pub fn is_cgi ( self )  {
        "Test whether self.path corresponds to a CGI script.

        Returns true && updates the cgi_info attribute to the tuple
        (dir, rest) if self.path requires running a CGI script.
        Returns false otherwise.

        If any exception == raised, the caller should assume that
        self.path was rejected as invalid && act accordingly.

        The default implementation tests whether the normalized url
        path begins with one of the strings in self.cgi_directories
        (and the next character == a '/' || the end of the string).

        ";
        collapsed_path = _url_collapse_path ( self . path );
        dir_sep = collapsed_path . find ( "/" , 1 );
        while dir_sep > 0 && !collapsed_path [ : dir_sep ] in self . cgi_directories  {
        dir_sep = collapsed_path . find ( "/" , dir_sep + 1 );
        if dir_sep > 0 {
        head , tail = collapsed_path [ : dir_sep ] , collapsed_path [ dir_sep + 1 : ];
        self . cgi_info = head , tail;
        return  true;
        return  false;
        cgi_directories = [ "/cgi-bin" , "/htbin" ];
        pub fn is_executable ( &self, path )  {
        "Test whether argument path == an executable file.";
        return  executable ( path );
        pub fn is_python ( &self, path )  {
        "Test whether argument path == a Python script.";
        head , tail = os . path . splitext ( path );
        return  tail . lower ( ) in ( ".py" , ".pyw" );
        pub fn run_cgi ( self )  {
        "Execute a CGI script.";
        dir , rest = self . cgi_info;
        path = dir + "/" + rest;
        i = path . find ( "/" , len ( dir ) + 1 );
        while i >= 0  {
        nextdir = path [ : i ];
        nextrest = path [ i + 1 : ];
        scriptdir = self . translate_path ( nextdir );
        if os . path . isdir ( scriptdir ) {
        dir , rest = nextdir , nextrest;
        i = path . find ( "/" , len ( dir ) + 1 );
        } else {
        break;
        rest , _ , query = rest . partition ( "?" );
        i = rest . find ( "/" );
        if i >= 0 {
        script , rest = rest [ : i ] , rest [ i : ];
        } else {
        script , rest = rest , "";
        scriptname = dir + "/" + script;
        scriptfile = self . translate_path ( scriptname );
        if !os . path . exists ( scriptfile ) {
        self . send_error (;
        HTTPStatus . NOT_FOUND ,;
        "No such CGI script (%r)" % scriptname );
        return;
        if !os . path . isfile ( scriptfile ) {
        self . send_error (;
        HTTPStatus . FORBIDDEN ,;
        "CGI script == !a plain file (%r)" % scriptname );
        return;
        ispy = self . is_python ( scriptname );
        if self . have_fork || !ispy {
        if !self . is_executable ( scriptfile ) {
        self . send_error (;
        HTTPStatus . FORBIDDEN ,;
        "CGI script == !executable (%r)" % scriptname );
        return;
        env = copy . deepcopy ( os . environ );
        env [ "SERVER_SOFTWARE" ] = self . version_string ( );
        env [ "SERVER_NAME" ] = self . server . server_name;
        env [ "GATEWAY_INTERFACE" ] = "CGI/1.1";
        env [ "SERVER_PROTOCOL" ] = self . protocol_version;
        env [ "SERVER_PORT" ] = str ( self . server . server_port );
        env [ "REQUEST_METHOD" ] = self . command;
        uqrest = urllib . parse . unquote ( rest );
        env [ "PATH_INFO" ] = uqrest;
        env [ "PATH_TRANSLATED" ] = self . translate_path ( uqrest );
        env [ "SCRIPT_NAME" ] = scriptname;
        env [ "QUERY_STRING" ] = query;
        env [ "REMOTE_ADDR" ] = self . client_address [ 0 ];
        authorization = self . headers . get ( "authorization" );
        if authorization {
        authorization = authorization . split ( );
        if len ( authorization ) == 2 {
        import base64 , binascii;
        env [ "AUTH_TYPE" ] = authorization [ 0 ];
        if authorization [ 0 ] . lower ( ) == "basic" {
        // try {
        authorization = authorization [ 1 ] . encode ( "ascii" );
        authorization = base64 . decodebytes ( authorization ) . \;
        decode ( "ascii" );
        // } catch  ( binascii . Error , UnicodeError )  {
        // pass
        } else {
        authorization = authorization . split ( ":" );
        if len ( authorization ) == 2 {
        env [ "REMOTE_USER" ] = authorization [ 0 ];
        if self . headers . get ( "content-type" ) is None /* Option */ {
        env [ "CONTENT_TYPE" ] = self . headers . get_content_type ( );
        } else {
        env [ "CONTENT_TYPE" ] = self . headers [ "content-type" ];
        length = self . headers . get ( "content-length" );
        if length {
        env [ "CONTENT_LENGTH" ] = length;
        referer = self . headers . get ( "referer" );
        if referer {
        env [ "HTTP_REFERER" ] = referer;
        accept = self . headers . get_all ( "accept" , ( ) );
        env [ "HTTP_ACCEPT" ] = "," . join ( accept );
        ua = self . headers . get ( "user-agent" );
        if ua {
        env [ "HTTP_USER_AGENT" ] = ua;
        co = filter ( None /* Option */ , self . headers . get_all ( "cookie" , [ ] ) );
        cookie_str = ", " . join ( co );
        if cookie_str {
        env [ "HTTP_COOKIE" ] = cookie_str;
        for k in ( "QUERY_STRING" , "REMOTE_HOST" , "CONTENT_LENGTH" ,.iter() {
        "HTTP_USER_AGENT" , "HTTP_COOKIE" , "HTTP_REFERER" ) ;
        env . setdefault ( k , "" );
        self . send_response ( HTTPStatus . OK , "Script output follows" );
        self . flush_headers ( );
        decoded_query = query . replace ( "+" , " " );
        if self . have_fork {
        args = [ script ];
        if "=" !in decoded_query {
        args . append ( decoded_query );
        nobody = nobody_uid ( );
        self . wfile . flush ( );
        pid = os . fork ( );
        if pid != 0 {
        pid , sts = os . waitpid ( pid , 0 );
        while select . select ( [ self . rfile ] , [ ] , [ ] , 0 ) [ 0 ]  {
        if !self . rfile . read ( 1 ) {
        break;
        exitcode = os . waitstatus_to_exitcode ( sts );
        if exitcode {
        self . log_error ( f "CGI script exit code {exitcode}" );
        return;
        // try {
        // try {
        os . setuid ( nobody );
        // } catch  OSError  {
        // pass
        os . dup2 ( self . rfile . fileno ( ) , 0 );
        os . dup2 ( self . wfile . fileno ( ) , 1 );
        os . execve ( scriptfile , args , env );
        // } catch   {
        self . server . handle_error ( self . request , self . client_address );
        os . _exit ( 127 );
        } else {
        import subprocess;
        cmdline = [ scriptfile ];
        if self . is_python ( scriptfile ) {
        interp = sys . executable;
        if interp . lower ( ) . endswith ( "w.exe" ) {
        interp = interp [ : -5 ] + interp [ -4 : ];
        cmdline = [ interp , "-u" ] + cmdline;
        if "=" !in query {
        cmdline . append ( query );
        self . log_message ( "command: %s" , subprocess . list2cmdline ( cmdline ) );
        // try {
        nbytes = int ( length );
        // } catch  ( TypeError , ValueError )  {
        nbytes = 0;
        p = subprocess . Popen ( cmdline ,;
        stdin = subprocess . PIPE ,;
        stdout = subprocess . PIPE ,;
        stderr = subprocess . PIPE ,;
        env = env;
        );
        if self . command . lower ( ) == "post" && nbytes > 0 {
        data = self . rfile . read ( nbytes );
        } else {
        data = None /* Option */;
        while select . select ( [ self . rfile . _sock ] , [ ] , [ ] , 0 ) [ 0 ]  {
        if !self . rfile . _sock . recv ( 1 ) {
        break;
        stdout , stderr = p . communicate ( data );
        self . wfile . write ( stdout );
        if stderr {
        self . log_error ( "%s" , stderr );
        p . stderr . close ( );
        p . stdout . close ( );
        status = p . returncode;
        if status {
        self . log_error ( "CGI script exit status %#x" , status );
        } else {
        self . log_message ( "CGI script exited OK" );
        pub fn _get_best_family ( * address )  {
        infos = socket . getaddrinfo (;
        * address ,;
        type = socket . SOCK_STREAM ,;
        flags = socket . AI_PASSIVE ,;
        );
        family , type , proto , canonname , sockaddr = next ( iter ( infos ) );
        return  family , sockaddr;
        pub fn test ( HandlerClass = BaseHTTPRequestHandler , {
        ServerClass = ThreadingHTTPServer ,;
        protocol = "HTTP/1.0" , port = 8000 , bind = None /* Option */ ) ;
        "Test the HTTP request handler class.

    This runs an HTTP server on port 8000 (or the port argument).

    ";
        ServerClass . address_family , addr = _get_best_family ( bind , port );
        HandlerClass . protocol_version = protocol;
        // with scope: ServerClass ( addr , HandlerClass ) as httpd  {
        host , port = httpd . socket . getsockname ( ) [ : 2 ];
        url_host = format!("[{host}]" iformat!(":" in host else host);
        println!();
        format!("Serving HTTP on {host} port {port} ");
        format!("(http://{url_host}:{port}/) ...");
        );
        // try {
        httpd . serve_forever ( );
        // } catch  KeyboardInterrupt  {
        println!( "\nKeyboard interrupt received, exiting." );
        sys . exit ( 0 );
        fn main() {
        import argparse;
        import contextlib;
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "--cgi" , action = "store_true" ,;
        help = "run as CGI server" );
        parser . add_argument ( "-b" , "--bind" , metavar = "ADDRESS" ,;
        help = "bind to this address ";
        "(default: all interfaces)" );
        parser . add_argument ( "-d" , "--directory" , default = os . getcwd ( ) ,;
        help = "serve this directory ";
        "(default: current directory)" );
        parser . add_argument ( "-p" , "--protocol" , metavar = "VERSION" ,;
        default = "HTTP/1.0" ,;
        help = "conform to this HTTP version ";
        "(default: %(default)s)" );
        parser . add_argument ( "port" , default = 8000 , type = int , nargs = "?" ,;
        help = "bind to this port ";
        "(default: %(default)s)" );
        args = parser . parse_args ( );
        if args . cgi {
        handler_class = CGIHTTPRequestHandler;
        } else {
        handler_class = SimpleHTTPRequestHandler;
        class DualStackServer ( ThreadingHTTPServer ) ;
        pub fn server_bind ( self )  {
        // with scope: contextlib . suppress ( Exception )  {
        self . socket . setsockopt (;
        socket . IPPROTO_IPV6 , socket . IPV6_V6ONLY , 0 );
        return  super ( ) . server_bind ( );
        pub fn finish_request ( &self, request , client_address )  {
        self . RequestHandlerClass ( request , client_address , self ,;
        directory = args . directory );
        test (;
        HandlerClass = handler_class ,;
        ServerClass = DualStackServer ,;
        port = args . port ,;
        bind = args . bind ,;
        protocol = args . protocol ,;
        );
}

