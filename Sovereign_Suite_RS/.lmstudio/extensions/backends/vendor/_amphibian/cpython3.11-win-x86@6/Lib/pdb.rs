//! pdb.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use regex::Regex;
// use crate::cmd;
// use crate::dis;
// use crate::glob;
// use crate::signal;
// use crate::tokenize;
// use crate::traceback;
// use /* typing */::{Union};
// use crate::runpy;
// use crate::readline;
// use crate::shlex;
// use crate::__main__;
// use crate::pydoc;
// use crate::getopt;
// use crate::pdb;

pub struct Restart {
    pub use_rawinput: String, // TODO: infer type
    pub prompt: String, // TODO: infer type
    pub aliases: String, // TODO: infer type
    pub displaying: String, // TODO: infer type
    pub mainpyfile: String, // TODO: infer type
    pub _wait_for_mainpyfile: String, // TODO: infer type
    pub tb_lineno: String, // TODO: infer type
    pub allow_kbdint: String, // TODO: infer type
    pub nosigint: String, // TODO: infer type
    pub rcLines: String, // TODO: infer type
    pub commands: String, // TODO: infer type
    pub commands_doprompt: String, // TODO: infer type
    pub commands_silent: String, // TODO: infer type
    pub commands_defining: String, // TODO: infer type
    pub commands_bnum: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub curindex: String, // TODO: infer type
    pub curframe: String, // TODO: infer type
    pub curframe_locals: String, // TODO: infer type
    pub cmdqueue: String, // TODO: infer type
    pub currentbp: String, // TODO: infer type
    pub lastcmd: String, // TODO: infer type
    pub _user_requested_quit: String, // TODO: infer type
}

impl Restart {
}

