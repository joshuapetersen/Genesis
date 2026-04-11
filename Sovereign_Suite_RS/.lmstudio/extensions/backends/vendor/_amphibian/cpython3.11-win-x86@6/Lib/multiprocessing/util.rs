//! util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::atexit;
// use crate::subprocess::{_args_from_interpreter_flags};
// use crate::.::{process};
// use crate::logging;
// use crate::shutil;
// use crate::traceback;
// use crate::_posixsubprocess;
// use crate::test::{support};
// use crate::multiprocessing::{forkserver};

pub const __all__: f64 = [;
pub const NOTSET: u64 = 0;
pub const SUBDEBUG: u64 = 5;
pub const DEBUG: u64 = 10;
pub const INFO: u64 = 20;
pub const SUBWARNING: u64 = 25;
pub const LOGGER_NAME: &str = "multiprocessing";
pub const DEFAULT_LOGGING_FORMAT: &str = "[%(levelname)s/%(processName)s] %(message)s";
pub const _logger: f64 = None;
pub const _log_to_stderr: f64 = False;
pub fn sub_debug(msg: &str, args: &str) {
        if _logger {
        _logger . log ( SUBDEBUG , msg , * args , stacklevel = 2 );
        pub fn debug ( msg , * args )  {
        if _logger {
        _logger . log ( DEBUG , msg , * args , stacklevel = 2 );
        pub fn info ( msg , * args )  {
        if _logger {
        _logger . log ( INFO , msg , * args , stacklevel = 2 );
        pub fn sub_warning ( msg , * args )  {
        if _logger {
        _logger . log ( SUBWARNING , msg , * args , stacklevel = 2 );
        pub fn get_logger ( )  {
        "
    Returns logger used by multiprocessing
    ";
        global _logger;
        import logging;
        logging . _acquireLock ( );
        // try {
        if !_logger {
        _logger = logging . getLogger ( LOGGER_NAME );
        _logger . propagate = 0;
        if hasattr ( atexit , "unregister" ) {
        atexit . unregister ( _exit_function );
        atexit . register ( _exit_function );
        } else {
        atexit . _exithandlers . remove ( ( _exit_function , ( ) , { } ) );
        atexit . _exithandlers . append ( ( _exit_function , ( ) , { } ) );
        // } finally {
        logging . _releaseLock ( );
        return  _logger;
        pub fn log_to_stderr ( level = None /* Option */ )  {
        "
    Turn on logging && add a handler which prints to stderr
    ";
        global _log_to_stderr;
        import logging;
        logger = get_logger ( );
        formatter = logging . Formatter ( DEFAULT_LOGGING_FORMAT );
        handler = logging . StreamHandler ( );
        handler . setFormatter ( formatter );
        logger . addHandler ( handler );
        if level {
        logger . setLevel ( level );
        _log_to_stderr = true;
        return  _logger;
        pub fn _platform_supports_abstract_sockets ( )  {
        if sys . platform == "linux" {
        return  true;
        if hasattr ( sys , "getandroidapilevel" ) {
        return  true;
        return  false;
        pub fn is_abstract_socket_namespace ( address )  {
        if !address {
        return  false;
        if isinstance ( address , bytes ) {
        return  address [ 0 ] == 0;
        } else if isinstance ( address , str ) {
        return  address [ 0 ] == "\0";
        panic!("TypeError ( f "address type of {address!r} unrecognized" )");
        abstract_sockets_supported = _platform_supports_abstract_sockets ( );
        pub fn _remove_temp_dir ( rmtree , tempdir )  {
        pub fn onerror ( func , path , err_info )  {
        if !issubclass ( err_info [ 0 ] , FileNotFoundError ) {
        panic!("");
        rmtree ( tempdir , onerror = onerror );
        current_process = process . current_process ( );
        if current_process is !None /* Option */ {
        current_process . _config [ "tempdir" ] = None /* Option */;
        pub fn get_temp_dir ( )  {
        tempdir = process . current_process ( ) . _config . get ( "tempdir" );
        if tempdir is None /* Option */ {
        import shutil , tempfile;
        tempdir = tempfile . mkdtemp ( prefix = "pymp-" );
        info ( "created temp directory %s" , tempdir );
        Finalize ( None /* Option */ , _remove_temp_dir , args = ( shutil . rmtree , tempdir ) ,;
        exitpriority = -100 );
        process . current_process ( ) . _config [ "tempdir" ] = tempdir;
        return  tempdir;
        _afterfork_registry = weakref . WeakValueDictionary ( );
        _afterfork_counter = itertools . count ( );
        pub fn _run_after_forkers ( )  {
        items = list ( _afterfork_registry . items ( ) );
        items . sort ( );
        for ( index , ident , func ) , obj in items .iter() {
        // try {
        func ( obj );
        // } catch  Exception as e  {
        info ( "after forker raised exception %s" , e );
        pub fn register_after_fork ( obj , func )  {
        _afterfork_registry [ ( next ( _afterfork_counter ) , id ( obj ) , func ) ] = obj;
        _finalizer_registry = { };
        _finalizer_counter = itertools . count ( );
        class Finalize ( object ) ;
        "
    Class which supports object finalization using weakrefs
    ";
        pub fn __init__ ( &self, obj , callback , args = ( ) , kwargs = None /* Option */ , exitpriority = None /* Option */ )  {
        if ( exitpriority is !None /* Option */ ) && !isinstance ( exitpriority , int ) {
        panic!("TypeError (");
        "Exitpriority ({0!r}) must be None /* Option */ || int, !{1!s}" . format (;
        exitpriority , type ( exitpriority ) ) );
        if obj is !None /* Option */ {
        self . _weakref = weakref . ref ( obj , self );
        } else if exitpriority is None /* Option */ {
        panic!("ValueError ( "Without object, exitpriority cannot be None /* Option */" )");
        self . _callback = callback;
        self . _args = args;
        self . _kwargs = kwargs || { };
        self . _key = ( exitpriority , next ( _finalizer_counter ) );
        self . _pid = os . getpid ( );
        _finalizer_registry [ self . _key ] = self;
        pub fn __call__ ( &self, wr = None /* Option */ , {
        _finalizer_registry = _finalizer_registry ,;
        sub_debug = sub_debug , getpid = os . getpid ) ;
        "
        Run the callback unless it has already been called || cancelled
        ";
        // try {
        del _finalizer_registry [ self . _key ];
        // } catch  KeyError  {
        sub_debug ( "finalizer no longer registered" );
        } else {
        if self . _pid != getpid ( ) {
        sub_debug ( "finalizer ignored because different process" );
        res = None /* Option */;
        } else {
        sub_debug ( "finalizer calling %s with args %s && kwargs %s" ,;
        self . _callback , self . _args , self . _kwargs );
        res = self . _callback ( * self . _args , ** self . _kwargs );
        self . _weakref = self . _callback = self . _args = \;
        self . _kwargs = self . _key = None /* Option */;
        return  res;
        pub fn cancel ( self )  {
        "
        Cancel finalization of the object
        ";
        // try {
        del _finalizer_registry [ self . _key ];
        // } catch  KeyError  {
        // pass
        } else {
        self . _weakref = self . _callback = self . _args = \;
        self . _kwargs = self . _key = None /* Option */;
        pub fn still_active ( self )  {
        "
        Return whether this finalizer == still waiting to invoke callback
        ";
        return  self . _key in _finalizer_registry;
        pub fn __repr__ ( self )  {
        // try {
        obj = self . _weakref ( );
        // } catch  ( AttributeError , TypeError )  {
        obj = None /* Option */;
        if obj is None /* Option */ {
        return  "<%s object, dead>" % self . __class__ . __name__;
        x = "<%s object, callback=%s" % (;
        self . __class__ . __name__ ,;
        getattr ( self . _callback , "__name__" , self . _callback ) );
        if self . _args {
        x + = ", args=" + str ( self . _args );
        if self . _kwargs {
        x + = ", kwargs=" + str ( self . _kwargs );
        if self . _key [ 0 ] is !None /* Option */ {
        x + = ", exitpriority=" + str ( self . _key [ 0 ] );
        return  x + ">";
        pub fn _run_finalizers ( minpriority = None /* Option */ )  {
        "
    Run all finalizers whose exit priority == !None /* Option */ && at least minpriority

    Finalizers with highest priority are called first; finalizers with
    the same priority will be called in reverse order of creation.
    ";
        if _finalizer_registry is None /* Option */ {
        return;
        if minpriority is None /* Option */ {
        f = |p | {  p [ 0 ] == !None /* Option */ };
        } else {
        f = |p | {  p [ 0 ] == !None /* Option */ && p [ 0 ] >= minpriority };
        keys = vec![ key.iter().map(|key| list ( _finalizer_registry ) if f ( key ) ).collect();
        keys . sort ( reverse = true );
        for key in keys .iter() {
        finalizer = _finalizer_registry . get ( key );
        if finalizer is !None /* Option */ {
        sub_debug ( "calling %s" , finalizer );
        // try {
        finalizer ( );
        // } catch  Exception  {
        import traceback;
        traceback . print_exc ( );
        if minpriority is None /* Option */ {
        _finalizer_registry . clear ( );
        pub fn is_exiting ( )  {
        "
    Returns true if the process == shutting down
    ";
        return  _exiting || _exiting is None /* Option */;
        _exiting = false;
        pub fn _exit_function ( info = info , debug = debug , _run_finalizers = _run_finalizers , {
        active_children = process . active_children ,;
        current_process = process . current_process ) ;
        global _exiting;
        if !_exiting {
        _exiting = true;
        info ( "process shutting down" );
        debug ( "running all "atexit" finalizers with priority >= 0" );
        _run_finalizers ( 0 );
        if current_process ( ) is !None /* Option */ {
        for p in active_children ( ) .iter() {
        if p . daemon {
        info ( "calling terminate() for daemon %s" , p . name );
        p . _popen . terminate ( );
        for p in active_children ( ) .iter() {
        info ( "calling join() for process %s" , p . name );
        p . join ( );
        debug ( "running the remaining "atexit" finalizers" );
        _run_finalizers ( );
        atexit . register ( _exit_function );
        class ForkAwareThreadLock ( object ) ;
        pub fn __init__ ( self )  {
        self . _lock = threading . Lock ( );
        self . acquire = self . _lock . acquire;
        self . release = self . _lock . release;
        register_after_fork ( self , ForkAwareThreadLock . _at_fork_reinit );
        pub fn _at_fork_reinit ( self )  {
        self . _lock . _at_fork_reinit ( );
        pub fn __enter__ ( self )  {
        return  self . _lock . __enter__ ( );
        pub fn __exit__ ( &self, * args )  {
        return  self . _lock . __exit__ ( * args );
        class ForkAwareLocal ( threading . local ) ;
        pub fn __init__ ( self )  {
        register_after_fork ( self , |obj | {  obj . __dict__ . clear ( ) ) };
        pub fn __reduce__ ( self )  {
        return  type ( self ) , ( );
        // try {
        MAXFD = os . sysconf ( "SC_OPEN_MAX" );
        // } catch  Exception  {
        MAXFD = 256;
        pub fn close_all_fds_except ( fds )  {
        fds = list ( fds ) + [ -1 , MAXFD ];
        fds . sort ( );
        assert fds [ -1 ] == MAXFD , "fd too large";
        for i in range ( len ( fds ) - 1 ) .iter() {
        os . closerange ( fds [ i ] + 1 , fds [ i + 1 ] );
        pub fn _close_stdin ( )  {
        if sys . stdin is None /* Option */ {
        return;
        // try {
        sys . stdin . close ( );
        // } catch  ( OSError , ValueError )  {
        // pass
        // try {
        fd = os . open ( os . devnull , os . O_RDONLY );
        // try {
        sys . stdin = open ( fd , encoding = "utf-8" , closefd = false );
        // } catch   {
        os . close ( fd );
        panic!("");
        // } catch  ( OSError , ValueError )  {
        // pass
        pub fn _flush_std_streams ( )  {
        // try {
        sys . stdout . flush ( );
        // } catch  ( AttributeError , ValueError )  {
        // pass
        // try {
        sys . stderr . flush ( );
        // } catch  ( AttributeError , ValueError )  {
        // pass
        pub fn spawnv_passfds ( path , args , passfds )  {
        import _posixsubprocess;
        import subprocess;
        passfds = tuple ( sorted ( map ( int , passfds ) ) );
        errpipe_read , errpipe_write = os . pipe ( );
        // try {
        return  _posixsubprocess . fork_exec (;
        args , [ path ] , true , passfds , None /* Option */ , None /* Option */ ,;
        -1 , -1 , -1 , -1 , -1 , -1 , errpipe_read , errpipe_write ,;
        false , false , -1 , None /* Option */ , None /* Option */ , None /* Option */ , -1 , None /* Option */ ,;
        subprocess . _USE_VFORK );
        // } finally {
        os . close ( errpipe_read );
        os . close ( errpipe_write );
        pub fn close_fds ( * fds )  {
        "Close each file descriptor given as an argument";
        for fd in fds .iter() {
        os . close ( fd );
        pub fn _cleanup_tests ( )  {
        "Cleanup multiprocessing resources when multiprocessing tests
    completed.";
        from test import support;
        process . _cleanup ( );
        from multiprocessing import forkserver;
        forkserver . _forkserver . _stop ( );
        from multiprocessing import resource_tracker;
        resource_tracker . _resource_tracker . _stop ( );
        _run_finalizers ( );
        support . gc_collect ( );
        support . reap_children ( );
}

