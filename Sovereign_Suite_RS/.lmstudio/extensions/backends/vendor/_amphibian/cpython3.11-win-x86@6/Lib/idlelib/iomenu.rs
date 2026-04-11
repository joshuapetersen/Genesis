//! iomenu.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::shlex;
// use crate::tempfile;
// use crate::tkinter::{filedialog};
// use crate::idlelib::{idleConf};
// use crate::unittest::{main};

pub const py_extensions: &str = " " . join ("*" + ext for ext in py_extensions );
pub const encoding: &str = "utf-8";
pub const errors: &str = "surrogatepass" if sys . platform =="win32" else"surrogateescape";
pub struct IOBinding {
    pub editwin: String, // TODO: infer type
    pub text: String, // TODO: infer type
    pub __id_open: String, // TODO: infer type
    pub __id_save: String, // TODO: infer type
    pub __id_saveas: String, // TODO: infer type
    pub __id_savecopy: String, // TODO: infer type
    pub fileencoding: String, // TODO: infer type
    pub __id_print: String, // TODO: infer type
    pub filename_change_hook: String, // TODO: infer type
    pub filename: String, // TODO: infer type
    pub dirname: String, // TODO: infer type
    pub eol_convention: String, // TODO: infer type
    pub opendialog: String, // TODO: infer type
    pub savedialog: String, // TODO: infer type
    pub flist: String, // TODO: infer type
}

impl IOBinding {
    pub fn new(editwin: &str) -> Self {
        self . editwin = editwin;
        self . text = editwin . text;
        self . __id_open = self . text . bind ( "<<open-window-from-file>>" , self . open );
        self . __id_save = self . text . bind ( "<<save-window>>" , self . save );
        self . __id_saveas = self . text . bind ( "<<save-window-as-file>>" ,;
        self . save_as );
        self . __id_savecopy = self . text . bind ( "<<save-copy-of-window-as-file>>" ,;
        self . save_a_copy );
        self . fileencoding = "utf-8";
        self . __id_print = self . text . bind ( "<<print-window>>" , self . print_window );
    }

    pub fn _io_binding(&self, parent: &str) {
        from tkinter import Toplevel , Text;
        top = Toplevel ( parent );
        top . title ( "Test IOBinding" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        class MyEditWin ;
        pub fn __init__ ( &self, text )  {
        self . text = text;
        self . flist = None /* Option */;
        self . text . bind ( "<Control-o>" , self . open );
        self . text . bind ( "<Control-p>" , self . print );
        self . text . bind ( "<Control-s>" , self . save );
        self . text . bind ( "<Alt-s>" , self . saveas );
        self . text . bind ( "<Control-c>" , self . savecopy );
        pub fn get_saved ( self )  {  return 0; }
        pub fn set_saved ( &self, flag )  {  pass; }
        pub fn reset_undo ( self )  {  pass; }
        pub fn open ( &self, event )  {
        self . text . event_generate ( "<<open-window-from-file>>" );
        pub fn print ( &self, event )  {
        self . text . event_generate ( "<<print-window>>" );
        pub fn save ( &self, event )  {
        self . text . event_generate ( "<<save-window>>" );
        pub fn saveas ( &self, event )  {
        self . text . event_generate ( "<<save-window-as-file>>" );
        pub fn savecopy ( &self, event )  {
        self . text . event_generate ( "<<save-copy-of-window-as-file>>" );
        text = Text ( top );
        text . pack ( );
        text . focus_set ( );
        editwin = MyEditWin ( text );
        IOBinding ( editwin );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_iomenu" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _io_binding );
    }

}

