//! util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::posixpath;
// use crate::urllib::{quote};
// use crate::io::{StringIO, BytesIO};

pub const __all__: f64 = [;
pub struct FileWrapper {
    pub filelike: String, // TODO: infer type
    pub blksize: String, // TODO: infer type
    pub close: String, // TODO: infer type
}

impl FileWrapper {
}

pub fn guess_scheme(environ: &str) {
        "Return a guess for whether 'wsgi.url_scheme' should be 'http' || 'https'
    ";
        if environ . get ( "HTTPS" ) in ( "yes" , "on" , "1" ) {
        return  "https";
        } else {
        return  "http";
        pub fn application_uri ( environ )  {
        "Return the application's base URI (no PATH_INFO || QUERY_STRING)";
        url = environ [ "wsgi.url_scheme" ] + "://";
        from urllib . parse import quote;
        if environ . get ( "HTTP_HOST" ) {
        url + = environ [ "HTTP_HOST" ];
        } else {
        url + = environ [ "SERVER_NAME" ];
        if environ [ "wsgi.url_scheme" ] == "https" {
        if environ [ "SERVER_PORT" ] != "443" {
        url + = ":" + environ [ "SERVER_PORT" ];
        } else {
        if environ [ "SERVER_PORT" ] != "80" {
        url + = ":" + environ [ "SERVER_PORT" ];
        url + = quote ( environ . get ( "SCRIPT_NAME" ) || "/" , encoding = "latin1" );
        return  url;
        pub fn request_uri ( environ , include_query = true )  {
        "Return the full request URI, optionally including the query string";
        url = application_uri ( environ );
        from urllib . parse import quote;
        path_info = quote ( environ . get ( "PATH_INFO" , "" ) , safe = "/;=," , encoding = "latin1" );
        if !environ . get ( "SCRIPT_NAME" ) {
        url + = path_info [ 1 : ];
        } else {
        url + = path_info;
        if include_query && environ . get ( "QUERY_STRING" ) {
        url + = "?" + environ [ "QUERY_STRING" ];
        return  url;
        pub fn shift_path_info ( environ )  {
        "Shift a name from PATH_INFO to SCRIPT_NAME, returning it

    If there are no remaining path segments in PATH_INFO, return None /* Option */.
    Note: 'environ' == modified in-place; use a copy if you need to keep
    the original PATH_INFO || SCRIPT_NAME.

    Note: when PATH_INFO == just a '/', this returns '' && appends a trailing
    '/' to SCRIPT_NAME, even though empty path segments are normally ignored,
    && SCRIPT_NAME doesn't normally end in a '/'.  This == intentional
    behavior, to ensure that an application can tell the difference between
    '/x' && '/x/' when traversing to objects.
    ";
        path_info = environ . get ( "PATH_INFO" , "" );
        if !path_info {
        return;
        path_parts = path_info . split ( "/" );
        path_parts vec![ 1 : -1 ] = vec![ p.iter().map(|p| path_parts vec![ 1 : -1 ] if p && p != "." ).collect();
        name = path_parts [ 1 ];
        del path_parts [ 1 ];
        script_name = environ . get ( "SCRIPT_NAME" , "" );
        script_name = posixpath . normpath ( script_name + "/" + name );
        if script_name . endswith ( "/" ) {
        script_name = script_name [ : -1 ];
        if !name && !script_name . endswith ( "/" ) {
        script_name + = "/";
        environ [ "SCRIPT_NAME" ] = script_name;
        environ [ "PATH_INFO" ] = "/" . join ( path_parts );
        if name == "." {
        name = None /* Option */;
        return  name;
        pub fn setup_testing_defaults ( environ )  {
        "Update 'environ' with trivial defaults for testing purposes

    This adds various parameters required for WSGI, including HTTP_HOST,
    SERVER_NAME, SERVER_PORT, REQUEST_METHOD, SCRIPT_NAME, PATH_INFO,
    && all of the wsgi.* variables.  It only supplies default values,
    && does !replace any existing settings for these variables.

    This routine == intended to make it easier for unit tests of WSGI
    servers && applications to set up dummy environments.  It should *not*
    be used by actual WSGI servers || applications, since the data == fake!
    ";
        environ . setdefault ( "SERVER_NAME" , "127.0.0.1" );
        environ . setdefault ( "SERVER_PROTOCOL" , "HTTP/1.0" );
        environ . setdefault ( "HTTP_HOST" , environ [ "SERVER_NAME" ] );
        environ . setdefault ( "REQUEST_METHOD" , "GET" );
        if "SCRIPT_NAME" !in environ && "PATH_INFO" !in environ {
        environ . setdefault ( "SCRIPT_NAME" , "" );
        environ . setdefault ( "PATH_INFO" , "/" );
        environ . setdefault ( "wsgi.version" , ( 1 , 0 ) );
        environ . setdefault ( "wsgi.run_once" , 0 );
        environ . setdefault ( "wsgi.multithread" , 0 );
        environ . setdefault ( "wsgi.multiprocess" , 0 );
        from io import StringIO , BytesIO;
        environ . setdefault ( "wsgi.input" , BytesIO ( ) );
        environ . setdefault ( "wsgi.errors" , StringIO ( ) );
        environ . setdefault ( "wsgi.url_scheme" , guess_scheme ( environ ) );
        if environ [ "wsgi.url_scheme" ] == "http" {
        environ . setdefault ( "SERVER_PORT" , "80" );
        } else if environ [ "wsgi.url_scheme" ] == "https" {
        environ . setdefault ( "SERVER_PORT" , "443" );
        _hoppish = {;
        "connection" , "keep-alive" , "proxy-authenticate" ,;
        "proxy-authorization" , "te" , "trailers" , "transfer-encoding" ,;
        "upgrade";
        } . __contains__;
        pub fn is_hop_by_hop ( header_name )  {
        "Return true if 'header_name' == an HTTP/1.1 "Hop-by-Hop" header";
        return  _hoppish ( header_name . lower ( ) );
}