pub const __all__: &str = ["run" ,"pm" ,"Pdb" ,"runeval" ,"runctx" ,"runcall" ,"set_trace" ,;
pub fn find_function(funcname: &str, filename: &str) {
        cre = re . compile ( r "def\s+%s\s*[(]" % re . escape ( funcname ) );
        // try {
        fp = tokenize . open ( filename );
        // } catch  OSError  {
        return;
        // with scope: fp  {
        for lineno , line in enumerate ( fp , start = 1 ) .iter() {
        if cre . match ( line ) {
        return  funcname , filename , lineno;
        return;
        pub fn lasti2lineno ( code , lasti )  {
        linestarts = list ( dis . findlinestarts ( code ) );
        linestarts . reverse ( );
        for i , lineno in linestarts .iter() {
        if lasti >= i {
        return  lineno;
        return  0;
        class _rstr ( str ) ;
        "String that doesn't quote its repr.";
        pub fn __repr__ ( self )  {
        return  self;
        class _ScriptTarget ( str ) ;
        pub fn __new__ ( cls , val )  {
        res = super ( ) . __new__ ( cls , os . path . realpath ( val ) );
        res . orig = val;
        return  res;
        pub fn check ( self )  {
        if !os . path . exists ( self ) {
        println!( "Error:" , self . orig , "does !exist" );
        sys . exit ( 1 );
        if os . path . isdir ( self ) {
        println!( "Error:" , self . orig , "is a directory" );
        sys . exit ( 1 );
        sys . path [ 0 ] = os . path . dirname ( self );
        @ property;
        pub fn filename ( self )  {
        return  self;
        @ property;
        pub fn namespace ( self )  {
        return  dict (;
        __name__ = "__main__" ,;
        __file__ = self ,;
        __builtins__ = __builtins__ ,;
        __spec__ = None /* Option */ ,;
        );
        @ property;
        pub fn code ( self )  {
        // with scope: io . open_code ( self ) as fp  {
        return  f "exec(compile({fp.read()!r}, {self!r}, 'exec'))";
        class _ModuleTarget ( str ) ;
        pub fn check ( self )  {
        // try {
        self . _details;
        // } catch  ImportError as e  {
        println!( f "ImportError: {e}" );
        sys . exit ( 1 );
        // } catch  Exception  {
        traceback . print_exc ( );
        sys . exit ( 1 );
        @ functools . cached_property;
        pub fn _details ( self )  {
        import runpy;
        return  runpy . _get_module_details ( self );
        @ property;
        pub fn filename ( self )  {
        return  self . code . co_filename;
        @ property;
        pub fn code ( self )  {
        name , spec , code = self . _details;
        return  code;
        @ property;
        pub fn _spec ( self )  {
        name , spec , code = self . _details;
        return  spec;
        @ property;
        pub fn namespace ( self )  {
        return  dict (;
        __name__ = "__main__" ,;
        __file__ = os . path . normcase ( os . path . abspath ( self . filename ) ) ,;
        __package__ = self . _spec . parent ,;
        __loader__ = self . _spec . loader ,;
        __spec__ = self . _spec ,;
        __builtins__ = __builtins__ ,;
        );
        line_prefix = "\n-> ";
        class Pdb ( bdb . Bdb , cmd . Cmd ) ;
        _previous_sigint_handler = None /* Option */;
        pub fn __init__ ( &self, completekey = "tab" , stdin = None /* Option */ , stdout = None /* Option */ , skip = None /* Option */ , {
        nosigint = false , readrc = true ) ;
        bdb . Bdb . __init__ ( self , skip = skip );
        cmd . Cmd . __init__ ( self , completekey , stdin , stdout );
        sys . audit ( "pdb.Pdb" );
        if stdout {
        self . use_rawinput = 0;
        self . prompt = "(Pdb) ";
        self . aliases = { };
        self . displaying = { };
        self . mainpyfile = "";
        self . _wait_for_mainpyfile = false;
        self . tb_lineno = { };
        // try {
        import readline;
        readline . set_completer_delims ( " \t\n`@#$%^&*()=+[{]}\\|;:\'",<>?" );
        // } catch  ImportError  {
        // pass
        self . allow_kbdint = false;
        self . nosigint = nosigint;
        self . rcLines = [ ];
        if readrc {
        // try {
        // with scope: open ( os . path . expanduser ( "~/.pdbrc" ) , encoding = "utf-8" ) as rcFile  {
        self . rcLines . extend ( rcFile );
        // } catch  OSError  {
        // pass
        // try {
        // with scope: open ( ".pdbrc" , encoding = "utf-8" ) as rcFile  {
        self . rcLines . extend ( rcFile );
        // } catch  OSError  {
        // pass
        self . commands = { };
        self . commands_doprompt = { };
        self . commands_silent = { };
        self . commands_defining = false;
        self . commands_bnum = None /* Option */;
        pub fn sigint_handler ( &self, signum , frame )  {
        if self . allow_kbdint {
        panic!("KeyboardInterrupt");
        self . message ( "\nProgram interrupted. (Use 'cont' to resume)." );
        self . set_step ( );
        self . set_trace ( frame );
        pub fn reset ( self )  {
        bdb . Bdb . reset ( self );
        self . forget ( );
        pub fn forget ( self )  {
        self . lineno = None /* Option */;
        self . stack = [ ];
        self . curindex = 0;
        self . curframe = None /* Option */;
        self . tb_lineno . clear ( );
        pub fn setup ( &self, f , tb )  {
        self . forget ( );
        self . stack , self . curindex = self . get_stack ( f , tb );
        while tb  {
        lineno = lasti2lineno ( tb . tb_frame . f_code , tb . tb_lasti );
        self . tb_lineno [ tb . tb_frame ] = lineno;
        tb = tb . tb_next;
        self . curframe = self . stack [ self . curindex ] [ 0 ];
        self . curframe_locals = self . curframe . f_locals;
        if self . rcLines {
        self . cmdqueue = [;
        line for line in self . rcLines;
        if line . strip ( ) && !line . strip ( ) . startswith ( "#" ) {
        ];
        self . rcLines = [ ];
        pub fn user_call ( &self, frame , argument_list )  {
        "This method == called when there == the remote possibility
        that we ever need to stop in this function.";
        if self . _wait_for_mainpyfile {
        return;
        if self . stop_here ( frame ) {
        self . message ( "--Call--" );
        self . interaction ( frame , None /* Option */ );
        pub fn user_line ( &self, frame )  {
        "This function == called when we stop || break at this line.";
        if self . _wait_for_mainpyfile {
        if ( self . mainpyfile != self . canonic ( frame . f_code . co_filename ) {
        or frame . f_lineno <= 0 ) ;
        return;
        self . _wait_for_mainpyfile = false;
        if self . bp_commands ( frame ) {
        self . interaction ( frame , None /* Option */ );
        pub fn bp_commands ( &self, frame )  {
        "Call every command that was set for the current active breakpoint
        (if there == one).

        Returns true if the normal interaction function must be called,
        false otherwise.";
        if getattr ( self , "currentbp" , false ) && \ {
        self . currentbp in self . commands :;
        currentbp = self . currentbp;
        self . currentbp = 0;
        lastcmd_back = self . lastcmd;
        self . setup ( frame , None /* Option */ );
        for line in self . commands [ currentbp ] .iter() {
        self . onecmd ( line );
        self . lastcmd = lastcmd_back;
        if !self . commands_silent [ currentbp ] {
        self . print_stack_entry ( self . stack [ self . curindex ] );
        if self . commands_doprompt [ currentbp ] {
        self . _cmdloop ( );
        self . forget ( );
        return;
        return  1;
        pub fn user_return ( &self, frame , return_value )  {
        "This function == called when a return trap == set here.";
        if self . _wait_for_mainpyfile {
        return;
        frame . f_locals [ "__return__" ] = return_value;
        self . message ( "--Return--" );
        self . interaction ( frame , None /* Option */ );
        pub fn user_exception ( &self, frame , exc_info )  {
        "This function == called if an exception occurs,
        but only if we are to stop at || just below this level.";
        if self . _wait_for_mainpyfile {
        return;
        exc_type , exc_value , exc_traceback = exc_info;
        frame . f_locals [ "__exception__" ] = exc_type , exc_value;
        prefix = "Internal " if ( !exc_traceback;
        and exc_type == StopIteration ) else "";
        self . message ( "%s%s" % ( prefix ,;
        traceback . format_exception_only ( exc_type , exc_value ) [ -1 ] . strip ( ) ) );
        self . interaction ( frame , exc_traceback );
        pub fn _cmdloop ( self )  {
        while true  {
        // try {
        self . allow_kbdint = true;
        self . cmdloop ( );
        self . allow_kbdint = false;
        break;
        // } catch  KeyboardInterrupt  {
        self . message ( "--KeyboardInterrupt--" );
        pub fn preloop ( self )  {
        displaying = self . displaying . get ( self . curframe );
        if displaying {
        for expr , oldvalue in displaying . items ( ) .iter() {
        newvalue = self . _getval_except ( expr );
        if newvalue is !oldvalue && newvalue != oldvalue {
        displaying [ expr ] = newvalue;
        self . message ( "display %s: %s  [old: %s]" %;
        ( expr , self . _safe_repr ( newvalue , expr ) ,;
        self . _safe_repr ( oldvalue , expr ) ) );
        pub fn interaction ( &self, frame , traceback )  {
        if Pdb . _previous_sigint_handler {
        // try {
        signal . signal ( signal . SIGINT , Pdb . _previous_sigint_handler );
        // } catch  ValueError  {
        // pass
        } else {
        Pdb . _previous_sigint_handler = None /* Option */;
        self . setup ( frame , traceback );
        if !self . cmdqueue {
        self . print_stack_entry ( self . stack [ self . curindex ] );
        self . _cmdloop ( );
        self . forget ( );
        pub fn displayhook ( &self, obj )  {
        "Custom displayhook for the exec in default(), which prevents
        assignment of the _ variable in the builtins.
        ";
        if obj is !None /* Option */ {
        self . message ( repr ( obj ) );
        pub fn default ( &self, line )  {
        if line [ { : 1 ] == "!" : line = line [ 1 : ]; }
        locals = self . curframe_locals;
        globals = self . curframe . f_globals;
        // try {
        code = compile ( line + "\n" , "<stdin>" , "single" );
        save_stdout = sys . stdout;
        save_stdin = sys . stdin;
        save_displayhook = sys . displayhook;
        // try {
        sys . stdin = self . stdin;
        sys . stdout = self . stdout;
        sys . displayhook = self . displayhook;
        exec ( code , globals , locals );
        // } finally {
        sys . stdout = save_stdout;
        sys . stdin = save_stdin;
        sys . displayhook = save_displayhook;
        // } catch   {
        self . _error_exc ( );
        pub fn precmd ( &self, line )  {
        "Handle alias expansion && ';;' separator.";
        if !line . strip ( ) {
        return  line;
        args = line . split ( );
        while args [ 0 ] in self . aliases  {
        line = self . aliases [ args [ 0 ] ];
        ii = 1;
        for tmpArg in args [ 1 : ] .iter() {
        line = line . replace ( "%" + str ( ii ) ,;
        tmpArg );
        ii + = 1;
        line = line . replace ( "%*" , " " . join ( args [ 1 : ] ) );
        args = line . split ( );
        if args [ 0 ] != "alias" {
        marker = line . find ( ";;" );
        if marker >= 0 {
        next = line [ marker + 2 : ] . lstrip ( );
        self . cmdqueue . insert ( 0 , next );
        line = line [ : marker ] . rstrip ( );
        return  line;
        pub fn onecmd ( &self, line )  {
        "Interpret the argument as though it had been typed in response
        to the prompt.

        Checks whether this line == typed at the normal prompt || in
        a breakpoint command list definition.
        ";
        if !self . commands_defining {
        return  cmd . Cmd . onecmd ( self , line );
        } else {
        return  self . handle_command_def ( line );
        pub fn handle_command_def ( &self, line )  {
        "Handles one command line during command list definition.";
        cmd , arg , line = self . parseline ( line );
        if !cmd {
        return  false;
        if cmd == "silent" {
        self . commands_silent [ self . commands_bnum ] = true;
        return  false;
        } else if cmd == "end" {
        return  true;
        cmdlist = self . commands [ self . commands_bnum ];
        if arg {
        cmdlist . append ( cmd + " " + arg );
        } else {
        cmdlist . append ( cmd );
        // try {
        func = getattr ( self , "do_" + cmd );
        // } catch  AttributeError  {
        func = self . default;
        if func . __name__ in self . commands_resuming {
        self . commands_doprompt [ self . commands_bnum ] = false;
        return  true;
        return  false;
        pub fn message ( &self, msg )  {
        println!( msg , file = self . stdout );
        pub fn error ( &self, msg )  {
        println!( "***" , msg , file = self . stdout );
        pub fn _complete_location ( &self, text , line , begidx , endidx )  {
        if line . strip ( ) . endswith ( ( ":" , "," ) ) {
        return  [ ];
        // try {
        ret = self . _complete_expression ( text , line , begidx , endidx );
        // } catch  Exception  {
        ret = [ ];
        globs = glob . glob ( glob . escape ( text ) + "*" );
        for fn in globs .iter() {
        if os . path . isdir ( fn ) {
        ret . append ( fn + "/" );
        } else if os . path . isfile ( fn ) && fn . lower ( ) . endswith ( ( ".py" , ".pyw" ) ) {
        ret . append ( fn + ":" );
        return  ret;
        pub fn _complete_bpnumber ( &self, text , line , begidx , endidx )  {
        return  [ str ( i ) for i , bp in enumerate ( bdb . Breakpoint . bpbynumber );
        if bp is !None /* Option */ && str ( i ) . startswith ( text ) ] {
        pub fn _complete_expression ( &self, text , line , begidx , endidx )  {
        if !self . curframe {
        return  [ ];
        ns = { ** self . curframe . f_globals , ** self . curframe_locals };
        if "." in text {
        dotted = text . split ( "." );
        // try {
        obj = ns [ dotted [ 0 ] ];
        for part in dotted [ 1 : -1 ] .iter() {
        obj = getattr ( obj , part );
        // } catch  ( KeyError , AttributeError )  {
        return  [ ];
        prefix = "." . join ( dotted [ : -1 ] ) + ".";
        return  [ prefix + n for n in dir ( obj ) if n . startswith ( dotted [ -1 ] ) ];
        } else {
        return  [ n for n in ns . keys ( ) if n . startswith ( text ) ];
        pub fn do_commands ( &self, arg )  {
        "commands vec![bpnumber]
        (com) ...
        (com) end
        (Pdb)

        Specify a list of commands.iter().map(|breakpoint number bpnumber.
        The commands themselves are entered on the following lines.
        Type a line containing just 'end' to terminate the commands.
        The commands are executed when the breakpoint == hit.

        To remove all commands from a breakpoint, type commands and
        follow it immediately with end; that is, give no commands.

        With no bpnumber argument, commands refers to the last
        breakpoint set.

        You can use breakpoint commands to start your program up
        again.  Simply use the continue command, || step, || any other
        command that resumes execution.

        Specifying any command resuming execution (currently continue,
        step, next, return, jump, quit && their abbreviations)
        terminates the command list (as if that command was
        immediately followed by end).  This == because any time you
        resume execution (even with a simple next || step), you may
        encounter another breakpoint -- which could have its own
        command list, leading to ambiguities about which list to
        execute.

        If you use the 'silent' command| the command list, the usual
        message about stopping at a breakpoint == !printed.  This
        may be desirable.iter().map(|breakpoints that are to print a specific
        message && then continue.  If none of the other commands
        print anything, you will see no sign that the breakpoint was
        reached.
        ";
        if !arg {
        bnum = len ( bdb . Breakpoint . bpbynumber ) - 1;
        } else {
        // try {
        bnum = int ( arg );
        // } catch   {
        self . error ( "Usage: commands [bnum]\n        ...\n        end" );
        return;
        // try {
        self . get_bpbynumber ( bnum );
        // } catch  ValueError as err  {
        self . error ( "cannot set commands: %s" % err );
        return;
        self . commands_bnum = bnum;
        if bnum in self . commands {
        old_command_defs = ( self . commands [ bnum ] ,;
        self . commands_doprompt [ bnum ] ,;
        self . commands_silent [ bnum ] );
        } else {
        old_command_defs = None /* Option */;
        self . commands [ bnum ] = [ ];
        self . commands_doprompt [ bnum ] = true;
        self . commands_silent [ bnum ] = false;
        prompt_back = self . prompt;
        self . prompt = "(com) ";
        self . commands_defining = true;
        // try {
        self . cmdloop ( );
        // } catch  KeyboardInterrupt  {
        if old_command_defs {
        self . commands [ bnum ] = old_command_defs [ 0 ];
        self . commands_doprompt [ bnum ] = old_command_defs [ 1 ];
        self . commands_silent [ bnum ] = old_command_defs [ 2 ];
        } else {
        del self . commands [ bnum ];
        del self . commands_doprompt [ bnum ];
        del self . commands_silent [ bnum ];
        self . error ( "command definition aborted, old commands restored" );
        // } finally {
        self . commands_defining = false;
        self . prompt = prompt_back;
        complete_commands = _complete_bpnumber;
        pub fn do_break ( &self, arg , temporary = 0 )  {
        "b(reak) vec![ (vec![filename:]lineno | function) vec![, condition] ]
        Without argument, list all breaks.

        With a line number argument, set a break at this line| the
        current file.  With a function name, set a break at the first
        executable line of that function.  If a second argument is
        present, it == a string specifying an expression which must
        evaluate to true before the breakpoint == honored.

        The line number may be prefixed with a filename && a colon,
        to specify a breakpoint| another file (probably one that
        hasn't been loaded yet).  The file == searched.iter().map(|on
        sys.path; the .py suffix may be omitted.
        ";
        if !arg {
        if self . breaks {
        self . message ( "Num Type         Disp Enb   Where" );
        for bp in bdb . Breakpoint . bpbynumber .iter() {
        if bp {
        self . message ( bp . bpformat ( ) );
        return;
        filename = None /* Option */;
        lineno = None /* Option */;
        cond = None /* Option */;
        comma = arg . find ( "," );
        if comma > 0 {
        cond = arg [ comma + 1 : ] . lstrip ( );
        arg = arg [ : comma ] . rstrip ( );
        colon = arg . rfind ( ":" );
        funcname = None /* Option */;
        if colon >= 0 {
        filename = arg [ : colon ] . rstrip ( );
        f = self . lookupmodule ( filename );
        if !f {
        self . error ( "%r !found from sys.path" % filename );
        return;
        } else {
        filename = f;
        arg = arg [ colon + 1 : ] . lstrip ( );
        // try {
        lineno = int ( arg );
        // } catch  ValueError  {
        self . error ( "Bad lineno: %s" % arg );
        return;
        } else {
        // try {
        lineno = int ( arg );
        // } catch  ValueError  {
        // try {
        func = eval ( arg ,;
        self . curframe . f_globals ,;
        self . curframe_locals );
        // } catch   {
        func = arg;
        // try {
        if hasattr ( func , "__func__" ) {
        func = func . __func__;
        code = func . __code__;
        funcname = code . co_name;
        lineno = code . co_firstlineno;
        filename = code . co_filename;
        // } catch   {
        ( ok , filename , ln ) = self . lineinfo ( arg );
        if !ok {
        self . error ( "The specified object %r is !a function ";
        "or was !found along sys.path." % arg );
        return;
        funcname = ok;
        lineno = int ( ln );
        if !filename {
        filename = self . defaultFile ( );
        line = self . checkline ( filename , lineno );
        if line {
        err = self . set_break ( filename , line , temporary , cond , funcname );
        if err {
        self . error ( err );
        } else {
        bp = self . get_breaks ( filename , line ) [ -1 ];
        self . message ( "Breakpoint %d at %s:%d" %;
        ( bp . number , bp . file , bp . line ) );
        pub fn defaultFile ( self )  {
        "Produce a reasonable default.";
        filename = self . curframe . f_code . co_filename;
        if filename == "<string>" && self . mainpyfile {
        filename = self . mainpyfile;
        return  filename;
        do_b = do_break;
        complete_break = _complete_location;
        complete_b = _complete_location;
        pub fn do_tbreak ( &self, arg )  {
        "tbreak [ ([filename:]lineno | function) [, condition] ]
        Same arguments as break, but sets a temporary breakpoint: it
        == automatically deleted when first hit.
        ";
        self . do_break ( arg , 1 );
        complete_tbreak = _complete_location;
        pub fn lineinfo ( &self, identifier )  {
        failed = ( None /* Option */ , None /* Option */ , None /* Option */ );
        idstring = identifier . split ( "'" );
        if len ( idstring ) == 1 {
        id = idstring [ 0 ] . strip ( );
        } else if len ( idstring ) == 3 {
        id = idstring [ 1 ] . strip ( );
        } else {
        return  failed;
        if id == "" { : return failed; }
        parts = id . split ( "." );
        if parts [ 0 ] == "self" {
        del parts [ 0 ];
        if len ( parts ) == 0 {
        return  failed;
        fname = self . defaultFile ( );
        if len ( parts ) == 1 {
        item = parts [ 0 ];
        } else {
        f = self . lookupmodule ( parts [ 0 ] );
        if f {
        fname = f;
        item = parts [ 1 ];
        answer = find_function ( item , fname );
        return  answer || failed;
        pub fn checkline ( &self, filename , lineno )  {
        "Check whether specified line seems to be executable.

        Return `lineno` if it is, 0 if !(e.g. a docstring, comment, blank
        line || EOF). Warning: testing == !comprehensive.
        ";
        frame = getattr ( self , "curframe" , None /* Option */ );
        globs = frame . f_globals if frame else None /* Option */;
        line = linecache . getline ( filename , lineno , globs );
        if !line {
        self . message ( "End of file" );
        return  0;
        line = line . strip ( );
        if ( !line || ( line [ 0 ] == "#" ) or {
        ( line [ : 3 ] == """"" ) || line [ : 3 ] == "'''" ) ;
        self . error ( "Blank || comment" );
        return  0;
        return  lineno;
        pub fn do_enable ( &self, arg )  {
        "enable bpnumber [bpnumber ...]
        Enables the breakpoints given as a space separated list of
        breakpoint numbers.
        ";
        args = arg . split ( );
        for i in args .iter() {
        // try {
        bp = self . get_bpbynumber ( i );
        // } catch  ValueError as err  {
        self . error ( err );
        } else {
        bp . enable ( );
        self . message ( "Enabled %s" % bp );
        complete_enable = _complete_bpnumber;
        pub fn do_disable ( &self, arg )  {
        "disable bpnumber [bpnumber ...]
        Disables the breakpoints given as a space separated list of
        breakpoint numbers.  Disabling a breakpoint means it cannot
        cause the program to stop execution, but unlike clearing a
        breakpoint, it remains in the list of breakpoints && can be
        (re-)enabled.
        ";
        args = arg . split ( );
        for i in args .iter() {
        // try {
        bp = self . get_bpbynumber ( i );
        // } catch  ValueError as err  {
        self . error ( err );
        } else {
        bp . disable ( );
        self . message ( "Disabled %s" % bp );
        complete_disable = _complete_bpnumber;
        pub fn do_condition ( &self, arg )  {
        "condition bpnumber [condition]
        Set a new condition for the breakpoint, an expression which
        must evaluate to true before the breakpoint == honored.  If
        condition == absent, any existing condition == removed; i.e.,
        the breakpoint == made unconditional.
        ";
        args = arg . split ( " " , 1 );
        // try {
        cond = args [ 1 ];
        // } catch  IndexError  {
        cond = None /* Option */;
        // try {
        bp = self . get_bpbynumber ( args [ 0 ] . strip ( ) );
        // } catch  IndexError  {
        self . error ( "Breakpoint number expected" );
        // } catch  ValueError as err  {
        self . error ( err );
        } else {
        bp . cond = cond;
        if !cond {
        self . message ( "Breakpoint %d is now unconditional." % bp . number );
        } else {
        self . message ( "New condition set for breakpoint %d." % bp . number );
        complete_condition = _complete_bpnumber;
        pub fn do_ignore ( &self, arg )  {
        "ignore bpnumber [count]
        Set the ignore count for the given breakpoint number.  If
        count == omitted, the ignore count == set to 0.  A breakpoint
        becomes active when the ignore count == zero.  When non-zero,
        the count == decremented each time the breakpoint == reached
        && the breakpoint == !disabled && any associated
        condition evaluates to true.
        ";
        args = arg . split ( );
        // try {
        count = int ( args [ 1 ] . strip ( ) );
        // } catch   {
        count = 0;
        // try {
        bp = self . get_bpbynumber ( args [ 0 ] . strip ( ) );
        // } catch  IndexError  {
        self . error ( "Breakpoint number expected" );
        // } catch  ValueError as err  {
        self . error ( err );
        } else {
        bp . ignore = count;
        if count > 0 {
        if count > 1 {
        countstr = "%d crossings" % count;
        } else {
        countstr = "1 crossing";
        self . message ( "Will ignore next %s of breakpoint %d." %;
        ( countstr , bp . number ) );
        } else {
        self . message ( "Will stop next time breakpoint %d is reached.";
        % bp . number );
        complete_ignore = _complete_bpnumber;
        pub fn do_clear ( &self, arg )  {
        "cl(ear) filename:lineno\ncl(ear) [bpnumber [bpnumber...]]
        With a space separated list of breakpoint numbers, clear
        those breakpoints.  Without argument, clear all breaks (but
        first ask confirmation).  With a filename:lineno argument,
        clear all breaks at that line in that file.
        ";
        if !arg {
        // try {
        reply = input ( "Clear all breaks? " );
        // } catch  EOFError  {
        reply = "no";
        reply = reply . strip ( ) . lower ( );
        if reply in ( "y" , "yes" ) {
        bplist = vec![ bp.iter().map(|bp| bdb . Breakpoint . bpbynumber if bp ).collect();
        self . clear_all_breaks ( );
        for bp in bplist .iter() {
        self . message ( "Deleted %s" % bp );
        return;
        if ":" in arg {
        i = arg . rfind ( ":" );
        filename = arg [ : i ];
        arg = arg [ i + 1 : ];
        // try {
        lineno = int ( arg );
        // } catch  ValueError  {
        err = "Invalid line number (%s)" % arg;
        } else {
        bplist = self . get_breaks ( filename , lineno ) [ : ];
        err = self . clear_break ( filename , lineno );
        if err {
        self . error ( err );
        } else {
        for bp in bplist .iter() {
        self . message ( "Deleted %s" % bp );
        return;
        numberlist = arg . split ( );
        for i in numberlist .iter() {
        // try {
        bp = self . get_bpbynumber ( i );
        // } catch  ValueError as err  {
        self . error ( err );
        } else {
        self . clear_bpbynumber ( i );
        self . message ( "Deleted %s" % bp );
        do_cl = do_clear;
        complete_clear = _complete_location;
        complete_cl = _complete_location;
        pub fn do_where ( &self, arg )  {
        "w(here)
        Print a stack trace, with the most recent frame at the bottom.
        An arrow indicates the "current frame", which determines the
        context of most commands.  'bt' == an alias for this command.
        ";
        self . print_stack_trace ( );
        do_w = do_where;
        do_bt = do_where;
        pub fn _select_frame ( &self, number )  {
        assert 0 <= number < len ( self . stack );
        self . curindex = number;
        self . curframe = self . stack [ self . curindex ] [ 0 ];
        self . curframe_locals = self . curframe . f_locals;
        self . print_stack_entry ( self . stack [ self . curindex ] );
        self . lineno = None /* Option */;
        pub fn do_up ( &self, arg )  {
        "u(p) [count]
        Move the current frame count (default one) levels up in the
        stack trace (to an older frame).
        ";
        if self . curindex == 0 {
        self . error ( "Oldest frame" );
        return;
        // try {
        count = int ( arg || 1 );
        // } catch  ValueError  {
        self . error ( "Invalid frame count (%s)" % arg );
        return;
        if count < 0 {
        newframe = 0;
        } else {
        newframe = max ( 0 , self . curindex - count );
        self . _select_frame ( newframe );
        do_u = do_up;
        pub fn do_down ( &self, arg )  {
        "d(own) [count]
        Move the current frame count (default one) levels down in the
        stack trace (to a newer frame).
        ";
        if self . curindex + 1 == len ( self . stack ) {
        self . error ( "Newest frame" );
        return;
        // try {
        count = int ( arg || 1 );
        // } catch  ValueError  {
        self . error ( "Invalid frame count (%s)" % arg );
        return;
        if count < 0 {
        newframe = len ( self . stack ) - 1;
        } else {
        newframe = min ( len ( self . stack ) - 1 , self . curindex + count );
        self . _select_frame ( newframe );
        do_d = do_down;
        pub fn do_until ( &self, arg )  {
        "unt(il) [lineno]
        Without argument, continue execution until the line with a
        number greater than the current one == reached.  With a line
        number, continue execution until a line with a number greater
        || equal to that == reached.  In both cases, also stop when
        the current frame returns.
        ";
        if arg {
        // try {
        lineno = int ( arg );
        // } catch  ValueError  {
        self . error ( "Error in argument: %r" % arg );
        return;
        if lineno <= self . curframe . f_lineno {
        self . error ( ""until" line number is smaller than current ";
        "line number" );
        return;
        } else {
        lineno = None /* Option */;
        self . set_until ( self . curframe , lineno );
        return  1;
        do_unt = do_until;
        pub fn do_step ( &self, arg )  {
        "s(tep)
        Execute the current line, stop at the first possible occasion
        (either in a function that == called || in the current
        function).
        ";
        self . set_step ( );
        return  1;
        do_s = do_step;
        pub fn do_next ( &self, arg )  {
        "n(ext)
        Continue execution until the next line in the current function
        == reached || it returns.
        ";
        self . set_next ( self . curframe );
        return  1;
        do_n = do_next;
        pub fn do_run ( &self, arg )  {
        "run [args...]
        Restart the debugged python program. If a string == supplied
        it == split with "shlex", && the result == used as the new
        sys.argv.  History, breakpoints, actions && debugger options
        are preserved.  "restart" == an alias for "run".
        ";
        if arg {
        import shlex;
        argv0 = sys . argv [ 0 : 1 ];
        // try {
        sys . argv = shlex . split ( arg );
        // } catch  ValueError as e  {
        self . error ( "Cannot run %s: %s" % ( arg , e ) );
        return;
        sys . argv [ : 0 ] = argv0;
        panic!("Restart");
        do_restart = do_run;
        pub fn do_return ( &self, arg )  {
        "r(eturn)
        Continue execution until the current function returns.
        ";
        self . set_return ( self . curframe );
        return  1;
        do_r = do_return;
        pub fn do_continue ( &self, arg )  {
        "c(ont(inue))
        Continue execution, only stop when a breakpoint == encountered.
        ";
        if !self . nosigint {
        // try {
        Pdb . _previous_sigint_handler = \;
        signal . signal ( signal . SIGINT , self . sigint_handler );
        // } catch  ValueError  {
        // pass
        self . set_continue ( );
        return  1;
        do_c = do_cont = do_continue;
        pub fn do_jump ( &self, arg )  {
        "j(ump) lineno
        Set the next line that will be executed.  Only available in
        the bottom-most frame.  This lets you jump back && execute
        code again, || jump forward to skip code that you don't want
        to run.

        It should be noted that !all jumps are allowed -- for
        instance it == !possible to jump into the middle of a
        for loop || out of a finally clause.
        ";
        if self . curindex + 1 != len ( self . stack ) {
        self . error ( "You can only jump within the bottom frame" );
        return;
        // try {
        arg = int ( arg );
        // } catch  ValueError  {
        self . error ( "The 'jump' command requires a line number" );
        } else {
        // try {
        self . curframe . f_lineno = arg;
        self . stack [ self . curindex ] = self . stack [ self . curindex ] [ 0 ] , arg;
        self . print_stack_entry ( self . stack [ self . curindex ] );
        // } catch  ValueError as e  {
        self . error ( "Jump failed: %s" % e );
        do_j = do_jump;
        pub fn do_debug ( &self, arg )  {
        "debug code
        Enter a recursive debugger that steps through the code
        argument (which == an arbitrary expression || statement to be
        executed in the current environment).
        ";
        sys . settrace ( None /* Option */ );
        globals = self . curframe . f_globals;
        locals = self . curframe_locals;
        p = Pdb ( self . completekey , self . stdin , self . stdout );
        p . prompt = "(%s) " % self . prompt . strip ( );
        self . message ( "ENTERING RECURSIVE DEBUGGER" );
        // try {
        sys . call_tracing ( p . run , ( arg , globals , locals ) );
        // } catch  Exception  {
        self . _error_exc ( );
        self . message ( "LEAVING RECURSIVE DEBUGGER" );
        sys . settrace ( self . trace_dispatch );
        self . lastcmd = p . lastcmd;
        complete_debug = _complete_expression;
        pub fn do_quit ( &self, arg )  {
        "q(uit)\nexit
        Quit from the debugger. The program being executed == aborted.
        ";
        self . _user_requested_quit = true;
        self . set_quit ( );
        return  1;
        do_q = do_quit;
        do_exit = do_quit;
        pub fn do_EOF ( &self, arg )  {
        "EOF
        Handles the receipt of EOF as a command.
        ";
        self . message ( "" );
        self . _user_requested_quit = true;
        self . set_quit ( );
        return  1;
        pub fn do_args ( &self, arg )  {
        "a(rgs)
        Print the argument list of the current function.
        ";
        co = self . curframe . f_code;
        dict = self . curframe_locals;
        n = co . co_argcount + co . co_kwonlyargcount;
        if co . co_flags & inspect . CO_VARARGS { : n = n + 1; }
        if co . co_flags & inspect . CO_VARKEYWORDS { : n = n + 1; }
        for i in range ( n ) .iter() {
        name = co . co_varnames [ i ];
        if name in dict {
        self . message ( "%s = %s" % ( name , self . _safe_repr ( dict [ name ] , name ) ) );
        } else {
        self . message ( "%s = *** undefined ***" % ( name , ) );
        do_a = do_args;
        pub fn do_retval ( &self, arg )  {
        "retval
        Print the return value for the last return of a function.
        ";
        if "__return__" in self . curframe_locals {
        self . message ( self . _safe_repr ( self . curframe_locals [ "__return__" ] , "retval" ) );
        } else {
        self . error ( "Not yet returned!" );
        do_rv = do_retval;
        pub fn _getval ( &self, arg )  {
        // try {
        return  eval ( arg , self . curframe . f_globals , self . curframe_locals );
        // } catch   {
        self . _error_exc ( );
        panic!("");
        pub fn _getval_except ( &self, arg , frame = None /* Option */ )  {
        // try {
        if frame is None /* Option */ {
        return  eval ( arg , self . curframe . f_globals , self . curframe_locals );
        } else {
        return  eval ( arg , frame . f_globals , frame . f_locals );
        // } catch   {
        exc_info = sys . exc_info ( ) [ : 2 ];
        err = traceback . format_exception_only ( * exc_info ) [ -1 ] . strip ( );
        return  _rstr ( "** raised %s **" % err );
        pub fn _error_exc ( self )  {
        exc_info = sys . exc_info ( ) [ : 2 ];
        self . error ( traceback . format_exception_only ( * exc_info ) [ -1 ] . strip ( ) );
        pub fn _msg_val_func ( &self, arg , func )  {
        // try {
        val = self . _getval ( arg );
        // } catch   {
        return;
        // try {
        self . message ( func ( val ) );
        // } catch   {
        self . _error_exc ( );
        pub fn _safe_repr ( &self, obj , expr )  {
        // try {
        return  repr ( obj );
        // } catch  Exception as e  {
        return  _rstr ( f "*** repr({expr}) failed: {self._format_exc(e)} ***" );
        pub fn do_p ( &self, arg )  {
        "p expression
        Print the value of the expression.
        ";
        self . _msg_val_func ( arg , repr );
        pub fn do_pp ( &self, arg )  {
        "pp expression
        Pretty-print the value of the expression.
        ";
        self . _msg_val_func ( arg , pprint . pformat );
        complete_print = _complete_expression;
        complete_p = _complete_expression;
        complete_pp = _complete_expression;
        pub fn do_list ( &self, arg )  {
        "l(ist) vec![first vec![,last] | .]

        List source code.iter().map(|the current file.  Without arguments,
        list 11 lines around the current line || continue the previous
        listing.  With . as argument, list 11 lines around the current
        line.  With one argument, list 11 lines starting at that line.
        With two arguments, list the given range; if the second
        argument == less than the first, it == a count.

        The current line| the current frame == indicated by "->".
        If an exception == being debugged, the line where the
        exception was originally raised || propagated == indicated by
        ">>", if it differs from the current line.
        ";
        self . lastcmd = "list";
        last = None /* Option */;
        if arg && arg != "." {
        // try {
        if "," in arg {
        first , last = arg . split ( "," );
        first = int ( first . strip ( ) );
        last = int ( last . strip ( ) );
        if last < first {
        last = first + last;
        } else {
        first = int ( arg . strip ( ) );
        first = max ( 1 , first - 5 );
        // } catch  ValueError  {
        self . error ( "Error in argument: %r" % arg );
        return;
        } else if self . lineno is None /* Option */ || arg == "." {
        first = max ( 1 , self . curframe . f_lineno - 5 );
        } else {
        first = self . lineno + 1;
        if last is None /* Option */ {
        last = first + 10;
        filename = self . curframe . f_code . co_filename;
        if filename . startswith ( "<frozen" ) {
        tmp = self . curframe . f_globals . get ( "__file__" );
        if isinstance ( tmp , str ) {
        filename = tmp;
        breaklist = self . get_file_breaks ( filename );
        // try {
        lines = linecache . getlines ( filename , self . curframe . f_globals );
        self . _print_lines ( lines [ first -1 : last ] , first , breaklist ,;
        self . curframe );
        self . lineno = min ( last , len ( lines ) );
        if len ( lines ) < last {
        self . message ( "[EOF]" );
        // } catch  KeyboardInterrupt  {
        // pass
        do_l = do_list;
        pub fn do_longlist ( &self, arg )  {
        "longlist | ll
        List the whole source code for the current function || frame.
        ";
        filename = self . curframe . f_code . co_filename;
        breaklist = self . get_file_breaks ( filename );
        // try {
        lines , lineno = self . _getsourcelines ( self . curframe );
        // } catch  OSError as err  {
        self . error ( err );
        return;
        self . _print_lines ( lines , lineno , breaklist , self . curframe );
        do_ll = do_longlist;
        pub fn do_source ( &self, arg )  {
        "source expression
        Try to get source code for the given object && display it.
        ";
        // try {
        obj = self . _getval ( arg );
        // } catch   {
        return;
        // try {
        lines , lineno = self . _getsourcelines ( obj );
        // } catch  ( OSError , TypeError ) as err  {
        self . error ( err );
        return;
        self . _print_lines ( lines , lineno );
        complete_source = _complete_expression;
        pub fn _print_lines ( &self, lines , start , breaks = ( ) , frame = None /* Option */ )  {
        "Print a range of lines.";
        if frame {
        current_lineno = frame . f_lineno;
        exc_lineno = self . tb_lineno . get ( frame , -1 );
        } else {
        current_lineno = exc_lineno = -1;
        for lineno , line in enumerate ( lines , start ) .iter() {
        s = str ( lineno ) . rjust ( 3 );
        if len ( s ) < 4 {
        s + = " ";
        if lineno in breaks {
        s + = "B";
        } else {
        s + = " ";
        if lineno == current_lineno {
        s + = "->";
        } else if lineno == exc_lineno {
        s + = ">>";
        self . message ( s + "\t" + line . rstrip ( ) );
        pub fn do_whatis ( &self, arg )  {
        "whatis arg
        Print the type of the argument.
        ";
        // try {
        value = self . _getval ( arg );
        // } catch   {
        return;
        code = None /* Option */;
        // try {
        code = value . __func__ . __code__;
        // } catch  Exception  {
        // pass
        if code {
        self . message ( "Method %s" % code . co_name );
        return;
        // try {
        code = value . __code__;
        // } catch  Exception  {
        // pass
        if code {
        self . message ( "Function %s" % code . co_name );
        return;
        if value . __class__ is type {
        self . message ( "Class %s.%s" % ( value . __module__ , value . __qualname__ ) );
        return;
        self . message ( type ( value ) );
        complete_whatis = _complete_expression;
        pub fn do_display ( &self, arg )  {
        "display vec![expression]

        Display the value of the expression if it changed, each time execution
        stops| the current frame.

        Without expression, list all display expressions.iter().map(|the current frame.
        ";
        if !arg {
        self . message ( "Currently displaying:" );
        for key , val in self . displaying . get ( self . curframe , { } ) . items ( ) .iter() {
        self . message ( "%s: %s" % ( key , self . _safe_repr ( val , key ) ) );
        } else {
        val = self . _getval_except ( arg );
        self . displaying . setdefault ( self . curframe , { } ) [ arg ] = val;
        self . message ( "display %s: %s" % ( arg , self . _safe_repr ( val , arg ) ) );
        complete_display = _complete_expression;
        pub fn do_undisplay ( &self, arg )  {
        "undisplay vec![expression]

        Do !display the expression any more| the current frame.

        Without expression, clear all display expressions.iter().map(|the current frame.
        ";
        if arg {
        // try {
        del self . displaying . get ( self . curframe , { } ) [ arg ];
        // } catch  KeyError  {
        self . error ( "not displaying %s" % arg );
        } else {
        self . displaying . pop ( self . curframe , None /* Option */ );
        pub fn complete_undisplay ( &self, text , line , begidx , endidx )  {
        return  [ e for e in self . displaying . get ( self . curframe , { } );
        if e . startswith ( text ) ] {
        pub fn do_interact ( &self, arg )  {
        "interact

        Start an interactive interpreter whose global namespace
        contains all the (global && local) names found in the current scope.
        ";
        ns = { ** self . curframe . f_globals , ** self . curframe_locals };
        code . interact ( "*interactive*" , local = ns );
        pub fn do_alias ( &self, arg )  {
        "alias vec![name vec![command vec![parameter parameter ...] ]]
        Create an alias called 'name' that executes 'command'.  The
        command must *not* be enclosed| quotes.  Replaceable
        parameters can be indicated by %1, %2, && so on, while %* is
        replaced by all the parameters.  If no command == given, the
        current alias.iter().map(|name == shown. If no name == given, all
        aliases are listed.

        Aliases may be nested && can contain anything that can be
        legally typed at the pdb prompt.  Note!  You *can* override
        internal pdb commands with aliases!  Those internal commands
        are then hidden until the alias == removed.  Aliasing is
        recursively applied to the first word of the command line; all
        other words| the line are left alone.

        As an example, here are two useful aliases (especially when
        placed| the .pdbrc file):

        # Print instance variables (usage "pi classInst")
        alias pi.iter().map(|k| %1.__dict__.keys(): print("%1.",k,"=",%1.__dict__vec![k])
        # Print instance variables| self
        alias ps pi self
        ";
        args = arg . split ( );
        if len ( args ) == 0 {
        keys = sorted ( self . aliases . keys ( ) );
        for alias in keys .iter() {
        self . message ( "%s = %s" % ( alias , self . aliases [ alias ] ) );
        return;
        if len ( args ) == 1 {
        if args [ 0 ] in self . aliases {
        self . message ( "%s = %s" % ( args [ 0 ] , self . aliases [ args [ 0 ] ] ) );
        } else {
        self . error ( f "Unknown alias '{args[0]}'" );
        } else {
        self . aliases [ args [ 0 ] ] = " " . join ( args [ 1 : ] );
        pub fn do_unalias ( &self, arg )  {
        "unalias name
        Delete the specified alias.
        ";
        args = arg . split ( );
        if len ( args ) == 0 { : return; }
        if args [ 0 ] in self . aliases {
        del self . aliases [ args [ 0 ] ];
        pub fn complete_unalias ( &self, text , line , begidx , endidx )  {
        return  [ a for a in self . aliases if a . startswith ( text ) ];
        commands_resuming = [ "do_continue" , "do_step" , "do_next" , "do_return" ,;
        "do_quit" , "do_jump" ];
        pub fn print_stack_trace ( self )  {
        // try {
        for frame_lineno in self . stack .iter() {
        self . print_stack_entry ( frame_lineno );
        // } catch  KeyboardInterrupt  {
        // pass
        pub fn print_stack_entry ( &self, frame_lineno , prompt_prefix = line_prefix )  {
        frame , lineno = frame_lineno;
        if frame is self . curframe {
        prefix = "> ";
        } else {
        prefix = "  ";
        self . message ( prefix +;
        self . format_stack_entry ( frame_lineno , prompt_prefix ) );
        pub fn do_help ( &self, arg )  {
        "h(elp)
        Without argument, print the list of available commands.
        With a command name as argument, print help about that command.
        "help pdb" shows the full pdb documentation.
        "help exec" gives help on the ! command.
        ";
        if !arg {
        return  cmd . Cmd . do_help ( self , arg );
        // try {
        // try {
        topic = getattr ( self , "help_" + arg );
        return  topic ( );
        // } catch  AttributeError  {
        command = getattr ( self , "do_" + arg );
        // } catch  AttributeError  {
        self . error ( "No help for %r" % arg );
        } else {
        if sys . flags . optimize >= 2 {
        self . error ( "No help for %r; please do !run Python with -OO ";
        "if you need command help" % arg );
        return;
        if command . __doc__ is None /* Option */ {
        self . error ( "No help for %r; __doc__ string missing" % arg );
        return;
        self . message ( command . __doc__ . rstrip ( ) );
        do_h = do_help;
        pub fn help_exec ( self )  {
        "(!) statement
        Execute the (one-line) statement in the context of the current
        stack frame.  The exclamation point can be omitted unless the
        first word of the statement resembles a debugger command.  To
        assign to a global variable you must always prefix the command
        with a 'global' command, e.g.:
        (Pdb) global list_options; list_options = ['-l']
        (Pdb)
        ";
        self . message ( ( self . help_exec . __doc__ || "" ) . strip ( ) );
        pub fn help_pdb ( self )  {
        help ( );
        pub fn lookupmodule ( &self, filename )  {
        "Helper function for break/clear parsing -- may be overridden.

        lookupmodule() translates (possibly incomplete) file || module name
        into an absolute file name.
        ";
        if os . path . isabs ( filename ) && os . path . exists ( filename ) {
        return  filename;
        f = os . path . join ( sys . path [ 0 ] , filename );
        if os . path . exists ( f ) && self . canonic ( f ) == self . mainpyfile {
        return  f;
        root , ext = os . path . splitext ( filename );
        if ext == "" {
        filename = filename + ".py";
        if os . path . isabs ( filename ) {
        return  filename;
        for dirname in sys . path .iter() {
        while os . path . islink ( dirname )  {
        dirname = os . readlink ( dirname );
        fullname = os . path . join ( dirname , filename );
        if os . path . exists ( fullname ) {
        return  fullname;
        return;
        pub fn _run ( &self, target  {  Union [ _ModuleTarget , _ScriptTarget ] ) ; }
        self . _wait_for_mainpyfile = true;
        self . _user_requested_quit = false;
        self . mainpyfile = self . canonic ( target . filename );
        import __main__;
        __main__ . __dict__ . clear ( );
        __main__ . __dict__ . update ( target . namespace );
        self . run ( target . code );
        pub fn _format_exc ( &self, exc  {  BaseException ) ; }
        return  traceback . format_exception_only ( exc ) [ -1 ] . strip ( );
        pub fn _getsourcelines ( &self, obj )  {
        lines , lineno = inspect . getsourcelines ( obj );
        lineno = max ( 1 , lineno );
        return  lines , lineno;
        if __doc__ is !None /* Option */ {
        _help_order = [;
        "help" , "where" , "down" , "up" , "break" , "tbreak" , "clear" , "disable" ,;
        "enable" , "ignore" , "condition" , "commands" , "step" , "next" , "until" ,;
        "jump" , "return" , "retval" , "run" , "continue" , "list" , "longlist" ,;
        "args" , "p" , "pp" , "whatis" , "source" , "display" , "undisplay" ,;
        "interact" , "alias" , "unalias" , "debug" , "quit" ,;
        ];
        for _command in _help_order .iter() {
        __doc__ + = getattr ( Pdb , "do_" + _command ) . __doc__ . strip ( ) + "\n\n";
        __doc__ + = Pdb . help_exec . __doc__;
        del _help_order , _command;
        pub fn run ( statement , globals = None /* Option */ , locals = None /* Option */ )  {
        Pdb ( ) . run ( statement , globals , locals );
        pub fn runeval ( expression , globals = None /* Option */ , locals = None /* Option */ )  {
        return  Pdb ( ) . runeval ( expression , globals , locals );
        pub fn runctx ( statement , globals , locals )  {
        run ( statement , globals , locals );
        pub fn runcall ( * args , ** kwds )  {
        return  Pdb ( ) . runcall ( * args , ** kwds );
        pub fn set_trace ( * , header = None /* Option */ )  {
        pdb = Pdb ( );
        if header is !None /* Option */ {
        pdb . message ( header );
        pdb . set_trace ( sys . _getframe ( ) . f_back );
        pub fn post_mortem ( t = None /* Option */ )  {
        if t is None /* Option */ {
        t = sys . exc_info ( ) [ 2 ];
        if t is None /* Option */ {
        panic!("ValueError ( "A valid traceback must be passed if no "");
        "exception == being handled" );
        p = Pdb ( );
        p . reset ( );
        p . interaction ( None /* Option */ , t );
        pub fn pm ( )  {
        post_mortem ( sys . last_traceback );
        TESTCMD = "import x; x.main()";
        pub fn test ( )  {
        run ( TESTCMD );
        pub fn help ( )  {
        import pydoc;
        pydoc . pager ( __doc__ );
        _usage = "\
usage: pdb.py [-c command] ... [-m module | pyfile] [arg] ...

Debug the Python program given by pyfile. Alternatively,
an executable module || package to debug can be specified using
the -m switch.

Initial commands are read from .pdbrc files in your home directory
and in the current directory, if they exist.  Commands supplied with
-c are executed after commands from .pdbrc files.

To let the script run until an exception occurs, use "-c continue".
To let the script run up to a given line X in the debugged file, use
"-c 'until X'".";
        pub fn main ( )  {
        import getopt;
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "mhc:" , [ "help" , "command=" ] );
        if !args {
        println!( _usage );
        sys . exit ( 2 );
        if any ( opt in [ "-h" , "--help" ] for opt , optarg in opts ) {
        println!( _usage );
        sys . exit ( );
        commands = vec![ optarg.iter().map(|opt , optarg| opts if opt| vec![ "-c" , "--command" ] ).collect();
        module_indicated = any ( opt| vec![ "-m" ].iter().map(|opt , optarg| opts );
        cls = _ModuleTarget if module_indicated else _ScriptTarget;
        target = cls ( args [ 0 ] );
        target . check ( );
        sys . argv [ : ] = args;
        pdb = Pdb ( );
        pdb . rcLines . extend ( commands );
        while true  {
        // try {
        pdb . _run ( target );
        if pdb . _user_requested_quit {
        break;
        println!( "The program finished && will be restarted" );
        // } catch  Restart  {
        println!( "Restarting" , target , "with arguments:" );
        println!( "\t" + " " . join ( sys . argv [ 1 : ] ) );
        // } catch  SystemExit  {
        println!( "The program exited via sys.exit(). Exit status:" , end = " " );
        println!( sys . exc_info ( ) [ 1 ] );
        // } catch  SyntaxError  {
        traceback . print_exc ( );
        sys . exit ( 1 );
        // } catch   {
        traceback . print_exc ( );
        println!( "Uncaught exception. Entering post mortem debugging" );
        println!( "Running 'cont' || 'step' will restart the program" );
        t = sys . exc_info ( ) [ 2 ];
        pdb . interaction ( None /* Option */ , t );
        println!( "Post mortem debugger finished. The " + target );
        " will be restarted" );
        fn main() {
        import pdb;
        pdb . main ( );
}

