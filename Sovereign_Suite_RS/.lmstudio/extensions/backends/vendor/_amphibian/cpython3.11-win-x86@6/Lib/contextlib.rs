//! contextlib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc;
// use std::env;
// use std::collections::{deque};
// use crate::functools::{wraps};
// use crate::types::{MethodType, GenericAlias};

pub const __all__: &str = ["asynccontextmanager" ,"contextmanager" ,"closing" ,"nullcontext" ,;
pub struct AbstractContextManager {
    pub gen: String, // TODO: infer type
    pub kwds: String, // TODO: infer type
    pub __doc__: String, // TODO: infer type
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl AbstractContextManager {
}

pub struct AbstractAsyncContextManager {
    pub gen: String, // TODO: infer type
    pub kwds: String, // TODO: infer type
    pub __doc__: String, // TODO: infer type
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl AbstractAsyncContextManager {
}

pub struct ContextDecorator {
    pub gen: String, // TODO: infer type
    pub kwds: String, // TODO: infer type
    pub __doc__: String, // TODO: infer type
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl ContextDecorator {
}

pub struct AsyncContextDecorator {
    pub gen: String, // TODO: infer type
    pub kwds: String, // TODO: infer type
    pub __doc__: String, // TODO: infer type
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl AsyncContextDecorator {
}

pub struct _GeneratorContextManagerBase {
    pub gen: String, // TODO: infer type
    pub kwds: String, // TODO: infer type
    pub __doc__: String, // TODO: infer type
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl _GeneratorContextManagerBase {
}

pub struct _GeneratorContextManager {
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl _GeneratorContextManager {
}

pub struct _AsyncGeneratorContextManager {
    pub thing: String, // TODO: infer type
    pub _new_target: String, // TODO: infer type
    pub _old_targets: String, // TODO: infer type
    pub _exceptions: String, // TODO: infer type
    pub _exit_callbacks: String, // TODO: infer type
    pub enter_result: String, // TODO: infer type
    pub path: String, // TODO: infer type
    pub _old_cwd: String, // TODO: infer type
}

impl _AsyncGeneratorContextManager {
}

pub fn contextmanager(func: &str) {
        "@contextmanager decorator.

    Typical usage:

        @contextmanager
        def some_generator(<arguments>):
            <setup>
            try:
                yield <value>
            finally:
                <cleanup>

    This makes this:

        with some_generator(<arguments>) as <variable>:
            <body>

    equivalent to this:

        <setup>
        try:
            <variable> = <value>
            <body>
        finally:
            <cleanup>
    ";
        @ wraps ( func );
        pub fn helper ( * args , ** kwds )  {
        return  _GeneratorContextManager ( func , args , kwds );
        return  helper;
        pub fn asynccontextmanager ( func )  {
        "@asynccontextmanager decorator.

    Typical usage:

        @asynccontextmanager
        async def some_async_generator(<arguments>):
            <setup>
            try:
                yield <value>
            finally:
                <cleanup>

    This makes this:

        async with some_async_generator(<arguments>) as <variable>:
            <body>

    equivalent to this:

        <setup>
        try:
            <variable> = <value>
            <body>
        finally:
            <cleanup>
    ";
        @ wraps ( func );
        pub fn helper ( * args , ** kwds )  {
        return  _AsyncGeneratorContextManager ( func , args , kwds );
        return  helper;
        class closing ( AbstractContextManager ) ;
        "Context to automatically close something at the end of a block.

    Code like this:

        with closing(<module>.open(<arguments>)) as f:
            <block>

    == equivalent to this:

        f = <module>.open(<arguments>)
        try:
            <block>
        finally:
            f.close()

    ";
        pub fn __init__ ( &self, thing )  {
        self . thing = thing;
        pub fn __enter__ ( self )  {
        return  self . thing;
        pub fn __exit__ ( &self, * exc_info )  {
        self . thing . close ( );
        class aclosing ( AbstractAsyncContextManager ) ;
        "Async context manager for safely finalizing an asynchronously cleaned-up
    resource such as an async generator, calling its ``aclose()`` method.

    Code like this:

        async with aclosing(<module>.fetch(<arguments>)) as agen:
            <block>

    == equivalent to this:

        agen = <module>.fetch(<arguments>)
        try:
            <block>
        finally:
            await agen.aclose()

    ";
        pub fn __init__ ( &self, thing )  {
        self . thing = thing;
        async def __aenter__ ( self ) ;
        return  self . thing;
        async def __aexit__ ( self , * exc_info ) ;
        await self . thing . aclose ( );
        class _RedirectStream ( AbstractContextManager ) ;
        _stream = None /* Option */;
        pub fn __init__ ( &self, new_target )  {
        self . _new_target = new_target;
        self . _old_targets = [ ];
        pub fn __enter__ ( self )  {
        self . _old_targets . append ( getattr ( sys , self . _stream ) );
        setattr ( sys , self . _stream , self . _new_target );
        return  self . _new_target;
        pub fn __exit__ ( &self, exctype , excinst , exctb )  {
        setattr ( sys , self . _stream , self . _old_targets . pop ( ) );
        class redirect_stdout ( _RedirectStream ) ;
        "Context manager for temporarily redirecting stdout to another file.

        # How to send help() to stderr
        with redirect_stdout(sys.stderr):
            help(dir)

        # How to write help() to a file
        with open('help.txt', 'w') as f:
            with redirect_stdout(f):
                help(pow)
    ";
        _stream = "stdout";
        class redirect_stderr ( _RedirectStream ) ;
        "Context manager for temporarily redirecting stderr to another file.";
        _stream = "stderr";
        class suppress ( AbstractContextManager ) ;
        "Context manager to suppress specified exceptions

    After the exception == suppressed, execution proceeds with the next
    statement following the with statement.

         with suppress(FileNotFoundError):
             os.remove(somefile)
         # Execution still resumes here if the file was already removed
    ";
        pub fn __init__ ( &self, * exceptions )  {
        self . _exceptions = exceptions;
        pub fn __enter__ ( self )  {
        // pass
        pub fn __exit__ ( &self, exctype , excinst , exctb )  {
        return  exctype is !None /* Option */ && issubclass ( exctype , self . _exceptions );
        class _BaseExitStack ;
        "A base class for ExitStack && AsyncExitStack.";
        @ staticmethod;
        pub fn _create_exit_wrapper ( cm , cm_exit )  {
        return  MethodType ( cm_exit , cm );
        @ staticmethod;
        pub fn _create_cb_wrapper ( callback , / , * args , ** kwds )  {
        pub fn _exit_wrapper ( exc_type , exc , tb )  {
        callback ( * args , ** kwds );
        return  _exit_wrapper;
        pub fn __init__ ( self )  {
        self . _exit_callbacks = deque ( );
        pub fn pop_all ( self )  {
        "Preserve the context stack by transferring it to a new instance.";
        new_stack = type ( self ) ( );
        new_stack . _exit_callbacks = self . _exit_callbacks;
        self . _exit_callbacks = deque ( );
        return  new_stack;
        pub fn push ( &self, exit )  {
        "Registers a callback with the standard __exit__ method signature.

        Can suppress exceptions the same way __exit__ method can.
        Also accepts any object with an __exit__ method (registering a call
        to the method instead of the object itself).
        ";
        _cb_type = type ( exit );
        // try {
        exit_method = _cb_type . __exit__;
        // } catch  AttributeError  {
        self . _push_exit_callback ( exit );
        } else {
        self . _push_cm_exit ( exit , exit_method );
        return  exit;
        pub fn enter_context ( &self, cm )  {
        "Enters the supplied context manager.

        If successful, also pushes its __exit__ method as a callback and
        returns the result of the __enter__ method.
        ";
        cls = type ( cm );
        // try {
        _enter = cls . __enter__;
        _exit = cls . __exit__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "'{cls.__module__}.{cls.__qualname__}' object does "");
        format!("not support the context manager protocol" ) from None /* Option */);
        result = _enter ( cm );
        self . _push_cm_exit ( cm , _exit );
        return  result;
        pub fn callback ( &self, callback , / , * args , ** kwds )  {
        "Registers an arbitrary callback && arguments.

        Cannot suppress exceptions.
        ";
        _exit_wrapper = self . _create_cb_wrapper ( callback , * args , ** kwds );
        _exit_wrapper . __wrapped__ = callback;
        self . _push_exit_callback ( _exit_wrapper );
        return  callback;
        pub fn _push_cm_exit ( &self, cm , cm_exit )  {
        "Helper to correctly register callbacks to __exit__ methods.";
        _exit_wrapper = self . _create_exit_wrapper ( cm , cm_exit );
        self . _push_exit_callback ( _exit_wrapper , true );
        pub fn _push_exit_callback ( &self, callback , is_sync = true )  {
        self . _exit_callbacks . append ( ( is_sync , callback ) );
        class ExitStack ( _BaseExitStack , AbstractContextManager ) ;
        "Context manager.iter().map(|dynamic management of a stack of exit callbacks.

    For example:
        with ExitStack() as stack:
            files = vec![stack.enter_context(open(fname)).iter().map(|fname| filenames]
            # All opened files will automatically be closed at the end of
            # the with statement, even if attempts to open files later
            #| the list raise an exception.
    ";
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * exc_details )  {
        received_exc = exc_details [ 0 ] == !None /* Option */;
        frame_exc = sys . exc_info ( ) [ 1 ];
        pub fn _fix_exception_context ( new_exc , old_exc )  {
        while 1  {
        exc_context = new_exc . __context__;
        if exc_context is None /* Option */ || exc_context is old_exc {
        return;
        if exc_context is frame_exc {
        break;
        new_exc = exc_context;
        new_exc . __context__ = old_exc;
        suppressed_exc = false;
        pending_raise = false;
        while self . _exit_callbacks  {
        is_sync , cb = self . _exit_callbacks . pop ( );
        assert is_sync;
        // try {
        if cb ( * exc_details ) {
        suppressed_exc = true;
        pending_raise = false;
        exc_details = ( None /* Option */ , None /* Option */ , None /* Option */ );
        // } catch   {
        new_exc_details = sys . exc_info ( );
        _fix_exception_context ( new_exc_details [ 1 ] , exc_details [ 1 ] );
        pending_raise = true;
        exc_details = new_exc_details;
        if pending_raise {
        // try {
        fixed_ctx = exc_details [ 1 ] . __context__;
        panic!("exc_details [ 1 ]");
        // } catch  BaseException  {
        exc_details [ 1 ] . __context__ = fixed_ctx;
        panic!("");
        return  received_exc && suppressed_exc;
        pub fn close ( self )  {
        "Immediately unwind the context stack.";
        self . __exit__ ( None /* Option */ , None /* Option */ , None /* Option */ );
        class AsyncExitStack ( _BaseExitStack , AbstractAsyncContextManager ) ;
        "Async context manager.iter().map(|dynamic management of a stack of exit
    callbacks.

    For example:
        async with AsyncExitStack() as stack:
            connections = vec![await stack.enter_async_context(get_connection())
               .iter().map(|i| range(5)]
            # All opened connections will automatically be released at the
            # end of the async with statement, even if attempts to open a
            # connection later| the list raise an exception.
    ";
        @ staticmethod;
        pub fn _create_async_exit_wrapper ( cm , cm_exit )  {
        return  MethodType ( cm_exit , cm );
        @ staticmethod;
        pub fn _create_async_cb_wrapper ( callback , / , * args , ** kwds )  {
        async def _exit_wrapper ( exc_type , exc , tb ) ;
        await callback ( * args , ** kwds );
        return  _exit_wrapper;
        async def enter_async_context ( self , cm ) ;
        "Enters the supplied async context manager.

        If successful, also pushes its __aexit__ method as a callback and
        returns the result of the __aenter__ method.
        ";
        cls = type ( cm );
        // try {
        _enter = cls . __aenter__;
        _exit = cls . __aexit__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "'{cls.__module__}.{cls.__qualname__}' object does "");
        format!("not support the asynchronous context manager protocol");
        ) from None /* Option */;
        result = await _enter ( cm );
        self . _push_async_cm_exit ( cm , _exit );
        return  result;
        pub fn push_async_exit ( &self, exit )  {
        "Registers a coroutine function with the standard __aexit__ method
        signature.

        Can suppress exceptions the same way __aexit__ method can.
        Also accepts any object with an __aexit__ method (registering a call
        to the method instead of the object itself).
        ";
        _cb_type = type ( exit );
        // try {
        exit_method = _cb_type . __aexit__;
        // } catch  AttributeError  {
        self . _push_exit_callback ( exit , false );
        } else {
        self . _push_async_cm_exit ( exit , exit_method );
        return  exit;
        pub fn push_async_callback ( &self, callback , / , * args , ** kwds )  {
        "Registers an arbitrary coroutine function && arguments.

        Cannot suppress exceptions.
        ";
        _exit_wrapper = self . _create_async_cb_wrapper ( callback , * args , ** kwds );
        _exit_wrapper . __wrapped__ = callback;
        self . _push_exit_callback ( _exit_wrapper , false );
        return  callback;
        async def aclose ( self ) ;
        "Immediately unwind the context stack.";
        await self . __aexit__ ( None /* Option */ , None /* Option */ , None /* Option */ );
        pub fn _push_async_cm_exit ( &self, cm , cm_exit )  {
        "Helper to correctly register coroutine function to __aexit__
        method.";
        _exit_wrapper = self . _create_async_exit_wrapper ( cm , cm_exit );
        self . _push_exit_callback ( _exit_wrapper , false );
        async def __aenter__ ( self ) ;
        return  self;
        async def __aexit__ ( self , * exc_details ) ;
        received_exc = exc_details [ 0 ] == !None /* Option */;
        frame_exc = sys . exc_info ( ) [ 1 ];
        pub fn _fix_exception_context ( new_exc , old_exc )  {
        while 1  {
        exc_context = new_exc . __context__;
        if exc_context is None /* Option */ || exc_context is old_exc {
        return;
        if exc_context is frame_exc {
        break;
        new_exc = exc_context;
        new_exc . __context__ = old_exc;
        suppressed_exc = false;
        pending_raise = false;
        while self . _exit_callbacks  {
        is_sync , cb = self . _exit_callbacks . pop ( );
        // try {
        if is_sync {
        cb_suppress = cb ( * exc_details );
        } else {
        cb_suppress = await cb ( * exc_details );
        if cb_suppress {
        suppressed_exc = true;
        pending_raise = false;
        exc_details = ( None /* Option */ , None /* Option */ , None /* Option */ );
        // } catch   {
        new_exc_details = sys . exc_info ( );
        _fix_exception_context ( new_exc_details [ 1 ] , exc_details [ 1 ] );
        pending_raise = true;
        exc_details = new_exc_details;
        if pending_raise {
        // try {
        fixed_ctx = exc_details [ 1 ] . __context__;
        panic!("exc_details [ 1 ]");
        // } catch  BaseException  {
        exc_details [ 1 ] . __context__ = fixed_ctx;
        panic!("");
        return  received_exc && suppressed_exc;
        class nullcontext ( AbstractContextManager , AbstractAsyncContextManager ) ;
        "Context manager that does no additional processing.

    Used as a stand-in for a normal context manager, when a particular
    block of code == only sometimes used with a normal context manager:

    cm = optional_cm if condition else nullcontext()
    with cm:
        # Perform operation, using optional_cm if condition == true
    ";
        pub fn __init__ ( &self, enter_result = None /* Option */ )  {
        self . enter_result = enter_result;
        pub fn __enter__ ( self )  {
        return  self . enter_result;
        pub fn __exit__ ( &self, * excinfo )  {
        // pass
        async def __aenter__ ( self ) ;
        return  self . enter_result;
        async def __aexit__ ( self , * excinfo ) ;
        // pass
        class chdir ( AbstractContextManager ) ;
        "Non thread-safe context manager to change the current working directory.";
        pub fn __init__ ( &self, path )  {
        self . path = path;
        self . _old_cwd = [ ];
        pub fn __enter__ ( self )  {
        self . _old_cwd . append ( os . getcwd ( ) );
        os . chdir ( self . path );
        pub fn __exit__ ( &self, * excinfo )  {
        os . chdir ( self . _old_cwd . pop ( ) );
}

