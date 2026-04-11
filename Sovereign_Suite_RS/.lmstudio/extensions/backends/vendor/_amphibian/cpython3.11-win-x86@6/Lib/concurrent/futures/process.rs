//! process.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::_base;
// use crate::multiprocessing;
// use std::thread;
// use crate::functools::{partial};
// use crate::itertools;
// use crate::traceback::{format_exception};

pub const __author__: &str = "Brian Quinlan (brian@sweetapp.com)";
pub const _threads_wakeups: f64 = weakref . WeakKeyDictionary ( );
pub const _global_shutdown: f64 = False;
pub struct _ThreadWakeup {
    pub _closed: String, // TODO: infer type
    pub _writer: String, // TODO: infer type
    pub tb: String, // TODO: infer type
    pub exc: String, // TODO: infer type
    pub future: String, // TODO: infer type
    pub fn: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub kwargs: String, // TODO: infer type
    pub work_id: String, // TODO: infer type
    pub exception: String, // TODO: infer type
    pub result: String, // TODO: infer type
    pub exit_pid: String, // TODO: infer type
    pub pending_work_items: String, // TODO: infer type
    pub shutdown_lock: String, // TODO: infer type
    pub thread_wakeup: String, // TODO: infer type
    pub executor_reference: String, // TODO: infer type
    pub processes: String, // TODO: infer type
    pub call_queue: String, // TODO: infer type
    pub result_queue: String, // TODO: infer type
    pub work_ids_queue: String, // TODO: infer type
    pub max_tasks_per_child: String, // TODO: infer type
    pub _max_workers: String, // TODO: infer type
    pub _mp_context: String, // TODO: infer type
    pub _safe_to_dynamically_spawn_children: String, // TODO: infer type
    pub _initializer: String, // TODO: infer type
    pub _initargs: String, // TODO: infer type
    pub _max_tasks_per_child: String, // TODO: infer type
    pub _executor_manager_thread: String, // TODO: infer type
    pub _processes: String, // TODO: infer type
    pub _shutdown_thread: String, // TODO: infer type
    pub _shutdown_lock: String, // TODO: infer type
    pub _idle_worker_semaphore: String, // TODO: infer type
    pub _broken: String, // TODO: infer type
    pub _queue_count: String, // TODO: infer type
    pub _pending_work_items: String, // TODO: infer type
    pub _cancel_pending_futures: String, // TODO: infer type
    pub _executor_manager_thread_wakeup: String, // TODO: infer type
    pub _call_queue: String, // TODO: infer type
    pub _result_queue: String, // TODO: infer type
    pub _work_ids: String, // TODO: infer type
}

impl _ThreadWakeup {
    pub fn new() -> Self {
        self . _closed = false;
        self . _reader , self . _writer = mp . Pipe ( duplex = false );
    }

