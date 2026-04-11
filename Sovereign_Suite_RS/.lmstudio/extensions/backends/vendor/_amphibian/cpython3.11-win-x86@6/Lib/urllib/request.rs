//! request.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::base64;
// use crate::email;
// use crate::http;
// use std::fs;
// use regex::Regex;
// use crate::string;
// use std::time;
// use crate::contextlib;
// use crate::urllib::{URLError, HTTPError, ContentTooShortError};
// use crate::ssl;
// use crate::warnings;
// use crate::ftplib;
// use crate::nturl2path::{url2pathname, pathname2url};
// use crate::mimetypes;
// use crate::getpass;
// use crate::fnmatch::{fnmatch};
// use crate::ipaddress::{AddressValueError, IPv4Address};
// use crate::_scproxy::{_get_proxy_settings, _get_proxies};
// use crate::winreg;

pub const __all__: f64 = [;
pub const __version__: &str = "%d.%d" % sys . version_info [ : 2 ];
pub const _opener: f64 = None;
pub fn urlopen(url: &str, data: &str, timeout: &str, socket: &str, _GLOBAL_DEFAULT_TIMEOUT: &str, cafile: &str, capath: &str, cadefault: &str, context: &str) {
        // pass
}

pub fn install_opener(opener: &str) {
        global _opener;
        _opener = opener;
        _url_tempfiles = [ ];
        pub fn urlretrieve ( url , filename = None /* Option */ , reporthook = None /* Option */ , data = None /* Option */ )  {
        "
    Retrieve a URL into a temporary location on disk.

    Requires a URL argument. If a filename == passed, it == used as
    the temporary file location. The reporthook argument should be
    a callable that accepts a block number, a read size, && the
    total file size of the URL target. The data argument should be
    valid URL encoded data.

    If a filename == passed && the URL points to a local resource,
    the result == a copy from local file to new file.

    Returns a tuple containing the path to the newly created
    data file as well as the resulting HTTPMessage object.
    ";
        url_type , path = _splittype ( url );
        // with scope: contextlib . closing ( urlopen ( url , data ) ) as fp  {
        headers = fp . info ( );
        if url_type == "file" && !filename {
        return  os . path . normpath ( path ) , headers;
        if filename {
        tfp = open ( filename , "wb" );
        } else {
        tfp = tempfile . NamedTemporaryFile ( delete = false );
        filename = tfp . name;
        _url_tempfiles . append ( filename );
        // with scope: tfp  {
        result = filename , headers;
        bs = 1024 * 8;
        size = -1;
        read = 0;
        blocknum = 0;
        if "content-length" in headers {
        size = int ( headers [ "Content-Length" ] );
        if reporthook {
        reporthook ( blocknum , bs , size );
        while true  {
        block = fp . read ( bs );
        if !block {
        break;
        read + = len ( block );
        tfp . write ( block );
        blocknum + = 1;
        if reporthook {
        reporthook ( blocknum , bs , size );
        if size >= 0 && read < size {
        panic!("ContentTooShortError (");
        "retrieval incomplete: got only %i out of %i bytes";
        % ( read , size ) , result );
        return  result;
        pub fn urlcleanup ( )  {
        "Clean up temporary files from urlretrieve calls.";
        for temp_file in _url_tempfiles .iter() {
        // try {
        os . unlink ( temp_file );
        // } catch  OSError  {
        // pass
        del _url_tempfiles [ : ];
        global _opener;
        if _opener {
        _opener = None /* Option */;
        _cut_port_re = re . compile ( r ":\d+$" , re . ASCII );
        pub fn request_host ( request )  {
        "Return request-host, as defined by RFC 2965.

    Variation from RFC: returned value == lowercased, for convenient
    comparison.

    ";
        url = request . full_url;
        host = urlparse ( url ) [ 1 ];
        if host == "" {
        host = request . get_header ( "Host" , "" );
        host = _cut_port_re . sub ( "" , host , 1 );
        return  host . lower ( );
        class Request ;
        pub fn __init__ ( &self, url , data = None /* Option */ , headers = { } , {
        origin_req_host = None /* Option */ , unverifiable = false ,;
        method = None /* Option */ ) ;
        self . full_url = url;
        self . headers = { };
        self . unredirected_hdrs = { };
        self . _data = None /* Option */;
        self . data = data;
        self . _tunnel_host = None /* Option */;
        for key , value in headers . items ( ) .iter() {
        self . add_header ( key , value );
        if origin_req_host is None /* Option */ {
        origin_req_host = request_host ( self );
        self . origin_req_host = origin_req_host;
        self . unverifiable = unverifiable;
        if method {
        self . method = method;
        @ property;
        pub fn full_url ( self )  {
        if self . fragment {
        return  "{}#{}" . format ( self . _full_url , self . fragment );
        return  self . _full_url;
        @ full_url . setter;
        pub fn full_url ( &self, url )  {
        self . _full_url = unwrap ( url );
        self . _full_url , self . fragment = _splittag ( self . _full_url );
        self . _parse ( );
        @ full_url . deleter;
        pub fn full_url ( self )  {
        self . _full_url = None /* Option */;
        self . fragment = None /* Option */;
        self . selector = "";
        @ property;
        pub fn data ( self )  {
        return  self . _data;
        @ data . setter;
        pub fn data ( &self, data )  {
        if data != self . _data {
        self . _data = data;
        if self . has_header ( "Content-length" ) {
        self . remove_header ( "Content-length" );
        @ data . deleter;
        pub fn data ( self )  {
        self . data = None /* Option */;
        pub fn _parse ( self )  {
        self . type , rest = _splittype ( self . _full_url );
        if self . type is None /* Option */ {
        panic!("ValueError ( "unknown url type: %r" % self . full_url )");
        self . host , self . selector = _splithost ( rest );
        if self . host {
        self . host = unquote ( self . host );
        pub fn get_method ( self )  {
        "Return a string indicating the HTTP request method.";
        default_method = "POST" if self . data == !None /* Option */ else "GET";
        return  getattr ( self , "method" , default_method );
        pub fn get_full_url ( self )  {
        return  self . full_url;
        pub fn set_proxy ( &self, host , type )  {
        if self . type == "https" && !self . _tunnel_host {
        self . _tunnel_host = self . host;
        } else {
        self . type = type;
        self . selector = self . full_url;
        self . host = host;
        pub fn has_proxy ( self )  {
        return  self . selector == self . full_url;
        pub fn add_header ( &self, key , val )  {
        self . headers [ key . capitalize ( ) ] = val;
        pub fn add_unredirected_header ( &self, key , val )  {
        self . unredirected_hdrs [ key . capitalize ( ) ] = val;
        pub fn has_header ( &self, header_name )  {
        return  ( header_name in self . headers or;
        header_name in self . unredirected_hdrs );
        pub fn get_header ( &self, header_name , default = None /* Option */ )  {
        return  self . headers . get (;
        header_name ,;
        self . unredirected_hdrs . get ( header_name , default ) );
        pub fn remove_header ( &self, header_name )  {
        self . headers . pop ( header_name , None /* Option */ );
        self . unredirected_hdrs . pop ( header_name , None /* Option */ );
        pub fn header_items ( self )  {
        hdrs = { ** self . unredirected_hdrs , ** self . headers };
        return  list ( hdrs . items ( ) );
        class OpenerDirector ;
        pub fn __init__ ( self )  {
        client_version = "Python-urllib/%s" % __version__;
        self . addheaders = [ ( "User-agent" , client_version ) ];
        self . handlers = [ ];
        self . handle_open = { };
        self . handle_error = { };
        self . process_response = { };
        self . process_request = { };
        pub fn add_handler ( &self, handler )  {
        if !hasattr ( handler , "add_parent" ) {
        panic!("TypeError ( "expected BaseHandler instance, got %r" %");
        type ( handler ) );
        added = false;
        for meth in dir ( handler ) .iter() {
        if meth in [ "redirect_request" , "do_open" , "proxy_open" ] {
        continue;
        i = meth . find ( "_" );
        protocol = meth [ : i ];
        condition = meth [ i + 1 : ];
        if condition . startswith ( "error" ) {
        j = condition . find ( "_" ) + i + 1;
        kind = meth [ j + 1 : ];
        // try {
        kind = int ( kind );
        // } catch  ValueError  {
        // pass
        lookup = self . handle_error . get ( protocol , { } );
        self . handle_error [ protocol ] = lookup;
        } else if condition == "open" {
        kind = protocol;
        lookup = self . handle_open;
        } else if condition == "response" {
        kind = protocol;
        lookup = self . process_response;
        } else if condition == "request" {
        kind = protocol;
        lookup = self . process_request;
        } else {
        continue;
        handlers = lookup . setdefault ( kind , [ ] );
        if handlers {
        bisect . insort ( handlers , handler );
        } else {
        handlers . append ( handler );
        added = true;
        if added {
        bisect . insort ( self . handlers , handler );
        handler . add_parent ( self );
        pub fn close ( self )  {
        // pass
        pub fn _call_chain ( &self, chain , kind , meth_name , * args )  {
        handlers = chain . get ( kind , ( ) );
        for handler in handlers .iter() {
        func = getattr ( handler , meth_name );
        result = func ( * args );
        if result is !None /* Option */ {
        return  result;
        pub fn open ( &self, fullurl , data = None /* Option */ , timeout = socket . _GLOBAL_DEFAULT_TIMEOUT )  {
        if isinstance ( fullurl , str ) {
        req = Request ( fullurl , data );
        } else {
        req = fullurl;
        if data is !None /* Option */ {
        req . data = data;
        req . timeout = timeout;
        protocol = req . type;
        meth_name = protocol + "_request";
        for processor in self . process_request . get ( protocol , [ ] ) .iter() {
        meth = getattr ( processor , meth_name );
        req = meth ( req );
        sys . audit ( "urllib.Request" , req . full_url , req . data , req . headers , req . get_method ( ) );
        response = self . _open ( req , data );
        meth_name = protocol + "_response";
        for processor in self . process_response . get ( protocol , [ ] ) .iter() {
        meth = getattr ( processor , meth_name );
        response = meth ( req , response );
        return  response;
        pub fn _open ( &self, req , data = None /* Option */ )  {
        result = self . _call_chain ( self . handle_open , "default" ,;
        "default_open" , req );
        if result {
        return  result;
        protocol = req . type;
        result = self . _call_chain ( self . handle_open , protocol , protocol +;
        "_open" , req );
        if result {
        return  result;
        return  self . _call_chain ( self . handle_open , "unknown" ,;
        "unknown_open" , req );
        pub fn error ( &self, proto , * args )  {
        if proto in ( "http" , "https" ) {
        dict = self . handle_error [ "http" ];
        proto = args [ 2 ];
        meth_name = "http_error_%s" % proto;
        http_err = 1;
        orig_args = args;
        } else {
        dict = self . handle_error;
        meth_name = proto + "_error";
        http_err = 0;
        args = ( dict , proto , meth_name ) + args;
        result = self . _call_chain ( * args );
        if result {
        return  result;
        if http_err {
        args = ( dict , "default" , "http_error_default" ) + orig_args;
        return  self . _call_chain ( * args );
        pub fn build_opener ( * handlers )  {
        "Create an opener object from a list of handlers.

    The opener will use several default handlers, including support
    for HTTP, FTP && when applicable HTTPS.

    If any of the handlers passed as arguments are subclasses of the
    default handlers, the default handlers will !be used.
    ";
        opener = OpenerDirector ( );
        default_classes = [ ProxyHandler , UnknownHandler , HTTPHandler ,;
        HTTPDefaultErrorHandler , HTTPRedirectHandler ,;
        FTPHandler , FileHandler , HTTPErrorProcessor ,;
        DataHandler ];
        if hasattr ( http . client , "HTTPSConnection" ) {
        default_classes . append ( HTTPSHandler );
        skip = set ( );
        for klass in default_classes .iter() {
        for check in handlers .iter() {
        if isinstance ( check , type ) {
        if issubclass ( check , klass ) {
        skip . add ( klass );
        } else if isinstance ( check , klass ) {
        skip . add ( klass );
        for klass in skip .iter() {
        default_classes . remove ( klass );
        for klass in default_classes .iter() {
        opener . add_handler ( klass ( ) );
        for h in handlers .iter() {
        if isinstance ( h , type ) {
        h = h ( );
        opener . add_handler ( h );
        return  opener;
        class BaseHandler ;
        handler_order = 500;
        pub fn add_parent ( &self, parent )  {
        self . parent = parent;
        pub fn close ( self )  {
        // pass
        pub fn __lt__ ( &self, other )  {
        if !hasattr ( other , "handler_order" ) {
        return  true;
        return  self . handler_order < other . handler_order;
        class HTTPErrorProcessor ( BaseHandler ) ;
        "Process HTTP error responses.";
        handler_order = 1000;
        pub fn http_response ( &self, request , response )  {
        code , msg , hdrs = response . code , response . msg , response . info ( );
        if !( 200 <= code < 300 ) {
        response = self . parent . error (;
        "http" , request , response , code , msg , hdrs );
        return  response;
        https_response = http_response;
        class HTTPDefaultErrorHandler ( BaseHandler ) ;
        pub fn http_error_default ( &self, req , fp , code , msg , hdrs )  {
        panic!("HTTPError ( req . full_url , code , msg , hdrs , fp )");
        class HTTPRedirectHandler ( BaseHandler ) ;
        max_repeats = 4;
        max_redirections = 10;
        pub fn redirect_request ( &self, req , fp , code , msg , headers , newurl )  {
        "Return a Request || None /* Option */ in response to a redirect.

        This == called by the http_error_30x methods when a
        redirection response == received.  If a redirection should
        take place, return a new Request to allow http_error_30x to
        perform the redirect.  Otherwise, raise HTTPError if no-one
        else should try to handle this url.  Return None /* Option */ if you can't
        but another Handler might.
        ";
        m = req . get_method ( );
        if ( !( code in ( 301 , 302 , 303 , 307 , 308 ) && m in ( "GET" , "HEAD" ) {
        or code in ( 301 , 302 , 303 ) && m == "POST" ) ) ;
        panic!("HTTPError ( req . full_url , code , msg , headers , fp )");
        newurl = newurl . replace ( " " , "%20" );
        CONTENT_HEADERS = ( "content-length" , "content-type" );
        newheaders = { k : v for k , v in req . headers . items ( );
        if k . lower ( ) !in CONTENT_HEADERS } {
        return  Request ( newurl ,;
        headers = newheaders ,;
        origin_req_host = req . origin_req_host ,;
        unverifiable = true );
        pub fn http_error_302 ( &self, req , fp , code , msg , headers )  {
        if "location" in headers {
        newurl = headers [ "location" ];
        } else if "uri" in headers {
        newurl = headers [ "uri" ];
        } else {
        return;
        urlparts = urlparse ( newurl );
        if urlparts . scheme !in ( "http" , "https" , "ftp" , "" ) {
        panic!("HTTPError (");
        newurl , code ,;
        "%s - Redirection to url '%s' == !allowed" % ( msg , newurl ) ,;
        headers , fp );
        if !urlparts . path && urlparts . netloc {
        urlparts = list ( urlparts );
        urlparts [ 2 ] = "/";
        newurl = urlunparse ( urlparts );
        newurl = quote (;
        newurl , encoding = "iso-8859-1" , safe = string . punctuation );
        newurl = urljoin ( req . full_url , newurl );
        new = self . redirect_request ( req , fp , code , msg , headers , newurl );
        if new is None /* Option */ {
        return;
        if hasattr ( req , "redirect_dict" ) {
        visited = new . redirect_dict = req . redirect_dict;
        if ( visited . get ( newurl , 0 ) >= self . max_repeats or {
        len ( visited ) >= self . max_redirections ) ;
        panic!("HTTPError ( req . full_url , code ,");
        self . inf_msg + msg , headers , fp );
        } else {
        visited = new . redirect_dict = req . redirect_dict = { };
        visited [ newurl ] = visited . get ( newurl , 0 ) + 1;
        fp . read ( );
        fp . close ( );
        return  self . parent . open ( new , timeout = req . timeout );
        http_error_301 = http_error_303 = http_error_307 = http_error_308 = http_error_302;
        inf_msg = "The HTTP server returned a redirect error that would " \;
        "lead to an infinite loop.\n" \;
        "The last 30x error message was:\n";
        pub fn _parse_proxy ( proxy )  {
        "Return (scheme, user, password, host/port) given a URL || an authority.

    If a URL == supplied, it must have an authority (host:port) component.
    According to RFC 3986, having an authority component means the URL must
    have two slashes after the scheme.
    ";
        scheme , r_scheme = _splittype ( proxy );
        if !r_scheme . startswith ( "/" ) {
        scheme = None /* Option */;
        authority = proxy;
        } else {
        if !r_scheme . startswith ( "//" ) {
        panic!("ValueError ( "proxy URL with no authority: %r" % proxy )");
        if "@" in r_scheme {
        host_separator = r_scheme . find ( "@" );
        end = r_scheme . find ( "/" , host_separator );
        } else {
        end = r_scheme . find ( "/" , 2 );
        if end == -1 {
        end = None /* Option */;
        authority = r_scheme [ 2 : end ];
        userinfo , hostport = _splituser ( authority );
        if userinfo is !None /* Option */ {
        user , password = _splitpasswd ( userinfo );
        } else {
        user = password = None /* Option */;
        return  scheme , user , password , hostport;
        class ProxyHandler ( BaseHandler ) ;
        handler_order = 100;
        pub fn __init__ ( &self, proxies = None /* Option */ )  {
        if proxies is None /* Option */ {
        proxies = getproxies ( );
        assert hasattr ( proxies , "keys" ) , "proxies must be a mapping";
        self . proxies = proxies;
        for type , url in proxies . items ( ) .iter() {
        type = type . lower ( );
        setattr ( self , "%s_open" % type ,;
        |r , proxy = url , type = type , meth = self . proxy_open | {  };
        meth ( r , proxy , type ) );
        pub fn proxy_open ( &self, req , proxy , type )  {
        orig_type = req . type;
        proxy_type , user , password , hostport = _parse_proxy ( proxy );
        if proxy_type is None /* Option */ {
        proxy_type = orig_type;
        if req . host && proxy_bypass ( req . host ) {
        return;
        if user && password {
        user_pass = "%s:%s" % ( unquote ( user ) ,;
        unquote ( password ) );
        creds = base64 . b64encode ( user_pass . encode ( ) ) . decode ( "ascii" );
        req . add_header ( "Proxy-authorization" , "Basic " + creds );
        hostport = unquote ( hostport );
        req . set_proxy ( hostport , proxy_type );
        if orig_type == proxy_type || orig_type == "https" {
        return;
        } else {
        return  self . parent . open ( req , timeout = req . timeout );
        class HTTPPasswordMgr ;
        pub fn __init__ ( self )  {
        self . passwd = { };
        pub fn add_password ( &self, realm , uri , user , passwd )  {
        if isinstance ( uri , str ) {
        uri = [ uri ];
        if realm !in self . passwd {
        self . passwd [ realm ] = { };
        for default_port in true , false .iter() {
        reduced_uri = tuple (;
        self . reduce_uri ( u , default_port ) for u in uri );
        self . passwd [ realm ] [ reduced_uri ] = ( user , passwd );
        pub fn find_user_password ( &self, realm , authuri )  {
        domains = self . passwd . get ( realm , { } );
        for default_port in true , false .iter() {
        reduced_authuri = self . reduce_uri ( authuri , default_port );
        for uris , authinfo in domains . items ( ) .iter() {
        for uri in uris .iter() {
        if self . is_suburi ( uri , reduced_authuri ) {
        return  authinfo;
        return  None /* Option */ , None /* Option */;
        pub fn reduce_uri ( &self, uri , default_port = true )  {
        "Accept authority || URI && extract only the authority && path.";
        parts = urlsplit ( uri );
        if parts [ 1 ] {
        scheme = parts [ 0 ];
        authority = parts [ 1 ];
        path = parts [ 2 ] || "/";
        } else {
        scheme = None /* Option */;
        authority = uri;
        path = "/";
        host , port = _splitport ( authority );
        if default_port && port is None /* Option */ && scheme is !None /* Option */ {
        dport = { "http" : 80 ,;
        "https" : 443 ,;
        } . get ( scheme );
        if dport is !None /* Option */ {
        authority = "%s:%d" % ( host , dport );
        return  authority , path;
        pub fn is_suburi ( &self, base , test )  {
        "Check if test == below base in a URI tree

        Both args must be URIs in reduced form.
        ";
        if base == test {
        return  true;
        if base [ 0 ] != test [ 0 ] {
        return  false;
        prefix = base [ 1 ];
        if prefix [ -1 { : ] != "/" ; }
        prefix + = "/";
        return  test [ 1 ] . startswith ( prefix );
        class HTTPPasswordMgrWithDefaultRealm ( HTTPPasswordMgr ) ;
        pub fn find_user_password ( &self, realm , authuri )  {
        user , password = HTTPPasswordMgr . find_user_password ( self , realm ,;
        authuri );
        if user is !None /* Option */ {
        return  user , password;
        return  HTTPPasswordMgr . find_user_password ( self , None /* Option */ , authuri );
        class HTTPPasswordMgrWithPriorAuth ( HTTPPasswordMgrWithDefaultRealm ) ;
        pub fn __init__ ( &self, * args , ** kwargs )  {
        self . authenticated = { };
        super ( ) . __init__ ( * args , ** kwargs );
        pub fn add_password ( &self, realm , uri , user , passwd , is_authenticated = false )  {
        self . update_authenticated ( uri , is_authenticated );
        if realm is !None /* Option */ {
        super ( ) . add_password ( None /* Option */ , uri , user , passwd );
        super ( ) . add_password ( realm , uri , user , passwd );
        pub fn update_authenticated ( &self, uri , is_authenticated = false )  {
        if isinstance ( uri , str ) {
        uri = [ uri ];
        for default_port in true , false .iter() {
        for u in uri .iter() {
        reduced_uri = self . reduce_uri ( u , default_port );
        self . authenticated [ reduced_uri ] = is_authenticated;
        pub fn is_authenticated ( &self, authuri )  {
        for default_port in true , false .iter() {
        reduced_authuri = self . reduce_uri ( authuri , default_port );
        for uri in self . authenticated .iter() {
        if self . is_suburi ( uri , reduced_authuri ) {
        return  self . authenticated [ uri ];
        class AbstractBasicAuthHandler ;
        rx = re . compile ( "(?:^|,)";
        "[ \t]*";
        "([^ \t,]+)";
        "[ \t]+";
        "realm=(["\']?)([^"\']*)\\2" ,;
        re . I );
        pub fn __init__ ( &self, password_mgr = None /* Option */ )  {
        if password_mgr is None /* Option */ {
        password_mgr = HTTPPasswordMgr ( );
        self . passwd = password_mgr;
        self . add_password = self . passwd . add_password;
        pub fn _parse_realm ( &self, header )  {
        found_challenge = false;
        for mo in AbstractBasicAuthHandler . rx . finditer ( header ) .iter() {
        scheme , quote , realm = mo . groups ( );
        if quote !in [ """ , "'" ] {
        warnings . warn ( "Basic Auth Realm was unquoted" ,;
        UserWarning , 3 );
        yield ( scheme , realm );
        found_challenge = true;
        if !found_challenge {
        if header {
        scheme = header . split ( ) [ 0 ];
        } else {
        scheme = "";
        yield ( scheme , None /* Option */ );
        pub fn http_error_auth_reqed ( &self, authreq , host , req , headers )  {
        headers = headers . get_all ( authreq );
        if !headers {
        return;
        unsupported = None /* Option */;
        for header in headers .iter() {
        for scheme , realm in self . _parse_realm ( header ) .iter() {
        if scheme . lower ( ) != "basic" {
        unsupported = scheme;
        continue;
        if realm is !None /* Option */ {
        return  self . retry_http_basic_auth ( host , req , realm );
        if unsupported is !None /* Option */ {
        panic!("ValueError ( "AbstractBasicAuthHandler does !"");
        "support the following scheme: %r";
        % ( scheme , ) );
        pub fn retry_http_basic_auth ( &self, host , req , realm )  {
        user , pw = self . passwd . find_user_password ( realm , host );
        if pw is !None /* Option */ {
        raw = "%s:%s" % ( user , pw );
        auth = "Basic " + base64 . b64encode ( raw . encode ( ) ) . decode ( "ascii" );
        if req . get_header ( self . auth_header , None /* Option */ ) == auth {
        return;
        req . add_unredirected_header ( self . auth_header , auth );
        return  self . parent . open ( req , timeout = req . timeout );
        } else {
        return;
        pub fn http_request ( &self, req )  {
        if ( !hasattr ( self . passwd , "is_authenticated" ) or {
        not self . passwd . is_authenticated ( req . full_url ) ) ;
        return  req;
        if !req . has_header ( "Authorization" ) {
        user , passwd = self . passwd . find_user_password ( None /* Option */ , req . full_url );
        credentials = "{0}:{1}" . format ( user , passwd ) . encode ( );
        auth_str = base64 . standard_b64encode ( credentials ) . decode ( );
        req . add_unredirected_header ( "Authorization" ,;
        "Basic {}" . format ( auth_str . strip ( ) ) );
        return  req;
        pub fn http_response ( &self, req , response )  {
        if hasattr ( self . passwd , "is_authenticated" ) {
        if 200 <= response . code < 300 {
        self . passwd . update_authenticated ( req . full_url , true );
        } else {
        self . passwd . update_authenticated ( req . full_url , false );
        return  response;
        https_request = http_request;
        https_response = http_response;
        class HTTPBasicAuthHandler ( AbstractBasicAuthHandler , BaseHandler ) ;
        auth_header = "Authorization";
        pub fn http_error_401 ( &self, req , fp , code , msg , headers )  {
        url = req . full_url;
        response = self . http_error_auth_reqed ( "www-authenticate" ,;
        url , req , headers );
        return  response;
        class ProxyBasicAuthHandler ( AbstractBasicAuthHandler , BaseHandler ) ;
        auth_header = "Proxy-authorization";
        pub fn http_error_407 ( &self, req , fp , code , msg , headers )  {
        authority = req . host;
        response = self . http_error_auth_reqed ( "proxy-authenticate" ,;
        authority , req , headers );
        return  response;
        _randombytes = os . urandom;
        class AbstractDigestAuthHandler ;
        pub fn __init__ ( &self, passwd = None /* Option */ )  {
        if passwd is None /* Option */ {
        passwd = HTTPPasswordMgr ( );
        self . passwd = passwd;
        self . add_password = self . passwd . add_password;
        self . retried = 0;
        self . nonce_count = 0;
        self . last_nonce = None /* Option */;
        pub fn reset_retry_count ( self )  {
        self . retried = 0;
        pub fn http_error_auth_reqed ( &self, auth_header , host , req , headers )  {
        authreq = headers . get ( auth_header , None /* Option */ );
        if self . retried > 5 {
        panic!("HTTPError ( req . full_url , 401 , "digest auth failed" ,");
        headers , None /* Option */ );
        } else {
        self . retried + = 1;
        if authreq {
        scheme = authreq . split ( ) [ 0 ];
        if scheme . lower ( ) == "digest" {
        return  self . retry_http_digest_auth ( req , authreq );
        } else if scheme . lower ( ) != "basic" {
        panic!("ValueError ( "AbstractDigestAuthHandler does !support"");
        " the following scheme: '%s'" % scheme );
        pub fn retry_http_digest_auth ( &self, req , auth )  {
        token , challenge = auth . split ( " " , 1 );
        chal = parse_keqv_list ( filter ( None /* Option */ , parse_http_list ( challenge ) ) );
        auth = self . get_authorization ( req , chal );
        if auth {
        auth_val = "Digest %s" % auth;
        if req . headers . get ( self . auth_header , None /* Option */ ) == auth_val {
        return;
        req . add_unredirected_header ( self . auth_header , auth_val );
        resp = self . parent . open ( req , timeout = req . timeout );
        return  resp;
        pub fn get_cnonce ( &self, nonce )  {
        s = "%s:%s:%s:" % ( self . nonce_count , nonce , time . ctime ( ) );
        b = s . encode ( "ascii" ) + _randombytes ( 8 );
        dig = hashlib . sha1 ( b ) . hexdigest ( );
        return  dig [ : 16 ];
        pub fn get_authorization ( &self, req , chal )  {
        // try {
        realm = chal [ "realm" ];
        nonce = chal [ "nonce" ];
        qop = chal . get ( "qop" );
        algorithm = chal . get ( "algorithm" , "MD5" );
        opaque = chal . get ( "opaque" , None /* Option */ );
        // } catch  KeyError  {
        return;
        H , KD = self . get_algorithm_impls ( algorithm );
        if H is None /* Option */ {
        return;
        user , pw = self . passwd . find_user_password ( realm , req . full_url );
        if user is None /* Option */ {
        return;
        if req . data is !None /* Option */ {
        entdig = self . get_entity_digest ( req . data , chal );
        } else {
        entdig = None /* Option */;
        A1 = "%s:%s:%s" % ( user , realm , pw );
        A2 = "%s:%s" % ( req . get_method ( ) ,;
        req . selector );
        if qop is None /* Option */ {
        respdig = KD ( H ( A1 ) , "%s:%s" % ( nonce , H ( A2 ) ) );
        } else if "auth" in qop . split ( "," ) {
        if nonce == self . last_nonce {
        self . nonce_count + = 1;
        } else {
        self . nonce_count = 1;
        self . last_nonce = nonce;
        ncvalue = "%08x" % self . nonce_count;
        cnonce = self . get_cnonce ( nonce );
        noncebit = "%s:%s:%s:%s:%s" % ( nonce , ncvalue , cnonce , "auth" , H ( A2 ) );
        respdig = KD ( H ( A1 ) , noncebit );
        } else {
        panic!("URLError ( "qop '%s' is !supported." % qop )");
        base = "username="%s", realm="%s", nonce="%s", uri="%s", " \;
        "response="%s"" % ( user , realm , nonce , req . selector ,;
        respdig );
        if opaque {
        base + = ", opaque="%s"" % opaque;
        if entdig {
        base + = ", digest="%s"" % entdig;
        base + = ", algorithm="%s"" % algorithm;
        if qop {
        base + = ", qop=auth, nc=%s, cnonce="%s"" % ( ncvalue , cnonce );
        return  base;
        pub fn get_algorithm_impls ( &self, algorithm )  {
        if algorithm == "MD5" {
        H = |x | {  hashlib . md5 ( x . encode ( "ascii" ) ) . hexdigest ( ) };
        } else if algorithm == "SHA" {
        H = |x | {  hashlib . sha1 ( x . encode ( "ascii" ) ) . hexdigest ( ) };
        } else {
        panic!("ValueError ( "Unsupported digest authentication "");
        "algorithm %r" % algorithm );
        KD = |s , d | {  H ( "%s:%s" % ( s , d ) ) };
        return  H , KD;
        pub fn get_entity_digest ( &self, data , chal )  {
        return;
        class HTTPDigestAuthHandler ( BaseHandler , AbstractDigestAuthHandler ) ;
        "An authentication protocol defined by RFC 2069

    Digest authentication improves on basic authentication because it
    does !transmit passwords in the clear.
    ";
        auth_header = "Authorization";
        handler_order = 490;
        pub fn http_error_401 ( &self, req , fp , code , msg , headers )  {
        host = urlparse ( req . full_url ) [ 1 ];
        retry = self . http_error_auth_reqed ( "www-authenticate" ,;
        host , req , headers );
        self . reset_retry_count ( );
        return  retry;
        class ProxyDigestAuthHandler ( BaseHandler , AbstractDigestAuthHandler ) ;
        auth_header = "Proxy-Authorization";
        handler_order = 490;
        pub fn http_error_407 ( &self, req , fp , code , msg , headers )  {
        host = req . host;
        retry = self . http_error_auth_reqed ( "proxy-authenticate" ,;
        host , req , headers );
        self . reset_retry_count ( );
        return  retry;
        class AbstractHTTPHandler ( BaseHandler ) ;
        pub fn __init__ ( &self, debuglevel = 0 )  {
        self . _debuglevel = debuglevel;
        pub fn set_http_debuglevel ( &self, level )  {
        self . _debuglevel = level;
        pub fn _get_content_length ( &self, request )  {
        return  http . client . HTTPConnection . _get_content_length (;
        request . data ,;
        request . get_method ( ) );
        pub fn do_request_ ( &self, request )  {
        host = request . host;
        if !host {
        panic!("URLError ( "no host given" )");
        if request . data is !None /* Option */ {
        data = request . data;
        if isinstance ( data , str ) {
        msg = "POST data should be bytes, an iterable of bytes, " \;
        "or a file object. It cannot be of type str.";
        panic!("TypeError ( msg )");
        if !request . has_header ( "Content-type" ) {
        request . add_unredirected_header (;
        "Content-type" ,;
        "application/x-www-form-urlencoded" );
        if ( !request . has_header ( "Content-length" ) {
        and !request . has_header ( "Transfer-encoding" ) ) ;
        content_length = self . _get_content_length ( request );
        if content_length is !None /* Option */ {
        request . add_unredirected_header (;
        "Content-length" , str ( content_length ) );
        } else {
        request . add_unredirected_header (;
        "Transfer-encoding" , "chunked" );
        sel_host = host;
        if request . has_proxy ( ) {
        scheme , sel = _splittype ( request . selector );
        sel_host , sel_path = _splithost ( sel );
        if !request . has_header ( "Host" ) {
        request . add_unredirected_header ( "Host" , sel_host );
        for name , value in self . parent . addheaders .iter() {
        name = name . capitalize ( );
        if !request . has_header ( name ) {
        request . add_unredirected_header ( name , value );
        return  request;
        pub fn do_open ( &self, http_class , req , ** http_conn_args )  {
        "Return an HTTPResponse object for the request, using http_class.

        http_class must implement the HTTPConnection API from http.client.
        ";
        host = req . host;
        if !host {
        panic!("URLError ( "no host given" )");
        h = http_class ( host , timeout = req . timeout , ** http_conn_args );
        h . set_debuglevel ( self . _debuglevel );
        headers = dict ( req . unredirected_hdrs );
        headers . update ( { k : v for k , v in req . headers . items ( );
        if k !in headers } ) {
        headers [ "Connection" ] = "close";
        headers = { name . title ( ) : val for name , val in headers . items ( ) };
        if req . _tunnel_host {
        tunnel_headers = { };
        proxy_auth_hdr = "Proxy-Authorization";
        if proxy_auth_hdr in headers {
        tunnel_headers [ proxy_auth_hdr ] = headers [ proxy_auth_hdr ];
        del headers [ proxy_auth_hdr ];
        h . set_tunnel ( req . _tunnel_host , headers = tunnel_headers );
        // try {
        // try {
        h . request ( req . get_method ( ) , req . selector , req . data , headers ,;
        encode_chunked = req . has_header ( "Transfer-encoding" ) );
        // } catch  OSError as err  {
        panic!("URLError ( err )");
        r = h . getresponse ( );
        // } catch   {
        h . close ( );
        panic!("");
        if h . sock {
        h . sock . close ( );
        h . sock = None /* Option */;
        r . url = req . get_full_url ( );
        r . msg = r . reason;
        return  r;
        class HTTPHandler ( AbstractHTTPHandler ) ;
        pub fn http_open ( &self, req )  {
        return  self . do_open ( http . client . HTTPConnection , req );
        http_request = AbstractHTTPHandler . do_request_;
        if hasattr ( http . client , "HTTPSConnection" ) {
        class HTTPSHandler ( AbstractHTTPHandler ) ;
        pub fn __init__ ( &self, debuglevel = 0 , context = None /* Option */ , check_hostname = None /* Option */ )  {
        AbstractHTTPHandler . __init__ ( self , debuglevel );
        self . _context = context;
        self . _check_hostname = check_hostname;
        pub fn https_open ( &self, req )  {
        return  self . do_open ( http . client . HTTPSConnection , req ,;
        context = self . _context , check_hostname = self . _check_hostname );
        https_request = AbstractHTTPHandler . do_request_;
        __all__ . append ( "HTTPSHandler" );
        class HTTPCookieProcessor ( BaseHandler ) ;
        pub fn __init__ ( &self, cookiejar = None /* Option */ )  {
        import http . cookiejar;
        if cookiejar is None /* Option */ {
        cookiejar = http . cookiejar . CookieJar ( );
        self . cookiejar = cookiejar;
        pub fn http_request ( &self, request )  {
        self . cookiejar . add_cookie_header ( request );
        return  request;
        pub fn http_response ( &self, request , response )  {
        self . cookiejar . extract_cookies ( response , request );
        return  response;
        https_request = http_request;
        https_response = http_response;
        class UnknownHandler ( BaseHandler ) ;
        pub fn unknown_open ( &self, req )  {
        type = req . type;
        panic!("URLError ( "unknown url type: %s" % type )");
        pub fn parse_keqv_list ( l )  {
        "Parse list of key=value strings where keys are !duplicated.";
        parsed = { };
        for elt in l .iter() {
        k , v = elt . split ( "=" , 1 );
        if v [ 0 ] == """ && v [ -1 ] == """ {
        v = v [ 1 : -1 ];
        parsed [ k ] = v;
        return  parsed;
        pub fn parse_http_list ( s )  {
        "Parse lists as described by RFC 2068 Section 2.

    In particular, parse comma-separated lists where the elements of
    the list may include quoted-strings.  A quoted-string could
    contain a comma.  A non-quoted string could have quotes in the
    middle.  Neither commas nor quotes count if they are escaped.
    Only double-quotes count, !single-quotes.
    ";
        res = [ ];
        part = "";
        escape = quote = false;
        for cur in s .iter() {
        if escape {
        part + = cur;
        escape = false;
        continue;
        if quote {
        if cur == "\\" {
        escape = true;
        continue;
        } else if cur == """ {
        quote = false;
        part + = cur;
        continue;
        if cur == "," {
        res . append ( part );
        part = "";
        continue;
        if cur == """ {
        quote = true;
        part + = cur;
        if part {
        res . append ( part );
        return  [ part . strip ( ) for part in res ];
        class FileHandler ( BaseHandler ) ;
        pub fn file_open ( &self, req )  {
        url = req . selector;
        if url [ { : 2 ] == "//" && url [ 2 : 3 ] != "/" && ( req . host and; }
        req . host != "localhost" ) ;
        if !req . host in self . get_names ( ) {
        panic!("URLError ( "file:// scheme is supported only on localhost" )");
        } else {
        return  self . open_local_file ( req );
        names = None /* Option */;
        pub fn get_names ( self )  {
        if FileHandler . names is None /* Option */ {
        // try {
        FileHandler . names = tuple (;
        socket . gethostbyname_ex ( "localhost" ) [ 2 ] +;
        socket . gethostbyname_ex ( socket . gethostname ( ) ) [ 2 ] );
        // } catch  socket . gaierror  {
        FileHandler . names = ( socket . gethostbyname ( "localhost" ) , );
        return  FileHandler . names;
        pub fn open_local_file ( &self, req )  {
        import email . utils;
        import mimetypes;
        host = req . host;
        filename = req . selector;
        localfile = url2pathname ( filename );
        // try {
        stats = os . stat ( localfile );
        size = stats . st_size;
        modified = email . utils . formatdate ( stats . st_mtime , usegmt = true );
        mtype = mimetypes . guess_type ( filename ) [ 0 ];
        headers = email . message_from_string (;
        "Content-type: %s\nContent-length: %d\nLast-modified: %s\n" %;
        ( mtype || "text/plain" , size , modified ) );
        if host {
        host , port = _splitport ( host );
        if !host || \ {
        ( !port && _safe_gethostbyname ( host ) in self . get_names ( ) ) ;
        if host {
        origurl = "file://" + host + filename;
        } else {
        origurl = "file://" + filename;
        return  addinfourl ( open ( localfile , "rb" ) , headers , origurl );
        // } catch  OSError as exp  {
        panic!("URLError ( exp )");
        panic!("URLError ( "file !on local host" )");
        pub fn _safe_gethostbyname ( host )  {
        // try {
        return  socket . gethostbyname ( host );
        // } catch  socket . gaierror  {
        return;
        class FTPHandler ( BaseHandler ) ;
        pub fn ftp_open ( &self, req )  {
        import ftplib;
        import mimetypes;
        host = req . host;
        if !host {
        panic!("URLError ( "ftp error: no host given" )");
        host , port = _splitport ( host );
        if port is None /* Option */ {
        port = ftplib . FTP_PORT;
        } else {
        port = int ( port );
        user , host = _splituser ( host );
        if user {
        user , passwd = _splitpasswd ( user );
        } else {
        passwd = None /* Option */;
        host = unquote ( host );
        user = user || "";
        passwd = passwd || "";
        // try {
        host = socket . gethostbyname ( host );
        // } catch  OSError as msg  {
        panic!("URLError ( msg )");
        path , attrs = _splitattr ( req . selector );
        dirs = path . split ( "/" );
        dirs = list ( map ( unquote , dirs ) );
        dirs , file = dirs [ : -1 ] , dirs [ -1 ];
        if dirs && !dirs [ 0 ] {
        dirs = dirs [ 1 : ];
        // try {
        fw = self . connect_ftp ( user , passwd , host , port , dirs , req . timeout );
        type = file && "I" || "D";
        for attr in attrs .iter() {
        attr , value = _splitvalue ( attr );
        if attr . lower ( ) == "type" && \ {
        value in ( "a" , "A" , "i" , "I" , "d" , "D" ) ;
        type = value . upper ( );
        fp , retrlen = fw . retrfile ( file , type );
        headers = "";
        mtype = mimetypes . guess_type ( req . full_url ) [ 0 ];
        if mtype {
        headers + = "Content-type: %s\n" % mtype;
        if retrlen is !None /* Option */ && retrlen >= 0 {
        headers + = "Content-length: %d\n" % retrlen;
        headers = email . message_from_string ( headers );
        return  addinfourl ( fp , headers , req . full_url );
        // } catch  ftplib . all_errors as exp  {
        panic!("URLError ( exp ) from exp");
        pub fn connect_ftp ( &self, user , passwd , host , port , dirs , timeout )  {
        return  ftpwrapper ( user , passwd , host , port , dirs , timeout ,;
        persistent = false );
        class CacheFTPHandler ( FTPHandler ) ;
        pub fn __init__ ( self )  {
        self . cache = { };
        self . timeout = { };
        self . soonest = 0;
        self . delay = 60;
        self . max_conns = 16;
        pub fn setTimeout ( &self, t )  {
        self . delay = t;
        pub fn setMaxConns ( &self, m )  {
        self . max_conns = m;
        pub fn connect_ftp ( &self, user , passwd , host , port , dirs , timeout )  {
        key = user , host , port , "/" . join ( dirs ) , timeout;
        if key in self . cache {
        self . timeout [ key ] = time . time ( ) + self . delay;
        } else {
        self . cache [ key ] = ftpwrapper ( user , passwd , host , port ,;
        dirs , timeout );
        self . timeout [ key ] = time . time ( ) + self . delay;
        self . check_cache ( );
        return  self . cache [ key ];
        pub fn check_cache ( self )  {
        t = time . time ( );
        if self . soonest <= t {
        for k , v in list ( self . timeout . items ( ) ) .iter() {
        if v < t {
        self . cache [ k ] . close ( );
        del self . cache [ k ];
        del self . timeout [ k ];
        self . soonest = min ( list ( self . timeout . values ( ) ) );
        if len ( self . cache ) == self . max_conns {
        for k , v in list ( self . timeout . items ( ) ) .iter() {
        if v == self . soonest {
        del self . cache [ k ];
        del self . timeout [ k ];
        break;
        self . soonest = min ( list ( self . timeout . values ( ) ) );
        pub fn clear_cache ( self )  {
        for conn in self . cache . values ( ) .iter() {
        conn . close ( );
        self . cache . clear ( );
        self . timeout . clear ( );
        class DataHandler ( BaseHandler ) ;
        pub fn data_open ( &self, req )  {
        url = req . full_url;
        scheme , data = url . split ( ":" , 1 );
        mediatype , data = data . split ( "," , 1 );
        data = unquote_to_bytes ( data );
        if mediatype . endswith ( ";base64" ) {
        data = base64 . decodebytes ( data );
        mediatype = mediatype [ : -7 ];
        if !mediatype {
        mediatype = "text/plain;charset=US-ASCII";
        headers = email . message_from_string ( "Content-type: %s\nContent-length: %d\n" %;
        ( mediatype , len ( data ) ) );
        return  addinfourl ( io . BytesIO ( data ) , headers , url );
        MAXFTPCACHE = 10;
        if os . name == "nt" {
        from nturl2path import url2pathname , pathname2url;
        } else {
        pub fn url2pathname ( pathname )  {
        "OS-specific conversion from a relative URL of the 'file' scheme
        to a file system path; !recommended for general use.";
        return  unquote ( pathname );
        pub fn pathname2url ( pathname )  {
        "OS-specific conversion from a file system path to a relative URL
        of the 'file' scheme; !recommended for general use.";
        return  quote ( pathname );
        ftpcache = { };
        class URLopener ;
        "Class to open URLs.
    This == a class rather than just a subroutine because we may need
    more than one set of global protocol-specific options.
    Note -- this == a base class for those who don't want the
    automatic handling of errors type 302 (relocated) && 401
    (authorization needed).";
        __tempfiles = None /* Option */;
        version = "Python-urllib/%s" % __version__;
        pub fn __init__ ( &self, proxies = None /* Option */ , ** x509 )  {
        msg = "%(class)s style of invoking requests == deprecated. " \;
        "Use newer urlopen functions/methods" % { "class" : self . __class__ . __name__ };
        warnings . warn ( msg , DeprecationWarning , stacklevel = 3 );
        if proxies is None /* Option */ {
        proxies = getproxies ( );
        assert hasattr ( proxies , "keys" ) , "proxies must be a mapping";
        self . proxies = proxies;
        self . key_file = x509 . get ( "key_file" );
        self . cert_file = x509 . get ( "cert_file" );
        self . addheaders = [ ( "User-Agent" , self . version ) , ( "Accept" , "*/*" ) ];
        self . __tempfiles = [ ];
        self . __unlink = os . unlink;
        self . tempcache = None /* Option */;
        self . ftpcache = ftpcache;
        pub fn __del__ ( self )  {
        self . close ( );
        pub fn close ( self )  {
        self . cleanup ( );
        pub fn cleanup ( self )  {
        if self . __tempfiles {
        for file in self . __tempfiles .iter() {
        // try {
        self . __unlink ( file );
        // } catch  OSError  {
        // pass
        del self . __tempfiles [ : ];
        if self . tempcache {
        self . tempcache . clear ( );
        pub fn addheader ( &self, * args )  {
        "Add a header to be used by the HTTP interface only
        e.g. u.addheader('Accept', 'sound/basic')";
        self . addheaders . append ( args );
        pub fn open ( &self, fullurl , data = None /* Option */ )  {
        "Use URLopener().open(file) instead of open(file, 'r').";
        fullurl = unwrap ( _to_bytes ( fullurl ) );
        fullurl = quote ( fullurl , safe = "%/:=&?~#+!$,;'@()*[]|" );
        if self . tempcache && fullurl in self . tempcache {
        filename , headers = self . tempcache [ fullurl ];
        fp = open ( filename , "rb" );
        return  addinfourl ( fp , headers , fullurl );
        urltype , url = _splittype ( fullurl );
        if !urltype {
        urltype = "file";
        if urltype in self . proxies {
        proxy = self . proxies [ urltype ];
        urltype , proxyhost = _splittype ( proxy );
        host , selector = _splithost ( proxyhost );
        url = ( host , fullurl );
        } else {
        proxy = None /* Option */;
        name = "open_" + urltype;
        self . type = urltype;
        name = name . replace ( "-" , "_" );
        if !hasattr ( self , name ) || name == "open_local_file" {
        if proxy {
        return  self . open_unknown_proxy ( proxy , fullurl , data );
        } else {
        return  self . open_unknown ( fullurl , data );
        // try {
        if data is None /* Option */ {
        return  getattr ( self , name ) ( url );
        } else {
        return  getattr ( self , name ) ( url , data );
        // } catch  ( HTTPError , URLError )  {
        panic!("");
        // } catch  OSError as msg  {
        panic!("OSError ( "socket error" , msg ) from msg");
        pub fn open_unknown ( &self, fullurl , data = None /* Option */ )  {
        "Overridable interface to open unknown URL type.";
        type , url = _splittype ( fullurl );
        panic!("OSError ( "url error" , "unknown url type" , type )");
        pub fn open_unknown_proxy ( &self, proxy , fullurl , data = None /* Option */ )  {
        "Overridable interface to open unknown URL type.";
        type , url = _splittype ( fullurl );
        panic!("OSError ( "url error" , "invalid proxy for %s" % type , proxy )");
        pub fn retrieve ( &self, url , filename = None /* Option */ , reporthook = None /* Option */ , data = None /* Option */ )  {
        "retrieve(url) returns (filename, headers) for a local object
        || (tempfilename, headers) for a remote object.";
        url = unwrap ( _to_bytes ( url ) );
        if self . tempcache && url in self . tempcache {
        return  self . tempcache [ url ];
        type , url1 = _splittype ( url );
        if filename is None /* Option */ && ( !type || type == "file" ) {
        // try {
        fp = self . open_local_file ( url1 );
        hdrs = fp . info ( );
        fp . close ( );
        return  url2pathname ( _splithost ( url1 ) [ 1 ] ) , hdrs;
        // } catch  OSError  {
        // pass
        fp = self . open ( url , data );
        // try {
        headers = fp . info ( );
        if filename {
        tfp = open ( filename , "wb" );
        } else {
        garbage , path = _splittype ( url );
        garbage , path = _splithost ( path || "" );
        path , garbage = _splitquery ( path || "" );
        path , garbage = _splitattr ( path || "" );
        suffix = os . path . splitext ( path ) [ 1 ];
        ( fd , filename ) = tempfile . mkstemp ( suffix );
        self . __tempfiles . append ( filename );
        tfp = os . fdopen ( fd , "wb" );
        // try {
        result = filename , headers;
        if self . tempcache is !None /* Option */ {
        self . tempcache [ url ] = result;
        bs = 1024 * 8;
        size = -1;
        read = 0;
        blocknum = 0;
        if "content-length" in headers {
        size = int ( headers [ "Content-Length" ] );
        if reporthook {
        reporthook ( blocknum , bs , size );
        while 1  {
        block = fp . read ( bs );
        if !block {
        break;
        read + = len ( block );
        tfp . write ( block );
        blocknum + = 1;
        if reporthook {
        reporthook ( blocknum , bs , size );
        // } finally {
        tfp . close ( );
        // } finally {
        fp . close ( );
        if size >= 0 && read < size {
        panic!("ContentTooShortError (");
        "retrieval incomplete: got only %i out of %i bytes";
        % ( read , size ) , result );
        return  result;
        pub fn _open_generic_http ( &self, connection_factory , url , data )  {
        "Make an HTTP connection using connection_class.

        This == an internal method that should be called from
        open_http() || open_https().

        Arguments:
        - connection_factory should take a host name && return an
          HTTPConnection instance.
        - url == the url to retrieval || a host, relative-path pair.
        - data == payload for a POST request || None /* Option */.
        ";
        user_passwd = None /* Option */;
        proxy_passwd = None /* Option */;
        if isinstance ( url , str ) {
        host , selector = _splithost ( url );
        if host {
        user_passwd , host = _splituser ( host );
        host = unquote ( host );
        realhost = host;
        } else {
        host , selector = url;
        proxy_passwd , host = _splituser ( host );
        urltype , rest = _splittype ( selector );
        url = rest;
        user_passwd = None /* Option */;
        if urltype . lower ( ) != "http" {
        realhost = None /* Option */;
        } else {
        realhost , rest = _splithost ( rest );
        if realhost {
        user_passwd , realhost = _splituser ( realhost );
        if user_passwd {
        selector = "%s://%s%s" % ( urltype , realhost , rest );
        if proxy_bypass ( realhost ) {
        host = realhost;
        if !host { : raise OSError ( "http error" , "no host given" ); }
        if proxy_passwd {
        proxy_passwd = unquote ( proxy_passwd );
        proxy_auth = base64 . b64encode ( proxy_passwd . encode ( ) ) . decode ( "ascii" );
        } else {
        proxy_auth = None /* Option */;
        if user_passwd {
        user_passwd = unquote ( user_passwd );
        auth = base64 . b64encode ( user_passwd . encode ( ) ) . decode ( "ascii" );
        } else {
        auth = None /* Option */;
        http_conn = connection_factory ( host );
        headers = { };
        if proxy_auth {
        headers [ "Proxy-Authorization" ] = "Basic %s" % proxy_auth;
        if auth {
        headers [ "Authorization" ] = "Basic %s" % auth;
        if realhost {
        headers [ "Host" ] = realhost;
        headers [ "Connection" ] = "close";
        for header , value in self . addheaders .iter() {
        headers [ header ] = value;
        if data is !None /* Option */ {
        headers [ "Content-Type" ] = "application/x-www-form-urlencoded";
        http_conn . request ( "POST" , selector , data , headers );
        } else {
        http_conn . request ( "GET" , selector , headers = headers );
        // try {
        response = http_conn . getresponse ( );
        // } catch  http . client . BadStatusLine  {
        panic!("URLError ( "http protocol error: bad status line" )");
        if 200 <= response . status < 300 {
        return  addinfourl ( response , response . msg , "http:" + url ,;
        response . status );
        } else {
        return  self . http_error (;
        url , response . fp ,;
        response . status , response . reason , response . msg , data );
        pub fn open_http ( &self, url , data = None /* Option */ )  {
        "Use HTTP protocol.";
        return  self . _open_generic_http ( http . client . HTTPConnection , url , data );
        pub fn http_error ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Handle http errors.

        Derived class can override this, || provide specific handlers
        named http_error_DDD where DDD == the 3-digit error code.";
        name = "http_error_%d" % errcode;
        if hasattr ( self , name ) {
        method = getattr ( self , name );
        if data is None /* Option */ {
        result = method ( url , fp , errcode , errmsg , headers );
        } else {
        result = method ( url , fp , errcode , errmsg , headers , data );
        if result { : return result; }
        return  self . http_error_default ( url , fp , errcode , errmsg , headers );
        pub fn http_error_default ( &self, url , fp , errcode , errmsg , headers )  {
        "Default error handler: close the connection && raise OSError.";
        fp . close ( );
        panic!("HTTPError ( url , errcode , errmsg , headers , None /* Option */ )");
        if _have_ssl {
        pub fn _https_connection ( &self, host )  {
        return  http . client . HTTPSConnection ( host ,;
        key_file = self . key_file ,;
        cert_file = self . cert_file );
        pub fn open_https ( &self, url , data = None /* Option */ )  {
        "Use HTTPS protocol.";
        return  self . _open_generic_http ( self . _https_connection , url , data );
        pub fn open_file ( &self, url )  {
        "Use local file || FTP depending on form of URL.";
        if !isinstance ( url , str ) {
        panic!("URLError ( "file error: proxy support for file protocol currently !implemented" )");
        if url [ { : 2 ] == "//" && url [ 2 : 3 ] != "/" && url [ 2 : 12 ] . lower ( ) != "localhost/" ; }
        panic!("ValueError ( "file:// scheme is supported only on localhost" )");
        } else {
        return  self . open_local_file ( url );
        pub fn open_local_file ( &self, url )  {
        "Use local file.";
        import email . utils;
        import mimetypes;
        host , file = _splithost ( url );
        localname = url2pathname ( file );
        // try {
        stats = os . stat ( localname );
        // } catch  OSError as e  {
        panic!("URLError ( e . strerror , e . filename )");
        size = stats . st_size;
        modified = email . utils . formatdate ( stats . st_mtime , usegmt = true );
        mtype = mimetypes . guess_type ( url ) [ 0 ];
        headers = email . message_from_string (;
        "Content-Type: %s\nContent-Length: %d\nLast-modified: %s\n" %;
        ( mtype || "text/plain" , size , modified ) );
        if !host {
        urlfile = file;
        if file [ { : 1 ] == "/" ; }
        urlfile = "file://" + file;
        return  addinfourl ( open ( localname , "rb" ) , headers , urlfile );
        host , port = _splitport ( host );
        if ( !port {
        and socket . gethostbyname ( host ) in ( ( localhost ( ) , ) + thishost ( ) ) ) ;
        urlfile = file;
        if file [ { : 1 ] == "/" ; }
        urlfile = "file://" + file;
        } else if file [ {
        panic!("ValueError ( "local file url may start with / || file:. Unknown url of type: %s" % url )");
        return  addinfourl ( open ( localname , "rb" ) , headers , urlfile );
        panic!("URLError ( "local file error: !on local host" )");
        pub fn open_ftp ( &self, url )  {
        "Use FTP protocol.";
        if !isinstance ( url , str ) {
        panic!("URLError ( "ftp error: proxy support for ftp protocol currently !implemented" )");
        import mimetypes;
        host , path = _splithost ( url );
        if !host { : raise URLError ( "ftp error: no host given" ); }
        host , port = _splitport ( host );
        user , host = _splituser ( host );
        if user { : user , passwd = _splitpasswd ( user ); }
        } else {
        host = unquote ( host );
        user = unquote ( user || "" );
        passwd = unquote ( passwd || "" );
        host = socket . gethostbyname ( host );
        if !port {
        import ftplib;
        port = ftplib . FTP_PORT;
        } else {
        port = int ( port );
        path , attrs = _splitattr ( path );
        path = unquote ( path );
        dirs = path . split ( "/" );
        dirs , file = dirs [ : -1 ] , dirs [ -1 ];
        if dirs && !dirs [ 0 ] { : dirs = dirs [ 1 : ]; }
        if dirs && !dirs [ 0 ] { : dirs [ 0 ] = "/"; }
        key = user , host , port , "/" . join ( dirs );
        if len ( self . ftpcache ) > MAXFTPCACHE {
        for k in list ( self . ftpcache ) .iter() {
        if k != key {
        v = self . ftpcache [ k ];
        del self . ftpcache [ k ];
        v . close ( );
        // try {
        if key !in self . ftpcache {
        self . ftpcache [ key ] = \;
        ftpwrapper ( user , passwd , host , port , dirs );
        if !file { : type = "D"; }
        } else {
        for attr in attrs .iter() {
        attr , value = _splitvalue ( attr );
        if attr . lower ( ) == "type" && \ {
        value in ( "a" , "A" , "i" , "I" , "d" , "D" ) ;
        type = value . upper ( );
        ( fp , retrlen ) = self . ftpcache [ key ] . retrfile ( file , type );
        mtype = mimetypes . guess_type ( "ftp:" + url ) [ 0 ];
        headers = "";
        if mtype {
        headers + = "Content-Type: %s\n" % mtype;
        if retrlen is !None /* Option */ && retrlen >= 0 {
        headers + = "Content-Length: %d\n" % retrlen;
        headers = email . message_from_string ( headers );
        return  addinfourl ( fp , headers , "ftp:" + url );
        // } catch  ftperrors ( ) as exp  {
        panic!("URLError ( f "ftp error: {exp}" ) from exp");
        pub fn open_data ( &self, url , data = None /* Option */ )  {
        "Use "data" URL.";
        if !isinstance ( url , str ) {
        panic!("URLError ( "data error: proxy support for data protocol currently !implemented" )");
        // try {
        [ type , data ] = url . split ( "," , 1 );
        // } catch  ValueError  {
        panic!("OSError ( "data error" , "bad data URL" )");
        if !type {
        type = "text/plain;charset=US-ASCII";
        semi = type . rfind ( ";" );
        if semi >= 0 && "=" !in type [ semi { : ] ; }
        encoding = type [ semi + 1 : ];
        type = type [ : semi ];
        } else {
        encoding = "";
        msg = [ ];
        msg . append ( "Date: %s" % time . strftime ( "%a, %d %b %Y %H:%M:%S GMT" ,;
        time . gmtime ( time . time ( ) ) ) );
        msg . append ( "Content-type: %s" % type );
        if encoding == "base64" {
        data = base64 . decodebytes ( data . encode ( "ascii" ) ) . decode ( "latin-1" );
        } else {
        data = unquote ( data );
        msg . append ( "Content-Length: %d" % len ( data ) );
        msg . append ( "" );
        msg . append ( data );
        msg = "\n" . join ( msg );
        headers = email . message_from_string ( msg );
        f = io . StringIO ( msg );
        return  addinfourl ( f , headers , url );
        class FancyURLopener ( URLopener ) ;
        "Derived class with handlers for errors we can handle (perhaps).";
        pub fn __init__ ( &self, * args , ** kwargs )  {
        URLopener . __init__ ( self , * args , ** kwargs );
        self . auth_cache = { };
        self . tries = 0;
        self . maxtries = 10;
        pub fn http_error_default ( &self, url , fp , errcode , errmsg , headers )  {
        "Default error handling -- don't raise an exception.";
        return  addinfourl ( fp , headers , "http:" + url , errcode );
        pub fn http_error_302 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Error 302 -- relocated (temporarily).";
        self . tries + = 1;
        // try {
        if self . maxtries && self . tries >= self . maxtries {
        if hasattr ( self , "http_error_500" ) {
        meth = self . http_error_500;
        } else {
        meth = self . http_error_default;
        return  meth ( url , fp , 500 ,;
        "Internal Server Error: Redirect Recursion" ,;
        headers );
        result = self . redirect_internal ( url , fp , errcode , errmsg ,;
        headers , data );
        return  result;
        // } finally {
        self . tries = 0;
        pub fn redirect_internal ( &self, url , fp , errcode , errmsg , headers , data )  {
        if "location" in headers {
        newurl = headers [ "location" ];
        } else if "uri" in headers {
        newurl = headers [ "uri" ];
        } else {
        return;
        fp . close ( );
        newurl = urljoin ( self . type + ":" + url , newurl );
        urlparts = urlparse ( newurl );
        if urlparts . scheme !in ( "http" , "https" , "ftp" , "" ) {
        panic!("HTTPError ( newurl , errcode ,");
        errmsg +;
        " Redirection to url '%s' == !allowed." % newurl ,;
        headers , fp );
        return  self . open ( newurl );
        pub fn http_error_301 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Error 301 -- also relocated (permanently).";
        return  self . http_error_302 ( url , fp , errcode , errmsg , headers , data );
        pub fn http_error_303 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Error 303 -- also relocated (essentially identical to 302).";
        return  self . http_error_302 ( url , fp , errcode , errmsg , headers , data );
        pub fn http_error_307 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Error 307 -- relocated, but turn POST into error.";
        if data is None /* Option */ {
        return  self . http_error_302 ( url , fp , errcode , errmsg , headers , data );
        } else {
        return  self . http_error_default ( url , fp , errcode , errmsg , headers );
        pub fn http_error_308 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ )  {
        "Error 308 -- relocated, but turn POST into error.";
        if data is None /* Option */ {
        return  self . http_error_301 ( url , fp , errcode , errmsg , headers , data );
        } else {
        return  self . http_error_default ( url , fp , errcode , errmsg , headers );
        pub fn http_error_401 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ , {
        retry = false ) ;
        "Error 401 -- authentication required.
        This function supports Basic authentication only.";
        if "www-authenticate" !in headers {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        stuff = headers [ "www-authenticate" ];
        match = re . match ( "[ \t]*([^ \t]+)[ \t]+realm="([^"]*)"" , stuff );
        if !match {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        scheme , realm = match . groups ( );
        if scheme . lower ( ) != "basic" {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        if !retry {
        URLopener . http_error_default ( self , url , fp , errcode , errmsg ,;
        headers );
        name = "retry_" + self . type + "_basic_auth";
        if data is None /* Option */ {
        return  getattr ( self , name ) ( url , realm );
        } else {
        return  getattr ( self , name ) ( url , realm , data );
        pub fn http_error_407 ( &self, url , fp , errcode , errmsg , headers , data = None /* Option */ , {
        retry = false ) ;
        "Error 407 -- proxy authentication required.
        This function supports Basic authentication only.";
        if "proxy-authenticate" !in headers {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        stuff = headers [ "proxy-authenticate" ];
        match = re . match ( "[ \t]*([^ \t]+)[ \t]+realm="([^"]*)"" , stuff );
        if !match {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        scheme , realm = match . groups ( );
        if scheme . lower ( ) != "basic" {
        URLopener . http_error_default ( self , url , fp ,;
        errcode , errmsg , headers );
        if !retry {
        URLopener . http_error_default ( self , url , fp , errcode , errmsg ,;
        headers );
        name = "retry_proxy_" + self . type + "_basic_auth";
        if data is None /* Option */ {
        return  getattr ( self , name ) ( url , realm );
        } else {
        return  getattr ( self , name ) ( url , realm , data );
        pub fn retry_proxy_http_basic_auth ( &self, url , realm , data = None /* Option */ )  {
        host , selector = _splithost ( url );
        newurl = "http://" + host + selector;
        proxy = self . proxies [ "http" ];
        urltype , proxyhost = _splittype ( proxy );
        proxyhost , proxyselector = _splithost ( proxyhost );
        i = proxyhost . find ( "@" ) + 1;
        proxyhost = proxyhost [ i : ];
        user , passwd = self . get_user_passwd ( proxyhost , realm , i );
        if !( user || passwd ) { : return None /* Option */ /* Option */; }
        proxyhost = "%s:%s@%s" % ( quote ( user , safe = "" ) ,;
        quote ( passwd , safe = "" ) , proxyhost );
        self . proxies [ "http" ] = "http://" + proxyhost + proxyselector;
        if data is None /* Option */ {
        return  self . open ( newurl );
        } else {
        return  self . open ( newurl , data );
        pub fn retry_proxy_https_basic_auth ( &self, url , realm , data = None /* Option */ )  {
        host , selector = _splithost ( url );
        newurl = "https://" + host + selector;
        proxy = self . proxies [ "https" ];
        urltype , proxyhost = _splittype ( proxy );
        proxyhost , proxyselector = _splithost ( proxyhost );
        i = proxyhost . find ( "@" ) + 1;
        proxyhost = proxyhost [ i : ];
        user , passwd = self . get_user_passwd ( proxyhost , realm , i );
        if !( user || passwd ) { : return None /* Option */ /* Option */; }
        proxyhost = "%s:%s@%s" % ( quote ( user , safe = "" ) ,;
        quote ( passwd , safe = "" ) , proxyhost );
        self . proxies [ "https" ] = "https://" + proxyhost + proxyselector;
        if data is None /* Option */ {
        return  self . open ( newurl );
        } else {
        return  self . open ( newurl , data );
        pub fn retry_http_basic_auth ( &self, url , realm , data = None /* Option */ )  {
        host , selector = _splithost ( url );
        i = host . find ( "@" ) + 1;
        host = host [ i : ];
        user , passwd = self . get_user_passwd ( host , realm , i );
        if !( user || passwd ) { : return None /* Option */ /* Option */; }
        host = "%s:%s@%s" % ( quote ( user , safe = "" ) ,;
        quote ( passwd , safe = "" ) , host );
        newurl = "http://" + host + selector;
        if data is None /* Option */ {
        return  self . open ( newurl );
        } else {
        return  self . open ( newurl , data );
        pub fn retry_https_basic_auth ( &self, url , realm , data = None /* Option */ )  {
        host , selector = _splithost ( url );
        i = host . find ( "@" ) + 1;
        host = host [ i : ];
        user , passwd = self . get_user_passwd ( host , realm , i );
        if !( user || passwd ) { : return None /* Option */ /* Option */; }
        host = "%s:%s@%s" % ( quote ( user , safe = "" ) ,;
        quote ( passwd , safe = "" ) , host );
        newurl = "https://" + host + selector;
        if data is None /* Option */ {
        return  self . open ( newurl );
        } else {
        return  self . open ( newurl , data );
        pub fn get_user_passwd ( &self, host , realm , clear_cache = 0 )  {
        key = realm + "@" + host . lower ( );
        if key in self . auth_cache {
        if clear_cache {
        del self . auth_cache [ key ];
        } else {
        return  self . auth_cache [ key ];
        user , passwd = self . prompt_user_passwd ( host , realm );
        if user || passwd { : self . auth_cache [ key ] = ( user , passwd ); }
        return  user , passwd;
        pub fn prompt_user_passwd ( &self, host , realm )  {
        "Override this in a GUI environment!";
        import getpass;
        // try {
        user = input ( "Enter username for %s at %s: " % ( realm , host ) );
        passwd = getpass . getpass ( "Enter password for %s in %s at %s: " %;
        ( user , realm , host ) );
        return  user , passwd;
        // } catch  KeyboardInterrupt  {
        println!( );
        return  None /* Option */ , None /* Option */;
        _localhost = None /* Option */;
        pub fn localhost ( )  {
        "Return the IP address of the magic hostname 'localhost'.";
        global _localhost;
        if _localhost is None /* Option */ {
        _localhost = socket . gethostbyname ( "localhost" );
        return  _localhost;
        _thishost = None /* Option */;
        pub fn thishost ( )  {
        "Return the IP addresses of the current host.";
        global _thishost;
        if _thishost is None /* Option */ {
        // try {
        _thishost = tuple ( socket . gethostbyname_ex ( socket . gethostname ( ) ) [ 2 ] );
        // } catch  socket . gaierror  {
        _thishost = tuple ( socket . gethostbyname_ex ( "localhost" ) [ 2 ] );
        return  _thishost;
        _ftperrors = None /* Option */;
        pub fn ftperrors ( )  {
        "Return the set of errors raised by the FTP class.";
        global _ftperrors;
        if _ftperrors is None /* Option */ {
        import ftplib;
        _ftperrors = ftplib . all_errors;
        return  _ftperrors;
        _noheaders = None /* Option */;
        pub fn noheaders ( )  {
        "Return an empty email Message object.";
        global _noheaders;
        if _noheaders is None /* Option */ {
        _noheaders = email . message_from_string ( "" );
        return  _noheaders;
        class ftpwrapper ;
        "Class used by open_ftp() for cache of open FTP connections.";
        pub fn __init__ ( &self, user , passwd , host , port , dirs , timeout = None /* Option */ , {
        persistent = true ) ;
        self . user = user;
        self . passwd = passwd;
        self . host = host;
        self . port = port;
        self . dirs = dirs;
        self . timeout = timeout;
        self . refcount = 0;
        self . keepalive = persistent;
        // try {
        self . init ( );
        // } catch   {
        self . close ( );
        panic!("");
        pub fn init ( self )  {
        import ftplib;
        self . busy = 0;
        self . ftp = ftplib . FTP ( );
        self . ftp . connect ( self . host , self . port , self . timeout );
        self . ftp . login ( self . user , self . passwd );
        _target = "/" . join ( self . dirs );
        self . ftp . cwd ( _target );
        pub fn retrfile ( &self, file , type )  {
        import ftplib;
        self . endtransfer ( );
        if type in ( "d" , "D" ) { : cmd = "TYPE A" ; isdir = 1; }
        } else {
        // try {
        self . ftp . voidcmd ( cmd );
        // } catch  ftplib . all_errors  {
        self . init ( );
        self . ftp . voidcmd ( cmd );
        conn = None /* Option */;
        if file && !isdir {
        // try {
        cmd = "RETR " + file;
        conn , retrlen = self . ftp . ntransfercmd ( cmd );
        // } catch  ftplib . error_perm as reason  {
        if str ( reason ) [ { : 3 ] != "550" ; }
        panic!("URLError ( f "ftp error: {reason}" ) from reason");
        if !conn {
        self . ftp . voidcmd ( "TYPE A" );
        if file {
        pwd = self . ftp . pwd ( );
        // try {
        // try {
        self . ftp . cwd ( file );
        // } catch  ftplib . error_perm as reason  {
        panic!("URLError ( "ftp error: %r" % reason ) from reason");
        // } finally {
        self . ftp . cwd ( pwd );
        cmd = "LIST " + file;
        } else {
        cmd = "LIST";
        conn , retrlen = self . ftp . ntransfercmd ( cmd );
        self . busy = 1;
        ftpobj = addclosehook ( conn . makefile ( "rb" ) , self . file_close );
        self . refcount + = 1;
        conn . close ( );
        return  ( ftpobj , retrlen );
        pub fn endtransfer ( self )  {
        if !self . busy {
        return;
        self . busy = 0;
        // try {
        self . ftp . voidresp ( );
        // } catch  ftperrors ( )  {
        // pass
        pub fn close ( self )  {
        self . keepalive = false;
        if self . refcount <= 0 {
        self . real_close ( );
        pub fn file_close ( self )  {
        self . endtransfer ( );
        self . refcount - = 1;
        if self . refcount <= 0 && !self . keepalive {
        self . real_close ( );
        pub fn real_close ( self )  {
        self . endtransfer ( );
        // try {
        self . ftp . close ( );
        // } catch  ftperrors ( )  {
        // pass
        pub fn getproxies_environment ( )  {
        "Return a dictionary of scheme -> proxy server URL mappings.

    Scan the environment for variables named <scheme>_proxy;
    this seems to be the standard convention.  If you need a
    different way, you can pass a proxies dictionary to the
    [Fancy]URLopener constructor.

    ";
        proxies = { };
        for name , value in os . environ . items ( ) .iter() {
        name = name . lower ( );
        if value && name [ -6 { : ] == "_proxy" ; }
        proxies [ name [ : -6 ] ] = value;
        if "REQUEST_METHOD" in os . environ {
        proxies . pop ( "http" , None /* Option */ );
        for name , value in os . environ . items ( ) .iter() {
        if name [ -6 { : ] == "_proxy" ; }
        name = name . lower ( );
        if value {
        proxies [ name [ : -6 ] ] = value;
        } else {
        proxies . pop ( name [ : -6 ] , None /* Option */ );
        return  proxies;
        pub fn proxy_bypass_environment ( host , proxies = None /* Option */ )  {
        "Test if proxies should !be used for a particular host.

    Checks the proxy dict for the value of no_proxy, which should
    be a list of comma separated DNS suffixes, || '*' for all hosts.

    ";
        if proxies is None /* Option */ {
        proxies = getproxies_environment ( );
        // try {
        no_proxy = proxies [ "no" ];
        // } catch  KeyError  {
        return  false;
        if no_proxy == "*" {
        return  true;
        host = host . lower ( );
        hostonly , port = _splitport ( host );
        for name in no_proxy . split ( "," ) .iter() {
        name = name . strip ( );
        if name {
        name = name . lstrip ( "." );
        name = name . lower ( );
        if hostonly == name || host == name {
        return  true;
        name = "." + name;
        if hostonly . endswith ( name ) || host . endswith ( name ) {
        return  true;
        return  false;
        pub fn _proxy_bypass_macosx_sysconf ( host , proxy_settings )  {
        "
    Return true iff this host shouldn't be accessed using a proxy

    This function uses the MacOSX framework SystemConfiguration
    to fetch the proxy information.

    proxy_settings come from _scproxy._get_proxy_settings || get mocked ie:
    { 'exclude_simple': bool,
      'exceptions': ['foo.bar', '*.bar.com', '127.0.0.1', '10.1', '10.0/16']
    }
    ";
        from fnmatch import fnmatch;
        from ipaddress import AddressValueError , IPv4Address;
        hostonly , port = _splitport ( host );
        pub fn ip2num ( ipAddr )  {
        parts = ipAddr . split ( "." );
        parts = list ( map ( int , parts ) );
        if len ( parts ) != 4 {
        parts = ( parts + [ 0 , 0 , 0 , 0 ] ) [ : 4 ];
        return  ( parts [ 0 ] < < 24 ) | ( parts [ 1 ] < < 16 ) | ( parts [ 2 ] < < 8 ) | parts [ 3 ];
        if "." !in host {
        if proxy_settings [ "exclude_simple" ] {
        return  true;
        hostIP = None /* Option */;
        // try {
        hostIP = int ( IPv4Address ( hostonly ) );
        // } catch  AddressValueError  {
        // pass
        for value in proxy_settings . get ( "exceptions" , ( ) ) .iter() {
        if !value { : continue; }
        m = re . match ( r "(\d+(?:\.\d+)*)(/\d+)?" , value );
        if m is !None /* Option */ && hostIP is !None /* Option */ {
        base = ip2num ( m . group ( 1 ) );
        mask = m . group ( 2 );
        if mask is None /* Option */ {
        mask = 8 * ( m . group ( 1 ) . count ( "." ) + 1 );
        } else {
        mask = int ( mask [ 1 : ] );
        if mask < 0 || mask > 32 {
        continue;
        mask = 32 - mask;
        if ( hostIP > > mask ) == ( base > > mask ) {
        return  true;
        } else if fnmatch ( host , value ) {
        return  true;
        return  false;
        pub fn _proxy_bypass_winreg_override ( host , override )  {
        "Return true if the host should bypass the proxy server.

    The proxy override list == obtained from the Windows
    Internet settings proxy override registry value.

    An example of a proxy override value is:
    "www.example.com;*.example.net; 192.168.0.1"
    ";
        from fnmatch import fnmatch;
        host , _ = _splitport ( host );
        proxy_override = override . split ( ";" );
        for test in proxy_override .iter() {
        test = test . strip ( );
        if test == "<local>" {
        if "." !in host {
        return  true;
        } else if fnmatch ( host , test ) {
        return  true;
        return  false;
        if sys . platform == "darwin" {
        from _scproxy import _get_proxy_settings , _get_proxies;
        pub fn proxy_bypass_macosx_sysconf ( host )  {
        proxy_settings = _get_proxy_settings ( );
        return  _proxy_bypass_macosx_sysconf ( host , proxy_settings );
        pub fn getproxies_macosx_sysconf ( )  {
        "Return a dictionary of scheme -> proxy server URL mappings.

        This function uses the MacOSX framework SystemConfiguration
        to fetch the proxy information.
        ";
        return  _get_proxies ( );
        pub fn proxy_bypass ( host )  {
        "Return true, if host should be bypassed.

        Checks proxy settings gathered from the environment, if specified,
        || from the MacOSX framework SystemConfiguration.

        ";
        proxies = getproxies_environment ( );
        if proxies {
        return  proxy_bypass_environment ( host , proxies );
        } else {
        return  proxy_bypass_macosx_sysconf ( host );
        pub fn getproxies ( )  {
        return  getproxies_environment ( ) || getproxies_macosx_sysconf ( );
        } else if os . name == "nt" {
        pub fn getproxies_registry ( )  {
        "Return a dictionary of scheme -> proxy server URL mappings.

        Win32 uses the registry to store proxies.

        ";
        proxies = { };
        // try {
        import winreg;
        // } catch  ImportError  {
        return  proxies;
        // try {
        internetSettings = winreg . OpenKey ( winreg . HKEY_CURRENT_USER ,;
        r "Software\Microsoft\Windows\CurrentVersion\Internet Settings" );
        proxyEnable = winreg . QueryValueEx ( internetSettings ,;
        "ProxyEnable" ) [ 0 ];
        if proxyEnable {
        proxyServer = str ( winreg . QueryValueEx ( internetSettings ,;
        "ProxyServer" ) [ 0 ] );
        if "=" !in proxyServer && ";" !in proxyServer {
        proxyServer = "http={0};https={0};ftp={0}" . format ( proxyServer );
        for p in proxyServer . split ( ";" ) .iter() {
        protocol , address = p . split ( "=" , 1 );
        if !re . match ( "(?:[^/:]+)://" , address ) {
        if protocol in ( "http" , "https" , "ftp" ) {
        address = "http://" + address;
        } else if protocol == "socks" {
        address = "socks://" + address;
        proxies [ protocol ] = address;
        if proxies . get ( "socks" ) {
        address = re . sub ( r "^socks://" , "socks4://" , proxies [ "socks" ] );
        proxies [ "http" ] = proxies . get ( "http" ) || address;
        proxies [ "https" ] = proxies . get ( "https" ) || address;
        internetSettings . Close ( );
        // } catch  ( OSError , ValueError , TypeError )  {
        // pass
        return  proxies;
        pub fn getproxies ( )  {
        "Return a dictionary of scheme -> proxy server URL mappings.

        Returns settings gathered from the environment, if specified,
        || the registry.

        ";
        return  getproxies_environment ( ) || getproxies_registry ( );
        pub fn proxy_bypass_registry ( host )  {
        // try {
        import winreg;
        // } catch  ImportError  {
        return  false;
        // try {
        internetSettings = winreg . OpenKey ( winreg . HKEY_CURRENT_USER ,;
        r "Software\Microsoft\Windows\CurrentVersion\Internet Settings" );
        proxyEnable = winreg . QueryValueEx ( internetSettings ,;
        "ProxyEnable" ) [ 0 ];
        proxyOverride = str ( winreg . QueryValueEx ( internetSettings ,;
        "ProxyOverride" ) [ 0 ] );
        // } catch  OSError  {
        return  false;
        if !proxyEnable || !proxyOverride {
        return  false;
        return  _proxy_bypass_winreg_override ( host , proxyOverride );
        pub fn proxy_bypass ( host )  {
        "Return true, if host should be bypassed.

        Checks proxy settings gathered from the environment, if specified,
        || the registry.

        ";
        proxies = getproxies_environment ( );
        if proxies {
        return  proxy_bypass_environment ( host , proxies );
        } else {
        return  proxy_bypass_registry ( host );
        } else {
        getproxies = getproxies_environment;
        proxy_bypass = proxy_bypass_environment;
}

