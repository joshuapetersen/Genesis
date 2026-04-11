//! __main__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::tkinter::{};
// use crate::idlelib::{ColorDelegator, color_config};
// use crate::turtledemo::{__doc__, about_turtledemo};
// use crate::turtle;
// use crate::subprocess;

pub const demo_dir: f64 = os . path . dirname ( os . path . abspath ( __file__ ) );
pub const darwin: &str = sys . platform =="darwin";
pub const STARTUP: u64 = 1;
pub const READY: u64 = 2;
pub const RUNNING: u64 = 3;
pub const DONE: u64 = 4;
pub const EVENTDRIVEN: u64 = 5;
pub const menufont: &str = ("Arial" , 12 , NORMAL );
pub const btnfont: &str = ("Arial" , 12 ,"bold" );
pub const txtfont: &str = ["Lucida Console" , 10 ,"normal" ];
pub const MINIMUM_FONT_SIZE: u64 = 6;
pub const MAXIMUM_FONT_SIZE: u64 = 100;
pub const font_sizes: f64 = [ 8 , 9 , 10 , 11 , 12 , 14 , 18 , 20 , 22 , 24 , 30 ];
pub fn getExampleEntries() {
        return  [ entry [ : -3 ] for entry in os . listdir ( demo_dir ) if;
        entry . endswith ( ".py" ) && entry [ 0 ] != "_" ];
        help_entries = (;
        ( "Turtledemo help" , __doc__ ) ,;
        ( "About turtledemo" , about_turtledemo ) ,;
        ( "About turtle module" , turtle . __doc__ ) ,;
        );
        class DemoWindow ( object ) ;
        pub fn __init__ ( &self, filename = None /* Option */ )  {
        self . root = root = turtle . _root = Tk ( );
        root . title ( "Python turtle-graphics examples" );
        root . wm_protocol ( "WM_DELETE_WINDOW" , self . _destroy );
        if darwin {
        import subprocess;
        subprocess . run (;
        [;
        "osascript" ,;
        "-e" , "tell application "System Events"" ,;
        "-e" , "set frontmost of the first process whose ";
        "unix id == {} to true" . format ( os . getpid ( ) ) ,;
        "-e" , "end tell" ,;
        ] ,;
        stderr = subprocess . DEVNULL ,;
        stdout = subprocess . DEVNULL , );
        root . grid_rowconfigure ( 0 , weight = 1 );
        root . grid_columnconfigure ( 0 , weight = 1 );
        root . grid_columnconfigure ( 1 , minsize = 90 , weight = 1 );
        root . grid_columnconfigure ( 2 , minsize = 90 , weight = 1 );
        root . grid_columnconfigure ( 3 , minsize = 90 , weight = 1 );
        self . mBar = Menu ( root , relief = RAISED , borderwidth = 2 );
        self . mBar . add_cascade ( menu = self . makeLoadDemoMenu ( self . mBar ) ,;
        label = "Examples" , underline = 0 );
        self . mBar . add_cascade ( menu = self . makeFontMenu ( self . mBar ) ,;
        label = "Fontsize" , underline = 0 );
        self . mBar . add_cascade ( menu = self . makeHelpMenu ( self . mBar ) ,;
        label = "Help" , underline = 0 );
        root [ "menu" ] = self . mBar;
        pane = PanedWindow ( root , orient = HORIZONTAL , sashwidth = 5 ,;
        sashrelief = SOLID , bg = "#ddd" );
        pane . add ( self . makeTextFrame ( pane ) );
        pane . add ( self . makeGraphFrame ( pane ) );
        pane . grid ( row = 0 , columnspan = 4 , sticky = "news" );
        self . output_lbl = Label ( root , height = 1 , text = " --- " , bg = "#ddf" ,;
        font = ( "Arial" , 16 , "normal" ) , borderwidth = 2 ,;
        relief = RIDGE );
        if darwin {
        self . start_btn = Button ( root , text = " START " , font = btnfont ,;
        fg = "#00cc22" , command = self . startDemo );
        self . stop_btn = Button ( root , text = " STOP " , font = btnfont ,;
        fg = "#00cc22" , command = self . stopIt );
        self . clear_btn = Button ( root , text = " CLEAR " , font = btnfont ,;
        fg = "#00cc22" , command = self . clearCanvas );
        } else {
        self . start_btn = Button ( root , text = " START " , font = btnfont ,;
        fg = "white" , disabledforeground = "#fed" ,;
        command = self . startDemo );
        self . stop_btn = Button ( root , text = " STOP " , font = btnfont ,;
        fg = "white" , disabledforeground = "#fed" ,;
        command = self . stopIt );
        self . clear_btn = Button ( root , text = " CLEAR " , font = btnfont ,;
        fg = "white" , disabledforeground = "#fed" ,;
        command = self . clearCanvas );
        self . output_lbl . grid ( row = 1 , column = 0 , sticky = "news" , padx = ( 0 , 5 ) );
        self . start_btn . grid ( row = 1 , column = 1 , sticky = "ew" );
        self . stop_btn . grid ( row = 1 , column = 2 , sticky = "ew" );
        self . clear_btn . grid ( row = 1 , column = 3 , sticky = "ew" );
        Percolator ( self . text ) . insertfilter ( ColorDelegator ( ) );
        self . dirty = false;
        self . exitflag = false;
        if filename {
        self . loadfile ( filename );
        self . configGUI ( DISABLED , DISABLED , DISABLED ,;
        "Choose example from menu" , "black" );
        self . state = STARTUP;
        pub fn onResize ( &self, event )  {
        cwidth = self . canvas . winfo_width ( );
        cheight = self . canvas . winfo_height ( );
        self . canvas . xview_moveto ( 0.5 * ( self . canvwidth - cwidth ) / self . canvwidth );
        self . canvas . yview_moveto ( 0.5 * ( self . canvheight - cheight ) / self . canvheight );
        pub fn makeTextFrame ( &self, root )  {
        self . text_frame = text_frame = Frame ( root );
        self . text = text = Text ( text_frame , name = "text" , padx = 5 ,;
        wrap = "none" , width = 45 );
        color_config ( text );
        self . vbar = vbar = Scrollbar ( text_frame , name = "vbar" );
        vbar [ "command" ] = text . yview;
        vbar . pack ( side = LEFT , fill = Y );
        self . hbar = hbar = Scrollbar ( text_frame , name = "hbar" , orient = HORIZONTAL );
        hbar [ "command" ] = text . xview;
        hbar . pack ( side = BOTTOM , fill = X );
        text [ "yscrollcommand" ] = vbar . set;
        text [ "xscrollcommand" ] = hbar . set;
        text [ "font" ] = tuple ( txtfont );
        shortcut = "Command" if darwin else "Control";
        text . bind_all ( "<%s-minus>" % shortcut , self . decrease_size );
        text . bind_all ( "<%s-underscore>" % shortcut , self . decrease_size );
        text . bind_all ( "<%s-equal>" % shortcut , self . increase_size );
        text . bind_all ( "<%s-plus>" % shortcut , self . increase_size );
        text . bind ( "<Control-MouseWheel>" , self . update_mousewheel );
        text . bind ( "<Control-Button-4>" , self . increase_size );
        text . bind ( "<Control-Button-5>" , self . decrease_size );
        text . pack ( side = LEFT , fill = BOTH , expand = 1 );
        return  text_frame;
        pub fn makeGraphFrame ( &self, root )  {
        turtle . _Screen . _root = root;
        self . canvwidth = 1000;
        self . canvheight = 800;
        turtle . _Screen . _canvas = self . canvas = canvas = turtle . ScrolledCanvas (;
        root , 800 , 600 , self . canvwidth , self . canvheight );
        canvas . adjustScrolls ( );
        canvas . _rootwindow . bind ( "<Configure>" , self . onResize );
        canvas . _canvas [ "borderwidth" ] = 0;
        self . screen = screen = turtle . Screen ( );
        turtle . TurtleScreen . __init__ ( screen , canvas );
        turtle . RawTurtle . screens = [ screen ];
        return  canvas;
        pub fn set_txtsize ( &self, size )  {
        txtfont [ 1 ] = size;
        self . text [ "font" ] = tuple ( txtfont );
        self . output_lbl [ "text" ] = "Font size %d" % size;
        pub fn decrease_size ( &self, dummy = None /* Option */ )  {
        self . set_txtsize ( max ( txtfont [ 1 ] - 1 , MINIMUM_FONT_SIZE ) );
        return  "break";
        pub fn increase_size ( &self, dummy = None /* Option */ )  {
        self . set_txtsize ( min ( txtfont [ 1 ] + 1 , MAXIMUM_FONT_SIZE ) );
        return  "break";
        pub fn update_mousewheel ( &self, event )  {
        if ( event . delta < 0 ) == ( !darwin ) {
        return  self . decrease_size ( );
        } else {
        return  self . increase_size ( );
        pub fn configGUI ( &self, start , stop , clear , txt = "" , color = "blue" )  {
        if darwin {
        self . start_btn . config ( state = start );
        self . stop_btn . config ( state = stop );
        self . clear_btn . config ( state = clear );
        } else {
        self . start_btn . config ( state = start ,;
        bg = "#d00" if start == NORMAL else "#fca" );
        self . stop_btn . config ( state = stop ,;
        bg = "#d00" if stop == NORMAL else "#fca" );
        self . clear_btn . config ( state = clear ,;
        bg = "#d00" if clear == NORMAL else "#fca" );
        self . output_lbl . config ( text = txt , fg = color );
        pub fn makeLoadDemoMenu ( &self, master )  {
        menu = Menu ( master );
        for entry in getExampleEntries ( ) .iter() {
        pub fn load ( entry = entry )  {
        self . loadfile ( entry );
        menu . add_command ( label = entry , underline = 0 ,;
        font = menufont , command = load );
        return  menu;
        pub fn makeFontMenu ( &self, master )  {
        menu = Menu ( master );
        menu . add_command ( label = "Decrease (C-'-')" , command = self . decrease_size ,;
        font = menufont );
        menu . add_command ( label = "Increase (C-'+')" , command = self . increase_size ,;
        font = menufont );
        menu . add_separator ( );
        for size in font_sizes .iter() {
        pub fn resize ( size = size )  {
        self . set_txtsize ( size );
        menu . add_command ( label = str ( size ) , underline = 0 ,;
        font = menufont , command = resize );
        return  menu;
        pub fn makeHelpMenu ( &self, master )  {
        menu = Menu ( master );
        for help_label , help_file in help_entries .iter() {
        pub fn show ( help_label = help_label , help_file = help_file )  {
        view_text ( self . root , help_label , help_file );
        menu . add_command ( label = help_label , font = menufont , command = show );
        return  menu;
        pub fn refreshCanvas ( self )  {
        if self . dirty {
        self . screen . clear ( );
        self . dirty = false;
        pub fn loadfile ( &self, filename )  {
        self . clearCanvas ( );
        turtle . TurtleScreen . _RUNNING = false;
        modname = "turtledemo." + filename;
        __import__ ( modname );
        self . module = sys . modules [ modname ];
        // with scope: open ( self . module . __file__ , "r" ) as f  {
        chars = f . read ( );
        self . text . delete ( "1.0" , "end" );
        self . text . insert ( "1.0" , chars );
        self . root . title ( filename + " - a Python turtle graphics example" );
        self . configGUI ( NORMAL , DISABLED , DISABLED ,;
        "Press start button" , "red" );
        self . state = READY;
        pub fn startDemo ( self )  {
        self . refreshCanvas ( );
        self . dirty = true;
        turtle . TurtleScreen . _RUNNING = true;
        self . configGUI ( DISABLED , NORMAL , DISABLED ,;
        "demo running..." , "black" );
        self . screen . clear ( );
        self . screen . mode ( "standard" );
        self . state = RUNNING;
        // try {
        result = self . module . main ( );
        if result == "EVENTLOOP" {
        self . state = EVENTDRIVEN;
        } else {
        self . state = DONE;
        // } catch  turtle . Terminator  {
        if self . root is None /* Option */ {
        return;
        self . state = DONE;
        result = "stopped!";
        if self . state == DONE {
        self . configGUI ( NORMAL , DISABLED , NORMAL ,;
        result );
        } else if self . state == EVENTDRIVEN {
        self . exitflag = true;
        self . configGUI ( DISABLED , NORMAL , DISABLED ,;
        "use mouse/keys || STOP" , "red" );
        pub fn clearCanvas ( self )  {
        self . refreshCanvas ( );
        self . screen . _delete ( "all" );
        self . canvas . config ( cursor = "" );
        self . configGUI ( NORMAL , DISABLED , DISABLED );
        pub fn stopIt ( self )  {
        if self . exitflag {
        self . clearCanvas ( );
        self . exitflag = false;
        self . configGUI ( NORMAL , DISABLED , DISABLED ,;
        "STOPPED!" , "red" );
        turtle . TurtleScreen . _RUNNING = false;
        pub fn _destroy ( self )  {
        turtle . TurtleScreen . _RUNNING = false;
        self . root . destroy ( );
        self . root = None /* Option */;
        pub fn main ( )  {
        demo = DemoWindow ( );
        demo . root . mainloop ( );
        fn main() {
        main ( );
}

