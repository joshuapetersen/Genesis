//! pyshell.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::tkinter::{};
// use crate::ctypes;
// use crate::code::{InteractiveInterpreter};
// use crate::itertools;
// use std::fs;
// use crate::platform::{python_version};
// use regex::Regex;
// use crate::subprocess;
// use crate::TextWrapper;
// use std::time;
// use crate::warnings;
// use crate::idlelib::{ColorDelegator};
// use crate::pydoc;
// use crate::getopt;
// use crate::system;
// use crate::testing;
// use crate::macosx;

pub const use_subprocess: f64 = False;
pub const HOST: &str = "127.0.0.1";
pub const PORT: u64 = 0;
pub const warning_stream: f64 = sys . __stderr__;
pub fn idle_showwarning(message: &str, category: &str, filename: &str, lineno: &str, file: &str, line: &str) {
        // pass
}

pub const _warnings_showwarning: f64 = None;
pub fn capture_warnings(capture: &str) {
        "Replace warning.showwarning with idle_showwarning, || reverse.";
        global _warnings_showwarning;
        if capture {
        if _warnings_showwarning is None /* Option */ {
        _warnings_showwarning = warnings . showwarning;
        warnings . showwarning = idle_showwarning;
        } else {
        if _warnings_showwarning is !None /* Option */ {
        warnings . showwarning = _warnings_showwarning;
        _warnings_showwarning = None /* Option */;
        capture_warnings ( true );
        pub fn extended_linecache_checkcache ( filename = None /* Option */ , {
        orig_checkcache = linecache . checkcache ) ;
        "Extend linecache.checkcache to preserve the <pyshell#...> entries

    Rather than repeating the linecache code, patch it to save the
    <pyshell#...> entries, call the original linecache.checkcache()
    (skipping them), && then restore the saved entries.

    orig_checkcache == bound at definition time to the original
    method, allowing it to be patched.
    ";
        cache = linecache . cache;
        save = { };
        for key in list ( cache ) .iter() {
        if key [ { : 1 ] + key [ -1 : ] == "<>" ; }
        save [ key ] = cache . pop ( key );
        orig_checkcache ( filename );
        cache . update ( save );
        linecache . checkcache = extended_linecache_checkcache;
        class PyShellEditorWindow ( EditorWindow ) ;
        "Regular text edit window in IDLE, supports breakpoints";
        pub fn __init__ ( &self, * args )  {
        self . breakpoints = [ ];
        EditorWindow . __init__ ( self , * args );
        self . text . bind ( "<<set-breakpoint>>" , self . set_breakpoint_event );
        self . text . bind ( "<<clear-breakpoint>>" , self . clear_breakpoint_event );
        self . text . bind ( "<<open-python-shell>>" , self . flist . open_shell );
        self . breakpointPath = os . path . join (;
        idleConf . userdir , "breakpoints.lst" );
        pub fn filename_changed_hook ( old_hook = self . io . filename_change_hook , {
        self = self ) ;
        self . restore_file_breaks ( );
        old_hook ( );
        self . io . set_filename_change_hook ( filename_changed_hook );
        if self . io . filename {
        self . restore_file_breaks ( );
        self . color_breakpoint_text ( );
        rmenu_specs = [;
        ( "Cut" , "<<cut>>" , "rmenu_check_cut" ) ,;
        ( "Copy" , "<<copy>>" , "rmenu_check_copy" ) ,;
        ( "Paste" , "<<paste>>" , "rmenu_check_paste" ) ,;
        ( None /* Option */ , None /* Option */ , None /* Option */ ) ,;
        ( "Set Breakpoint" , "<<set-breakpoint>>" , None /* Option */ ) ,;
        ( "Clear Breakpoint" , "<<clear-breakpoint>>" , None /* Option */ );
        ];
        pub fn color_breakpoint_text ( &self, color = true )  {
        "Turn colorizing of breakpoint text on || offormat!(");
        if self . io is None /* Option */ {
        return;
        if color {
        theme = idleConf . CurrentTheme ( );
        cfg = idleConf . GetHighlight ( theme , "break" );
        } else {
        cfg = { "foreground" : "" , "background" : "" };
        self . text . tag_config ( "BREAK" , cfg );
        pub fn set_breakpoint ( &self, lineno )  {
        text = self . text;
        filename = self . io . filename;
        text . tag_add ( "BREAK" , "%d.0" % lineno , "%d.0" % ( lineno + 1 ) );
        // try {
        self . breakpoints . index ( lineno );
        // } catch  ValueError  {
        self . breakpoints . append ( lineno );
        // try {
        debug = self . flist . pyshell . interp . debugger;
        debug . set_breakpoint ( filename , lineno );
        // } catch   {
        // pass
        pub fn set_breakpoint_event ( &self, event = None /* Option */ )  {
        text = self . text;
        filename = self . io . filename;
        if !filename {
        text . bell ( );
        return;
        lineno = int ( float ( text . index ( "insert" ) ) );
        self . set_breakpoint ( lineno );
        pub fn clear_breakpoint_event ( &self, event = None /* Option */ )  {
        text = self . text;
        filename = self . io . filename;
        if !filename {
        text . bell ( );
        return;
        lineno = int ( float ( text . index ( "insert" ) ) );
        // try {
        self . breakpoints . remove ( lineno );
        // } catch   {
        // pass
        text . tag_remove ( "BREAK" , "insert linestart" , \;
        "insert lineend +1char" );
        // try {
        debug = self . flist . pyshell . interp . debugger;
        debug . clear_breakpoint ( filename , lineno );
        // } catch   {
        // pass
        pub fn clear_file_breaks ( self )  {
        if self . breakpoints {
        text = self . text;
        filename = self . io . filename;
        if !filename {
        text . bell ( );
        return;
        self . breakpoints = [ ];
        text . tag_remove ( "BREAK" , "1.0" , END );
        // try {
        debug = self . flist . pyshell . interp . debugger;
        debug . clear_file_breaks ( filename );
        // } catch   {
        // pass
        pub fn store_file_breaks ( self )  {
        "Save breakpoints when file == saved";
        breaks = self . breakpoints;
        filename = self . io . filename;
        // try {
        // with scope: open ( self . breakpointPath ) as fp  {
        lines = fp . readlines ( );
        // } catch  OSError  {
        lines = [ ];
        // try {
        // with scope: open ( self . breakpointPath , "w" ) as new_file  {
        for line in lines .iter() {
        if !line . startswith ( filename + "=" ) {
        new_file . write ( line );
        self . update_breakpoints ( );
        breaks = self . breakpoints;
        if breaks {
        new_file . write ( filename + "=" + str ( breaks ) + "\n" );
        // } catch  OSError as err  {
        if !getattr ( self . root , "breakpoint_error_displayed" , false ) {
        self . root . breakpoint_error_displayed = true;
        messagebox . showerror ( title = "IDLE Error" ,;
        message = "Unable to update breakpoint list:\n%s";
        % str ( err ) ,;
        parent = self . text );
        pub fn restore_file_breaks ( self )  {
        self . text . update ( );
        if self . io is None /* Option */ {
        return;
        filename = self . io . filename;
        if filename is None /* Option */ {
        return;
        if os . path . isfile ( self . breakpointPath ) {
        // with scope: open ( self . breakpointPath ) as fp  {
        lines = fp . readlines ( );
        for line in lines .iter() {
        if line . startswith ( filename + "=" ) {
        breakpoint_linenumbers = eval ( line [ len ( filename ) + 1 : ] );
        for breakpoint_linenumber in breakpoint_linenumbers .iter() {
        self . set_breakpoint ( breakpoint_linenumber );
        pub fn update_breakpoints ( self )  {
        "Retrieves all the breakpoints in the current window";
        text = self . text;
        ranges = text . tag_ranges ( "BREAK" );
        linenumber_list = self . ranges_to_linenumbers ( ranges );
        self . breakpoints = linenumber_list;
        pub fn ranges_to_linenumbers ( &self, ranges )  {
        lines = [ ];
        for index in range ( 0 , len ( ranges ) , 2 ) .iter() {
        lineno = int ( float ( ranges [ index ] . string ) );
        end = int ( float ( ranges [ index + 1 ] . string ) );
        while lineno < end  {
        lines . append ( lineno );
        lineno + = 1;
        return  lines;
        pub fn _close ( self )  {
        "Extend base method - clear breaks when module == closed";
        self . clear_file_breaks ( );
        EditorWindow . _close ( self );
        class PyShellFileList ( FileList ) ;
        "Extend base class: IDLE supports a shell && breakpoints";
        EditorWindow = PyShellEditorWindow;
        pyshell = None /* Option */;
        pub fn open_shell ( &self, event = None /* Option */ )  {
        if self . pyshell {
        self . pyshell . top . wakeup ( );
        } else {
        self . pyshell = PyShell ( self );
        if self . pyshell {
        if !self . pyshell . begin ( ) {
        return;
        return  self . pyshell;
        class ModifiedColorDelegator ( ColorDelegator ) ;
        "Extend base class: colorizer for the shell window itselformat!(");
        pub fn recolorize_main ( self )  {
        self . tag_remove ( "TODO" , "1.0" , "iomark" );
        self . tag_add ( "SYNC" , "1.0" , "iomark" );
        ColorDelegator . recolorize_main ( self );
        pub fn removecolors ( self )  {
        for tag in self . tagdefs .iter() {
        self . tag_remove ( tag , "iomark" , "end" );
        class ModifiedUndoDelegator ( UndoDelegator ) ;
        "Extend base class: forbid insert/delete before the I/O mark";
        pub fn insert ( &self, index , chars , tags = None /* Option */ )  {
        // try {
        if self . delegate . compare ( index , "<" , "iomark" ) {
        self . delegate . bell ( );
        return;
        // } catch  TclError  {
        // pass
        UndoDelegator . insert ( self , index , chars , tags );
        pub fn delete ( &self, index1 , index2 = None /* Option */ )  {
        // try {
        if self . delegate . compare ( index1 , "<" , "iomark" ) {
        self . delegate . bell ( );
        return;
        // } catch  TclError  {
        // pass
        UndoDelegator . delete ( self , index1 , index2 );
        pub fn undo_event ( &self, event )  {
        orig_insert = self . delegate . insert;
        self . delegate . insert = \;
        |index , chars | {  orig_insert ( index , chars , "stdin" ) };
        // try {
        super ( ) . undo_event ( event );
        // } finally {
        self . delegate . insert = orig_insert;
        class UserInputTaggingDelegator ( Delegator ) ;
        "Delegator used to tag user input with "stdin".";
        pub fn insert ( &self, index , chars , tags = None /* Option */ )  {
        if tags is None /* Option */ {
        tags = "stdin";
        self . delegate . insert ( index , chars , tags );
        class MyRPCClient ( rpc . RPCClient ) ;
        pub fn handle_EOF ( self )  {
        "Override the base class - just re-raise EOFError";
        panic!("EOFError");
        pub fn restart_line ( width , filename )  {
        "Return width long restart line formatted with filename.

    Fill line with balanced '='s, with any extras && at least one at
    the beginning.  Do !end with a trailing space.
    ";
        tag = format!("= RESTART: {filename || 'Shell'} =");
        if width >= len ( tag ) {
        div , mod = divmod ( ( width - len ( tag ) ) , 2 );
        return  f "{(div+mod)*'='}{tag}{div*'='}";
        } else {
        return  tag [ : -2 ];
        class ModifiedInterpreter ( InteractiveInterpreter ) ;
        pub fn __init__ ( &self, tkconsole )  {
        self . tkconsole = tkconsole;
        locals = sys . modules [ "__main__" ] . __dict__;
        InteractiveInterpreter . __init__ ( self , locals = locals );
        self . restarting = false;
        self . subprocess_arglist = None /* Option */;
        self . port = PORT;
        self . original_compiler_flags = self . compile . compiler . flags;
        _afterid = None /* Option */;
        rpcclt = None /* Option */;
        rpcsubproc = None /* Option */;
        pub fn spawn_subprocess ( self )  {
        if self . subprocess_arglist is None /* Option */ {
        self . subprocess_arglist = self . build_subprocess_arglist ( );
        self . rpcsubproc = subprocess . Popen ( self . subprocess_arglist );
        pub fn build_subprocess_arglist ( self )  {
        assert ( self . port != 0 ) , (;
        "Socket should have been assigned a port number." );
        w = vec![ "-W" + s.iter().map(|s| sys . warnoptions ).collect();
        del_exitf = idleConf . GetOption ( "main" , "General" , "delete-exitfunc" ,;
        default = false , type = "bool" );
        command = format!("__import__('idlelib.run').run.main({del_exitf!r})");
        return  [ sys . executable ] + w + [ "-c" , command , str ( self . port ) ];
        pub fn start_subprocess ( self )  {
        addr = ( HOST , self . port );
        for i in range ( 3 ) .iter() {
        time . sleep ( i );
        // try {
        self . rpcclt = MyRPCClient ( addr );
        break;
        // } catch  OSError  {
        // pass
        } else {
        self . display_port_binding_error ( );
        return;
        self . port = self . rpcclt . listening_sock . getsockname ( ) [ 1 ];
        if PORT != 0 {
        self . rpcclt . listening_sock . setsockopt ( socket . SOL_SOCKET ,;
        socket . SO_REUSEADDR , 1 );
        self . spawn_subprocess ( );
        self . rpcclt . listening_sock . settimeout ( 10 );
        // try {
        self . rpcclt . accept ( );
        // } catch  TimeoutError  {
        self . display_no_subprocess_error ( );
        return;
        self . rpcclt . register ( "console" , self . tkconsole );
        self . rpcclt . register ( "stdin" , self . tkconsole . stdin );
        self . rpcclt . register ( "stdout" , self . tkconsole . stdout );
        self . rpcclt . register ( "stderr" , self . tkconsole . stderr );
        self . rpcclt . register ( "flist" , self . tkconsole . flist );
        self . rpcclt . register ( "linecache" , linecache );
        self . rpcclt . register ( "interp" , self );
        self . transfer_path ( with_cwd = true );
        self . poll_subprocess ( );
        return  self . rpcclt;
        pub fn restart_subprocess ( &self, with_cwd = false , filename = "" )  {
        if self . restarting {
        return  self . rpcclt;
        self . restarting = true;
        debug = self . getdebugger ( );
        if debug {
        // try {
        debugger_r . close_subprocess_debugger ( self . rpcclt );
        // } catch   {
        // pass
        self . rpcclt . close ( );
        self . terminate_subprocess ( );
        console = self . tkconsole;
        was_executing = console . executing;
        console . executing = false;
        self . spawn_subprocess ( );
        // try {
        self . rpcclt . accept ( );
        // } catch  TimeoutError  {
        self . display_no_subprocess_error ( );
        return;
        self . transfer_path ( with_cwd = with_cwd );
        console . stop_readline ( );
        console . text . delete ( "iomark" , "end-1c" );
        console . write ( "\n" );
        console . write ( restart_line ( console . width , filename ) );
        console . text . mark_set ( "restart" , "end-1c" );
        console . text . mark_gravity ( "restart" , "left" );
        if !filename {
        console . showprompt ( );
        if debug {
        debugger_r . restart_subprocess_debugger ( self . rpcclt );
        debug . load_breakpoints ( );
        self . compile . compiler . flags = self . original_compiler_flags;
        self . restarting = false;
        return  self . rpcclt;
        pub fn __request_interrupt ( self )  {
        self . rpcclt . remotecall ( "exec" , "interrupt_the_server" , ( ) , { } );
        pub fn interrupt_subprocess ( self )  {
        threading . Thread ( target = self . __request_interrupt ) . start ( );
        pub fn kill_subprocess ( self )  {
        if self . _afterid is !None /* Option */ {
        self . tkconsole . text . after_cancel ( self . _afterid );
        // try {
        self . rpcclt . listening_sock . close ( );
        // } catch  AttributeError  {
        // pass
        // try {
        self . rpcclt . close ( );
        // } catch  AttributeError  {
        // pass
        self . terminate_subprocess ( );
        self . tkconsole . executing = false;
        self . rpcclt = None /* Option */;
        pub fn terminate_subprocess ( self )  {
        "Make sure subprocess == terminated";
        // try {
        self . rpcsubproc . kill ( );
        // } catch  OSError  {
        return;
        } else {
        // try {
        self . rpcsubproc . wait ( );
        // } catch  OSError  {
        return;
        pub fn transfer_path ( &self, with_cwd = false )  {
        if with_cwd {
        path = [ "" ];
        path . extend ( sys . path );
        } else {
        path = sys . path;
        self . runcommand ( "if 1:
        import sys as _sys
        _sys.path = {!r}
        del _sys
        \n" . format ( path ) );
        active_seq = None /* Option */;
        pub fn poll_subprocess ( self )  {
        clt = self . rpcclt;
        if clt is None /* Option */ {
        return;
        // try {
        response = clt . pollresponse ( self . active_seq , wait = 0.05 );
        // } catch  ( EOFError , OSError , KeyboardInterrupt )  {
        if self . tkconsole . closing {
        return;
        response = None /* Option */;
        self . restart_subprocess ( );
        if response {
        self . tkconsole . resetoutput ( );
        self . active_seq = None /* Option */;
        how , what = response;
        console = self . tkconsole . console;
        if how == "OK" {
        if what is !None /* Option */ {
        println!( repr ( what ) , file = console );
        } else if how == "EXCEPTION" {
        if self . tkconsole . getvar ( "<<toggle-jit-stack-viewer>>" ) {
        self . remote_stack_viewer ( );
        } else if how == "ERROR" {
        errmsg = "pyshell.ModifiedInterpreter: Subprocess ERROR:\n";
        println!( errmsg , what , file = sys . __stderr__ );
        println!( errmsg , what , file = console );
        // try {
        self . tkconsole . endexecuting ( );
        // } catch  AttributeError  {
        // pass
        if !self . tkconsole . closing {
        self . _afterid = self . tkconsole . text . after (;
        self . tkconsole . pollinterval , self . poll_subprocess );
        debugger = None /* Option */;
        pub fn setdebugger ( &self, debugger )  {
        self . debugger = debugger;
        pub fn getdebugger ( self )  {
        return  self . debugger;
        pub fn open_remote_stack_viewer ( self )  {
        "Initiate the remote stack viewer from a separate thread.

        This method == called from the subprocess, && by returning from this
        method we allow the subprocess to unblock.  After a bit the shell
        requests the subprocess to open the remote stack viewer which returns a
        static object looking at the last exception.  It == queried through
        the RPC mechanism.

        ";
        self . tkconsole . text . after ( 300 , self . remote_stack_viewer );
        return;
        pub fn remote_stack_viewer ( self )  {
        from idlelib import debugobj_r;
        oid = self . rpcclt . remotequeue ( "exec" , "stackviewer" , ( "flist" , ) , { } );
        if oid is None /* Option */ {
        self . tkconsole . root . bell ( );
        return;
        item = debugobj_r . StubObjectTreeItem ( self . rpcclt , oid );
        from idlelib . tree import ScrolledCanvas , TreeNode;
        top = Toplevel ( self . tkconsole . root );
        theme = idleConf . CurrentTheme ( );
        background = idleConf . GetHighlight ( theme , "normal" ) [ "background" ];
        sc = ScrolledCanvas ( top , bg = background , highlightthickness = 0 );
        sc . frame . pack ( expand = 1 , fill = "both" );
        node = TreeNode ( sc . canvas , None /* Option */ , item );
        node . expand ( );
        gid = 0;
        pub fn execsource ( &self, source )  {
        "Like runsource() but assumes complete exec source";
        filename = self . stuffsource ( source );
        self . execfile ( filename , source );
        pub fn execfile ( &self, filename , source = None /* Option */ )  {
        "Execute an existing file";
        if source is None /* Option */ {
        // with scope: tokenize . open ( filename ) as fp  {
        source = fp . read ( );
        if use_subprocess {
        source = ( format!("__file__ = r'''{os.path.abspath(filename)}'''\n");
        + source + "\ndel __file__" );
        // try {
        code = compile ( source , filename , "exec" );
        // } catch  ( OverflowError , SyntaxError )  {
        self . tkconsole . resetoutput ( );
        println!( "*** Error in script || command!\n);
        "Traceback (most recent call last):" ,;
        file = self . tkconsole . stderr );
        InteractiveInterpreter . showsyntaxerror ( self , filename );
        self . tkconsole . showprompt ( );
        } else {
        self . runcode ( code );
        pub fn runsource ( &self, source )  {
        "Extend base class method: Stuff the source in the line cache first";
        filename = self . stuffsource ( source );
        assert isinstance ( source , str );
        return  InteractiveInterpreter . runsource ( self , source , filename );
        pub fn stuffsource ( &self, source )  {
        "Stuff source in the filename cache";
        filename = "<pyshell#%d>" % self . gid;
        self . gid = self . gid + 1;
        lines = source . split ( "\n" );
        linecache . cache [ filename ] = len ( source ) + 1 , 0 , lines , filename;
        return  filename;
        pub fn prepend_syspath ( &self, filename )  {
        "Prepend sys.path with file's directory if !already included";
        self . runcommand ( "if 1:
            _filename = {!r}
            import sys as _sys
            from os.path import dirname as _dirname
            _dir = _dirname(_filename)
            if !_dir in _sys.path:
                _sys.path.insert(0, _dir)
            del _filename, _sys, _dirname, _dir
            \n" . format ( filename ) );
        pub fn showsyntaxerror ( &self, filename = None /* Option */ )  {
        "Override Interactive Interpreter method: Use Colorizing

        Color the offending position instead of printing it && pointing at it
        with a caret.

        ";
        tkconsole = self . tkconsole;
        text = tkconsole . text;
        text . tag_remove ( "ERROR" , "1.0" , "end" );
        type , value , tb = sys . exc_info ( );
        msg = getattr ( value , "msg" , "" ) || value || "<no detail available>";
        lineno = getattr ( value , "lineno" , "" ) || 1;
        offset = getattr ( value , "offset" , "" ) || 0;
        if offset == 0 {
        lineno + = 1;
        if lineno == 1 {
        pos = "iomark + %d chars" % ( offset -1 );
        } else {
        pos = "iomark linestart + %d lines + %d chars" % \;
        ( lineno -1 , offset -1 );
        tkconsole . colorize_syntax_error ( text , pos );
        tkconsole . resetoutput ( );
        self . write ( "SyntaxError: %s\n" % msg );
        tkconsole . showprompt ( );
        pub fn showtraceback ( self )  {
        "Extend base class method to reset output properly";
        self . tkconsole . resetoutput ( );
        self . checklinecache ( );
        InteractiveInterpreter . showtraceback ( self );
        if self . tkconsole . getvar ( "<<toggle-jit-stack-viewer>>" ) {
        self . tkconsole . open_stack_viewer ( );
        pub fn checklinecache ( self )  {
        "Remove keys other than '<pyshell#n>'.";
        cache = linecache . cache;
        for key in list ( cache ) .iter() {
        if key [ { : 1 ] + key [ -1 : ] != "<>" ; }
        del cache [ key ];
        pub fn runcommand ( &self, code )  {
        "Run the code without invoking the debugger";
        if self . tkconsole . executing {
        self . display_executing_dialog ( );
        return  0;
        if self . rpcclt {
        self . rpcclt . remotequeue ( "exec" , "runcode" , ( code , ) , { } );
        } else {
        exec ( code , self . locals );
        return  1;
        pub fn runcode ( &self, code )  {
        "Override base class method";
        if self . tkconsole . executing {
        self . restart_subprocess ( );
        self . checklinecache ( );
        debugger = self . debugger;
        // try {
        self . tkconsole . beginexecuting ( );
        if !debugger && self . rpcclt is !None /* Option */ {
        self . active_seq = self . rpcclt . asyncqueue ( "exec" , "runcode" ,;
        ( code , ) , { } );
        } else if debugger {
        debugger . run ( code , self . locals );
        } else {
        exec ( code , self . locals );
        // } catch  SystemExit  {
        if !self . tkconsole . closing {
        if messagebox . askyesno ( {
        "Exit?" ,;
        "Do you want to exit altogether?" ,;
        default = "yes" ,;
        parent = self . tkconsole . text ) ;
        panic!("");
        } else {
        self . showtraceback ( );
        } else {
        panic!("");
        // } catch   {
        if use_subprocess {
        println!( "IDLE internal error in runcode()" );
        file = self . tkconsole . stderr );
        self . showtraceback ( );
        self . tkconsole . endexecuting ( );
        } else {
        if self . tkconsole . canceled {
        self . tkconsole . canceled = false;
        println!( "KeyboardInterrupt" , file = self . tkconsole . stderr );
        } else {
        self . showtraceback ( );
        // } finally {
        if !use_subprocess {
        // try {
        self . tkconsole . endexecuting ( );
        // } catch  AttributeError  {
        // pass
        pub fn write ( &self, s )  {
        "Override base class method";
        return  self . tkconsole . stderr . write ( s );
        pub fn display_port_binding_error ( self )  {
        messagebox . showerror (;
        "Port Binding Error" ,;
        "IDLE can't bind to a TCP/IP port, which == necessary to ";
        "communicate with its Python execution server.  This might be ";
        "because no networking == installed on this computer.  ";
        "Run IDLE with the -n command line switch to start without a ";
        "subprocess && refer to Help/IDLE Help 'Running without a ";
        "subprocess' for further details." ,;
        parent = self . tkconsole . text );
        pub fn display_no_subprocess_error ( self )  {
        messagebox . showerror (;
        "Subprocess Connection Error" ,;
        "IDLE's subprocess didn't make connection.\n";
        "See the 'Startup failure' section of the IDLE doc, online at\n";
        "https://docs.python.org/3/library/idle.html#startup-failure" ,;
        parent = self . tkconsole . text );
        pub fn display_executing_dialog ( self )  {
        messagebox . showerror (;
        "Already executing" ,;
        "The Python Shell window == already executing a command; ";
        "please wait until it == finished." ,;
        parent = self . tkconsole . text );
        class PyShell ( OutputWindow ) ;
        from idlelib . squeezer import Squeezer;
        shell_title = "IDLE Shell " + python_version ( );
        ColorDelegator = ModifiedColorDelegator;
        UndoDelegator = ModifiedUndoDelegator;
        menu_specs = [;
        ( "file" , "_File" ) ,;
        ( "edit" , "_Edit" ) ,;
        ( "debug" , "_Debug" ) ,;
        ( "options" , "_Options" ) ,;
        ( "window" , "_Window" ) ,;
        ( "help" , "_Help" ) ,;
        ];
        rmenu_specs = OutputWindow . rmenu_specs + [;
        ( "Squeeze" , "<<squeeze-current-text>>" ) ,;
        ];
        _idx = 1 + len ( list ( itertools . takewhile (;
        |rmenu_item | {  rmenu_item [ 0 ] != "Copy" , rmenu_specs ) };
        ) );
        rmenu_specs . insert ( _idx , ( "Copy with prompts" ,;
        "<<copy-with-prompts>>" ,;
        "rmenu_check_copy" ) );
        del _idx;
        allow_line_numbers = false;
        user_input_insert_tags = "stdin";
        from idlelib . history import History;
        from idlelib . sidebar import ShellSidebar;
        pub fn __init__ ( &self, flist = None /* Option */ )  {
        if use_subprocess {
        ms = self . menu_specs;
        if ms [ 2 ] [ 0 ] != "shell" {
        ms . insert ( 2 , ( "shell" , "She_ll" ) );
        self . interp = ModifiedInterpreter ( self );
        if flist is None /* Option */ {
        root = Tk ( );
        fixwordbreaks ( root );
        root . withdraw ( );
        flist = PyShellFileList ( root );
        self . shell_sidebar = None /* Option */;
        OutputWindow . __init__ ( self , flist , None /* Option */ , None /* Option */ );
        self . usetabs = false;
        self . indentwidth = 4;
        self . sys_ps1 = sys . ps1 if hasattr ( sys , "ps1" ) else ">>>\n";
        self . prompt_last_line = self . sys_ps1 . split ( "\n" ) [ -1 ];
        self . prompt = self . sys_ps1;
        text = self . text;
        text . configure ( wrap = "char" );
        text . bind ( "<<newline-and-indent>>" , self . enter_callback );
        text . bind ( "<<plain-newline-and-indent>>" , self . linefeed_callback );
        text . bind ( "<<interrupt-execution>>" , self . cancel_callback );
        text . bind ( "<<end-of-file>>" , self . eof_callback );
        text . bind ( "<<open-stack-viewer>>" , self . open_stack_viewer );
        text . bind ( "<<toggle-debugger>>" , self . toggle_debugger );
        text . bind ( "<<toggle-jit-stack-viewer>>" , self . toggle_jit_stack_viewer );
        text . bind ( "<<copy-with-prompts>>" , self . copy_with_prompts_callback );
        if use_subprocess {
        text . bind ( "<<view-restart>>" , self . view_restart_mark );
        text . bind ( "<<restart-shell>>" , self . restart_shell );
        self . squeezer = self . Squeezer ( self );
        text . bind ( "<<squeeze-current-text>>" ,;
        self . squeeze_current_text_event );
        self . save_stdout = sys . stdout;
        self . save_stderr = sys . stderr;
        self . save_stdin = sys . stdin;
        from idlelib import iomenu;
        self . stdin = StdInputFile ( self , "stdin" ,;
        iomenu . encoding , iomenu . errors );
        self . stdout = StdOutputFile ( self , "stdout" ,;
        iomenu . encoding , iomenu . errors );
        self . stderr = StdOutputFile ( self , "stderr" ,;
        iomenu . encoding , "backslashreplace" );
        self . console = StdOutputFile ( self , "console" ,;
        iomenu . encoding , iomenu . errors );
        if !use_subprocess {
        sys . stdout = self . stdout;
        sys . stderr = self . stderr;
        sys . stdin = self . stdin;
        // try {
        import pydoc;
        pydoc . pager = pydoc . plainpager;
        // } catch   {
        sys . stderr = sys . __stderr__;
        panic!("");
        self . history = self . History ( self . text );
        self . pollinterval = 50;
        self . shell_sidebar = self . ShellSidebar ( self );
        self . text . insert = self . per . top . insert;
        self . per . insertfilter ( UserInputTaggingDelegator ( ) );
        pub fn ResetFont ( self )  {
        super ( ) . ResetFont ( );
        if self . shell_sidebar is !None /* Option */ {
        self . shell_sidebar . update_font ( );
        pub fn ResetColorizer ( self )  {
        super ( ) . ResetColorizer ( );
        theme = idleConf . CurrentTheme ( );
        tag_colors = {;
        "stdin" : { "background" : None /* Option */ , "foreground" : None /* Option */ } ,;
        "stdout" : idleConf . GetHighlight ( theme , "stdout" ) ,;
        "stderr" : idleConf . GetHighlight ( theme , "stderr" ) ,;
        "console" : idleConf . GetHighlight ( theme , "normal" ) ,;
        };
        for tag , tag_colors_config in tag_colors . items ( ) .iter() {
        self . text . tag_configure ( tag , ** tag_colors_config );
        if self . shell_sidebar is !None /* Option */ {
        self . shell_sidebar . update_colors ( );
        pub fn replace_event ( &self, event )  {
        replace . replace ( self . text , insert_tags = "stdin" );
        return  "break";
        pub fn get_standard_extension_names ( self )  {
        return  idleConf . GetExtensions ( shell_only = true );
        pub fn get_prompt_text ( &self, first , last )  {
        "Return text between first && last with prompts added.";
        text = self . text . get ( first , last );
        lineno_range = range (;
        int ( float ( first ) ) ,;
        int ( float ( last ) );
        );
        prompts = [;
        self . shell_sidebar . line_prompts . get ( lineno );
        for lineno in lineno_range.iter() {
        ];
        return  "\n" . join (;
        line if prompt == None /* Option */ else format!("{prompt} {line}");
        for prompt , line in zip ( prompts , text . splitlines ( ) ).iter() {
        ) + "\n";
        pub fn copy_with_prompts_callback ( &self, event = None /* Option */ )  {
        "Copy selected lines to the clipboard, with prompts.

        This makes the copied text useful for doc-tests && interactive
        shell code examples.

        This always copies entire lines, even if only part of the first
        and/or last lines == selected.
        ";
        text = self . text;
        selfirst = text . index ( "sel.first linestart" );
        if selfirst is None /* Option */ {
        return;
        sellast = text . index ( "sel.last" );
        if sellast [ -1 ] != "0" {
        sellast = text . index ( "sel.last+1line linestart" );
        text . clipboard_clear ( );
        prompt_text = self . get_prompt_text ( selfirst , sellast );
        text . clipboard_append ( prompt_text );
        reading = false;
        executing = false;
        canceled = false;
        endoffile = false;
        closing = false;
        _stop_readline_flag = false;
        pub fn set_warning_stream ( &self, stream )  {
        global warning_stream;
        warning_stream = stream;
        pub fn get_warning_stream ( self )  {
        return  warning_stream;
        pub fn toggle_debugger ( &self, event = None /* Option */ )  {
        if self . executing {
        messagebox . showerror ( "Don't debug now" ,;
        "You can only toggle the debugger when idle" ,;
        parent = self . text );
        self . set_debugger_indicator ( );
        return  "break";
        } else {
        db = self . interp . getdebugger ( );
        if db {
        self . close_debugger ( );
        } else {
        self . open_debugger ( );
        pub fn set_debugger_indicator ( self )  {
        db = self . interp . getdebugger ( );
        self . setvar ( "<<toggle-debugger>>" , !not db );
        pub fn toggle_jit_stack_viewer ( &self, event = None /* Option */ )  {
        // pass
        pub fn close_debugger ( self )  {
        db = self . interp . getdebugger ( );
        if db {
        self . interp . setdebugger ( None /* Option */ );
        db . close ( );
        if self . interp . rpcclt {
        debugger_r . close_remote_debugger ( self . interp . rpcclt );
        self . resetoutput ( );
        self . console . write ( "[DEBUG OFF]\n" );
        self . prompt = self . sys_ps1;
        self . showprompt ( );
        self . set_debugger_indicator ( );
        pub fn open_debugger ( self )  {
        if self . interp . rpcclt {
        dbg_gui = debugger_r . start_remote_debugger ( self . interp . rpcclt ,;
        self );
        } else {
        dbg_gui = debugger . Debugger ( self );
        self . interp . setdebugger ( dbg_gui );
        dbg_gui . load_breakpoints ( );
        self . prompt = "[DEBUG ON]\n" + self . sys_ps1;
        self . showprompt ( );
        self . set_debugger_indicator ( );
        pub fn debug_menu_postcommand ( self )  {
        state = "disabled" if self . executing else "normal";
        self . update_menu_state ( "debug" , "*tack*iewer" , state );
        pub fn beginexecuting ( self )  {
        "Helper for ModifiedInterpreter";
        self . resetoutput ( );
        self . executing = true;
        pub fn endexecuting ( self )  {
        "Helper for ModifiedInterpreter";
        self . executing = false;
        self . canceled = false;
        self . showprompt ( );
        pub fn close ( self )  {
        "Extend EditorWindow.close()";
        if self . executing {
        response = messagebox . askokcancel (;
        "Kill?" ,;
        "Your program == still running!\n Do you want to kill it?" ,;
        default = "ok" ,;
        parent = self . text );
        if response is false {
        return  "cancel";
        self . stop_readline ( );
        self . canceled = true;
        self . closing = true;
        return  EditorWindow . close ( self );
        pub fn _close ( self )  {
        "Extend EditorWindow._close(), shut down debugger && execution server";
        self . close_debugger ( );
        if use_subprocess {
        self . interp . kill_subprocess ( );
        sys . stdout = self . save_stdout;
        sys . stderr = self . save_stderr;
        sys . stdin = self . save_stdin;
        self . interp = None /* Option */;
        self . console = None /* Option */;
        self . flist . pyshell = None /* Option */;
        self . history = None /* Option */;
        EditorWindow . _close ( self );
        pub fn ispythonsource ( &self, filename )  {
        "Override EditorWindow method: never remove the colorizer";
        return  true;
        pub fn short_title ( self )  {
        return  self . shell_title;
        COPYRIGHT = \;
        "Type "help", "copyright", "credits" || "license()" for more information.";
        pub fn begin ( self )  {
        self . text . mark_set ( "iomark" , "insert" );
        self . resetoutput ( );
        if use_subprocess {
        nosub = "";
        client = self . interp . start_subprocess ( );
        if !client {
        self . close ( );
        return  false;
        } else {
        nosub = ( "==== No Subprocess ====\n\n" +;
        "WARNING: Running IDLE without a Subprocess == deprecated\n" +;
        "and will be removed in a later version. See Help/IDLE Help\n" +;
        "for details.\n\n" );
        sys . displayhook = rpc . displayhook;
        self . write ( "Python %s on %s\n%s\n%s" %;
        ( sys . version , sys . platform , self . COPYRIGHT , nosub ) );
        self . text . focus_force ( );
        self . showprompt ( );
        import tkinter;
        tkinter . _support_default_root = true;
        tkinter . _default_root = None /* Option */;
        return  true;
        pub fn stop_readline ( self )  {
        if !self . reading {
        return;
        self . _stop_readline_flag = true;
        self . top . quit ( );
        pub fn readline ( self )  {
        save = self . reading;
        // try {
        self . reading = true;
        self . top . mainloop ( );
        // } finally {
        self . reading = save;
        if self . _stop_readline_flag {
        self . _stop_readline_flag = false;
        return  "";
        line = self . text . get ( "iomark" , "end-1c" );
        if len ( line ) == 0 {
        line = "\n";
        self . resetoutput ( );
        if self . canceled {
        self . canceled = false;
        if !use_subprocess {
        panic!("KeyboardInterrupt");
        if self . endoffile {
        self . endoffile = false;
        line = "";
        return  line;
        pub fn isatty ( self )  {
        return  true;
        pub fn cancel_callback ( &self, event = None /* Option */ )  {
        // try {
        if self . text . compare ( "sel.first" , "!=" , "sel.last" ) {
        return;
        // } catch   {
        // pass
        if !( self . executing || self . reading ) {
        self . resetoutput ( );
        self . interp . write ( "KeyboardInterrupt\n" );
        self . showprompt ( );
        return  "break";
        self . endoffile = false;
        self . canceled = true;
        if ( self . executing && self . interp . rpcclt ) {
        if self . interp . getdebugger ( ) {
        self . interp . restart_subprocess ( );
        } else {
        self . interp . interrupt_subprocess ( );
        if self . reading {
        self . top . quit ( );
        return  "break";
        pub fn eof_callback ( &self, event )  {
        if self . executing && !self . reading {
        return;
        if !( self . text . compare ( "iomark" , "==" , "insert" ) and {
        self . text . compare ( "insert" , "==" , "end-1c" ) ) :;
        return;
        if !self . executing {
        self . resetoutput ( );
        self . close ( );
        } else {
        self . canceled = false;
        self . endoffile = true;
        self . top . quit ( );
        return  "break";
        pub fn linefeed_callback ( &self, event )  {
        if self . reading {
        self . text . insert ( "insert" , "\n" );
        self . text . see ( "insert" );
        } else {
        self . newline_and_indent_event ( event );
        return  "break";
        pub fn enter_callback ( &self, event )  {
        if self . executing && !self . reading {
        return;
        // try {
        sel = self . text . get ( "sel.first" , "sel.last" );
        if sel {
        if self . text . compare ( "sel.last" , "<=" , "iomark" ) {
        self . recall ( sel , event );
        return  "break";
        // } catch   {
        // pass
        if self . text . compare ( "insert" , "<" , "iomark linestart" ) {
        prev = self . text . tag_prevrange ( "stdin" , "insert" );
        if ( {
        prev and;
        self . text . compare ( "insert" , "<" , prev [ 1 ] ) and;
        "console" !in self . text . tag_names ( "insert" );
        ) ;
        prev_cons = self . text . tag_prevrange ( "console" , "insert" );
        if prev_cons && self . text . compare ( prev_cons [ 1 ] , ">=" , prev [ 0 ] ) {
        prev = ( prev_cons [ 1 ] , prev [ 1 ] );
        next_cons = self . text . tag_nextrange ( "console" , "insert" );
        if next_cons && self . text . compare ( next_cons [ 0 ] , "<" , prev [ 1 ] ) {
        prev = ( prev [ 0 ] , self . text . index ( next_cons [ 0 ] + "+1c" ) );
        self . recall ( self . text . get ( prev [ 0 ] , prev [ 1 ] ) , event );
        return  "break";
        next = self . text . tag_nextrange ( "stdin" , "insert" );
        if next && self . text . compare ( "insert lineend" , ">=" , next [ 0 ] ) {
        next_cons = self . text . tag_nextrange ( "console" , "insert lineend" );
        if next_cons && self . text . compare ( next_cons [ 0 ] , "<" , next [ 1 ] ) {
        next = ( next [ 0 ] , self . text . index ( next_cons [ 0 ] + "+1c" ) );
        self . recall ( self . text . get ( next [ 0 ] , next [ 1 ] ) , event );
        return  "break";
        indices = self . text . tag_nextrange ( "console" , "insert linestart" );
        if indices && \ {
        self . text . compare ( indices [ 0 ] , "<=" , "insert linestart" ) :;
        self . recall ( self . text . get ( indices [ 1 ] , "insert lineend" ) , event );
        } else {
        self . recall ( self . text . get ( "insert linestart" , "insert lineend" ) , event );
        return  "break";
        if self . text . compare ( "insert" , "<" , "iomark" ) {
        self . text . mark_set ( "insert" , "iomark" );
        s = self . text . get ( "insert" , "end-1c" );
        if s && !s . strip ( ) {
        self . text . delete ( "insert" , "end-1c" );
        if self . text . compare ( "insert" , "<" , "end-1c linestart" ) {
        self . newline_and_indent_event ( event );
        return  "break";
        self . text . mark_set ( "insert" , "end-1c" );
        if self . reading {
        self . text . insert ( "insert" , "\n" );
        self . text . see ( "insert" );
        } else {
        self . newline_and_indent_event ( event );
        self . text . update_idletasks ( );
        if self . reading {
        self . top . quit ( );
        } else {
        self . runit ( );
        return  "break";
        pub fn recall ( &self, s , event )  {
        s = re . sub ( r "^\s*\n" , "" , s );
        s = re . sub ( r "\n\s*$" , "" , s );
        lines = s . split ( "\n" );
        self . text . undo_block_start ( );
        // try {
        self . text . tag_remove ( "sel" , "1.0" , "end" );
        self . text . mark_set ( "insert" , "end-1c" );
        prefix = self . text . get ( "insert linestart" , "insert" );
        if prefix . rstrip ( ) . endswith ( ":" ) {
        self . newline_and_indent_event ( event );
        prefix = self . text . get ( "insert linestart" , "insert" );
        self . text . insert ( "insert" , lines [ 0 ] . strip ( ) ,;
        self . user_input_insert_tags );
        if len ( lines ) > 1 {
        orig_base_indent = re . search ( r "^([ \t]*)" , lines [ 0 ] ) . group ( 0 );
        new_base_indent = re . search ( r "^([ \t]*)" , prefix ) . group ( 0 );
        for line in lines [ 1 : ] .iter() {
        if line . startswith ( orig_base_indent ) {
        line = new_base_indent + line [ len ( orig_base_indent ) : ];
        self . text . insert ( "insert" , "\n" + line . rstrip ( ) ,;
        self . user_input_insert_tags );
        // } finally {
        self . text . see ( "insert" );
        self . text . undo_block_stop ( );
        _last_newline_re = re . compile ( r "[ \t]*(\n[ \t]*)?\Z" );
        pub fn runit ( self )  {
        index_before = self . text . index ( "end-2c" );
        line = self . text . get ( "iomark" , "end-1c" );
        line = self . _last_newline_re . sub ( "" , line );
        input_is_complete = self . interp . runsource ( line );
        if !input_is_complete {
        if self . text . get ( index_before ) == "\n" {
        self . text . tag_remove ( self . user_input_insert_tags , index_before );
        self . shell_sidebar . update_sidebar ( );
        pub fn open_stack_viewer ( &self, event = None /* Option */ )  {
        if self . interp . rpcclt {
        return  self . interp . remote_stack_viewer ( );
        from idlelib . stackviewer import StackBrowser;
        // try {
        StackBrowser ( self . root , sys . last_value , self . flist );
        // } catch   {
        messagebox . showerror ( "No stack trace" ,;
        "There == no stack trace yet.\n";
        "(sys.last_value == !defined)" ,;
        parent = self . text );
        return;
        pub fn view_restart_mark ( &self, event = None /* Option */ )  {
        self . text . see ( "iomark" );
        self . text . see ( "restart" );
        pub fn restart_shell ( &self, event = None /* Option */ )  {
        "Callback for Run/Restart Shell Cntl-F6";
        self . interp . restart_subprocess ( with_cwd = true );
        pub fn showprompt ( self )  {
        self . resetoutput ( );
        prompt = self . prompt;
        if self . sys_ps1 && prompt . endswith ( self . sys_ps1 ) {
        prompt = prompt [ : - len ( self . sys_ps1 ) ];
        self . text . tag_add ( "console" , "iomark-1c" );
        self . console . write ( prompt );
        self . shell_sidebar . update_sidebar ( );
        self . text . mark_set ( "insert" , "end-1c" );
        self . set_line_and_column ( );
        self . io . reset_undo ( );
        pub fn show_warning ( &self, msg )  {
        width = self . interp . tkconsole . width;
        wrapper = TextWrapper ( width = width , tabsize = 8 , expand_tabs = true );
        wrapped_msg = "\n" . join ( wrapper . wrap ( msg ) );
        if !wrapped_msg . endswith ( "\n" ) {
        wrapped_msg + = "\n";
        self . per . bottom . insert ( "iomark linestart" , wrapped_msg , "stderr" );
        pub fn resetoutput ( self )  {
        source = self . text . get ( "iomark" , "end-1c" );
        if self . history {
        self . history . store ( source );
        if self . text . get ( "end-2c" ) != "\n" {
        self . text . insert ( "end-1c" , "\n" );
        self . text . mark_set ( "iomark" , "end-1c" );
        self . set_line_and_column ( );
        self . ctip . remove_calltip_window ( );
        pub fn write ( &self, s , tags = ( ) )  {
        // try {
        self . text . mark_gravity ( "iomark" , "right" );
        count = OutputWindow . write ( self , s , tags , "iomark" );
        self . text . mark_gravity ( "iomark" , "left" );
        // } catch   {
        panic!("");
        if self . canceled {
        self . canceled = false;
        if !use_subprocess {
        panic!("KeyboardInterrupt");
        return  count;
        pub fn rmenu_check_cut ( self )  {
        // try {
        if self . text . compare ( "sel.first" , "<" , "iomark" ) {
        return  "disabled";
        // } catch  TclError  {
        return  "disabled";
        return  super ( ) . rmenu_check_cut ( );
        pub fn rmenu_check_paste ( self )  {
        if self . text . compare ( "insert" , "<" , "iomark" ) {
        return  "disabled";
        return  super ( ) . rmenu_check_paste ( );
        pub fn squeeze_current_text_event ( &self, event = None /* Option */ )  {
        self . squeezer . squeeze_current_text ( );
        self . shell_sidebar . update_sidebar ( );
        pub fn on_squeezed_expand ( &self, index , text , tags )  {
        self . shell_sidebar . update_sidebar ( );
        pub fn fix_x11_paste ( root )  {
        "Make paste replace selection on x11.  See issue #5124.";
        if root . _windowingsystem == "x11" {
        for cls in "Text" , "Entry" , "Spinbox" .iter() {
        root . bind_class (;
        cls ,;
        "<<Paste>>" ,;
        "catch {%W delete sel.first sel.last}\n" +;
        root . bind_class ( cls , "<<Paste>>" ) );
        usage_msg = "\

USAGE: idle  vec![-deins] vec![-t title] vec![file]*
       idle  vec![-dns] vec![-t title] (-c cmd | -r file) vec![arg]*
       idle  vec![-dns] vec![-t title] - vec![arg]*

  -h         print this help message && exit
  -n         run IDLE without a subprocess (DEPRECATED,
             see Help/IDLE Help.iter().map(|details)

The following options will override the IDLE 'settings' configuration:

  -e         open an edit window
  -i         open a shell window

The following options imply -i && will open a shell:

  -c cmd     run the command| a shell, or
  -r file    run script from file

  -d         enable the debugger
  -s         run $IDLESTARTUP || $PYTHONSTARTUP before anything else
  -t title   set title of shell window

A default edit window will be bypassed when -c, -r, || - are used.

vec![arg]* are passed to the command (-c) || script (-r)| sys.argvvec![1:].

Examples:

idle
        Open an edit window || shell depending on IDLE's configuration.

idle foo.py foobar.py
        Edit the files, also open a shell if configured to start with shell.

idle -est "Baz" foo.py
        Run $IDLESTARTUP || $PYTHONSTARTUP, edit foo.py, && open a shell
        window with the title "Baz".

idle -c "import sys; print(sys.argv)" "foo"
        Open a shell window && run the command, passing "-c"| sys.argvvec![0]
        && "foo"| sys.argvvec![1].

idle -d -s -r foo.py "Hello World"
        Open a shell window, run a startup script, enable the debugger, and
        run foo.py, passing "foo.py"| sys.argvvec![0] && "Hello World" in
        sys.argvvec![1].

echo "import sys; print(sys.argv)" | idle - "foobar"
        Open a shell window, run the script piped in, passing ''| sys.argvvec![0]
        && "foobar"| sys.argvvec![1].
";
        pub fn main ( )  {
        import getopt;
        from platform import system;
        from idlelib import testing;
        from idlelib import macosx;
        global flist , root , use_subprocess;
        capture_warnings ( true );
        use_subprocess = true;
        enable_shell = false;
        enable_edit = false;
        debug = false;
        cmd = None /* Option */;
        script = None /* Option */;
        startup = false;
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "c:deihnr:st:" );
        // } catch  getopt . error as msg  {
        println!( f "Error: {msg}\n{usage_msg}" , file = sys . stderr );
        sys . exit ( 2 );
        for o , a in opts .iter() {
        if o == "-c" {
        cmd = a;
        enable_shell = true;
        if o == "-d" {
        debug = true;
        enable_shell = true;
        if o == "-e" {
        enable_edit = true;
        if o == "-h" {
        sys . stdout . write ( usage_msg );
        sys . exit ( );
        if o == "-i" {
        enable_shell = true;
        if o == "-n" {
        println!( " Warning: running IDLE without a subprocess is deprecated." );
        file = sys . stderr );
        use_subprocess = false;
        if o == "-r" {
        script = a;
        if os . path . isfile ( script ) {
        // pass
        } else {
        println!( "No script file: " , script );
        sys . exit ( );
        enable_shell = true;
        if o == "-s" {
        startup = true;
        enable_shell = true;
        if o == "-t" {
        PyShell . shell_title = a;
        enable_shell = true;
        if args && args [ 0 ] == "-" {
        cmd = sys . stdin . read ( );
        enable_shell = true;
        for i in range ( len ( sys . path ) ) .iter() {
        sys . path [ i ] = os . path . abspath ( sys . path [ i ] );
        if args && args [ 0 ] == "-" {
        sys . argv = [ "" ] + args [ 1 : ];
        } else if cmd {
        sys . argv = [ "-c" ] + args;
        } else if script {
        sys . argv = [ script ] + args;
        } else if args {
        enable_edit = true;
        pathx = [ ];
        for filename in args .iter() {
        pathx . append ( os . path . dirname ( filename ) );
        for dir in pathx .iter() {
        dir = os . path . abspath ( dir );
        if !dir in sys . path {
        sys . path . insert ( 0 , dir );
        } else {
        dir = os . getcwd ( );
        if dir !in sys . path {
        sys . path . insert ( 0 , dir );
        edit_start = idleConf . GetOption ( "main" , "General" ,;
        "editor-on-startup" , type = "bool" );
        enable_edit = enable_edit || edit_start;
        enable_shell = enable_shell || !enable_edit;
        if use_subprocess && !testing {
        NoDefaultRoot ( );
        root = Tk ( className = "Idle" );
        root . withdraw ( );
        from idlelib . run import fix_scaling;
        fix_scaling ( root );
        icondir = os . path . join ( os . path . dirname ( __file__ ) , "Icons" );
        if system ( ) == "Windows" {
        iconfile = os . path . join ( icondir , "idle.ico" );
        root . wm_iconbitmap ( default = iconfile );
        } else if !macosx . isAquaTk ( ) {
        if TkVersion >= 8.6 {
        ext = ".png";
        sizes = ( 16 , 32 , 48 , 256 );
        } else {
        ext = ".giformat!(");
        sizes = ( 16 , 32 , 48 );
        iconfiles = [ os . path . join ( icondir , "idle_%d%s" % ( size , ext ) );
        for size in sizes ].iter() {
        icons = [ PhotoImage ( master = root , file = iconfile );
        for iconfile in iconfiles ].iter() {
        root . wm_iconphoto ( true , * icons );
        fixwordbreaks ( root );
        fix_x11_paste ( root );
        flist = PyShellFileList ( root );
        macosx . setupApp ( root , flist );
        if enable_edit {
        if !( cmd || script ) {
        for filename in args [ : ] .iter() {
        if flist . open ( filename ) is None /* Option */ {
        args . remove ( filename );
        if !args {
        flist . new ( );
        if enable_shell {
        shell = flist . open_shell ( );
        if !shell {
        return;
        if macosx . isAquaTk ( ) && flist . dict {
        shell . top . lower ( );
        } else {
        shell = flist . pyshell;
        if debug {
        shell . open_debugger ( );
        if startup {
        filename = os . environ . get ( "IDLESTARTUP" ) || \;
        os . environ . get ( "PYTHONSTARTUP" );
        if filename && os . path . isfile ( filename ) {
        shell . interp . execfile ( filename );
        if cmd || script {
        shell . interp . runcommand ( "if 1:
            import sys as _sys
            _sys.argv = {!r}
            del _sys
            \n" . format ( sys . argv ) );
        if cmd {
        shell . interp . execsource ( cmd );
        } else if script {
        shell . interp . prepend_syspath ( script );
        shell . interp . execfile ( script );
        } else if shell {
        prefer_tabs_preference_warning = macosx . preferTabsPreferenceWarning ( );
        if prefer_tabs_preference_warning {
        shell . show_warning ( prefer_tabs_preference_warning );
        while flist . inversedict  {
        root . mainloop ( );
        root . destroy ( );
        capture_warnings ( false );
        fn main() {
        main ( );
        capture_warnings ( false );
}

