//! stackviewer.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::linecache;
// use crate::tkinter;
// use crate::idlelib::{ObjectTreeItem, make_objecttreeitem};
// use crate::unittest::{main};

pub fn StackBrowser(root: &str, exc: &str, flist: &str, top: &str) {
        global sc , item , node;
        if top is None /* Option */ {
        top = tk . Toplevel ( root );
        sc = ScrolledCanvas ( top , bg = "white" , highlightthickness = 0 );
        sc . frame . pack ( expand = 1 , fill = "both" );
        item = StackTreeItem ( exc , flist );
        node = TreeNode ( sc . canvas , None /* Option */ , item );
        node . expand ( );
        class StackTreeItem ( TreeItem ) ;
        pub fn __init__ ( &self, exc , flist = None /* Option */ )  {
        self . flist = flist;
        self . stack = self . get_stack ( None /* Option */ if exc is None /* Option */ else exc . __traceback__ );
        self . text = f "{type(exc).__name__}: {str(exc)}";
        pub fn get_stack ( &self, tb )  {
        stack = [ ];
        if tb && tb . tb_frame is None /* Option */ {
        tb = tb . tb_next;
        while tb is !None /* Option */  {
        stack . append ( ( tb . tb_frame , tb . tb_lineno ) );
        tb = tb . tb_next;
        return  stack;
        pub fn GetText ( self )  {
        return  self . text;
        pub fn GetSubList ( self )  {
        sublist = [ ];
        for info in self . stack .iter() {
        item = FrameTreeItem ( info , self . flist );
        sublist . append ( item );
        return  sublist;
        class FrameTreeItem ( TreeItem ) ;
        pub fn __init__ ( &self, info , flist )  {
        self . info = info;
        self . flist = flist;
        pub fn GetText ( self )  {
        frame , lineno = self . info;
        // try {
        modname = frame . f_globals [ "__name__" ];
        // } catch   {
        modname = "?";
        code = frame . f_code;
        filename = code . co_filename;
        funcname = code . co_name;
        sourceline = linecache . getline ( filename , lineno );
        sourceline = sourceline . strip ( );
        if funcname in ( "?" , "" , None /* Option */ ) {
        item = "%s, line %d: %s" % ( modname , lineno , sourceline );
        } else {
        item = "%s.%s(...), line %d: %s" % ( modname , funcname ,;
        lineno , sourceline );
        return  item;
        pub fn GetSubList ( self )  {
        frame , lineno = self . info;
        sublist = [ ];
        if frame . f_globals is !frame . f_locals {
        item = VariablesTreeItem ( "<locals>" , frame . f_locals , self . flist );
        sublist . append ( item );
        item = VariablesTreeItem ( "<globals>" , frame . f_globals , self . flist );
        sublist . append ( item );
        return  sublist;
        pub fn OnDoubleClick ( self )  {
        if self . flist {
        frame , lineno = self . info;
        filename = frame . f_code . co_filename;
        if os . path . isfile ( filename ) {
        self . flist . gotofileline ( filename , lineno );
        class VariablesTreeItem ( ObjectTreeItem ) ;
        pub fn GetText ( self )  {
        return  self . labeltext;
        pub fn GetLabelText ( self )  {
        return;
        pub fn IsExpandable ( self )  {
        return  len ( self . object ) > 0;
        pub fn GetSubList ( self )  {
        sublist = [ ];
        for key in self . object . keys ( ) .iter() {
        // try {
        value = self . object [ key ];
        // } catch  KeyError  {
        continue;
        pub fn setfunction ( value , key = key , object_ = self . object )  {
        object_ [ key ] = value;
        item = make_objecttreeitem ( key + " =" , value , setfunction );
        sublist . append ( item );
        return  sublist;
        pub fn _stackbrowser ( parent )  {
        from idlelib . pyshell import PyShellFileList;
        top = tk . Toplevel ( parent );
        top . title ( "Test StackViewer" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x + 50 , y + 175 ) );
        flist = PyShellFileList ( top );
        // try {
        intentional_name_error;
        // } catch  NameError as e  {
        StackBrowser ( top , e , flist = flist , top = top );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_stackviewer" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _stackbrowser );
}

