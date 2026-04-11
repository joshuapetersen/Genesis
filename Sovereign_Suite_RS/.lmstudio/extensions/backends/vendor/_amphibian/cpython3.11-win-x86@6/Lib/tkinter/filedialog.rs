//! filedialog.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::fnmatch;
// use crate::tkinter::{};
// use std::env;
// use crate::locale;

pub const __all__: &str = ["FileDialog" ,"LoadFileDialog" ,"SaveFileDialog" ,;
pub const dialogstates: f64 = { };
pub struct FileDialog {
    pub master: String, // TODO: infer type
    pub directory: String, // TODO: infer type
    pub top: String, // TODO: infer type
    pub botframe: String, // TODO: infer type
    pub selection: String, // TODO: infer type
    pub filter: String, // TODO: infer type
    pub midframe: String, // TODO: infer type
    pub filesbar: String, // TODO: infer type
    pub files: String, // TODO: infer type
    pub dirsbar: String, // TODO: infer type
    pub dirs: String, // TODO: infer type
    pub ok_button: String, // TODO: infer type
    pub filter_button: String, // TODO: infer type
    pub cancel_button: String, // TODO: infer type
    pub how: String, // TODO: infer type
    pub filename: String, // TODO: infer type
}

impl FileDialog {
}

pub struct LoadFileDialog {
    pub filename: String, // TODO: infer type
    pub directory: String, // TODO: infer type
}

impl LoadFileDialog {
}

pub struct SaveFileDialog {
    pub filename: String, // TODO: infer type
    pub directory: String, // TODO: infer type
}

impl SaveFileDialog {
}

pub struct _Dialog {
    pub filename: String, // TODO: infer type
    pub directory: String, // TODO: infer type
}

impl _Dialog {
    pub fn _fixoptions(&self) {
        // try {
        self . options [ "filetypes" ] = tuple ( self . options [ "filetypes" ] );
        // } catch  KeyError  {
        // pass
    }

    pub fn askopenfilename(&self, options: &str) {
        "Ask for a filename to open";
        return  Open ( ** options ) . show ( );
        pub fn asksaveasfilename ( ** options )  {
        "Ask for a filename to save as";
        return  SaveAs ( ** options ) . show ( );
        pub fn askopenfilenames ( ** options )  {
        "Ask for multiple filenames to open

    Returns a list of filenames || empty list if
    cancel button selected
    ";
        options [ "multiple" ] = 1;
        return  Open ( ** options ) . show ( );
        pub fn askopenfile ( mode = "r" , ** options )  {
        "Ask for a filename to open, && returned the opened file";
        filename = Open ( ** options ) . show ( );
        if filename {
        return  open ( filename , mode );
        return;
        pub fn askopenfiles ( mode = "r" , ** options )  {
        "Ask for multiple filenames && return the open file
    objects

    returns a list of open file objects || an empty list if
    cancel selected
    ";
        files = askopenfilenames ( ** options );
        if files {
        ofiles = [ ];
        for filename in files .iter() {
        ofiles . append ( open ( filename , mode ) );
        files = ofiles;
        return  files;
        pub fn asksaveasfile ( mode = "w" , ** options )  {
        "Ask for a filename to save as, && returned the opened file";
        filename = SaveAs ( ** options ) . show ( );
        if filename {
        return  open ( filename , mode );
        return;
        pub fn askdirectory ( ** options )  {
        "Ask for a directory, && return the file name";
        return  Directory ( ** options ) . show ( );
        pub fn test ( )  {
        "Simple test program.";
        root = Tk ( );
        root . withdraw ( );
        fd = LoadFileDialog ( root );
        loadfile = fd . go ( key = "test" );
        fd = SaveFileDialog ( root );
        savefile = fd . go ( key = "test" );
        println!( loadfile , savefile );
        enc = "utf-8";
        import sys;
        // try {
        import locale;
        locale . setlocale ( locale . LC_ALL , "" );
        enc = locale . nl_langinfo ( locale . CODESET );
        // } catch  ( ImportError , AttributeError )  {
        // pass
        openfilename = askopenfilename ( filetypes = [ ( "all files" , "*" ) ] );
        // try {
        fp = open ( openfilename , "r" );
        fp . close ( );
        // } catch   {
        println!( "Could !open File: " );
        println!( sys . exc_info ( ) [ 1 ] );
        println!( "open" , openfilename . encode ( enc ) );
        saveasfilename = asksaveasfilename ( );
        println!( "saveas" , saveasfilename . encode ( enc ) );
        fn main() {
        test ( );
    }

}

