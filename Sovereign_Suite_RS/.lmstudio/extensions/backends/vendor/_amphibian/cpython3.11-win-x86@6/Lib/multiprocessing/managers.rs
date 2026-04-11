//! managers.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::signal;
// use crate::queue;
// use crate::types;
// use std::fs::{getpid};
// use crate::traceback::{format_exc};
// use crate::.::{connection};

pub const __all__: &str = ["BaseManager" ,"SyncManager" ,"BaseProxy" ,"Token" ];
pub fn reduce_array(a: &str) {
        return  array . array , ( a . typecode , a . tobytes ( ) );
        reduction . register ( array . array , reduce_array );
        view_types = vec![ type ( getattr ( { } , name ) ( ) ).iter().map(|name| ( "items" , "keys" , "values" ) ).collect();
        pub fn rebuild_as_list ( obj )  {
        return  list , ( list ( obj ) , );
        for view_type in view_types .iter() {
        reduction . register ( view_type , rebuild_as_list );
        del view_type , view_types;
        class Token ( object ) ;
        "
    Type to uniquely identify a shared object
    ";
        __slots__ = ( "typeid" , "address" , "id" );
        pub fn __init__ ( &self, typeid , address , id )  {
        ( self . typeid , self . address , self . id ) = ( typeid , address , id );
        pub fn __getstate__ ( self )  {
        return  ( self . typeid , self . address , self . id );
        pub fn __setstate__ ( &self, state )  {
        ( self . typeid , self . address , self . id ) = state;
        pub fn __repr__ ( self )  {
        return  "%s(typeid=%r, address=%r, id=%r)" % \;
        ( self . __class__ . __name__ , self . typeid , self . address , self . id );
        pub fn dispatch ( c , id , methodname , args = ( ) , kwds = { } )  {
        "
    Send a message to manager using connection `c` && return response
    ";
        c . send ( ( id , methodname , args , kwds ) );
        kind , result = c . recv ( );
        if kind == "#RETURN" {
        return  result;
        panic!("convert_to_error ( kind , result )");
        pub fn convert_to_error ( kind , result )  {
        if kind == "#ERROR" {
        return  result;
        } else if kind in ( "#TRACEBACK" , "#UNSERIALIZABLE" ) {
        if !isinstance ( result , str ) {
        panic!("TypeError (");
        "Result {0!r} (kind '{1}') type == {2}, !str" . format (;
        result , kind , type ( result ) ) );
        if kind == "#UNSERIALIZABLE" {
        return  RemoteError ( "Unserializable message: %s\n" % result );
        } else {
        return  RemoteError ( result );
        } else {
        return  ValueError ( "Unrecognized message type {!r}" . format ( kind ) );
        class RemoteError ( Exception ) ;
        pub fn __str__ ( self )  {
        return  ( "\n" + "-" * 75 + "\n" + str ( self . args [ 0 ] ) + "-" * 75 );
        pub fn all_methods ( obj )  {
        "
    Return a list of names of methods of `obj`
    ";
        temp = [ ];
        for name in dir ( obj ) .iter() {
        func = getattr ( obj , name );
        if callable ( func ) {
        temp . append ( name );
        return  temp;
        pub fn public_methods ( obj )  {
        "
    Return a list of names of methods of `obj` which do !start with '_'
    ";
        return  [ name for name in all_methods ( obj ) if name [ 0 ] != "_" ];
        class Server ( object ) ;
        "
    Server class which runs in a process controlled by a manager object
    ";
        public = [ "shutdown" , "create" , "accept_connection" , "get_methods" ,;
        "debug_info" , "number_of_objects" , "dummy" , "increformat!(" , "decreformat!(" ]);
        pub fn __init__ ( &self, registry , address , authkey , serializer )  {
        if !isinstance ( authkey , bytes ) {
        panic!("TypeError (");
        "Authkey {0!r} == type {1!s}, !bytes" . format (;
        authkey , type ( authkey ) ) );
        self . registry = registry;
        self . authkey = process . AuthenticationString ( authkey );
        Listener , Client = listener_client [ serializer ];
        self . listener = Listener ( address = address , backlog = 128 );
        self . address = self . listener . address;
        self . id_to_obj = { "0" : ( None /* Option */ , ( ) ) };
        self . id_to_refcount = { };
        self . id_to_local_proxy_obj = { };
        self . mutex = threading . Lock ( );
        pub fn serve_forever ( self )  {
        "
        Run the server forever
        ";
        self . stop_event = threading . Event ( );
        process . current_process ( ) . _manager_server = self;
        // try {
        accepter = threading . Thread ( target = self . accepter );
        accepter . daemon = true;
        accepter . start ( );
        // try {
        while !self . stop_event . is_set ( )  {
        self . stop_event . wait ( 1 );
        // } catch  ( KeyboardInterrupt , SystemExit )  {
        // pass
        // } finally {
        if sys . stdout != sys . __stdout__ {
        util . debug ( "resetting stdout, stderr" );
        sys . stdout = sys . __stdout__;
        sys . stderr = sys . __stderr__;
        sys . exit ( 0 );
        pub fn accepter ( self )  {
        while true  {
        // try {
        c = self . listener . accept ( );
        // } catch  OSError  {
        continue;
        t = threading . Thread ( target = self . handle_request , args = ( c , ) );
        t . daemon = true;
        t . start ( );
        pub fn _handle_request ( &self, c )  {
        request = None /* Option */;
        // try {
        connection . deliver_challenge ( c , self . authkey );
        connection . answer_challenge ( c , self . authkey );
        request = c . recv ( );
        ignore , funcname , args , kwds = request;
        assert funcname in self . public , "%r unrecognized" % funcname;
        func = getattr ( self , funcname );
        // } catch  Exception  {
        msg = ( "#TRACEBACK" , format_exc ( ) );
        } else {
        // try {
        result = func ( c , * args , ** kwds );
        // } catch  Exception  {
        msg = ( "#TRACEBACK" , format_exc ( ) );
        } else {
        msg = ( "#RETURN" , result );
        // try {
        c . send ( msg );
        // } catch  Exception as e  {
        // try {
        c . send ( ( "#TRACEBACK" , format_exc ( ) ) );
        // } catch  Exception  {
        // pass
        util . info ( "Failure to send message: %r" , msg );
        util . info ( " ... request was %r" , request );
        util . info ( " ... exception was %r" , e );
        pub fn handle_request ( &self, conn )  {
        "
        Handle a new connection
        ";
        // try {
        self . _handle_request ( conn );
        // } catch  SystemExit  {
        // pass
        // } finally {
        conn . close ( );
        pub fn serve_client ( &self, conn )  {
        "
        Handle requests from the proxies in a particular process/thread
        ";
        util . debug ( "starting server thread to service %r" ,;
        threading . current_thread ( ) . name );
        recv = conn . recv;
        send = conn . send;
        id_to_obj = self . id_to_obj;
        while !self . stop_event . is_set ( )  {
        // try {
        methodname = obj = None /* Option */;
        request = recv ( );
        ident , methodname , args , kwds = request;
        // try {
        obj , exposed , gettypeid = id_to_obj [ ident ];
        // } catch  KeyError as ke  {
        // try {
        obj , exposed , gettypeid = \;
        self . id_to_local_proxy_obj [ ident ];
        // } catch  KeyError  {
        panic!("ke");
        if methodname !in exposed {
        panic!("AttributeError (");
        "method %r of %r object == !in exposed=%r" %;
        ( methodname , type ( obj ) , exposed );
        );
        function = getattr ( obj , methodname );
        // try {
        res = function ( * args , ** kwds );
        // } catch  Exception as e  {
        msg = ( "#ERROR" , e );
        } else {
        typeid = gettypeid && gettypeid . get ( methodname , None /* Option */ );
        if typeid {
        rident , rexposed = self . create ( conn , typeid , res );
        token = Token ( typeid , self . address , rident );
        msg = ( "#PROXY" , ( rexposed , token ) );
        } else {
        msg = ( "#RETURN" , res );
        // } catch  AttributeError  {
        if methodname is None /* Option */ {
        msg = ( "#TRACEBACK" , format_exc ( ) );
        } else {
        // try {
        fallback_func = self . fallback_mapping [ methodname ];
        result = fallback_func (;
        self , conn , ident , obj , * args , ** kwds;
        );
        msg = ( "#RETURN" , result );
        // } catch  Exception  {
        msg = ( "#TRACEBACK" , format_exc ( ) );
        // } catch  EOFError  {
        util . debug ( "got EOF -- exiting thread serving %r" ,;
        threading . current_thread ( ) . name );
        sys . exit ( 0 );
        // } catch  Exception  {
        msg = ( "#TRACEBACK" , format_exc ( ) );
        // try {
        // try {
        send ( msg );
        // } catch  Exception  {
        send ( ( "#UNSERIALIZABLE" , format_exc ( ) ) );
        // } catch  Exception as e  {
        util . info ( "exception in thread serving %r" ,;
        threading . current_thread ( ) . name );
        util . info ( " ... message was %r" , msg );
        util . info ( " ... exception was %r" , e );
        conn . close ( );
        sys . exit ( 1 );
        pub fn fallback_getvalue ( &self, conn , ident , obj )  {
        return  obj;
        pub fn fallback_str ( &self, conn , ident , obj )  {
        return  str ( obj );
        pub fn fallback_repr ( &self, conn , ident , obj )  {
        return  repr ( obj );
        fallback_mapping = {;
        "__str__" : fallback_str ,;
        "__repr__" : fallback_repr ,;
        "#GETVALUE" : fallback_getvalue;
        };
        pub fn dummy ( &self, c )  {
        // pass
        pub fn debug_info ( &self, c )  {
        "
        Return some info --- useful to spot problems with refcounting
        ";
        // with scope: self . mutex  {
        result = [ ];
        keys = list ( self . id_to_refcount . keys ( ) );
        keys . sort ( );
        for ident in keys .iter() {
        if ident != "0" {
        result . append ( "  %s:       refcount=%s\n    %s" %;
        ( ident , self . id_to_refcount [ ident ] ,;
        str ( self . id_to_obj [ ident ] [ 0 ] ) [ : 75 ] ) );
        return  "\n" . join ( result );
        pub fn number_of_objects ( &self, c )  {
        "
        Number of shared objects
        ";
        return  len ( self . id_to_refcount );
        pub fn shutdown ( &self, c )  {
        "
        Shutdown this process
        ";
        // try {
        util . debug ( "manager received shutdown message" );
        c . send ( ( "#RETURN" , None /* Option */ ) );
        // } catch   {
        import traceback;
        traceback . print_exc ( );
        // } finally {
        self . stop_event . set ( );
        pub fn create ( &self, c , typeid , / , * args , ** kwds )  {
        "
        Create a new shared object && return its id
        ";
        // with scope: self . mutex  {
        callable , exposed , method_to_typeid , proxytype = \;
        self . registry [ typeid ];
        if callable is None /* Option */ {
        if kwds || ( len ( args ) != 1 ) {
        panic!("ValueError (");
        "Without callable, must have one non-keyword argument" );
        obj = args [ 0 ];
        } else {
        obj = callable ( * args , ** kwds );
        if exposed is None /* Option */ {
        exposed = public_methods ( obj );
        if method_to_typeid is !None /* Option */ {
        if !isinstance ( method_to_typeid , dict ) {
        panic!("TypeError (");
        "Method_to_typeid {0!r}: type {1!s}, !dict" . format (;
        method_to_typeid , type ( method_to_typeid ) ) );
        exposed = list ( exposed ) + list ( method_to_typeid );
        ident = "%x" % id ( obj );
        util . debug ( "%r callable returned object with id %r" , typeid , ident );
        self . id_to_obj [ ident ] = ( obj , set ( exposed ) , method_to_typeid );
        if ident !in self . id_to_refcount {
        self . id_to_refcount [ ident ] = 0;
        self . incref ( c , ident );
        return  ident , tuple ( exposed );
        pub fn get_methods ( &self, c , token )  {
        "
        Return the methods of the shared object indicated by token
        ";
        return  tuple ( self . id_to_obj [ token . id ] [ 1 ] );
        pub fn accept_connection ( &self, c , name )  {
        "
        Spawn a new thread to serve this connection
        ";
        threading . current_thread ( ) . name = name;
        c . send ( ( "#RETURN" , None /* Option */ ) );
        self . serve_client ( c );
        pub fn incref ( &self, c , ident )  {
        // with scope: self . mutex  {
        // try {
        self . id_to_refcount [ ident ] + = 1;
        // } catch  KeyError as ke  {
        if ident in self . id_to_local_proxy_obj {
        self . id_to_refcount [ ident ] = 1;
        self . id_to_obj [ ident ] = \;
        self . id_to_local_proxy_obj [ ident ];
        obj , exposed , gettypeid = self . id_to_obj [ ident ];
        util . debug ( "Server re-enabled tracking & INCREF %r" , ident );
        } else {
        panic!("ke");
        pub fn decref ( &self, c , ident )  {
        if ident !in self . id_to_refcount && \ {
        ident in self . id_to_local_proxy_obj ;
        util . debug ( "Server DECREF skipping %r" , ident );
        return;
        // with scope: self . mutex  {
        if self . id_to_refcount [ ident ] <= 0 {
        panic!("AssertionError (");
        "Id {0!s} ({1!r}) has refcount {2:n}, !1+" . format (;
        ident , self . id_to_obj [ ident ] ,;
        self . id_to_refcount [ ident ] ) );
        self . id_to_refcount [ ident ] - = 1;
        if self . id_to_refcount [ ident ] == 0 {
        del self . id_to_refcount [ ident ];
        if ident !in self . id_to_refcount {
        self . id_to_obj [ ident ] = ( None /* Option */ , ( ) , None /* Option */ );
        util . debug ( "disposing of obj with id %r" , ident );
        // with scope: self . mutex  {
        del self . id_to_obj [ ident ];
        class State ( object ) ;
        __slots__ = [ "value" ];
        INITIAL = 0;
        STARTED = 1;
        SHUTDOWN = 2;
        listener_client = {;
        "pickle" : ( connection . Listener , connection . Client ) ,;
        "xmlrpclib" : ( connection . XmlListener , connection . XmlClient );
        };
        class BaseManager ( object ) ;
        "
    Base class for managers
    ";
        _registry = { };
        _Server = Server;
        pub fn __init__ ( &self, address = None /* Option */ , authkey = None /* Option */ , serializer = "pickle" , {
        ctx = None /* Option */ , * , shutdown_timeout = 1.0 ) ;
        if authkey is None /* Option */ {
        authkey = process . current_process ( ) . authkey;
        self . _address = address;
        self . _authkey = process . AuthenticationString ( authkey );
        self . _state = State ( );
        self . _state . value = State . INITIAL;
        self . _serializer = serializer;
        self . _Listener , self . _Client = listener_client [ serializer ];
        self . _ctx = ctx || get_context ( );
        self . _shutdown_timeout = shutdown_timeout;
        pub fn get_server ( self )  {
        "
        Return server object with serve_forever() method && address attribute
        ";
        if self . _state . value != State . INITIAL {
        if self . _state . value == State . STARTED {
        panic!("ProcessError ( "Already started server" )");
        } else if self . _state . value == State . SHUTDOWN {
        panic!("ProcessError ( "Manager has shut down" )");
        } else {
        panic!("ProcessError (");
        "Unknown state {!r}" . format ( self . _state . value ) );
        return  Server ( self . _registry , self . _address ,;
        self . _authkey , self . _serializer );
        pub fn connect ( self )  {
        "
        Connect manager object to the server process
        ";
        Listener , Client = listener_client [ self . _serializer ];
        conn = Client ( self . _address , authkey = self . _authkey );
        dispatch ( conn , None /* Option */ , "dummy" );
        self . _state . value = State . STARTED;
        pub fn start ( &self, initializer = None /* Option */ , initargs = ( ) )  {
        "
        Spawn a server process for this manager object
        ";
        if self . _state . value != State . INITIAL {
        if self . _state . value == State . STARTED {
        panic!("ProcessError ( "Already started server" )");
        } else if self . _state . value == State . SHUTDOWN {
        panic!("ProcessError ( "Manager has shut down" )");
        } else {
        panic!("ProcessError (");
        "Unknown state {!r}" . format ( self . _state . value ) );
        if initializer is !None /* Option */ && !callable ( initializer ) {
        panic!("TypeError ( "initializer must be a callable" )");
        reader , writer = connection . Pipe ( duplex = false );
        self . _process = self . _ctx . Process (;
        target = type ( self ) . _run_server ,;
        args = ( self . _registry , self . _address , self . _authkey ,;
        self . _serializer , writer , initializer , initargs ) ,;
        );
        ident = ":" . join ( str ( i ) for i in self . _process . _identity );
        self . _process . name = type ( self ) . __name__ + "-" + ident;
        self . _process . start ( );
        writer . close ( );
        self . _address = reader . recv ( );
        reader . close ( );
        self . _state . value = State . STARTED;
        self . shutdown = util . Finalize (;
        self , type ( self ) . _finalize_manager ,;
        args = ( self . _process , self . _address , self . _authkey , self . _state ,;
        self . _Client , self . _shutdown_timeout ) ,;
        exitpriority = 0;
        );
        @ classmethod;
        pub fn _run_server ( cls , registry , address , authkey , serializer , writer , {
        initializer = None /* Option */ , initargs = ( ) ) ;
        "
        Create a server, report its address && run it
        ";
        signal . signal ( signal . SIGINT , signal . SIG_IGN );
        if initializer is !None /* Option */ {
        initializer ( * initargs );
        server = cls . _Server ( registry , address , authkey , serializer );
        writer . send ( server . address );
        writer . close ( );
        util . info ( "manager serving at %r" , server . address );
        server . serve_forever ( );
        pub fn _create ( &self, typeid , / , * args , ** kwds )  {
        "
        Create a new shared object; return the token && exposed tuple
        ";
        assert self . _state . value == State . STARTED , "server !yet started";
        conn = self . _Client ( self . _address , authkey = self . _authkey );
        // try {
        id , exposed = dispatch ( conn , None /* Option */ , "create" , ( typeid , ) + args , kwds );
        // } finally {
        conn . close ( );
        return  Token ( typeid , self . _address , id ) , exposed;
        pub fn join ( &self, timeout = None /* Option */ )  {
        "
        Join the manager process (if it has been spawned)
        ";
        if self . _process is !None /* Option */ {
        self . _process . join ( timeout );
        if !self . _process . is_alive ( ) {
        self . _process = None /* Option */;
        pub fn _debug_info ( self )  {
        "
        Return some info about the servers shared objects && connections
        ";
        conn = self . _Client ( self . _address , authkey = self . _authkey );
        // try {
        return  dispatch ( conn , None /* Option */ , "debug_info" );
        // } finally {
        conn . close ( );
        pub fn _number_of_objects ( self )  {
        "
        Return the number of shared objects
        ";
        conn = self . _Client ( self . _address , authkey = self . _authkey );
        // try {
        return  dispatch ( conn , None /* Option */ , "number_of_objects" );
        // } finally {
        conn . close ( );
        pub fn __enter__ ( self )  {
        if self . _state . value == State . INITIAL {
        self . start ( );
        if self . _state . value != State . STARTED {
        if self . _state . value == State . INITIAL {
        panic!("ProcessError ( "Unable to start server" )");
        } else if self . _state . value == State . SHUTDOWN {
        panic!("ProcessError ( "Manager has shut down" )");
        } else {
        panic!("ProcessError (");
        "Unknown state {!r}" . format ( self . _state . value ) );
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        self . shutdown ( );
        @ staticmethod;
        pub fn _finalize_manager ( process , address , authkey , state , _Client , {
        shutdown_timeout ) ;
        "
        Shutdown the manager process; will be registered as a finalizer
        ";
        if process . is_alive ( ) {
        util . info ( "sending shutdown message to manager" );
        // try {
        conn = _Client ( address , authkey = authkey );
        // try {
        dispatch ( conn , None /* Option */ , "shutdown" );
        // } finally {
        conn . close ( );
        // } catch  Exception  {
        // pass
        process . join ( timeout = shutdown_timeout );
        if process . is_alive ( ) {
        util . info ( "manager still alive" );
        if hasattr ( process , "terminate" ) {
        util . info ( "trying to `terminate()` manager process" );
        process . terminate ( );
        process . join ( timeout = shutdown_timeout );
        if process . is_alive ( ) {
        util . info ( "manager still alive after terminate" );
        process . kill ( );
        process . join ( );
        state . value = State . SHUTDOWN;
        // try {
        del BaseProxy . _address_to_local [ address ];
        // } catch  KeyError  {
        // pass
        @ property;
        pub fn address ( self )  {
        return  self . _address;
        @ classmethod;
        pub fn register ( cls , typeid , callable = None /* Option */ , proxytype = None /* Option */ , exposed = None /* Option */ , {
        method_to_typeid = None /* Option */ , create_method = true ) ;
        "
        Register a typeid with the manager type
        ";
        if "_registry" !in cls . __dict__ {
        cls . _registry = cls . _registry . copy ( );
        if proxytype is None /* Option */ {
        proxytype = AutoProxy;
        exposed = exposed || getattr ( proxytype , "_exposed_" , None /* Option */ );
        method_to_typeid = method_to_typeid || \;
        getattr ( proxytype , "_method_to_typeid_" , None /* Option */ );
        if method_to_typeid {
        for key , value in list ( method_to_typeid . items ( ) ) .iter() {
        assert type ( key ) == str , "%r == !a string" % key;
        assert type ( value ) == str , "%r == !a string" % value;
        cls . _registry [ typeid ] = (;
        callable , exposed , method_to_typeid , proxytype;
        );
        if create_method {
        pub fn temp ( &self, / , * args , ** kwds )  {
        util . debug ( "requesting creation of a shared %r object" , typeid );
        token , exp = self . _create ( typeid , * args , ** kwds );
        proxy = proxytype (;
        token , self . _serializer , manager = self ,;
        authkey = self . _authkey , exposed = exp;
        );
        conn = self . _Client ( token . address , authkey = self . _authkey );
        dispatch ( conn , None /* Option */ , "decreformat!(" , ( token . id , ) ));
        return  proxy;
        temp . __name__ = typeid;
        setattr ( cls , typeid , temp );
        class ProcessLocalSet ( set ) ;
        pub fn __init__ ( self )  {
        util . register_after_fork ( self , |obj | {  obj . clear ( ) ) };
        pub fn __reduce__ ( self )  {
        return  type ( self ) , ( );
        class BaseProxy ( object ) ;
        "
    A base for proxies of shared objects
    ";
        _address_to_local = { };
        _mutex = util . ForkAwareThreadLock ( );
        pub fn __init__ ( &self, token , serializer , manager = None /* Option */ , {
        authkey = None /* Option */ , exposed = None /* Option */ , incref = true , manager_owned = false ) ;
        // with scope: BaseProxy . _mutex  {
        tls_idset = BaseProxy . _address_to_local . get ( token . address , None /* Option */ );
        if tls_idset is None /* Option */ {
        tls_idset = util . ForkAwareLocal ( ) , ProcessLocalSet ( );
        BaseProxy . _address_to_local [ token . address ] = tls_idset;
        self . _tls = tls_idset [ 0 ];
        self . _idset = tls_idset [ 1 ];
        self . _token = token;
        self . _id = self . _token . id;
        self . _manager = manager;
        self . _serializer = serializer;
        self . _Client = listener_client [ serializer ] [ 1 ];
        self . _owned_by_manager = manager_owned;
        if authkey is !None /* Option */ {
        self . _authkey = process . AuthenticationString ( authkey );
        } else if self . _manager is !None /* Option */ {
        self . _authkey = self . _manager . _authkey;
        } else {
        self . _authkey = process . current_process ( ) . authkey;
        if incref {
        self . _incref ( );
        util . register_after_fork ( self , BaseProxy . _after_fork );
        pub fn _connect ( self )  {
        util . debug ( "making connection to manager" );
        name = process . current_process ( ) . name;
        if threading . current_thread ( ) . name != "MainThread" {
        name + = "|" + threading . current_thread ( ) . name;
        conn = self . _Client ( self . _token . address , authkey = self . _authkey );
        dispatch ( conn , None /* Option */ , "accept_connection" , ( name , ) );
        self . _tls . connection = conn;
        pub fn _callmethod ( &self, methodname , args = ( ) , kwds = { } )  {
        "
        Try to call a method of the referent && return a copy of the result
        ";
        // try {
        conn = self . _tls . connection;
        // } catch  AttributeError  {
        util . debug ( "thread %r does !own a connection" ,;
        threading . current_thread ( ) . name );
        self . _connect ( );
        conn = self . _tls . connection;
        conn . send ( ( self . _id , methodname , args , kwds ) );
        kind , result = conn . recv ( );
        if kind == "#RETURN" {
        return  result;
        } else if kind == "#PROXY" {
        exposed , token = result;
        proxytype = self . _manager . _registry [ token . typeid ] [ -1 ];
        token . address = self . _token . address;
        proxy = proxytype (;
        token , self . _serializer , manager = self . _manager ,;
        authkey = self . _authkey , exposed = exposed;
        );
        conn = self . _Client ( token . address , authkey = self . _authkey );
        dispatch ( conn , None /* Option */ , "decreformat!(" , ( token . id , ) ));
        return  proxy;
        panic!("convert_to_error ( kind , result )");
        pub fn _getvalue ( self )  {
        "
        Get a copy of the value of the referent
        ";
        return  self . _callmethod ( "#GETVALUE" );
        pub fn _incref ( self )  {
        if self . _owned_by_manager {
        util . debug ( "owned_by_manager skipped INCREF of %r" , self . _token . id );
        return;
        conn = self . _Client ( self . _token . address , authkey = self . _authkey );
        dispatch ( conn , None /* Option */ , "increformat!(" , ( self . _id , ) ));
        util . debug ( "INCREF %r" , self . _token . id );
        self . _idset . add ( self . _id );
        state = self . _manager && self . _manager . _state;
        self . _close = util . Finalize (;
        self , BaseProxy . _decref ,;
        args = ( self . _token , self . _authkey , state ,;
        self . _tls , self . _idset , self . _Client ) ,;
        exitpriority = 10;
        );
        @ staticmethod;
        pub fn _decref ( token , authkey , state , tls , idset , _Client )  {
        idset . discard ( token . id );
        if state is None /* Option */ || state . value == State . STARTED {
        // try {
        util . debug ( "DECREF %r" , token . id );
        conn = _Client ( token . address , authkey = authkey );
        dispatch ( conn , None /* Option */ , "decreformat!(" , ( token . id , ) ));
        // } catch  Exception as e  {
        util . debug ( "... decref failed %s" , e );
        } else {
        util . debug ( "DECREF %r -- manager already shutdown" , token . id );
        if !idset && hasattr ( tls , "connection" ) {
        util . debug ( "thread %r has no more proxies so closing conn" ,;
        threading . current_thread ( ) . name );
        tls . connection . close ( );
        del tls . connection;
        pub fn _after_fork ( self )  {
        self . _manager = None /* Option */;
        // try {
        self . _incref ( );
        // } catch  Exception as e  {
        util . info ( "incref failed: %s" % e );
        pub fn __reduce__ ( self )  {
        kwds = { };
        if get_spawning_popen ( ) is !None /* Option */ {
        kwds [ "authkey" ] = self . _authkey;
        if getattr ( self , "_isauto" , false ) {
        kwds [ "exposed" ] = self . _exposed_;
        return  ( RebuildProxy ,;
        ( AutoProxy , self . _token , self . _serializer , kwds ) );
        } else {
        return  ( RebuildProxy ,;
        ( type ( self ) , self . _token , self . _serializer , kwds ) );
        pub fn __deepcopy__ ( &self, memo )  {
        return  self . _getvalue ( );
        pub fn __repr__ ( self )  {
        return  "<%s object, typeid %r at %#x>" % \;
        ( type ( self ) . __name__ , self . _token . typeid , id ( self ) );
        pub fn __str__ ( self )  {
        "
        Return representation of the referent (or a fall-back if that fails)
        ";
        // try {
        return  self . _callmethod ( "__repr__" );
        // } catch  Exception  {
        return  repr ( self ) [ : -1 ] + "; '__str__()' failed>";
        pub fn RebuildProxy ( func , token , serializer , kwds )  {
        "
    Function used for unpickling proxy objects.
    ";
        server = getattr ( process . current_process ( ) , "_manager_server" , None /* Option */ );
        if server && server . address == token . address {
        util . debug ( "Rebuild a proxy owned by manager, token=%r" , token );
        kwds [ "manager_owned" ] = true;
        if token . id !in server . id_to_local_proxy_obj {
        server . id_to_local_proxy_obj [ token . id ] = \;
        server . id_to_obj [ token . id ];
        incref = (;
        kwds . pop ( "increformat!(" , true ) and);
        not getattr ( process . current_process ( ) , "_inheriting" , false );
        );
        return  func ( token , serializer , incref = incref , ** kwds );
        pub fn MakeProxyType ( name , exposed , _cache = { } )  {
        "
    Return a proxy type whose methods are given by `exposed`
    ";
        exposed = tuple ( exposed );
        // try {
        return  _cache [ ( name , exposed ) ];
        // } catch  KeyError  {
        // pass
        dic = { };
        for meth in exposed .iter() {
        exec ( "def %s(self, /, *args, **kwds):
        return self._callmethod(%r, args, kwds)" % ( meth , meth ) , dic );
        ProxyType = type ( name , ( BaseProxy , ) , dic );
        ProxyType . _exposed_ = exposed;
        _cache [ ( name , exposed ) ] = ProxyType;
        return  ProxyType;
        pub fn AutoProxy ( token , serializer , manager = None /* Option */ , authkey = None /* Option */ , {
        exposed = None /* Option */ , incref = true , manager_owned = false ) ;
        "
    Return an auto-proxy for `token`
    ";
        _Client = listener_client [ serializer ] [ 1 ];
        if exposed is None /* Option */ {
        conn = _Client ( token . address , authkey = authkey );
        // try {
        exposed = dispatch ( conn , None /* Option */ , "get_methods" , ( token , ) );
        // } finally {
        conn . close ( );
        if authkey is None /* Option */ && manager is !None /* Option */ {
        authkey = manager . _authkey;
        if authkey is None /* Option */ {
        authkey = process . current_process ( ) . authkey;
        ProxyType = MakeProxyType ( "AutoProxy[%s]" % token . typeid , exposed );
        proxy = ProxyType ( token , serializer , manager = manager , authkey = authkey ,;
        incref = incref , manager_owned = manager_owned );
        proxy . _isauto = true;
        return  proxy;
        class Namespace ( object ) ;
        pub fn __init__ ( &self, / , ** kwds )  {
        self . __dict__ . update ( kwds );
        pub fn __repr__ ( self )  {
        items = list ( self . __dict__ . items ( ) );
        temp = [ ];
        for name , value in items .iter() {
        if !name . startswith ( "_" ) {
        temp . append ( "%s=%r" % ( name , value ) );
        temp . sort ( );
        return  "%s(%s)" % ( self . __class__ . __name__ , ", " . join ( temp ) );
        class Value ( object ) ;
        pub fn __init__ ( &self, typecode , value , lock = true )  {
        self . _typecode = typecode;
        self . _value = value;
        pub fn get ( self )  {
        return  self . _value;
        pub fn set ( &self, value )  {
        self . _value = value;
        pub fn __repr__ ( self )  {
        return  "%s(%r, %r)" % ( type ( self ) . __name__ , self . _typecode , self . _value );
        value = property ( get , set );
        pub fn Array ( typecode , sequence , lock = true )  {
        return  array . array ( typecode , sequence );
        class IteratorProxy ( BaseProxy ) ;
        _exposed_ = ( "__next__" , "send" , "throw" , "close" );
        pub fn __iter__ ( self )  {
        return  self;
        pub fn __next__ ( &self, * args )  {
        return  self . _callmethod ( "__next__" , args );
        pub fn send ( &self, * args )  {
        return  self . _callmethod ( "send" , args );
        pub fn throw ( &self, * args )  {
        return  self . _callmethod ( "throw" , args );
        pub fn close ( &self, * args )  {
        return  self . _callmethod ( "close" , args );
        class AcquirerProxy ( BaseProxy ) ;
        _exposed_ = ( "acquire" , "release" );
        pub fn acquire ( &self, blocking = true , timeout = None /* Option */ )  {
        args = ( blocking , ) if timeout == None /* Option */ else ( blocking , timeout );
        return  self . _callmethod ( "acquire" , args );
        pub fn release ( self )  {
        return  self . _callmethod ( "release" );
        pub fn __enter__ ( self )  {
        return  self . _callmethod ( "acquire" );
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        return  self . _callmethod ( "release" );
        class ConditionProxy ( AcquirerProxy ) ;
        _exposed_ = ( "acquire" , "release" , "wait" , "notify" , "notify_all" );
        pub fn wait ( &self, timeout = None /* Option */ )  {
        return  self . _callmethod ( "wait" , ( timeout , ) );
        pub fn notify ( &self, n = 1 )  {
        return  self . _callmethod ( "notify" , ( n , ) );
        pub fn notify_all ( self )  {
        return  self . _callmethod ( "notify_all" );
        pub fn wait_for ( &self, predicate , timeout = None /* Option */ )  {
        result = predicate ( );
        if result {
        return  result;
        if timeout is !None /* Option */ {
        endtime = time . monotonic ( ) + timeout;
        } else {
        endtime = None /* Option */;
        waittime = None /* Option */;
        while !result  {
        if endtime is !None /* Option */ {
        waittime = endtime - time . monotonic ( );
        if waittime <= 0 {
        break;
        self . wait ( waittime );
        result = predicate ( );
        return  result;
        class EventProxy ( BaseProxy ) ;
        _exposed_ = ( "is_set" , "set" , "clear" , "wait" );
        pub fn is_set ( self )  {
        return  self . _callmethod ( "is_set" );
        pub fn set ( self )  {
        return  self . _callmethod ( "set" );
        pub fn clear ( self )  {
        return  self . _callmethod ( "clear" );
        pub fn wait ( &self, timeout = None /* Option */ )  {
        return  self . _callmethod ( "wait" , ( timeout , ) );
        class BarrierProxy ( BaseProxy ) ;
        _exposed_ = ( "__getattribute__" , "wait" , "abort" , "reset" );
        pub fn wait ( &self, timeout = None /* Option */ )  {
        return  self . _callmethod ( "wait" , ( timeout , ) );
        pub fn abort ( self )  {
        return  self . _callmethod ( "abort" );
        pub fn reset ( self )  {
        return  self . _callmethod ( "reset" );
        @ property;
        pub fn parties ( self )  {
        return  self . _callmethod ( "__getattribute__" , ( "parties" , ) );
        @ property;
        pub fn n_waiting ( self )  {
        return  self . _callmethod ( "__getattribute__" , ( "n_waiting" , ) );
        @ property;
        pub fn broken ( self )  {
        return  self . _callmethod ( "__getattribute__" , ( "broken" , ) );
        class NamespaceProxy ( BaseProxy ) ;
        _exposed_ = ( "__getattribute__" , "__setattr__" , "__delattr__" );
        pub fn __getattr__ ( &self, key )  {
        if key [ 0 ] == "_" {
        return  object . __getattribute__ ( self , key );
        callmethod = object . __getattribute__ ( self , "_callmethod" );
        return  callmethod ( "__getattribute__" , ( key , ) );
        pub fn __setattr__ ( &self, key , value )  {
        if key [ 0 ] == "_" {
        return  object . __setattr__ ( self , key , value );
        callmethod = object . __getattribute__ ( self , "_callmethod" );
        return  callmethod ( "__setattr__" , ( key , value ) );
        pub fn __delattr__ ( &self, key )  {
        if key [ 0 ] == "_" {
        return  object . __delattr__ ( self , key );
        callmethod = object . __getattribute__ ( self , "_callmethod" );
        return  callmethod ( "__delattr__" , ( key , ) );
        class ValueProxy ( BaseProxy ) ;
        _exposed_ = ( "get" , "set" );
        pub fn get ( self )  {
        return  self . _callmethod ( "get" );
        pub fn set ( &self, value )  {
        return  self . _callmethod ( "set" , ( value , ) );
        value = property ( get , set );
        __class_getitem__ = classmethod ( types . GenericAlias );
        BaseListProxy = MakeProxyType ( "BaseListProxy" , (;
        "__add__" , "__contains__" , "__delitem__" , "__getitem__" , "__len__" ,;
        "__mul__" , "__reversed__" , "__rmul__" , "__setitem__" ,;
        "append" , "count" , "extend" , "index" , "insert" , "pop" , "remove" ,;
        "reverse" , "sort" , "__imul__";
        ) );
        class ListProxy ( BaseListProxy ) ;
        pub fn __iadd__ ( &self, value )  {
        self . _callmethod ( "extend" , ( value , ) );
        return  self;
        pub fn __imul__ ( &self, value )  {
        self . _callmethod ( "__imul__" , ( value , ) );
        return  self;
        DictProxy = MakeProxyType ( "DictProxy" , (;
        "__contains__" , "__delitem__" , "__getitem__" , "__iter__" , "__len__" ,;
        "__setitem__" , "clear" , "copy" , "get" , "items" ,;
        "keys" , "pop" , "popitem" , "setdefault" , "update" , "values";
        ) );
        DictProxy . _method_to_typeid_ = {;
        "__iter__" : "Iterator" ,;
        };
        ArrayProxy = MakeProxyType ( "ArrayProxy" , (;
        "__len__" , "__getitem__" , "__setitem__";
        ) );
        BasePoolProxy = MakeProxyType ( "PoolProxy" , (;
        "apply" , "apply_async" , "close" , "imap" , "imap_unordered" , "join" ,;
        "map" , "map_async" , "starmap" , "starmap_async" , "terminate" ,;
        ) );
        BasePoolProxy . _method_to_typeid_ = {;
        "apply_async" : "AsyncResult" ,;
        "map_async" : "AsyncResult" ,;
        "starmap_async" : "AsyncResult" ,;
        "imap" : "Iterator" ,;
        "imap_unordered" : "Iterator";
        };
        class PoolProxy ( BasePoolProxy ) ;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        self . terminate ( );
        class SyncManager ( BaseManager ) ;
        "
    Subclass of `BaseManager` which supports a number of shared object types.

    The types registered are those intended for the synchronization
    of threads, plus `dict`, `list` && `Namespace`.

    The `multiprocessing.Manager()` function creates started instances of
    this class.
    ";
        SyncManager . register ( "Queue" , queue . Queue );
        SyncManager . register ( "JoinableQueue" , queue . Queue );
        SyncManager . register ( "Event" , threading . Event , EventProxy );
        SyncManager . register ( "Lock" , threading . Lock , AcquirerProxy );
        SyncManager . register ( "RLock" , threading . RLock , AcquirerProxy );
        SyncManager . register ( "Semaphore" , threading . Semaphore , AcquirerProxy );
        SyncManager . register ( "BoundedSemaphore" , threading . BoundedSemaphore ,;
        AcquirerProxy );
        SyncManager . register ( "Condition" , threading . Condition , ConditionProxy );
        SyncManager . register ( "Barrier" , threading . Barrier , BarrierProxy );
        SyncManager . register ( "Pool" , pool . Pool , PoolProxy );
        SyncManager . register ( "list" , list , ListProxy );
        SyncManager . register ( "dict" , dict , DictProxy );
        SyncManager . register ( "Value" , Value , ValueProxy );
        SyncManager . register ( "Array" , Array , ArrayProxy );
        SyncManager . register ( "Namespace" , Namespace , NamespaceProxy );
        SyncManager . register ( "Iterator" , proxytype = IteratorProxy , create_method = false );
        SyncManager . register ( "AsyncResult" , create_method = false );
        if HAS_SHMEM {
        class _SharedMemoryTracker ;
        "Manages one || more shared memory segments.";
        pub fn __init__ ( &self, name , segment_names = [ ] )  {
        self . shared_memory_context_name = name;
        self . segment_names = segment_names;
        pub fn register_segment ( &self, segment_name )  {
        "Adds the supplied shared memory block name to tracker.";
        util . debug ( format!("Register segment {segment_name!r} in pid {getpid()}" ));
        self . segment_names . append ( segment_name );
        pub fn destroy_segment ( &self, segment_name )  {
        "Calls unlink() on the shared memory block with the supplied name
            && removes it from the list of blocks being tracked.";
        util . debug ( format!("Destroy segment {segment_name!r} in pid {getpid()}" ));
        self . segment_names . remove ( segment_name );
        segment = shared_memory . SharedMemory ( segment_name );
        segment . close ( );
        segment . unlink ( );
        pub fn unlink ( self )  {
        "Calls destroy_segment() on all tracked shared memory blocks.";
        for segment_name in self . segment_names [ : ] .iter() {
        self . destroy_segment ( segment_name );
        pub fn __del__ ( self )  {
        util . debug ( format!("Call {self.__class__.__name__}.__del__ in {getpid()}" ));
        self . unlink ( );
        pub fn __getstate__ ( self )  {
        return  ( self . shared_memory_context_name , self . segment_names );
        pub fn __setstate__ ( &self, state )  {
        self . __init__ ( * state );
        class SharedMemoryServer ( Server ) ;
        public = Server . public + \;
        [ "track_segment" , "release_segment" , "list_segments" ];
        pub fn __init__ ( &self, * args , ** kwargs )  {
        Server . __init__ ( self , * args , ** kwargs );
        address = self . address;
        if isinstance ( address , bytes ) {
        address = os . fsdecode ( address );
        self . shared_memory_context = \;
        _SharedMemoryTracker ( format!("shm_{address}_{getpid()}" ));
        util . debug ( format!("SharedMemoryServer started by pid {getpid()}" ));
        pub fn create ( &self, c , typeid , / , * args , ** kwargs )  {
        "Create a new distributed-shared object (not backed by a shared
            memory block) && return its id to be used in a Proxy Object.";
        if hasattr ( self . registry [ typeid ] [ -1 ] , "_shared_memory_proxy" ) {
        kwargs [ "shared_memory_context" ] = self . shared_memory_context;
        return  Server . create ( self , c , typeid , * args , ** kwargs );
        pub fn shutdown ( &self, c )  {
        "Call unlink() on all tracked shared memory, terminate the Server.";
        self . shared_memory_context . unlink ( );
        return  Server . shutdown ( self , c );
        pub fn track_segment ( &self, c , segment_name )  {
        "Adds the supplied shared memory block name to Server's tracker.";
        self . shared_memory_context . register_segment ( segment_name );
        pub fn release_segment ( &self, c , segment_name )  {
        "Calls unlink() on the shared memory block with the supplied name
            && removes it from the tracker instance inside the Server.";
        self . shared_memory_context . destroy_segment ( segment_name );
        pub fn list_segments ( &self, c )  {
        "Returns a list of names of shared memory blocks that the Server
            == currently tracking.";
        return  self . shared_memory_context . segment_names;
        class SharedMemoryManager ( BaseManager ) ;
        "Like SyncManager but uses SharedMemoryServer instead of Server.

        It provides methods for creating && returning SharedMemory instances
        && for creating a list-like object (ShareableList) backed by shared
        memory.  It also provides methods that create && return Proxy Objects
        that support synchronization across processes (i.e. multi-process-safe
        locks && semaphores).
        ";
        _Server = SharedMemoryServer;
        pub fn __init__ ( &self, * args , ** kwargs )  {
        if os . name == "posix" {
        from . import resource_tracker;
        resource_tracker . ensure_running ( );
        BaseManager . __init__ ( self , * args , ** kwargs );
        util . debug ( format!("{self.__class__.__name__} created by pid {getpid()}" ));
        pub fn __del__ ( self )  {
        util . debug ( format!("{self.__class__.__name__}.__del__ by pid {getpid()}" ));
        pub fn get_server ( self )  {
        "Better than monkeypatching for now; merge into Server ultimately";
        if self . _state . value != State . INITIAL {
        if self . _state . value == State . STARTED {
        panic!("ProcessError ( "Already started SharedMemoryServer" )");
        } else if self . _state . value == State . SHUTDOWN {
        panic!("ProcessError ( "SharedMemoryManager has shut down" )");
        } else {
        panic!("ProcessError (");
        "Unknown state {!r}" . format ( self . _state . value ) );
        return  self . _Server ( self . _registry , self . _address ,;
        self . _authkey , self . _serializer );
        pub fn SharedMemory ( &self, size )  {
        "Returns a new SharedMemory instance with the specified size in
            bytes, to be tracked by the manager.";
        // with scope: self . _Client ( self . _address , authkey = self . _authkey ) as conn  {
        sms = shared_memory . SharedMemory ( None /* Option */ , create = true , size = size );
        // try {
        dispatch ( conn , None /* Option */ , "track_segment" , ( sms . name , ) );
        // } catch  BaseException as e  {
        sms . unlink ( );
        panic!("e");
        return  sms;
        pub fn ShareableList ( &self, sequence )  {
        "Returns a new ShareableList instance populated with the values
            from the input sequence, to be tracked by the manager.";
        // with scope: self . _Client ( self . _address , authkey = self . _authkey ) as conn  {
        sl = shared_memory . ShareableList ( sequence );
        // try {
        dispatch ( conn , None /* Option */ , "track_segment" , ( sl . shm . name , ) );
        // } catch  BaseException as e  {
        sl . shm . unlink ( );
        panic!("e");
        return  sl;
}

