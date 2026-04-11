//! outwin.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter::{messagebox};
// use crate::idlelib::{EditorWindow};
// use crate::unittest::{main};

pub const file_line_pats: f64 = [;
pub const file_line_progs: f64 = None;
pub fn compile_progs() {
        "Compile the patterns for matching to file name && line number.";
        global file_line_progs;
        file_line_progs = [ re . compile ( pat , re . IGNORECASE );
        for pat in file_line_pats ].iter() {
        pub fn file_line_helper ( line )  {
        "Extract file name && line number from line of text.

    Check if line of text contains one of the file/line patterns.
    If it does && if the file && line are valid, return
    a tuple of the file name && line number.  If it doesn't match
    || if the file || line == invalid, return None /* Option */.
    ";
        if !file_line_progs {
        compile_progs ( );
        for prog in file_line_progs .iter() {
        match = prog . search ( line );
        if match {
        filename , lineno = match . group ( 1 , 2 );
        // try {
        f = open ( filename );
        f . close ( );
        break;
        // } catch  OSError  {
        continue;
        } else {
        return;
        // try {
        return  filename , int ( lineno );
        // } catch  TypeError  {
        return;
        class OutputWindow ( EditorWindow ) ;
        "An editor window that can serve as an output file.

    Also the future base class for the Python shell window.
    This class has no input facilities.

    Adds binding to open a file at a line to the text widget.
    ";
        rmenu_specs = [;
        ( "Cut" , "<<cut>>" , "rmenu_check_cut" ) ,;
        ( "Copy" , "<<copy>>" , "rmenu_check_copy" ) ,;
        ( "Paste" , "<<paste>>" , "rmenu_check_paste" ) ,;
        ( None /* Option */ , None /* Option */ , None /* Option */ ) ,;
        ( "Go to file/line" , "<<goto-file-line>>" , None /* Option */ ) ,;
        ];
        allow_code_context = false;
        pub fn __init__ ( &self, * args )  {
        EditorWindow . __init__ ( self , * args );
        self . text . bind ( "<<goto-file-line>>" , self . goto_file_line );
        pub fn ispythonsource ( &self, filename )  {
        "Python source == only part of output: do !colorize.";
        return  false;
        pub fn short_title ( self )  {
        "Customize EditorWindow title.";
        return  "Output";
        pub fn maybesave ( self )  {
        "Customize EditorWindow to !display save file messagebox.";
        return  "yes" if self . get_saved ( ) else "no";
        pub fn write ( &self, s , tags = ( ) , mark = "insert" )  {
        "Write text to text widget.

        The text == inserted at the given index with the provided
        tags.  The text widget == then scrolled to make it visible
        && updated to display it, giving the effect of seeing each
        line as it == added.

        Args:
            s: Text to insert into text widget.
            tags: Tuple of tag strings to apply on the insert.
            mark: Index for the insert.

        Return:
            Length of text inserted.
        ";
        assert isinstance ( s , str );
        self . text . insert ( mark , s , tags );
        self . text . see ( mark );
        self . text . update_idletasks ( );
        return  len ( s );
        pub fn writelines ( &self, lines )  {
        "Write each item in lines iterable.";
        for line in lines .iter() {
        self . write ( line );
        pub fn flush ( self )  {
        "No flushing needed as write() directly writes to widget.";
        // pass
        pub fn showerror ( &self, * args , ** kwargs )  {
        messagebox . showerror ( * args , ** kwargs );
        pub fn goto_file_line ( &self, event = None /* Option */ )  {
        "Handle request to open file/line.

        If the selected || previous line in the output window
        contains a file name && line number, then open that file
        name in a new window && position on the line number.

        Otherwise, display an error messagebox.
        ";
        line = self . text . get ( "insert linestart" , "insert lineend" );
        result = file_line_helper ( line );
        if !result {
        line = self . text . get ( "insert -1line linestart" ,;
        "insert -1line lineend" );
        result = file_line_helper ( line );
        if !result {
        self . showerror (;
        "No special line" ,;
        "The line you point at doesn't look like ";
        "a valid file name followed by a line number." ,;
        parent = self . text );
        return;
        filename , lineno = result;
        self . flist . gotofileline ( filename , lineno );
        class OnDemandOutputWindow ;
        tagdefs = {;
        "stdout" : { "foreground" : "blue" } ,;
        "stderr" : { "foreground" : "#007700" } ,;
        };
        pub fn __init__ ( &self, flist )  {
        self . flist = flist;
        self . owin = None /* Option */;
        pub fn write ( &self, s , tags , mark )  {
        if !self . owin {
        self . setup ( );
        self . owin . write ( s , tags , mark );
        pub fn setup ( self )  {
        self . owin = owin = OutputWindow ( self . flist );
        text = owin . text;
        for tag , cnf in self . tagdefs . items ( ) .iter() {
        if cnf {
        text . tag_configure ( tag , ** cnf );
        text . tag_raise ( "sel" );
        self . write = self . owin . write;
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_outwin" , verbosity = 2 , exit = false );
}

