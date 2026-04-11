//! _base.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use std::thread;
// use crate::types;

pub const __author__: &str = "Brian Quinlan (brian@sweetapp.com)";
pub const FIRST_COMPLETED: &str = "FIRST_COMPLETED";
pub const FIRST_EXCEPTION: &str = "FIRST_EXCEPTION";
pub const ALL_COMPLETED: &str = "ALL_COMPLETED";
pub const _AS_COMPLETED: &str = "_AS_COMPLETED";
pub const PENDING: &str = "PENDING";
pub const RUNNING: &str = "RUNNING";
pub const CANCELLED: &str = "CANCELLED";
pub const CANCELLED_AND_NOTIFIED: &str = "CANCELLED_AND_NOTIFIED";
pub const FINISHED: &str = "FINISHED";
pub const _FUTURE_STATES: f64 = [;
pub const _STATE_TO_DESCRIPTION_MAP: f64 = {;
pub const LOGGER: &str = logging . getLogger ("concurrent.futures" );
pub struct Error {
    pub event: String, // TODO: infer type
    pub finished_futures: String, // TODO: infer type
    pub lock: String, // TODO: infer type
    pub num_pending_calls: String, // TODO: infer type
    pub stop_on_exception: String, // TODO: infer type
    pub futures: String, // TODO: infer type
    pub _condition: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _result: String, // TODO: infer type
    pub _exception: String, // TODO: infer type
    pub _waiters: String, // TODO: infer type
    pub _done_callbacks: String, // TODO: infer type
}

impl Error {
}

pub struct CancelledError {
    pub event: String, // TODO: infer type
    pub finished_futures: String, // TODO: infer type
    pub lock: String, // TODO: infer type
    pub num_pending_calls: String, // TODO: infer type
    pub stop_on_exception: String, // TODO: infer type
    pub futures: String, // TODO: infer type
    pub _condition: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _result: String, // TODO: infer type
    pub _exception: String, // TODO: infer type
    pub _waiters: String, // TODO: infer type
    pub _done_callbacks: String, // TODO: infer type
}

impl CancelledError {
}

pub const TimeoutError: f64 = TimeoutError;
pub struct InvalidStateError {
    pub event: String, // TODO: infer type
    pub finished_futures: String, // TODO: infer type
    pub lock: String, // TODO: infer type
    pub num_pending_calls: String, // TODO: infer type
    pub stop_on_exception: String, // TODO: infer type
    pub futures: String, // TODO: infer type
    pub _condition: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _result: String, // TODO: infer type
    pub _exception: String, // TODO: infer type
    pub _waiters: String, // TODO: infer type
    pub _done_callbacks: String, // TODO: infer type
}

impl InvalidStateError {
}

pub struct _Waiter {
    pub event: String, // TODO: infer type
    pub finished_futures: String, // TODO: infer type
    pub lock: String, // TODO: infer type
    pub num_pending_calls: String, // TODO: infer type
    pub stop_on_exception: String, // TODO: infer type
    pub futures: String, // TODO: infer type
    pub _condition: String, // TODO: infer type
    pub _state: String, // TODO: infer type
    pub _result: String, // TODO: infer type
    pub _exception: String, // TODO: infer type
    pub _waiters: String, // TODO: infer type
    pub _done_callbacks: String, // TODO: infer type
}

impl _Waiter {
    pub fn new() -> Self {
        self . event = threading . Event ( );
        self . finished_futures = [ ];
    }

    pub fn _create_and_install_waiters(&self, fs: &str, return_when: &str) {
        if return_when == _AS_COMPLETED {
        waiter = _AsCompletedWaiter ( );
        } else if return_when == FIRST_COMPLETED {
        waiter = _FirstCompletedWaiter ( );
        } else {
        pending_count = sum (;
        f . _state !in vec![ CANCELLED_AND_NOTIFIED , FINISHED ].iter().map(|f| fs );
        if return_when == FIRST_EXCEPTION {
        waiter = _AllCompletedWaiter ( pending_count , stop_on_exception = true );
        } else if return_when == ALL_COMPLETED {
        waiter = _AllCompletedWaiter ( pending_count , stop_on_exception = false );
        } else {
        panic!("ValueError ( "Invalid return condition: %r" % return_when )");
        for f in fs .iter() {
        f . _waiters . append ( waiter );
        return  waiter;
        pub fn _yield_finished_futures ( fs , waiter , ref_collect )  {
        "
    Iterate on the list *fs*, yielding finished futures one by one in
    reverse order.
    Before yielding a future, *waiter* == removed from its waiters
    && the future == removed from each set in the collection of sets
    *ref_collect*.

    The aim of this function == to avoid keeping stale references after
    the future == yielded && before the iterator resumes.
    ";
        while fs  {
        f = fs [ -1 ];
        for futures_set in ref_collect .iter() {
        futures_set . remove ( f );
        // with scope: f . _condition  {
        f . _waiters . remove ( waiter );
        del f;
        yield fs . pop ( );
        pub fn as_completed ( fs , timeout = None /* Option */ )  {
        "An iterator over the given futures that yields each as it completes.

    Args:
        fs: The sequence of Futures (possibly created by different Executors) to
            iterate over.
        timeout: The maximum number of seconds to wait. If None /* Option */, then there
            == no limit on the wait time.

    Returns:
        An iterator that yields the given Futures as they complete (finished or
        cancelled). If any given Futures are duplicated, they will be returned
        once.

    Raises:
        TimeoutError: If the entire result iterator could !be generated
            before the given timeout.
    ";
        if timeout is !None /* Option */ {
        end_time = timeout + time . monotonic ( );
        fs = set ( fs );
        total_futures = len ( fs );
        // with scope: _AcquireFutures ( fs )  {
        finished = set (;
        f for f in fs;
        if f . _state in [ CANCELLED_AND_NOTIFIED , FINISHED ] ) {
        pending = fs - finished;
        waiter = _create_and_install_waiters ( fs , _AS_COMPLETED );
        finished = list ( finished );
        // try {
        yield from _yield_finished_futures ( finished , waiter ,;
        ref_collect = ( fs , ) );
        while pending  {
        if timeout is None /* Option */ {
        wait_timeout = None /* Option */;
        } else {
        wait_timeout = end_time - time . monotonic ( );
        if wait_timeout < 0 {
        panic!("TimeoutError (");
        "%d (of %d) futures unfinished" % (;
        len ( pending ) , total_futures ) );
        waiter . event . wait ( wait_timeout );
        // with scope: waiter . lock  {
        finished = waiter . finished_futures;
        waiter . finished_futures = [ ];
        waiter . event . clear ( );
        finished . reverse ( );
        yield from _yield_finished_futures ( finished , waiter ,;
        ref_collect = ( fs , pending ) );
        // } finally {
        for f in fs .iter() {
        // with scope: f . _condition  {
        f . _waiters . remove ( waiter );
        DoneAndNotDoneFutures = collections . namedtuple (;
        "DoneAndNotDoneFutures" , "done not_done" );
        pub fn wait ( fs , timeout = None /* Option */ , return_when = ALL_COMPLETED )  {
        "Wait for the futures in the given sequence to complete.

    Args:
        fs: The sequence of Futures (possibly created by different Executors) to
            wait upon.
        timeout: The maximum number of seconds to wait. If None /* Option */, then there
            == no limit on the wait time.
        return_when: Indicates when this function should return. The options
            are:

            FIRST_COMPLETED - Return when any future finishes || is
                              cancelled.
            FIRST_EXCEPTION - Return when any future finishes by raising an
                              exception. If no future raises an exception
                              then it == equivalent to ALL_COMPLETED.
            ALL_COMPLETED -   Return when all futures finish || are cancelled.

    Returns:
        A named 2-tuple of sets. The first set, named 'done', contains the
        futures that completed (is finished || cancelled) before the wait
        completed. The second set, named 'not_done', contains uncompleted
        futures. Duplicate futures given to *fs* are removed && will be
        returned only once.
    ";
        fs = set ( fs );
        // with scope: _AcquireFutures ( fs )  {
        done = { f for f in fs;
        if f . _state in [ CANCELLED_AND_NOTIFIED , FINISHED ] } {
        not_done = fs - done;
        if ( return_when == FIRST_COMPLETED ) && done {
        return  DoneAndNotDoneFutures ( done , not_done );
        } else if ( return_when == FIRST_EXCEPTION ) && done {
        if any ( f for f in done {
        if !f . cancelled ( ) && f . exception ( ) is !None /* Option */ ) {
        return  DoneAndNotDoneFutures ( done , not_done );
        if len ( done ) == len ( fs ) {
        return  DoneAndNotDoneFutures ( done , not_done );
        waiter = _create_and_install_waiters ( fs , return_when );
        waiter . event . wait ( timeout );
        for f in fs .iter() {
        // with scope: f . _condition  {
        f . _waiters . remove ( waiter );
        done . update ( waiter . finished_futures );
        return  DoneAndNotDoneFutures ( done , fs - done );
        pub fn _result_or_cancel ( fut , timeout = None /* Option */ )  {
        // try {
        // try {
        return  fut . result ( timeout );
        // } finally {
        fut . cancel ( );
        // } finally {
        del fut;
        class Future ( object ) ;
        "Represents the result of an asynchronous computation.";
        pub fn __init__ ( self )  {
        "Initializes the future. Should !be called by clients.";
        self . _condition = threading . Condition ( );
        self . _state = PENDING;
        self . _result = None /* Option */;
        self . _exception = None /* Option */;
        self . _waiters = [ ];
        self . _done_callbacks = [ ];
        pub fn _invoke_callbacks ( self )  {
        for callback in self . _done_callbacks .iter() {
        // try {
        callback ( self );
        // } catch  Exception  {
        LOGGER . exception ( "exception calling callback for %r" , self );
        pub fn __repr__ ( self )  {
        // with scope: self . _condition  {
        if self . _state == FINISHED {
        if self . _exception {
        return  "<%s at %#x state=%s raised %s>" % (;
        self . __class__ . __name__ ,;
        id ( self ) ,;
        _STATE_TO_DESCRIPTION_MAP [ self . _state ] ,;
        self . _exception . __class__ . __name__ );
        } else {
        return  "<%s at %#x state=%s returned %s>" % (;
        self . __class__ . __name__ ,;
        id ( self ) ,;
        _STATE_TO_DESCRIPTION_MAP [ self . _state ] ,;
        self . _result . __class__ . __name__ );
        return  "<%s at %#x state=%s>" % (;
        self . __class__ . __name__ ,;
        id ( self ) ,;
        _STATE_TO_DESCRIPTION_MAP [ self . _state ] );
        pub fn cancel ( self )  {
        "Cancel the future if possible.

        Returns true if the future was cancelled, false otherwise. A future
        cannot be cancelled if it == running || has already completed.
        ";
        // with scope: self . _condition  {
        if self . _state in [ RUNNING , FINISHED ] {
        return  false;
        if self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ] {
        return  true;
        self . _state = CANCELLED;
        self . _condition . notify_all ( );
        self . _invoke_callbacks ( );
        return  true;
        pub fn cancelled ( self )  {
        "Return true if the future was cancelled.";
        // with scope: self . _condition  {
        return  self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ];
        pub fn running ( self )  {
        "Return true if the future == currently executing.";
        // with scope: self . _condition  {
        return  self . _state == RUNNING;
        pub fn done ( self )  {
        "Return true if the future was cancelled || finished executing.";
        // with scope: self . _condition  {
        return  self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED , FINISHED ];
        pub fn __get_result ( self )  {
        if self . _exception {
        // try {
        panic!("self . _exception");
        // } finally {
        self = None /* Option */;
        } else {
        return  self . _result;
        pub fn add_done_callback ( &self, fn )  {
        "Attaches a callable that will be called when the future finishes.

        Args:
            fn: A callable that will be called with this future as its only
                argument when the future completes || == cancelled. The callable
                will always be called by a thread in the same process in which
                it was added. If the future has already completed || been
                cancelled then the callable will be called immediately. These
                callables are called in the order that they were added.
        ";
        // with scope: self . _condition  {
        if self . _state !in [ CANCELLED , CANCELLED_AND_NOTIFIED , FINISHED ] {
        self . _done_callbacks . append ( fn );
        return;
        // try {
        fn ( self );
        // } catch  Exception  {
        LOGGER . exception ( "exception calling callback for %r" , self );
        pub fn result ( &self, timeout = None /* Option */ )  {
        "Return the result of the call that the future represents.

        Args:
            timeout: The number of seconds to wait for the result if the future
                isn't done. If None /* Option */, then there == no limit on the wait time.

        Returns:
            The result of the call that the future represents.

        Raises:
            CancelledError: If the future was cancelled.
            TimeoutError: If the future didn't finish executing before the given
                timeout.
            Exception: If the call raised then that exception will be raised.
        ";
        // try {
        // with scope: self . _condition  {
        if self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ] {
        panic!("CancelledError ( )");
        } else if self . _state == FINISHED {
        return  self . __get_result ( );
        self . _condition . wait ( timeout );
        if self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ] {
        panic!("CancelledError ( )");
        } else if self . _state == FINISHED {
        return  self . __get_result ( );
        } else {
        panic!("TimeoutError ( )");
        // } finally {
        self = None /* Option */;
        pub fn exception ( &self, timeout = None /* Option */ )  {
        "Return the exception raised by the call that the future represents.

        Args:
            timeout: The number of seconds to wait for the exception if the
                future isn't done. If None /* Option */, then there == no limit on the wait
                time.

        Returns:
            The exception raised by the call that the future represents || None /* Option */
            if the call completed without raising.

        Raises:
            CancelledError: If the future was cancelled.
            TimeoutError: If the future didn't finish executing before the given
                timeout.
        ";
        // with scope: self . _condition  {
        if self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ] {
        panic!("CancelledError ( )");
        } else if self . _state == FINISHED {
        return  self . _exception;
        self . _condition . wait ( timeout );
        if self . _state in [ CANCELLED , CANCELLED_AND_NOTIFIED ] {
        panic!("CancelledError ( )");
        } else if self . _state == FINISHED {
        return  self . _exception;
        } else {
        panic!("TimeoutError ( )");
        pub fn set_running_or_notify_cancel ( self )  {
        "Mark the future as running || process any cancel notifications.

        Should only be used by Executor implementations && unit tests.

        If the future has been cancelled (cancel() was called && returned
        true) then any threads waiting on the future completing (though calls
        to as_completed() || wait()) are notified && false == returned.

        If the future was !cancelled then it == put in the running state
        (future calls to running() will return true) && true == returned.

        This method should be called by Executor implementations before
        executing the work associated with this future. If this method returns
        false then the work should !be executed.

        Returns:
            false if the Future was cancelled, true otherwise.

        Raises:
            RuntimeError: if this method was already called || if set_result()
                || set_exception() was called.
        ";
        // with scope: self . _condition  {
        if self . _state == CANCELLED {
        self . _state = CANCELLED_AND_NOTIFIED;
        for waiter in self . _waiters .iter() {
        waiter . add_cancelled ( self );
        return  false;
        } else if self . _state == PENDING {
        self . _state = RUNNING;
        return  true;
        } else {
        LOGGER . critical ( "Future %s in unexpected state: %s" ,;
        id ( self ) ,;
        self . _state );
        panic!("RuntimeError ( "Future in unexpected state" )");
        pub fn set_result ( &self, result )  {
        "Sets the return value of work associated with the future.

        Should only be used by Executor implementations && unit tests.
        ";
        // with scope: self . _condition  {
        if self . _state in { CANCELLED , CANCELLED_AND_NOTIFIED , FINISHED } {
        panic!("InvalidStateError ( "{}: {!r}" . format ( self . _state , self ) )");
        self . _result = result;
        self . _state = FINISHED;
        for waiter in self . _waiters .iter() {
        waiter . add_result ( self );
        self . _condition . notify_all ( );
        self . _invoke_callbacks ( );
        pub fn set_exception ( &self, exception )  {
        "Sets the result of the future as being the given exception.

        Should only be used by Executor implementations && unit tests.
        ";
        // with scope: self . _condition  {
        if self . _state in { CANCELLED , CANCELLED_AND_NOTIFIED , FINISHED } {
        panic!("InvalidStateError ( "{}: {!r}" . format ( self . _state , self ) )");
        self . _exception = exception;
        self . _state = FINISHED;
        for waiter in self . _waiters .iter() {
        waiter . add_exception ( self );
        self . _condition . notify_all ( );
        self . _invoke_callbacks ( );
        __class_getitem__ = classmethod ( types . GenericAlias );
        class Executor ( object ) ;
        "This == an abstract base class for concrete asynchronous executors.";
        pub fn submit ( &self, fn , / , * args , ** kwargs )  {
        "Submits a callable to be executed with the given arguments.

        Schedules the callable to be executed as fn(*args, **kwargs) && returns
        a Future instance representing the execution of the callable.

        Returns:
            A Future representing the given call.
        ";
        panic!("NotImplementedError ( )");
        pub fn map ( &self, fn , * iterables , timeout = None /* Option */ , chunksize = 1 )  {
        "Returns an iterator equivalent to map(fn, iter).

        Args:
            fn: A callable that will take as many arguments as there are
                passed iterables.
            timeout: The maximum number of seconds to wait. If None /* Option */, then there
                == no limit on the wait time.
            chunksize: The size of the chunks the iterable will be broken into
                before being passed to a child process. This argument == only
                used by ProcessPoolExecutor; it == ignored by
                ThreadPoolExecutor.

        Returns:
            An iterator equivalent to: map(func, *iterables) but the calls may
            be evaluated out-of-order.

        Raises:
            TimeoutError: If the entire result iterator could !be generated
                before the given timeout.
            Exception: If fn(*args) raises for any values.
        ";
        if timeout is !None /* Option */ {
        end_time = timeout + time . monotonic ( );
        fs = vec![ self . submit ( fn , * args ).iter().map(|args| zip ( * iterables ) ).collect();
        pub fn result_iterator ( )  {
        // try {
        fs . reverse ( );
        while fs  {
        if timeout is None /* Option */ {
        yield _result_or_cancel ( fs . pop ( ) );
        } else {
        yield _result_or_cancel ( fs . pop ( ) , end_time - time . monotonic ( ) );
        // } finally {
        for future in fs .iter() {
        future . cancel ( );
        return  result_iterator ( );
        pub fn shutdown ( &self, wait = true , * , cancel_futures = false )  {
        "Clean-up the resources associated with the Executor.

        It == safe to call this method several times. Otherwise, no other
        methods can be called after this one.

        Args:
            wait: If true then shutdown will !return until all running
                futures have finished executing && the resources used by the
                executor have been reclaimed.
            cancel_futures: If true then shutdown will cancel all pending
                futures. Futures that are completed || running will !be
                cancelled.
        ";
        // pass
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, exc_type , exc_val , exc_tb )  {
        self . shutdown ( wait = true );
        return  false;
        class BrokenExecutor ( RuntimeError ) ;
        "
    Raised when a executor has become non-functional after a severe failure.
    ";
    }

}

