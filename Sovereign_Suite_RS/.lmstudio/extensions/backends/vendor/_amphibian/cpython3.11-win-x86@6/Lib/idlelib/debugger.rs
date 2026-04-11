//! debugger.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::bdb;
// use crate::tkinter::{};
// use crate::idlelib::{macosx};
// use crate::linecache;
// use crate::reprlib;
// use crate::unittest::{main};

pub struct Idb {
    pub gui: String, // TODO: infer type
    pub pyshell: String, // TODO: infer type
    pub idb: String, // TODO: infer type
    pub frame: String, // TODO: infer type
    pub interacting: String, // TODO: infer type
    pub nesting_level: String, // TODO: infer type
    pub stackviewer: String, // TODO: infer type
    pub flist: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub top: String, // TODO: infer type
    pub bframe: String, // TODO: infer type
    pub buttons: String, // TODO: infer type
    pub bcont: String, // TODO: infer type
    pub bstep: String, // TODO: infer type
    pub bnext: String, // TODO: infer type
    pub bret: String, // TODO: infer type
    pub cframe: String, // TODO: infer type
    pub bstack: String, // TODO: infer type
    pub bsource: String, // TODO: infer type
    pub blocals: String, // TODO: infer type
    pub bglobals: String, // TODO: infer type
    pub status: String, // TODO: infer type
    pub error: String, // TODO: infer type
    pub errorbg: String, // TODO: infer type
    pub fstack: String, // TODO: infer type
    pub flocals: String, // TODO: infer type
    pub fglobals: String, // TODO: infer type
    pub localsviewer: String, // TODO: infer type
    pub globalsviewer: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub master: String, // TODO: infer type
    pub title: String, // TODO: infer type
    pub repr: String, // TODO: infer type
    pub label: String, // TODO: infer type
    pub vbar: String, // TODO: infer type
    pub canvas: String, // TODO: infer type
    pub subframe: String, // TODO: infer type
    pub sfid: String, // TODO: infer type
    pub prev_odict: String, // TODO: infer type
}

impl Idb {
}

