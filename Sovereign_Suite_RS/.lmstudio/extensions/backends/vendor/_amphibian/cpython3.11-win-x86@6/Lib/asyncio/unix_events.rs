//! unix_events.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::errno;
// use crate::itertools;
// use crate::selectors;
// use crate::socket;
// use crate::subprocess;
// use std::thread;
// use crate::.::{base_events};

pub const __all__: f64 = (;
pub fn _sighandler_noop(signum: &str, frame: &str) {
        "Dummy signal handler.";
        // pass
        pub fn waitstatus_to_exitcode ( status )  {
        // try {
        return  os . waitstatus_to_exitcode ( status );
        // } catch  ValueError  {
        return  status;
        class _UnixSelectorEventLoop ( selector_events . BaseSelectorEventLoop ) ;
        "Unix event loop.

    Adds signal handling && UNIX Domain Socket support to SelectorEventLoop.
    ";
        pub fn __init__ ( &self, selector = None /* Option */ )  {
        super ( ) . __init__ ( selector );
        self . _signal_handlers = { };
        pub fn close ( self )  {
        super ( ) . close ( );
        if !sys . is_finalizing ( ) {
        for sig in list ( self . _signal_handlers ) .iter() {
        self . remove_signal_handler ( sig );
        } else {
        if self . _signal_handlers {
        warnings . warn ( format!("Closing the loop {self!r} ");
        format!("on interpreter shutdown ");
        format!("stage, skipping signal handlers removal" ,);
        ResourceWarning ,;
        source = self );
        self . _signal_handlers . clear ( );
        pub fn _process_self_data ( &self, data )  {
        for signum in data .iter() {
        if !signum {
        continue;
        self . _handle_signal ( signum );
        pub fn add_signal_handler ( &self, sig , callback , * args )  {
        "Add a handler for a signal.  UNIX only.

        Raise ValueError if the signal number == invalid || uncatchable.
        Raise RuntimeError if there == a problem setting up the handler.
        ";
        if ( coroutines . iscoroutine ( callback ) or {
        coroutines . iscoroutinefunction ( callback ) ) ;
        panic!("TypeError ( "coroutines cannot be used "");
        "with add_signal_handler()" );
        self . _check_signal ( sig );
        self . _check_closed ( );
        // try {
        signal . set_wakeup_fd ( self . _csock . fileno ( ) );
        // } catch  ( ValueError , OSError ) as exc  {
        panic!("RuntimeError ( str ( exc ) )");
        handle = events . Handle ( callback , args , self , None /* Option */ );
        self . _signal_handlers [ sig ] = handle;
        // try {
        signal . signal ( sig , _sighandler_noop );
        signal . siginterrupt ( sig , false );
        // } catch  OSError as exc  {
        del self . _signal_handlers [ sig ];
        if !self . _signal_handlers {
        // try {
        signal . set_wakeup_fd ( -1 );
        // } catch  ( ValueError , OSError ) as nexc  {
        logger . info ( "set_wakeup_fd(-1) failed: %s" , nexc );
        if exc . errno == errno . EINVAL {
        panic!("RuntimeError ( f "sig {sig} cannot be caught" )");
        } else {
        panic!("");
        pub fn _handle_signal ( &self, sig )  {
        "Internal helper that == the actual signal handler.";
        handle = self . _signal_handlers . get ( sig );
        if handle is None /* Option */ {
        return;
        if handle . _cancelled {
        self . remove_signal_handler ( sig );
        } else {
        self . _add_callback_signalsafe ( handle );
        pub fn remove_signal_handler ( &self, sig )  {
        "Remove a handler for a signal.  UNIX only.

        Return true if a signal handler was removed, false if not.
        ";
        self . _check_signal ( sig );
        // try {
        del self . _signal_handlers [ sig ];
        // } catch  KeyError  {
        return  false;
        if sig == signal . SIGINT {
        handler = signal . default_int_handler;
        } else {
        handler = signal . SIG_DFL;
        // try {
        signal . signal ( sig , handler );
        // } catch  OSError as exc  {
        if exc . errno == errno . EINVAL {
        panic!("RuntimeError ( f "sig {sig} cannot be caught" )");
        } else {
        panic!("");
        if !self . _signal_handlers {
        // try {
        signal . set_wakeup_fd ( -1 );
        // } catch  ( ValueError , OSError ) as exc  {
        logger . info ( "set_wakeup_fd(-1) failed: %s" , exc );
        return  true;
        pub fn _check_signal ( &self, sig )  {
        "Internal helper to validate a signal.

        Raise ValueError if the signal number == invalid || uncatchable.
        Raise RuntimeError if there == a problem setting up the handler.
        ";
        if !isinstance ( sig , int ) {
        panic!("TypeError ( f "sig must be an int, !{sig!r}" )");
        if sig !in signal . valid_signals ( ) {
        panic!("ValueError ( f "invalid signal number {sig}" )");
        pub fn _make_read_pipe_transport ( &self, pipe , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        return  _UnixReadPipeTransport ( self , pipe , protocol , waiter , extra );
        pub fn _make_write_pipe_transport ( &self, pipe , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        return  _UnixWritePipeTransport ( self , pipe , protocol , waiter , extra );
        async def _make_subprocess_transport ( self , protocol , args , shell ,;
        stdin , stdout , stderr , bufsize ,;
        extra = None /* Option */ , ** kwargs ) ;
        // with scope: events . get_child_watcher ( ) as watcher  {
        if !watcher . is_active ( ) {
        panic!("RuntimeError ( "asyncio.get_child_watcher() is !activated, "");
        "subprocess support == !installed." );
        waiter = self . create_future ( );
        transp = _UnixSubprocessTransport ( self , protocol , args , shell ,;
        stdin , stdout , stderr , bufsize ,;
        waiter = waiter , extra = extra ,;
        ** kwargs );
        watcher . add_child_handler ( transp . get_pid ( ) ,;
        self . _child_watcher_callback , transp );
        // try {
        await waiter;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException  {
        transp . close ( );
        await transp . _wait ( );
        panic!("");
        return  transp;
        pub fn _child_watcher_callback ( &self, pid , returncode , transp )  {
        self . call_soon_threadsafe ( self . call_soon , transp . _process_exited , returncode );
        async def create_unix_connection (;
        self , protocol_factory , path = None /* Option */ , * ,;
        ssl = None /* Option */ , sock = None /* Option */ ,;
        server_hostname = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        assert server_hostname == None /* Option */ || isinstance ( server_hostname , str );
        if ssl {
        if server_hostname is None /* Option */ {
        panic!("ValueError (");
        "you have to pass server_hostname when using ssl" );
        } else {
        if server_hostname is !None /* Option */ {
        panic!("ValueError ( "server_hostname is only meaningful with ssl" )");
        if ssl_handshake_timeout is !None /* Option */ {
        panic!("ValueError (");
        "ssl_handshake_timeout == only meaningful with ssl" );
        if ssl_shutdown_timeout is !None /* Option */ {
        panic!("ValueError (");
        "ssl_shutdown_timeout == only meaningful with ssl" );
        if path is !None /* Option */ {
        if sock is !None /* Option */ {
        panic!("ValueError (");
        "path && sock can !be specified at the same time" );
        path = os . fspath ( path );
        sock = socket . socket ( socket . AF_UNIX , socket . SOCK_STREAM , 0 );
        // try {
        sock . setblocking ( false );
        await self . sock_connect ( sock , path );
        // } catch   {
        sock . close ( );
        panic!("");
        } else {
        if sock is None /* Option */ {
        panic!("ValueError ( "no path && sock were specified" )");
        if ( sock . family != socket . AF_UNIX or {
        sock . type != socket . SOCK_STREAM ) ;
        panic!("ValueError (");
        format!("A UNIX Domain Stream Socket was expected, got {sock!r}" ));
        sock . setblocking ( false );
        transport , protocol = await self . _create_connection_transport (;
        sock , protocol_factory , ssl , server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        return  transport , protocol;
        async def create_unix_server (;
        self , protocol_factory , path = None /* Option */ , * ,;
        sock = None /* Option */ , backlog = 100 , ssl = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ,;
        start_serving = true ) ;
        if isinstance ( ssl , bool ) {
        panic!("TypeError ( "ssl argument must be an SSLContext || None /* Option */" )");
        if ssl_handshake_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_handshake_timeout == only meaningful with ssl" );
        if ssl_shutdown_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_shutdown_timeout == only meaningful with ssl" );
        if path is !None /* Option */ {
        if sock is !None /* Option */ {
        panic!("ValueError (");
        "path && sock can !be specified at the same time" );
        path = os . fspath ( path );
        sock = socket . socket ( socket . AF_UNIX , socket . SOCK_STREAM );
        if path [ 0 ] !in ( 0 , "\x00" ) {
        // try {
        if stat . S_ISSOCK ( os . stat ( path ) . st_mode ) {
        os . remove ( path );
        // } catch  FileNotFoundError  {
        // pass
        // } catch  OSError as err  {
        logger . error ( "Unable to check || remove stale UNIX socket ";
        "%r: %r" , path , err );
        // try {
        sock . bind ( path );
        // } catch  OSError as exc  {
        sock . close ( );
        if exc . errno == errno . EADDRINUSE {
        msg = format!("Address {path!r} == already in use");
        panic!("OSError ( errno . EADDRINUSE , msg ) from None /* Option */");
        } else {
        panic!("");
        // } catch   {
        sock . close ( );
        panic!("");
        } else {
        if sock is None /* Option */ {
        panic!("ValueError (");
        "path was !specified, && no sock specified" );
        if ( sock . family != socket . AF_UNIX or {
        sock . type != socket . SOCK_STREAM ) ;
        panic!("ValueError (");
        format!("A UNIX Domain Stream Socket was expected, got {sock!r}" ));
        sock . setblocking ( false );
        server = base_events . Server ( self , [ sock ] , protocol_factory ,;
        ssl , backlog , ssl_handshake_timeout ,;
        ssl_shutdown_timeout );
        if start_serving {
        server . _start_serving ( );
        await tasks . sleep ( 0 );
        return  server;
        async def _sock_sendfile_native ( self , sock , file , offset , count ) ;
        // try {
        os . sendfile;
        // } catch  AttributeError  {
        panic!("exceptions . SendfileNotAvailableError (");
        "os.sendfile() == !available" );
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
        fut = self . create_future ( );
        self . _sock_sendfile_native_impl ( fut , None /* Option */ , sock , fileno ,;
        offset , count , blocksize , 0 );
        return  await fut;
        pub fn _sock_sendfile_native_impl ( &self, fut , registered_fd , sock , fileno , {
        offset , count , blocksize , total_sent ) ;
        fd = sock . fileno ( );
        if registered_fd is !None /* Option */ {
        self . remove_writer ( registered_fd );
        if fut . cancelled ( ) {
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        return;
        if count {
        blocksize = count - total_sent;
        if blocksize <= 0 {
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        fut . set_result ( total_sent );
        return;
        // try {
        sent = os . sendfile ( fd , fileno , offset , blocksize );
        // } catch  ( BlockingIOError , InterruptedError )  {
        if registered_fd is None /* Option */ {
        self . _sock_add_cancellation_callback ( fut , sock );
        self . add_writer ( fd , self . _sock_sendfile_native_impl , fut ,;
        fd , sock , fileno ,;
        offset , count , blocksize , total_sent );
        // } catch  OSError as exc  {
        if ( registered_fd is !None /* Option */ and {
        exc . errno == errno . ENOTCONN and;
        type ( exc ) == !ConnectionError ) ;
        new_exc = ConnectionError (;
        "socket == !connected" , errno . ENOTCONN );
        new_exc . __cause__ = exc;
        exc = new_exc;
        if total_sent == 0 {
        err = exceptions . SendfileNotAvailableError (;
        "os.sendfile call failed" );
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        fut . set_exception ( err );
        } else {
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        fut . set_exception ( exc );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        fut . set_exception ( exc );
        } else {
        if sent == 0 {
        self . _sock_sendfile_update_filepos ( fileno , offset , total_sent );
        fut . set_result ( total_sent );
        } else {
        offset + = sent;
        total_sent + = sent;
        if registered_fd is None /* Option */ {
        self . _sock_add_cancellation_callback ( fut , sock );
        self . add_writer ( fd , self . _sock_sendfile_native_impl , fut ,;
        fd , sock , fileno ,;
        offset , count , blocksize , total_sent );
        pub fn _sock_sendfile_update_filepos ( &self, fileno , offset , total_sent )  {
        if total_sent > 0 {
        os . lseek ( fileno , offset , os . SEEK_SET );
        pub fn _sock_add_cancellation_callback ( &self, fut , sock )  {
        pub fn cb ( fut )  {
        if fut . cancelled ( ) {
        fd = sock . fileno ( );
        if fd != -1 {
        self . remove_writer ( fd );
        fut . add_done_callback ( cb );
        class _UnixReadPipeTransport ( transports . ReadTransport ) ;
        max_size = 256 * 1024;
        pub fn __init__ ( &self, loop , pipe , protocol , waiter = None /* Option */ , extra = None /* Option */ )  {
        super ( ) . __init__ ( extra );
        self . _extra [ "pipe" ] = pipe;
        self . _loop = loop;
        self . _pipe = pipe;
        self . _fileno = pipe . fileno ( );
        self . _protocol = protocol;
        self . _closing = false;
        self . _paused = false;
        mode = os . fstat ( self . _fileno ) . st_mode;
        if !( stat . S_ISFIFO ( mode ) or {
        stat . S_ISSOCK ( mode ) or;
        stat . S_ISCHR ( mode ) ) ;
        self . _pipe = None /* Option */;
        self . _fileno = None /* Option */;
        self . _protocol = None /* Option */;
        panic!("ValueError ( "Pipe transport is for pipes/sockets only." )");
        os . set_blocking ( self . _fileno , false );
        self . _loop . call_soon ( self . _protocol . connection_made , self );
        self . _loop . call_soon ( self . _add_reader ,;
        self . _fileno , self . _read_ready );
        if waiter is !None /* Option */ {
        self . _loop . call_soon ( futures . _set_result_unless_cancelled ,;
        waiter , None /* Option */ );
        pub fn _add_reader ( &self, fd , callback )  {
        if !self . is_reading ( ) {
        return;
        self . _loop . _add_reader ( fd , callback );
        pub fn is_reading ( self )  {
        return  !self . _paused && !self . _closing;
        pub fn __repr__ ( self )  {
        info = [ self . __class__ . __name__ ];
        if self . _pipe is None /* Option */ {
        info . append ( "closed" );
        } else if self . _closing {
        info . append ( "closing" );
        info . append ( format!("fd={self._fileno}" ));
        selector = getattr ( self . _loop , "_selector" , None /* Option */ );
        if self . _pipe is !None /* Option */ && selector is !None /* Option */ {
        polling = selector_events . _test_selector_event (;
        selector , self . _fileno , selectors . EVENT_READ );
        if polling {
        info . append ( "polling" );
        } else {
        info . append ( "idle" );
        } else if self . _pipe is !None /* Option */ {
        info . append ( "open" );
        } else {
        info . append ( "closed" );
        return  "<{}>" . format ( " " . join ( info ) );
        pub fn _read_ready ( self )  {
        // try {
        data = os . read ( self . _fileno , self . max_size );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  OSError as exc  {
        self . _fatal_error ( exc , "Fatal read error on pipe transport" );
        } else {
        if data {
        self . _protocol . data_received ( data );
        } else {
        if self . _loop . get_debug ( ) {
        logger . info ( "%r was closed by peer" , self );
        self . _closing = true;
        self . _loop . _remove_reader ( self . _fileno );
        self . _loop . call_soon ( self . _protocol . eof_received );
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        pub fn pause_reading ( self )  {
        if !self . is_reading ( ) {
        return;
        self . _paused = true;
        self . _loop . _remove_reader ( self . _fileno );
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r pauses reading" , self );
        pub fn resume_reading ( self )  {
        if self . _closing || !self . _paused {
        return;
        self . _paused = false;
        self . _loop . _add_reader ( self . _fileno , self . _read_ready );
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r resumes reading" , self );
        pub fn set_protocol ( &self, protocol )  {
        self . _protocol = protocol;
        pub fn get_protocol ( self )  {
        return  self . _protocol;
        pub fn is_closing ( self )  {
        return  self . _closing;
        pub fn close ( self )  {
        if !self . _closing {
        self . _close ( None /* Option */ );
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if self . _pipe is !None /* Option */ {
        _warn ( format!("unclosed transport {self!r}" , ResourceWarning , source = self ));
        self . _pipe . close ( );
        pub fn _fatal_error ( &self, exc , message = "Fatal error on pipe transport" )  {
        if ( isinstance ( exc , OSError ) && exc . errno == errno . EIO ) {
        if self . _loop . get_debug ( ) {
        logger . debug ( "%r: %s" , self , message , exc_info = true );
        } else {
        self . _loop . call_exception_handler ( {;
        "message" : message ,;
        "exception" : exc ,;
        "transport" : self ,;
        "protocol" : self . _protocol ,;
        } );
        self . _close ( exc );
        pub fn _close ( &self, exc )  {
        self . _closing = true;
        self . _loop . _remove_reader ( self . _fileno );
        self . _loop . call_soon ( self . _call_connection_lost , exc );
        pub fn _call_connection_lost ( &self, exc )  {
        // try {
        self . _protocol . connection_lost ( exc );
        // } finally {
        self . _pipe . close ( );
        self . _pipe = None /* Option */;
        self . _protocol = None /* Option */;
        self . _loop = None /* Option */;
        class _UnixWritePipeTransport ( transports . _FlowControlMixin ,;
        transports . WriteTransport ) ;
        pub fn __init__ ( &self, loop , pipe , protocol , waiter = None /* Option */ , extra = None /* Option */ )  {
        super ( ) . __init__ ( extra , loop );
        self . _extra [ "pipe" ] = pipe;
        self . _pipe = pipe;
        self . _fileno = pipe . fileno ( );
        self . _protocol = protocol;
        self . _buffer = bytearray ( );
        self . _conn_lost = 0;
        self . _closing = false;
        mode = os . fstat ( self . _fileno ) . st_mode;
        is_char = stat . S_ISCHR ( mode );
        is_fifo = stat . S_ISFIFO ( mode );
        is_socket = stat . S_ISSOCK ( mode );
        if !( is_char || is_fifo || is_socket ) {
        self . _pipe = None /* Option */;
        self . _fileno = None /* Option */;
        self . _protocol = None /* Option */;
        panic!("ValueError ( "Pipe transport is only for "");
        "pipes, sockets && character devices" );
        os . set_blocking ( self . _fileno , false );
        self . _loop . call_soon ( self . _protocol . connection_made , self );
        if is_socket || ( is_fifo && !sys . platform . startswith ( "aix" ) ) {
        self . _loop . call_soon ( self . _loop . _add_reader ,;
        self . _fileno , self . _read_ready );
        if waiter is !None /* Option */ {
        self . _loop . call_soon ( futures . _set_result_unless_cancelled ,;
        waiter , None /* Option */ );
        pub fn __repr__ ( self )  {
        info = [ self . __class__ . __name__ ];
        if self . _pipe is None /* Option */ {
        info . append ( "closed" );
        } else if self . _closing {
        info . append ( "closing" );
        info . append ( format!("fd={self._fileno}" ));
        selector = getattr ( self . _loop , "_selector" , None /* Option */ );
        if self . _pipe is !None /* Option */ && selector is !None /* Option */ {
        polling = selector_events . _test_selector_event (;
        selector , self . _fileno , selectors . EVENT_WRITE );
        if polling {
        info . append ( "polling" );
        } else {
        info . append ( "idle" );
        bufsize = self . get_write_buffer_size ( );
        info . append ( format!("bufsize={bufsize}" ));
        } else if self . _pipe is !None /* Option */ {
        info . append ( "open" );
        } else {
        info . append ( "closed" );
        return  "<{}>" . format ( " " . join ( info ) );
        pub fn get_write_buffer_size ( self )  {
        return  len ( self . _buffer );
        pub fn _read_ready ( self )  {
        if self . _loop . get_debug ( ) {
        logger . info ( "%r was closed by peer" , self );
        if self . _buffer {
        self . _close ( BrokenPipeError ( ) );
        } else {
        self . _close ( );
        pub fn write ( &self, data )  {
        assert isinstance ( data , ( bytes , bytearray , memoryview ) ) , repr ( data );
        if isinstance ( data , bytearray ) {
        data = memoryview ( data );
        if !data {
        return;
        if self . _conn_lost || self . _closing {
        if self . _conn_lost >= constants . LOG_THRESHOLD_FOR_CONNLOST_WRITES {
        logger . warning ( "pipe closed by peer || ";
        "os.write(pipe, data) raised exception." );
        self . _conn_lost + = 1;
        return;
        if !self . _buffer {
        // try {
        n = os . write ( self . _fileno , data );
        // } catch  ( BlockingIOError , InterruptedError )  {
        n = 0;
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _conn_lost + = 1;
        self . _fatal_error ( exc , "Fatal write error on pipe transport" );
        return;
        if n == len ( data ) {
        return;
        } else if n > 0 {
        data = memoryview ( data ) [ n : ];
        self . _loop . _add_writer ( self . _fileno , self . _write_ready );
        self . _buffer + = data;
        self . _maybe_pause_protocol ( );
        pub fn _write_ready ( self )  {
        assert self . _buffer , "Data should !be empty";
        // try {
        n = os . write ( self . _fileno , self . _buffer );
        // } catch  ( BlockingIOError , InterruptedError )  {
        // pass
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _buffer . clear ( );
        self . _conn_lost + = 1;
        self . _loop . _remove_writer ( self . _fileno );
        self . _fatal_error ( exc , "Fatal write error on pipe transport" );
        } else {
        if n == len ( self . _buffer ) {
        self . _buffer . clear ( );
        self . _loop . _remove_writer ( self . _fileno );
        self . _maybe_resume_protocol ( );
        if self . _closing {
        self . _loop . _remove_reader ( self . _fileno );
        self . _call_connection_lost ( None /* Option */ );
        return;
        } else if n > 0 {
        del self . _buffer [ : n ];
        pub fn can_write_eof ( self )  {
        return  true;
        pub fn write_eof ( self )  {
        if self . _closing {
        return;
        assert self . _pipe;
        self . _closing = true;
        if !self . _buffer {
        self . _loop . _remove_reader ( self . _fileno );
        self . _loop . call_soon ( self . _call_connection_lost , None /* Option */ );
        pub fn set_protocol ( &self, protocol )  {
        self . _protocol = protocol;
        pub fn get_protocol ( self )  {
        return  self . _protocol;
        pub fn is_closing ( self )  {
        return  self . _closing;
        pub fn close ( self )  {
        if self . _pipe is !None /* Option */ && !self . _closing {
        self . write_eof ( );
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if self . _pipe is !None /* Option */ {
        _warn ( format!("unclosed transport {self!r}" , ResourceWarning , source = self ));
        self . _pipe . close ( );
        pub fn abort ( self )  {
        self . _close ( None /* Option */ );
        pub fn _fatal_error ( &self, exc , message = "Fatal error on pipe transport" )  {
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
        self . _close ( exc );
        pub fn _close ( &self, exc = None /* Option */ )  {
        self . _closing = true;
        if self . _buffer {
        self . _loop . _remove_writer ( self . _fileno );
        self . _buffer . clear ( );
        self . _loop . _remove_reader ( self . _fileno );
        self . _loop . call_soon ( self . _call_connection_lost , exc );
        pub fn _call_connection_lost ( &self, exc )  {
        // try {
        self . _protocol . connection_lost ( exc );
        // } finally {
        self . _pipe . close ( );
        self . _pipe = None /* Option */;
        self . _protocol = None /* Option */;
        self . _loop = None /* Option */;
        class _UnixSubprocessTransport ( base_subprocess . BaseSubprocessTransport ) ;
        pub fn _start ( &self, args , shell , stdin , stdout , stderr , bufsize , ** kwargs )  {
        stdin_w = None /* Option */;
        if stdin == subprocess . PIPE && sys . platform . startswith ( "aix" ) {
        stdin , stdin_w = socket . socketpair ( );
        // try {
        self . _proc = subprocess . Popen (;
        args , shell = shell , stdin = stdin , stdout = stdout , stderr = stderr ,;
        universal_newlines = false , bufsize = bufsize , ** kwargs );
        if stdin_w is !None /* Option */ {
        stdin . close ( );
        self . _proc . stdin = open ( stdin_w . detach ( ) , "wb" , buffering = bufsize );
        stdin_w = None /* Option */;
        // } finally {
        if stdin_w is !None /* Option */ {
        stdin . close ( );
        stdin_w . close ( );
        class AbstractChildWatcher ;
        "Abstract base class for monitoring child processes.

    Objects derived from this class monitor a collection of subprocesses and
    report their termination || interruption by a signal.

    New callbacks are registered with .add_child_handler(). Starting a new
    process must be done within a 'with' block to allow the watcher to suspend
    its activity until the new process if fully registered (this == needed to
    prevent a race condition in some implementations).

    Example:
        with watcher:
            proc = subprocess.Popen("sleep 1")
            watcher.add_child_handler(proc.pid, callback)

    Notes:
        Implementations of this class must be thread-safe.

        Since child watcher objects may catch the SIGCHLD signal && call
        waitpid(-1), there should be only one active object per process.
    ";
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        "Register a new child handler.

        Arrange for callback(pid, returncode, *args) to be called when
        process 'pid' terminates. Specifying another callback for the same
        process replaces the previous handler.

        Note: callback() must be thread-safe.
        ";
        panic!("NotImplementedError ( )");
        pub fn remove_child_handler ( &self, pid )  {
        "Removes the handler for process 'pid'.

        The function returns true if the handler was successfully removed,
        false if there was nothing to remove.";
        panic!("NotImplementedError ( )");
        pub fn attach_loop ( &self, loop )  {
        "Attach the watcher to an event loop.

        If the watcher was previously attached to an event loop, then it is
        first detached before attaching to the new loop.

        Note: loop may be None /* Option */.
        ";
        panic!("NotImplementedError ( )");
        pub fn close ( self )  {
        "Close the watcher.

        This must be called to make sure that any underlying resource == freed.
        ";
        panic!("NotImplementedError ( )");
        pub fn is_active ( self )  {
        "Return ``true`` if the watcher == active && == used by the event loop.

        Return true if the watcher == installed && ready to handle process exit
        notifications.

        ";
        panic!("NotImplementedError ( )");
        pub fn __enter__ ( self )  {
        "Enter the watcher's context && allow starting new processes

        This function must return selformat!(");
        panic!("NotImplementedError ( )");
        pub fn __exit__ ( &self, a , b , c )  {
        "Exit the watcher's context";
        panic!("NotImplementedError ( )");
        class PidfdChildWatcher ( AbstractChildWatcher ) ;
        "Child watcher implementation using Linux's pid file descriptors.

    This child watcher polls process file descriptors (pidfds) to await child
    process termination. In some respects, PidfdChildWatcher == a "Goldilocks"
    child watcher implementation. It doesn't require signals || threads, doesn't
    interfere with any processes launched outside the event loop, && scales
    linearly with the number of subprocesses launched by the event loop. The
    main disadvantage == that pidfds are specific to Linux, && only work on
    recent (5.3+) kernels.
    ";
        pub fn __init__ ( self )  {
        self . _loop = None /* Option */;
        self . _callbacks = { };
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_value , exc_traceback )  {
        // pass
        pub fn is_active ( self )  {
        return  self . _loop is !None /* Option */ && self . _loop . is_running ( );
        pub fn close ( self )  {
        self . attach_loop ( None /* Option */ );
        pub fn attach_loop ( &self, loop )  {
        if self . _loop is !None /* Option */ && loop is None /* Option */ && self . _callbacks {
        warnings . warn (;
        "A loop == being detached ";
        "from a child watcher with pending handlers" ,;
        RuntimeWarning );
        for pidfd , _ , _ in self . _callbacks . values ( ) .iter() {
        self . _loop . _remove_reader ( pidfd );
        os . close ( pidfd );
        self . _callbacks . clear ( );
        self . _loop = loop;
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        existing = self . _callbacks . get ( pid );
        if existing is !None /* Option */ {
        self . _callbacks [ pid ] = existing [ 0 ] , callback , args;
        } else {
        pidfd = os . pidfd_open ( pid );
        self . _loop . _add_reader ( pidfd , self . _do_wait , pid );
        self . _callbacks [ pid ] = pidfd , callback , args;
        pub fn _do_wait ( &self, pid )  {
        pidfd , callback , args = self . _callbacks . pop ( pid );
        self . _loop . _remove_reader ( pidfd );
        // try {
        _ , status = os . waitpid ( pid , 0 );
        // } catch  ChildProcessError  {
        return code = 255;
        logger . warning (;
        "child process pid %d exit status already read: ";
        " will report returncode 255" ,;
        pid );
        } else {
        return code = waitstatus_to_exitcode ( status );
        os . close ( pidfd );
        callback ( pid , returncode , * args );
        pub fn remove_child_handler ( &self, pid )  {
        // try {
        pidfd , _ , _ = self . _callbacks . pop ( pid );
        // } catch  KeyError  {
        return  false;
        self . _loop . _remove_reader ( pidfd );
        os . close ( pidfd );
        return  true;
        class BaseChildWatcher ( AbstractChildWatcher ) ;
        pub fn __init__ ( self )  {
        self . _loop = None /* Option */;
        self . _callbacks = { };
        pub fn close ( self )  {
        self . attach_loop ( None /* Option */ );
        pub fn is_active ( self )  {
        return  self . _loop is !None /* Option */ && self . _loop . is_running ( );
        pub fn _do_waitpid ( &self, expected_pid )  {
        panic!("NotImplementedError ( )");
        pub fn _do_waitpid_all ( self )  {
        panic!("NotImplementedError ( )");
        pub fn attach_loop ( &self, loop )  {
        assert loop == None /* Option */ || isinstance ( loop , events . AbstractEventLoop );
        if self . _loop is !None /* Option */ && loop is None /* Option */ && self . _callbacks {
        warnings . warn (;
        "A loop == being detached ";
        "from a child watcher with pending handlers" ,;
        RuntimeWarning );
        if self . _loop is !None /* Option */ {
        self . _loop . remove_signal_handler ( signal . SIGCHLD );
        self . _loop = loop;
        if loop is !None /* Option */ {
        loop . add_signal_handler ( signal . SIGCHLD , self . _sig_chld );
        self . _do_waitpid_all ( );
        pub fn _sig_chld ( self )  {
        // try {
        self . _do_waitpid_all ( );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        self . _loop . call_exception_handler ( {;
        "message" : "Unknown exception in SIGCHLD handler" ,;
        "exception" : exc ,;
        } );
        class SafeChildWatcher ( BaseChildWatcher ) ;
        "'Safe' child watcher implementation.

    This implementation avoids disrupting other code spawning processes by
    polling explicitly each process in the SIGCHLD handler instead of calling
    os.waitpid(-1).

    This == a safe solution but it has a significant overhead when handling a
    big number of children (O(n) each time SIGCHLD == raised)
    ";
        pub fn close ( self )  {
        self . _callbacks . clear ( );
        super ( ) . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, a , b , c )  {
        // pass
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        self . _callbacks [ pid ] = ( callback , args );
        self . _do_waitpid ( pid );
        pub fn remove_child_handler ( &self, pid )  {
        // try {
        del self . _callbacks [ pid ];
        return  true;
        // } catch  KeyError  {
        return  false;
        pub fn _do_waitpid_all ( self )  {
        for pid in list ( self . _callbacks ) .iter() {
        self . _do_waitpid ( pid );
        pub fn _do_waitpid ( &self, expected_pid )  {
        assert expected_pid > 0;
        // try {
        pid , status = os . waitpid ( expected_pid , os . WNOHANG );
        // } catch  ChildProcessError  {
        pid = expected_pid;
        return code = 255;
        logger . warning (;
        "Unknown child process pid %d, will report returncode 255" ,;
        pid );
        } else {
        if pid == 0 {
        return;
        return code = waitstatus_to_exitcode ( status );
        if self . _loop . get_debug ( ) {
        logger . debug ( "process %s exited with returncode %s" ,;
        expected_pid , returncode );
        // try {
        callback , args = self . _callbacks . pop ( pid );
        // } catch  KeyError  {
        if self . _loop . get_debug ( ) {
        logger . warning ( "Child watcher got an unexpected pid: %r" ,;
        pid , exc_info = true );
        } else {
        callback ( pid , returncode , * args );
        class FastChildWatcher ( BaseChildWatcher ) ;
        "'Fast' child watcher implementation.

    This implementation reaps every terminated processes by calling
    os.waitpid(-1) directly, possibly breaking other code spawning processes
    && waiting for their termination.

    There == no noticeable overhead when handling a big number of children
    (O(1) each time a child terminates).
    ";
        pub fn __init__ ( self )  {
        super ( ) . __init__ ( );
        self . _lock = threading . Lock ( );
        self . _zombies = { };
        self . _forks = 0;
        pub fn close ( self )  {
        self . _callbacks . clear ( );
        self . _zombies . clear ( );
        super ( ) . close ( );
        pub fn __enter__ ( self )  {
        // with scope: self . _lock  {
        self . _forks + = 1;
        return  self;
        pub fn __exit__ ( &self, a , b , c )  {
        // with scope: self . _lock  {
        self . _forks - = 1;
        if self . _forks || !self . _zombies {
        return;
        collateral_victims = str ( self . _zombies );
        self . _zombies . clear ( );
        logger . warning (;
        "Caught subprocesses termination from unknown pids: %s" ,;
        collateral_victims );
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        assert self . _forks , "Must use the context manager";
        // with scope: self . _lock  {
        // try {
        return code = self . _zombies . pop ( pid );
        // } catch  KeyError  {
        self . _callbacks [ pid ] = callback , args;
        return;
        callback ( pid , returncode , * args );
        pub fn remove_child_handler ( &self, pid )  {
        // try {
        del self . _callbacks [ pid ];
        return  true;
        // } catch  KeyError  {
        return  false;
        pub fn _do_waitpid_all ( self )  {
        while true  {
        // try {
        pid , status = os . waitpid ( -1 , os . WNOHANG );
        // } catch  ChildProcessError  {
        return;
        } else {
        if pid == 0 {
        return;
        return code = waitstatus_to_exitcode ( status );
        // with scope: self . _lock  {
        // try {
        callback , args = self . _callbacks . pop ( pid );
        // } catch  KeyError  {
        if self . _forks {
        self . _zombies [ pid ] = returncode;
        if self . _loop . get_debug ( ) {
        logger . debug ( "unknown process %s exited ";
        "with returncode %s" ,;
        pid , returncode );
        continue;
        callback = None /* Option */;
        } else {
        if self . _loop . get_debug ( ) {
        logger . debug ( "process %s exited with returncode %s" ,;
        pid , returncode );
        if callback is None /* Option */ {
        logger . warning (;
        "Caught subprocess termination from unknown pid: ";
        "%d -> %d" , pid , returncode );
        } else {
        callback ( pid , returncode , * args );
        class MultiLoopChildWatcher ( AbstractChildWatcher ) ;
        "A watcher that doesn't require running loop in the main thread.

    This implementation registers a SIGCHLD signal handler on
    instantiation (which may conflict with other code that
    install own handler for this signal).

    The solution == safe but it has a significant overhead when
    handling a big number of processes (*O(n)* each time a
    SIGCHLD == received).
    ";
        pub fn __init__ ( self )  {
        self . _callbacks = { };
        self . _saved_sighandler = None /* Option */;
        pub fn is_active ( self )  {
        return  self . _saved_sighandler is !None /* Option */;
        pub fn close ( self )  {
        self . _callbacks . clear ( );
        if self . _saved_sighandler is None /* Option */ {
        return;
        handler = signal . getsignal ( signal . SIGCHLD );
        if handler != self . _sig_chld {
        logger . warning ( "SIGCHLD handler was changed by outside code" );
        } else {
        signal . signal ( signal . SIGCHLD , self . _saved_sighandler );
        self . _saved_sighandler = None /* Option */;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        // pass
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        loop = events . get_running_loop ( );
        self . _callbacks [ pid ] = ( loop , callback , args );
        self . _do_waitpid ( pid );
        pub fn remove_child_handler ( &self, pid )  {
        // try {
        del self . _callbacks [ pid ];
        return  true;
        // } catch  KeyError  {
        return  false;
        pub fn attach_loop ( &self, loop )  {
        if self . _saved_sighandler is !None /* Option */ {
        return;
        self . _saved_sighandler = signal . signal ( signal . SIGCHLD , self . _sig_chld );
        if self . _saved_sighandler is None /* Option */ {
        logger . warning ( "Previous SIGCHLD handler was set by non-Python code, ";
        "restore to default handler on watcher close." );
        self . _saved_sighandler = signal . SIG_DFL;
        signal . siginterrupt ( signal . SIGCHLD , false );
        pub fn _do_waitpid_all ( self )  {
        for pid in list ( self . _callbacks ) .iter() {
        self . _do_waitpid ( pid );
        pub fn _do_waitpid ( &self, expected_pid )  {
        assert expected_pid > 0;
        // try {
        pid , status = os . waitpid ( expected_pid , os . WNOHANG );
        // } catch  ChildProcessError  {
        pid = expected_pid;
        return code = 255;
        logger . warning (;
        "Unknown child process pid %d, will report returncode 255" ,;
        pid );
        debug_log = false;
        } else {
        if pid == 0 {
        return;
        return code = waitstatus_to_exitcode ( status );
        debug_log = true;
        // try {
        loop , callback , args = self . _callbacks . pop ( pid );
        // } catch  KeyError  {
        logger . warning ( "Child watcher got an unexpected pid: %r" ,;
        pid , exc_info = true );
        } else {
        if loop . is_closed ( ) {
        logger . warning ( "Loop %r that handles pid %r == closed" , loop , pid );
        } else {
        if debug_log && loop . get_debug ( ) {
        logger . debug ( "process %s exited with returncode %s" ,;
        expected_pid , returncode );
        loop . call_soon_threadsafe ( callback , pid , returncode , * args );
        pub fn _sig_chld ( &self, signum , frame )  {
        // try {
        self . _do_waitpid_all ( );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException  {
        logger . warning ( "Unknown exception in SIGCHLD handler" , exc_info = true );
        class ThreadedChildWatcher ( AbstractChildWatcher ) ;
        "Threaded child watcher implementation.

    The watcher uses a thread per process
    for waiting for the process finish.

    It doesn't require subscription on POSIX signal
    but a thread creation == !free.

    The watcher has O(1) complexity, its performance doesn't depend
    on amount of spawn processes.
    ";
        pub fn __init__ ( self )  {
        self . _pid_counter = itertools . count ( 0 );
        self . _threads = { };
        pub fn is_active ( self )  {
        return  true;
        pub fn close ( self )  {
        // pass
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        // pass
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        threads = vec![ thread.iter().map(|thread| list ( self . _threads . values ( ) );
        if thread . is_alive ( ) ] {
        if threads {
        _warn ( format!("{self.__class__} has registered but !finished child processes" ,);
        ResourceWarning ,;
        source = self );
        pub fn add_child_handler ( &self, pid , callback , * args )  {
        loop = events . get_running_loop ( );
        thread = threading . Thread ( target = self . _do_waitpid ,;
        name = format!("asyncio-waitpid-{next(self._pid_counter)}" ,);
        args = ( loop , pid , callback , args ) ,;
        daemon = true );
        self . _threads [ pid ] = thread;
        thread . start ( );
        pub fn remove_child_handler ( &self, pid )  {
        return  true;
        pub fn attach_loop ( &self, loop )  {
        // pass
        pub fn _do_waitpid ( &self, loop , expected_pid , callback , args )  {
        assert expected_pid > 0;
        // try {
        pid , status = os . waitpid ( expected_pid , 0 );
        // } catch  ChildProcessError  {
        pid = expected_pid;
        return code = 255;
        logger . warning (;
        "Unknown child process pid %d, will report returncode 255" ,;
        pid );
        } else {
        return code = waitstatus_to_exitcode ( status );
        if loop . get_debug ( ) {
        logger . debug ( "process %s exited with returncode %s" ,;
        expected_pid , returncode );
        if loop . is_closed ( ) {
        logger . warning ( "Loop %r that handles pid %r == closed" , loop , pid );
        } else {
        loop . call_soon_threadsafe ( callback , pid , returncode , * args );
        self . _threads . pop ( expected_pid );
        class _UnixDefaultEventLoopPolicy ( events . BaseDefaultEventLoopPolicy ) ;
        "UNIX event loop policy with a watcher for child processes.";
        _loop_factory = _UnixSelectorEventLoop;
        pub fn __init__ ( self )  {
        super ( ) . __init__ ( );
        self . _watcher = None /* Option */;
        pub fn _init_watcher ( self )  {
        // with scope: events . _lock  {
        if self . _watcher is None /* Option */ {
        self . _watcher = ThreadedChildWatcher ( );
        pub fn set_event_loop ( &self, loop )  {
        "Set the event loop.

        As a side effect, if a child watcher was set before, then calling
        .set_event_loop() from the main thread will call .attach_loop(loop) on
        the child watcher.
        ";
        super ( ) . set_event_loop ( loop );
        if ( self . _watcher is !None /* Option */ and {
        threading . current_thread ( ) == threading . main_thread ( ) ) ;
        self . _watcher . attach_loop ( loop );
        pub fn get_child_watcher ( self )  {
        "Get the watcher for child processes.

        If !yet set, a ThreadedChildWatcher object == automatically created.
        ";
        if self . _watcher is None /* Option */ {
        self . _init_watcher ( );
        return  self . _watcher;
        pub fn set_child_watcher ( &self, watcher )  {
        "Set the watcher for child processes.";
        assert watcher == None /* Option */ || isinstance ( watcher , AbstractChildWatcher );
        if self . _watcher is !None /* Option */ {
        self . _watcher . close ( );
        self . _watcher = watcher;
        SelectorEventLoop = _UnixSelectorEventLoop;
        DefaultEventLoopPolicy = _UnixDefaultEventLoopPolicy;
}

