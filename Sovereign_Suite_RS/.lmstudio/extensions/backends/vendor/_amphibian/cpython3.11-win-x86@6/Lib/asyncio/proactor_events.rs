//! proactor_events.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::socket;
// use crate::signal;
// use std::collections;
// use crate::.::{base_events};

pub const __all__: &str = "BaseProactorEventLoop" ,;
pub fn _set_socket_extra(transport: &str, sock: &str) {
        transport . _extra [ "socket" ] = trsock . TransportSocket ( sock );
        // try {
        transport . _extra [ "sockname" ] = sock . getsockname ( );
        // } catch  socket . error  {
        if transport . _loop . get_debug ( ) {
        logger . warning (;
        "getsockname() failed on %r" , sock , exc_info = true );
        if "peername" !in transport . _extra {
        // try {
        transport . _extra [ "peername" ] = sock . getpeername ( );
        // } catch  socket . error  {
        transport . _extra [ "peername" ] = None /* Option */;
        class _ProactorBasePipeTransport ( transports . _FlowControlMixin ,;
        transports . BaseTransport ) ;
        "Base class for pipe && socket transports.";
        pub fn __init__ ( &self, loop , sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        super ( ) . __init__ ( extra , loop );
        self . _set_extra ( sock );
        self . _sock = sock;
        self . set_protocol ( protocol );
        self . _server = server;
        self . _buffer = None /* Option */;
        self . _read_fut = None /* Option */;
        self . _write_fut = None /* Option */;
        self . _pending_write = 0;
        self . _conn_lost = 0;
        self . _closing = false;
        self . _called_connection_lost = false;
        self . _eof_written = false;
        if self . _server is !None /* Option */ {
        self . _server . _attach ( );
        self . _loop . call_soon ( self . _protocol . connection_made , self );
        if waiter is !None /* Option */ {
        self . _loop . call_soon ( futures . _set_result_unless_cancelled ,;
        waiter , None /* Option */ );
        pub fn __repr__ ( self )  {
        info = [ self . __class__ . __name__ ];
        if self . _sock is None /* Option */ {
        info . append ( "closed" );
        } else if self . _closing {
        info . append ( "closing" );
        if self . _sock is !None /* Option */ {
        info . append ( format!("fd={self._sock.fileno()}" ));
        if self . _read_fut is !None /* Option */ {
        info . append ( format!("read={self._read_fut!r}" ));
        if self . _write_fut is !None /* Option */ {
        info . append ( format!("write={self._write_fut!r}" ));
        if self . _buffer {
        info . append ( format!("write_bufsize={len(self._buffer)}" ));
        if self . _eof_written {
        info . append ( "EOF written" );
        return  "<{}>" . format ( " " . join ( info ) );
        pub fn _set_extra ( &self, sock )  {
        self . _extra [ "pipe" ] = sock;
        pub fn set_protocol ( &self, protocol )  {
        self . _protocol = protocol;
        pub fn get_protocol ( self )  {
        return  self . _protocol;
        pub fn is_closing ( self )  {
        return  self . _closing;
        pub fn close ( self )  {
        if self . _closing {
        return;
        self . _closing = true;
        self . _conn_lost + = 1;
        if !self . _buffer && self . _write_fut is None /* Option */ {
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        if self . _read_fut is !None /* Option */ {
        self . _read_fut . cancel ( );
        self . _read_fut = None /* Option */;
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if self . _sock is !None /* Option */ {
        _warn ( format!("unclosed transport {self!r}" , ResourceWarning , source = self ));
        self . _sock . close ( );
        pub fn _fatal_error ( &self, exc , message = "Fatal error on pipe transport" )  {
        // try {
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
        // } finally {
        self . _force_close ( exc );
        pub fn _force_close ( &self, exc )  {
        if self . _empty_waiter is !None /* Option */ && !self . _empty_waiter . done ( ) {
        if exc is None /* Option */ {
        self . _empty_waiter . set_result ( None /* Option */ );
        } else {
        self . _empty_waiter . set_exception ( exc );
        if self . _closing && self . _called_connection_lost {
        return;
        self . _closing = true;
        self . _conn_lost + = 1;
        if self . _write_fut {
        self . _write_fut . cancel ( );
        self . _write_fut = None /* Option */;
        if self . _read_fut {
        self . _read_fut . cancel ( );
        self . _read_fut = None /* Option */;
        self . _pending_write = 0;
        self . _buffer = None /* Option */;
        self . _loop . call_soon ( self . _call_connection_lost , exc );
        pub fn _call_connection_lost ( &self, exc )  {
        if self . _called_connection_lost {
        return;
        // try {
        self . _protocol . connection_lost ( exc );
        // } finally {
        if hasattr ( self . _sock , "shutdown" ) && self . _sock . fileno ( ) != -1 {
        self . _sock . shutdown ( socket . SHUT_RDWR );
        self . _sock . close ( );
        self . _sock = None /* Option */;
        server = self . _server;
        if server is !None /* Option */ {
        server . _detach ( );
        self . _server = None /* Option */;
        self . _called_connection_lost = true;
        pub fn get_write_buffer_size ( self )  {
        size = self . _pending_write;
        if self . _buffer is !None /* Option */ {
        size + = len ( self . _buffer );
        return  size;
        class _ProactorReadPipeTransport ( _ProactorBasePipeTransport ,;
        transports . ReadTransport ) ;
        "Transport for read pipes.";
        pub fn __init__ ( &self, loop , sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ , server = None /* Option */ , buffer_size = 65536 ) ;
        self . _pending_data_length = -1;
        self . _paused = true;
        super ( ) . __init__ ( loop , sock , protocol , waiter , extra , server );
        self . _data = bytearray ( buffer_size );
        self . _loop . call_soon ( self . _loop_reading );
        self . _paused = false;
        pub fn is_reading ( self )  {
        return  !self . _paused && !self . _closing;
        pub fn pause_reading ( self )  {
        if self . _closing || self . _paused {
        return;
        self . _paused = true;
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r pauses reading" , self );
        pub fn resume_reading ( self )  {
        if self . _closing || !self . _paused {
        return;
        self . _paused = false;
        if self . _read_fut is None /* Option */ {
        self . _loop . call_soon ( self . _loop_reading , None /* Option */ );
        length = self . _pending_data_length;
        self . _pending_data_length = -1;
        if length > -1 {
        self . _loop . call_soon ( self . _data_received , self . _data [ : length ] , length );
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r resumes reading" , self );
        pub fn _eof_received ( self )  {
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
        if !keep_open {
        self . close ( );
        pub fn _data_received ( &self, data , length )  {
        if self . _paused {
        assert self . _pending_data_length == -1;
        self . _pending_data_length = length;
        return;
        if length == 0 {
        self . _eof_received ( );
        return;
        if isinstance ( self . _protocol , protocols . BufferedProtocol ) {
        // try {
        protocols . _feed_data_to_buffered_proto ( self . _protocol , data );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _fatal_error ( exc ,;
        "Fatal error: protocol.buffer_updated() ";
        "call failed." );
        return;
        } else {
        self . _protocol . data_received ( data );
        pub fn _loop_reading ( &self, fut = None /* Option */ )  {
        length = -1;
        data = None /* Option */;
        // try {
        if fut is !None /* Option */ {
        assert self . _read_fut == fut || ( self . _read_fut == None /* Option */ and;
        self . _closing );
        self . _read_fut = None /* Option */;
        if fut . done ( ) {
        length = fut . result ( );
        if length == 0 {
        return;
        data = self . _data [ : length ];
        } else {
        fut . cancel ( );
        if self . _closing {
        return;
        if !self . _paused {
        self . _read_fut = self . _loop . _proactor . recv_into ( self . _sock , self . _data );
        // } catch  ConnectionAbortedError as exc  {
        if !self . _closing {
        self . _fatal_error ( exc , "Fatal read error on pipe transport" );
        } else if self . _loop . get_debug ( ) {
        logger . debug ( "Read error on pipe transport while closing" ,;
        exc_info = true );
        // } catch  ConnectionResetError as exc  {
        self . _force_close ( exc );
        // } catch  OSError as exc  {
        self . _fatal_error ( exc , "Fatal read error on pipe transport" );
        // } catch  exceptions . CancelledError  {
        if !self . _closing {
        panic!("");
        } else {
        if !self . _paused {
        self . _read_fut . add_done_callback ( self . _loop_reading );
        // } finally {
        if length > -1 {
        self . _data_received ( data , length );
        class _ProactorBaseWritePipeTransport ( _ProactorBasePipeTransport ,;
        transports . WriteTransport ) ;
        "Transport for write pipes.";
        _start_tls_compatible = true;
        pub fn __init__ ( &self, * args , ** kw )  {
        super ( ) . __init__ ( * args , ** kw );
        self . _empty_waiter = None /* Option */;
        pub fn write ( &self, data )  {
        if !isinstance ( data , ( bytes , bytearray , memoryview ) ) {
        panic!("TypeError (");
        format!("data argument must be a bytes-like object, ");
        format!("not {type(data).__name__}" ));
        if self . _eof_written {
        panic!("RuntimeError ( "write_eof() already called" )");
        if self . _empty_waiter is !None /* Option */ {
        panic!("RuntimeError ( "unable to write; sendfile is in progress" )");
        if !data {
        return;
        if self . _conn_lost {
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "socket.send() raised exception." );
        self . _conn_lost + = 1;
        return;
        if self . _write_fut is None /* Option */ {
        assert self . _buffer == None /* Option */;
        self . _loop_writing ( data = bytes ( data ) );
        } else if !self . _buffer {
        self . _buffer = bytearray ( data );
        self . _maybe_pause_protocol ( );
        } else {
        self . _buffer . extend ( data );
        self . _maybe_pause_protocol ( );
        pub fn _loop_writing ( &self, f = None /* Option */ , data = None /* Option */ )  {
        // try {
        if f is !None /* Option */ && self . _write_fut is None /* Option */ && self . _closing {
        return;
        assert f == self . _write_fut;
        self . _write_fut = None /* Option */;
        self . _pending_write = 0;
        if f {
        f . result ( );
        if data is None /* Option */ {
        data = self . _buffer;
        self . _buffer = None /* Option */;
        if !data {
        if self . _closing {
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        if self . _eof_written {
        self . _sock . shutdown ( socket . SHUT_WR );
        self . _maybe_resume_protocol ( );
        } else {
        self . _write_fut = self . _loop . _proactor . send ( self . _sock , data );
        if !self . _write_fut . done ( ) {
        assert self . _pending_write == 0;
        self . _pending_write = len ( data );
        self . _write_fut . add_done_callback ( self . _loop_writing );
        self . _maybe_pause_protocol ( );
        } else {
        self . _write_fut . add_done_callback ( self . _loop_writing );
        if self . _empty_waiter is !None /* Option */ && self . _write_fut is None /* Option */ {
        self . _empty_waiter . set_result ( None /* Option */ );
        // } catch  ConnectionResetError as exc  {
        self . _force_close ( exc );
        // } catch  OSError as exc  {
        self . _fatal_error ( exc , "Fatal write error on pipe transport" );
        pub fn can_write_eof ( self )  {
        return  true;
        pub fn write_eof ( self )  {
        self . close ( );
        pub fn abort ( self )  {
        self . _force_close ( None /* Option */ );
        pub fn _make_empty_waiter ( self )  {
        if self . _empty_waiter is !None /* Option */ {
        panic!("RuntimeError ( "Empty waiter is already set" )");
        self . _empty_waiter = self . _loop . create_future ( );
        if self . _write_fut is None /* Option */ {
        self . _empty_waiter . set_result ( None /* Option */ );
        return  self . _empty_waiter;
        pub fn _reset_empty_waiter ( self )  {
        self . _empty_waiter = None /* Option */;
        class _ProactorWritePipeTransport ( _ProactorBaseWritePipeTransport ) ;
        pub fn __init__ ( &self, * args , ** kw )  {
        super ( ) . __init__ ( * args , ** kw );
        self . _read_fut = self . _loop . _proactor . recv ( self . _sock , 16 );
        self . _read_fut . add_done_callback ( self . _pipe_closed );
        pub fn _pipe_closed ( &self, fut )  {
        if fut . cancelled ( ) {
        return;
        assert fut . result ( ) == b "";
        if self . _closing {
        assert self . _read_fut == None /* Option */;
        return;
        assert fut == self . _read_fut , ( fut , self . _read_fut );
        self . _read_fut = None /* Option */;
        if self . _write_fut is !None /* Option */ {
        self . _force_close ( BrokenPipeError ( ) );
        } else {
        self . close ( );
        class _ProactorDatagramTransport ( _ProactorBasePipeTransport ,;
        transports . DatagramTransport ) ;
        max_size = 256 * 1024;
        pub fn __init__ ( &self, loop , sock , protocol , address = None /* Option */ , {
        waiter = None /* Option */ , extra = None /* Option */ ) ;
        self . _address = address;
        self . _empty_waiter = None /* Option */;
        self . _buffer_size = 0;
        super ( ) . __init__ ( loop , sock , protocol , waiter = waiter , extra = extra );
        self . _buffer = collections . deque ( );
        self . _loop . call_soon ( self . _loop_reading );
        pub fn _set_extra ( &self, sock )  {
        _set_socket_extra ( self , sock );
        pub fn get_write_buffer_size ( self )  {
        return  self . _buffer_size;
        pub fn abort ( self )  {
        self . _force_close ( None /* Option */ );
        pub fn sendto ( &self, data , addr = None /* Option */ )  {
        if !isinstance ( data , ( bytes , bytearray , memoryview ) ) {
        panic!("TypeError ( "data argument must be bytes-like object (%r)" ,");
        type ( data ) );
        if !data {
        return;
        if self . _address is !None /* Option */ && addr !in ( None /* Option */ , self . _address ) {
        panic!("ValueError (");
        format!("Invalid address: must be None /* Option */ || {self._address}" ));
        if self . _conn_lost && self . _address {
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "socket.sendto() raised exception." );
        self . _conn_lost + = 1;
        return;
        self . _buffer . append ( ( bytes ( data ) , addr ) );
        self . _buffer_size + = len ( data );
        if self . _write_fut is None /* Option */ {
        self . _loop_writing ( );
        self . _maybe_pause_protocol ( );
        pub fn _loop_writing ( &self, fut = None /* Option */ )  {
        // try {
        if self . _conn_lost {
        return;
        assert fut == self . _write_fut;
        self . _write_fut = None /* Option */;
        if fut {
        fut . result ( );
        if !self . _buffer || ( self . _conn_lost && self . _address ) {
        if self . _closing {
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        return;
        data , addr = self . _buffer . popleft ( );
        self . _buffer_size - = len ( data );
        if self . _address is !None /* Option */ {
        self . _write_fut = self . _loop . _proactor . send ( self . _sock ,;
        data );
        } else {
        self . _write_fut = self . _loop . _proactor . sendto ( self . _sock ,;
        data ,;
        addr = addr );
        // } catch  OSError as exc  {
        self . _protocol . error_received ( exc );
        // } catch  Exception as exc  {
        self . _fatal_error ( exc , "Fatal write error on datagram transport" );
        } else {
        self . _write_fut . add_done_callback ( self . _loop_writing );
        self . _maybe_resume_protocol ( );
        pub fn _loop_reading ( &self, fut = None /* Option */ )  {
        data = None /* Option */;
        // try {
        if self . _conn_lost {
        return;
        assert self . _read_fut == fut || ( self . _read_fut == None /* Option */ and;
        self . _closing );
        self . _read_fut = None /* Option */;
        if fut is !None /* Option */ {
        res = fut . result ( );
        if self . _closing {
        data = None /* Option */;
        return;
        if self . _address is !None /* Option */ {
        data , addr = res , self . _address;
        } else {
        data , addr = res;
        if self . _conn_lost {
        return;
        if self . _address is !None /* Option */ {
        self . _read_fut = self . _loop . _proactor . recv ( self . _sock ,;
        self . max_size );
        } else {
        self . _read_fut = self . _loop . _proactor . recvfrom ( self . _sock ,;
        self . max_size );
        // } catch  OSError as exc  {
        self . _protocol . error_received ( exc );
        // } catch  exceptions . CancelledError  {
        if !self . _closing {
        panic!("");
        } else {
        if self . _read_fut is !None /* Option */ {
        self . _read_fut . add_done_callback ( self . _loop_reading );
        // } finally {
        if data {
        self . _protocol . datagram_received ( data , addr );
        class _ProactorDuplexPipeTransport ( _ProactorReadPipeTransport ,;
        _ProactorBaseWritePipeTransport ,;
        transports . Transport ) ;
        "Transport for duplex pipes.";
        pub fn can_write_eof ( self )  {
        return  false;
        pub fn write_eof ( self )  {
        panic!("NotImplementedError");
        class _ProactorSocketTransport ( _ProactorReadPipeTransport ,;
        _ProactorBaseWritePipeTransport ,;
        transports . Transport ) ;
        "Transport for connected sockets.";
        _sendfile_compatible = constants . _SendfileMode . TRY_NATIVE;
        pub fn __init__ ( &self, loop , sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        super ( ) . __init__ ( loop , sock , protocol , waiter , extra , server );
        base_events . _set_nodelay ( sock );
        pub fn _set_extra ( &self, sock )  {
        _set_socket_extra ( self , sock );
        pub fn can_write_eof ( self )  {
        return  true;
        pub fn write_eof ( self )  {
        if self . _closing || self . _eof_written {
        return;
        self . _eof_written = true;
        if self . _write_fut is None /* Option */ {
        self . _sock . shutdown ( socket . SHUT_WR );
        class BaseProactorEventLoop ( base_events . BaseEventLoop ) ;
        pub fn __init__ ( &self, proactor )  {
        super ( ) . __init__ ( );
        logger . debug ( "Using proactor: %s" , proactor . __class__ . __name__ );
        self . _proactor = proactor;
        self . _selector = proactor;
        self . _self_reading_future = None /* Option */;
        self . _accept_futures = { };
        proactor . set_loop ( self );
        self . _make_self_pipe ( );
        if threading . current_thread ( ) is threading . main_thread ( ) {
        signal . set_wakeup_fd ( self . _csock . fileno ( ) );
        pub fn _make_socket_transport ( &self, sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        return  _ProactorSocketTransport ( self , sock , protocol , waiter ,;
        extra , server );
        pub fn _make_ssl_transport ( {
        self , rawsock , protocol , sslcontext , waiter = None /* Option */ ,;
        * , server_side = false , server_hostname = None /* Option */ ,;
        extra = None /* Option */ , server = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        ssl_protocol = sslproto . SSLProtocol (;
        self , protocol , sslcontext , waiter ,;
        server_side , server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        _ProactorSocketTransport ( self , rawsock , ssl_protocol ,;
        extra = extra , server = server );
        return  ssl_protocol . _app_transport;
        pub fn _make_datagram_transport ( &self, sock , protocol , {
        address = None /* Option */ , waiter = None /* Option */ , extra = None /* Option */ ) ;
        return  _ProactorDatagramTransport ( self , sock , protocol , address ,;
        waiter , extra );
        pub fn _make_duplex_pipe_transport ( &self, sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        return  _ProactorDuplexPipeTransport ( self ,;
        sock , protocol , waiter , extra );
        pub fn _make_read_pipe_transport ( &self, sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        return  _ProactorReadPipeTransport ( self , sock , protocol , waiter , extra );
        pub fn _make_write_pipe_transport ( &self, sock , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        return  _ProactorWritePipeTransport ( self ,;
        sock , protocol , waiter , extra );
        pub fn close ( self )  {
        if self . is_running ( ) {
        panic!("RuntimeError ( "Cannot close a running event loop" )");
        if self . is_closed ( ) {
        return;
        if threading . current_thread ( ) is threading . main_thread ( ) {
        signal . set_wakeup_fd ( -1 );
        self . _stop_accept_futures ( );
        self . _close_self_pipe ( );
        self . _proactor . close ( );
        self . _proactor = None /* Option */;
        self . _selector = None /* Option */;
        super ( ) . close ( );
        async def sock_recv ( self , sock , n ) ;
        return  await self . _proactor . recv ( sock , n );
        async def sock_recv_into ( self , sock , buf ) ;
        return  await self . _proactor . recv_into ( sock , buf );
        async def sock_recvfrom ( self , sock , bufsize ) ;
        return  await self . _proactor . recvfrom ( sock , bufsize );
        async def sock_recvfrom_into ( self , sock , buf , nbytes = 0 ) ;
        if !nbytes {
        nbytes = len ( buf );
        return  await self . _proactor . recvfrom_into ( sock , buf , nbytes );
        async def sock_sendall ( self , sock , data ) ;
        return  await self . _proactor . send ( sock , data );
        async def sock_sendto ( self , sock , data , address ) ;
        return  await self . _proactor . sendto ( sock , data , 0 , address );
        async def sock_connect ( self , sock , address ) ;
        return  await self . _proactor . connect ( sock , address );
        async def sock_accept ( self , sock ) ;
        return  await self . _proactor . accept ( sock );
        async def _sock_sendfile_native ( self , sock , file , offset , count ) ;
        // try {
        fileno = file . fileno ( );
        // } catch  ( AttributeError , io . UnsupportedOperation ) as err  {
        panic!("exceptions . SendfileNotAvailableError ( "not a regular file" )");
        // try {
        fsize = os . fstat ( fileno ) . st_size;
        // } catch  OSError  {
        panic!("exceptions . SendfileNotAvailableError ( "not a regular file" )");
        blocksize = count if count else fsize;
        if !blocksize {
        return  0;
        blocksize = min ( blocksize , 0x ffff_ffff );
        end_pos = min ( offset + count , fsize ) if count else fsize;
        offset = min ( offset , fsize );
        total_sent = 0;
        // try {
        while true  {
        blocksize = min ( end_pos - offset , blocksize );
        if blocksize <= 0 {
        return  total_sent;
        await self . _proactor . sendfile ( sock , file , offset , blocksize );
        offset + = blocksize;
        total_sent + = blocksize;
        // } finally {
        if total_sent > 0 {
        file . seek ( offset );
        async def _sendfile_native ( self , transp , file , offset , count ) ;
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
        pub fn _close_self_pipe ( self )  {
        if self . _self_reading_future is !None /* Option */ {
        self . _self_reading_future . cancel ( );
        self . _self_reading_future = None /* Option */;
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
        pub fn _loop_self_reading ( &self, f = None /* Option */ )  {
        // try {
        if f is !None /* Option */ {
        f . result ( );
        if self . _self_reading_future is !f {
        return;
        f = self . _proactor . recv ( self . _ssock , 4096 );
        // } catch  exceptions . CancelledError  {
        return;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . call_exception_handler ( {;
        "message" : "Error on reading from the event loop self pipe" ,;
        "exception" : exc ,;
        "loop" : self ,;
        } );
        } else {
        self . _self_reading_future = f;
        f . add_done_callback ( self . _loop_self_reading );
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
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        pub fn loop ( f = None /* Option */ )  {
        // try {
        if f is !None /* Option */ {
        conn , addr = f . result ( );
        if self . _debug {
        logger . debug ( "%r got a new connection from %r: %r" ,;
        server , addr , conn );
        protocol = protocol_factory ( );
        if sslcontext is !None /* Option */ {
        self . _make_ssl_transport (;
        conn , protocol , sslcontext , server_side = true ,;
        extra = { "peername" : addr } , server = server ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        } else {
        self . _make_socket_transport (;
        conn , protocol ,;
        extra = { "peername" : addr } , server = server );
        if self . is_closed ( ) {
        return;
        f = self . _proactor . accept ( sock );
        // } catch  OSError as exc  {
        if sock . fileno ( ) != -1 {
        self . call_exception_handler ( {;
        "message" : "Accept failed on a socket" ,;
        "exception" : exc ,;
        "socket" : trsock . TransportSocket ( sock ) ,;
        } );
        sock . close ( );
        } else if self . _debug {
        logger . debug ( "Accept failed on socket %r" ,;
        sock , exc_info = true );
        // } catch  exceptions . CancelledError  {
        sock . close ( );
        } else {
        self . _accept_futures [ sock . fileno ( ) ] = f;
        f . add_done_callback ( loop );
        self . call_soon ( loop );
        pub fn _process_events ( &self, event_list )  {
        // pass
        pub fn _stop_accept_futures ( self )  {
        for future in self . _accept_futures . values ( ) .iter() {
        future . cancel ( );
        self . _accept_futures . clear ( );
        pub fn _stop_serving ( &self, sock )  {
        future = self . _accept_futures . pop ( sock . fileno ( ) , None /* Option */ );
        if future {
        future . cancel ( );
        self . _proactor . _stop_serving ( sock );
        sock . close ( );
}

