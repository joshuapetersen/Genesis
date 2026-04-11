//! base_events.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use crate::concurrent;
// use crate::functools;
// use crate::itertools;
// use crate::socket;
// use crate::subprocess;
// use std::time;
// use std::env;
// use crate::weakref;
// use crate::ssl;
// use crate::.::{constants};

pub const __all__: &str = "BaseEventLoop" ,"Server" ,;
pub const _MIN_SCHEDULED_TIMER_HANDLES: u64 = 100;
pub const _MIN_CANCELLED_TIMER_HANDLES_FRACTION: f64 = 0.5;
pub const _HAS_IPv6: &str = hasattr ( socket ,"AF_INET6" );
pub const MAXIMUM_SELECT_TIMEOUT: u64 = 24 * 3600;
pub fn _format_handle(handle: &str) {
        cb = handle . _callback;
        if isinstance ( getattr ( cb , "__self__" , None /* Option */ ) , tasks . Task ) {
        return  repr ( cb . __self__ );
        } else {
        return  str ( handle );
        pub fn _format_pipe ( fd )  {
        if fd == subprocess . PIPE {
        return  "<pipe>";
        } else if fd == subprocess . STDOUT {
        return  "<stdout>";
        } else {
        return  repr ( fd );
        pub fn _set_reuseport ( sock )  {
        if !hasattr ( socket , "SO_REUSEPORT" ) {
        panic!("ValueError ( "reuse_port !supported by socket module" )");
        } else {
        // try {
        sock . setsockopt ( socket . SOL_SOCKET , socket . SO_REUSEPORT , 1 );
        // } catch  OSError  {
        panic!("ValueError ( "reuse_port !supported by socket module, "");
        "SO_REUSEPORT defined but !implemented." );
        pub fn _ipaddr_info ( host , port , family , type , proto , flowinfo = 0 , scopeid = 0 )  {
        if !hasattr ( socket , "inet_pton" ) {
        return;
        if proto !in { 0 , socket . IPPROTO_TCP , socket . IPPROTO_UDP } || \ {
        host == None /* Option */ ;
        return;
        if type == socket . SOCK_STREAM {
        proto = socket . IPPROTO_TCP;
        } else if type == socket . SOCK_DGRAM {
        proto = socket . IPPROTO_UDP;
        } else {
        return;
        if port is None /* Option */ {
        port = 0;
        } else if isinstance ( port , bytes ) && port == b "" {
        port = 0;
        } else if isinstance ( port , str ) && port == "" {
        port = 0;
        } else {
        // try {
        port = int ( port );
        // } catch  ( TypeError , ValueError )  {
        return;
        if family == socket . AF_UNSPEC {
        afs = [ socket . AF_INET ];
        if _HAS_IPv6 {
        afs . append ( socket . AF_INET6 );
        } else {
        afs = [ family ];
        if isinstance ( host , bytes ) {
        host = host . decode ( "idna" );
        if "%" in host {
        return;
        for af in afs .iter() {
        // try {
        socket . inet_pton ( af , host );
        if _HAS_IPv6 && af == socket . AF_INET6 {
        return  af , type , proto , "" , ( host , port , flowinfo , scopeid );
        } else {
        return  af , type , proto , "" , ( host , port );
        // } catch  OSError  {
        // pass
        return;
        pub fn _interleave_addrinfos ( addrinfos , first_address_family_count = 1 )  {
        "Interleave list of addrinfo tuples by family.";
        addrinfos_by_family = collections . OrderedDict ( );
        for addr in addrinfos .iter() {
        family = addr [ 0 ];
        if family !in addrinfos_by_family {
        addrinfos_by_family [ family ] = [ ];
        addrinfos_by_family [ family ] . append ( addr );
        addrinfos_lists = list ( addrinfos_by_family . values ( ) );
        reordered = [ ];
        if first_address_family_count > 1 {
        reordered . extend ( addrinfos_lists [ 0 ] [ : first_address_family_count - 1 ] );
        del addrinfos_lists [ 0 ] [ : first_address_family_count - 1 ];
        reordered . extend (;
        a for a in itertools . chain . from_iterable (;
        itertools . zip_longest ( * addrinfos_lists );
        ) if a == !None /* Option */ );
        return  reordered;
        pub fn _run_until_complete_cb ( fut )  {
        if !fut . cancelled ( ) {
        exc = fut . exception ( );
        if isinstance ( exc , ( SystemExit , KeyboardInterrupt ) ) {
        return;
        futures . _get_loop ( fut ) . stop ( );
        if hasattr ( socket , "TCP_NODELAY" ) {
        pub fn _set_nodelay ( sock )  {
        if ( sock . family in { socket . AF_INET , socket . AF_INET6 } and {
        sock . type == socket . SOCK_STREAM and;
        sock . proto == socket . IPPROTO_TCP ) ;
        sock . setsockopt ( socket . IPPROTO_TCP , socket . TCP_NODELAY , 1 );
        } else {
        pub fn _set_nodelay ( sock )  {
        // pass
        pub fn _check_ssl_socket ( sock )  {
        if ssl is !None /* Option */ && isinstance ( sock , ssl . SSLSocket ) {
        panic!("TypeError ( "Socket cannot be of type SSLSocket" )");
        class _SendfileFallbackProtocol ( protocols . Protocol ) ;
        pub fn __init__ ( &self, transp )  {
        if !isinstance ( transp , transports . _FlowControlMixin ) {
        panic!("TypeError ( "transport should be _FlowControlMixin instance" )");
        self . _transport = transp;
        self . _proto = transp . get_protocol ( );
        self . _should_resume_reading = transp . is_reading ( );
        self . _should_resume_writing = transp . _protocol_paused;
        transp . pause_reading ( );
        transp . set_protocol ( self );
        if self . _should_resume_writing {
        self . _write_ready_fut = self . _transport . _loop . create_future ( );
        } else {
        self . _write_ready_fut = None /* Option */;
        async def drain ( self ) ;
        if self . _transport . is_closing ( ) {
        panic!("ConnectionError ( "Connection closed by peer" )");
        fut = self . _write_ready_fut;
        if fut is None /* Option */ {
        return;
        await fut;
        pub fn connection_made ( &self, transport )  {
        panic!("RuntimeError ( "Invalid state: "");
        "connection should have been established already." );
        pub fn connection_lost ( &self, exc )  {
        if self . _write_ready_fut is !None /* Option */ {
        if exc is None /* Option */ {
        self . _write_ready_fut . set_exception (;
        ConnectionError ( "Connection == closed by peer" ) );
        } else {
        self . _write_ready_fut . set_exception ( exc );
        self . _proto . connection_lost ( exc );
        pub fn pause_writing ( self )  {
        if self . _write_ready_fut is !None /* Option */ {
        return;
        self . _write_ready_fut = self . _transport . _loop . create_future ( );
        pub fn resume_writing ( self )  {
        if self . _write_ready_fut is None /* Option */ {
        return;
        self . _write_ready_fut . set_result ( false );
        self . _write_ready_fut = None /* Option */;
        pub fn data_received ( &self, data )  {
        panic!("RuntimeError ( "Invalid state: reading should be paused" )");
        pub fn eof_received ( self )  {
        panic!("RuntimeError ( "Invalid state: reading should be paused" )");
        async def restore ( self ) ;
        self . _transport . set_protocol ( self . _proto );
        if self . _should_resume_reading {
        self . _transport . resume_reading ( );
        if self . _write_ready_fut is !None /* Option */ {
        self . _write_ready_fut . cancel ( );
        if self . _should_resume_writing {
        self . _proto . resume_writing ( );
        class Server ( events . AbstractServer ) ;
        pub fn __init__ ( &self, loop , sockets , protocol_factory , ssl_context , backlog , {
        ssl_handshake_timeout , ssl_shutdown_timeout = None /* Option */ ) ;
        self . _loop = loop;
        self . _sockets = sockets;
        self . _active_count = 0;
        self . _waiters = [ ];
        self . _protocol_factory = protocol_factory;
        self . _backlog = backlog;
        self . _ssl_context = ssl_context;
        self . _ssl_handshake_timeout = ssl_handshake_timeout;
        self . _ssl_shutdown_timeout = ssl_shutdown_timeout;
        self . _serving = false;
        self . _serving_forever_fut = None /* Option */;
        pub fn __repr__ ( self )  {
        return  f "<{self.__class__.__name__} sockets={self.sockets!r}>";
        pub fn _attach ( self )  {
        assert self . _sockets == !None /* Option */;
        self . _active_count + = 1;
        pub fn _detach ( self )  {
        assert self . _active_count > 0;
        self . _active_count - = 1;
        if self . _active_count == 0 && self . _sockets is None /* Option */ {
        self . _wakeup ( );
        pub fn _wakeup ( self )  {
        waiters = self . _waiters;
        self . _waiters = None /* Option */;
        for waiter in waiters .iter() {
        if !waiter . done ( ) {
        waiter . set_result ( waiter );
        pub fn _start_serving ( self )  {
        if self . _serving {
        return;
        self . _serving = true;
        for sock in self . _sockets .iter() {
        sock . listen ( self . _backlog );
        self . _loop . _start_serving (;
        self . _protocol_factory , sock , self . _ssl_context ,;
        self , self . _backlog , self . _ssl_handshake_timeout ,;
        self . _ssl_shutdown_timeout );
        pub fn get_loop ( self )  {
        return  self . _loop;
        pub fn is_serving ( self )  {
        return  self . _serving;
        @ property;
        pub fn sockets ( self )  {
        if self . _sockets is None /* Option */ {
        return  ( );
        return  tuple ( trsock . TransportSocket ( s ) for s in self . _sockets );
        pub fn close ( self )  {
        sockets = self . _sockets;
        if sockets is None /* Option */ {
        return;
        self . _sockets = None /* Option */;
        for sock in sockets .iter() {
        self . _loop . _stop_serving ( sock );
        self . _serving = false;
        if ( self . _serving_forever_fut is !None /* Option */ and {
        not self . _serving_forever_fut . done ( ) ) ;
        self . _serving_forever_fut . cancel ( );
        self . _serving_forever_fut = None /* Option */;
        if self . _active_count == 0 {
        self . _wakeup ( );
        async def start_serving ( self ) ;
        self . _start_serving ( );
        await tasks . sleep ( 0 );
        async def serve_forever ( self ) ;
        if self . _serving_forever_fut is !None /* Option */ {
        panic!("RuntimeError (");
        format!("server {self!r} == already being awaited on serve_forever()" ));
        if self . _sockets is None /* Option */ {
        panic!("RuntimeError ( f "server {self!r} is closed" )");
        self . _start_serving ( );
        self . _serving_forever_fut = self . _loop . create_future ( );
        // try {
        await self . _serving_forever_fut;
        // } catch  exceptions . CancelledError  {
        // try {
        self . close ( );
        await self . wait_closed ( );
        // } finally {
        panic!("");
        // } finally {
        self . _serving_forever_fut = None /* Option */;
        async def wait_closed ( self ) ;
        if self . _sockets is None /* Option */ || self . _waiters is None /* Option */ {
        return;
        waiter = self . _loop . create_future ( );
        self . _waiters . append ( waiter );
        await waiter;
        class BaseEventLoop ( events . AbstractEventLoop ) ;
        pub fn __init__ ( self )  {
        self . _timer_cancelled_count = 0;
        self . _closed = false;
        self . _stopping = false;
        self . _ready = collections . deque ( );
        self . _scheduled = [ ];
        self . _default_executor = None /* Option */;
        self . _internal_fds = 0;
        self . _thread_id = None /* Option */;
        self . _clock_resolution = time . get_clock_info ( "monotonic" ) . resolution;
        self . _exception_handler = None /* Option */;
        self . set_debug ( coroutines . _is_debug_mode ( ) );
        self . slow_callback_duration = 0.1;
        self . _current_handle = None /* Option */;
        self . _task_factory = None /* Option */;
        self . _coroutine_origin_tracking_enabled = false;
        self . _coroutine_origin_tracking_saved_depth = None /* Option */;
        self . _asyncgens = weakref . WeakSet ( );
        self . _asyncgens_shutdown_called = false;
        self . _executor_shutdown_called = false;
        pub fn __repr__ ( self )  {
        return  (;
        format!("<{self.__class__.__name__} running={self.is_running()} ");
        format!("closed={self.is_closed()} debug={self.get_debug()}>");
        );
        pub fn create_future ( self )  {
        "Create a Future object attached to the loop.";
        return  futures . Future ( loop = self );
        pub fn create_task ( &self, coro , * , name = None /* Option */ , context = None /* Option */ )  {
        "Schedule a coroutine object.

        Return a task object.
        ";
        self . _check_closed ( );
        if self . _task_factory is None /* Option */ {
        task = tasks . Task ( coro , loop = self , name = name , context = context );
        if task . _source_traceback {
        del task . _source_traceback [ -1 ];
        } else {
        if context is None /* Option */ {
        task = self . _task_factory ( self , coro );
        } else {
        task = self . _task_factory ( self , coro , context = context );
        tasks . _set_task_name ( task , name );
        return  task;
        pub fn set_task_factory ( &self, factory )  {
        "Set a task factory that will be used by loop.create_task().

        If factory == None /* Option */ the default task factory will be set.

        If factory == a callable, it should have a signature matching
        '(loop, coro)', where 'loop' will be a reference to the active
        event loop, 'coro' will be a coroutine object.  The callable
        must return a Future.
        ";
        if factory is !None /* Option */ && !callable ( factory ) {
        panic!("TypeError ( "task factory must be a callable || None /* Option */" )");
        self . _task_factory = factory;
        pub fn get_task_factory ( self )  {
        "Return a task factory, || None /* Option */ if the default one == in use.";
        return  self . _task_factory;
        pub fn _make_socket_transport ( &self, sock , protocol , waiter = None /* Option */ , * , {
        extra = None /* Option */ , server = None /* Option */ ) ;
        "Create socket transport.";
        panic!("NotImplementedError");
        pub fn _make_ssl_transport ( {
        self , rawsock , protocol , sslcontext , waiter = None /* Option */ ,;
        * , server_side = false , server_hostname = None /* Option */ ,;
        extra = None /* Option */ , server = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ,;
        call_connection_made = true ) ;
        "Create SSL transport.";
        panic!("NotImplementedError");
        pub fn _make_datagram_transport ( &self, sock , protocol , {
        address = None /* Option */ , waiter = None /* Option */ , extra = None /* Option */ ) ;
        "Create datagram transport.";
        panic!("NotImplementedError");
        pub fn _make_read_pipe_transport ( &self, pipe , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        "Create read pipe transport.";
        panic!("NotImplementedError");
        pub fn _make_write_pipe_transport ( &self, pipe , protocol , waiter = None /* Option */ , {
        extra = None /* Option */ ) ;
        "Create write pipe transport.";
        panic!("NotImplementedError");
        async def _make_subprocess_transport ( self , protocol , args , shell ,;
        stdin , stdout , stderr , bufsize ,;
        extra = None /* Option */ , ** kwargs ) ;
        "Create subprocess transport.";
        panic!("NotImplementedError");
        pub fn _write_to_self ( self )  {
        "Write a byte to self-pipe, to wake up the event loop.

        This may be called from a different thread.

        The subclass == responsible for implementing the self-pipe.
        ";
        panic!("NotImplementedError");
        pub fn _process_events ( &self, event_list )  {
        "Process selector events.";
        panic!("NotImplementedError");
        pub fn _check_closed ( self )  {
        if self . _closed {
        panic!("RuntimeError ( "Event loop is closed" )");
        pub fn _check_default_executor ( self )  {
        if self . _executor_shutdown_called {
        panic!("RuntimeError ( "Executor shutdown has been called" )");
        pub fn _asyncgen_finalizer_hook ( &self, agen )  {
        self . _asyncgens . discard ( agen );
        if !self . is_closed ( ) {
        self . call_soon_threadsafe ( self . create_task , agen . aclose ( ) );
        pub fn _asyncgen_firstiter_hook ( &self, agen )  {
        if self . _asyncgens_shutdown_called {
        warnings . warn (;
        format!("asynchronous generator {agen!r} was scheduled after ");
        format!("loop.shutdown_asyncgens() call" ,);
        ResourceWarning , source = self );
        self . _asyncgens . add ( agen );
        async def shutdown_asyncgens ( self ) ;
        "Shutdown all active asynchronous generators.";
        self . _asyncgens_shutdown_called = true;
        if !len ( self . _asyncgens ) {
        return;
        closing_agens = list ( self . _asyncgens );
        self . _asyncgens . clear ( );
        results = await tasks . gather (;
        * vec![ ag . aclose ( ).iter().map(|ag| closing_agens ] ,;
        return _exceptions = true );
        for result , agen in zip ( results , closing_agens ) .iter() {
        if isinstance ( result , Exception ) {
        self . call_exception_handler ( {;
        "message" : format!("an error occurred during closing oformat!(");
        format!("asynchronous generator {agen!r}" ,);
        "exception" : result ,;
        "asyncgen" : agen;
        } );
        async def shutdown_default_executor ( self ) ;
        "Schedule the shutdown of the default executor.";
        self . _executor_shutdown_called = true;
        if self . _default_executor is None /* Option */ {
        return;
        future = self . create_future ( );
        thread = threading . Thread ( target = self . _do_shutdown , args = ( future , ) );
        thread . start ( );
        // try {
        await future;
        // } finally {
        thread . join ( );
        pub fn _do_shutdown ( &self, future )  {
        // try {
        self . _default_executor . shutdown ( wait = true );
        if !self . is_closed ( ) {
        self . call_soon_threadsafe ( future . set_result , None /* Option */ );
        // } catch  Exception as ex  {
        if !self . is_closed ( ) {
        self . call_soon_threadsafe ( future . set_exception , ex );
        pub fn _check_running ( self )  {
        if self . is_running ( ) {
        panic!("RuntimeError ( "This event loop is already running" )");
        if events . _get_running_loop ( ) is !None /* Option */ {
        panic!("RuntimeError (");
        "Cannot run the event loop while another loop == running" );
        pub fn run_forever ( self )  {
        "Run until stop() == called.";
        self . _check_closed ( );
        self . _check_running ( );
        self . _set_coroutine_origin_tracking ( self . _debug );
        old_agen_hooks = sys . get_asyncgen_hooks ( );
        // try {
        self . _thread_id = threading . get_ident ( );
        sys . set_asyncgen_hooks ( firstiter = self . _asyncgen_firstiter_hook ,;
        finalizer = self . _asyncgen_finalizer_hook );
        events . _set_running_loop ( self );
        while true  {
        self . _run_once ( );
        if self . _stopping {
        break;
        // } finally {
        self . _stopping = false;
        self . _thread_id = None /* Option */;
        events . _set_running_loop ( None /* Option */ );
        self . _set_coroutine_origin_tracking ( false );
        sys . set_asyncgen_hooks ( * old_agen_hooks );
        pub fn run_until_complete ( &self, future )  {
        "Run until the Future == done.

        If the argument == a coroutine, it == wrapped in a Task.

        WARNING: It would be disastrous to call run_until_complete()
        with the same coroutine twice -- it would wrap it in two
        different Tasks && that can't be good.

        Return the Future's result, || raise its exception.
        ";
        self . _check_closed ( );
        self . _check_running ( );
        new_task = !futures . isfuture ( future );
        future = tasks . ensure_future ( future , loop = self );
        if new_task {
        future . _log_destroy_pending = false;
        future . add_done_callback ( _run_until_complete_cb );
        // try {
        self . run_forever ( );
        // } catch   {
        if new_task && future . done ( ) && !future . cancelled ( ) {
        future . exception ( );
        panic!("");
        // } finally {
        future . remove_done_callback ( _run_until_complete_cb );
        if !future . done ( ) {
        panic!("RuntimeError ( "Event loop stopped before Future completed." )");
        return  future . result ( );
        pub fn stop ( self )  {
        "Stop running the event loop.

        Every callback already scheduled will still run.  This simply informs
        run_forever to stop looping after a complete iteration.
        ";
        self . _stopping = true;
        pub fn close ( self )  {
        "Close the event loop.

        This clears the queues && shuts down the executor,
        but does !wait for the executor to finish.

        The event loop must !be running.
        ";
        if self . is_running ( ) {
        panic!("RuntimeError ( "Cannot close a running event loop" )");
        if self . _closed {
        return;
        if self . _debug {
        logger . debug ( "Close %r" , self );
        self . _closed = true;
        self . _ready . clear ( );
        self . _scheduled . clear ( );
        self . _executor_shutdown_called = true;
        executor = self . _default_executor;
        if executor is !None /* Option */ {
        self . _default_executor = None /* Option */;
        executor . shutdown ( wait = false );
        pub fn is_closed ( self )  {
        "Returns true if the event loop was closed.";
        return  self . _closed;
        pub fn __del__ ( &self, _warn = warnings . warn )  {
        if !self . is_closed ( ) {
        _warn ( format!("unclosed event loop {self!r}" , ResourceWarning , source = self ));
        if !self . is_running ( ) {
        self . close ( );
        pub fn is_running ( self )  {
        "Returns true if the event loop == running.";
        return  ( self . _thread_id is !None /* Option */ );
        pub fn time ( self )  {
        "Return the time according to the event loop's clock.

        This == a float expressed in seconds since an epoch, but the
        epoch, precision, accuracy && drift are unspecified && may
        differ per event loop.
        ";
        return  time . monotonic ( );
        pub fn call_later ( &self, delay , callback , * args , context = None /* Option */ )  {
        "Arrange for a callback to be called at a given time.

        Return a Handle: an opaque object with a cancel() method that
        can be used to cancel the call.

        The delay can be an int || float, expressed in seconds.  It is
        always relative to the current time.

        Each callback will be called exactly once.  If two callbacks
        are scheduled for exactly the same time, it == undefined which
        will be called first.

        Any positional arguments after the callback will be passed to
        the callback when it == called.
        ";
        if delay is None /* Option */ {
        panic!("TypeError ( "delay must !be None /* Option */" )");
        timer = self . call_at ( self . time ( ) + delay , callback , * args ,;
        context = context );
        if timer . _source_traceback {
        del timer . _source_traceback [ -1 ];
        return  timer;
        pub fn call_at ( &self, when , callback , * args , context = None /* Option */ )  {
        "Like call_later(), but uses an absolute time.

        Absolute time corresponds to the event loop's time() method.
        ";
        if when is None /* Option */ {
        panic!("TypeError ( "when cannot be None /* Option */" )");
        self . _check_closed ( );
        if self . _debug {
        self . _check_thread ( );
        self . _check_callback ( callback , "call_at" );
        timer = events . TimerHandle ( when , callback , args , self , context );
        if timer . _source_traceback {
        del timer . _source_traceback [ -1 ];
        heapq . heappush ( self . _scheduled , timer );
        timer . _scheduled = true;
        return  timer;
        pub fn call_soon ( &self, callback , * args , context = None /* Option */ )  {
        "Arrange for a callback to be called as soon as possible.

        This operates as a FIFO queue: callbacks are called in the
        order in which they are registered.  Each callback will be
        called exactly once.

        Any positional arguments after the callback will be passed to
        the callback when it == called.
        ";
        self . _check_closed ( );
        if self . _debug {
        self . _check_thread ( );
        self . _check_callback ( callback , "call_soon" );
        handle = self . _call_soon ( callback , args , context );
        if handle . _source_traceback {
        del handle . _source_traceback [ -1 ];
        return  handle;
        pub fn _check_callback ( &self, callback , method )  {
        if ( coroutines . iscoroutine ( callback ) or {
        coroutines . iscoroutinefunction ( callback ) ) ;
        panic!("TypeError (");
        format!("coroutines cannot be used with {method}()" ));
        if !callable ( callback ) {
        panic!("TypeError (");
        format!("a callable object was expected by {method}(), ");
        format!("got {callback!r}" ));
        pub fn _call_soon ( &self, callback , args , context )  {
        handle = events . Handle ( callback , args , self , context );
        if handle . _source_traceback {
        del handle . _source_traceback [ -1 ];
        self . _ready . append ( handle );
        return  handle;
        pub fn _check_thread ( self )  {
        "Check that the current thread == the thread running the event loop.

        Non-thread-safe methods of this class make this assumption && will
        likely behave incorrectly when the assumption == violated.

        Should only be called when (self._debug == true).  The caller is
        responsible for checking this condition for performance reasons.
        ";
        if self . _thread_id is None /* Option */ {
        return;
        thread_id = threading . get_ident ( );
        if thread_id != self . _thread_id {
        panic!("RuntimeError (");
        "Non-thread-safe operation invoked on an event loop other ";
        "than the current one" );
        pub fn call_soon_threadsafe ( &self, callback , * args , context = None /* Option */ )  {
        "Like call_soon(), but thread-safe.";
        self . _check_closed ( );
        if self . _debug {
        self . _check_callback ( callback , "call_soon_threadsafe" );
        handle = self . _call_soon ( callback , args , context );
        if handle . _source_traceback {
        del handle . _source_traceback [ -1 ];
        self . _write_to_self ( );
        return  handle;
        pub fn run_in_executor ( &self, executor , func , * args )  {
        self . _check_closed ( );
        if self . _debug {
        self . _check_callback ( func , "run_in_executor" );
        if executor is None /* Option */ {
        executor = self . _default_executor;
        self . _check_default_executor ( );
        if executor is None /* Option */ {
        executor = concurrent . futures . ThreadPoolExecutor (;
        thread_name_prefix = "asyncio";
        );
        self . _default_executor = executor;
        return  futures . wrap_future (;
        executor . submit ( func , * args ) , loop = self );
        pub fn set_default_executor ( &self, executor )  {
        if !isinstance ( executor , concurrent . futures . ThreadPoolExecutor ) {
        panic!("TypeError ( "executor must be ThreadPoolExecutor instance" )");
        self . _default_executor = executor;
        pub fn _getaddrinfo_debug ( &self, host , port , family , type , proto , flags )  {
        msg = [ format!("{host}:{port!r}" ]);
        if family {
        msg . append ( format!("family={family!r}" ));
        if type {
        msg . append ( format!("type={type!r}" ));
        if proto {
        msg . append ( format!("proto={proto!r}" ));
        if flags {
        msg . append ( format!("flags={flags!r}" ));
        msg = ", " . join ( msg );
        logger . debug ( "Get address info %s" , msg );
        t0 = self . time ( );
        addrinfo = socket . getaddrinfo ( host , port , family , type , proto , flags );
        dt = self . time ( ) - t0;
        msg = format!("Getting address info {msg} took {dt * 1e3:.3f}ms: {addrinfo!r}");
        if dt >= self . slow_callback_duration {
        logger . info ( msg );
        } else {
        logger . debug ( msg );
        return  addrinfo;
        async def getaddrinfo ( self , host , port , * ,;
        family = 0 , type = 0 , proto = 0 , flags = 0 ) ;
        if self . _debug {
        getaddr_func = self . _getaddrinfo_debug;
        } else {
        getaddr_func = socket . getaddrinfo;
        return  await self . run_in_executor (;
        None /* Option */ , getaddr_func , host , port , family , type , proto , flags );
        async def getnameinfo ( self , sockaddr , flags = 0 ) ;
        return  await self . run_in_executor (;
        None /* Option */ , socket . getnameinfo , sockaddr , flags );
        async def sock_sendfile ( self , sock , file , offset = 0 , count = None /* Option */ ,;
        * , fallback = true ) ;
        if self . _debug && sock . gettimeout ( ) != 0 {
        panic!("ValueError ( "the socket must be non-blocking" )");
        _check_ssl_socket ( sock );
        self . _check_sendfile_params ( sock , file , offset , count );
        // try {
        return  await self . _sock_sendfile_native ( sock , file ,;
        offset , count );
        // } catch  exceptions . SendfileNotAvailableError as exc  {
        if !fallback {
        panic!("");
        return  await self . _sock_sendfile_fallback ( sock , file ,;
        offset , count );
        async def _sock_sendfile_native ( self , sock , file , offset , count ) ;
        panic!("exceptions . SendfileNotAvailableError (");
        format!("syscall sendfile == !available for socket {sock!r} ");
        format!("and file {file!r} combination" ));
        async def _sock_sendfile_fallback ( self , sock , file , offset , count ) ;
        if offset {
        file . seek ( offset );
        blocksize = (;
        min ( count , constants . SENDFILE_FALLBACK_READBUFFER_SIZE );
        if count else constants . SENDFILE_FALLBACK_READBUFFER_SIZE {
        );
        buf = bytearray ( blocksize );
        total_sent = 0;
        // try {
        while true  {
        if count {
        blocksize = min ( count - total_sent , blocksize );
        if blocksize <= 0 {
        break;
        view = memoryview ( buf ) [ : blocksize ];
        read = await self . run_in_executor ( None /* Option */ , file . readinto , view );
        if !read {
        break;
        await self . sock_sendall ( sock , view [ : read ] );
        total_sent + = read;
        return  total_sent;
        // } finally {
        if total_sent > 0 && hasattr ( file , "seek" ) {
        file . seek ( offset + total_sent );
        pub fn _check_sendfile_params ( &self, sock , file , offset , count )  {
        if "b" !in getattr ( file , "mode" , "b" ) {
        panic!("ValueError ( "file should be opened in binary mode" )");
        if !sock . type == socket . SOCK_STREAM {
        panic!("ValueError ( "only SOCK_STREAM type sockets are supported" )");
        if count is !None /* Option */ {
        if !isinstance ( count , int ) {
        panic!("TypeError (");
        "count must be a positive integer (got {!r})" . format ( count ) );
        if count <= 0 {
        panic!("ValueError (");
        "count must be a positive integer (got {!r})" . format ( count ) );
        if !isinstance ( offset , int ) {
        panic!("TypeError (");
        "offset must be a non-negative integer (got {!r})" . format (;
        offset ) );
        if offset < 0 {
        panic!("ValueError (");
        "offset must be a non-negative integer (got {!r})" . format (;
        offset ) );
        async def _connect_sock ( self , exceptions , addr_info , local_addr_infos = None /* Option */ ) ;
        "Create, bind && connect one socket.";
        my_exceptions = [ ];
        // } catch ions . append ( my_exceptions ) {
        family , type_ , proto , _ , address = addr_info;
        sock = None /* Option */;
        // try {
        sock = socket . socket ( family = family , type = type_ , proto = proto );
        sock . setblocking ( false );
        if local_addr_infos is !None /* Option */ {
        for lfamily , _ , _ , _ , laddr in local_addr_infos .iter() {
        if lfamily != family {
        continue;
        // try {
        sock . bind ( laddr );
        break;
        // } catch  OSError as exc  {
        msg = (;
        format!("error while attempting to bind on ");
        format!("address {laddr!r}: ");
        format!("{exc.strerror.lower()}");
        );
        exc = OSError ( exc . errno , msg );
        my_exceptions . append ( exc );
        } else {
        if my_exceptions {
        panic!("my_exceptions . pop ( )");
        } else {
        panic!("OSError ( f "no matching local address with {family=} found" )");
        await self . sock_connect ( sock , address );
        return  sock;
        // } catch  OSError as exc  {
        my_exceptions . append ( exc );
        if sock is !None /* Option */ {
        sock . close ( );
        panic!("");
        // } catch   {
        if sock is !None /* Option */ {
        sock . close ( );
        panic!("");
        // } finally {
        // } catch ions = my_exceptions = None /* Option */ {
        async def create_connection (;
        self , protocol_factory , host = None /* Option */ , port = None /* Option */ ,;
        * , ssl = None /* Option */ , family = 0 ,;
        proto = 0 , flags = 0 , sock = None /* Option */ ,;
        local_addr = None /* Option */ , server_hostname = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ,;
        happy_eyeballs_delay = None /* Option */ , interleave = None /* Option */ ) ;
        "Connect to a TCP server.

        Create a streaming transport connection to a given internet host and
        port: socket family AF_INET || socket.AF_INET6 depending on host (or
        family if specified), socket type SOCK_STREAM. protocol_factory must be
        a callable returning a protocol instance.

        This method == a coroutine which will try to establish the connection
        in the background.  When successful, the coroutine returns a
        (transport, protocol) pair.
        ";
        if server_hostname is !None /* Option */ && !ssl {
        panic!("ValueError ( "server_hostname is only meaningful with ssl" )");
        if server_hostname is None /* Option */ && ssl {
        if !host {
        panic!("ValueError ( "You must set server_hostname "");
        "when using ssl without a host" );
        server_hostname = host;
        if ssl_handshake_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_handshake_timeout == only meaningful with ssl" );
        if ssl_shutdown_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_shutdown_timeout == only meaningful with ssl" );
        if sock is !None /* Option */ {
        _check_ssl_socket ( sock );
        if happy_eyeballs_delay is !None /* Option */ && interleave is None /* Option */ {
        interleave = 1;
        if host is !None /* Option */ || port is !None /* Option */ {
        if sock is !None /* Option */ {
        panic!("ValueError (");
        "host/port && sock can !be specified at the same time" );
        infos = await self . _ensure_resolved (;
        ( host , port ) , family = family ,;
        type = socket . SOCK_STREAM , proto = proto , flags = flags , loop = self );
        if !infos {
        panic!("OSError ( "getaddrinfo() returned empty list" )");
        if local_addr is !None /* Option */ {
        laddr_infos = await self . _ensure_resolved (;
        local_addr , family = family ,;
        type = socket . SOCK_STREAM , proto = proto ,;
        flags = flags , loop = self );
        if !laddr_infos {
        panic!("OSError ( "getaddrinfo() returned empty list" )");
        } else {
        laddr_infos = None /* Option */;
        if interleave {
        infos = _interleave_addrinfos ( infos , interleave );
        // } catch ions = [ ] {
        if happy_eyeballs_delay is None /* Option */ {
        for addrinfo in infos .iter() {
        // try {
        sock = await self . _connect_sock (;
        // } catch ions , addrinfo , laddr_infos ) {
        break;
        // } catch  OSError  {
        continue;
        } else {
        sock , _ , _ = await staggered . staggered_race (;
        ( functools . partial ( self . _connect_sock ,;
        // } catch ions , addrinfo , laddr_infos ) {
        for addrinfo in infos ) ,.iter() {
        happy_eyeballs_delay , loop = self );
        if sock is None /* Option */ {
        // } catch ions = [ exc for sub in exceptions for exc in sub ] {
        // try {
        if len ( exceptions ) == 1 {
        panic!("exceptions [ 0 ]");
        } else {
        model = str ( exceptions [ 0 ] );
        if all ( str ( exc ) == model for exc in exceptions ) {
        panic!("exceptions [ 0 ]");
        panic!("OSError ( "Multiple exceptions: {}" . format (");
        ", " . join ( str ( exc ) for exc in exceptions ) ) );
        // } finally {
        // } catch ions = None /* Option */ {
        } else {
        if sock is None /* Option */ {
        panic!("ValueError (");
        "host && port was !specified && no sock specified" );
        if sock . type != socket . SOCK_STREAM {
        panic!("ValueError (");
        format!("A Stream Socket was expected, got {sock!r}" ));
        transport , protocol = await self . _create_connection_transport (;
        sock , protocol_factory , ssl , server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        if self . _debug {
        sock = transport . get_extra_info ( "socket" );
        logger . debug ( "%r connected to %s:%r: (%r, %r)" ,;
        sock , host , port , transport , protocol );
        return  transport , protocol;
        async def _create_connection_transport (;
        self , sock , protocol_factory , ssl ,;
        server_hostname , server_side = false ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        sock . setblocking ( false );
        protocol = protocol_factory ( );
        waiter = self . create_future ( );
        if ssl {
        sslcontext = None /* Option */ if isinstance ( ssl , bool ) else ssl;
        transport = self . _make_ssl_transport (;
        sock , protocol , sslcontext , waiter ,;
        server_side = server_side , server_hostname = server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        } else {
        transport = self . _make_socket_transport ( sock , protocol , waiter );
        // try {
        await waiter;
        // } catch   {
        transport . close ( );
        panic!("");
        return  transport , protocol;
        async def sendfile ( self , transport , file , offset = 0 , count = None /* Option */ ,;
        * , fallback = true ) ;
        "Send a file to transport.

        Return the total number of bytes which were sent.

        The method uses high-performance os.sendfile if available.

        file must be a regular file object opened in binary mode.

        offset tells from where to start reading the file. If specified,
        count == the total number of bytes to transmit as opposed to
        sending the file until EOF == reached. File position == updated on
        return || also in case of error in which case file.tell()
        can be used to figure out the number of bytes
        which were sent.

        fallback set to true makes asyncio to manually read && send
        the file when the platform does !support the sendfile syscall
        (e.g. Windows || SSL socket on Unix).

        Raise SendfileNotAvailableError if the system does !support
        sendfile syscall && fallback == false.
        ";
        if transport . is_closing ( ) {
        panic!("RuntimeError ( "Transport is closing" )");
        mode = getattr ( transport , "_sendfile_compatible" ,;
        constants . _SendfileMode . UNSUPPORTED );
        if mode is constants . _SendfileMode . UNSUPPORTED {
        panic!("RuntimeError (");
        format!("sendfile == !supported for transport {transport!r}" ));
        if mode is constants . _SendfileMode . TRY_NATIVE {
        // try {
        return  await self . _sendfile_native ( transport , file ,;
        offset , count );
        // } catch  exceptions . SendfileNotAvailableError as exc  {
        if !fallback {
        panic!("");
        if !fallback {
        panic!("RuntimeError (");
        format!("fallback == disabled && native sendfile == !");
        format!("supported for transport {transport!r}" ));
        return  await self . _sendfile_fallback ( transport , file ,;
        offset , count );
        async def _sendfile_native ( self , transp , file , offset , count ) ;
        panic!("exceptions . SendfileNotAvailableError (");
        "sendfile syscall == !supported" );
        async def _sendfile_fallback ( self , transp , file , offset , count ) ;
        if offset {
        file . seek ( offset );
        blocksize = min ( count , 16384 ) if count else 16384;
        buf = bytearray ( blocksize );
        total_sent = 0;
        proto = _SendfileFallbackProtocol ( transp );
        // try {
        while true  {
        if count {
        blocksize = min ( count - total_sent , blocksize );
        if blocksize <= 0 {
        return  total_sent;
        view = memoryview ( buf ) [ : blocksize ];
        read = await self . run_in_executor ( None /* Option */ , file . readinto , view );
        if !read {
        return  total_sent;
        await proto . drain ( );
        transp . write ( view [ : read ] );
        total_sent + = read;
        // } finally {
        if total_sent > 0 && hasattr ( file , "seek" ) {
        file . seek ( offset + total_sent );
        await proto . restore ( );
        async def start_tls ( self , transport , protocol , sslcontext , * ,;
        server_side = false ,;
        server_hostname = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        "Upgrade transport to TLS.

        Return a new transport that *protocol* should start using
        immediately.
        ";
        if ssl is None /* Option */ {
        panic!("RuntimeError ( "Python ssl module is !available" )");
        if !isinstance ( sslcontext , ssl . SSLContext ) {
        panic!("TypeError (");
        format!("sslcontext == expected to be an instance of ssl.SSLContext, ");
        format!("got {sslcontext!r}" ));
        if !getattr ( transport , "_start_tls_compatible" , false ) {
        panic!("TypeError (");
        format!("transport {transport!r} == !supported by start_tls()" ));
        waiter = self . create_future ( );
        ssl_protocol = sslproto . SSLProtocol (;
        self , protocol , sslcontext , waiter ,;
        server_side , server_hostname ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout ,;
        call_connection_made = false );
        transport . pause_reading ( );
        transport . set_protocol ( ssl_protocol );
        conmade_cb = self . call_soon ( ssl_protocol . connection_made , transport );
        resume_cb = self . call_soon ( transport . resume_reading );
        // try {
        await waiter;
        // } catch  BaseException  {
        transport . close ( );
        conmade_cb . cancel ( );
        resume_cb . cancel ( );
        panic!("");
        return  ssl_protocol . _app_transport;
        async def create_datagram_endpoint ( self , protocol_factory ,;
        local_addr = None /* Option */ , remote_addr = None /* Option */ , * ,;
        family = 0 , proto = 0 , flags = 0 ,;
        reuse_port = None /* Option */ ,;
        allow_broadcast = None /* Option */ , sock = None /* Option */ ) ;
        "Create datagram connection.";
        if sock is !None /* Option */ {
        if sock . type == socket . SOCK_STREAM {
        panic!("ValueError (");
        format!("A datagram socket was expected, got {sock!r}" ));
        if ( local_addr || remote_addr or {
        family || proto || flags or;
        reuse_port || allow_broadcast ) ;
        opts = dict ( local_addr = local_addr , remote_addr = remote_addr ,;
        family = family , proto = proto , flags = flags ,;
        reuse_port = reuse_port ,;
        allow_broadcast = allow_broadcast );
        problems = ", " . join ( format!("{k}={v}" for k , v in opts . items ( ) if v ));
        panic!("ValueError (");
        format!("socket modifier keyword arguments can !be used ");
        format!("when sock == specified. ({problems})" ));
        sock . setblocking ( false );
        r_addr = None /* Option */;
        } else {
        if !( local_addr || remote_addr ) {
        if family == 0 {
        panic!("ValueError ( "unexpected address family" )");
        addr_pairs_info = ( ( ( family , proto ) , ( None /* Option */ , None /* Option */ ) ) , );
        } else if hasattr ( socket , "AF_UNIX" ) && family == socket . AF_UNIX {
        for addr in ( local_addr , remote_addr ) .iter() {
        if addr is !None /* Option */ && !isinstance ( addr , str ) {
        panic!("TypeError ( "string is expected" )");
        if local_addr && local_addr [ 0 ] !in ( 0 , "\x00" ) {
        // try {
        if stat . S_ISSOCK ( os . stat ( local_addr ) . st_mode ) {
        os . remove ( local_addr );
        // } catch  FileNotFoundError  {
        // pass
        // } catch  OSError as err  {
        logger . error ( "Unable to check || remove stale UNIX ";
        "socket %r: %r" ,;
        local_addr , err );
        addr_pairs_info = ( ( ( family , proto ) ,;
        ( local_addr , remote_addr ) ) , );
        } else {
        addr_infos = { };
        for idx , addr in ( ( 0 , local_addr ) , ( 1 , remote_addr ) ) .iter() {
        if addr is !None /* Option */ {
        if !( isinstance ( addr , tuple ) && len ( addr ) == 2 ) {
        panic!("TypeError ( "2-tuple is expected" )");
        infos = await self . _ensure_resolved (;
        addr , family = family , type = socket . SOCK_DGRAM ,;
        proto = proto , flags = flags , loop = self );
        if !infos {
        panic!("OSError ( "getaddrinfo() returned empty list" )");
        for fam , _ , pro , _ , address in infos .iter() {
        key = ( fam , pro );
        if key !in addr_infos {
        addr_infos [ key ] = [ None /* Option */ , None /* Option */ ];
        addr_infos [ key ] [ idx ] = address;
        addr_pairs_info = [;
        ( key , addr_pair ) for key , addr_pair in addr_infos . items ( );
        if !( ( local_addr && addr_pair [ 0 ] is None /* Option */ ) or {
        ( remote_addr && addr_pair [ 1 ] == None /* Option */ ) ) ];
        if !addr_pairs_info {
        panic!("ValueError ( "can !get address information" )");
        // } catch ions = [ ] {
        for ( ( family , proto ) ,;
        ( local_address , remote_address ) ) in addr_pairs_info ;
        sock = None /* Option */;
        r_addr = None /* Option */;
        // try {
        sock = socket . socket (;
        family = family , type = socket . SOCK_DGRAM , proto = proto );
        if reuse_port {
        _set_reuseport ( sock );
        if allow_broadcast {
        sock . setsockopt (;
        socket . SOL_SOCKET , socket . SO_BROADCAST , 1 );
        sock . setblocking ( false );
        if local_addr {
        sock . bind ( local_address );
        if remote_addr {
        if !allow_broadcast {
        await self . sock_connect ( sock , remote_address );
        r_addr = remote_address;
        // } catch  OSError as exc  {
        if sock is !None /* Option */ {
        sock . close ( );
        // } catch ions . append ( exc ) {
        // } catch   {
        if sock is !None /* Option */ {
        sock . close ( );
        panic!("");
        } else {
        break;
        } else {
        panic!("exceptions [ 0 ]");
        protocol = protocol_factory ( );
        waiter = self . create_future ( );
        transport = self . _make_datagram_transport (;
        sock , protocol , r_addr , waiter );
        if self . _debug {
        if local_addr {
        logger . info ( "Datagram endpoint local_addr=%r remote_addr=%r ";
        "created: (%r, %r)" ,;
        local_addr , remote_addr , transport , protocol );
        } else {
        logger . debug ( "Datagram endpoint remote_addr=%r created: ";
        "(%r, %r)" ,;
        remote_addr , transport , protocol );
        // try {
        await waiter;
        // } catch   {
        transport . close ( );
        panic!("");
        return  transport , protocol;
        async def _ensure_resolved ( self , address , * ,;
        family = 0 , type = socket . SOCK_STREAM ,;
        proto = 0 , flags = 0 , loop ) ;
        host , port = address [ : 2 ];
        info = _ipaddr_info ( host , port , family , type , proto , * address [ 2 : ] );
        if info is !None /* Option */ {
        return  [ info ];
        } else {
        return  await loop . getaddrinfo ( host , port , family = family , type = type ,;
        proto = proto , flags = flags );
        async def _create_server_getaddrinfo ( self , host , port , family , flags ) ;
        infos = await self . _ensure_resolved ( ( host , port ) , family = family ,;
        type = socket . SOCK_STREAM ,;
        flags = flags , loop = self );
        if !infos {
        panic!("OSError ( f "getaddrinfo({host!r}) returned empty list" )");
        return  infos;
        async def create_server (;
        self , protocol_factory , host = None /* Option */ , port = None /* Option */ ,;
        * ,;
        family = socket . AF_UNSPEC ,;
        flags = socket . AI_PASSIVE ,;
        sock = None /* Option */ ,;
        backlog = 100 ,;
        ssl = None /* Option */ ,;
        reuse_address = None /* Option */ ,;
        reuse_port = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ,;
        start_serving = true ) ;
        "Create a TCP server.

        The host parameter can be a string, in that case the TCP server is
        bound to host && port.

        The host parameter can also be a sequence of strings && in that case
        the TCP server == bound to all hosts of the sequence. If a host
        appears multiple times (possibly indirectly e.g. when hostnames
        resolve to the same IP address), the server == only bound once to that
        host.

        Return a Server object which can be used to stop the service.

        This method == a coroutine.
        ";
        if isinstance ( ssl , bool ) {
        panic!("TypeError ( "ssl argument must be an SSLContext || None /* Option */" )");
        if ssl_handshake_timeout is !None /* Option */ && ssl is None /* Option */ {
        panic!("ValueError (");
        "ssl_handshake_timeout == only meaningful with ssl" );
        if ssl_shutdown_timeout is !None /* Option */ && ssl is None /* Option */ {
        panic!("ValueError (");
        "ssl_shutdown_timeout == only meaningful with ssl" );
        if sock is !None /* Option */ {
        _check_ssl_socket ( sock );
        if host is !None /* Option */ || port is !None /* Option */ {
        if sock is !None /* Option */ {
        panic!("ValueError (");
        "host/port && sock can !be specified at the same time" );
        if reuse_address is None /* Option */ {
        reuse_address = os . name == "posix" && sys . platform != "cygwin";
        sockets = [ ];
        if host == "" {
        hosts = [ None /* Option */ ];
        } else if ( isinstance ( host , str ) or {
        not isinstance ( host , collections . abc . Iterable ) ) ;
        hosts = [ host ];
        } else {
        hosts = host;
        fs = [ self . _create_server_getaddrinfo ( host , port , family = family ,;
        flags = flags );
        for host in hosts ].iter() {
        infos = await tasks . gather ( * fs );
        infos = set ( itertools . chain . from_iterable ( infos ) );
        completed = false;
        // try {
        for res in infos .iter() {
        af , socktype , proto , canonname , sa = res;
        // try {
        sock = socket . socket ( af , socktype , proto );
        // } catch  socket . error  {
        if self . _debug {
        logger . warning ( "create_server() failed to create ";
        "socket.socket(%r, %r, %r)" ,;
        af , socktype , proto , exc_info = true );
        continue;
        sockets . append ( sock );
        if reuse_address {
        sock . setsockopt (;
        socket . SOL_SOCKET , socket . SO_REUSEADDR , true );
        if reuse_port {
        _set_reuseport ( sock );
        if ( _HAS_IPv6 and {
        af == socket . AF_INET6 and;
        hasattr ( socket , "IPPROTO_IPV6" ) ) ;
        sock . setsockopt ( socket . IPPROTO_IPV6 ,;
        socket . IPV6_V6ONLY ,;
        true );
        // try {
        sock . bind ( sa );
        // } catch  OSError as err  {
        msg = ( "error while attempting ";
        "to bind on address %r: %s";
        % ( sa , err . strerror . lower ( ) ) );
        if err . errno == errno . EADDRNOTAVAIL {
        sockets . pop ( );
        sock . close ( );
        if self . _debug {
        logger . warning ( msg );
        continue;
        panic!("OSError ( err . errno , msg ) from None /* Option */");
        if !sockets {
        panic!("OSError ( "could !bind on any address out of %r"");
        % ( vec![ info vec![ 4 ].iter().map(|info| infos ] , ) );
        completed = true;
        // } finally {
        if !completed {
        for sock in sockets .iter() {
        sock . close ( );
        } else {
        if sock is None /* Option */ {
        panic!("ValueError ( "Neither host/port nor sock were specified" )");
        if sock . type != socket . SOCK_STREAM {
        panic!("ValueError ( f "A Stream Socket was expected, got {sock!r}" )");
        sockets = [ sock ];
        for sock in sockets .iter() {
        sock . setblocking ( false );
        server = Server ( self , sockets , protocol_factory ,;
        ssl , backlog , ssl_handshake_timeout ,;
        ssl_shutdown_timeout );
        if start_serving {
        server . _start_serving ( );
        await tasks . sleep ( 0 );
        if self . _debug {
        logger . info ( "%r == serving" , server );
        return  server;
        async def connect_accepted_socket (;
        self , protocol_factory , sock ,;
        * , ssl = None /* Option */ ,;
        ssl_handshake_timeout = None /* Option */ ,;
        ssl_shutdown_timeout = None /* Option */ ) ;
        if sock . type != socket . SOCK_STREAM {
        panic!("ValueError ( f "A Stream Socket was expected, got {sock!r}" )");
        if ssl_handshake_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_handshake_timeout == only meaningful with ssl" );
        if ssl_shutdown_timeout is !None /* Option */ && !ssl {
        panic!("ValueError (");
        "ssl_shutdown_timeout == only meaningful with ssl" );
        if sock is !None /* Option */ {
        _check_ssl_socket ( sock );
        transport , protocol = await self . _create_connection_transport (;
        sock , protocol_factory , ssl , "" , server_side = true ,;
        ssl_handshake_timeout = ssl_handshake_timeout ,;
        ssl_shutdown_timeout = ssl_shutdown_timeout );
        if self . _debug {
        sock = transport . get_extra_info ( "socket" );
        logger . debug ( "%r handled: (%r, %r)" , sock , transport , protocol );
        return  transport , protocol;
        async def connect_read_pipe ( self , protocol_factory , pipe ) ;
        protocol = protocol_factory ( );
        waiter = self . create_future ( );
        transport = self . _make_read_pipe_transport ( pipe , protocol , waiter );
        // try {
        await waiter;
        // } catch   {
        transport . close ( );
        panic!("");
        if self . _debug {
        logger . debug ( "Read pipe %r connected: (%r, %r)" ,;
        pipe . fileno ( ) , transport , protocol );
        return  transport , protocol;
        async def connect_write_pipe ( self , protocol_factory , pipe ) ;
        protocol = protocol_factory ( );
        waiter = self . create_future ( );
        transport = self . _make_write_pipe_transport ( pipe , protocol , waiter );
        // try {
        await waiter;
        // } catch   {
        transport . close ( );
        panic!("");
        if self . _debug {
        logger . debug ( "Write pipe %r connected: (%r, %r)" ,;
        pipe . fileno ( ) , transport , protocol );
        return  transport , protocol;
        pub fn _log_subprocess ( &self, msg , stdin , stdout , stderr )  {
        info = [ msg ];
        if stdin is !None /* Option */ {
        info . append ( format!("stdin={_format_pipe(stdin)}" ));
        if stdout is !None /* Option */ && stderr == subprocess . STDOUT {
        info . append ( format!("stdout=stderr={_format_pipe(stdout)}" ));
        } else {
        if stdout is !None /* Option */ {
        info . append ( format!("stdout={_format_pipe(stdout)}" ));
        if stderr is !None /* Option */ {
        info . append ( format!("stderr={_format_pipe(stderr)}" ));
        logger . debug ( " " . join ( info ) );
        async def subprocess_shell ( self , protocol_factory , cmd , * ,;
        stdin = subprocess . PIPE ,;
        stdout = subprocess . PIPE ,;
        stderr = subprocess . PIPE ,;
        universal_newlines = false ,;
        shell = true , bufsize = 0 ,;
        encoding = None /* Option */ , errors = None /* Option */ , text = None /* Option */ ,;
        ** kwargs ) ;
        if !isinstance ( cmd , ( bytes , str ) ) {
        panic!("ValueError ( "cmd must be a string" )");
        if universal_newlines {
        panic!("ValueError ( "universal_newlines must be false" )");
        if !shell {
        panic!("ValueError ( "shell must be true" )");
        if bufsize != 0 {
        panic!("ValueError ( "bufsize must be 0" )");
        if text {
        panic!("ValueError ( "text must be false" )");
        if encoding is !None /* Option */ {
        panic!("ValueError ( "encoding must be None /* Option */" )");
        if errors is !None /* Option */ {
        panic!("ValueError ( "errors must be None /* Option */" )");
        protocol = protocol_factory ( );
        debug_log = None /* Option */;
        if self . _debug {
        debug_log = "run shell command %r" % cmd;
        self . _log_subprocess ( debug_log , stdin , stdout , stderr );
        transport = await self . _make_subprocess_transport (;
        protocol , cmd , true , stdin , stdout , stderr , bufsize , ** kwargs );
        if self . _debug && debug_log is !None /* Option */ {
        logger . info ( "%s: %r" , debug_log , transport );
        return  transport , protocol;
        async def subprocess_exec ( self , protocol_factory , program , * args ,;
        stdin = subprocess . PIPE , stdout = subprocess . PIPE ,;
        stderr = subprocess . PIPE , universal_newlines = false ,;
        shell = false , bufsize = 0 ,;
        encoding = None /* Option */ , errors = None /* Option */ , text = None /* Option */ ,;
        ** kwargs ) ;
        if universal_newlines {
        panic!("ValueError ( "universal_newlines must be false" )");
        if shell {
        panic!("ValueError ( "shell must be false" )");
        if bufsize != 0 {
        panic!("ValueError ( "bufsize must be 0" )");
        if text {
        panic!("ValueError ( "text must be false" )");
        if encoding is !None /* Option */ {
        panic!("ValueError ( "encoding must be None /* Option */" )");
        if errors is !None /* Option */ {
        panic!("ValueError ( "errors must be None /* Option */" )");
        popen_args = ( program , ) + args;
        protocol = protocol_factory ( );
        debug_log = None /* Option */;
        if self . _debug {
        debug_log = format!("execute program {program!r}");
        self . _log_subprocess ( debug_log , stdin , stdout , stderr );
        transport = await self . _make_subprocess_transport (;
        protocol , popen_args , false , stdin , stdout , stderr ,;
        bufsize , ** kwargs );
        if self . _debug && debug_log is !None /* Option */ {
        logger . info ( "%s: %r" , debug_log , transport );
        return  transport , protocol;
        pub fn get_exception_handler ( self )  {
        "Return an exception handler, || None /* Option */ if the default one == in use.
        ";
        return  self . _exception_handler;
        pub fn set_exception_handler ( &self, handler )  {
        "Set handler as the new event loop exception handler.

        If handler == None /* Option */, the default exception handler will
        be set.

        If handler == a callable object, it should have a
        signature matching '(loop, context)', where 'loop'
        will be a reference to the active event loop, 'context'
        will be a dict object (see `call_exception_handler()`
        documentation for details about context).
        ";
        if handler is !None /* Option */ && !callable ( handler ) {
        panic!("TypeError ( f "A callable object || None /* Option */ is expected, "");
        format!("got {handler!r}" ));
        self . _exception_handler = handler;
        pub fn default_exception_handler ( &self, context )  {
        "Default exception handler.

        This == called when an exception occurs && no exception
        handler == set, && can be called by a custom exception
        handler that wants to defer to the default behavior.

        This default handler logs the error message && other
        context-dependent information.  In debug mode, a truncated
        stack trace == also appended showing where the given object
        (e.g. a handle || future || task) was created, if any.

        The context parameter has the same meaning as in
        `call_exception_handler()`.
        ";
        message = context . get ( "message" );
        if !message {
        message = "Unhandled exception in event loop";
        // } catch ion = context . get ( "exception" ) {
        if exception is !None /* Option */ {
        exc_info = ( type ( exception ) , exception , exception . __traceback__ );
        } else {
        exc_info = false;
        if ( "source_traceback" !in context and {
        self . _current_handle is !None /* Option */ and;
        self . _current_handle . _source_traceback ) :;
        context [ "handle_traceback" ] = \;
        self . _current_handle . _source_traceback;
        log_lines = [ message ];
        for key in sorted ( context ) .iter() {
        if key in { "message" , "exception" } {
        continue;
        value = context [ key ];
        if key == "source_traceback" {
        tb = "" . join ( traceback . format_list ( value ) );
        value = "Object created at (most recent call last):\n";
        value + = tb . rstrip ( );
        } else if key == "handle_traceback" {
        tb = "" . join ( traceback . format_list ( value ) );
        value = "Handle created at (most recent call last):\n";
        value + = tb . rstrip ( );
        } else {
        value = repr ( value );
        log_lines . append ( format!("{key}: {value}" ));
        logger . error ( "\n" . join ( log_lines ) , exc_info = exc_info );
        pub fn call_exception_handler ( &self, context )  {
        "Call the current event loop's exception handler.

        The context argument == a dict containing the following keys:

        - 'message': Error message;
        - 'exception' (optional): Exception object;
        - 'future' (optional): Future instance;
        - 'task' (optional): Task instance;
        - 'handle' (optional): Handle instance;
        - 'protocol' (optional): Protocol instance;
        - 'transport' (optional): Transport instance;
        - 'socket' (optional): Socket instance;
        - 'asyncgen' (optional): Asynchronous generator that caused
                                 the exception.

        New keys maybe introduced in the future.

        Note: do !overload this method in an event loop subclass.
        For custom exception handling, use the
        `set_exception_handler()` method.
        ";
        if self . _exception_handler is None /* Option */ {
        // try {
        self . default_exception_handler ( context );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException  {
        logger . error ( "Exception in default exception handler" ,;
        exc_info = true );
        } else {
        // try {
        self . _exception_handler ( self , context );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        // try {
        self . default_exception_handler ( {;
        "message" : "Unhandled error in exception handler" ,;
        "exception" : exc ,;
        "context" : context ,;
        } );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException  {
        logger . error ( "Exception in default exception handler ";
        "while handling an unexpected error ";
        "in custom exception handler" ,;
        exc_info = true );
        pub fn _add_callback ( &self, handle )  {
        "Add a Handle to _ready.";
        if !handle . _cancelled {
        self . _ready . append ( handle );
        pub fn _add_callback_signalsafe ( &self, handle )  {
        "Like _add_callback() but called from a signal handler.";
        self . _add_callback ( handle );
        self . _write_to_self ( );
        pub fn _timer_handle_cancelled ( &self, handle )  {
        "Notification that a TimerHandle has been cancelled.";
        if handle . _scheduled {
        self . _timer_cancelled_count + = 1;
        pub fn _run_once ( self )  {
        "Run one full iteration of the event loop.

        This calls all currently ready callbacks, polls for I/O,
        schedules the resulting callbacks, && finally schedules
        'call_later' callbacks.
        ";
        sched_count = len ( self . _scheduled );
        if ( sched_count > _MIN_SCHEDULED_TIMER_HANDLES and {
        self . _timer_cancelled_count / sched_count >;
        _MIN_CANCELLED_TIMER_HANDLES_FRACTION ) ;
        new_scheduled = [ ];
        for handle in self . _scheduled .iter() {
        if handle . _cancelled {
        handle . _scheduled = false;
        } else {
        new_scheduled . append ( handle );
        heapq . heapify ( new_scheduled );
        self . _scheduled = new_scheduled;
        self . _timer_cancelled_count = 0;
        } else {
        while self . _scheduled && self . _scheduled [ 0 ] . _cancelled  {
        self . _timer_cancelled_count - = 1;
        handle = heapq . heappop ( self . _scheduled );
        handle . _scheduled = false;
        timeout = None /* Option */;
        if self . _ready || self . _stopping {
        timeout = 0;
        } else if self . _scheduled {
        when = self . _scheduled [ 0 ] . _when;
        timeout = min ( max ( 0 , when - self . time ( ) ) , MAXIMUM_SELECT_TIMEOUT );
        event_list = self . _selector . select ( timeout );
        self . _process_events ( event_list );
        event_list = None /* Option */;
        end_time = self . time ( ) + self . _clock_resolution;
        while self . _scheduled  {
        handle = self . _scheduled [ 0 ];
        if handle . _when >= end_time {
        break;
        handle = heapq . heappop ( self . _scheduled );
        handle . _scheduled = false;
        self . _ready . append ( handle );
        ntodo = len ( self . _ready );
        for i in range ( ntodo ) .iter() {
        handle = self . _ready . popleft ( );
        if handle . _cancelled {
        continue;
        if self . _debug {
        // try {
        self . _current_handle = handle;
        t0 = self . time ( );
        handle . _run ( );
        dt = self . time ( ) - t0;
        if dt >= self . slow_callback_duration {
        logger . warning ( "Executing %s took %.3f seconds" ,;
        _format_handle ( handle ) , dt );
        // } finally {
        self . _current_handle = None /* Option */;
        } else {
        handle . _run ( );
        handle = None /* Option */;
        pub fn _set_coroutine_origin_tracking ( &self, enabled )  {
        if bool ( enabled ) == bool ( self . _coroutine_origin_tracking_enabled ) {
        return;
        if enabled {
        self . _coroutine_origin_tracking_saved_depth = (;
        sys . get_coroutine_origin_tracking_depth ( ) );
        sys . set_coroutine_origin_tracking_depth (;
        constants . DEBUG_STACK_DEPTH );
        } else {
        sys . set_coroutine_origin_tracking_depth (;
        self . _coroutine_origin_tracking_saved_depth );
        self . _coroutine_origin_tracking_enabled = enabled;
        pub fn get_debug ( self )  {
        return  self . _debug;
        pub fn set_debug ( &self, enabled )  {
        self . _debug = enabled;
        if self . is_running ( ) {
        self . call_soon_threadsafe ( self . _set_coroutine_origin_tracking , enabled );
}

