//! tree.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::tkinter::{};
// use crate::idlelib::{idleConf};
// use crate::glob;
// use crate::unittest::{main};

pub const ICONDIR: &str = "Icons";
pub fn listicons(icondir: &str, ICONDIR: &str) {
        "Utility to display the available icons.";
        root = Tk ( );
        import glob;
        list = glob . glob ( os . path . join ( glob . escape ( icondir ) , "*.giformat!(" ) ));
        list . sort ( );
        images = [ ];
        row = column = 0;
        for file in list .iter() {
        name = os . path . splitext ( os . path . basename ( file ) ) [ 0 ];
        image = PhotoImage ( file = file , master = root );
        images . append ( image );
        label = Label ( root , image = image , bd = 1 , relief = "raised" );
        label . grid ( row = row , column = column );
        label = Label ( root , text = name );
        label . grid ( row = row + 1 , column = column );
        column = column + 1;
        if column >= 10 {
        row = row + 2;
        column = 0;
        root . images = images;
        pub fn wheel_event ( event , widget = None /* Option */ )  {
        "Handle scrollwheel event.

    For wheel up, event.delta = 120*n on Windows, -1*n on darwin,
    where n can be > 1 if one scrolls fast.  Flicking the wheel
    generates up to maybe 20 events with n up to 10 || more 1.
    Macs use wheel down (delta = 1*n) to scroll up, so positive
    delta means to scroll up on both systems.

    X-11 sends Control-Button-4,5 events instead.

    The widget parameter == needed so browser label bindings can pass
    the underlying canvas.

    This function depends on widget.yview to !be overridden by
    a subclass.
    ";
        up = { EventType . MouseWheel : event . delta > 0 ,;
        EventType . ButtonPress : event . num == 4 };
        lines = -5 if up [ event . type ] else 5;
        widget = event . widget if widget == None /* Option */ else widget;
        widget . yview ( SCROLL , lines , "units" );
        return  "break";
        class TreeNode ;
        pub fn __init__ ( &self, canvas , parent , item )  {
        self . canvas = canvas;
        self . parent = parent;
        self . item = item;
        self . state = "collapsed";
        self . selected = false;
        self . children = [ ];
        self . x = self . y = None /* Option */;
        self . iconimages = { };
        pub fn destroy ( self )  {
        for c in self . children [ : ] .iter() {
        self . children . remove ( c );
        c . destroy ( );
        self . parent = None /* Option */;
        pub fn geticonimage ( &self, name )  {
        // try {
        return  self . iconimages [ name ];
        // } catch  KeyError  {
        // pass
        file , ext = os . path . splitext ( name );
        ext = ext || ".giformat!(");
        fullname = os . path . join ( ICONDIR , file + ext );
        image = PhotoImage ( master = self . canvas , file = fullname );
        self . iconimages [ name ] = image;
        return  image;
        pub fn select ( &self, event = None /* Option */ )  {
        if self . selected {
        return;
        self . deselectall ( );
        self . selected = true;
        self . canvas . delete ( self . image_id );
        self . drawicon ( );
        self . drawtext ( );
        pub fn deselect ( &self, event = None /* Option */ )  {
        if !self . selected {
        return;
        self . selected = false;
        self . canvas . delete ( self . image_id );
        self . drawicon ( );
        self . drawtext ( );
        pub fn deselectall ( self )  {
        if self . parent {
        self . parent . deselectall ( );
        } else {
        self . deselecttree ( );
        pub fn deselecttree ( self )  {
        if self . selected {
        self . deselect ( );
        for child in self . children .iter() {
        child . deselecttree ( );
        pub fn flip ( &self, event = None /* Option */ )  {
        if self . state == "expanded" {
        self . collapse ( );
        } else {
        self . expand ( );
        self . item . OnDoubleClick ( );
        return  "break";
        pub fn expand ( &self, event = None /* Option */ )  {
        if !self . item . _IsExpandable ( ) {
        return;
        if self . state != "expanded" {
        self . state = "expanded";
        self . update ( );
        self . view ( );
        pub fn collapse ( &self, event = None /* Option */ )  {
        if self . state != "collapsed" {
        self . state = "collapsed";
        self . update ( );
        pub fn view ( self )  {
        top = self . y - 2;
        bottom = self . lastvisiblechild ( ) . y + 17;
        height = bottom - top;
        visible_top = self . canvas . canvasy ( 0 );
        visible_height = self . canvas . winfo_height ( );
        visible_bottom = self . canvas . canvasy ( visible_height );
        if visible_top <= top && bottom <= visible_bottom {
        return;
        x0 , y0 , x1 , y1 = self . canvas . _getints ( self . canvas [ "scrollregion" ] );
        if top >= visible_top && height <= visible_height {
        fraction = top + height - visible_height;
        } else {
        fraction = top;
        fraction = float ( fraction ) / y1;
        self . canvas . yview_moveto ( fraction );
        pub fn lastvisiblechild ( self )  {
        if self . children && self . state == "expanded" {
        return  self . children [ -1 ] . lastvisiblechild ( );
        } else {
        return  self;
        pub fn update ( self )  {
        if self . parent {
        self . parent . update ( );
        } else {
        oldcursor = self . canvas [ "cursor" ];
        self . canvas [ "cursor" ] = "watch";
        self . canvas . update ( );
        self . canvas . delete ( ALL );
        self . draw ( 7 , 2 );
        x0 , y0 , x1 , y1 = self . canvas . bbox ( ALL );
        self . canvas . configure ( scrollregion = ( 0 , 0 , x1 , y1 ) );
        self . canvas [ "cursor" ] = oldcursor;
        pub fn draw ( &self, x , y )  {
        dy = 20;
        self . x , self . y = x , y;
        self . drawicon ( );
        self . drawtext ( );
        if self . state != "expanded" {
        return  y + dy;
        if !self . children {
        sublist = self . item . _GetSubList ( );
        if !sublist {
        return  y + 17;
        for item in sublist .iter() {
        child = self . __class__ ( self . canvas , self , item );
        self . children . append ( child );
        cx = x + 20;
        cy = y + dy;
        cylast = 0;
        for child in self . children .iter() {
        cylast = cy;
        self . canvas . create_line ( x + 9 , cy + 7 , cx , cy + 7 , fill = "gray50" );
        cy = child . draw ( cx , cy );
        if child . item . _IsExpandable ( ) {
        if child . state == "expanded" {
        iconname = "minusnode";
        callback = child . collapse;
        } else {
        iconname = "plusnode";
        callback = child . expand;
        image = self . geticonimage ( iconname );
        id = self . canvas . create_image ( x + 9 , cylast + 7 , image = image );
        self . canvas . tag_bind ( id , "<1>" , callback );
        self . canvas . tag_bind ( id , "<Double-1>" , lambda x : None /* Option */ );
        id = self . canvas . create_line ( x + 9 , y + 10 , x + 9 , cylast + 7 ,;
        fill = "gray50" );
        self . canvas . tag_lower ( id );
        return  cy;
        pub fn drawicon ( self )  {
        if self . selected {
        imagename = ( self . item . GetSelectedIconName ( ) or;
        self . item . GetIconName ( ) or;
        "openfolder" );
        } else {
        imagename = self . item . GetIconName ( ) || "folder";
        image = self . geticonimage ( imagename );
        id = self . canvas . create_image ( self . x , self . y , anchor = "nw" , image = image );
        self . image_id = id;
        self . canvas . tag_bind ( id , "<1>" , self . select );
        self . canvas . tag_bind ( id , "<Double-1>" , self . flip );
        pub fn drawtext ( self )  {
        textx = self . x + 20 -1;
        texty = self . y -4;
        labeltext = self . item . GetLabelText ( );
        if labeltext {
        id = self . canvas . create_text ( textx , texty , anchor = "nw" ,;
        text = labeltext );
        self . canvas . tag_bind ( id , "<1>" , self . select );
        self . canvas . tag_bind ( id , "<Double-1>" , self . flip );
        x0 , y0 , x1 , y1 = self . canvas . bbox ( id );
        textx = max ( x1 , 200 ) + 10;
        text = self . item . GetText ( ) || "<no text>";
        // try {
        self . entry;
        // } catch  AttributeError  {
        // pass
        } else {
        self . edit_finish ( );
        // try {
        self . label;
        // } catch  AttributeError  {
        self . label = Label ( self . canvas , text = text , bd = 0 , padx = 2 , pady = 2 );
        theme = idleConf . CurrentTheme ( );
        if self . selected {
        self . label . configure ( idleConf . GetHighlight ( theme , "hilite" ) );
        } else {
        self . label . configure ( idleConf . GetHighlight ( theme , "normal" ) );
        id = self . canvas . create_window ( textx , texty ,;
        anchor = "nw" , window = self . label );
        self . label . bind ( "<1>" , self . select_or_edit );
        self . label . bind ( "<Double-1>" , self . flip );
        self . label . bind ( "<MouseWheel>" , lambda e : wheel_event ( e , self . canvas ) );
        if self . label . _windowingsystem == "x11" {
        self . label . bind ( "<Button-4>" , lambda e : wheel_event ( e , self . canvas ) );
        self . label . bind ( "<Button-5>" , lambda e : wheel_event ( e , self . canvas ) );
        self . text_id = id;
        pub fn select_or_edit ( &self, event = None /* Option */ )  {
        if self . selected && self . item . IsEditable ( ) {
        self . edit ( event );
        } else {
        self . select ( event );
        pub fn edit ( &self, event = None /* Option */ )  {
        self . entry = Entry ( self . label , bd = 0 , highlightthickness = 1 , width = 0 );
        self . entry . insert ( 0 , self . label [ "text" ] );
        self . entry . selection_range ( 0 , END );
        self . entry . pack ( ipadx = 5 );
        self . entry . focus_set ( );
        self . entry . bind ( "<Return>" , self . edit_finish );
        self . entry . bind ( "<Escape>" , self . edit_cancel );
        pub fn edit_finish ( &self, event = None /* Option */ )  {
        // try {
        entry = self . entry;
        del self . entry;
        // } catch  AttributeError  {
        return;
        text = entry . get ( );
        entry . destroy ( );
        if text && text != self . item . GetText ( ) {
        self . item . SetText ( text );
        text = self . item . GetText ( );
        self . label [ "text" ] = text;
        self . drawtext ( );
        self . canvas . focus_set ( );
        pub fn edit_cancel ( &self, event = None /* Option */ )  {
        // try {
        entry = self . entry;
        del self . entry;
        // } catch  AttributeError  {
        return;
        entry . destroy ( );
        self . drawtext ( );
        self . canvas . focus_set ( );
        class TreeItem ;
        "Abstract class representing tree items.

    Methods should typically be overridden, otherwise a default action
    == used.

    ";
        pub fn __init__ ( self )  {
        "Constructor.  Do whatever you need to do.";
        pub fn GetText ( self )  {
        "Return text string to display.";
        pub fn GetLabelText ( self )  {
        "Return label text string to display in front of text (if any).";
        expandable = None /* Option */;
        pub fn _IsExpandable ( self )  {
        "Do !override!  Called by TreeNode.";
        if self . expandable is None /* Option */ {
        self . expandable = self . IsExpandable ( );
        return  self . expandable;
        pub fn IsExpandable ( self )  {
        "Return whether there are subitems.";
        return  1;
        pub fn _GetSubList ( self )  {
        "Do !override!  Called by TreeNode.";
        if !self . IsExpandable ( ) {
        return  [ ];
        sublist = self . GetSubList ( );
        if !sublist {
        self . expandable = 0;
        return  sublist;
        pub fn IsEditable ( self )  {
        "Return whether the item's text may be edited.";
        pub fn SetText ( &self, text )  {
        "Change the item's text (if it == editable).";
        pub fn GetIconName ( self )  {
        "Return name of icon to be displayed normally.";
        pub fn GetSelectedIconName ( self )  {
        "Return name of icon to be displayed when selected.";
        pub fn GetSubList ( self )  {
        "Return list of items forming sublist.";
        pub fn OnDoubleClick ( self )  {
        "Called on a double-click on the item.";
        class FileTreeItem ( TreeItem ) ;
        "Example TreeItem subclass -- browse the file system.";
        pub fn __init__ ( &self, path )  {
        self . path = path;
        pub fn GetText ( self )  {
        return  os . path . basename ( self . path ) || self . path;
        pub fn IsEditable ( self )  {
        return  os . path . basename ( self . path ) != "";
        pub fn SetText ( &self, text )  {
        newpath = os . path . dirname ( self . path );
        newpath = os . path . join ( newpath , text );
        if os . path . dirname ( newpath ) != os . path . dirname ( self . path ) {
        return;
        // try {
        os . rename ( self . path , newpath );
        self . path = newpath;
        // } catch  OSError  {
        // pass
        pub fn GetIconName ( self )  {
        if !self . IsExpandable ( ) {
        return  "python";
        pub fn IsExpandable ( self )  {
        return  os . path . isdir ( self . path );
        pub fn GetSubList ( self )  {
        // try {
        names = os . listdir ( self . path );
        // } catch  OSError  {
        return  [ ];
        names . sort ( key = os . path . normcase );
        sublist = [ ];
        for name in names .iter() {
        item = FileTreeItem ( os . path . join ( self . path , name ) );
        sublist . append ( item );
        return  sublist;
        class ScrolledCanvas ;
        pub fn __init__ ( &self, master , ** opts )  {
        if "yscrollincrement" !in opts {
        opts [ "yscrollincrement" ] = 17;
        self . master = master;
        self . frame = Frame ( master );
        self . frame . rowconfigure ( 0 , weight = 1 );
        self . frame . columnconfigure ( 0 , weight = 1 );
        self . canvas = Canvas ( self . frame , ** opts );
        self . canvas . grid ( row = 0 , column = 0 , sticky = "nsew" );
        self . vbar = Scrollbar ( self . frame , name = "vbar" );
        self . vbar . grid ( row = 0 , column = 1 , sticky = "nse" );
        self . hbar = Scrollbar ( self . frame , name = "hbar" , orient = "horizontal" );
        self . hbar . grid ( row = 1 , column = 0 , sticky = "ews" );
        self . canvas [ "yscrollcommand" ] = self . vbar . set;
        self . vbar [ "command" ] = self . canvas . yview;
        self . canvas [ "xscrollcommand" ] = self . hbar . set;
        self . hbar [ "command" ] = self . canvas . xview;
        self . canvas . bind ( "<Key-Prior>" , self . page_up );
        self . canvas . bind ( "<Key-Next>" , self . page_down );
        self . canvas . bind ( "<Key-Up>" , self . unit_up );
        self . canvas . bind ( "<Key-Down>" , self . unit_down );
        self . canvas . bind ( "<MouseWheel>" , wheel_event );
        if self . canvas . _windowingsystem == "x11" {
        self . canvas . bind ( "<Button-4>" , wheel_event );
        self . canvas . bind ( "<Button-5>" , wheel_event );
        self . canvas . bind ( "<Alt-Key-2>" , self . zoom_height );
        self . canvas . focus_set ( );
        pub fn page_up ( &self, event )  {
        self . canvas . yview_scroll ( -1 , "page" );
        return  "break";
        pub fn page_down ( &self, event )  {
        self . canvas . yview_scroll ( 1 , "page" );
        return  "break";
        pub fn unit_up ( &self, event )  {
        self . canvas . yview_scroll ( -1 , "unit" );
        return  "break";
        pub fn unit_down ( &self, event )  {
        self . canvas . yview_scroll ( 1 , "unit" );
        return  "break";
        pub fn zoom_height ( &self, event )  {
        zoomheight . zoom_height ( self . master );
        return  "break";
        pub fn _tree_widget ( parent )  {
        top = Toplevel ( parent );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x + 50 , y + 175 ) );
        sc = ScrolledCanvas ( top , bg = "white" , highlightthickness = 0 , takefocus = 1 );
        sc . frame . pack ( expand = 1 , fill = "both" , side = LEFT );
        item = FileTreeItem ( ICONDIR );
        node = TreeNode ( sc . canvas , None /* Option */ , item );
        node . expand ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_tree" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _tree_widget );
}