pub fn _in_rpc_code(frame: &str) {
        "Determine if debugger == within RPC code.";
        if frame . f_code . co_filename . count ( "rpc.py" ) {
        return  true;
        } else {
        prev_frame = frame . f_back;
        if prev_frame is None /* Option */ {
        return  false;
        prev_name = prev_frame . f_code . co_filename;
        if "idlelib" in prev_name && "debugger" in prev_name {
        return  false;
        return  _in_rpc_code ( prev_frame );
        pub fn _frame2message ( frame )  {
        "Return a message string for frame.";
        code = frame . f_code;
        filename = code . co_filename;
        lineno = frame . f_lineno;
        basename = os . path . basename ( filename );
        message = format!("{basename}:{lineno}");
        if code . co_name != "?" {
        message = format!("{message}: {code.co_name}()");
        return  message;
        class Debugger ;
        "The debugger interface.

    This class handles the drawing of the debugger window and
    the interactions with the underlying debugger session.
    ";
        vstack = None /* Option */;
        vsource = None /* Option */;
        vlocals = None /* Option */;
        vglobals = None /* Option */;
        stackviewer = None /* Option */;
        localsviewer = None /* Option */;
        globalsviewer = None /* Option */;
        pub fn __init__ ( &self, pyshell , idb = None /* Option */ )  {
        "Instantiate && draw a debugger window.

        :param pyshell: An instance of the PyShell Window
        :type  pyshell: :class:`idlelib.pyshell.PyShell`

        :param idb: An instance of the IDLE debugger (optional)
        :type  idb: :class:`idlelib.debugger.Idb`
        ";
        if idb is None /* Option */ {
        idb = Idb ( self );
        self . pyshell = pyshell;
        self . idb = idb;
        self . frame = None /* Option */;
        self . make_gui ( );
        self . interacting = false;
        self . nesting_level = 0;
        pub fn run ( &self, * args )  {
        "Run the debugger.";
        if self . nesting_level > 0 {
        self . abort_loop ( );
        self . root . after ( 100 , lambda : self . run ( * args ) );
        return;
        // try {
        self . interacting = true;
        return  self . idb . run ( * args );
        // } finally {
        self . interacting = false;
        pub fn close ( &self, event = None /* Option */ )  {
        "Close the debugger && window.";
        // try {
        self . quit ( );
        // } catch  Exception  {
        // pass
        if self . interacting {
        self . top . bell ( );
        return;
        if self . stackviewer {
        self . stackviewer . close ( ) ; self . stackviewer = None /* Option */;
        self . pyshell . close_debugger ( );
        self . top . destroy ( );
        pub fn make_gui ( self )  {
        "Draw the debugger gui on the screen.";
        pyshell = self . pyshell;
        self . flist = pyshell . flist;
        self . root = root = pyshell . root;
        self . top = top = ListedToplevel ( root );
        self . top . wm_title ( "Debug Control" );
        self . top . wm_iconname ( "Debug" );
        top . wm_protocol ( "WM_DELETE_WINDOW" , self . close );
        self . top . bind ( "<Escape>" , self . close );
        self . bframe = bframe = Frame ( top );
        self . bframe . pack ( anchor = "w" );
        self . buttons = bl = [ ];
        self . bcont = b = Button ( bframe , text = "Go" , command = self . cont );
        bl . append ( b );
        self . bstep = b = Button ( bframe , text = "Step" , command = self . step );
        bl . append ( b );
        self . bnext = b = Button ( bframe , text = "Over" , command = self . next );
        bl . append ( b );
        self . bret = b = Button ( bframe , text = "Out" , command = self . ret );
        bl . append ( b );
        self . bret = b = Button ( bframe , text = "Quit" , command = self . quit );
        bl . append ( b );
        for b in bl .iter() {
        b . configure ( state = "disabled" );
        b . pack ( side = "left" );
        self . cframe = cframe = Frame ( bframe );
        self . cframe . pack ( side = "left" );
        if !self . vstack {
        self . __class__ . vstack = BooleanVar ( top );
        self . vstack . set ( 1 );
        self . bstack = Checkbutton ( cframe ,;
        text = "Stack" , command = self . show_stack , variable = self . vstack );
        self . bstack . grid ( row = 0 , column = 0 );
        if !self . vsource {
        self . __class__ . vsource = BooleanVar ( top );
        self . bsource = Checkbutton ( cframe ,;
        text = "Source" , command = self . show_source , variable = self . vsource );
        self . bsource . grid ( row = 0 , column = 1 );
        if !self . vlocals {
        self . __class__ . vlocals = BooleanVar ( top );
        self . vlocals . set ( 1 );
        self . blocals = Checkbutton ( cframe ,;
        text = "Locals" , command = self . show_locals , variable = self . vlocals );
        self . blocals . grid ( row = 1 , column = 0 );
        if !self . vglobals {
        self . __class__ . vglobals = BooleanVar ( top );
        self . bglobals = Checkbutton ( cframe ,;
        text = "Globals" , command = self . show_globals , variable = self . vglobals );
        self . bglobals . grid ( row = 1 , column = 1 );
        self . status = Label ( top , anchor = "w" );
        self . status . pack ( anchor = "w" );
        self . error = Label ( top , anchor = "w" );
        self . error . pack ( anchor = "w" , fill = "x" );
        self . errorbg = self . error . cget ( "background" );
        self . fstack = Frame ( top , height = 1 );
        self . fstack . pack ( expand = 1 , fill = "both" );
        self . flocals = Frame ( top );
        self . flocals . pack ( expand = 1 , fill = "both" );
        self . fglobals = Frame ( top , height = 1 );
        self . fglobals . pack ( expand = 1 , fill = "both" );
        if self . vstack . get ( ) {
        self . show_stack ( );
        if self . vlocals . get ( ) {
        self . show_locals ( );
        if self . vglobals . get ( ) {
        self . show_globals ( );
        pub fn interaction ( &self, message , frame , info = None /* Option */ )  {
        self . frame = frame;
        self . status . configure ( text = message );
        if info {
        type , value , tb = info;
        // try {
        m1 = type . __name__;
        // } catch  AttributeError  {
        m1 = "%s" % str ( type );
        if value is !None /* Option */ {
        // try {
        m1 = format!("{m1}: {value}");
        // } catch   {
        // pass
        bg = "yellow";
        } else {
        m1 = "";
        tb = None /* Option */;
        bg = self . errorbg;
        self . error . configure ( text = m1 , background = bg );
        sv = self . stackviewer;
        if sv {
        stack , i = self . idb . get_stack ( self . frame , tb );
        sv . load_stack ( stack , i );
        self . show_variables ( 1 );
        if self . vsource . get ( ) {
        self . sync_source_line ( );
        for b in self . buttons .iter() {
        b . configure ( state = "normal" );
        self . top . wakeup ( );
        self . nesting_level + = 1;
        self . root . tk . call ( "vwait" , "::idledebugwait" );
        self . nesting_level - = 1;
        for b in self . buttons .iter() {
        b . configure ( state = "disabled" );
        self . status . configure ( text = "" );
        self . error . configure ( text = "" , background = self . errorbg );
        self . frame = None /* Option */;
        pub fn sync_source_line ( self )  {
        frame = self . frame;
        if !frame {
        return;
        filename , lineno = self . __frame2fileline ( frame );
        if filename [ { : 1 ] + filename [ -1 : ] != "<>" && os . path . exists ( filename ) ; }
        self . flist . gotofileline ( filename , lineno );
        pub fn __frame2fileline ( &self, frame )  {
        code = frame . f_code;
        filename = code . co_filename;
        lineno = frame . f_lineno;
        return  filename , lineno;
        pub fn cont ( self )  {
        self . idb . set_continue ( );
        self . abort_loop ( );
        pub fn step ( self )  {
        self . idb . set_step ( );
        self . abort_loop ( );
        pub fn next ( self )  {
        self . idb . set_next ( self . frame );
        self . abort_loop ( );
        pub fn ret ( self )  {
        self . idb . set_return ( self . frame );
        self . abort_loop ( );
        pub fn quit ( self )  {
        self . idb . set_quit ( );
        self . abort_loop ( );
        pub fn abort_loop ( self )  {
        self . root . tk . call ( "set" , "::idledebugwait" , "1" );
        pub fn show_stack ( self )  {
        if !self . stackviewer && self . vstack . get ( ) {
        self . stackviewer = sv = StackViewer ( self . fstack , self . flist , self );
        if self . frame {
        stack , i = self . idb . get_stack ( self . frame , None /* Option */ );
        sv . load_stack ( stack , i );
        } else {
        sv = self . stackviewer;
        if sv && !self . vstack . get ( ) {
        self . stackviewer = None /* Option */;
        sv . close ( );
        self . fstack [ "height" ] = 1;
        pub fn show_source ( self )  {
        if self . vsource . get ( ) {
        self . sync_source_line ( );
        pub fn show_frame ( &self, stackitem )  {
        self . frame = stackitem [ 0 ];
        self . show_variables ( );
        pub fn show_locals ( self )  {
        lv = self . localsviewer;
        if self . vlocals . get ( ) {
        if !lv {
        self . localsviewer = NamespaceViewer ( self . flocals , "Locals" );
        } else {
        if lv {
        self . localsviewer = None /* Option */;
        lv . close ( );
        self . flocals [ "height" ] = 1;
        self . show_variables ( );
        pub fn show_globals ( self )  {
        gv = self . globalsviewer;
        if self . vglobals . get ( ) {
        if !gv {
        self . globalsviewer = NamespaceViewer ( self . fglobals , "Globals" );
        } else {
        if gv {
        self . globalsviewer = None /* Option */;
        gv . close ( );
        self . fglobals [ "height" ] = 1;
        self . show_variables ( );
        pub fn show_variables ( &self, force = 0 )  {
        lv = self . localsviewer;
        gv = self . globalsviewer;
        frame = self . frame;
        if !frame {
        ldict = gdict = None /* Option */;
        } else {
        ldict = frame . f_locals;
        gdict = frame . f_globals;
        if lv && gv && ldict is gdict {
        ldict = None /* Option */;
        if lv {
        lv . load_dict ( ldict , force , self . pyshell . interp . rpcclt );
        if gv {
        gv . load_dict ( gdict , force , self . pyshell . interp . rpcclt );
        pub fn set_breakpoint ( &self, filename , lineno )  {
        "Set a filename-lineno breakpoint in the debugger.

        Called from self.load_breakpoints && EW.setbreakpoint
        ";
        self . idb . set_break ( filename , lineno );
        pub fn clear_breakpoint ( &self, filename , lineno )  {
        self . idb . clear_break ( filename , lineno );
        pub fn clear_file_breaks ( &self, filename )  {
        self . idb . clear_all_file_breaks ( filename );
        pub fn load_breakpoints ( self )  {
        "Load PyShellEditorWindow breakpoints into subprocess debugger.";
        for editwin in self . pyshell . flist . inversedict .iter() {
        filename = editwin . io . filename;
        // try {
        for lineno in editwin . breakpoints .iter() {
        self . set_breakpoint ( filename , lineno );
        // } catch  AttributeError  {
        continue;
        class StackViewer ( ScrolledList ) ;
        "Code stack viewer for debugger GUI.";
        pub fn __init__ ( &self, master , flist , gui )  {
        if macosx . isAquaTk ( ) {
        ScrolledList . __init__ ( self , master );
        } else {
        ScrolledList . __init__ ( self , master , width = 80 );
        self . flist = flist;
        self . gui = gui;
        self . stack = [ ];
        pub fn load_stack ( &self, stack , index = None /* Option */ )  {
        self . stack = stack;
        self . clear ( );
        for i in range ( len ( stack ) ) .iter() {
        frame , lineno = stack [ i ];
        // try {
        modname = frame . f_globals [ "__name__" ];
        // } catch   {
        modname = "?";
        code = frame . f_code;
        filename = code . co_filename;
        funcname = code . co_name;
        import linecache;
        sourceline = linecache . getline ( filename , lineno );
        sourceline = sourceline . strip ( );
        if funcname in ( "?" , "" , None /* Option */ ) {
        item = "%s, line %d: %s" % ( modname , lineno , sourceline );
        } else {
        item = "%s.%s(), line %d: %s" % ( modname , funcname ,;
        lineno , sourceline );
        if i == index {
        item = "> " + item;
        self . append ( item );
        if index is !None /* Option */ {
        self . select ( index );
        pub fn popup_event ( &self, event )  {
        "Override base method.";
        if self . stack {
        return  ScrolledList . popup_event ( self , event );
        pub fn fill_menu ( self )  {
        "Override base method.";
        menu = self . menu;
        menu . add_command ( label = "Go to source line" ,;
        command = self . goto_source_line );
        menu . add_command ( label = "Show stack frame" ,;
        command = self . show_stack_frame );
        pub fn on_select ( &self, index )  {
        "Override base method.";
        if 0 <= index < len ( self . stack ) {
        self . gui . show_frame ( self . stack [ index ] );
        pub fn on_double ( &self, index )  {
        "Override base method.";
        self . show_source ( index );
        pub fn goto_source_line ( self )  {
        index = self . listbox . index ( "active" );
        self . show_source ( index );
        pub fn show_stack_frame ( self )  {
        index = self . listbox . index ( "active" );
        if 0 <= index < len ( self . stack ) {
        self . gui . show_frame ( self . stack [ index ] );
        pub fn show_source ( &self, index )  {
        if !( 0 <= index < len ( self . stack ) ) {
        return;
        frame , lineno = self . stack [ index ];
        code = frame . f_code;
        filename = code . co_filename;
        if os . path . isfile ( filename ) {
        edit = self . flist . open ( filename );
        if edit {
        edit . gotoline ( lineno );
        class NamespaceViewer ;
        "Global/local namespace viewer for debugger GUI.";
        pub fn __init__ ( &self, master , title , odict = None /* Option */ )  {
        width = 0;
        height = 40;
        if odict {
        height = 20 * len ( odict );
        self . master = master;
        self . title = title;
        import reprlib;
        self . repr = reprlib . Repr ( );
        self . repr . maxstring = 60;
        self . repr . maxother = 60;
        self . frame = frame = Frame ( master );
        self . frame . pack ( expand = 1 , fill = "both" );
        self . label = Label ( frame , text = title , borderwidth = 2 , relief = "groove" );
        self . label . pack ( fill = "x" );
        self . vbar = vbar = Scrollbar ( frame , name = "vbar" );
        vbar . pack ( side = "right" , fill = "y" );
        self . canvas = canvas = Canvas ( frame ,;
        height = min ( 300 , max ( 40 , height ) ) ,;
        scrollregion = ( 0 , 0 , width , height ) );
        canvas . pack ( side = "left" , fill = "both" , expand = 1 );
        vbar [ "command" ] = canvas . yview;
        canvas [ "yscrollcommand" ] = vbar . set;
        self . subframe = subframe = Frame ( canvas );
        self . sfid = canvas . create_window ( 0 , 0 , window = subframe , anchor = "nw" );
        self . load_dict ( odict );
        prev_odict = -1;
        pub fn load_dict ( &self, odict , force = 0 , rpc_client = None /* Option */ )  {
        if odict is self . prev_odict && !force {
        return;
        subframe = self . subframe;
        frame = self . frame;
        for c in list ( subframe . children . values ( ) ) .iter() {
        c . destroy ( );
        self . prev_odict = None /* Option */;
        if !odict {
        l = Label ( subframe , text = "None /* Option */" );
        l . grid ( row = 0 , column = 0 );
        } else {
        keys_list = odict . keys ( );
        names = sorted ( keys_list );
        row = 0;
        for name in names .iter() {
        value = odict [ name ];
        svalue = self . repr . repr ( value );
        if rpc_client {
        svalue = svalue [ 1 : -1 ];
        l = Label ( subframe , text = name );
        l . grid ( row = row , column = 0 , sticky = "nw" );
        l = Entry ( subframe , width = 0 , borderwidth = 0 );
        l . insert ( 0 , svalue );
        l . grid ( row = row , column = 1 , sticky = "nw" );
        row = row + 1;
        self . prev_odict = odict;
        subframe . update_idletasks ( );
        width = subframe . winfo_reqwidth ( );
        height = subframe . winfo_reqheight ( );
        canvas = self . canvas;
        self . canvas [ "scrollregion" ] = ( 0 , 0 , width , height );
        if height > 300 {
        canvas [ "height" ] = 300;
        frame . pack ( expand = 1 );
        } else {
        canvas [ "height" ] = height;
        frame . pack ( expand = 0 );
        pub fn close ( self )  {
        self . frame . destroy ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_debugger" , verbosity = 2 , exit = false );
}

