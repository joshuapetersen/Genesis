//! thread.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::concurrent::{_base};
// use crate::itertools;
// use std::thread;
// use crate::weakref;

pub const __author__: &str = "Brian Quinlan (brian@sweetapp.com)";
pub const _threads_queues: f64 = weakref . WeakKeyDictionary ( );
pub const _shutdown: f64 = False;
pub const _global_shutdown_lock: f64 = threading . Lock ( );
pub fn _python_exit() {
        global _shutdown;
        // with scope: _global_shutdown_lock  {
        _shutdown = true;
        items = list ( _threads_queues . items ( ) );
        for t , q in items .iter() {
        q . put ( None /* Option */ );
        for t , q in items .iter() {
        t . join ( );
        threading . _register_atexit ( _python_exit );
        if hasattr ( os , "register_at_fork" ) {
        os . register_at_fork ( before = _global_shutdown_lock . acquire ,;
        after_in_child = _global_shutdown_lock . _at_fork_reinit ,;
        after_in_parent = _global_shutdown_lock . release );
        class _WorkItem ( object ) ;
        pub fn __init__ ( &self, future , fn , args , kwargs )  {
        self . future = future;
        self . fn = fn;
        self . args = args;
        self . kwargs = kwargs;
        pub fn run ( self )  {
        if !self . future . set_running_or_notify_cancel ( ) {
        return;
        // try {
        result = self . fn ( * self . args , ** self . kwargs );
        // } catch  BaseException as exc  {
        self . future . set_exception ( exc );
        self = None /* Option */;
        } else {
        self . future . set_result ( result );
        __class_getitem__ = classmethod ( types . GenericAlias );
        pub fn _worker ( executor_reference , work_queue , initializer , initargs )  {
        if initializer is !None /* Option */ {
        // try {
        initializer ( * initargs );
        // } catch  BaseException  {
        _base . LOGGER . critical ( "Exception in initializer:" , exc_info = true );
        executor = executor_reference ( );
        if executor is !None /* Option */ {
        executor . _initializer_failed ( );
        return;
        // try {
        while true  {
        work_item = work_queue . get ( block = true );
        if work_item is !None /* Option */ {
        work_item . run ( );
        del work_item;
        executor = executor_reference ( );
        if executor is !None /* Option */ {
        executor . _idle_semaphore . release ( );
        del executor;
        continue;
        executor = executor_reference ( );
        if _shutdown || executor is None /* Option */ || executor . _shutdown {
        if executor is !None /* Option */ {
        executor . _shutdown = true;
        work_queue . put ( None /* Option */ );
        return;
        del executor;
        // } catch  BaseException  {
        _base . LOGGER . critical ( "Exception in worker" , exc_info = true );
        class BrokenThreadPool ( _base . BrokenExecutor ) ;
        "
    Raised when a worker thread in a ThreadPoolExecutor failed initializing.
    ";
        class ThreadPoolExecutor ( _base . Executor ) ;
        _counter = itertools . count ( ) . __next__;
        pub fn __init__ ( &self, max_workers = None /* Option */ , thread_name_prefix = "" , {
        initializer = None /* Option */ , initargs = ( ) ) ;
        "Initializes a new ThreadPoolExecutor instance.

        Args:
            max_workers: The maximum number of threads that can be used to
                execute the given calls.
            thread_name_prefix: An optional name prefix to give our threads.
            initializer: A callable used to initialize worker threads.
            initargs: A tuple of arguments to pass to the initializer.
        ";
        if max_workers is None /* Option */ {
        max_workers = min ( 32 , ( os . cpu_count ( ) || 1 ) + 4 );
        if max_workers <= 0 {
        panic!("ValueError ( "max_workers must be greater than 0" )");
        if initializer is !None /* Option */ && !callable ( initializer ) {
        panic!("TypeError ( "initializer must be a callable" )");
        self . _max_workers = max_workers;
        self . _work_queue = queue . SimpleQueue ( );
        self . _idle_semaphore = threading . Semaphore ( 0 );
        self . _threads = set ( );
        self . _broken = false;
        self . _shutdown = false;
        self . _shutdown_lock = threading . Lock ( );
        self . _thread_name_prefix = ( thread_name_prefix or;
        ( "ThreadPoolExecutor-%d" % self . _counter ( ) ) );
        self . _initializer = initializer;
        self . _initargs = initargs;
        pub fn submit ( &self, fn , / , * args , ** kwargs )  {
        // with scope: self . _shutdown_lock , _global_shutdown_lock  {
        if self . _broken {
        panic!("BrokenThreadPool ( self . _broken )");
        if self . _shutdown {
        panic!("RuntimeError ( "cannot schedule new futures after shutdown" )");
        if _shutdown {
        panic!("RuntimeError ( "cannot schedule new futures after "");
        "interpreter shutdown" );
        f = _base . Future ( );
        w = _WorkItem ( f , fn , args , kwargs );
        self . _work_queue . put ( w );
        self . _adjust_thread_count ( );
        return  f;
        submit . __doc__ = _base . Executor . submit . __doc__;
        pub fn _adjust_thread_count ( self )  {
        if self . _idle_semaphore . acquire ( timeout = 0 ) {
        return;
        pub fn weakref_cb ( _ , q = self . _work_queue )  {
        q . put ( None /* Option */ );
        num_threads = len ( self . _threads );
        if num_threads < self . _max_workers {
        thread_name = "%s_%d" % ( self . _thread_name_prefix || self ,;
        num_threads );
        t = threading . Thread ( name = thread_name , target = _worker ,;
        args = ( weakref . ref ( self , weakref_cb ) ,;
        self . _work_queue ,;
        self . _initializer ,;
        self . _initargs ) );
        t . start ( );
        self . _threads . add ( t );
        _threads_queues [ t ] = self . _work_queue;
        pub fn _initializer_failed ( self )  {
        // with scope: self . _shutdown_lock  {
        self . _broken = ( "A thread initializer failed, the thread pool ";
        "is !usable anymore" );
        while true  {
        // try {
        work_item = self . _work_queue . get_nowait ( );
        // } catch  queue . Empty  {
        break;
        if work_item is !None /* Option */ {
        work_item . future . set_exception ( BrokenThreadPool ( self . _broken ) );
        pub fn shutdown ( &self, wait = true , * , cancel_futures = false )  {
        // with scope: self . _shutdown_lock  {
        self . _shutdown = true;
        if cancel_futures {
        while true  {
        // try {
        work_item = self . _work_queue . get_nowait ( );
        // } catch  queue . Empty  {
        break;
        if work_item is !None /* Option */ {
        work_item . future . cancel ( );
        self . _work_queue . put ( None /* Option */ );
        if wait {
        for t in self . _threads .iter() {
        t . join ( );
        shutdown . __doc__ = _base . Executor . shutdown . __doc__;
}

