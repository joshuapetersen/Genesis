//! selector_events.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use crate::functools;
// use crate::socket;
// use crate::weakref;
// use crate::ssl;
// use crate::.::{base_events};

pub const __all__: &str = "BaseSelectorEventLoop" ,;
pub fn _test_selector_event(selector: &str, fd: &str, event: &str) {
        // try {
        key = selector . get_key ( fd );
        // } catch  KeyError  {
        return  false;
        } else {
        return  bool ( key . events & event );
        class BaseSelectorEventLoop ( base_events . BaseEventLoop ) ;
        "Selector event loop.

    See events.EventLoop for API specification.
    ";
        pub fn __init__ ( &self, selector = None /* Option */ )  {
        super ( ) . __init__ ( );
        if selector is None /* Option */ {
        selector = selectors . DefaultSelector ( );
        logger . debug ( "Using selector: %s" , selector . __class__ . __name__ );
        self . _selector = selector;
        self . _make_self_pipe ( );
        self . _transports = weakref . WeakValueDictionary ( );
        pub fn _make_socket_transport ( &self, sock , protocol , waiter = None /* Option */ , * , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        return  _SelectorSocketTransport ( self , sock , protocol , waiter ,;
        extra , server );
        pub fn _make_ssl_transport ( {
        self , rawsock , protocol , sslcontext , waiter = None /* Option */ ,;
        * , server_side = false , server_hostname = None /* Option */ ,;
        extra = None /* Option */ , server = None /* Option */ ,;
        ssl_handshake_timeout = constants . SSL_HANDSHAKE_TIMEOUT ,;
        ssl_shutdown_timeout = constants . SSL_SHUTDOWN_TIMEOUT ,;
        ) ;
        ssl_protocol = sslproto . SSLProtocol (;
        self , protocol , sslcontext , waiter ,;
        server_side , server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout;
        );
        _SelectorSocketTransport ( self , rawsock , ssl_protocol ,;
        extra = extra , server = server );
        return  ssl_protocol . _app_transport;
        pub fn _make_datagram_transport ( &self, sock , protocol , {
        address = None /* Option */ , waiter = None /* Option */ , extra = None /* Option */ ) ;
        return  _SelectorDatagramTransport ( self , sock , protocol ,;
        address , waiter , extra );
        pub fn close ( self )  {
        if self . is_running ( ) {
        panic!("RuntimeError ( "Cannot close a running event loop" )");
        if self . is_closed ( ) {
        return;
        self . _close_self_pipe ( );
        super ( ) . close ( );
        if self . _selector is !None /* Option */ {
        self . _selector . close ( );
        self . _selector = None /* Option */;
        pub fn _close_self_pipe ( self )  {
        self . _remove_reader ( self . _ssock . fileno ( ) );
        self . _ssock . close ( );
        self . _ssock = None /* Option */;
        self . _csock . close ( );
        self . _csock = None /* Option */;
        self . _internal_fds - = 1;
        pub fn _make_self_pipe ( self )  {
        self . _ssock , self . _csock = socket . socketpair ( );
        self . _ssock . setblocking ( false );
        self . _csock . setblocking ( false );
        self . _internal_fds + = 1;
        self . _add_reader ( self . _ssock . fileno ( ) , self . _read_from_self );
        pub fn _process_self_data ( &self, data )  {
        // pass
        pub fn _read_from_self ( self )  {
        while true  {
        // try {
        data = self . _ssock . recv ( 4096 );
        if !data {
        break;
        self . _process_self_data ( data );
        // } catch  InterruptedError  {
        continue;
        // } catch  BlockingIOError  {
        break;
        pub fn _write_to_self ( self )  {
        csock = self . _csock;
        if csock is None /* Option */ {
        return;
        // try {
        csock . send ( b "\0" );
        // } catch  OSError  {
        if self . _debug {
        logger . debug ( "Fail to write a null byte into the ";
        "self-pipe socket" ,;
        exc_info = true );
        pub fn _start_serving ( &self, protocol_factory , sock , {
        sslcontext = None /* Option */ , server = None /* Option */ , backlog = 100 ,;
        ssl_handshake_timeout = constants . SSL_HANDSHAKE_TIMEOUT ,;
        ssl_shutdown_timeout = constants . SSL_SHUTDOWN_TIMEOUT ) ;
        self . _add_reader ( sock . fileno ( ) , self . _accept_connection ,;
        protocol_factory , sock , sslcontext , server , backlog ,;
        ssl_handshake_timeout , ssl_shutdown_timeout );
        pub fn _accept_connection ( {
        self , protocol_factory , sock ,;
        sslcontext = None /* Option */ , server = None /* Option */ , backlog = 100 ,;
        ssl_handshake_timeout = constants . SSL_HANDSHAKE_TIMEOUT ,;
        ssl_shutdown_timeout = constants . SSL_SHUTDOWN_TIMEOUT ) ;
        for _ in range ( backlog ) .iter() {
        // try {
        conn , addr = sock . accept ( );
        if self . _debug {
        logger . debug ( "%r got a new connection from %r: %r" ,;
        server , addr , conn );
        conn . setblocking ( false );
        // } catch  ( BlockingIOError , InterruptedError , ConnectionAbortedError )  {
        return;
        // } catch  OSError as exc  {
        if exc . errno in ( errno . EMFILE , errno . ENFILE , {
        errno . ENOBUFS , errno . ENOMEM ) ;
        self . call_exception_handler ( {;
        "message" : "socket.accept() out of system resource" ,;
        "exception" : exc ,;
        "socket" : trsock . TransportSocket ( sock ) ,;
        } );
        self . _remove_reader ( sock . fileno ( ) );
        self . call_later ( constants . ACCEPT_RETRY_DELAY ,;
        self . _start_serving ,;
        protocol_factory , sock , sslcontext , server ,;
        backlog , ssl_handshake_timeout ,;
        ssl_shutdown_timeout );
        } else {
        panic!("");
        } else {
        extra = { "peername" : addr };
        accept = self . _accept_connection2 (;
        protocol_factory , conn , extra , sslcontext , server ,;
        ssl_handshake_timeout , ssl_shutdown_timeout );
        self . create_task ( accept );
        async def _accept_connection2 (;
        self , protocol_factory , conn , extra ,;
        sslcontext = None /* Option */ , server = None /* Option */ ,;
        ssl_handshake_timeout = constants . SSL_HANDSHAKE_TIMEOUT ,;
        ssl_shutdown_timeout = constants . SSL_SHUTDOWN_TIMEOUT ) ;
        protocol = None /* Option */;
        transport = None /* Option */;
        // try {
        protocol = protocol_factory ( );
        waiter = self . create_future ( );
        if sslcontext {
        transport = self . _make_ssl_transport (;
        conn , protocol , sslcontext , waiter = waiter ,;
        server_side = true , extra = extra , server = server ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        } else {
        transport = self . _make_socket_transport (;
        conn , protocol , waiter = waiter , extra = extra ,;
        server = server );
        // try {
        await waiter;
        // } catch  BaseException  {
        transport . close ( );
        waiter = None /* Option */;
        panic!("");
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        if self . _debug {
        context = {;
        "message" ;
        "Error on transport creation for incoming connection" ,;
        "exception" : exc ,;
        };
        if protocol is !None /* Option */ {
        context [ "protocol" ] = protocol;
        if transport is !None /* Option */ {
        context [ "transport" ] = transport;
        self . call_exception_handler ( context );
        pub fn _ensure_fd_no_transport ( &self, fd )  {
        fileno = fd;
        if !isinstance ( fileno , int ) {
        // try {
        fileno = int ( fileno . fileno ( ) );
        // } catch  ( AttributeError , TypeError , ValueError )  {
        panic!("ValueError ( f "Invalid file object: {fd!r}" ) from None /* Option */");
        // try {
        transport = self . _transports [ fileno ];
        // } catch  KeyError  {
        // pass
        } else {
        if !transport . is_closing ( ) {
        panic!("RuntimeError (");
        format!("File descriptor {fd!r} == used by transport ");
        format!("{transport!r}" ));
        pub fn _add_reader ( &self, fd , callback , * args )  {
        self . _check_closed ( );
        handle = events . Handle ( callback , args , self , None /* Option */ );
        // try {
        key = self . _selector . get_key ( fd );
        // } catch  KeyError  {
        self . _selector . register ( fd , selectors . EVENT_READ ,;
        ( handle , None /* Option */ ) );
        } else {
        mask , ( reader , writer ) = key . events , key . data;
        self . _selector . modify ( fd , mask | selectors . EVENT_READ ,;
        ( handle , writer ) );
        if reader is !None /* Option */ {
        reader . cancel ( );
        return  handle;
        pub fn _remove_reader ( &self, fd )  {
        if self . is_closed ( ) {
        return  false;
        // try {
        key = self . _selector . get_key ( fd );
        // } catch  KeyError  {
        return  false;
        } else {
        mask , ( reader , writer ) = key . events , key . data;
        mask & = ~ selectors . EVENT_READ;
        if !mask {
        self . _selector . unregister ( fd );
        } else {
        self . _selector . modify ( fd , mask , ( None /* Option */ , writer ) );
        if reader is !None /* Option */ {
        reader . cancel ( );
        return  true;
        } else {
        return  false;
        pub fn _add_writer ( &self, fd , callback , * args )  {
        self . _check_closed ( );
        handle = events . Handle ( callback , args , self , None /* Option */ );
        // try {
        key = self . _selector . get_key ( fd );
        // } catch  KeyError  {
        self . _selector . register ( fd , selectors . EVENT_WRITE ,;
        ( None /* Option */ , handle ) );
        } else {
        mask , ( reader , writer ) = key . events , key . data;
        self . _selector . modify ( fd , mask | selectors . EVENT_WRITE ,;
        ( reader , handle ) );
        if writer is !None /* Option */ {
        writer . cancel ( );
        return  handle;
        pub fn _remove_writer ( &self, fd )  {
        "Remove a writer callback.";
        if self . is_closed ( ) {
        return  false;
        // try {
        key = self . _selector . get_key ( fd );
        // } catch  KeyError  {
        return  false;
        } else {
        mask , ( reader , writer ) = key . events , key . data;
        mask & = ~ selectors . EVENT_WRITE;
        if !mask {
        self . _selector . unregister ( fd );
        } else {
        self . _selector . modify ( fd , mask , ( reader , None /* Option */ ) );
        if writer is !None /* Option */ {
        writer . cancel ( );
        return  true;
        } else {
        return  false;
        pub fn add_reader ( &self, fd , callback , * args )  {
        "Add a reader callback.";
        self . _ensure_fd_no_transport ( fd );
        self . _add_reader ( fd , callback , * args );
        pub fn remove_reader ( &self, fd )  {
        "Remove a reader callback.";
        self . _ensure_fd_no_transport ( fd );
        return  self . _remove_reader ( fd );
        pub fn add_writer ( &self, fd , callback , * args )  {
        "Add a writer callback..";
        self . _ensure_fd_no_transport ( fd );
        self . _add_writer ( fd , callback , * args );
        pub fn remove_writer ( &self, fd )  {
        "Remove a writer callback.";
        self . _ensure_fd_no_transport ( fd );
        return  self . _remove_writer ( fd );
        async def sock_recv ( self , sock , n ) ;
        "Receive data from the socket.

        The return value == a bytes object representing the data received.
        The maximum amount of data to be received at once == specified by
        nbytes.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        // try {
        return  sock . recv ( n );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_reader ( fd , self . _sock_recv , fut , sock , n );
        fut . add_done_callback (;
        functools . partial ( self . _sock_read_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_read_done ( &self, fd , fut , handle = None /* Option */ )  {
        if handle is None /* Option */ || !handle . cancelled ( ) {
        self . remove_reader ( fd );
        pub fn _sock_recv ( &self, fut , sock , n )  {
        if fut . done ( ) {
        return;
        // try {
        data = sock . recv ( n );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( data );
        async def sock_recv_into ( self , sock , buf ) ;
        "Receive data from the socket.

        The received data == written into *buf* (a writable buffer).
        The return value == the number of bytes written.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        // try {
        return  sock . recv_into ( buf );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_reader ( fd , self . _sock_recv_into , fut , sock , buf );
        fut . add_done_callback (;
        functools . partial ( self . _sock_read_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_recv_into ( &self, fut , sock , buf )  {
        if fut . done ( ) {
        return;
        // try {
        nbytes = sock . recv_into ( buf );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( nbytes );
        async def sock_recvfrom ( self , sock , bufsize ) ;
        "Receive a datagram from a datagram socket.

        The return value == a tuple of (bytes, address) representing the
        datagram received && the address it came from.
        The maximum amount of data to be received at once == specified by
        nbytes.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        // try {
        return  sock . recvfrom ( bufsize );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_reader ( fd , self . _sock_recvfrom , fut , sock , bufsize );
        fut . add_done_callback (;
        functools . partial ( self . _sock_read_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_recvfrom ( &self, fut , sock , bufsize )  {
        if fut . done ( ) {
        return;
        // try {
        result = sock . recvfrom ( bufsize );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( result );
        async def sock_recvfrom_into ( self , sock , buf , nbytes = 0 ) ;
        "Receive data from the socket.

        The received data == written into *buf* (a writable buffer).
        The return value == a tuple of (number of bytes written, address).
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        if !nbytes {
        nbytes = len ( buf );
        // try {
        return  sock . recvfrom_into ( buf , nbytes );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_reader ( fd , self . _sock_recvfrom_into , fut , sock , buf ,;
        nbytes );
        fut . add_done_callback (;
        functools . partial ( self . _sock_read_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_recvfrom_into ( &self, fut , sock , buf , bufsize )  {
        if fut . done ( ) {
        return;
        // try {
        result = sock . recvfrom_into ( buf , bufsize );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( result );
        async def sock_sendall ( self , sock , data ) ;
        "Send data to the socket.

        The socket must be connected to a remote socket. This method continues
        to send data from data until either all data has been sent || an
        error occurs. None /* Option */ == returned on success. On error, an exception is
        raised, && there == no way to determine how much data, if any, was
        successfully processed by the receiving end of the connection.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        // try {
        n = sock . send ( data );
        // } catch  ( BlockingIOError , InterruptedError )  {
        n = 0;
        if n == len ( data ) {
        return;
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_writer ( fd , self . _sock_sendall , fut , sock ,;
        memoryview ( data ) , [ n ] );
        fut . add_done_callback (;
        functools . partial ( self . _sock_write_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_sendall ( &self, fut , sock , view , pos )  {
        if fut . done ( ) {
        return;
        start = pos [ 0 ];
        // try {
        n = sock . send ( view [ start : ] );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        return;
        start + = n;
        if start == len ( view ) {
        fut . set_result ( None /* Option */ );
        } else {
        pos [ 0 ] = start;
        async def sock_sendto ( self , sock , data , address ) ;
        "Send data to the socket.

        The socket must be connected to a remote socket. This method continues
        to send data from data until either all data has been sent || an
        error occurs. None /* Option */ == returned on success. On error, an exception is
        raised, && there == no way to determine how much data, if any, was
        successfully processed by the receiving end of the connection.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        // try {
        return  sock . sendto ( data , address );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        fut = self . create_future ( );
        fd = sock . fileno ( );
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_writer ( fd , self . _sock_sendto , fut , sock , data ,;
        address );
        fut . add_done_callback (;
        functools . partial ( self . _sock_write_done , fd , handle = handle ) );
        return  await fut;
        pub fn _sock_sendto ( &self, fut , sock , data , address )  {
        if fut . done ( ) {
        return;
        // try {
        n = sock . sendto ( data , 0 , address );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( n );
        async def sock_connect ( self , sock , address ) ;
        "Connect to a remote socket at address.

        This method == a coroutine.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        if sock . family == socket . AF_INET || ( {
        base_events . _HAS_IPv6 && sock . family == socket . AF_INET6 ) ;
        resolved = await self . _ensure_resolved (;
        address , family = sock . family , type = sock . type , proto = sock . proto ,;
        loop = self ,;
        );
        _ , _ , _ , _ , address = resolved [ 0 ];
        fut = self . create_future ( );
        self . _sock_connect ( fut , sock , address );
        // try {
        return  await fut;
        // } finally {
        fut = None /* Option */;
        pub fn _sock_connect ( &self, fut , sock , address )  {
        fd = sock . fileno ( );
        // try {
        sock . connect ( address );
        // } catch  ( BlockingIOError , InterruptedError )  {
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_writer (;
        fd , self . _sock_connect_cb , fut , sock , address );
        fut . add_done_callback (;
        functools . partial ( self . _sock_write_done , fd , handle = handle ) );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( None /* Option */ );
        // } finally {
        fut = None /* Option */;
        pub fn _sock_write_done ( &self, fd , fut , handle = None /* Option */ )  {
        if handle is None /* Option */ || !handle . cancelled ( ) {
        self . remove_writer ( fd );
        pub fn _sock_connect_cb ( &self, fut , sock , address )  {
        if fut . done ( ) {
        return;
        // try {
        err = sock . getsockopt ( socket . SOL_SOCKET , socket . SO_ERROR );
        if err != 0 {
        panic!("OSError ( err , f "Connect call failed {address}" )");
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( None /* Option */ );
        // } finally {
        fut = None /* Option */;
        async def sock_accept ( self , sock ) ;
        "Accept a connection.

        The socket must be bound to an address && listening for connections.
        The return value == a pair (conn, address) where conn == a new socket
        object usable to send && receive data on the connection, && address
        == the address bound to the socket on the other end of the connection.
        ";
        base_events . _check_ssl_socket ( sock );
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        fut = self . create_future ( );
        self . _sock_accept ( fut , sock );
        return  await fut;
        pub fn _sock_accept ( &self, fut , sock )  {
        fd = sock . fileno ( );
        // try {
        conn , address = sock . accept ( );
        conn . setblocking ( false );
        // } catch  ( BlockingIOError , InterruptedError )  {
        self . _ensure_fd_no_transport ( fd );
        handle = self . _add_reader ( fd , self . _sock_accept , fut , sock );
        fut . add_done_callback (;
        functools . partial ( self . _sock_read_done , fd , handle = handle ) );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        fut . set_exception ( exc );
        } else {
        fut . set_result ( ( conn , address ) );
        async def _sendfile_native ( self , transp , file , offset , count ) ;
        del self . _transports [ transp . _sock_fd ];
        resume_reading = transp . is_reading ( );
        transp . pause_reading ( );
        await transp . _make_empty_waiter ( );
        // try {
        return  await self . sock_sendfile ( transp . _sock , file , offset , count ,;
        fallback = false );
        // } finally {
        transp . _reset_empty_waiter ( );
        if resume_reading {
        transp . resume_reading ( );
        self . _transports [ transp . _sock_fd ] = transp;
        pub fn _process_events ( &self, event_list )  {
        for key , mask in event_list .iter() {
        fileobj , ( reader , writer ) = key . fileobj , key . data;
        if mask & selectors . EVENT_READ && reader is !None /* Option */ {
        if reader . _cancelled {
        self . _remove_reader ( fileobj );
        } else {
        self . _add_callback ( reader );
        if mask & selectors . EVENT_WRITE && writer is !None /* Option */ {
        if writer . _cancelled {
        self . _remove_writer ( fileobj );
        } else {
        self . _add_callback ( writer );
        pub fn _stop_serving ( &self, sock )  {
        self . _remove_reader ( sock . fileno ( ) );
        sock . close ( );
        class _SelectorTransport ( transports . _FlowControlMixin ,;
        transports . Transport ) ;
        max_size = 256 * 1024;
        _buffer_factory = bytearray;
        _sock = None /* Option */;
        pub fn __init__ ( &self, loop , sock , protocol , extra = None /* Option */ , server = None /* Option */ )  {
        super ( ) . __init__ ( extra , loop );
        self . _extra [ "socket" ] = trsock . TransportSocket ( sock );
        // try {
        self . _extra [ "sockname" ] = sock . getsockname ( );
        // } catch  OSError  {
        self . _extra [ "sockname" ] = None /* Option */;
        if "peername" !in self . _extra {
        // try {
        self . _extra [ "peername" ] = sock . getpeername ( );
        // } catch  socket . error  {
        self . _extra [ "peername" ] = None /* Option */;
        self . _sock = sock;
        self . _sock_fd = sock . fileno ( );
        self . _protocol_connected = false;
        self . set_protocol ( protocol );
        self . _server = server;
        self . _buffer = self . _buffer_factory ( );
        self . _conn_lost = 0;
        self . _closing = false;
        self . _paused = false;
        if self . _server is !None /* Option */ {
        self . _server . _attach ( );
        loop . _transports [ self . _sock_fd ] = self;
        pub fn __repr__ ( self )  {
        info = [ self . __class__ . __name__ ];
        if self . _sock is None /* Option */ {
        info . append ( "closed" );
        } else if self . _closing {
        info . append ( "closing" );
        info . append ( format!("fd={self._sock_fd}" ));
        if self . _loop is !None /* Option */ && !self . _loop . is_closed ( ) {
        polling = _test_selector_event ( self . _loop . _selector ,;
        self . _sock_fd , selectors . EVENT_READ );
        if polling {
        info . append ( "read=polling" );
        } else {
        info . append ( "read=idle" );
        polling = _test_selector_event ( self . _loop . _selector ,;
        self . _sock_fd ,;
        selectors . EVENT_WRITE );
        if polling {
        state = "polling";
        } else {
        state = "idle";
        bufsize = self . get_write_buffer_size ( );
        info . append ( format!("write=<{state}, bufsize={bufsize}>" ));
        return  "<{}>" . format ( " " . join ( info ) );
        pub fn abort ( self )  {
        self . _force_close ( None /* Option */ );
        pub fn set_protocol ( &self, protocol )  {
        self . _protocol = protocol;
        self . _protocol_connected = true;
        pub fn get_protocol ( self )  {
        return  self . _protocol;
        pub fn is_closing ( self )  {
        return  self . _closing;
        pub fn is_reading ( self )  {
        return  !self . is_closing ( ) && !self . _paused;
        pub fn pause_reading ( self )  {
        if !self . is_reading ( ) {
        return;
        self . _paused = true;
        self . _loop . _remove_reader ( self . _sock_fd );
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r pauses reading" , self );
        pub fn resume_reading ( self )  {
        if self . _closing || !self . _paused {
        return;
        self . _paused = false;
        self . _add_reader ( self . _sock_fd , self . _read_ready );
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r resumes reading" , self );
        pub fn close ( self )  {
        if self . _closing {
        return;
        self . _closing = true;
        self . _loop . _remove_reader ( self . _sock_fd );
        if !self . _buffer {
        self . _conn_lost + = 1;
        self . _loop . _remove_writer ( self . _sock_fd );
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if self . _sock is !None /* Option */ {
        _warn ( format!("unclosed transport {self!r}" , ResourceWarning , source = self ));
        self . _sock . close ( );
        pub fn _fatal_error ( &self, exc , message = "Fatal error on transport" )  {
        if isinstance ( exc , OSError ) {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r: %s" , self , message , exc_info = true );
        } else {
        self . _loop . call_exception_handler ( {;
        "message" : message ,;
        "exception" : exc ,;
        "transport" : self ,;
        "protocol" : self . _protocol ,;
        } );
        self . _force_close ( exc );
        pub fn _force_close ( &self, exc )  {
        if self . _conn_lost {
        return;
        if self . _buffer {
        self . _buffer . clear ( );
        self . _loop . _remove_writer ( self . _sock_fd );
        if !self . _closing {
        self . _closing = true;
        self . _loop . _remove_reader ( self . _sock_fd );
        self . _conn_lost + = 1;
        self . _loop . call_soon ( self . _call_connection_lost , exc );
        pub fn _call_connection_lost ( &self, exc )  {
        // try {
        if self . _protocol_connected {
        self . _protocol . connection_lost ( exc );
        // } finally {
        self . _sock . close ( );
        self . _sock = None /* Option */;
        self . _protocol = None /* Option */;
        self . _loop = None /* Option */;
        server = self . _server;
        if server is !None /* Option */ {
        server . _detach ( );
        self . _server = None /* Option */;
        pub fn get_write_buffer_size ( self )  {
        return  len ( self . _buffer );
        pub fn _add_reader ( &self, fd , callback , * args )  {
        if !self . is_reading ( ) {
        return;
        self . _loop . _add_reader ( fd , callback , * args );
        class _SelectorSocketTransport ( _SelectorTransport ) ;
        _start_tls_compatible = true;
        _sendfile_compatible = constants . _SendfileMode . TRY_NATIVE;
        pub fn __init__ ( &self, loop , sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        self . _read_ready_cb = None /* Option */;
        super ( ) . __init__ ( loop , sock , protocol , extra , server );
        self . _eof = false;
        self . _empty_waiter = None /* Option */;
        base_events . _set_nodelay ( self . _sock );
        self . _loop . call_soon ( self . _protocol . connection_made , self );
        self . _loop . call_soon ( self . _add_reader ,;
        self . _sock_fd , self . _read_ready );
        if waiter is !None /* Option */ {
        self . _loop . call_soon ( futures . _set_result_unless_cancelled ,;
        waiter , None /* Option */ );
        pub fn set_protocol ( &self, protocol )  {
        if isinstance ( protocol , protocols . BufferedProtocol ) {
        self . _read_ready_cb = self . _read_ready__get_buffer;
        } else {
        self . _read_ready_cb = self . _read_ready__data_received;
        super ( ) . set_protocol ( protocol );
        pub fn _read_ready ( self )  {
        self . _read_ready_cb ( );
        pub fn _read_ready__get_buffer ( self )  {
        if self . _conn_lost {
        return;
        // try {
        buf = self . _protocol . get_buffer ( -1 );
        if !len ( buf ) {
        panic!("RuntimeError ( "get_buffer() returned an empty buffer" )");
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal error: protocol.get_buffer() call failed." );
        return;
        // try {
        nbytes = self . _sock . recv_into ( buf );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error ( exc , "Fatal read error on socket transport" );
        return;
        if !nbytes {
        self . _read_ready__on_eof ( );
        return;
        // try {
        self . _protocol . buffer_updated ( nbytes );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal error: protocol.buffer_updated() call failed." );
        pub fn _read_ready__data_received ( self )  {
        if self . _conn_lost {
        return;
        // try {
        data = self . _sock . recv ( self . max_size );
        // } catch  ( BlockingIOError , InterruptedError )  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error ( exc , "Fatal read error on socket transport" );
        return;
        if !data {
        self . _read_ready__on_eof ( );
        return;
        // try {
        self . _protocol . data_received ( data );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal error: protocol.data_received() call failed." );
        pub fn _read_ready__on_eof ( self )  {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r received EOF" , self );
        // try {
        keep_open = self . _protocol . eof_received ( );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal error: protocol.eof_received() call failed." );
        return;
        if keep_open {
        self . _loop . _remove_reader ( self . _sock_fd );
        } else {
        self . close ( );
        pub fn write ( &self, data )  {
        if !isinstance ( data , ( bytes , bytearray , memoryview ) ) {
        panic!("TypeError ( f "data argument must be a bytes-like object, "");
        format!("not {type(data).__name__!r}" ));
        if self . _eof {
        panic!("RuntimeError ( "Cannot call write() after write_eof()" )");
        if self . _empty_waiter is !None /* Option */ {
        panic!("RuntimeError ( "unable to write; sendfile is in progress" )");
        if !data {
        return;
        if self . _conn_lost {
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "socket.send() raised exception." );
        self . _conn_lost + = 1;
        return;
        if !self . _buffer {
        // try {
        n = self . _sock . send ( data );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error ( exc , "Fatal write error on socket transport" );
        return;
        } else {
        data = data [ n : ];
        if !data {
        return;
        self . _loop . _add_writer ( self . _sock_fd , self . _write_ready );
        self . _buffer . extend ( data );
        self . _maybe_pause_protocol ( );
        pub fn _write_ready ( self )  {
        assert self . _buffer , "Data should !be empty";
        if self . _conn_lost {
        return;
        // try {
        n = self . _sock . send ( self . _buffer );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _loop . _remove_writer ( self . _sock_fd );
        self . _buffer . clear ( );
        self . _fatal_error ( exc , "Fatal write error on socket transport" );
        if self . _empty_waiter is !None /* Option */ {
        self . _empty_waiter . set_exception ( exc );
        } else {
        if n {
        del self . _buffer [ : n ];
        self . _maybe_resume_protocol ( );
        if !self . _buffer {
        self . _loop . _remove_writer ( self . _sock_fd );
        if self . _empty_waiter is !None /* Option */ {
        self . _empty_waiter . set_result ( None /* Option */ );
        if self . _closing {
        self . _call_connection_lost ( None /* Option */ );
        } else if self . _eof {
        self . _sock . shutdown ( socket . SHUT_WR );
        pub fn write_eof ( self )  {
        if self . _closing || self . _eof {
        return;
        self . _eof = true;
        if !self . _buffer {
        self . _sock . shutdown ( socket . SHUT_WR );
        pub fn can_write_eof ( self )  {
        return  true;
        pub fn _call_connection_lost ( &self, exc )  {
        super ( ) . _call_connection_lost ( exc );
        if self . _empty_waiter is !None /* Option */ {
        self . _empty_waiter . set_exception (;
        ConnectionError ( "Connection == closed by peer" ) );
        pub fn _make_empty_waiter ( self )  {
        if self . _empty_waiter is !None /* Option */ {
        panic!("RuntimeError ( "Empty waiter is already set" )");
        self . _empty_waiter = self . _loop . create_future ( );
        if !self . _buffer {
        self . _empty_waiter . set_result ( None /* Option */ );
        return  self . _empty_waiter;
        pub fn _reset_empty_waiter ( self )  {
        self . _empty_waiter = None /* Option */;
        class _SelectorDatagramTransport ( _SelectorTransport ) ;
        _buffer_factory = collections . deque;
        pub fn __init__ ( &self, loop , sock , protocol , address = None /* Option */ , {
        waiter = None /* Option */ , extra = None /* Option */ ) ;
        super ( ) . __init__ ( loop , sock , protocol , extra );
        self . _address = address;
        self . _buffer_size = 0;
        self . _loop . call_soon ( self . _protocol . connection_made , self );
        self . _loop . call_soon ( self . _add_reader ,;
        self . _sock_fd , self . _read_ready );
        if waiter is !None /* Option */ {
        self . _loop . call_soon ( futures . _set_result_unless_cancelled ,;
        waiter , None /* Option */ );
        pub fn get_write_buffer_size ( self )  {
        return  self . _buffer_size;
        pub fn _read_ready ( self )  {
        if self . _conn_lost {
        return;
        // try {
        data , addr = self . _sock . recvfrom ( self . max_size );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  OSError as exc  {
        self . _protocol . error_received ( exc );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error ( exc , "Fatal read error on datagram transport" );
        } else {
        self . _protocol . datagram_received ( data , addr );
        pub fn sendto ( &self, data , addr = None /* Option */ )  {
        if !isinstance ( data , ( bytes , bytearray , memoryview ) ) {
        panic!("TypeError ( f "data argument must be a bytes-like object, "");
        format!("not {type(data).__name__!r}" ));
        if !data {
        return;
        if self . _address {
        if addr !in ( None /* Option */ , self . _address ) {
        panic!("ValueError (");
        format!("Invalid address: must be None /* Option */ || {self._address}" ));
        addr = self . _address;
        if self . _conn_lost && self . _address {
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "socket.send() raised exception." );
        self . _conn_lost + = 1;
        return;
        if !self . _buffer {
        // try {
        if self . _extra [ "peername" ] {
        self . _sock . send ( data );
        } else {
        self . _sock . sendto ( data , addr );
        return;
        // } catch  ( BlockingIOError , InterruptedError )  {
        self . _loop . _add_writer ( self . _sock_fd , self . _sendto_ready );
        // } catch  OSError as exc  {
        self . _protocol . error_received ( exc );
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal write error on datagram transport" );
        return;
        self . _buffer . append ( ( bytes ( data ) , addr ) );
        self . _buffer_size + = len ( data );
        self . _maybe_pause_protocol ( );
        pub fn _sendto_ready ( self )  {
        while self . _buffer  {
        data , addr = self . _buffer . popleft ( );
        self . _buffer_size - = len ( data );
        // try {
        if self . _extra [ "peername" ] {
        self . _sock . send ( data );
        } else {
        self . _sock . sendto ( data , addr );
        // } catch  ( BlockingIOError , InterruptedError )  {
        self . _buffer . appendleft ( ( data , addr ) );
        self . _buffer_size + = len ( data );
        break;
        // } catch  OSError as exc  {
        self . _protocol . error_received ( exc );
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error (;
        exc , "Fatal write error on datagram transport" );
        return;
        self . _maybe_resume_protocol ( );
        if !self . _buffer {
        self . _loop . _remove_writer ( self . _sock_fd );
        if self . _closing {
        self . _call_connection_lost ( None /* Option */ );
}

