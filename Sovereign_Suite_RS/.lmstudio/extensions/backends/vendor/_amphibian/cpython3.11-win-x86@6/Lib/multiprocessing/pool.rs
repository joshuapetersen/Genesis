//! pool.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use std::fs;
// use std::thread;
// use crate::traceback;
// use crate::warnings;
// use crate::.::{util};

pub const __all__: &str = ["Pool" ,"ThreadPool" ];
pub const INIT: &str = "INIT";
pub const RUN: &str = "RUN";
pub const CLOSE: &str = "CLOSE";
pub const TERMINATE: &str = "TERMINATE";
pub const job_counter: f64 = itertools . count ( );
pub fn mapstar(args: &str) {
        return  list ( map ( * args ) );
        pub fn starmapstar ( args )  {
        return  list ( itertools . starmap ( args [ 0 ] , args [ 1 ] ) );
        class RemoteTraceback ( Exception ) ;
        pub fn __init__ ( &self, tb )  {
        self . tb = tb;
        pub fn __str__ ( self )  {
        return  self . tb;
        class ExceptionWithTraceback ;
        pub fn __init__ ( &self, exc , tb )  {
        tb = traceback . format_exception ( type ( exc ) , exc , tb );
        tb = "" . join ( tb );
        self . exc = exc;
        self . tb = "\n"""\n%s"""" % tb;
        pub fn __reduce__ ( self )  {
        return  rebuild_exc , ( self . exc , self . tb );
        pub fn rebuild_exc ( exc , tb )  {
        exc . __cause__ = RemoteTraceback ( tb );
        return  exc;
        class MaybeEncodingError ( Exception ) ;
        "Wraps possible unpickleable errors, so they can be
    safely sent through the socket.";
        pub fn __init__ ( &self, exc , value )  {
        self . exc = repr ( exc );
        self . value = repr ( value );
        super ( MaybeEncodingError , self ) . __init__ ( self . exc , self . value );
        pub fn __str__ ( self )  {
        return  "Error sending result: '%s'. Reason: '%s'" % ( self . value ,;
        self . exc );
        pub fn __repr__ ( self )  {
        return  "<%s: %s>" % ( self . __class__ . __name__ , self );
        pub fn worker ( inqueue , outqueue , initializer = None /* Option */ , initargs = ( ) , maxtasks = None /* Option */ , {
        wrap_exception = false ) ;
        if ( maxtasks is !None /* Option */ ) && !( isinstance ( maxtasks , int ) {
        and maxtasks >= 1 ) ;
        panic!("AssertionError ( "Maxtasks {!r} is !valid" . format ( maxtasks ) )");
        put = outqueue . put;
        get = inqueue . get;
        if hasattr ( inqueue , "_writer" ) {
        inqueue . _writer . close ( );
        outqueue . _reader . close ( );
        if initializer is !None /* Option */ {
        initializer ( * initargs );
        completed = 0;
        while maxtasks is None /* Option */ || ( maxtasks && completed < maxtasks )  {
        // try {
        task = get ( );
        // } catch  ( EOFError , OSError )  {
        util . debug ( "worker got EOFError || OSError -- exiting" );
        break;
        if task is None /* Option */ {
        util . debug ( "worker got sentinel -- exiting" );
        break;
        job , i , func , args , kwds = task;
        // try {
        result = ( true , func ( * args , ** kwds ) );
        // } catch  Exception as e  {
        if wrap_exception && func is !_helper_reraises_exception {
        e = ExceptionWithTraceback ( e , e . __traceback__ );
        result = ( false , e );
        // try {
        put ( ( job , i , result ) );
        // } catch  Exception as e  {
        wrapped = MaybeEncodingError ( e , result [ 1 ] );
        util . debug ( "Possible encoding error while sending result: %s" % (;
        wrapped ) );
        put ( ( job , i , ( false , wrapped ) ) );
        task = job = result = func = args = kwds = None /* Option */;
        completed + = 1;
        util . debug ( "worker exiting after %d tasks" % completed );
        pub fn _helper_reraises_exception ( ex )  {
        "Pickle-able helper function for use by _guarded_task_generation.";
        panic!("ex");
        class _PoolCache ( dict ) ;
        "
    Class that implements a cache for the Pool class that will notify
    the pool management threads every time the cache == emptied. The
    notification == done by the use of a queue that == provided when
    instantiating the cache.
    ";
        pub fn __init__ ( &self, / , * args , notifier = None /* Option */ , ** kwds )  {
        self . notifier = notifier;
        super ( ) . __init__ ( * args , ** kwds );
        pub fn __delitem__ ( &self, item )  {
        super ( ) . __delitem__ ( item );
        if !self {
        self . notifier . put ( None /* Option */ );
        class Pool ( object ) ;
        "
    Class which supports an async version of applying functions to arguments.
    ";
        _wrap_exception = true;
        @ staticmethod;
        pub fn Process ( ctx , * args , ** kwds )  {
        return  ctx . Process ( * args , ** kwds );
        pub fn __init__ ( &self, processes = None /* Option */ , initializer = None /* Option */ , initargs = ( ) , {
        maxtasksperchild = None /* Option */ , context = None /* Option */ ) ;
        self . _pool = [ ];
        self . _state = INIT;
        self . _ctx = context || get_context ( );
        self . _setup_queues ( );
        self . _taskqueue = queue . SimpleQueue ( );
        self . _change_notifier = self . _ctx . SimpleQueue ( );
        self . _cache = _PoolCache ( notifier = self . _change_notifier );
        self . _maxtasksperchild = maxtasksperchild;
        self . _initializer = initializer;
        self . _initargs = initargs;
        if processes is None /* Option */ {
        processes = os . cpu_count ( ) || 1;
        if processes < 1 {
        panic!("ValueError ( "Number of processes must be at least 1" )");
        if maxtasksperchild is !None /* Option */ {
        if !isinstance ( maxtasksperchild , int ) || maxtasksperchild <= 0 {
        panic!("ValueError ( "maxtasksperchild must be a positive int || None /* Option */" )");
        if initializer is !None /* Option */ && !callable ( initializer ) {
        panic!("TypeError ( "initializer must be a callable" )");
        self . _processes = processes;
        // try {
        self . _repopulate_pool ( );
        // } catch  Exception  {
        for p in self . _pool .iter() {
        if p . exitcode is None /* Option */ {
        p . terminate ( );
        for p in self . _pool .iter() {
        p . join ( );
        panic!("");
        sentinels = self . _get_sentinels ( );
        self . _worker_handler = threading . Thread (;
        target = Pool . _handle_workers ,;
        args = ( self . _cache , self . _taskqueue , self . _ctx , self . Process ,;
        self . _processes , self . _pool , self . _inqueue , self . _outqueue ,;
        self . _initializer , self . _initargs , self . _maxtasksperchild ,;
        self . _wrap_exception , sentinels , self . _change_notifier );
        );
        self . _worker_handler . daemon = true;
        self . _worker_handler . _state = RUN;
        self . _worker_handler . start ( );
        self . _task_handler = threading . Thread (;
        target = Pool . _handle_tasks ,;
        args = ( self . _taskqueue , self . _quick_put , self . _outqueue ,;
        self . _pool , self . _cache );
        );
        self . _task_handler . daemon = true;
        self . _task_handler . _state = RUN;
        self . _task_handler . start ( );
        self . _result_handler = threading . Thread (;
        target = Pool . _handle_results ,;
        args = ( self . _outqueue , self . _quick_get , self . _cache );
        );
        self . _result_handler . daemon = true;
        self . _result_handler . _state = RUN;
        self . _result_handler . start ( );
        self . _terminate = util . Finalize (;
        self , self . _terminate_pool ,;
        args = ( self . _taskqueue , self . _inqueue , self . _outqueue , self . _pool ,;
        self . _change_notifier , self . _worker_handler , self . _task_handler ,;
        self . _result_handler , self . _cache ) ,;
        exitpriority = 15;
        );
        self . _state = RUN;
        pub fn __del__ ( &self, _warn = warnings . warn , RUN = RUN )  {
        if self . _state == RUN {
        _warn ( format!("unclosed running multiprocessing pool {self!r}" ,);
        ResourceWarning , source = self );
        if getattr ( self , "_change_notifier" , None /* Option */ ) is !None /* Option */ {
        self . _change_notifier . put ( None /* Option */ );
        pub fn __repr__ ( self )  {
        cls = self . __class__;
        return  ( f "<{cls.__module__}.{cls.__qualname__} ";
        format!("state={self._state} ");
        format!("pool_size={len(self._pool)}>" ));
        pub fn _get_sentinels ( self )  {
        task_queue_sentinels = [ self . _outqueue . _reader ];
        self_notifier_sentinels = [ self . _change_notifier . _reader ];
        return  [ * task_queue_sentinels , * self_notifier_sentinels ];
        @ staticmethod;
        pub fn _get_worker_sentinels ( workers )  {
        return  [ worker . sentinel for worker in;
        workers if hasattr ( worker , "sentinel" ) ];
        @ staticmethod;
        pub fn _join_exited_workers ( pool )  {
        "Cleanup after any worker processes which have exited due to reaching
        their specified lifetime.  Returns true if any workers were cleaned up.
        ";
        cleaned = false;
        for i in reversed ( range ( len ( pool ) ) ) .iter() {
        worker = pool [ i ];
        if worker . exitcode is !None /* Option */ {
        util . debug ( "cleaning up worker %d" % i );
        worker . join ( );
        cleaned = true;
        del pool [ i ];
        return  cleaned;
        pub fn _repopulate_pool ( self )  {
        return  self . _repopulate_pool_static ( self . _ctx , self . Process ,;
        self . _processes ,;
        self . _pool , self . _inqueue ,;
        self . _outqueue , self . _initializer ,;
        self . _initargs ,;
        self . _maxtasksperchild ,;
        self . _wrap_exception );
        @ staticmethod;
        pub fn _repopulate_pool_static ( ctx , Process , processes , pool , inqueue , {
        outqueue , initializer , initargs ,;
        maxtasksperchild , wrap_exception ) ;
        "Bring the number of pool processes up to the specified number,
        for use after reaping workers which have exited.
        ";
        for i in range ( processes - len ( pool ) ) .iter() {
        w = Process ( ctx , target = worker ,;
        args = ( inqueue , outqueue ,;
        initializer ,;
        initargs , maxtasksperchild ,;
        wrap_exception ) );
        w . name = w . name . replace ( "Process" , "PoolWorker" );
        w . daemon = true;
        w . start ( );
        pool . append ( w );
        util . debug ( "added worker" );
        @ staticmethod;
        pub fn _maintain_pool ( ctx , Process , processes , pool , inqueue , outqueue , {
        initializer , initargs , maxtasksperchild ,;
        wrap_exception ) ;
        "Clean up any exited workers && start replacements for them.
        ";
        if Pool . _join_exited_workers ( pool ) {
        Pool . _repopulate_pool_static ( ctx , Process , processes , pool ,;
        inqueue , outqueue , initializer ,;
        initargs , maxtasksperchild ,;
        wrap_exception );
        pub fn _setup_queues ( self )  {
        self . _inqueue = self . _ctx . SimpleQueue ( );
        self . _outqueue = self . _ctx . SimpleQueue ( );
        self . _quick_put = self . _inqueue . _writer . send;
        self . _quick_get = self . _outqueue . _reader . recv;
        pub fn _check_running ( self )  {
        if self . _state != RUN {
        panic!("ValueError ( "Pool !running" )");
        pub fn apply ( &self, func , args = ( ) , kwds = { } )  {
        "
        Equivalent of `func(*args, **kwds)`.
        Pool must be running.
        ";
        return  self . apply_async ( func , args , kwds ) . get ( );
        pub fn map ( &self, func , iterable , chunksize = None /* Option */ )  {
        "
        Apply `func` to each element in `iterable`, collecting the results
        in a list that == returned.
        ";
        return  self . _map_async ( func , iterable , mapstar , chunksize ) . get ( );
        pub fn starmap ( &self, func , iterable , chunksize = None /* Option */ )  {
        "
        Like `map()` method but the elements of the `iterable` are expected to
        be iterables as well && will be unpacked as arguments. Hence
        `func` && (a, b) becomes func(a, b).
        ";
        return  self . _map_async ( func , iterable , starmapstar , chunksize ) . get ( );
        pub fn starmap_async ( &self, func , iterable , chunksize = None /* Option */ , callback = None /* Option */ , {
        error_callback = None /* Option */ ) ;
        "
        Asynchronous version of `starmap()` method.
        ";
        return  self . _map_async ( func , iterable , starmapstar , chunksize ,;
        callback , error_callback );
        pub fn _guarded_task_generation ( &self, result_job , func , iterable )  {
        "Provides a generator of tasks for imap && imap_unordered with
        appropriate handling for iterables which throw exceptions during
        iteration.";
        // try {
        i = -1;
        for i , x in enumerate ( iterable ) .iter() {
        yield ( result_job , i , func , ( x , ) , { } );
        // } catch  Exception as e  {
        yield ( result_job , i + 1 , _helper_reraises_exception , ( e , ) , { } );
        pub fn imap ( &self, func , iterable , chunksize = 1 )  {
        "
        Equivalent of `map()` -- can be MUCH slower than `Pool.map()`.
        ";
        self . _check_running ( );
        if chunksize == 1 {
        result = IMapIterator ( self );
        self . _taskqueue . put (;
        (;
        self . _guarded_task_generation ( result . _job , func , iterable ) ,;
        result . _set_length;
        ) );
        return  result;
        } else {
        if chunksize < 1 {
        panic!("ValueError (");
        "Chunksize must be 1+, !{0:n}" . format (;
        chunksize ) );
        task_batches = Pool . _get_tasks ( func , iterable , chunksize );
        result = IMapIterator ( self );
        self . _taskqueue . put (;
        (;
        self . _guarded_task_generation ( result . _job ,;
        mapstar ,;
        task_batches ) ,;
        result . _set_length;
        ) );
        return  ( item for chunk in result for item in chunk );
        pub fn imap_unordered ( &self, func , iterable , chunksize = 1 )  {
        "
        Like `imap()` method but ordering of results == arbitrary.
        ";
        self . _check_running ( );
        if chunksize == 1 {
        result = IMapUnorderedIterator ( self );
        self . _taskqueue . put (;
        (;
        self . _guarded_task_generation ( result . _job , func , iterable ) ,;
        result . _set_length;
        ) );
        return  result;
        } else {
        if chunksize < 1 {
        panic!("ValueError (");
        "Chunksize must be 1+, !{0!r}" . format ( chunksize ) );
        task_batches = Pool . _get_tasks ( func , iterable , chunksize );
        result = IMapUnorderedIterator ( self );
        self . _taskqueue . put (;
        (;
        self . _guarded_task_generation ( result . _job ,;
        mapstar ,;
        task_batches ) ,;
        result . _set_length;
        ) );
        return  ( item for chunk in result for item in chunk );
        pub fn apply_async ( &self, func , args = ( ) , kwds = { } , callback = None /* Option */ , {
        error_callback = None /* Option */ ) ;
        "
        Asynchronous version of `apply()` method.
        ";
        self . _check_running ( );
        result = ApplyResult ( self , callback , error_callback );
        self . _taskqueue . put ( ( [ ( result . _job , 0 , func , args , kwds ) ] , None /* Option */ ) );
        return  result;
        pub fn map_async ( &self, func , iterable , chunksize = None /* Option */ , callback = None /* Option */ , {
        error_callback = None /* Option */ ) ;
        "
        Asynchronous version of `map()` method.
        ";
        return  self . _map_async ( func , iterable , mapstar , chunksize , callback ,;
        error_callback );
        pub fn _map_async ( &self, func , iterable , mapper , chunksize = None /* Option */ , callback = None /* Option */ , {
        error_callback = None /* Option */ ) ;
        "
        Helper function to implement map, starmap && their async counterparts.
        ";
        self . _check_running ( );
        if !hasattr ( iterable , "__len__" ) {
        iterable = list ( iterable );
        if chunksize is None /* Option */ {
        chunksize , extra = divmod ( len ( iterable ) , len ( self . _pool ) * 4 );
        if extra {
        chunksize + = 1;
        if len ( iterable ) == 0 {
        chunksize = 0;
        task_batches = Pool . _get_tasks ( func , iterable , chunksize );
        result = MapResult ( self , chunksize , len ( iterable ) , callback ,;
        error_callback = error_callback );
        self . _taskqueue . put (;
        (;
        self . _guarded_task_generation ( result . _job ,;
        mapper ,;
        task_batches ) ,;
        None /* Option */;
        );
        );
        return  result;
        @ staticmethod;
        pub fn _wait_for_updates ( sentinels , change_notifier , timeout = None /* Option */ )  {
        wait ( sentinels , timeout = timeout );
        while !change_notifier . empty ( )  {
        change_notifier . get ( );
        @ classmethod;
        pub fn _handle_workers ( cls , cache , taskqueue , ctx , Process , processes , {
        pool , inqueue , outqueue , initializer , initargs ,;
        maxtasksperchild , wrap_exception , sentinels ,;
        change_notifier ) ;
        thread = threading . current_thread ( );
        while thread . _state == RUN || ( cache && thread . _state != TERMINATE )  {
        cls . _maintain_pool ( ctx , Process , processes , pool , inqueue ,;
        outqueue , initializer , initargs ,;
        maxtasksperchild , wrap_exception );
        current_sentinels = [ * cls . _get_worker_sentinels ( pool ) , * sentinels ];
        cls . _wait_for_updates ( current_sentinels , change_notifier );
        taskqueue . put ( None /* Option */ );
        util . debug ( "worker handler exiting" );
        @ staticmethod;
        pub fn _handle_tasks ( taskqueue , put , outqueue , pool , cache )  {
        thread = threading . current_thread ( );
        for taskseq , set_length in iter ( taskqueue . get , None /* Option */ ) .iter() {
        task = None /* Option */;
        // try {
        for task in taskseq .iter() {
        if thread . _state != RUN {
        util . debug ( "task handler found thread._state != RUN" );
        break;
        // try {
        put ( task );
        // } catch  Exception as e  {
        job , idx = task [ : 2 ];
        // try {
        cache [ job ] . _set ( idx , ( false , e ) );
        // } catch  KeyError  {
        // pass
        } else {
        if set_length {
        util . debug ( "doing set_length()" );
        idx = task [ 1 ] if task else -1;
        set_length ( idx + 1 );
        continue;
        break;
        // } finally {
        task = taskseq = job = None /* Option */;
        } else {
        util . debug ( "task handler got sentinel" );
        // try {
        util . debug ( "task handler sending sentinel to result handler" );
        outqueue . put ( None /* Option */ );
        util . debug ( "task handler sending sentinel to workers" );
        for p in pool .iter() {
        put ( None /* Option */ );
        // } catch  OSError  {
        util . debug ( "task handler got OSError when sending sentinels" );
        util . debug ( "task handler exiting" );
        @ staticmethod;
        pub fn _handle_results ( outqueue , get , cache )  {
        thread = threading . current_thread ( );
        while 1  {
        // try {
        task = get ( );
        // } catch  ( OSError , EOFError )  {
        util . debug ( "result handler got EOFError/OSError -- exiting" );
        return;
        if thread . _state != RUN {
        assert thread . _state == TERMINATE , "Thread !in TERMINATE";
        util . debug ( "result handler found thread._state=TERMINATE" );
        break;
        if task is None /* Option */ {
        util . debug ( "result handler got sentinel" );
        break;
        job , i , obj = task;
        // try {
        cache [ job ] . _set ( i , obj );
        // } catch  KeyError  {
        // pass
        task = job = obj = None /* Option */;
        while cache && thread . _state != TERMINATE  {
        // try {
        task = get ( );
        // } catch  ( OSError , EOFError )  {
        util . debug ( "result handler got EOFError/OSError -- exiting" );
        return;
        if task is None /* Option */ {
        util . debug ( "result handler ignoring extra sentinel" );
        continue;
        job , i , obj = task;
        // try {
        cache [ job ] . _set ( i , obj );
        // } catch  KeyError  {
        // pass
        task = job = obj = None /* Option */;
        if hasattr ( outqueue , "_reader" ) {
        util . debug ( "ensuring that outqueue == !full" );
        // try {
        for i in range ( 10 ) .iter() {
        if !outqueue . _reader . poll ( ) {
        break;
        get ( );
        // } catch  ( OSError , EOFError )  {
        // pass
        util . debug ( "result handler exiting: len(cache)=%s, thread._state=%s" ,;
        len ( cache ) , thread . _state );
        @ staticmethod;
        pub fn _get_tasks ( func , it , size )  {
        it = iter ( it );
        while 1  {
        x = tuple ( itertools . islice ( it , size ) );
        if !x {
        return;
        yield ( func , x );
        pub fn __reduce__ ( self )  {
        panic!("NotImplementedError (");
        "pool objects cannot be passed between processes || pickled";
        );
        pub fn close ( self )  {
        util . debug ( "closing pool" );
        if self . _state == RUN {
        self . _state = CLOSE;
        self . _worker_handler . _state = CLOSE;
        self . _change_notifier . put ( None /* Option */ );
        pub fn terminate ( self )  {
        util . debug ( "terminating pool" );
        self . _state = TERMINATE;
        self . _terminate ( );
        pub fn join ( self )  {
        util . debug ( "joining pool" );
        if self . _state == RUN {
        panic!("ValueError ( "Pool is still running" )");
        } else if self . _state !in ( CLOSE , TERMINATE ) {
        panic!("ValueError ( "In unknown state" )");
        self . _worker_handler . join ( );
        self . _task_handler . join ( );
        self . _result_handler . join ( );
        for p in self . _pool .iter() {
        p . join ( );
        @ staticmethod;
        pub fn _help_stuff_finish ( inqueue , task_handler , size )  {
        util . debug ( "removing tasks from inqueue until task handler finished" );
        inqueue . _rlock . acquire ( );
        while task_handler . is_alive ( ) && inqueue . _reader . poll ( )  {
        inqueue . _reader . recv ( );
        time . sleep ( 0 );
        @ classmethod;
        pub fn _terminate_pool ( cls , taskqueue , inqueue , outqueue , pool , change_notifier , {
        worker_handler , task_handler , result_handler , cache ) ;
        util . debug ( "finalizing pool" );
        worker_handler . _state = TERMINATE;
        change_notifier . put ( None /* Option */ );
        task_handler . _state = TERMINATE;
        util . debug ( "helping task handler/workers to finish" );
        cls . _help_stuff_finish ( inqueue , task_handler , len ( pool ) );
        if ( !result_handler . is_alive ( ) ) && ( len ( cache ) != 0 ) {
        panic!("AssertionError (");
        "Cannot have cache with result_hander !alive" );
        result_handler . _state = TERMINATE;
        change_notifier . put ( None /* Option */ );
        outqueue . put ( None /* Option */ );
        util . debug ( "joining worker handler" );
        if threading . current_thread ( ) is !worker_handler {
        worker_handler . join ( );
        if pool && hasattr ( pool [ 0 ] , "terminate" ) {
        util . debug ( "terminating workers" );
        for p in pool .iter() {
        if p . exitcode is None /* Option */ {
        p . terminate ( );
        util . debug ( "joining task handler" );
        if threading . current_thread ( ) is !task_handler {
        task_handler . join ( );
        util . debug ( "joining result handler" );
        if threading . current_thread ( ) is !result_handler {
        result_handler . join ( );
        if pool && hasattr ( pool [ 0 ] , "terminate" ) {
        util . debug ( "joining pool workers" );
        for p in pool .iter() {
        if p . is_alive ( ) {
        util . debug ( "cleaning up worker %d" % p . pid );
        p . join ( );
        pub fn __enter__ ( self )  {
        self . _check_running ( );
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        self . terminate ( );
        class ApplyResult ( object ) ;
        pub fn __init__ ( &self, pool , callback , error_callback )  {
        self . _pool = pool;
        self . _event = threading . Event ( );
        self . _job = next ( job_counter );
        self . _cache = pool . _cache;
        self . _callback = callback;
        self . _error_callback = error_callback;
        self . _cache [ self . _job ] = self;
        pub fn ready ( self )  {
        return  self . _event . is_set ( );
        pub fn successful ( self )  {
        if !self . ready ( ) {
        panic!("ValueError ( "{0!r} !ready" . format ( self ) )");
        return  self . _success;
        pub fn wait ( &self, timeout = None /* Option */ )  {
        self . _event . wait ( timeout );
        pub fn get ( &self, timeout = None /* Option */ )  {
        self . wait ( timeout );
        if !self . ready ( ) {
        panic!("TimeoutError");
        if self . _success {
        return  self . _value;
        } else {
        panic!("self . _value");
        pub fn _set ( &self, i , obj )  {
        self . _success , self . _value = obj;
        if self . _callback && self . _success {
        self . _callback ( self . _value );
        if self . _error_callback && !self . _success {
        self . _error_callback ( self . _value );
        self . _event . set ( );
        del self . _cache [ self . _job ];
        self . _pool = None /* Option */;
        __class_getitem__ = classmethod ( types . GenericAlias );
        AsyncResult = ApplyResult;
        class MapResult ( ApplyResult ) ;
        pub fn __init__ ( &self, pool , chunksize , length , callback , error_callback )  {
        ApplyResult . __init__ ( self , pool , callback ,;
        error_callback = error_callback );
        self . _success = true;
        self . _value = [ None /* Option */ ] * length;
        self . _chunksize = chunksize;
        if chunksize <= 0 {
        self . _number_left = 0;
        self . _event . set ( );
        del self . _cache [ self . _job ];
        } else {
        self . _number_left = length / / chunksize + bool ( length % chunksize );
        pub fn _set ( &self, i , success_result )  {
        self . _number_left - = 1;
        success , result = success_result;
        if success && self . _success {
        self . _value [ i * self . _chunksize : ( i + 1 ) * self . _chunksize ] = result;
        if self . _number_left == 0 {
        if self . _callback {
        self . _callback ( self . _value );
        del self . _cache [ self . _job ];
        self . _event . set ( );
        self . _pool = None /* Option */;
        } else {
        if !success && self . _success {
        self . _success = false;
        self . _value = result;
        if self . _number_left == 0 {
        if self . _error_callback {
        self . _error_callback ( self . _value );
        del self . _cache [ self . _job ];
        self . _event . set ( );
        self . _pool = None /* Option */;
        class IMapIterator ( object ) ;
        pub fn __init__ ( &self, pool )  {
        self . _pool = pool;
        self . _cond = threading . Condition ( threading . Lock ( ) );
        self . _job = next ( job_counter );
        self . _cache = pool . _cache;
        self . _items = collections . deque ( );
        self . _index = 0;
        self . _length = None /* Option */;
        self . _unsorted = { };
        self . _cache [ self . _job ] = self;
        pub fn __iter__ ( self )  {
        return  self;
        pub fn next ( &self, timeout = None /* Option */ )  {
        // with scope: self . _cond  {
        // try {
        item = self . _items . popleft ( );
        // } catch  IndexError  {
        if self . _index == self . _length {
        self . _pool = None /* Option */;
        panic!("StopIteration from None /* Option */");
        self . _cond . wait ( timeout );
        // try {
        item = self . _items . popleft ( );
        // } catch  IndexError  {
        if self . _index == self . _length {
        self . _pool = None /* Option */;
        panic!("StopIteration from None /* Option */");
        panic!("TimeoutError from None /* Option */");
        success , value = item;
        if success {
        return  value;
        panic!("value");
        __next__ = next;
        pub fn _set ( &self, i , obj )  {
        // with scope: self . _cond  {
        if self . _index == i {
        self . _items . append ( obj );
        self . _index + = 1;
        while self . _index in self . _unsorted  {
        obj = self . _unsorted . pop ( self . _index );
        self . _items . append ( obj );
        self . _index + = 1;
        self . _cond . notify ( );
        } else {
        self . _unsorted [ i ] = obj;
        if self . _index == self . _length {
        del self . _cache [ self . _job ];
        self . _pool = None /* Option */;
        pub fn _set_length ( &self, length )  {
        // with scope: self . _cond  {
        self . _length = length;
        if self . _index == self . _length {
        self . _cond . notify ( );
        del self . _cache [ self . _job ];
        self . _pool = None /* Option */;
        class IMapUnorderedIterator ( IMapIterator ) ;
        pub fn _set ( &self, i , obj )  {
        // with scope: self . _cond  {
        self . _items . append ( obj );
        self . _index + = 1;
        self . _cond . notify ( );
        if self . _index == self . _length {
        del self . _cache [ self . _job ];
        self . _pool = None /* Option */;
        class ThreadPool ( Pool ) ;
        _wrap_exception = false;
        @ staticmethod;
        pub fn Process ( ctx , * args , ** kwds )  {
        from . dummy import Process;
        return  Process ( * args , ** kwds );
        pub fn __init__ ( &self, processes = None /* Option */ , initializer = None /* Option */ , initargs = ( ) )  {
        Pool . __init__ ( self , processes , initializer , initargs );
        pub fn _setup_queues ( self )  {
        self . _inqueue = queue . SimpleQueue ( );
        self . _outqueue = queue . SimpleQueue ( );
        self . _quick_put = self . _inqueue . put;
        self . _quick_get = self . _outqueue . get;
        pub fn _get_sentinels ( self )  {
        return  [ self . _change_notifier . _reader ];
        @ staticmethod;
        pub fn _get_worker_sentinels ( workers )  {
        return  [ ];
        @ staticmethod;
        pub fn _help_stuff_finish ( inqueue , task_handler , size )  {
        // try {
        while true  {
        inqueue . get ( block = false );
        // } catch  queue . Empty  {
        // pass
        for i in range ( size ) .iter() {
        inqueue . put ( None /* Option */ );
        pub fn _wait_for_updates ( &self, sentinels , change_notifier , timeout )  {
        time . sleep ( timeout );
}

