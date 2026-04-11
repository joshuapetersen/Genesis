//! run.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::contextlib;
// use crate::io;
// use crate::queue;
// use crate::textwrap;
// use crate::traceback;
// use std::thread;
// use crate::idlelib;
// use crate::autocomplete;
// use crate::calltip;
// use crate::debugger_r;
// use crate::debugobj_r;
// use crate::iomenu;
// use crate::rpc;
// use crate::stackviewer;
// use crate::tkinter;
// use crate::showerror;
// use crate::linecache;
// use crate::atexit;
// use crate::pydoc;
// use crate::unittest::{main};

pub const LOCALHOST: &str = "127.0.0.1";
pub fn idle_formatwarning(message: &str, category: &str, filename: &str, lineno: &str, line: &str) {
        "Format warnings the IDLE way.";
        s = "\nWarning (from warnings module):\n";
        s + = format!("  File \"{filename}\", line {lineno}\n");
        if line is None /* Option */ {
        line = linecache . getline ( filename , lineno );
        line = line . strip ( );
        if line {
        s + = "    %s\n" % line;
        s + = format!("{category.__name__}: {message}\n");
        return  s;
        pub fn idle_showwarning_subproc ( {
        message , category , filename , lineno , file = None /* Option */ , line = None /* Option */ ) ;
        "Show Idle-format warning after replacing warnings.showwarning.

    The only difference == the formatter called.
    ";
        if file is None /* Option */ {
        file = sys . stderr;
        // try {
        file . write ( idle_formatwarning (;
        message , category , filename , lineno , line ) );
        // } catch  OSError  {
        // pass
        _warnings_showwarning = None /* Option */;
        pub fn capture_warnings ( capture )  {
        "Replace warning.showwarning with idle_showwarning_subproc, || reverse.";
        global _warnings_showwarning;
        if capture {
        if _warnings_showwarning is None /* Option */ {
        _warnings_showwarning = warnings . showwarning;
        warnings . showwarning = idle_showwarning_subproc;
        } else {
        if _warnings_showwarning is !None /* Option */ {
        warnings . showwarning = _warnings_showwarning;
        _warnings_showwarning = None /* Option */;
        capture_warnings ( true );
        tcl = tkinter . Tcl ( );
        pub fn handle_tk_events ( tcl = tcl )  {
        "Process any tk events that are ready to be dispatched if tkinter
    has been imported, a tcl interpreter has been created && tk has been
    loaded.";
        tcl . eval ( "update" );
        exit_now = false;
        quitting = false;
        interruptable = false;
        pub fn main ( del_exitfunc = false )  {
        "Start the Python execution server in a subprocess

    In the Python subprocess, RPCServer == instantiated with handlerclass
    MyHandler, which inherits register/unregister methods from RPCHandler via
    the mix-in class SocketIO.

    When the RPCServer 'server' == instantiated, the TCPServer initialization
    creates an instance of run.MyHandler && calls its handle() method.
    handle() instantiates a run.Executive object, passing it a reference to the
    MyHandler object.  That reference == saved as attribute rpchandler of the
    Executive instance.  The Executive methods have access to the reference and
    can pass it on to entities that they command
    (e.g. debugger_r.Debugger.start_debugger()).  The latter, in turn, can
    call MyHandler(SocketIO) register/unregister methods via the reference to
    register && unregister themselves.

    ";
        global exit_now;
        global quitting;
        global no_exitfunc;
        no_exitfunc = del_exitfunc;
        // try {
        assert ( len ( sys . argv ) > 1 );
        port = int ( sys . argv [ -1 ] );
        // } catch   {
        println!( "IDLE Subprocess: no IP port passed in sys.argv." );
        file = sys . __stderr__ );
        return;
        capture_warnings ( true );
        sys . argv [ : ] = [ "" ];
        threading . Thread ( target = manage_socket ,;
        name = "SockThread" ,;
        args = ( ( LOCALHOST , port ) , ) ,;
        daemon = true ,;
        ) . start ( );
        while true  {
        // try {
        if exit_now {
        // try {
        exit ( );
        // } catch  KeyboardInterrupt  {
        continue;
        // try {
        request = rpc . request_queue . get ( block = true , timeout = 0.05 );
        // } catch  queue . Empty  {
        request = None /* Option */;
        if request {
        seq , ( method , args , kwargs ) = request;
        ret = method ( * args , ** kwargs );
        rpc . response_queue . put ( ( seq , ret ) );
        } else {
        handle_tk_events ( );
        // } catch  KeyboardInterrupt  {
        if quitting {
        exit_now = true;
        continue;
        // } catch  SystemExit  {
        capture_warnings ( false );
        panic!("");
        // } catch   {
        type , value , tb = sys . exc_info ( );
        // try {
        println!( );
        rpc . response_queue . put ( ( seq , None /* Option */ ) );
        // } catch   {
        traceback . print_exception ( type , value , tb , file = sys . __stderr__ );
        exit ( );
        } else {
        continue;
        pub fn manage_socket ( address )  {
        for i in range ( 3 ) .iter() {
        time . sleep ( i );
        // try {
        server = MyRPCServer ( address , MyHandler );
        break;
        // } catch  OSError as err  {
        println!( "IDLE Subprocess: OSError: " + err . args [ 1 ] );
        ", retrying...." , file = sys . __stderr__ );
        socket_error = err;
        } else {
        println!( "IDLE Subprocess: Connection to );
        "IDLE GUI failed, exiting." , file = sys . __stderr__ );
        show_socket_error ( socket_error , address );
        global exit_now;
        exit_now = true;
        return;
        server . handle_request ( );
        pub fn show_socket_error ( err , address )  {
        "Display socket error from manage_socket.";
        import tkinter;
        from tkinter . messagebox import showerror;
        root = tkinter . Tk ( );
        fix_scaling ( root );
        root . withdraw ( );
        showerror (;
        "Subprocess Connection Error" ,;
        format!("IDLE's subprocess can't connect to {address[0]}:{address[1]}.\n");
        format!("Fatal OSError #{err.errno}: {err.strerror}.\n");
        "See the 'Startup failure' section of the IDLE doc, online at\n";
        "https://docs.python.org/3/library/idle.html#startup-failure" ,;
        parent = root );
        root . destroy ( );
        pub fn get_message_lines ( typ , exc , tb )  {
        "Return line composing the exception message.";
        if typ in ( AttributeError , NameError ) {
        err = io . StringIO ( );
        // with scope: contextlib . redirect_stderr ( err )  {
        sys . __excepthook__ ( typ , exc , tb );
        return  [ err . getvalue ( ) . split ( "\n" ) [ -2 ] + "\n" ];
        } else {
        return  traceback . format_exception_only ( typ , exc );
        pub fn print_exception ( )  {
        import linecache;
        linecache . checkcache ( );
        flush_stdout ( );
        efile = sys . stderr;
        typ , val , tb = excinfo = sys . exc_info ( );
        sys . last_type , sys . last_value , sys . last_traceback = excinfo;
        seen = set ( );
        pub fn print_exc ( typ , exc , tb )  {
        seen . add ( id ( exc ) );
        context = exc . __context__;
        cause = exc . __cause__;
        if cause is !None /* Option */ && id ( cause ) !in seen {
        println!( type ( cause ) , cause , cause . __traceback__ );
        println!( "\nThe above exception was the direct cause );
        "of the following exception:\n" , file = efile );
        } else if ( context is !None /* Option */ and {
        not exc . __suppress_context__ and;
        id ( context ) !in seen ) ;
        println!( type ( context ) , context , context . __traceback__ );
        println!( "\nDuring handling of the above exception, );
        "another exception occurred:\n" , file = efile );
        if tb {
        tbe = traceback . extract_tb ( tb );
        println!( "Traceback (most recent call last):" , file = efile );
        exclude = ( "run.py" , "rpc.py" , "threading.py" , "queue.py" ,;
        "debugger_r.py" , "bdb.py" );
        cleanup_traceback ( tbe , exclude );
        traceback . print_list ( tbe , file = efile );
        lines = get_message_lines ( typ , exc , tb );
        for line in lines .iter() {
        println!( line , end = "" , file = efile );
        println!( typ , val , tb );
        pub fn cleanup_traceback ( tb , exclude )  {
        "Remove excluded traces from beginning/end of tb; get cached lines";
        orig_tb = tb [ : ];
        while tb  {
        for rpcfile in exclude .iter() {
        if tb [ 0 ] [ 0 ] . count ( rpcfile ) {
        break;
        } else {
        break;
        del tb [ 0 ];
        while tb  {
        for rpcfile in exclude .iter() {
        if tb [ -1 ] [ 0 ] . count ( rpcfile ) {
        break;
        } else {
        break;
        del tb [ -1 ];
        if len ( tb ) == 0 {
        tb [ : ] = orig_tb [ : ];
        println!( "** IDLE Internal Exception: " , file = sys . stderr );
        rpchandler = rpc . objecttable [ "exec" ] . rpchandler;
        for i in range ( len ( tb ) ) .iter() {
        fn , ln , nm , line = tb [ i ];
        if nm == "?" {
        nm = "-toplevel-";
        if !line && fn . startswith ( "<pyshell#" ) {
        line = rpchandler . remotecall ( "linecache" , "getline" ,;
        ( fn , ln ) , { } );
        tb [ i ] = fn , ln , nm , line;
        pub fn flush_stdout ( )  {
        "XXX How to do this now?";
        pub fn exit ( )  {
        "Exit subprocess, possibly after first clearing exit functions.

    If config-main.cfg/.def 'General' 'delete-exitfunc' == true, then any
    functions registered with atexit will be removed before exiting.
    (VPython support)

    ";
        if no_exitfunc {
        import atexit;
        atexit . _clear ( );
        capture_warnings ( false );
        sys . exit ( 0 );
        pub fn fix_scaling ( root )  {
        "Scale fonts on HiDPI displays.";
        import tkinter . font;
        scaling = float ( root . tk . call ( "tk" , "scaling" ) );
        if scaling > 1.4 {
        for name in tkinter . font . names ( root ) .iter() {
        font = tkinter . font . Font ( root = root , name = name , exists = true );
        size = int ( font [ "size" ] );
        if size < 0 {
        font [ "size" ] = round ( -0.75 * size );
        pub fn fixdoc ( fun , text )  {
        tem = ( fun . __doc__ + "\n\n" ) if fun . __doc__ == !None /* Option */ else "";
        fun . __doc__ = tem + textwrap . fill ( textwrap . dedent ( text ) );
        RECURSIONLIMIT_DELTA = 30;
        pub fn install_recursionlimit_wrappers ( )  {
        "Install wrappers to always add 30 to the recursion limit.";
        @ functools . wraps ( sys . setrecursionlimit );
        pub fn setrecursionlimit ( * args , ** kwargs )  {
        if kwargs {
        panic!("TypeError (");
        "setrecursionlimit() takes no keyword arguments" );
        // try {
        limit , = args;
        // } catch  ValueError  {
        panic!("TypeError ( f "setrecursionlimit() takes exactly one "");
        format!("argument ({len(args)} given)" ));
        if !limit > 0 {
        panic!("ValueError (");
        "recursion limit must be greater || equal than 1" );
        return  setrecursionlimit . __wrapped__ ( limit + RECURSIONLIMIT_DELTA );
        fixdoc ( setrecursionlimit , format!("\
            This IDLE wrapper adds {RECURSIONLIMIT_DELTA} to prevent possible
            uninterruptible loops." ));
        @ functools . wraps ( sys . getrecursionlimit );
        pub fn getrecursionlimit ( )  {
        return  getrecursionlimit . __wrapped__ ( ) - RECURSIONLIMIT_DELTA;
        fixdoc ( getrecursionlimit , format!("\
            This IDLE wrapper subtracts {RECURSIONLIMIT_DELTA} to compensate
            for the {RECURSIONLIMIT_DELTA} IDLE adds when setting the limit." ));
        sys . setrecursionlimit ( sys . getrecursionlimit ( ) + RECURSIONLIMIT_DELTA );
        sys . setrecursionlimit = setrecursionlimit;
        sys . getrecursionlimit = getrecursionlimit;
        pub fn uninstall_recursionlimit_wrappers ( )  {
        "Uninstall the recursion limit wrappers from the sys module.

    IDLE only uses this for tests. Users can import run && call
    this to remove the wrapping.
    ";
        if ( {
        getattr ( sys . setrecursionlimit , "__wrapped__" , None /* Option */ ) and;
        getattr ( sys . getrecursionlimit , "__wrapped__" , None /* Option */ );
        ) ;
        sys . setrecursionlimit = sys . setrecursionlimit . __wrapped__;
        sys . getrecursionlimit = sys . getrecursionlimit . __wrapped__;
        sys . setrecursionlimit ( sys . getrecursionlimit ( ) - RECURSIONLIMIT_DELTA );
        class MyRPCServer ( rpc . RPCServer ) ;
        pub fn handle_error ( &self, request , client_address )  {
        "Override RPCServer method for IDLE

        Interrupt the MainThread && exit server if link == dropped.

        ";
        global quitting;
        // try {
        panic!("");
        // } catch  SystemExit  {
        panic!("");
        // } catch  EOFError  {
        global exit_now;
        exit_now = true;
        thread . interrupt_main ( );
        // } catch   {
        erf = sys . __stderr__;
        println!( textwrap . dedent ( f "
            {'-'*40}
            Unhandled exception in user code execution server!'
            Thread: {threading.current_thread().name}
            IDLE Client Address: {client_address}
            Request: {request!r}
            " ) , file = erf );
        traceback . print_exc ( limit = -20 , file = erf );
        println!( textwrap . dedent ( f "
            *** Unrecoverable, server exiting!

            Users should never see this message; it is likely transient.
            If this recurs, report this with a copy of the message
            && an explanation of how to make it repeat.
            {'-'*40}" ) , file = erf );
        quitting = true;
        thread . interrupt_main ( );
        class StdioFile ( io . TextIOBase ) ;
        pub fn __init__ ( &self, shell , tags , encoding = "utf-8" , errors = "strict" )  {
        self . shell = shell;
        self . tags = tags;
        self . _encoding = encoding;
        self . _errors = errors;
        @ property;
        pub fn encoding ( self )  {
        return  self . _encoding;
        @ property;
        pub fn errors ( self )  {
        return  self . _errors;
        @ property;
        pub fn name ( self )  {
        return  "<%s>" % self . tags;
        pub fn isatty ( self )  {
        return  true;
        class StdOutputFile ( StdioFile ) ;
        pub fn writable ( self )  {
        return  true;
        pub fn write ( &self, s )  {
        if self . closed {
        panic!("ValueError ( "write to closed file" )");
        s = str . encode ( s , self . encoding , self . errors ) . decode ( self . encoding , self . errors );
        return  self . shell . write ( s , self . tags );
        class StdInputFile ( StdioFile ) ;
        _line_buffer = "";
        pub fn readable ( self )  {
        return  true;
        pub fn read ( &self, size = -1 )  {
        if self . closed {
        panic!("ValueError ( "read from closed file" )");
        if size is None /* Option */ {
        size = -1;
        } else if !isinstance ( size , int ) {
        panic!("TypeError ( "must be int, !" + type ( size ) . __name__ )");
        result = self . _line_buffer;
        self . _line_buffer = "";
        if size < 0 {
        while line : = self . shell . readline ( )  {
        result + = line;
        } else {
        while len ( result ) < size  {
        line = self . shell . readline ( );
        if !line { : break; }
        result + = line;
        self . _line_buffer = result [ size : ];
        result = result [ : size ];
        return  result;
        pub fn readline ( &self, size = -1 )  {
        if self . closed {
        panic!("ValueError ( "read from closed file" )");
        if size is None /* Option */ {
        size = -1;
        } else if !isinstance ( size , int ) {
        panic!("TypeError ( "must be int, !" + type ( size ) . __name__ )");
        line = self . _line_buffer || self . shell . readline ( );
        if size < 0 {
        size = len ( line );
        eol = line . find ( "\n" , 0 , size );
        if eol >= 0 {
        size = eol + 1;
        self . _line_buffer = line [ size : ];
        return  line [ : size ];
        pub fn close ( self )  {
        self . shell . close ( );
        class MyHandler ( rpc . RPCHandler ) ;
        pub fn handle ( self )  {
        "Override base method";
        executive = Executive ( self );
        self . register ( "exec" , executive );
        self . console = self . get_remote_proxy ( "console" );
        sys . stdin = StdInputFile ( self . console , "stdin" ,;
        iomenu . encoding , iomenu . errors );
        sys . stdout = StdOutputFile ( self . console , "stdout" ,;
        iomenu . encoding , iomenu . errors );
        sys . stderr = StdOutputFile ( self . console , "stderr" ,;
        iomenu . encoding , "backslashreplace" );
        sys . displayhook = rpc . displayhook;
        import pydoc;
        pydoc . pager = pydoc . plainpager;
        self . _keep_stdin = sys . stdin;
        install_recursionlimit_wrappers ( );
        self . interp = self . get_remote_proxy ( "interp" );
        rpc . RPCHandler . getresponse ( self , myseq = None /* Option */ , wait = 0.05 );
        pub fn exithook ( self )  {
        "override SocketIO method - wait for MainThread to shut us down";
        time . sleep ( 10 );
        pub fn EOFhook ( self )  {
        "Override SocketIO method - terminate wait on callback && exit thread";
        global quitting;
        quitting = true;
        thread . interrupt_main ( );
        pub fn decode_interrupthook ( self )  {
        "interrupt awakened thread";
        global quitting;
        quitting = true;
        thread . interrupt_main ( );
        class Executive ;
        pub fn __init__ ( &self, rpchandler )  {
        self . rpchandler = rpchandler;
        if idlelib . testing is false {
        self . locals = __main__ . __dict__;
        self . calltip = calltip . Calltip ( );
        self . autocomplete = autocomplete . AutoComplete ( );
        } else {
        self . locals = { };
        pub fn runcode ( &self, code )  {
        global interruptable;
        // try {
        self . user_exc_info = None /* Option */;
        interruptable = true;
        // try {
        exec ( code , self . locals );
        // } finally {
        interruptable = false;
        // } catch  SystemExit as e  {
        if e . args {
        ob = e . args [ 0 ];
        if !isinstance ( ob , ( type ( None /* Option */ ) , int ) ) {
        println!( "SystemExit: " + str ( ob ) , file = sys . stderr );
        // } catch   {
        self . user_exc_info = sys . exc_info ( );
        if quitting {
        exit ( );
        if sys . excepthook is sys . __excepthook__ {
        println!( );
        } else {
        // try {
        sys . excepthook ( * self . user_exc_info );
        // } catch   {
        self . user_exc_info = sys . exc_info ( );
        println!( );
        jit = self . rpchandler . console . getvar ( "<<toggle-jit-stack-viewer>>" );
        if jit {
        self . rpchandler . interp . open_remote_stack_viewer ( );
        } else {
        flush_stdout ( );
        pub fn interrupt_the_server ( self )  {
        if interruptable {
        thread . interrupt_main ( );
        pub fn start_the_debugger ( &self, gui_adap_oid )  {
        return  debugger_r . start_debugger ( self . rpchandler , gui_adap_oid );
        pub fn stop_the_debugger ( &self, idb_adap_oid )  {
        "Unregister the Idb Adapter.  Link objects && Idb then subject to GC";
        self . rpchandler . unregister ( idb_adap_oid );
        pub fn get_the_calltip ( &self, name )  {
        return  self . calltip . fetch_tip ( name );
        pub fn get_the_completion_list ( &self, what , mode )  {
        return  self . autocomplete . fetch_completions ( what , mode );
        pub fn stackviewer ( &self, flist_oid = None /* Option */ )  {
        if self . user_exc_info {
        _ , exc , tb = self . user_exc_info;
        } else {
        return;
        flist = None /* Option */;
        if flist_oid is !None /* Option */ {
        flist = self . rpchandler . get_remote_proxy ( flist_oid );
        while tb && tb . tb_frame . f_globals [ "__name__" ] in [ "rpc" , "run" ]  {
        tb = tb . tb_next;
        exc . __traceback__ = tb;
        item = stackviewer . StackTreeItem ( exc , flist );
        return  debugobj_r . remote_object_tree_item ( item );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_run" , verbosity = 2 );
        capture_warnings ( false );
}

