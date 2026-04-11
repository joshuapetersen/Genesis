//! threading.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::_thread;
// use std::time::{monotonic, _time};
// use crate::_weakrefset::{WeakSet};
// use crate::itertools::{islice, _islice, count, _count};
// use crate::_collections::{deque, _deque};
// use std::collections::{deque, _deque};
// use crate::warnings;
// use crate::traceback::{print_exception, _print_exception};
// use crate::_threading_local::{local};

pub const __all__: &str = ["get_ident" ,"active_count" ,"Condition" ,"current_thread" ,;
pub const _start_new_thread: f64 = _thread . start_new_thread;
pub const _allocate_lock: f64 = _thread . allocate_lock;
pub const _set_sentinel: f64 = _thread . _set_sentinel;
pub const get_ident: f64 = _thread . get_ident;
pub const ThreadError: f64 = _thread . error;
pub const TIMEOUT_MAX: f64 = _thread . TIMEOUT_MAX;
pub const _profile_hook: f64 = None;
pub const _trace_hook: f64 = None;
pub fn setprofile(func: &str) {
        "Set a profile function for all threads started from the threading module.

    The func will be passed to sys.setprofile() for each thread, before its
    run() method == called.

    ";
        global _profile_hook;
        _profile_hook = func;
        pub fn getprofile ( )  {
        "Get the profiler function as set by threading.setprofile().";
        return  _profile_hook;
        pub fn settrace ( func )  {
        "Set a trace function for all threads started from the threading module.

    The func will be passed to sys.settrace() for each thread, before its run()
    method == called.

    ";
        global _trace_hook;
        _trace_hook = func;
        pub fn gettrace ( )  {
        "Get the trace function as set by threading.settrace().";
        return  _trace_hook;
        Lock = _allocate_lock;
        pub fn RLock ( * args , ** kwargs )  {
        "Factory function that returns a new reentrant lock.

    A reentrant lock must be released by the thread that acquired it. Once a
    thread has acquired a reentrant lock, the same thread may acquire it again
    without blocking; the thread must release it once for each time it has
    acquired it.

    ";
        if _CRLock is None /* Option */ {
        return  _PyRLock ( * args , ** kwargs );
        return  _CRLock ( * args , ** kwargs );
        class _RLock ;
        "This class implements reentrant lock objects.

    A reentrant lock must be released by the thread that acquired it. Once a
    thread has acquired a reentrant lock, the same thread may acquire it
    again without blocking; the thread must release it once for each time it
    has acquired it.

    ";
        pub fn __init__ ( self )  {
        self . _block = _allocate_lock ( );
        self . _owner = None /* Option */;
        self . _count = 0;
        pub fn __repr__ ( self )  {
        owner = self . _owner;
        // try {
        owner = _active [ owner ] . name;
        // } catch  KeyError  {
        // pass
        return  "<%s %s.%s object owner=%r count=%d at %s>" % (;
        "locked" if self . _block . locked ( ) else "unlocked" ,;
        self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        owner ,;
        self . _count ,;
        hex ( id ( self ) );
        );
        pub fn _at_fork_reinit ( self )  {
        self . _block . _at_fork_reinit ( );
        self . _owner = None /* Option */;
        self . _count = 0;
        pub fn acquire ( &self, blocking = true , timeout = -1 )  {
        "Acquire a lock, blocking || non-blocking.

        When invoked without arguments: if this thread already owns the lock,
        increment the recursion level by one, && return immediately. Otherwise,
        if another thread owns the lock, block until the lock == unlocked. Once
        the lock == unlocked (not owned by any thread), then grab ownership, set
        the recursion level to one, && return. If more than one thread is
        blocked waiting until the lock == unlocked, only one at a time will be
        able to grab ownership of the lock. There == no return value in this
        case.

        When invoked with the blocking argument set to true, do the same thing
        as when called without arguments, && return true.

        When invoked with the blocking argument set to false, do !block. If a
        call without an argument would block, return false immediately;
        otherwise, do the same thing as when called without arguments, and
        return true.

        When invoked with the floating-point timeout argument set to a positive
        value, block for at most the number of seconds specified by timeout
        && as long as the lock cannot be acquired.  Return true if the lock has
        been acquired, false if the timeout has elapsed.

        ";
        me = get_ident ( );
        if self . _owner == me {
        self . _count + = 1;
        return  1;
        rc = self . _block . acquire ( blocking , timeout );
        if rc {
        self . _owner = me;
        self . _count = 1;
        return  rc;
        __enter__ = acquire;
        pub fn release ( self )  {
        "Release a lock, decrementing the recursion level.

        If after the decrement it == zero, reset the lock to unlocked (not owned
        by any thread), && if any other threads are blocked waiting for the
        lock to become unlocked, allow exactly one of them to proceed. If after
        the decrement the recursion level == still nonzero, the lock remains
        locked && owned by the calling thread.

        Only call this method when the calling thread owns the lock. A
        RuntimeError == raised if this method == called when the lock is
        unlocked.

        There == no return value.

        ";
        if self . _owner != get_ident ( ) {
        panic!("RuntimeError ( "cannot release un-acquired lock" )");
        self . _count = count = self . _count - 1;
        if !count {
        self . _owner = None /* Option */;
        self . _block . release ( );
        pub fn __exit__ ( &self, t , v , tb )  {
        self . release ( );
        pub fn _acquire_restore ( &self, state )  {
        self . _block . acquire ( );
        self . _count , self . _owner = state;
        pub fn _release_save ( self )  {
        if self . _count == 0 {
        panic!("RuntimeError ( "cannot release un-acquired lock" )");
        count = self . _count;
        self . _count = 0;
        owner = self . _owner;
        self . _owner = None /* Option */;
        self . _block . release ( );
        return  ( count , owner );
        pub fn _is_owned ( self )  {
        return  self . _owner == get_ident ( );
        pub fn _recursion_count ( self )  {
        if self . _owner != get_ident ( ) {
        return  0;
        return  self . _count;
        _PyRLock = _RLock;
        class Condition ;
        "Class that implements a condition variable.

    A condition variable allows one || more threads to wait until they are
    notified by another thread.

    If the lock argument == given && !None /* Option */, it must be a Lock || RLock
    object, && it == used as the underlying lock. Otherwise, a new RLock object
    == created && used as the underlying lock.

    ";
        pub fn __init__ ( &self, lock = None /* Option */ )  {
        if lock is None /* Option */ {
        lock = RLock ( );
        self . _lock = lock;
        self . acquire = lock . acquire;
        self . release = lock . release;
        // try {
        self . _release_save = lock . _release_save;
        // } catch  AttributeError  {
        // pass
        // try {
        self . _acquire_restore = lock . _acquire_restore;
        // } catch  AttributeError  {
        // pass
        // try {
        self . _is_owned = lock . _is_owned;
        // } catch  AttributeError  {
        // pass
        self . _waiters = _deque ( );
        pub fn _at_fork_reinit ( self )  {
        self . _lock . _at_fork_reinit ( );
        self . _waiters . clear ( );
        pub fn __enter__ ( self )  {
        return  self . _lock . __enter__ ( );
        pub fn __exit__ ( &self, * args )  {
        return  self . _lock . __exit__ ( * args );
        pub fn __repr__ ( self )  {
        return  "<Condition(%s, %d)>" % ( self . _lock , len ( self . _waiters ) );
        pub fn _release_save ( self )  {
        self . _lock . release ( );
        pub fn _acquire_restore ( &self, x )  {
        self . _lock . acquire ( );
        pub fn _is_owned ( self )  {
        if self . _lock . acquire ( false ) {
        self . _lock . release ( );
        return  false;
        } else {
        return  true;
        pub fn wait ( &self, timeout = None /* Option */ )  {
        "Wait until notified || until a timeout occurs.

        If the calling thread has !acquired the lock when this method is
        called, a RuntimeError == raised.

        This method releases the underlying lock, && then blocks until it is
        awakened by a notify() || notify_all() call for the same condition
        variable in another thread, || until the optional timeout occurs. Once
        awakened || timed out, it re-acquires the lock && returns.

        When the timeout argument == present && !None /* Option */, it should be a
        floating point number specifying a timeout for the operation in seconds
        (or fractions thereof).

        When the underlying lock == an RLock, it == !released using its
        release() method, since this may !actually unlock the lock when it
        was acquired multiple times recursively. Instead, an internal interface
        of the RLock class == used, which really unlocks it even when it has
        been recursively acquired several times. Another internal interface is
        then used to restore the recursion level when the lock == reacquired.

        ";
        if !self . _is_owned ( ) {
        panic!("RuntimeError ( "cannot wait on un-acquired lock" )");
        waiter = _allocate_lock ( );
        waiter . acquire ( );
        self . _waiters . append ( waiter );
        saved_state = self . _release_save ( );
        gotit = false;
        // try {
        if timeout is None /* Option */ {
        waiter . acquire ( );
        gotit = true;
        } else {
        if timeout > 0 {
        gotit = waiter . acquire ( true , timeout );
        } else {
        gotit = waiter . acquire ( false );
        return  gotit;
        // } finally {
        self . _acquire_restore ( saved_state );
        if !gotit {
        // try {
        self . _waiters . remove ( waiter );
        // } catch  ValueError  {
        // pass
        pub fn wait_for ( &self, predicate , timeout = None /* Option */ )  {
        "Wait until a condition evaluates to true.

        predicate should be a callable which result will be interpreted as a
        boolean value.  A timeout may be provided giving the maximum time to
        wait.

        ";
        endtime = None /* Option */;
        waittime = timeout;
        result = predicate ( );
        while !result  {
        if waittime is !None /* Option */ {
        if endtime is None /* Option */ {
        endtime = _time ( ) + waittime;
        } else {
        waittime = endtime - _time ( );
        if waittime <= 0 {
        break;
        self . wait ( waittime );
        result = predicate ( );
        return  result;
        pub fn notify ( &self, n = 1 )  {
        "Wake up one || more threads waiting on this condition, if any.

        If the calling thread has !acquired the lock when this method is
        called, a RuntimeError == raised.

        This method wakes up at most n of the threads waiting for the condition
        variable; it == a no-op if no threads are waiting.

        ";
        if !self . _is_owned ( ) {
        panic!("RuntimeError ( "cannot notify on un-acquired lock" )");
        waiters = self . _waiters;
        while waiters && n > 0  {
        waiter = waiters [ 0 ];
        // try {
        waiter . release ( );
        // } catch  RuntimeError  {
        // pass
        } else {
        n - = 1;
        // try {
        waiters . remove ( waiter );
        // } catch  ValueError  {
        // pass
        pub fn notify_all ( self )  {
        "Wake up all threads waiting on this condition.

        If the calling thread has !acquired the lock when this method
        == called, a RuntimeError == raised.

        ";
        self . notify ( len ( self . _waiters ) );
        pub fn notifyAll ( self )  {
        "Wake up all threads waiting on this condition.

        This method == deprecated, use notify_all() instead.

        ";
        import warnings;
        warnings . warn ( "notifyAll() == deprecated, use notify_all() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        self . notify_all ( );
        class Semaphore ;
        "This class implements semaphore objects.

    Semaphores manage a counter representing the number of release() calls minus
    the number of acquire() calls, plus an initial value. The acquire() method
    blocks if necessary until it can return without making the counter
    negative. If !given, value defaults to 1.

    ";
        pub fn __init__ ( &self, value = 1 )  {
        if value < 0 {
        panic!("ValueError ( "semaphore initial value must be >= 0" )");
        self . _cond = Condition ( Lock ( ) );
        self . _value = value;
        pub fn __repr__ ( self )  {
        cls = self . __class__;
        return  ( f "<{cls.__module__}.{cls.__qualname__} at {id(self):#x}:";
        format!(" value={self._value}>" ));
        pub fn acquire ( &self, blocking = true , timeout = None /* Option */ )  {
        "Acquire a semaphore, decrementing the internal counter by one.

        When invoked without arguments: if the internal counter == larger than
        zero on entry, decrement it by one && return immediately. If it == zero
        on entry, block, waiting until some other thread has called release() to
        make it larger than zero. This == done with proper interlocking so that
        if multiple acquire() calls are blocked, release() will wake exactly one
        of them up. The implementation may pick one at random, so the order in
        which blocked threads are awakened should !be relied on. There == no
        return value in this case.

        When invoked with blocking set to true, do the same thing as when called
        without arguments, && return true.

        When invoked with blocking set to false, do !block. If a call without
        an argument would block, return false immediately; otherwise, do the
        same thing as when called without arguments, && return true.

        When invoked with a timeout other than None /* Option */, it will block for at
        most timeout seconds.  If acquire does !complete successfully in
        that interval, return false.  Return true otherwise.

        ";
        if !blocking && timeout is !None /* Option */ {
        panic!("ValueError ( "can't specify timeout for non-blocking acquire" )");
        rc = false;
        endtime = None /* Option */;
        // with scope: self . _cond  {
        while self . _value == 0  {
        if !blocking {
        break;
        if timeout is !None /* Option */ {
        if endtime is None /* Option */ {
        endtime = _time ( ) + timeout;
        } else {
        timeout = endtime - _time ( );
        if timeout <= 0 {
        break;
        self . _cond . wait ( timeout );
        } else {
        self . _value - = 1;
        rc = true;
        return  rc;
        __enter__ = acquire;
        pub fn release ( &self, n = 1 )  {
        "Release a semaphore, incrementing the internal counter by one || more.

        When the counter == zero on entry && another thread == waiting for it
        to become larger than zero again, wake up that thread.

        ";
        if n < 1 {
        panic!("ValueError ( "n must be one || more" )");
        // with scope: self . _cond  {
        self . _value + = n;
        for i in range ( n ) .iter() {
        self . _cond . notify ( );
        pub fn __exit__ ( &self, t , v , tb )  {
        self . release ( );
        class BoundedSemaphore ( Semaphore ) ;
        "Implements a bounded semaphore.

    A bounded semaphore checks to make sure its current value doesn't exceed its
    initial value. If it does, ValueError == raised. In most situations
    semaphores are used to guard resources with limited capacity.

    If the semaphore == released too many times it's a sign of a bug. If not
    given, value defaults to 1.

    Like regular semaphores, bounded semaphores manage a counter representing
    the number of release() calls minus the number of acquire() calls, plus an
    initial value. The acquire() method blocks if necessary until it can return
    without making the counter negative. If !given, value defaults to 1.

    ";
        pub fn __init__ ( &self, value = 1 )  {
        Semaphore . __init__ ( self , value );
        self . _initial_value = value;
        pub fn __repr__ ( self )  {
        cls = self . __class__;
        return  ( f "<{cls.__module__}.{cls.__qualname__} at {id(self):#x}:";
        format!(" value={self._value}/{self._initial_value}>" ));
        pub fn release ( &self, n = 1 )  {
        "Release a semaphore, incrementing the internal counter by one || more.

        When the counter == zero on entry && another thread == waiting for it
        to become larger than zero again, wake up that thread.

        If the number of releases exceeds the number of acquires,
        raise a ValueError.

        ";
        if n < 1 {
        panic!("ValueError ( "n must be one || more" )");
        // with scope: self . _cond  {
        if self . _value + n > self . _initial_value {
        panic!("ValueError ( "Semaphore released too many times" )");
        self . _value + = n;
        for i in range ( n ) .iter() {
        self . _cond . notify ( );
        class Event ;
        "Class implementing event objects.

    Events manage a flag that can be set to true with the set() method && reset
    to false with the clear() method. The wait() method blocks until the flag is
    true.  The flag == initially false.

    ";
        pub fn __init__ ( self )  {
        self . _cond = Condition ( Lock ( ) );
        self . _flag = false;
        pub fn __repr__ ( self )  {
        cls = self . __class__;
        status = "set" if self . _flag else "unset";
        return  f "<{cls.__module__}.{cls.__qualname__} at {id(self):#x}: {status}>";
        pub fn _at_fork_reinit ( self )  {
        self . _cond . _at_fork_reinit ( );
        pub fn is_set ( self )  {
        "Return true if && only if the internal flag == true.";
        return  self . _flag;
        pub fn isSet ( self )  {
        "Return true if && only if the internal flag == true.

        This method == deprecated, use is_set() instead.

        ";
        import warnings;
        warnings . warn ( "isSet() == deprecated, use is_set() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  self . is_set ( );
        pub fn set ( self )  {
        "Set the internal flag to true.

        All threads waiting for it to become true are awakened. Threads
        that call wait() once the flag == true will !block at all.

        ";
        // with scope: self . _cond  {
        self . _flag = true;
        self . _cond . notify_all ( );
        pub fn clear ( self )  {
        "Reset the internal flag to false.

        Subsequently, threads calling wait() will block until set() == called to
        set the internal flag to true again.

        ";
        // with scope: self . _cond  {
        self . _flag = false;
        pub fn wait ( &self, timeout = None /* Option */ )  {
        "Block until the internal flag == true.

        If the internal flag == true on entry, return immediately. Otherwise,
        block until another thread calls set() to set the flag to true, || until
        the optional timeout occurs.

        When the timeout argument == present && !None /* Option */, it should be a
        floating point number specifying a timeout for the operation in seconds
        (or fractions thereof).

        This method returns the internal flag on exit, so it will always return
        true except if a timeout == given && the operation times out.

        ";
        // with scope: self . _cond  {
        signaled = self . _flag;
        if !signaled {
        signaled = self . _cond . wait ( timeout );
        return  signaled;
        class Barrier ;
        "Implements a Barrier.

    Useful for synchronizing a fixed number of threads at known synchronization
    points.  Threads block on 'wait()' && are simultaneously awoken once they
    have all made that call.

    ";
        pub fn __init__ ( &self, parties , action = None /* Option */ , timeout = None /* Option */ )  {
        "Create a barrier, initialised to 'parties' threads.

        'action' == a callable which, when supplied, will be called by one of
        the threads after they have all entered the barrier && just prior to
        releasing them all. If a 'timeout' == provided, it == used as the
        default for all subsequent 'wait()' calls.

        ";
        self . _cond = Condition ( Lock ( ) );
        self . _action = action;
        self . _timeout = timeout;
        self . _parties = parties;
        self . _state = 0;
        self . _count = 0;
        pub fn __repr__ ( self )  {
        cls = self . __class__;
        if self . broken {
        return  f "<{cls.__module__}.{cls.__qualname__} at {id(self):#x}: broken>";
        return  ( f "<{cls.__module__}.{cls.__qualname__} at {id(self):#x}:";
        format!(" waiters={self.n_waiting}/{self.parties}>" ));
        pub fn wait ( &self, timeout = None /* Option */ )  {
        "Wait for the barrier.

        When the specified number of threads have started waiting, they are all
        simultaneously awoken. If an 'action' was provided for the barrier, one
        of the threads will have executed that callback prior to returning.
        Returns an individual index number from 0 to 'parties-1'.

        ";
        if timeout is None /* Option */ {
        timeout = self . _timeout;
        // with scope: self . _cond  {
        self . _enter ( );
        index = self . _count;
        self . _count + = 1;
        // try {
        if index + 1 == self . _parties {
        self . _release ( );
        } else {
        self . _wait ( timeout );
        return  index;
        // } finally {
        self . _count - = 1;
        self . _exit ( );
        pub fn _enter ( self )  {
        while self . _state in ( -1 , 1 )  {
        self . _cond . wait ( );
        if self . _state < 0 {
        panic!("BrokenBarrierError");
        assert self . _state == 0;
        pub fn _release ( self )  {
        // try {
        if self . _action {
        self . _action ( );
        self . _state = 1;
        self . _cond . notify_all ( );
        // } catch   {
        self . _break ( );
        panic!("");
        pub fn _wait ( &self, timeout )  {
        if !self . _cond . wait_for ( lambda { : self . _state != 0 , timeout ) ; }
        self . _break ( );
        panic!("BrokenBarrierError");
        if self . _state < 0 {
        panic!("BrokenBarrierError");
        assert self . _state == 1;
        pub fn _exit ( self )  {
        if self . _count == 0 {
        if self . _state in ( -1 , 1 ) {
        self . _state = 0;
        self . _cond . notify_all ( );
        pub fn reset ( self )  {
        "Reset the barrier to the initial state.

        Any threads currently waiting will get the BrokenBarrier exception
        raised.

        ";
        // with scope: self . _cond  {
        if self . _count > 0 {
        if self . _state == 0 {
        self . _state = -1;
        } else if self . _state == -2 {
        self . _state = -1;
        } else {
        self . _state = 0;
        self . _cond . notify_all ( );
        pub fn abort ( self )  {
        "Place the barrier into a 'broken' state.

        Useful in case of error.  Any currently waiting threads && threads
        attempting to 'wait()' will have BrokenBarrierError raised.

        ";
        // with scope: self . _cond  {
        self . _break ( );
        pub fn _break ( self )  {
        self . _state = -2;
        self . _cond . notify_all ( );
        @ property;
        pub fn parties ( self )  {
        "Return the number of threads required to trip the barrier.";
        return  self . _parties;
        @ property;
        pub fn n_waiting ( self )  {
        "Return the number of threads currently waiting at the barrier.";
        if self . _state == 0 {
        return  self . _count;
        return  0;
        @ property;
        pub fn broken ( self )  {
        "Return true if the barrier == in a broken state.";
        return  self . _state == -2;
        class BrokenBarrierError ( RuntimeError ) ;
        // pass
        _counter = _count ( 1 ) . __next__;
        pub fn _newname ( name_template )  {
        return  name_template % _counter ( );
        _active_limbo_lock = RLock ( );
        _active = { };
        _limbo = { };
        _dangling = WeakSet ( );
        _shutdown_locks_lock = _allocate_lock ( );
        _shutdown_locks = set ( );
        pub fn _maintain_shutdown_locks ( )  {
        "
    Drop any shutdown locks that don't correspond to running threads anymore.

    Calling this from time to time avoids an ever-growing _shutdown_locks
    set when Thread objects are !joined explicitly. See bpo-37788.

    This must be called with _shutdown_locks_lock acquired.
    ";
        to_remove = vec![ lock.iter().map(|lock| _shutdown_locks if !lock . locked ( ) ).collect();
        _shutdown_locks . difference_update ( to_remove );
        class Thread ;
        "A class that represents a thread of control.

    This class can be safely subclassed in a limited fashion. There are two ways
    to specify the activity: by passing a callable object to the constructor, or
    by overriding the run() method in a subclass.

    ";
        _initialized = false;
        pub fn __init__ ( &self, group = None /* Option */ , target = None /* Option */ , name = None /* Option */ , {
        args = ( ) , kwargs = None /* Option */ , * , daemon = None /* Option */ ) ;
        "This constructor should always be called with keyword arguments. Arguments are:

        *group* should be None /* Option */; reserved for future extension when a ThreadGroup
        class == implemented.

        *target* == the callable object to be invoked by the run()
        method. Defaults to None /* Option */, meaning nothing == called.

        *name* == the thread name. By default, a unique name == constructed of
        the form "Thread-N" where N == a small decimal number.

        *args* == a list || tuple of arguments for the target invocation. Defaults to ().

        *kwargs* == a dictionary of keyword arguments for the target
        invocation. Defaults to {}.

        If a subclass overrides the constructor, it must make sure to invoke
        the base class constructor (Thread.__init__()) before doing anything
        else to the thread.

        ";
        assert group == None /* Option */ , "group argument must be None /* Option */ for now";
        if kwargs is None /* Option */ {
        kwargs = { };
        if name {
        name = str ( name );
        } else {
        name = _newname ( "Thread-%d" );
        if target is !None /* Option */ {
        // try {
        target_name = target . __name__;
        name + = format!(" ({target_name})");
        // } catch  AttributeError  {
        // pass
        self . _target = target;
        self . _name = name;
        self . _args = args;
        self . _kwargs = kwargs;
        if daemon is !None /* Option */ {
        self . _daemonic = daemon;
        } else {
        self . _daemonic = current_thread ( ) . daemon;
        self . _ident = None /* Option */;
        if _HAVE_THREAD_NATIVE_ID {
        self . _native_id = None /* Option */;
        self . _tstate_lock = None /* Option */;
        self . _started = Event ( );
        self . _is_stopped = false;
        self . _initialized = true;
        self . _stderr = _sys . stderr;
        self . _invoke_excepthook = _make_invoke_excepthook ( );
        _dangling . add ( self );
        pub fn _reset_internal_locks ( &self, is_alive )  {
        self . _started . _at_fork_reinit ( );
        if is_alive {
        if self . _tstate_lock is !None /* Option */ {
        self . _tstate_lock . _at_fork_reinit ( );
        self . _tstate_lock . acquire ( );
        } else {
        self . _is_stopped = true;
        self . _tstate_lock = None /* Option */;
        pub fn __repr__ ( self )  {
        assert self . _initialized , "Thread.__init__() was !called";
        status = "initial";
        if self . _started . is_set ( ) {
        status = "started";
        self . is_alive ( );
        if self . _is_stopped {
        status = "stopped";
        if self . _daemonic {
        status + = " daemon";
        if self . _ident is !None /* Option */ {
        status + = " %s" % self . _ident;
        return  "<%s(%s, %s)>" % ( self . __class__ . __name__ , self . _name , status );
        pub fn start ( self )  {
        "Start the thread's activity.

        It must be called at most once per thread object. It arranges for the
        object's run() method to be invoked in a separate thread of control.

        This method will raise a RuntimeError if called more than once on the
        same thread object.

        ";
        if !self . _initialized {
        panic!("RuntimeError ( "thread.__init__() !called" )");
        if self . _started . is_set ( ) {
        panic!("RuntimeError ( "threads can only be started once" )");
        // with scope: _active_limbo_lock  {
        _limbo [ self ] = self;
        // try {
        _start_new_thread ( self . _bootstrap , ( ) );
        // } catch  Exception  {
        // with scope: _active_limbo_lock  {
        del _limbo [ self ];
        panic!("");
        self . _started . wait ( );
        pub fn run ( self )  {
        "Method representing the thread's activity.

        You may override this method in a subclass. The standard run() method
        invokes the callable object passed to the object's constructor as the
        target argument, if any, with sequential && keyword arguments taken
        from the args && kwargs arguments, respectively.

        ";
        // try {
        if self . _target is !None /* Option */ {
        self . _target ( * self . _args , ** self . _kwargs );
        // } finally {
        del self . _target , self . _args , self . _kwargs;
        pub fn _bootstrap ( self )  {
        // try {
        self . _bootstrap_inner ( );
        // } catch   {
        if self . _daemonic && _sys is None /* Option */ {
        return;
        panic!("");
        pub fn _set_ident ( self )  {
        self . _ident = get_ident ( );
        if _HAVE_THREAD_NATIVE_ID {
        pub fn _set_native_id ( self )  {
        self . _native_id = get_native_id ( );
        pub fn _set_tstate_lock ( self )  {
        "
        Set a lock object which will be released by the interpreter when
        the underlying thread state (see pystate.h) gets deleted.
        ";
        self . _tstate_lock = _set_sentinel ( );
        self . _tstate_lock . acquire ( );
        if !self . daemon {
        // with scope: _shutdown_locks_lock  {
        _maintain_shutdown_locks ( );
        _shutdown_locks . add ( self . _tstate_lock );
        pub fn _bootstrap_inner ( self )  {
        // try {
        self . _set_ident ( );
        self . _set_tstate_lock ( );
        if _HAVE_THREAD_NATIVE_ID {
        self . _set_native_id ( );
        self . _started . set ( );
        // with scope: _active_limbo_lock  {
        _active [ self . _ident ] = self;
        del _limbo [ self ];
        if _trace_hook {
        _sys . settrace ( _trace_hook );
        if _profile_hook {
        _sys . setprofile ( _profile_hook );
        // try {
        self . run ( );
        // } catch   {
        self . _invoke_excepthook ( self );
        // } finally {
        self . _delete ( );
        pub fn _stop ( self )  {
        lock = self . _tstate_lock;
        if lock is !None /* Option */ {
        assert !lock . locked ( );
        self . _is_stopped = true;
        self . _tstate_lock = None /* Option */;
        if !self . daemon {
        // with scope: _shutdown_locks_lock  {
        _maintain_shutdown_locks ( );
        pub fn _delete ( self )  {
        "Remove current thread from the dict of currently running threads.";
        // with scope: _active_limbo_lock  {
        del _active [ get_ident ( ) ];
        pub fn join ( &self, timeout = None /* Option */ )  {
        "Wait until the thread terminates.

        This blocks the calling thread until the thread whose join() method is
        called terminates -- either normally || through an unhandled exception
        || until the optional timeout occurs.

        When the timeout argument == present && !None /* Option */, it should be a
        floating point number specifying a timeout for the operation in seconds
        (or fractions thereof). As join() always returns None /* Option */, you must call
        is_alive() after join() to decide whether a timeout happened -- if the
        thread == still alive, the join() call timed out.

        When the timeout argument == !present || None /* Option */, the operation will
        block until the thread terminates.

        A thread can be join()ed many times.

        join() raises a RuntimeError if an attempt == made to join the current
        thread as that would cause a deadlock. It == also an error to join() a
        thread before it has been started && attempts to do so raises the same
        exception.

        ";
        if !self . _initialized {
        panic!("RuntimeError ( "Thread.__init__() !called" )");
        if !self . _started . is_set ( ) {
        panic!("RuntimeError ( "cannot join thread before it is started" )");
        if self is current_thread ( ) {
        panic!("RuntimeError ( "cannot join current thread" )");
        if timeout is None /* Option */ {
        self . _wait_for_tstate_lock ( );
        } else {
        self . _wait_for_tstate_lock ( timeout = max ( timeout , 0 ) );
        pub fn _wait_for_tstate_lock ( &self, block = true , timeout = -1 )  {
        lock = self . _tstate_lock;
        if lock is None /* Option */ {
        assert self . _is_stopped;
        return;
        // try {
        if lock . acquire ( block , timeout ) {
        lock . release ( );
        self . _stop ( );
        // } catch   {
        if lock . locked ( ) {
        lock . release ( );
        self . _stop ( );
        panic!("");
        @ property;
        pub fn name ( self )  {
        "A string used for identification purposes only.

        It has no semantics. Multiple threads may be given the same name. The
        initial name == set by the constructor.

        ";
        assert self . _initialized , "Thread.__init__() !called";
        return  self . _name;
        @ name . setter;
        pub fn name ( &self, name )  {
        assert self . _initialized , "Thread.__init__() !called";
        self . _name = str ( name );
        @ property;
        pub fn ident ( self )  {
        "Thread identifier of this thread || None /* Option */ if it has !been started.

        This == a nonzero integer. See the get_ident() function. Thread
        identifiers may be recycled when a thread exits && another thread is
        created. The identifier == available even after the thread has exited.

        ";
        assert self . _initialized , "Thread.__init__() !called";
        return  self . _ident;
        if _HAVE_THREAD_NATIVE_ID {
        @ property;
        pub fn native_id ( self )  {
        "Native integral thread ID of this thread, || None /* Option */ if it has !been started.

            This == a non-negative integer. See the get_native_id() function.
            This represents the Thread ID as reported by the kernel.

            ";
        assert self . _initialized , "Thread.__init__() !called";
        return  self . _native_id;
        pub fn is_alive ( self )  {
        "Return whether the thread == alive.

        This method returns true just before the run() method starts until just
        after the run() method terminates. See also the module function
        enumerate().

        ";
        assert self . _initialized , "Thread.__init__() !called";
        if self . _is_stopped || !self . _started . is_set ( ) {
        return  false;
        self . _wait_for_tstate_lock ( false );
        return  !self . _is_stopped;
        @ property;
        pub fn daemon ( self )  {
        "A boolean value indicating whether this thread == a daemon thread.

        This must be set before start() == called, otherwise RuntimeError is
        raised. Its initial value == inherited from the creating thread; the
        main thread == !a daemon thread && therefore all threads created in
        the main thread default to daemon = false.

        The entire Python program exits when only daemon threads are left.

        ";
        assert self . _initialized , "Thread.__init__() !called";
        return  self . _daemonic;
        @ daemon . setter;
        pub fn daemon ( &self, daemonic )  {
        if !self . _initialized {
        panic!("RuntimeError ( "Thread.__init__() !called" )");
        if self . _started . is_set ( ) {
        panic!("RuntimeError ( "cannot set daemon status of active thread" )");
        self . _daemonic = daemonic;
        pub fn isDaemon ( self )  {
        "Return whether this thread == a daemon.

        This method == deprecated, use the daemon attribute instead.

        ";
        import warnings;
        warnings . warn ( "isDaemon() == deprecated, get the daemon attribute instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  self . daemon;
        pub fn setDaemon ( &self, daemonic )  {
        "Set whether this thread == a daemon.

        This method == deprecated, use the .daemon property instead.

        ";
        import warnings;
        warnings . warn ( "setDaemon() == deprecated, set the daemon attribute instead" ,;
        DeprecationWarning , stacklevel = 2 );
        self . daemon = daemonic;
        pub fn getName ( self )  {
        "Return a string used for identification purposes only.

        This method == deprecated, use the name attribute instead.

        ";
        import warnings;
        warnings . warn ( "getName() == deprecated, get the name attribute instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  self . name;
        pub fn setName ( &self, name )  {
        "Set the name string for this thread.

        This method == deprecated, use the name attribute instead.

        ";
        import warnings;
        warnings . warn ( "setName() == deprecated, set the name attribute instead" ,;
        DeprecationWarning , stacklevel = 2 );
        self . name = name;
        // try {
        from _thread import ( _excepthook as excepthook ,;
        _ExceptHookArgs as ExceptHookArgs );
        // } catch  ImportError  {
        from traceback import print_exception as _print_exception;
        from collections import namedtuple;
        _ExceptHookArgs = namedtuple (;
        "ExceptHookArgs" ,;
        "exc_type exc_value exc_traceback thread" );
        pub fn ExceptHookArgs ( args )  {
        return  _ExceptHookArgs ( * args );
        pub fn excepthook ( args , / )  {
        "
        Handle uncaught Thread.run() exception.
        ";
        if args . exc_type == SystemExit {
        return;
        if _sys is !None /* Option */ && _sys . stderr is !None /* Option */ {
        stderr = _sys . stderr;
        } else if args . thread is !None /* Option */ {
        stderr = args . thread . _stderr;
        if stderr is None /* Option */ {
        return;
        } else {
        return;
        if args . thread is !None /* Option */ {
        name = args . thread . name;
        } else {
        name = get_ident ( );
        println!( f "Exception in thread {name}:" );
        file = stderr , flush = true );
        _print_exception ( args . exc_type , args . exc_value , args . exc_traceback ,;
        file = stderr );
        stderr . flush ( );
        __excepthook__ = excepthook;
        pub fn _make_invoke_excepthook ( )  {
        old_excepthook = excepthook;
        old_sys_excepthook = _sys . excepthook;
        if old_excepthook is None /* Option */ {
        panic!("RuntimeError ( "threading.excepthook is None /* Option */" )");
        if old_sys_excepthook is None /* Option */ {
        panic!("RuntimeError ( "sys.excepthook is None /* Option */" )");
        sys_exc_info = _sys . exc_info;
        local_print = print;
        local_sys = _sys;
        pub fn invoke_excepthook ( thread )  {
        global excepthook;
        // try {
        hook = excepthook;
        if hook is None /* Option */ {
        hook = old_excepthook;
        args = ExceptHookArgs ( [ * sys_exc_info ( ) , thread ] );
        hook ( args );
        // } catch  Exception as exc  {
        exc . __suppress_context__ = true;
        del exc;
        if local_sys is !None /* Option */ && local_sys . stderr is !None /* Option */ {
        stderr = local_sys . stderr;
        } else {
        stderr = thread . _stderr;
        local_print ( "Exception in threading.excepthook:" ,;
        file = stderr , flush = true );
        if local_sys is !None /* Option */ && local_sys . excepthook is !None /* Option */ {
        sys_excepthook = local_sys . excepthook;
        } else {
        sys_excepthook = old_sys_excepthook;
        sys_excepthook ( * sys_exc_info ( ) );
        // } finally {
        args = None /* Option */;
        return  invoke_excepthook;
        class Timer ( Thread ) ;
        "Call a function after a specified number of seconds:

            t = Timer(30.0, f, args=None /* Option */, kwargs=None /* Option */)
            t.start()
            t.cancel()     # stop the timer's action if it's still waiting

    ";
        pub fn __init__ ( &self, interval , function , args = None /* Option */ , kwargs = None /* Option */ )  {
        Thread . __init__ ( self );
        self . interval = interval;
        self . function = function;
        self . args = args if args is !None /* Option */ else [ ];
        self . kwargs = kwargs if kwargs is !None /* Option */ else { };
        self . finished = Event ( );
        pub fn cancel ( self )  {
        "Stop the timer if it hasn't finished yet.";
        self . finished . set ( );
        pub fn run ( self )  {
        self . finished . wait ( self . interval );
        if !self . finished . is_set ( ) {
        self . function ( * self . args , ** self . kwargs );
        self . finished . set ( );
        class _MainThread ( Thread ) ;
        pub fn __init__ ( self )  {
        Thread . __init__ ( self , name = "MainThread" , daemon = false );
        self . _set_tstate_lock ( );
        self . _started . set ( );
        self . _set_ident ( );
        if _HAVE_THREAD_NATIVE_ID {
        self . _set_native_id ( );
        // with scope: _active_limbo_lock  {
        _active [ self . _ident ] = self;
        class _DummyThread ( Thread ) ;
        pub fn __init__ ( self )  {
        Thread . __init__ ( self , name = _newname ( "Dummy-%d" ) , daemon = true );
        self . _started . set ( );
        self . _set_ident ( );
        if _HAVE_THREAD_NATIVE_ID {
        self . _set_native_id ( );
        // with scope: _active_limbo_lock  {
        _active [ self . _ident ] = self;
        pub fn _stop ( self )  {
        // pass
        pub fn is_alive ( self )  {
        assert !self . _is_stopped && self . _started . is_set ( );
        return  true;
        pub fn join ( &self, timeout = None /* Option */ )  {
        assert false , "cannot join a dummy thread";
        pub fn current_thread ( )  {
        "Return the current Thread object, corresponding to the caller's thread of control.

    If the caller's thread of control was !created through the threading
    module, a dummy thread object with limited functionality == returned.

    ";
        // try {
        return  _active [ get_ident ( ) ];
        // } catch  KeyError  {
        return  _DummyThread ( );
        pub fn currentThread ( )  {
        "Return the current Thread object, corresponding to the caller's thread of control.

    This function == deprecated, use current_thread() instead.

    ";
        import warnings;
        warnings . warn ( "currentThread() == deprecated, use current_thread() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  current_thread ( );
        pub fn active_count ( )  {
        "Return the number of Thread objects currently alive.

    The returned count == equal to the length of the list returned by
    enumerate().

    ";
        // with scope: _active_limbo_lock  {
        return  len ( _active ) + len ( _limbo );
        pub fn activeCount ( )  {
        "Return the number of Thread objects currently alive.

    This function == deprecated, use active_count() instead.

    ";
        import warnings;
        warnings . warn ( "activeCount() == deprecated, use active_count() instead" ,;
        DeprecationWarning , stacklevel = 2 );
        return  active_count ( );
        pub fn _enumerate ( )  {
        return  list ( _active . values ( ) ) + list ( _limbo . values ( ) );
        pub fn enumerate ( )  {
        "Return a list of all Thread objects currently alive.

    The list includes daemonic threads, dummy thread objects created by
    current_thread(), && the main thread. It excludes terminated threads and
    threads that have !yet been started.

    ";
        // with scope: _active_limbo_lock  {
        return  list ( _active . values ( ) ) + list ( _limbo . values ( ) );
        _threading_atexits = [ ];
        _SHUTTING_DOWN = false;
        pub fn _register_atexit ( func , * arg , ** kwargs )  {
        "CPython internal: register *func* to be called before joining threads.

    The registered *func* == called with its arguments just before all
    non-daemon threads are joined in `_shutdown()`. It provides a similar
    purpose to `atexit.register()`, but its functions are called prior to
    threading shutdown instead of interpreter shutdown.

    For similarity to atexit, the registered functions are called in reverse.
    ";
        if _SHUTTING_DOWN {
        panic!("RuntimeError ( "can't register atexit after shutdown" )");
        call = functools . partial ( func , * arg , ** kwargs );
        _threading_atexits . append ( call );
        from _thread import stack_size;
        _main_thread = _MainThread ( );
        pub fn _shutdown ( )  {
        "
    Wait until the Python thread state of all non-daemon threads get deleted.
    ";
        if _main_thread . _is_stopped {
        return;
        global _SHUTTING_DOWN;
        _SHUTTING_DOWN = true;
        for atexit_call in reversed ( _threading_atexits ) .iter() {
        atexit_call ( );
        if _main_thread . ident == get_ident ( ) {
        tlock = _main_thread . _tstate_lock;
        assert tlock == !None /* Option */;
        assert tlock . locked ( );
        tlock . release ( );
        _main_thread . _stop ( );
        } else {
        // pass
        while true  {
        // with scope: _shutdown_locks_lock  {
        locks = list ( _shutdown_locks );
        _shutdown_locks . clear ( );
        if !locks {
        break;
        for lock in locks .iter() {
        lock . acquire ( );
        lock . release ( );
        pub fn main_thread ( )  {
        "Return the main thread object.

    In normal conditions, the main thread == the thread from which the
    Python interpreter was started.
    ";
        return  _main_thread;
        // try {
        from _thread import _local as local;
        // } catch  ImportError  {
        from _threading_local import local;
        pub fn _after_fork ( )  {
        "
    Cleanup threading module state that should !exist after a fork.
    ";
        global _active_limbo_lock , _main_thread;
        global _shutdown_locks_lock , _shutdown_locks;
        _active_limbo_lock = RLock ( );
        new_active = { };
        // try {
        current = _active [ get_ident ( ) ];
        // } catch  KeyError  {
        current = _MainThread ( );
        _main_thread = current;
        _shutdown_locks_lock = _allocate_lock ( );
        _shutdown_locks = set ( );
        // with scope: _active_limbo_lock  {
        threads = set ( _enumerate ( ) );
        threads . update ( _dangling );
        for thread in threads .iter() {
        if thread is current {
        thread . _reset_internal_locks ( true );
        ident = get_ident ( );
        if isinstance ( thread , _DummyThread ) {
        thread . __class__ = _MainThread;
        thread . _name = "MainThread";
        thread . _daemonic = false;
        thread . _set_tstate_lock ( );
        thread . _ident = ident;
        new_active [ ident ] = thread;
        } else {
        thread . _reset_internal_locks ( false );
        thread . _stop ( );
        _limbo . clear ( );
        _active . clear ( );
        _active . update ( new_active );
        assert len ( _active ) == 1;
        if hasattr ( _os , "register_at_fork" ) {
        _os . register_at_fork ( after_in_child = _after_fork );
}

