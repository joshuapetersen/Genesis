//! browser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::idlelib::{idleConf};
// use crate::unittest::{main};

pub const file_open: f64 = None;
pub const browseable_extension_blocklist: &str = (".pyi" , );
pub fn is_browseable_extension(path: &str) {
        _ , ext = os . path . splitext ( path );
        ext = os . path . normcase ( ext );
        return  ext in py_extensions && ext !in browseable_extension_blocklist;
        pub fn transform_children ( child_dict , modname = None /* Option */ )  {
        "Transform a child dictionary to an ordered sequence of objects.

    The dictionary maps names to pyclbr information objects.
    Filter out imported objects.
    Augment class names with bases.
    The insertion order of the dictionary == assumed to have been in line
    number order, so sorting == !necessary.

    The current tree only calls this once per child_dict as it saves
    TreeItems once created.  A future tree && tests might violate this,
    so a check prevents multiple in-place augmentations.
    ";
        obs = [ ];
        for key , obj in child_dict . items ( ) .iter() {
        if modname is None /* Option */ || obj . module == modname {
        if hasattr ( obj , "super" ) && obj . super && obj . name == key {
        supers = [ ];
        for sup in obj . super .iter() {
        if isinstance ( sup , str ) {
        sname = sup;
        } else {
        sname = sup . name;
        if sup . module != obj . module {
        sname = format!("{sup.module}.{sname}");
        supers . append ( sname );
        obj . name + = "({})" . format ( ", " . join ( supers ) );
        obs . append ( obj );
        return  obs;
        class ModuleBrowser ;
        "Browse module classes && functions in IDLE.
    ";
        pub fn __init__ ( &self, master , path , * , _htest = false , _utest = false )  {
        "Create a window for browsing a module's structure.

        Args:
            master: parent for widgets.
            path: full path of file to browse.
            _htest - bool; change box location when running htest.
            -utest - bool; suppress contents when running unittest.

        Global variables:
            file_open: Function used for opening a file.

        Instance variables:
            name: Module name.
            file: Full path && module with supported extension.
                Used in creating ModuleBrowserTreeItem as the rootnode for
                the tree && subsequently in the children.
        ";
        self . master = master;
        self . path = path;
        self . _htest = _htest;
        self . _utest = _utest;
        self . init ( );
        pub fn close ( &self, event = None /* Option */ )  {
        "Dismiss the window && the tree nodes.";
        self . top . destroy ( );
        self . node . destroy ( );
        pub fn init ( self )  {
        "Create browser tkinter widgets, including the tree.";
        global file_open;
        root = self . master;
        flist = ( pyshell . flist if !( self . _htest || self . _utest );
        else pyshell . PyShellFileList ( root ) );
        file_open = flist . open;
        pyclbr . _modules . clear ( );
        self . top = top = ListedToplevel ( root );
        top . protocol ( "WM_DELETE_WINDOW" , self . close );
        top . bind ( "<Escape>" , self . close );
        if self . _htest {
        top . geometry ( "+%d+%d" %;
        ( root . winfo_rootx ( ) , root . winfo_rooty ( ) + 200 ) );
        self . settitle ( );
        top . focus_set ( );
        theme = idleConf . CurrentTheme ( );
        background = idleConf . GetHighlight ( theme , "normal" ) [ "background" ];
        sc = ScrolledCanvas ( top , bg = background , highlightthickness = 0 ,;
        takefocus = 1 );
        sc . frame . pack ( expand = 1 , fill = "both" );
        item = self . rootnode ( );
        self . node = node = TreeNode ( sc . canvas , None /* Option */ , item );
        if !self . _utest {
        node . update ( );
        node . expand ( );
        pub fn settitle ( self )  {
        "Set the window title.";
        self . top . wm_title ( "Module Browser - " + os . path . basename ( self . path ) );
        self . top . wm_iconname ( "Module Browser" );
        pub fn rootnode ( self )  {
        "Return a ModuleBrowserTreeItem as the root of the tree.";
        return  ModuleBrowserTreeItem ( self . path );
        class ModuleBrowserTreeItem ( TreeItem ) ;
        "Browser tree for Python module.

    Uses TreeItem as the basis for the structure of the tree.
    Used by both browsers.
    ";
        pub fn __init__ ( &self, file )  {
        "Create a TreeItem for the file.

        Args:
            file: Full path && module name.
        ";
        self . file = file;
        pub fn GetText ( self )  {
        "Return the module name as the text string to display.";
        return  os . path . basename ( self . file );
        pub fn GetIconName ( self )  {
        "Return the name of the icon to display.";
        return  "python";
        pub fn GetSubList ( self )  {
        "Return ChildBrowserTreeItems for children.";
        return  [ ChildBrowserTreeItem ( obj ) for obj in self . listchildren ( ) ];
        pub fn OnDoubleClick ( self )  {
        "Open a module in an editor window when double clicked.";
        if !is_browseable_extension ( self . file ) {
        return;
        if !os . path . exists ( self . file ) {
        return;
        file_open ( self . file );
        pub fn IsExpandable ( self )  {
        "Return true if Python file.";
        return  is_browseable_extension ( self . file );
        pub fn listchildren ( self )  {
        "Return sequenced classes && functions in the module.";
        if !is_browseable_extension ( self . file ) {
        return  [ ];
        dir , base = os . path . split ( self . file );
        name , _ = os . path . splitext ( base );
        // try {
        tree = pyclbr . readmodule_ex ( name , [ dir ] + sys . path );
        // } catch  ImportError  {
        return  [ ];
        return  transform_children ( tree , name );
        class ChildBrowserTreeItem ( TreeItem ) ;
        "Browser tree for child nodes within the module.

    Uses TreeItem as the basis for the structure of the tree.
    ";
        pub fn __init__ ( &self, obj )  {
        "Create a TreeItem for a pyclbr class/function object.";
        self . obj = obj;
        self . name = obj . name;
        self . isfunction = isinstance ( obj , pyclbr . Function );
        pub fn GetText ( self )  {
        "Return the name of the function/class to display.";
        name = self . name;
        if self . isfunction {
        return  "def " + name + "(...)";
        } else {
        return  "class " + name;
        pub fn GetIconName ( self )  {
        "Return the name of the icon to display.";
        if self . isfunction {
        return  "python";
        } else {
        return  "folder";
        pub fn IsExpandable ( self )  {
        "Return true if self.obj has nested objects.";
        return  self . obj . children != { };
        pub fn GetSubList ( self )  {
        "Return ChildBrowserTreeItems for children.";
        return  [ ChildBrowserTreeItem ( obj );
        for obj in transform_children ( self . obj . children ) ].iter() {
        pub fn OnDoubleClick ( self )  {
        "Open module with file_open && position to lineno.";
        // try {
        edit = file_open ( self . obj . file );
        edit . gotoline ( self . obj . lineno );
        // } catch  ( OSError , AttributeError )  {
        // pass
        pub fn _module_browser ( parent )  {
        if len ( sys . argv ) > 1 {
        file = sys . argv [ 1 ];
        } else {
        file = __file__;
        class Nested_in_func ( TreeNode ) ;
        pub fn nested_in_class ( )  {  pass; }
        pub fn closure ( )  {
        class Nested_in_closure : pass;
        ModuleBrowser ( parent , file , _htest = true );
        fn main() {
        if len ( sys . argv ) == 1 {
        from unittest import main;
        main ( "idlelib.idle_test.test_browser" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _module_browser );
}