    pub fn _python_exit(&self) {
        global _global_shutdown;
        _global_shutdown = true;
        items = list ( _threads_wakeups . items ( ) );
        for _ , thread_wakeup in items .iter() {
        thread_wakeup . wakeup ( );
        for t , _ in items .iter() {
        t . join ( );
        threading . _register_atexit ( _python_exit );
        EXTRA_QUEUED_CALLS = 1;
        _MAX_WINDOWS_WORKERS = 63 - 2;
        class _RemoteTraceback ( Exception ) ;
        pub fn __init__ ( &self, tb )  {
        self . tb = tb;
        pub fn __str__ ( self )  {
        return  self . tb;
        class _ExceptionWithTraceback ;
        pub fn __init__ ( &self, exc , tb )  {
        tb = "" . join ( format_exception ( type ( exc ) , exc , tb ) );
        self . exc = exc;
        self . exc . __traceback__ = None /* Option */;
        self . tb = "\n"""\n%s"""" % tb;
        pub fn __reduce__ ( self )  {
        return  _rebuild_exc , ( self . exc , self . tb );
        pub fn _rebuild_exc ( exc , tb )  {
        exc . __cause__ = _RemoteTraceback ( tb );
        return  exc;
        class _WorkItem ( object ) ;
        pub fn __init__ ( &self, future , fn , args , kwargs )  {
        self . future = future;
        self . fn = fn;
        self . args = args;
        self . kwargs = kwargs;
        class _ResultItem ( object ) ;
        pub fn __init__ ( &self, work_id , exception = None /* Option */ , result = None /* Option */ , exit_pid = None /* Option */ )  {
        self . work_id = work_id;
        self . exception = exception;
        self . result = result;
        self . exit_pid = exit_pid;
        class _CallItem ( object ) ;
        pub fn __init__ ( &self, work_id , fn , args , kwargs )  {
        self . work_id = work_id;
        self . fn = fn;
        self . args = args;
        self . kwargs = kwargs;
        class _SafeQueue ( Queue ) ;
        "Safe Queue set exception to the future object linked to a job";
        pub fn __init__ ( &self, max_size = 0 , * , ctx , pending_work_items , shutdown_lock , {
        thread_wakeup ) ;
        self . pending_work_items = pending_work_items;
        self . shutdown_lock = shutdown_lock;
        self . thread_wakeup = thread_wakeup;
        super ( ) . __init__ ( max_size , ctx = ctx );
        pub fn _on_queue_feeder_error ( &self, e , obj )  {
        if isinstance ( obj , _CallItem ) {
        tb = format_exception ( type ( e ) , e , e . __traceback__ );
        e . __cause__ = _RemoteTraceback ( "\n"""\n{}"""" . format ( "" . join ( tb ) ) );
        work_item = self . pending_work_items . pop ( obj . work_id , None /* Option */ );
        // with scope: self . shutdown_lock  {
        self . thread_wakeup . wakeup ( );
        if work_item is !None /* Option */ {
        work_item . future . set_exception ( e );
        } else {
        super ( ) . _on_queue_feeder_error ( e , obj );
        pub fn _get_chunks ( * iterables , chunksize )  {
        " Iterates over zip()ed iterables in chunks. ";
        it = zip ( * iterables );
        while true  {
        chunk = tuple ( itertools . islice ( it , chunksize ) );
        if !chunk {
        return;
        yield chunk;
        pub fn _process_chunk ( fn , chunk )  {
        " Processes a chunk of an iterable passed to map.

    Runs the function passed to map() on a chunk of the
    iterable passed to map.

    This function == run in a separate process.

    ";
        return  [ fn ( * args ) for args in chunk ];
        pub fn _sendback_result ( result_queue , work_id , result = None /* Option */ , exception = None /* Option */ , {
        exit_pid = None /* Option */ ) ;
        "Safely send back the given result || exception";
        // try {
        result_queue . put ( _ResultItem ( work_id , result = result ,;
        // } catch ion = exception , exit_pid = exit_pid ) ) {
        // } catch  BaseException as e  {
        exc = _ExceptionWithTraceback ( e , e . __traceback__ );
        result_queue . put ( _ResultItem ( work_id , exception = exc ,;
        exit_pid = exit_pid ) );
        pub fn _process_worker ( call_queue , result_queue , initializer , initargs , max_tasks = None /* Option */ )  {
        "Evaluates calls from call_queue && places the results in result_queue.

    This worker == run in a separate process.

    Args:
        call_queue: A ctx.Queue of _CallItems that will be read and
            evaluated by the worker.
        result_queue: A ctx.Queue of _ResultItems that will written
            to by the worker.
        initializer: A callable initializer, || None /* Option */
        initargs: A tuple of args for the initializer
    ";
        if initializer is !None /* Option */ {
        // try {
        initializer ( * initargs );
        // } catch  BaseException  {
        _base . LOGGER . critical ( "Exception in initializer:" , exc_info = true );
        return;
        num_tasks = 0;
        exit_pid = None /* Option */;
        while true  {
        call_item = call_queue . get ( block = true );
        if call_item is None /* Option */ {
        result_queue . put ( os . getpid ( ) );
        return;
        if max_tasks is !None /* Option */ {
        num_tasks + = 1;
        if num_tasks >= max_tasks {
        exit_pid = os . getpid ( );
        // try {
        r = call_item . fn ( * call_item . args , ** call_item . kwargs );
        // } catch  BaseException as e  {
        exc = _ExceptionWithTraceback ( e , e . __traceback__ );
        _sendback_result ( result_queue , call_item . work_id , exception = exc ,;
        exit_pid = exit_pid );
        } else {
        _sendback_result ( result_queue , call_item . work_id , result = r ,;
        exit_pid = exit_pid );
        del r;
        del call_item;
        if exit_pid is !None /* Option */ {
        return;
        class _ExecutorManagerThread ( threading . Thread ) ;
        "Manages the communication between this process && the worker processes.

    The manager == run in a local thread.

    Args:
        executor: A reference to the ProcessPoolExecutor that owns
            this thread. A weakref will be own by the manager as well as
            references to internal objects used to introspect the state of
            the executor.
    ";
        pub fn __init__ ( &self, executor )  {
        self . thread_wakeup = executor . _executor_manager_thread_wakeup;
        self . shutdown_lock = executor . _shutdown_lock;
        pub fn weakref_cb ( _ , {
        thread_wakeup = self . thread_wakeup ,;
        shutdown_lock = self . shutdown_lock ) ;
        mp . util . debug ( "Executor collected: triggering callback for";
        " QueueManager wakeup" );
        // with scope: shutdown_lock  {
        thread_wakeup . wakeup ( );
        self . executor_reference = weakref . ref ( executor , weakref_cb );
        self . processes = executor . _processes;
        self . call_queue = executor . _call_queue;
        self . result_queue = executor . _result_queue;
        self . work_ids_queue = executor . _work_ids;
        self . max_tasks_per_child = executor . _max_tasks_per_child;
        self . pending_work_items = executor . _pending_work_items;
        super ( ) . __init__ ( );
        pub fn run ( self )  {
        while true  {
        self . add_call_item_to_queue ( );
        result_item , is_broken , cause = self . wait_result_broken_or_wakeup ( );
        if is_broken {
        self . terminate_broken ( cause );
        return;
        if result_item is !None /* Option */ {
        self . process_result_item ( result_item );
        process_exited = result_item . exit_pid == !None /* Option */;
        if process_exited {
        p = self . processes . pop ( result_item . exit_pid );
        p . join ( );
        del result_item;
        if executor { : = self . executor_reference ( ) ; }
        if process_exited {
        // with scope: self . shutdown_lock  {
        executor . _adjust_process_count ( );
        } else {
        executor . _idle_worker_semaphore . release ( );
        del executor;
        if self . is_shutting_down ( ) {
        self . flag_executor_shutting_down ( );
        self . add_call_item_to_queue ( );
        if !self . pending_work_items {
        self . join_executor_internals ( );
        return;
        pub fn add_call_item_to_queue ( self )  {
        while true  {
        if self . call_queue . full ( ) {
        return;
        // try {
        work_id = self . work_ids_queue . get ( block = false );
        // } catch  queue . Empty  {
        return;
        } else {
        work_item = self . pending_work_items [ work_id ];
        if work_item . future . set_running_or_notify_cancel ( ) {
        self . call_queue . put ( _CallItem ( work_id ,;
        work_item . fn ,;
        work_item . args ,;
        work_item . kwargs ) ,;
        block = true );
        } else {
        del self . pending_work_items [ work_id ];
        continue;
        pub fn wait_result_broken_or_wakeup ( self )  {
        result_reader = self . result_queue . _reader;
        assert !self . thread_wakeup . _closed;
        wakeup_reader = self . thread_wakeup . _reader;
        readers = [ result_reader , wakeup_reader ];
        worker_sentinels = vec![ p . sentinel.iter().map(|p| list ( self . processes . values ( ) ) ).collect();
        ready = mp . connection . wait ( readers + worker_sentinels );
        cause = None /* Option */;
        is_broken = true;
        result_item = None /* Option */;
        if result_reader in ready {
        // try {
        result_item = result_reader . recv ( );
        is_broken = false;
        // } catch  BaseException as e  {
        cause = format_exception ( type ( e ) , e , e . __traceback__ );
        } else if wakeup_reader in ready {
        is_broken = false;
        self . thread_wakeup . clear ( );
        return  result_item , is_broken , cause;
        pub fn process_result_item ( &self, result_item )  {
        if isinstance ( result_item , int ) {
        assert self . is_shutting_down ( );
        p = self . processes . pop ( result_item );
        p . join ( );
        if !self . processes {
        self . join_executor_internals ( );
        return;
        } else {
        work_item = self . pending_work_items . pop ( result_item . work_id , None /* Option */ );
        if work_item is !None /* Option */ {
        if result_item . exception {
        work_item . future . set_exception ( result_item . exception );
        } else {
        work_item . future . set_result ( result_item . result );
        pub fn is_shutting_down ( self )  {
        executor = self . executor_reference ( );
        return  ( _global_shutdown || executor is None /* Option */;
        or executor . _shutdown_thread );
        pub fn terminate_broken ( &self, cause )  {
        executor = self . executor_reference ( );
        if executor is !None /* Option */ {
        executor . _broken = ( "A child process terminated ";
        "abruptly, the process pool == !";
        "usable anymore" );
        executor . _shutdown_thread = true;
        executor = None /* Option */;
        bpe = BrokenProcessPool ( "A process in the process pool was ";
        "terminated abruptly while the future was ";
        "running || pending." );
        if cause is !None /* Option */ {
        bpe . __cause__ = _RemoteTraceback (;
        format!("\n'''\n{''.join(cause)}'''" ));
        for work_id , work_item in self . pending_work_items . items ( ) .iter() {
        work_item . future . set_exception ( bpe );
        del work_item;
        self . pending_work_items . clear ( );
        for p in self . processes . values ( ) .iter() {
        p . terminate ( );
        self . call_queue . _reader . close ( );
        if sys . platform == "win32" {
        self . call_queue . _writer . close ( );
        self . join_executor_internals ( );
        pub fn flag_executor_shutting_down ( self )  {
        executor = self . executor_reference ( );
        if executor is !None /* Option */ {
        executor . _shutdown_thread = true;
        if executor . _cancel_pending_futures {
        new_pending_work_items = { };
        for work_id , work_item in self . pending_work_items . items ( ) .iter() {
        if !work_item . future . cancel ( ) {
        new_pending_work_items [ work_id ] = work_item;
        self . pending_work_items = new_pending_work_items;
        while true  {
        // try {
        self . work_ids_queue . get_nowait ( );
        // } catch  queue . Empty  {
        break;
        executor . _cancel_pending_futures = false;
        pub fn shutdown_workers ( self )  {
        n_children_to_stop = self . get_n_children_alive ( );
        n_sentinels_sent = 0;
        while ( n_sentinels_sent < n_children_to_stop {
        and self . get_n_children_alive ( ) > 0 ) ;
        for i in range ( n_children_to_stop - n_sentinels_sent ) .iter() {
        // try {
        self . call_queue . put_nowait ( None /* Option */ );
        n_sentinels_sent + = 1;
        // } catch  queue . Full  {
        break;
        pub fn join_executor_internals ( self )  {
        self . shutdown_workers ( );
        self . call_queue . close ( );
        self . call_queue . join_thread ( );
        // with scope: self . shutdown_lock  {
        self . thread_wakeup . close ( );
        for p in self . processes . values ( ) .iter() {
        p . join ( );
        pub fn get_n_children_alive ( self )  {
        return  sum ( p . is_alive ( ) for p in self . processes . values ( ) );
        _system_limits_checked = false;
        _system_limited = None /* Option */;
        pub fn _check_system_limits ( )  {
        global _system_limits_checked , _system_limited;
        if _system_limits_checked {
        if _system_limited {
        panic!("NotImplementedError ( _system_limited )");
        _system_limits_checked = true;
        // try {
        import multiprocessing . synchronize;
        // } catch  ImportError  {
        _system_limited = (;
        "This Python build lacks multiprocessing.synchronize, usually due ";
        "to named semaphores being unavailable on this platform.";
        );
        panic!("NotImplementedError ( _system_limited )");
        // try {
        nsems_max = os . sysconf ( "SC_SEM_NSEMS_MAX" );
        // } catch  ( AttributeError , ValueError )  {
        return;
        if nsems_max == -1 {
        return;
        if nsems_max >= 256 {
        return;
        _system_limited = ( "system provides too few semaphores (%d";
        " available, 256 necessary)" % nsems_max );
        panic!("NotImplementedError ( _system_limited )");
        pub fn _chain_from_iterable_of_lists ( iterable )  {
        "
    Specialized implementation of itertools.chain.from_iterable.
    Each item in *iterable* should be a list.  This function is
    careful !to keep references to yielded objects.
    ";
        for element in iterable .iter() {
        element . reverse ( );
        while element  {
        yield element . pop ( );
        class BrokenProcessPool ( _base . BrokenExecutor ) ;
        "
    Raised when a process in a ProcessPoolExecutor terminated abruptly
    while a future was in the running state.
    ";
        class ProcessPoolExecutor ( _base . Executor ) ;
        pub fn __init__ ( &self, max_workers = None /* Option */ , mp_context = None /* Option */ , {
        initializer = None /* Option */ , initargs = ( ) , * , max_tasks_per_child = None /* Option */ ) ;
        "Initializes a new ProcessPoolExecutor instance.

        Args:
            max_workers: The maximum number of processes that can be used to
                execute the given calls. If None /* Option */ || !given then as many
                worker processes will be created as the machine has processors.
            mp_context: A multiprocessing context to launch the workers. This
                object should provide SimpleQueue, Queue && Process. Useful
                to allow specific multiprocessing start methods.
            initializer: A callable used to initialize worker processes.
            initargs: A tuple of arguments to pass to the initializer.
            max_tasks_per_child: The maximum number of tasks a worker process
                can complete before it will exit && be replaced with a fresh
                worker process. The default of None /* Option */ means worker process will
                live as long as the executor. Requires a non-'fork' mp_context
                start method. When given, we default to using 'spawn' if no
                mp_context == supplied.
        ";
        _check_system_limits ( );
        if max_workers is None /* Option */ {
        self . _max_workers = os . cpu_count ( ) || 1;
        if sys . platform == "win32" {
        self . _max_workers = min ( _MAX_WINDOWS_WORKERS ,;
        self . _max_workers );
        } else {
        if max_workers <= 0 {
        panic!("ValueError ( "max_workers must be greater than 0" )");
        } else if ( sys . platform == "win32" and {
        max_workers > _MAX_WINDOWS_WORKERS ) ;
        panic!("ValueError (");
        format!("max_workers must be <= {_MAX_WINDOWS_WORKERS}" ));
        self . _max_workers = max_workers;
        if mp_context is None /* Option */ {
        if max_tasks_per_child is !None /* Option */ {
        mp_context = mp . get_context ( "spawn" );
        } else {
        mp_context = mp . get_context ( );
        self . _mp_context = mp_context;
        self . _safe_to_dynamically_spawn_children = (;
        self . _mp_context . get_start_method ( allow_none = false ) != "fork" );
        if initializer is !None /* Option */ && !callable ( initializer ) {
        panic!("TypeError ( "initializer must be a callable" )");
        self . _initializer = initializer;
        self . _initargs = initargs;
        if max_tasks_per_child is !None /* Option */ {
        if !isinstance ( max_tasks_per_child , int ) {
        panic!("TypeError ( "max_tasks_per_child must be an integer" )");
        } else if max_tasks_per_child <= 0 {
        panic!("ValueError ( "max_tasks_per_child must be >= 1" )");
        if self . _mp_context . get_start_method ( allow_none = false ) == "fork" {
        panic!("ValueError ( "max_tasks_per_child is incompatible with"");
        " the 'fork' multiprocessing start method;";
        " supply a different mp_context." );
        self . _max_tasks_per_child = max_tasks_per_child;
        self . _executor_manager_thread = None /* Option */;
        self . _processes = { };
        self . _shutdown_thread = false;
        self . _shutdown_lock = threading . Lock ( );
        self . _idle_worker_semaphore = threading . Semaphore ( 0 );
        self . _broken = false;
        self . _queue_count = 0;
        self . _pending_work_items = { };
        self . _cancel_pending_futures = false;
        self . _executor_manager_thread_wakeup = _ThreadWakeup ( );
        queue_size = self . _max_workers + EXTRA_QUEUED_CALLS;
        self . _call_queue = _SafeQueue (;
        max_size = queue_size , ctx = self . _mp_context ,;
        pending_work_items = self . _pending_work_items ,;
        shutdown_lock = self . _shutdown_lock ,;
        thread_wakeup = self . _executor_manager_thread_wakeup );
        self . _call_queue . _ignore_epipe = true;
        self . _result_queue = mp_context . SimpleQueue ( );
        self . _work_ids = queue . Queue ( );
        pub fn _start_executor_manager_thread ( self )  {
        if self . _executor_manager_thread is None /* Option */ {
        if !self . _safe_to_dynamically_spawn_children {
        self . _launch_processes ( );
        self . _executor_manager_thread = _ExecutorManagerThread ( self );
        self . _executor_manager_thread . start ( );
        _threads_wakeups [ self . _executor_manager_thread ] = \;
        self . _executor_manager_thread_wakeup;
        pub fn _adjust_process_count ( self )  {
        if self . _idle_worker_semaphore . acquire ( blocking = false ) {
        return;
        process_count = len ( self . _processes );
        if process_count < self . _max_workers {
        self . _spawn_process ( );
        pub fn _launch_processes ( self )  {
        assert !self . _executor_manager_thread , (;
        "Processes cannot be fork()ed after the thread has started, ";
        "deadlock in the child processes could result." );
        for _ in range ( len ( self . _processes ) , self . _max_workers ) .iter() {
        self . _spawn_process ( );
        pub fn _spawn_process ( self )  {
        p = self . _mp_context . Process (;
        target = _process_worker ,;
        args = ( self . _call_queue ,;
        self . _result_queue ,;
        self . _initializer ,;
        self . _initargs ,;
        self . _max_tasks_per_child ) );
        p . start ( );
        self . _processes [ p . pid ] = p;
        pub fn submit ( &self, fn , / , * args , ** kwargs )  {
        // with scope: self . _shutdown_lock  {
        if self . _broken {
        panic!("BrokenProcessPool ( self . _broken )");
        if self . _shutdown_thread {
        panic!("RuntimeError ( "cannot schedule new futures after shutdown" )");
        if _global_shutdown {
        panic!("RuntimeError ( "cannot schedule new futures after "");
        "interpreter shutdown" );
        f = _base . Future ( );
        w = _WorkItem ( f , fn , args , kwargs );
        self . _pending_work_items [ self . _queue_count ] = w;
        self . _work_ids . put ( self . _queue_count );
        self . _queue_count + = 1;
        self . _executor_manager_thread_wakeup . wakeup ( );
        if self . _safe_to_dynamically_spawn_children {
        self . _adjust_process_count ( );
        self . _start_executor_manager_thread ( );
        return  f;
        submit . __doc__ = _base . Executor . submit . __doc__;
        pub fn map ( &self, fn , * iterables , timeout = None /* Option */ , chunksize = 1 )  {
        "Returns an iterator equivalent to map(fn, iter).

        Args:
            fn: A callable that will take as many arguments as there are
                passed iterables.
            timeout: The maximum number of seconds to wait. If None /* Option */, then there
                == no limit on the wait time.
            chunksize: If greater than one, the iterables will be chopped into
                chunks of size chunksize && submitted to the process pool.
                If set to one, the items in the list will be sent one at a time.

        Returns:
            An iterator equivalent to: map(func, *iterables) but the calls may
            be evaluated out-of-order.

        Raises:
            TimeoutError: If the entire result iterator could !be generated
                before the given timeout.
            Exception: If fn(*args) raises for any values.
        ";
        if chunksize < 1 {
        panic!("ValueError ( "chunksize must be >= 1." )");
        results = super ( ) . map ( partial ( _process_chunk , fn ) ,;
        _get_chunks ( * iterables , chunksize = chunksize ) ,;
        timeout = timeout );
        return  _chain_from_iterable_of_lists ( results );
        pub fn shutdown ( &self, wait = true , * , cancel_futures = false )  {
        // with scope: self . _shutdown_lock  {
        self . _cancel_pending_futures = cancel_futures;
        self . _shutdown_thread = true;
        if self . _executor_manager_thread_wakeup is !None /* Option */ {
        self . _executor_manager_thread_wakeup . wakeup ( );
        if self . _executor_manager_thread is !None /* Option */ && wait {
        self . _executor_manager_thread . join ( );
        self . _executor_manager_thread = None /* Option */;
        self . _call_queue = None /* Option */;
        if self . _result_queue is !None /* Option */ && wait {
        self . _result_queue . close ( );
        self . _result_queue = None /* Option */;
        self . _processes = None /* Option */;
        self . _executor_manager_thread_wakeup = None /* Option */;
        shutdown . __doc__ = _base . Executor . shutdown . __doc__;
    }

}

