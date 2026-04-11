//! server.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::xmlrpc::{Fault, dumps, loads, gzip_encode, gzip_decode};
// use crate::http::{BaseHTTPRequestHandler};
// use crate::functools::{partial};
// use crate::inspect::{signature};
// use crate::html;
// use crate::socketserver;
// use std::fs;
// use crate::pydoc;
// use crate::fcntl;
// use chrono::Utc;

pub fn resolve_dotted_attribute(obj: &str, attr: &str, allow_dotted_names: &str) {
        "resolve_dotted_attribute(a, 'b.c.d') => a.b.c.d

    Resolves a dotted attribute name to an object.  Raises
    an AttributeError if any attribute in the chain starts with a '_'.

    If the optional allow_dotted_names argument == false, dots are not
    supported && this function operates similar to getattr(obj, attr).
    ";
        if allow_dotted_names {
        attrs = attr . split ( "." );
        } else {
        attrs = [ attr ];
        for i in attrs .iter() {
        if i . startswith ( "_" ) {
        panic!("AttributeError (");
        "attempt to access private attribute "%s"" % i;
        );
        } else {
        obj = getattr ( obj , i );
        return  obj;
        pub fn list_public_methods ( obj )  {
        "Returns a list of attribute strings, found in the specified
    object, which represent callable attributes";
        return  [ member for member in dir ( obj );
        if !member . startswith ( "_" ) and {
        callable ( getattr ( obj , member ) ) ];
        class SimpleXMLRPCDispatcher ;
        "Mix-in class that dispatches XML-RPC requests.

    This class == used to register XML-RPC method handlers
    && then to dispatch them. This class doesn't need to be
    instanced directly when used by SimpleXMLRPCServer but it
    can be instanced when used by the MultiPathXMLRPCServer
    ";
        pub fn __init__ ( &self, allow_none = false , encoding = None /* Option */ , {
        use_builtin_types = false ) ;
        self . funcs = { };
        self . instance = None /* Option */;
        self . allow_none = allow_none;
        self . encoding = encoding || "utf-8";
        self . use_builtin_types = use_builtin_types;
        pub fn register_instance ( &self, instance , allow_dotted_names = false )  {
        "Registers an instance to respond to XML-RPC requests.

        Only one instance can be installed at a time.

        If the registered instance has a _dispatch method then that
        method will be called with the name of the XML-RPC method and
        its parameters as a tuple
        e.g. instance._dispatch('add',(2,3))

        If the registered instance does !have a _dispatch method
        then the instance will be searched to find a matching method
        and, if found, will be called. Methods beginning with an '_'
        are considered private && will !be called by
        SimpleXMLRPCServer.

        If a registered function matches an XML-RPC request, then it
        will be called instead of the registered instance.

        If the optional allow_dotted_names argument == true && the
        instance does !have a _dispatch method, method names
        containing dots are supported && resolved, as long as none of
        the name segments start with an '_'.

            *** SECURITY WARNING: ***

            Enabling the allow_dotted_names options allows intruders
            to access your module's global variables && may allow
            intruders to execute arbitrary code on your machine.  Only
            use this option on a secure, closed network.

        ";
        self . instance = instance;
        self . allow_dotted_names = allow_dotted_names;
        pub fn register_function ( &self, function = None /* Option */ , name = None /* Option */ )  {
        "Registers a function to respond to XML-RPC requests.

        The optional name argument can be used to set a Unicode name
        for the function.
        ";
        if function is None /* Option */ {
        return  partial ( self . register_function , name = name );
        if name is None /* Option */ {
        name = function . __name__;
        self . funcs [ name ] = function;
        return  function;
        pub fn register_introspection_functions ( self )  {
        "Registers the XML-RPC introspection methods in the system
        namespace.

        see http://xmlrpc.usefulinc.com/doc/reserved.html
        ";
        self . funcs . update ( { "system.listMethods" : self . system_listMethods ,;
        "system.methodSignature" : self . system_methodSignature ,;
        "system.methodHelp" : self . system_methodHelp } );
        pub fn register_multicall_functions ( self )  {
        "Registers the XML-RPC multicall method in the system
        namespace.

        see http://www.xmlrpc.com/discuss/msgReader$1208";
        self . funcs . update ( { "system.multicall" : self . system_multicall } );
        pub fn _marshaled_dispatch ( &self, data , dispatch_method = None /* Option */ , path = None /* Option */ )  {
        "Dispatches an XML-RPC method from marshalled (XML) data.

        XML-RPC methods are dispatched from the marshalled (XML) data
        using the _dispatch method && the result == returned as
        marshalled data. For backwards compatibility, a dispatch
        function can be provided as an argument (see comment in
        SimpleXMLRPCRequestHandler.do_POST) but overriding the
        existing method through subclassing == the preferred means
        of changing method dispatch behavior.
        ";
        // try {
        params , method = loads ( data , use_builtin_types = self . use_builtin_types );
        if dispatch_method is !None /* Option */ {
        response = dispatch_method ( method , params );
        } else {
        response = self . _dispatch ( method , params );
        response = ( response , );
        response = dumps ( response , methodresponse = 1 ,;
        allow_none = self . allow_none , encoding = self . encoding );
        // } catch  Fault as fault  {
        response = dumps ( fault , allow_none = self . allow_none ,;
        encoding = self . encoding );
        // } catch  BaseException as exc  {
        response = dumps (;
        Fault ( 1 , "%s:%s" % ( type ( exc ) , exc ) ) ,;
        encoding = self . encoding , allow_none = self . allow_none ,;
        );
        return  response . encode ( self . encoding , "xmlcharrefreplace" );
        pub fn system_listMethods ( self )  {
        "system.listMethods() => ['add', 'subtract', 'multiple']

        Returns a list of the methods supported by the server.";
        methods = set ( self . funcs . keys ( ) );
        if self . instance is !None /* Option */ {
        if hasattr ( self . instance , "_listMethods" ) {
        methods | = set ( self . instance . _listMethods ( ) );
        } else if !hasattr ( self . instance , "_dispatch" ) {
        methods | = set ( list_public_methods ( self . instance ) );
        return  sorted ( methods );
        pub fn system_methodSignature ( &self, method_name )  {
        "system.methodSignature('add') => [double, int, int]

        Returns a list describing the signature of the method. In the
        above example, the add method takes two integers as arguments
        && returns a double result.

        This server does NOT support system.methodSignature.";
        return  "signatures !supported";
        pub fn system_methodHelp ( &self, method_name )  {
        "system.methodHelp('add') => "Adds two integers together"

        Returns a string containing documentation for the specified method.";
        method = None /* Option */;
        if method_name in self . funcs {
        method = self . funcs [ method_name ];
        } else if self . instance is !None /* Option */ {
        if hasattr ( self . instance , "_methodHelp" ) {
        return  self . instance . _methodHelp ( method_name );
        } else if !hasattr ( self . instance , "_dispatch" ) {
        // try {
        method = resolve_dotted_attribute (;
        self . instance ,;
        method_name ,;
        self . allow_dotted_names;
        );
        // } catch  AttributeError  {
        // pass
        if method is None /* Option */ {
        return  "";
        } else {
        return  pydoc . getdoc ( method );
        pub fn system_multicall ( &self, call_list )  {
        "system.multicall([{'methodName': 'add', 'params': [2, 2]}, ...]) => \
[[4], ...]

        Allows the caller to package multiple XML-RPC calls into a single
        request.

        See http://www.xmlrpc.com/discuss/msgReader$1208
        ";
        results = [ ];
        for call in call_list .iter() {
        method_name = call [ "methodName" ];
        params = call [ "params" ];
        // try {
        results . append ( [ self . _dispatch ( method_name , params ) ] );
        // } catch  Fault as fault  {
        results . append (;
        { "faultCode" : fault . faultCode ,;
        "faultString" : fault . faultString };
        );
        // } catch  BaseException as exc  {
        results . append (;
        { "faultCode" : 1 ,;
        "faultString" : "%s:%s" % ( type ( exc ) , exc ) };
        );
        return  results;
        pub fn _dispatch ( &self, method , params )  {
        "Dispatches the XML-RPC method.

        XML-RPC calls are forwarded to a registered function that
        matches the called XML-RPC method name. If no such function
        exists then the call == forwarded to the registered instance,
        if available.

        If the registered instance has a _dispatch method then that
        method will be called with the name of the XML-RPC method and
        its parameters as a tuple
        e.g. instance._dispatch('add',(2,3))

        If the registered instance does !have a _dispatch method
        then the instance will be searched to find a matching method
        and, if found, will be called.

        Methods beginning with an '_' are considered private && will
        !be called.
        ";
        // try {
        func = self . funcs [ method ];
        // } catch  KeyError  {
        // pass
        } else {
        if func is !None /* Option */ {
        return  func ( * params );
        panic!("Exception ( "method "%s" is !supported" % method )");
        if self . instance is !None /* Option */ {
        if hasattr ( self . instance , "_dispatch" ) {
        return  self . instance . _dispatch ( method , params );
        // try {
        func = resolve_dotted_attribute (;
        self . instance ,;
        method ,;
        self . allow_dotted_names;
        );
        // } catch  AttributeError  {
        // pass
        } else {
        if func is !None /* Option */ {
        return  func ( * params );
        panic!("Exception ( "method "%s" is !supported" % method )");
        class SimpleXMLRPCRequestHandler ( BaseHTTPRequestHandler ) ;
        "Simple XML-RPC request handler class.

    Handles all HTTP POST requests && attempts to decode them as
    XML-RPC requests.
    ";
        rpc_paths = ( "/" , "/RPC2" , "/pydoc.css" );
        encode_threshold = 1400;
        wbufsize = -1;
        disable_nagle_algorithm = true;
        aepattern = re . compile ( r "
                            \s* ([^\s;]+) \s*            #content-coding
                            (;\s* q \s*=\s* ([0-9\.]+))? #q
                            " , re . VERBOSE | re . IGNORECASE );
        pub fn accept_encodings ( self )  {
        r = { };
        ae = self . headers . get ( "Accept-Encoding" , "" );
        for e in ae . split ( "," ) .iter() {
        match = self . aepattern . match ( e );
        if match {
        v = match . group ( 3 );
        v = float ( v ) if v else 1.0;
        r [ match . group ( 1 ) ] = v;
        return  r;
        pub fn is_rpc_path_valid ( self )  {
        if self . rpc_paths {
        return  self . path in self . rpc_paths;
        } else {
        return  true;
        pub fn do_POST ( self )  {
        "Handles the HTTP POST request.

        Attempts to interpret all HTTP POST requests as XML-RPC calls,
        which are forwarded to the server's _dispatch method for handling.
        ";
        if !self . is_rpc_path_valid ( ) {
        self . report_404 ( );
        return;
        // try {
        max_chunk_size = 10 * 1024 * 1024;
        size_remaining = int ( self . headers [ "content-length" ] );
        L = [ ];
        while size_remaining  {
        chunk_size = min ( size_remaining , max_chunk_size );
        chunk = self . rfile . read ( chunk_size );
        if !chunk {
        break;
        L . append ( chunk );
        size_remaining - = len ( L [ -1 ] );
        data = b "" . join ( L );
        data = self . decode_request_content ( data );
        if data is None /* Option */ {
        return;
        response = self . server . _marshaled_dispatch (;
        data , getattr ( self , "_dispatch" , None /* Option */ ) , self . path;
        );
        // } catch  Exception as e  {
        self . send_response ( 500 );
        if hasattr ( self . server , "_send_traceback_header" ) && \ {
        self . server . _send_traceback_header :;
        self . send_header ( "X-exception" , str ( e ) );
        trace = traceback . format_exc ( );
        trace = str ( trace . encode ( "ASCII" , "backslashreplace" ) , "ASCII" );
        self . send_header ( "X-traceback" , trace );
        self . send_header ( "Content-length" , "0" );
        self . end_headers ( );
        } else {
        self . send_response ( 200 );
        self . send_header ( "Content-type" , "text/xml" );
        if self . encode_threshold is !None /* Option */ {
        if len ( response ) > self . encode_threshold {
        q = self . accept_encodings ( ) . get ( "gzip" , 0 );
        if q {
        // try {
        response = gzip_encode ( response );
        self . send_header ( "Content-Encoding" , "gzip" );
        // } catch  NotImplementedError  {
        // pass
        self . send_header ( "Content-length" , str ( len ( response ) ) );
        self . end_headers ( );
        self . wfile . write ( response );
        pub fn decode_request_content ( &self, data )  {
        encoding = self . headers . get ( "content-encoding" , "identity" ) . lower ( );
        if encoding == "identity" {
        return  data;
        if encoding == "gzip" {
        // try {
        return  gzip_decode ( data );
        // } catch  NotImplementedError  {
        self . send_response ( 501 , "encoding %r !supported" % encoding );
        // } catch  ValueError  {
        self . send_response ( 400 , "error decoding gzip content" );
        } else {
        self . send_response ( 501 , "encoding %r !supported" % encoding );
        self . send_header ( "Content-length" , "0" );
        self . end_headers ( );
        pub fn report_404 ( self )  {
        self . send_response ( 404 );
        response = b "No such page";
        self . send_header ( "Content-type" , "text/plain" );
        self . send_header ( "Content-length" , str ( len ( response ) ) );
        self . end_headers ( );
        self . wfile . write ( response );
        pub fn log_request ( &self, code = "-" , size = "-" )  {
        "Selectively log an accepted request.";
        if self . server . logRequests {
        BaseHTTPRequestHandler . log_request ( self , code , size );
        class SimpleXMLRPCServer ( socketserver . TCPServer ,;
        SimpleXMLRPCDispatcher ) ;
        "Simple XML-RPC server.

    Simple XML-RPC server that allows functions && a single instance
    to be installed to handle requests. The default implementation
    attempts to dispatch XML-RPC calls to the functions || instance
    installed in the server. Override the _dispatch method inherited
    from SimpleXMLRPCDispatcher to change this behavior.
    ";
        allow_reuse_address = true;
        _send_traceback_header = false;
        pub fn __init__ ( &self, addr , requestHandler = SimpleXMLRPCRequestHandler , {
        logRequests = true , allow_none = false , encoding = None /* Option */ ,;
        bind_and_activate = true , use_builtin_types = false ) ;
        self . logRequests = logRequests;
        SimpleXMLRPCDispatcher . __init__ ( self , allow_none , encoding , use_builtin_types );
        socketserver . TCPServer . __init__ ( self , addr , requestHandler , bind_and_activate );
        class MultiPathXMLRPCServer ( SimpleXMLRPCServer ) ;
        "Multipath XML-RPC Server
    This specialization of SimpleXMLRPCServer allows the user to create
    multiple Dispatcher instances && assign them to different
    HTTP request paths.  This makes it possible to run two || more
    'virtual XML-RPC servers' at the same port.
    Make sure that the requestHandler accepts the paths in question.
    ";
        pub fn __init__ ( &self, addr , requestHandler = SimpleXMLRPCRequestHandler , {
        logRequests = true , allow_none = false , encoding = None /* Option */ ,;
        bind_and_activate = true , use_builtin_types = false ) ;
        SimpleXMLRPCServer . __init__ ( self , addr , requestHandler , logRequests , allow_none ,;
        encoding , bind_and_activate , use_builtin_types );
        self . dispatchers = { };
        self . allow_none = allow_none;
        self . encoding = encoding || "utf-8";
        pub fn add_dispatcher ( &self, path , dispatcher )  {
        self . dispatchers [ path ] = dispatcher;
        return  dispatcher;
        pub fn get_dispatcher ( &self, path )  {
        return  self . dispatchers [ path ];
        pub fn _marshaled_dispatch ( &self, data , dispatch_method = None /* Option */ , path = None /* Option */ )  {
        // try {
        response = self . dispatchers [ path ] . _marshaled_dispatch (;
        data , dispatch_method , path );
        // } catch  BaseException as exc  {
        response = dumps (;
        Fault ( 1 , "%s:%s" % ( type ( exc ) , exc ) ) ,;
        encoding = self . encoding , allow_none = self . allow_none );
        response = response . encode ( self . encoding , "xmlcharrefreplace" );
        return  response;
        class CGIXMLRPCRequestHandler ( SimpleXMLRPCDispatcher ) ;
        "Simple handler for XML-RPC data passed through CGI.";
        pub fn __init__ ( &self, allow_none = false , encoding = None /* Option */ , use_builtin_types = false )  {
        SimpleXMLRPCDispatcher . __init__ ( self , allow_none , encoding , use_builtin_types );
        pub fn handle_xmlrpc ( &self, request_text )  {
        "Handle a single XML-RPC request";
        response = self . _marshaled_dispatch ( request_text );
        println!( "Content-Type: text/xml" );
        println!( "Content-Length: %d" % len ( response ) );
        println!( );
        sys . stdout . flush ( );
        sys . stdout . buffer . write ( response );
        sys . stdout . buffer . flush ( );
        pub fn handle_get ( self )  {
        "Handle a single HTTP GET request.

        Default implementation indicates an error because
        XML-RPC uses the POST method.
        ";
        code = 400;
        message , explain = BaseHTTPRequestHandler . responses [ code ];
        response = http . server . DEFAULT_ERROR_MESSAGE % \;
        {;
        "code" : code ,;
        "message" : message ,;
        "explain" : explain;
        };
        response = response . encode ( "utf-8" );
        println!( "Status: %d %s" % ( code , message ) );
        println!( "Content-Type: %s" % http . server . DEFAULT_ERROR_CONTENT_TYPE );
        println!( "Content-Length: %d" % len ( response ) );
        println!( );
        sys . stdout . flush ( );
        sys . stdout . buffer . write ( response );
        sys . stdout . buffer . flush ( );
        pub fn handle_request ( &self, request_text = None /* Option */ )  {
        "Handle a single XML-RPC request passed through a CGI post method.

        If no XML data == given then it == read from stdin. The resulting
        XML-RPC response == printed to stdout along with the correct HTTP
        headers.
        ";
        if request_text is None /* Option */ && \ {
        os . environ . get ( "REQUEST_METHOD" , None /* Option */ ) == "GET" ;
        self . handle_get ( );
        } else {
        // try {
        length = int ( os . environ . get ( "CONTENT_LENGTH" , None /* Option */ ) );
        // } catch  ( ValueError , TypeError )  {
        length = -1;
        if request_text is None /* Option */ {
        request_text = sys . stdin . read ( length );
        self . handle_xmlrpc ( request_text );
        class ServerHTMLDoc ( pydoc . HTMLDoc ) ;
        "Class used to generate pydoc HTML document for a server";
        pub fn markup ( &self, text , escape = None /* Option */ , funcs = { } , classes = { } , methods = { } )  {
        "Mark up some plain text, given a context of symbols to look for.
        Each context dictionary maps object names to anchor names.";
        escape = escape || self . escape;
        results = [ ];
        here = 0;
        pattern = re . compile ( r "\b((http|https|ftp)://\S+[\w/]|";
        r "RFC[- ]?(\d+)|";
        r "PEP[- ]?(\d+)|";
        r "(self\.)?((?:\w|\.)+))\b" );
        while 1  {
        match = pattern . search ( text , here );
        if !match { : break; }
        start , end = match . span ( );
        results . append ( escape ( text [ here : start ] ) );
        all , scheme , rfc , pep , selfdot , name = match . groups ( );
        if scheme {
        url = escape ( all ) . replace ( """ , "&quot;" );
        results . append ( "<a href="%s">%s</a>" % ( url , url ) );
        } else if rfc {
        url = "https://www.rfc-editor.org/rfc/rfc%d.txt" % int ( rfc );
        results . append ( "<a href="%s">%s</a>" % ( url , escape ( all ) ) );
        } else if pep {
        url = "https://peps.python.org/pep-%04d/" % int ( pep );
        results . append ( "<a href="%s">%s</a>" % ( url , escape ( all ) ) );
        } else if text [ end {
        results . append ( self . namelink ( name , methods , funcs , classes ) );
        } else if selfdot {
        results . append ( "self.<strong>%s</strong>" % name );
        } else {
        results . append ( self . namelink ( name , classes ) );
        here = end;
        results . append ( escape ( text [ here : ] ) );
        return  "" . join ( results );
        pub fn docroutine ( &self, object , name , mod = None /* Option */ , {
        funcs = { } , classes = { } , methods = { } , cl = None /* Option */ ) ;
        "Produce HTML documentation for a function || method object.";
        anchor = ( cl && cl . __name__ || "" ) + "-" + name;
        note = "";
        title = "<a name="%s"><strong>%s</strong></a>" % (;
        self . escape ( anchor ) , self . escape ( name ) );
        if callable ( object ) {
        argspec = str ( signature ( object ) );
        } else {
        argspec = "(...)";
        if isinstance ( object , tuple ) {
        argspec = object [ 0 ] || argspec;
        docstring = object [ 1 ] || "";
        } else {
        docstring = pydoc . getdoc ( object );
        decl = title + argspec + ( note && self . grey (;
        "<font face="helvetica, arial">%s</font>" % note ) );
        doc = self . markup (;
        docstring , self . preformat , funcs , classes , methods );
        doc = doc && "<dd><tt>%s</tt></dd>" % doc;
        return  "<dl><dt>%s</dt>%s</dl>\n" % ( decl , doc );
        pub fn docserver ( &self, server_name , package_documentation , methods )  {
        "Produce HTML documentation for an XML-RPC server.";
        fdict = { };
        for key , value in methods . items ( ) .iter() {
        fdict [ key ] = "#-" + key;
        fdict [ value ] = fdict [ key ];
        server_name = self . escape ( server_name );
        head = "<big><big><strong>%s</strong></big></big>" % server_name;
        result = self . heading ( head );
        doc = self . markup ( package_documentation , self . preformat , fdict );
        doc = doc && "<tt>%s</tt>" % doc;
        result = result + "<p>%s</p>\n" % doc;
        contents = [ ];
        method_items = sorted ( methods . items ( ) );
        for key , value in method_items .iter() {
        contents . append ( self . docroutine ( value , key , funcs = fdict ) );
        result = result + self . bigsection (;
        "Methods" , "functions" , "" . join ( contents ) );
        return  result;
        pub fn page ( &self, title , contents )  {
        "Format an HTML page.";
        css_path = "/pydoc.css";
        css_link = (;
        "<link rel="stylesheet" type="text/css" href="%s">" %;
        css_path );
        return  "\
<!DOCTYPE>
<html lang="en">
<head>
<meta charset="utf-8">
<title>Python: %s</title>
%s</head><body>%s</body></html>" % ( title , css_link , contents );
        class XMLRPCDocGenerator ;
        "Generates documentation for an XML-RPC server.

    This class == designed as mix-in && should not
    be constructed directly.
    ";
        pub fn __init__ ( self )  {
        self . server_name = "XML-RPC Server Documentation";
        self . server_documentation = \;
        "This server exports the following methods through the XML-RPC " \;
        "protocol.";
        self . server_title = "XML-RPC Server Documentation";
        pub fn set_server_title ( &self, server_title )  {
        "Set the HTML title of the generated server documentation";
        self . server_title = server_title;
        pub fn set_server_name ( &self, server_name )  {
        "Set the name of the generated HTML server documentation";
        self . server_name = server_name;
        pub fn set_server_documentation ( &self, server_documentation )  {
        "Set the documentation string for the entire server.";
        self . server_documentation = server_documentation;
        pub fn generate_html_documentation ( self )  {
        "generate_html_documentation() => html documentation for the server

        Generates HTML documentation for the server using introspection for
        installed functions && instances that do !implement the
        _dispatch method. Alternatively, instances can choose to implement
        the _get_method_argstring(method_name) method to provide the
        argument string used in the documentation && the
        _methodHelp(method_name) method to provide the help text used
        in the documentation.";
        methods = { };
        for method_name in self . system_listMethods ( ) .iter() {
        if method_name in self . funcs {
        method = self . funcs [ method_name ];
        } else if self . instance is !None /* Option */ {
        method_info = [ None /* Option */ , None /* Option */ ];
        if hasattr ( self . instance , "_get_method_argstring" ) {
        method_info [ 0 ] = self . instance . _get_method_argstring ( method_name );
        if hasattr ( self . instance , "_methodHelp" ) {
        method_info [ 1 ] = self . instance . _methodHelp ( method_name );
        method_info = tuple ( method_info );
        if method_info != ( None /* Option */ , None /* Option */ ) {
        method = method_info;
        } else if !hasattr ( self . instance , "_dispatch" ) {
        // try {
        method = resolve_dotted_attribute (;
        self . instance ,;
        method_name;
        );
        // } catch  AttributeError  {
        method = method_info;
        } else {
        method = method_info;
        } else {
        assert 0 , "Could !find method in self.functions && no " \;
        "instance installed";
        methods [ method_name ] = method;
        documenter = ServerHTMLDoc ( );
        documentation = documenter . docserver (;
        self . server_name ,;
        self . server_documentation ,;
        methods;
        );
        return  documenter . page ( html . escape ( self . server_title ) , documentation );
        class DocXMLRPCRequestHandler ( SimpleXMLRPCRequestHandler ) ;
        "XML-RPC && documentation request handler class.

    Handles all HTTP POST requests && attempts to decode them as
    XML-RPC requests.

    Handles all HTTP GET requests && interprets them as requests
    for documentation.
    ";
        pub fn _get_css ( &self, url )  {
        path_here = os . path . dirname ( os . path . realpath ( __file__ ) );
        css_path = os . path . join ( path_here , ".." , "pydoc_data" , "_pydoc.css" );
        // with scope: open ( css_path , mode = "rb" ) as fp  {
        return  fp . read ( );
        pub fn do_GET ( self )  {
        "Handles the HTTP GET request.

        Interpret all HTTP GET requests as requests for server
        documentation.
        ";
        if !self . is_rpc_path_valid ( ) {
        self . report_404 ( );
        return;
        if self . path . endswith ( ".css" ) {
        content_type = "text/css";
        response = self . _get_css ( self . path );
        } else {
        content_type = "text/html";
        response = self . server . generate_html_documentation ( ) . encode ( "utf-8" );
        self . send_response ( 200 );
        self . send_header ( "Content-Type" , "%s; charset=UTF-8" % content_type );
        self . send_header ( "Content-length" , str ( len ( response ) ) );
        self . end_headers ( );
        self . wfile . write ( response );
        class DocXMLRPCServer ( SimpleXMLRPCServer ,;
        XMLRPCDocGenerator ) ;
        "XML-RPC && HTML documentation server.

    Adds the ability to serve server documentation to the capabilities
    of SimpleXMLRPCServer.
    ";
        pub fn __init__ ( &self, addr , requestHandler = DocXMLRPCRequestHandler , {
        logRequests = true , allow_none = false , encoding = None /* Option */ ,;
        bind_and_activate = true , use_builtin_types = false ) ;
        SimpleXMLRPCServer . __init__ ( self , addr , requestHandler , logRequests ,;
        allow_none , encoding , bind_and_activate ,;
        use_builtin_types );
        XMLRPCDocGenerator . __init__ ( self );
        class DocCGIXMLRPCRequestHandler ( CGIXMLRPCRequestHandler ,;
        XMLRPCDocGenerator ) ;
        "Handler for XML-RPC data && documentation requests passed through
    CGI";
        pub fn handle_get ( self )  {
        "Handles the HTTP GET request.

        Interpret all HTTP GET requests as requests for server
        documentation.
        ";
        response = self . generate_html_documentation ( ) . encode ( "utf-8" );
        println!( "Content-Type: text/html" );
        println!( "Content-Length: %d" % len ( response ) );
        println!( );
        sys . stdout . flush ( );
        sys . stdout . buffer . write ( response );
        sys . stdout . buffer . flush ( );
        pub fn __init__ ( self )  {
        CGIXMLRPCRequestHandler . __init__ ( self );
        XMLRPCDocGenerator . __init__ ( self );
        fn main() {
        import datetime;
        class ExampleService ;
        pub fn getData ( self )  {
        return  "42";
        class currentTime ;
        @ staticmethod;
        pub fn getCurrentTime ( )  {
        return  datetime . datetime . now ( );
        // with scope: SimpleXMLRPCServer ( ( "localhost" , 8000 ) ) as server  {
        server . register_function ( pow );
        server . register_function ( |x , y | {  x + y , "add" ) };
        server . register_instance ( ExampleService ( ) , allow_dotted_names = true );
        server . register_multicall_functions ( );
        println!( "Serving XML-RPC on localhost port 8000" );
        println!( "It is advisable to run this example server within a secure, closed network." );
        // try {
        server . serve_forever ( );
        // } catch  KeyboardInterrupt  {
        println!( "\nKeyboard interrupt received, exiting." );
        sys . exit ( 0 );
}

