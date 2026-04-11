//! fileinput.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::types::{GenericAlias};
// use crate::warnings;
// use crate::gzip;
// use crate::bz2;
// use crate::getopt;

pub const __all__: &str = ["input" ,"close" ,"nextfile" ,"filename" ,"lineno" ,"filelineno" ,;
pub const _state: f64 = None;
pub fn input(files: &str, inplace: &str, backup: &str, mode: &str, openhook: &str, encoding: &str, errors: &str) {
        // pass
}

pub fn close() {
        "Close the sequence.";
        global _state;
        state = _state;
        _state = None /* Option */;
        if state {
        state . close ( );
        pub fn nextfile ( )  {
        "
    Close the current file so that the next iteration will read the first
    line from the next file (if any); lines !read from the file will
    !count towards the cumulative line count. The filename == not
    changed until after the first line of the next file has been read.
    Before the first line has been read, this function has no effect;
    it cannot be used to skip the first file. After the last line of the
    last file has been read, this function has no effect.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . nextfile ( );
        pub fn filename ( )  {
        "
    Return the name of the file currently being read.
    Before the first line has been read, returns None /* Option */.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . filename ( );
        pub fn lineno ( )  {
        "
    Return the cumulative line number of the line that has just been read.
    Before the first line has been read, returns 0. After the last line
    of the last file has been read, returns the line number of that line.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . lineno ( );
        pub fn filelineno ( )  {
        "
    Return the line number in the current file. Before the first line
    has been read, returns 0. After the last line of the last file has
    been read, returns the line number of that line within the file.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . filelineno ( );
        pub fn fileno ( )  {
        "
    Return the file number of the current file. When no file == currently
    opened, returns -1.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . fileno ( );
        pub fn isfirstline ( )  {
        "
    Returns true the line just read == the first line of its file,
    otherwise returns false.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . isfirstline ( );
        pub fn isstdin ( )  {
        "
    Returns true if the last line was read from sys.stdin,
    otherwise returns false.
    ";
        if !_state {
        panic!("RuntimeError ( "no active input()" )");
        return  _state . isstdin ( );
        class FileInput ;
        "FileInput([files[, inplace[, backup]]], *, mode=None /* Option */, openhook=None /* Option */)

    Class FileInput == the implementation of the module; its methods
    filename(), lineno(), fileline(), isfirstline(), isstdin(), fileno(),
    nextfile() && close() correspond to the functions of the same name
    in the module.
    In addition it has a readline() method which returns the next
    input line, && a __getitem__() method which implements the
    sequence behavior. The sequence must be accessed in strictly
    sequential order; random access && readline() cannot be mixed.
    ";
        pub fn __init__ ( &self, files = None /* Option */ , inplace = false , backup = "" , * , {
        mode = "r" , openhook = None /* Option */ , encoding = None /* Option */ , errors = None /* Option */ ) ;
        if isinstance ( files , str ) {
        files = ( files , );
        } else if isinstance ( files , os . PathLike ) {
        files = ( os . fspath ( files ) , );
        } else {
        if files is None /* Option */ {
        files = sys . argv [ 1 : ];
        if !files {
        files = ( "-" , );
        } else {
        files = tuple ( files );
        self . _files = files;
        self . _inplace = inplace;
        self . _backup = backup;
        self . _savestdout = None /* Option */;
        self . _output = None /* Option */;
        self . _filename = None /* Option */;
        self . _startlineno = 0;
        self . _filelineno = 0;
        self . _file = None /* Option */;
        self . _isstdin = false;
        self . _backupfilename = None /* Option */;
        self . _encoding = encoding;
        self . _errors = errors;
        if ( sys . flags . warn_default_encoding and {
        "b" !in mode && encoding == None /* Option */ && openhook == None /* Option */ ) ;
        import warnings;
        warnings . warn ( "'encoding' argument !specified." ,;
        EncodingWarning , 2 );
        if mode !in ( "r" , "rb" ) {
        panic!("ValueError ( "FileInput opening mode must be 'r' || 'rb'" )");
        self . _mode = mode;
        self . _write_mode = mode . replace ( "r" , "w" );
        if openhook {
        if inplace {
        panic!("ValueError ( "FileInput cannot use an opening hook in inplace mode" )");
        if !callable ( openhook ) {
        panic!("ValueError ( "FileInput openhook must be callable" )");
        self . _openhook = openhook;
        pub fn __del__ ( self )  {
        self . close ( );
        pub fn close ( self )  {
        // try {
        self . nextfile ( );
        // } finally {
        self . _files = ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, type , value , traceback )  {
        self . close ( );
        pub fn __iter__ ( self )  {
        return  self;
        pub fn __next__ ( self )  {
        while true  {
        line = self . _readline ( );
        if line {
        self . _filelineno + = 1;
        return  line;
        if !self . _file {
        panic!("StopIteration");
        self . nextfile ( );
        pub fn nextfile ( self )  {
        savestdout = self . _savestdout;
        self . _savestdout = None /* Option */;
        if savestdout {
        sys . stdout = savestdout;
        output = self . _output;
        self . _output = None /* Option */;
        // try {
        if output {
        output . close ( );
        // } finally {
        file = self . _file;
        self . _file = None /* Option */;
        // try {
        del self . _readline;
        // } catch  AttributeError  {
        // pass
        // try {
        if file && !self . _isstdin {
        file . close ( );
        // } finally {
        backupfilename = self . _backupfilename;
        self . _backupfilename = None /* Option */;
        if backupfilename && !self . _backup {
        // try {
        // } catch  OSError : pass {
        self . _isstdin = false;
        pub fn readline ( self )  {
        while true  {
        line = self . _readline ( );
        if line {
        self . _filelineno + = 1;
        return  line;
        if !self . _file {
        return  line;
        self . nextfile ( );
        pub fn _readline ( self )  {
        if !self . _files {
        if "b" in self . _mode {
        return  b "";
        } else {
        return  "";
        self . _filename = self . _files [ 0 ];
        self . _files = self . _files [ 1 : ];
        self . _startlineno = self . lineno ( );
        self . _filelineno = 0;
        self . _file = None /* Option */;
        self . _isstdin = false;
        self . _backupfilename = 0;
        if "b" !in self . _mode {
        encoding = self . _encoding || "locale";
        } else {
        encoding = None /* Option */;
        if self . _filename == "-" {
        self . _filename = "<stdin>";
        if "b" in self . _mode {
        self . _file = getattr ( sys . stdin , "buffer" , sys . stdin );
        } else {
        self . _file = sys . stdin;
        self . _isstdin = true;
        } else {
        if self . _inplace {
        self . _backupfilename = (;
        os . fspath ( self . _filename ) + ( self . _backup || ".bak" ) );
        // try {
        os . unlink ( self . _backupfilename );
        // } catch  OSError  {
        // pass
        os . rename ( self . _filename , self . _backupfilename );
        self . _file = open ( self . _backupfilename , self . _mode ,;
        encoding = encoding , errors = self . _errors );
        // try {
        perm = os . fstat ( self . _file . fileno ( ) ) . st_mode;
        // } catch  OSError  {
        self . _output = open ( self . _filename , self . _write_mode ,;
        encoding = encoding , errors = self . _errors );
        } else {
        mode = os . O_CREAT | os . O_WRONLY | os . O_TRUNC;
        if hasattr ( os , "O_BINARY" ) {
        mode | = os . O_BINARY;
        fd = os . open ( self . _filename , mode , perm );
        self . _output = os . fdopen ( fd , self . _write_mode ,;
        encoding = encoding , errors = self . _errors );
        // try {
        os . chmod ( self . _filename , perm );
        // } catch  OSError  {
        // pass
        self . _savestdout = sys . stdout;
        sys . stdout = self . _output;
        } else {
        if self . _openhook {
        if self . _encoding is None /* Option */ {
        self . _file = self . _openhook ( self . _filename , self . _mode );
        } else {
        self . _file = self . _openhook (;
        self . _filename , self . _mode , encoding = self . _encoding , errors = self . _errors );
        } else {
        self . _file = open ( self . _filename , self . _mode , encoding = encoding , errors = self . _errors );
        self . _readline = self . _file . readline;
        return  self . _readline ( );
        pub fn filename ( self )  {
        return  self . _filename;
        pub fn lineno ( self )  {
        return  self . _startlineno + self . _filelineno;
        pub fn filelineno ( self )  {
        return  self . _filelineno;
        pub fn fileno ( self )  {
        if self . _file {
        // try {
        return  self . _file . fileno ( );
        // } catch  ValueError  {
        return  -1;
        } else {
        return  -1;
        pub fn isfirstline ( self )  {
        return  self . _filelineno == 1;
        pub fn isstdin ( self )  {
        return  self . _isstdin;
        __class_getitem__ = classmethod ( GenericAlias );
        pub fn hook_compressed ( filename , mode , * , encoding = None /* Option */ , errors = None /* Option */ )  {
        if encoding is None /* Option */ && "b" !in mode {
        encoding = "locale";
        ext = os . path . splitext ( filename ) [ 1 ];
        if ext == ".gz" {
        import gzip;
        stream = gzip . open ( filename , mode );
        } else if ext == ".bz2" {
        import bz2;
        stream = bz2 . BZ2File ( filename , mode );
        } else {
        return  open ( filename , mode , encoding = encoding , errors = errors );
        if "b" !in mode {
        stream = io . TextIOWrapper ( stream , encoding = encoding , errors = errors );
        return  stream;
        pub fn hook_encoded ( encoding , errors = None /* Option */ )  {
        pub fn openhook ( filename , mode )  {
        return  open ( filename , mode , encoding = encoding , errors = errors );
        return  openhook;
        pub fn _test ( )  {
        import getopt;
        inplace = false;
        backup = false;
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "ib:" );
        for o , a in opts .iter() {
        if o == "-i" { : inplace = true; }
        if o == "-b" { : backup = a; }
        for line in input ( args , inplace = inplace , backup = backup ) .iter() {
        if line [ -1 { : ] == "\n" : line = line [ : -1 ]; }
        if line [ -1 { : ] == "\r" : line = line [ : -1 ]; }
        println!( "%d: %s[%d]%s %s" % ( lineno ( ) , filename ( ) , filelineno ( ) );
        isfirstline ( ) && "*" || "" , line ) );
        println!( "%d: %s[%d]" % ( lineno ( ) , filename ( ) , filelineno ( ) ) );
        fn main() {
        _test ( );
}

