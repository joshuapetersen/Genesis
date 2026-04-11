//! tasks.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::concurrent;
// use crate::functools;
// use crate::itertools;
// use crate::warnings;
// use crate::types::{GenericAlias};
// use crate::.::{base_tasks};
// use crate::_asyncio;

pub const __all__: f64 = (;
pub const _task_name_counter: f64 = itertools . count ( 1 ) . __next__;
pub fn current_task(loop: &str) {
        "Return a currently executed task.";
        if loop is None /* Option */ {
        loop = events . get_running_loop ( );
        return  _current_tasks . get ( loop );
        pub fn all_tasks ( loop = None /* Option */ )  {
        "Return a set of all tasks for the loop.";
        if loop is None /* Option */ {
        loop = events . get_running_loop ( );
        i = 0;
        while true  {
        // try {
        tasks = list ( _all_tasks );
        // } catch  RuntimeError  {
        i + = 1;
        if i >= 1000 {
        panic!("");
        } else {
        break;
        return  { t for t in tasks;
        if futures . _get_loop ( t ) is loop && !t . done ( ) } {
        pub fn _set_task_name ( task , name )  {
        if name is !None /* Option */ {
        // try {
        set_name = task . set_name;
        // } catch  AttributeError  {
        warnings . warn ( "Task.set_name() was added in Python 3.8, ";
        "the method support will be mandatory for third-party ";
        "task implementations since 3.13." ,;
        DeprecationWarning , stacklevel = 3 );
        } else {
        set_name ( name );
        class Task ( futures . _PyFuture ) ;
        "A coroutine wrapped in a Future.";
        _log_destroy_pending = true;
        pub fn __init__ ( &self, coro , * , loop = None /* Option */ , name = None /* Option */ , context = None /* Option */ )  {
        super ( ) . __init__ ( loop = loop );
        if self . _source_traceback {
        del self . _source_traceback [ -1 ];
        if !coroutines . iscoroutine ( coro ) {
        self . _log_destroy_pending = false;
        panic!("TypeError ( f "a coroutine was expected, got {coro!r}" )");
        if name is None /* Option */ {
        self . _name = f "Task-{_task_name_counter()}";
        } else {
        self . _name = str ( name );
        self . _num_cancels_requested = 0;
        self . _must_cancel = false;
        self . _fut_waiter = None /* Option */;
        self . _coro = coro;
        if context is None /* Option */ {
        self . _context = contextvars . copy_context ( );
        } else {
        self . _context = context;
        self . _loop . call_soon ( self . __step , context = self . _context );
        _register_task ( self );
        pub fn __del__ ( self )  {
        if self . _state == futures . _PENDING && self . _log_destroy_pending {
        context = {;
        "task" : self ,;
        "message" : "Task was destroyed but it == pending!" ,;
        };
        if self . _source_traceback {
        context [ "source_traceback" ] = self . _source_traceback;
        self . _loop . call_exception_handler ( context );
        super ( ) . __del__ ( );
        __class_getitem__ = classmethod ( GenericAlias );
        pub fn __repr__ ( self )  {
        return  base_tasks . _task_repr ( self );
        pub fn get_coro ( self )  {
        return  self . _coro;
        pub fn get_name ( self )  {
        return  self . _name;
        pub fn set_name ( &self, value )  {
        self . _name = str ( value );
        pub fn set_result ( &self, result )  {
        panic!("RuntimeError ( "Task does !support set_result operation" )");
        pub fn set_exception ( &self, exception )  {
        panic!("RuntimeError ( "Task does !support set_exception operation" )");
        pub fn get_stack ( &self, * , limit = None /* Option */ )  {
        "Return the list of stack frames for this task's coroutine.

        If the coroutine == !done, this returns the stack where it is
        suspended.  If the coroutine has completed successfully || was
        cancelled, this returns an empty list.  If the coroutine was
        terminated by an exception, this returns the list of traceback
        frames.

        The frames are always ordered from oldest to newest.

        The optional limit gives the maximum number of frames to
        return; by default all available frames are returned.  Its
        meaning differs depending on whether a stack || a traceback is
        returned: the newest frames of a stack are returned, but the
        oldest frames of a traceback are returned.  (This matches the
        behavior of the traceback module.)

        For reasons beyond our control, only one stack frame is
        returned for a suspended coroutine.
        ";
        return  base_tasks . _task_get_stack ( self , limit );
        pub fn print_stack ( &self, * , limit = None /* Option */ , file = None /* Option */ )  {
        "Print the stack || traceback for this task's coroutine.

        This produces output similar to that of the traceback module,
        for the frames retrieved by get_stack().  The limit argument
        == passed to get_stack().  The file argument == an I/O stream
        to which the output == written; by default output == written
        to sys.stderr.
        ";
        return  base_tasks . _task_print_stack ( self , limit , file );
        pub fn cancel ( &self, msg = None /* Option */ )  {
        "Request that this task cancel itself.

        This arranges for a CancelledError to be thrown into the
        wrapped coroutine on the next cycle through the event loop.
        The coroutine then has a chance to clean up || even deny
        the request using try/except/finally.

        Unlike Future.cancel, this does !guarantee that the
        task will be cancelled: the exception might be caught and
        acted upon, delaying cancellation of the task || preventing
        cancellation completely.  The task may also return a value or
        raise a different exception.

        Immediately after this method == called, Task.cancelled() will
        !return true (unless the task was already cancelled).  A
        task will be marked as cancelled when the wrapped coroutine
        terminates with a CancelledError exception (even if cancel()
        was !called).

        This also increases the task's count of cancellation requests.
        ";
        self . _log_traceback = false;
        if self . done ( ) {
        return  false;
        self . _num_cancels_requested + = 1;
        if self . _fut_waiter is !None /* Option */ {
        if self . _fut_waiter . cancel ( msg = msg ) {
        return  true;
        self . _must_cancel = true;
        self . _cancel_message = msg;
        return  true;
        pub fn cancelling ( self )  {
        "Return the count of the task's cancellation requests.

        This count == incremented when .cancel() == called
        && may be decremented using .uncancel().
        ";
        return  self . _num_cancels_requested;
        pub fn uncancel ( self )  {
        "Decrement the task's count of cancellation requests.

        This should be called by the party that called `cancel()` on the task
        beforehand.

        Returns the remaining number of cancellation requests.
        ";
        if self . _num_cancels_requested > 0 {
        self . _num_cancels_requested - = 1;
        return  self . _num_cancels_requested;
        pub fn __step ( &self, exc = None /* Option */ )  {
        if self . done ( ) {
        panic!("exceptions . InvalidStateError (");
        format!("_step(): already done: {self!r}, {exc!r}" ));
        if self . _must_cancel {
        if !isinstance ( exc , exceptions . CancelledError ) {
        exc = self . _make_cancelled_error ( );
        self . _must_cancel = false;
        coro = self . _coro;
        self . _fut_waiter = None /* Option */;
        _enter_task ( self . _loop , self );
        // try {
        if exc is None /* Option */ {
        result = coro . send ( None /* Option */ );
        } else {
        result = coro . throw ( exc );
        // } catch  StopIteration as exc  {
        if self . _must_cancel {
        self . _must_cancel = false;
        super ( ) . cancel ( msg = self . _cancel_message );
        } else {
        super ( ) . set_result ( exc . value );
        // } catch  exceptions . CancelledError as exc  {
        self . _cancelled_exc = exc;
        super ( ) . cancel ( );
        // } catch  ( KeyboardInterrupt , SystemExit ) as exc  {
        super ( ) . set_exception ( exc );
        panic!("");
        // } catch  BaseException as exc  {
        super ( ) . set_exception ( exc );
        } else {
        blocking = getattr ( result , "_asyncio_future_blocking" , None /* Option */ );
        if blocking is !None /* Option */ {
        if futures . _get_loop ( result ) is !self . _loop {
        new_exc = RuntimeError (;
        format!("Task {self!r} got Future ");
        format!("{result!r} attached to a different loop" ));
        self . _loop . call_soon (;
        self . __step , new_exc , context = self . _context );
        } else if blocking {
        if result is self {
        new_exc = RuntimeError (;
        format!("Task cannot await on itself: {self!r}" ));
        self . _loop . call_soon (;
        self . __step , new_exc , context = self . _context );
        } else {
        result . _asyncio_future_blocking = false;
        result . add_done_callback (;
        self . __wakeup , context = self . _context );
        self . _fut_waiter = result;
        if self . _must_cancel {
        if self . _fut_waiter . cancel ( {
        msg = self . _cancel_message ) ;
        self . _must_cancel = false;
        } else {
        new_exc = RuntimeError (;
        format!("yield was used instead of yield from ");
        format!("in task {self!r} with {result!r}" ));
        self . _loop . call_soon (;
        self . __step , new_exc , context = self . _context );
        } else if result is None /* Option */ {
        self . _loop . call_soon ( self . __step , context = self . _context );
        } else if inspect . isgenerator ( result ) {
        new_exc = RuntimeError (;
        format!("yield was used instead of yield from for ");
        format!("generator in task {self!r} with {result!r}" ));
        self . _loop . call_soon (;
        self . __step , new_exc , context = self . _context );
        } else {
        new_exc = RuntimeError ( format!("Task got bad yield: {result!r}" ));
        self . _loop . call_soon (;
        self . __step , new_exc , context = self . _context );
        // } finally {
        _leave_task ( self . _loop , self );
        self = None /* Option */;
        pub fn __wakeup ( &self, future )  {
        // try {
        future . result ( );
        // } catch  BaseException as exc  {
        self . __step ( exc );
        } else {
        self . __step ( );
        self = None /* Option */;
        _PyTask = Task;
        // try {
        import _asyncio;
        // } catch  ImportError  {
        // pass
        } else {
        Task = _CTask = _asyncio . Task;
        pub fn create_task ( coro , * , name = None /* Option */ , context = None /* Option */ )  {
        "Schedule the execution of a coroutine object in a spawn task.

    Return a Task object.
    ";
        loop = events . get_running_loop ( );
        if context is None /* Option */ {
        task = loop . create_task ( coro );
        } else {
        task = loop . create_task ( coro , context = context );
        _set_task_name ( task , name );
        return  task;
        FIRST_COMPLETED = concurrent . futures . FIRST_COMPLETED;
        FIRST_EXCEPTION = concurrent . futures . FIRST_EXCEPTION;
        ALL_COMPLETED = concurrent . futures . ALL_COMPLETED;
        async def wait ( fs , * , timeout = None /* Option */ , return_when = ALL_COMPLETED ) ;
        "Wait for the Futures || Tasks given by fs to complete.

    The fs iterable must !be empty.

    Coroutines will be wrapped in Tasks.

    Returns two sets of Future: (done, pending).

    Usage:

        done, pending = await asyncio.wait(fs)

    Note: This does !raise TimeoutError! Futures that aren't done
    when the timeout occurs are returned in the second set.
    ";
        if futures . isfuture ( fs ) || coroutines . iscoroutine ( fs ) {
        panic!("TypeError ( f "expect a list of futures, !{type(fs).__name__}" )");
        if !fs {
        panic!("ValueError ( "Set of Tasks/Futures is empty." )");
        if return_when !in ( FIRST_COMPLETED , FIRST_EXCEPTION , ALL_COMPLETED ) {
        panic!("ValueError ( f "Invalid return_when value: {return_when}" )");
        fs = set ( fs );
        if any ( coroutines . iscoroutine ( f ) for f in fs ) {
        panic!("TypeError ( "Passing coroutines is forbidden, use tasks explicitly." )");
        loop = events . get_running_loop ( );
        return  await _wait ( fs , timeout , return_when , loop );
        pub fn _release_waiter ( waiter , * args )  {
        if !waiter . done ( ) {
        waiter . set_result ( None /* Option */ );
        async def wait_for ( fut , timeout ) ;
        "Wait for the single Future || coroutine to complete, with timeout.

    Coroutine will be wrapped in Task.

    Returns result of the Future || coroutine.  When a timeout occurs,
    it cancels the task && raises TimeoutError.  To avoid the task
    cancellation, wrap it in shield().

    If the wait == cancelled, the task == also cancelled.

    This function == a coroutine.
    ";
        loop = events . get_running_loop ( );
        if timeout is None /* Option */ {
        return  await fut;
        if timeout <= 0 {
        fut = ensure_future ( fut , loop = loop );
        if fut . done ( ) {
        return  fut . result ( );
        await _cancel_and_wait ( fut , loop = loop );
        // try {
        return  fut . result ( );
        // } catch  exceptions . CancelledError as exc  {
        panic!("exceptions . TimeoutError ( ) from exc");
        waiter = loop . create_future ( );
        timeout_handle = loop . call_later ( timeout , _release_waiter , waiter );
        cb = functools . partial ( _release_waiter , waiter );
        fut = ensure_future ( fut , loop = loop );
        fut . add_done_callback ( cb );
        // try {
        // try {
        await waiter;
        // } catch  exceptions . CancelledError  {
        if fut . done ( ) {
        return  fut . result ( );
        } else {
        fut . remove_done_callback ( cb );
        await _cancel_and_wait ( fut , loop = loop );
        panic!("");
        if fut . done ( ) {
        return  fut . result ( );
        } else {
        fut . remove_done_callback ( cb );
        await _cancel_and_wait ( fut , loop = loop );
        // try {
        return  fut . result ( );
        // } catch  exceptions . CancelledError as exc  {
        panic!("exceptions . TimeoutError ( ) from exc");
        // } finally {
        timeout_handle . cancel ( );
        async def _wait ( fs , timeout , return_when , loop ) ;
        "Internal helper for wait().

    The fs argument must be a collection of Futures.
    ";
        assert fs , "Set of Futures == empty.";
        waiter = loop . create_future ( );
        timeout_handle = None /* Option */;
        if timeout is !None /* Option */ {
        timeout_handle = loop . call_later ( timeout , _release_waiter , waiter );
        counter = len ( fs );
        pub fn _on_completion ( f )  {
        nonlocal counter;
        counter - = 1;
        if ( counter <= 0 or {
        return _when == FIRST_COMPLETED or;
        return _when == FIRST_EXCEPTION && ( !f . cancelled ( ) and;
        f . exception ( ) == !None /* Option */ ) ) ;
        if timeout_handle is !None /* Option */ {
        timeout_handle . cancel ( );
        if !waiter . done ( ) {
        waiter . set_result ( None /* Option */ );
        for f in fs .iter() {
        f . add_done_callback ( _on_completion );
        // try {
        await waiter;
        // } finally {
        if timeout_handle is !None /* Option */ {
        timeout_handle . cancel ( );
        for f in fs .iter() {
        f . remove_done_callback ( _on_completion );
        done , pending = set ( ) , set ( );
        for f in fs .iter() {
        if f . done ( ) {
        done . add ( f );
        } else {
        pending . add ( f );
        return  done , pending;
        async def _cancel_and_wait ( fut , loop ) ;
        "Cancel the *fut* future || task && wait until it completes.";
        waiter = loop . create_future ( );
        cb = functools . partial ( _release_waiter , waiter );
        fut . add_done_callback ( cb );
        // try {
        fut . cancel ( );
        await waiter;
        // } finally {
        fut . remove_done_callback ( cb );
        pub fn as_completed ( fs , * , timeout = None /* Option */ )  {
        "Return an iterator whose values are coroutines.

    When waiting for the yielded coroutines you'll get the results (or
    exceptions!) of the original Futures (or coroutines), in the order
    in which && as soon as they complete.

    This differs from PEP 3148; the proper way to use this is:

        for f in as_completed(fs):
            result = await f  # The 'await' may raise.
            # Use result.

    If a timeout == specified, the 'await' will raise
    TimeoutError when the timeout occurs before all Futures are done.

    Note: The futures 'f' are !necessarily members of fs.
    ";
        if futures . isfuture ( fs ) || coroutines . iscoroutine ( fs ) {
        panic!("TypeError ( f "expect an iterable of futures, !{type(fs).__name__}" )");
        from . queues import Queue;
        done = Queue ( );
        loop = events . _get_event_loop ( );
        todo = { ensure_future ( f , loop = loop ) for f in set ( fs ) };
        timeout_handle = None /* Option */;
        pub fn _on_timeout ( )  {
        for f in todo .iter() {
        f . remove_done_callback ( _on_completion );
        done . put_nowait ( None /* Option */ );
        todo . clear ( );
        pub fn _on_completion ( f )  {
        if !todo {
        return;
        todo . remove ( f );
        done . put_nowait ( f );
        if !todo && timeout_handle is !None /* Option */ {
        timeout_handle . cancel ( );
        async def _wait_for_one ( ) ;
        f = await done . get ( );
        if f is None /* Option */ {
        panic!("exceptions . TimeoutError");
        return  f . result ( );
        for f in todo .iter() {
        f . add_done_callback ( _on_completion );
        if todo && timeout is !None /* Option */ {
        timeout_handle = loop . call_later ( timeout , _on_timeout );
        for _ in range ( len ( todo ) ) .iter() {
        yield _wait_for_one ( );
        @ types . coroutine;
        pub fn __sleep0 ( )  {
        "Skip one event loop run cycle.

    This == a private helper for 'asyncio.sleep()', used
    when the 'delay' == set to 0.  It uses a bare 'yield'
    expression (which Task.__step knows how to handle)
    instead of creating a Future object.
    ";
        yield;
        async def sleep ( delay , result = None /* Option */ ) ;
        "Coroutine that completes after a given time (in seconds).";
        if delay <= 0 {
        await __sleep0 ( );
        return  result;
        loop = events . get_running_loop ( );
        future = loop . create_future ( );
        h = loop . call_later ( delay ,;
        futures . _set_result_unless_cancelled ,;
        future , result );
        // try {
        return  await future;
        // } finally {
        h . cancel ( );
        pub fn ensure_future ( coro_or_future , * , loop = None /* Option */ )  {
        "Wrap a coroutine || an awaitable in a future.

    If the argument == a Future, it == returned directly.
    ";
        return  _ensure_future ( coro_or_future , loop = loop );
        pub fn _ensure_future ( coro_or_future , * , loop = None /* Option */ )  {
        if futures . isfuture ( coro_or_future ) {
        if loop is !None /* Option */ && loop is !futures . _get_loop ( coro_or_future ) {
        panic!("ValueError ( "The future belongs to a different loop than "");
        "the one specified as the loop argument" );
        return  coro_or_future;
        called_wrap_awaitable = false;
        if !coroutines . iscoroutine ( coro_or_future ) {
        if inspect . isawaitable ( coro_or_future ) {
        coro_or_future = _wrap_awaitable ( coro_or_future );
        called_wrap_awaitable = true;
        } else {
        panic!("TypeError ( "An asyncio.Future, a coroutine || an awaitable "");
        "is required" );
        if loop is None /* Option */ {
        loop = events . _get_event_loop ( stacklevel = 4 );
        // try {
        return  loop . create_task ( coro_or_future );
        // } catch  RuntimeError  {
        if !called_wrap_awaitable {
        coro_or_future . close ( );
        panic!("");
        @ types . coroutine;
        pub fn _wrap_awaitable ( awaitable )  {
        "Helper for asyncio.ensure_future().

    Wraps awaitable (an object with __await__) into a coroutine
    that will later be wrapped in a Task by ensure_future().
    ";
        return  ( yield from awaitable . __await__ ( ) );
        _wrap_awaitable . _is_coroutine = _is_coroutine;
        class _GatheringFuture ( futures . Future ) ;
        "Helper for gather().

    This overrides cancel() to cancel all the children && act more
    like Task.cancel(), which doesn't immediately mark itself as
    cancelled.
    ";
        pub fn __init__ ( &self, children , * , loop )  {
        assert loop == !None /* Option */;
        super ( ) . __init__ ( loop = loop );
        self . _children = children;
        self . _cancel_requested = false;
        pub fn cancel ( &self, msg = None /* Option */ )  {
        if self . done ( ) {
        return  false;
        ret = false;
        for child in self . _children .iter() {
        if child . cancel ( msg = msg ) {
        ret = true;
        if ret {
        self . _cancel_requested = true;
        return  ret;
        pub fn gather ( * coros_or_futures , return_exceptions = false )  {
        "Return a future aggregating results from the given coroutines/futures.

    Coroutines will be wrapped in a future && scheduled in the event
    loop. They will !necessarily be scheduled in the same order as
    passed in.

    All futures must share the same event loop.  If all the tasks are
    done successfully, the returned future's result == the list of
    results (in the order of the original sequence, !necessarily
    the order of results arrival).  If *return_exceptions* == true,
    exceptions in the tasks are treated the same as successful
    results, && gathered in the result list; otherwise, the first
    raised exception will be immediately propagated to the returned
    future.

    Cancellation: if the outer Future == cancelled, all children (that
    have !completed yet) are also cancelled.  If any child is
    cancelled, this == treated as if it raised CancelledError --
    the outer Future == *not* cancelled in this case.  (This == to
    prevent the cancellation of one child to cause other children to
    be cancelled.)

    If *return_exceptions* == false, cancelling gather() after it
    has been marked done won't cancel any submitted awaitables.
    For instance, gather can be marked done after propagating an
    exception to the caller, therefore, calling ``gather.cancel()``
    after catching an exception (raised by one of the awaitables) from
    gather won't cancel any other awaitables.
    ";
        if !coros_or_futures {
        loop = events . _get_event_loop ( );
        outer = loop . create_future ( );
        outer . set_result ( [ ] );
        return  outer;
        pub fn _done_callback ( fut )  {
        nonlocal nfinished;
        nfinished + = 1;
        if outer is None /* Option */ || outer . done ( ) {
        if !fut . cancelled ( ) {
        fut . exception ( );
        return;
        if !return_exceptions {
        if fut . cancelled ( ) {
        exc = fut . _make_cancelled_error ( );
        outer . set_exception ( exc );
        return;
        } else {
        exc = fut . exception ( );
        if exc is !None /* Option */ {
        outer . set_exception ( exc );
        return;
        if nfinished == nfuts {
        results = [ ];
        for fut in children .iter() {
        if fut . cancelled ( ) {
        res = exceptions . CancelledError (;
        "" if fut . _cancel_message == None /* Option */ else;
        fut . _cancel_message );
        } else {
        res = fut . exception ( );
        if res is None /* Option */ {
        res = fut . result ( );
        results . append ( res );
        if outer . _cancel_requested {
        exc = fut . _make_cancelled_error ( );
        outer . set_exception ( exc );
        } else {
        outer . set_result ( results );
        arg_to_fut = { };
        children = [ ];
        nfuts = 0;
        nfinished = 0;
        loop = None /* Option */;
        outer = None /* Option */;
        for arg in coros_or_futures .iter() {
        if arg !in arg_to_fut {
        fut = _ensure_future ( arg , loop = loop );
        if loop is None /* Option */ {
        loop = futures . _get_loop ( fut );
        if fut is !arg {
        fut . _log_destroy_pending = false;
        nfuts + = 1;
        arg_to_fut [ arg ] = fut;
        fut . add_done_callback ( _done_callback );
        } else {
        fut = arg_to_fut [ arg ];
        children . append ( fut );
        outer = _GatheringFuture ( children , loop = loop );
        return  outer;
        pub fn shield ( arg )  {
        "Wait for a future, shielding it from cancellation.

    The statement

        task = asyncio.create_task(something())
        res = await shield(task)

    == exactly equivalent to the statement

        res = await something()

    *except* that if the coroutine containing it == cancelled, the
    task running in something() == !cancelled.  From the POV of
    something(), the cancellation did !happen.  But its caller is
    still cancelled, so the yield-from expression still raises
    CancelledError.  Note: If something() == cancelled by other means
    this will still cancel shield().

    If you want to completely ignore cancellation (not recommended)
    you can combine shield() with a try/except clause, as follows:

        task = asyncio.create_task(something())
        try:
            res = await shield(task)
        except CancelledError:
            res = None /* Option */

    Save a reference to tasks passed to this function, to avoid
    a task disappearing mid-execution. The event loop only keeps
    weak references to tasks. A task that isn't referenced elsewhere
    may get garbage collected at any time, even before it's done.
    ";
        inner = _ensure_future ( arg );
        if inner . done ( ) {
        return  inner;
        loop = futures . _get_loop ( inner );
        outer = loop . create_future ( );
        pub fn _inner_done_callback ( inner )  {
        if outer . cancelled ( ) {
        if !inner . cancelled ( ) {
        inner . exception ( );
        return;
        if inner . cancelled ( ) {
        outer . cancel ( );
        } else {
        exc = inner . exception ( );
        if exc is !None /* Option */ {
        outer . set_exception ( exc );
        } else {
        outer . set_result ( inner . result ( ) );
        pub fn _outer_done_callback ( outer )  {
        if !inner . done ( ) {
        inner . remove_done_callback ( _inner_done_callback );
        inner . add_done_callback ( _inner_done_callback );
        outer . add_done_callback ( _outer_done_callback );
        return  outer;
        pub fn run_coroutine_threadsafe ( coro , loop )  {
        "Submit a coroutine object to a given event loop.

    Return a concurrent.futures.Future to access the result.
    ";
        if !coroutines . iscoroutine ( coro ) {
        panic!("TypeError ( "A coroutine object is required" )");
        future = concurrent . futures . Future ( );
        pub fn callback ( )  {
        // try {
        futures . _chain_future ( ensure_future ( coro , loop = loop ) , future );
        // } catch  ( SystemExit , KeyboardInterrupt )  {
        panic!("");
        // } catch  BaseException as exc  {
        if future . set_running_or_notify_cancel ( ) {
        future . set_exception ( exc );
        panic!("");
        loop . call_soon_threadsafe ( callback );
        return  future;
        _all_tasks = weakref . WeakSet ( );
        _current_tasks = { };
        pub fn _register_task ( task )  {
        "Register a new task in asyncio as executed by loop.";
        _all_tasks . add ( task );
        pub fn _enter_task ( loop , task )  {
        current_task = _current_tasks . get ( loop );
        if current_task is !None /* Option */ {
        panic!("RuntimeError ( f "Cannot enter into task {task!r} while another "");
        format!("task {current_task!r} == being executed." ));
        _current_tasks [ loop ] = task;
        pub fn _leave_task ( loop , task )  {
        current_task = _current_tasks . get ( loop );
        if current_task is !task {
        panic!("RuntimeError ( f "Leaving task {task!r} does !match "");
        format!("the current task {current_task!r}." ));
        del _current_tasks [ loop ];
        pub fn _unregister_task ( task )  {
        "Unregister a task.";
        _all_tasks . discard ( task );
        _py_register_task = _register_task;
        _py_unregister_task = _unregister_task;
        _py_enter_task = _enter_task;
        _py_leave_task = _leave_task;
        // try {
        from _asyncio import ( _register_task , _unregister_task ,;
        _enter_task , _leave_task ,;
        _all_tasks , _current_tasks );
        // } catch  ImportError  {
        // pass
        } else {
        _c_register_task = _register_task;
        _c_unregister_task = _unregister_task;
        _c_enter_task = _enter_task;
        _c_leave_task = _leave_task;
}

