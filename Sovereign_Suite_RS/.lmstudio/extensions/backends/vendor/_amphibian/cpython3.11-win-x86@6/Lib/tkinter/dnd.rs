//! dnd.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter;

pub const __all__: &str = ["dnd_start" ,"DndHandler" ];
pub fn dnd_start(source: &str, event: &str) {
        h = DndHandler ( source , event );
        if h . root is !None /* Option */ {
        return  h;
        } else {
        return;
        class DndHandler ;
        root = None /* Option */;
        pub fn __init__ ( &self, source , event )  {
        if event . num > 5 {
        return;
        root = event . widget . _root ( );
        // try {
        root . __dnd;
        return;
        // } catch  AttributeError  {
        root . __dnd = self;
        self . root = root;
        self . source = source;
        self . target = None /* Option */;
        self . initial_button = button = event . num;
        self . initial_widget = widget = event . widget;
        self . release_pattern = "<B%d-ButtonRelease-%d>" % ( button , button );
        self . save_cursor = widget [ "cursor" ] || "";
        widget . bind ( self . release_pattern , self . on_release );
        widget . bind ( "<Motion>" , self . on_motion );
        widget [ "cursor" ] = "hand2";
        pub fn __del__ ( self )  {
        root = self . root;
        self . root = None /* Option */;
        if root is !None /* Option */ {
        // try {
        del root . __dnd;
        // } catch  AttributeError  {
        // pass
        pub fn on_motion ( &self, event )  {
        x , y = event . x_root , event . y_root;
        target_widget = self . initial_widget . winfo_containing ( x , y );
        source = self . source;
        new_target = None /* Option */;
        while target_widget is !None /* Option */  {
        // try {
        attr = target_widget . dnd_accept;
        // } catch  AttributeError  {
        // pass
        } else {
        new_target = attr ( source , event );
        if new_target is !None /* Option */ {
        break;
        target_widget = target_widget . master;
        old_target = self . target;
        if old_target is new_target {
        if old_target is !None /* Option */ {
        old_target . dnd_motion ( source , event );
        } else {
        if old_target is !None /* Option */ {
        self . target = None /* Option */;
        old_target . dnd_leave ( source , event );
        if new_target is !None /* Option */ {
        new_target . dnd_enter ( source , event );
        self . target = new_target;
        pub fn on_release ( &self, event )  {
        self . finish ( event , 1 );
        pub fn cancel ( &self, event = None /* Option */ )  {
        self . finish ( event , 0 );
        pub fn finish ( &self, event , commit = 0 )  {
        target = self . target;
        source = self . source;
        widget = self . initial_widget;
        root = self . root;
        // try {
        del root . __dnd;
        self . initial_widget . unbind ( self . release_pattern );
        self . initial_widget . unbind ( "<Motion>" );
        widget [ "cursor" ] = self . save_cursor;
        self . target = self . source = self . initial_widget = self . root = None /* Option */;
        if target is !None /* Option */ {
        if commit {
        target . dnd_commit ( source , event );
        } else {
        target . dnd_leave ( source , event );
        // } finally {
        source . dnd_end ( target , event );
        class Icon ;
        pub fn __init__ ( &self, name )  {
        self . name = name;
        self . canvas = self . label = self . id = None /* Option */;
        pub fn attach ( &self, canvas , x = 10 , y = 10 )  {
        if canvas is self . canvas {
        self . canvas . coords ( self . id , x , y );
        return;
        if self . canvas is !None /* Option */ {
        self . detach ( );
        if canvas is None /* Option */ {
        return;
        label = tkinter . Label ( canvas , text = self . name ,;
        borderwidth = 2 , relief = "raised" );
        id = canvas . create_window ( x , y , window = label , anchor = "nw" );
        self . canvas = canvas;
        self . label = label;
        self . id = id;
        label . bind ( "<ButtonPress>" , self . press );
        pub fn detach ( self )  {
        canvas = self . canvas;
        if canvas is None /* Option */ {
        return;
        id = self . id;
        label = self . label;
        self . canvas = self . label = self . id = None /* Option */;
        canvas . delete ( id );
        label . destroy ( );
        pub fn press ( &self, event )  {
        if dnd_start ( self , event ) {
        self . x_off = event . x;
        self . y_off = event . y;
        self . x_orig , self . y_orig = self . canvas . coords ( self . id );
        pub fn move ( &self, event )  {
        x , y = self . where ( self . canvas , event );
        self . canvas . coords ( self . id , x , y );
        pub fn putback ( self )  {
        self . canvas . coords ( self . id , self . x_orig , self . y_orig );
        pub fn where ( &self, canvas , event )  {
        x_org = canvas . winfo_rootx ( );
        y_org = canvas . winfo_rooty ( );
        x = event . x_root - x_org;
        y = event . y_root - y_org;
        return  x - self . x_off , y - self . y_off;
        pub fn dnd_end ( &self, target , event )  {
        // pass
        class Tester ;
        pub fn __init__ ( &self, root )  {
        self . top = tkinter . Toplevel ( root );
        self . canvas = tkinter . Canvas ( self . top , width = 100 , height = 100 );
        self . canvas . pack ( fill = "both" , expand = 1 );
        self . canvas . dnd_accept = self . dnd_accept;
        pub fn dnd_accept ( &self, source , event )  {
        return  self;
        pub fn dnd_enter ( &self, source , event )  {
        self . canvas . focus_set ( );
        x , y = source . where ( self . canvas , event );
        x1 , y1 , x2 , y2 = source . canvas . bbox ( source . id );
        dx , dy = x2 - x1 , y2 - y1;
        self . dndid = self . canvas . create_rectangle ( x , y , x + dx , y + dy );
        self . dnd_motion ( source , event );
        pub fn dnd_motion ( &self, source , event )  {
        x , y = source . where ( self . canvas , event );
        x1 , y1 , x2 , y2 = self . canvas . bbox ( self . dndid );
        self . canvas . move ( self . dndid , x - x1 , y - y1 );
        pub fn dnd_leave ( &self, source , event )  {
        self . top . focus_set ( );
        self . canvas . delete ( self . dndid );
        self . dndid = None /* Option */;
        pub fn dnd_commit ( &self, source , event )  {
        self . dnd_leave ( source , event );
        x , y = source . where ( self . canvas , event );
        source . attach ( self . canvas , x , y );
        pub fn test ( )  {
        root = tkinter . Tk ( );
        root . geometry ( "+1+1" );
        tkinter . Button ( command = root . quit , text = "Quit" ) . pack ( );
        t1 = Tester ( root );
        t1 . top . geometry ( "+1+60" );
        t2 = Tester ( root );
        t2 . top . geometry ( "+120+60" );
        t3 = Tester ( root );
        t3 . top . geometry ( "+240+60" );
        i1 = Icon ( "ICON1" );
        i2 = Icon ( "ICON2" );
        i3 = Icon ( "ICON3" );
        i1 . attach ( t1 . canvas );
        i2 . attach ( t2 . canvas );
        i3 . attach ( t3 . canvas );
        root . mainloop ( );
        fn main() {
        test ( );
}

