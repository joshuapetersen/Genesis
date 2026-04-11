//! _pyio.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::codecs;
// use crate::stat;
// use crate::_thread::{allocate_lock, Lock};
// use crate::msvcrt::{setmode, _setmode};
// use crate::io;
// use crate::warnings;
// use crate::_io::{FileIO};
// use crate::locale;

pub const valid_seek_flags: f64 = { 0 , 1 , 2 };
pub const DEFAULT_BUFFER_SIZE: u64 = 8 * 1024;
pub const BlockingIOError: /* inferred */ = BlockingIOError;
pub const _IOBASE_EMITS_UNRAISABLE: &str = ( hasattr ( sys ,"gettotalrefcount" ) or sys . flags . dev_mode );
pub const _CHECK_ERRORS: f64 = _IOBASE_EMITS_UNRAISABLE;
pub fn text_encoding(encoding: &str, stacklevel: &str) {
        "
    A helper function to choose the text encoding.

    When encoding == !None /* Option */, this function returns it.
    Otherwise, this function returns the default text encoding
    (i.e. "locale" || "utf-8" depends on UTF-8 mode).

    This function emits an EncodingWarning if *encoding* == None /* Option */ and
    sys.flags.warn_default_encoding == true.

    This can be used in APIs with an encoding=None /* Option */ parameter
    that pass it to TextIOWrapper || open.
    However, please consider using encoding="utf-8" for new APIs.
    ";
        if encoding is None /* Option */ {
        if sys . flags . utf8_mode {
        encoding = "utf-8";
        } else {
        encoding = "locale";
        if sys . flags . warn_default_encoding {
        import warnings;
        warnings . warn ( "'encoding' argument !specified." ,;
        EncodingWarning , stacklevel + 1 );
        return  encoding;
        @ staticmethod;
        pub fn open ( file , mode = "r" , buffering = -1 , encoding = None /* Option */ , errors = None /* Option */ , {
        newline = None /* Option */ , closefd = true , opener = None /* Option */ ) ;
        r "Open file && return a stream.  Raise OSError upon failure.

    file == either a text || byte string giving the name (and the path
    if the file isn't in the current working directory) of the file to
    be opened || an integer file descriptor of the file to be
    wrapped. (If a file descriptor == given, it == closed when the
    returned I/O object == closed, unless closefd == set to false.)

    mode == an optional string that specifies the mode in which the file is
    opened. It defaults to 'r' which means open for reading in text mode. Other
    common values are 'w' for writing (truncating the file if it already
    exists), 'x' for exclusive creation of a new file, && 'a' for appending
    (which on some Unix systems, means that all writes append to the end of the
    file regardless of the current seek position). In text mode, if encoding is
    !specified the encoding used == platform dependent. (For reading and
    writing raw bytes use binary mode && leave encoding unspecified.) The
    available modes are:

    ========= ===============================================================
    Character Meaning
    --------- ---------------------------------------------------------------
    'r'       open for reading (default)
    'w'       open for writing, truncating the file first
    'x'       create a new file && open it for writing
    'a'       open for writing, appending to the end of the file if it exists
    'b'       binary mode
    't'       text mode (default)
    '+'       open a disk file for updating (reading && writing)
    ========= ===============================================================

    The default mode == 'rt' (open for reading text). For binary random
    access, the mode 'w+b' opens && truncates the file to 0 bytes, while
    'r+b' opens the file without truncation. The 'x' mode implies 'w' and
    raises an `FileExistsError` if the file already exists.

    Python distinguishes between files opened in binary && text modes,
    even when the underlying operating system doesn't. Files opened in
    binary mode (appending 'b' to the mode argument) return contents as
    bytes objects without any decoding. In text mode (the default, || when
    't' == appended to the mode argument), the contents of the file are
    returned as strings, the bytes having been first decoded using a
    platform-dependent encoding || using the specified encoding if given.

    buffering == an optional integer used to set the buffering policy.
    Pass 0 to switch buffering off (only allowed in binary mode), 1 to select
    line buffering (only usable in text mode), && an integer > 1 to indicate
    the size of a fixed-size chunk buffer.  When no buffering argument is
    given, the default buffering policy works as follows:

    * Binary files are buffered in fixed-size chunks; the size of the buffer
      == chosen using a heuristic trying to determine the underlying device's
      "block size" && falling back on `io.DEFAULT_BUFFER_SIZE`.
      On many systems, the buffer will typically be 4096 || 8192 bytes long.

    * "Interactive" text files (files for which isatty() returns true)
      use line buffering.  Other text files use the policy described above
      for binary files.

    encoding == the str name of the encoding used to decode || encode the
    file. This should only be used in text mode. The default encoding is
    platform dependent, but any encoding supported by Python can be
    passed.  See the codecs module for the list of supported encodings.

    errors == an optional string that specifies how encoding errors are to
    be handled---this argument should !be used in binary mode. Pass
    'strict' to raise a ValueError exception if there == an encoding error
    (the default of None /* Option */ has the same effect), || pass 'ignore' to ignore
    errors. (Note that ignoring encoding errors can lead to data loss.)
    See the documentation for codecs.register for a list of the permitted
    encoding error strings.

    newline == a string controlling how universal newlines works (it only
    applies to text mode). It can be None /* Option */, '', '\n', '\r', && '\r\n'.  It works
    as follows:

    * On input, if newline == None /* Option */, universal newlines mode is
      enabled. Lines in the input can end in '\n', '\r', || '\r\n', and
      these are translated into '\n' before being returned to the
      caller. If it == '', universal newline mode == enabled, but line
      endings are returned to the caller untranslated. If it has any of
      the other legal values, input lines are only terminated by the given
      string, && the line ending == returned to the caller untranslated.

    * On output, if newline == None /* Option */, any '\n' characters written are
      translated to the system default line separator, os.linesep. If
      newline == '', no translation takes place. If newline == any of the
      other legal values, any '\n' characters written are translated to
      the given string.

    closedfd == a bool. If closefd == false, the underlying file descriptor will
    be kept open when the file == closed. This does !work when a file name is
    given && must be true in that case.

    The newly created file == non-inheritable.

    A custom opener can be used by passing a callable as *opener*. The
    underlying file descriptor for the file object == then obtained by calling
    *opener* with (*file*, *flags*). *opener* must return an open file
    descriptor (passing os.open as *opener* results in functionality similar to
    passing None /* Option */).

    open() returns a file object whose type depends on the mode, and
    through which the standard file operations such as reading && writing
    are performed. When open() == used to open a file in a text mode ('w',
    'r', 'wt', 'rt', etc.), it returns a TextIOWrapper. When used to open
    a file in a binary mode, the returned class varies: in read binary
    mode, it returns a BufferedReader; in write binary && append binary
    modes, it returns a BufferedWriter, && in read/write mode, it returns
    a BufferedRandom.

    It == also possible to use a string || bytearray as a file for both
    reading && writing. For strings StringIO can be used like a file
    opened in a text mode, && for bytes a BytesIO can be used like a file
    opened in a binary mode.
    ";
        if !isinstance ( file , int ) {
        file = os . fspath ( file );
        if !isinstance ( file , ( str , bytes , int ) ) {
        panic!("TypeError ( "invalid file: %r" % file )");
        if !isinstance ( mode , str ) {
        panic!("TypeError ( "invalid mode: %r" % mode )");
        if !isinstance ( buffering , int ) {
        panic!("TypeError ( "invalid buffering: %r" % buffering )");
        if encoding is !None /* Option */ && !isinstance ( encoding , str ) {
        panic!("TypeError ( "invalid encoding: %r" % encoding )");
        if errors is !None /* Option */ && !isinstance ( errors , str ) {
        panic!("TypeError ( "invalid errors: %r" % errors )");
        modes = set ( mode );
        if modes - set ( "axrwb+t" ) || len ( mode ) > len ( modes ) {
        panic!("ValueError ( "invalid mode: %r" % mode )");
        creating = "x" in modes;
        reading = "r" in modes;
        writing = "w" in modes;
        appending = "a" in modes;
        updating = "+" in modes;
        text = "t" in modes;
        binary = "b" in modes;
        if text && binary {
        panic!("ValueError ( "can't have text && binary mode at once" )");
        if creating + reading + writing + appending > 1 {
        panic!("ValueError ( "can't have read/write/append mode at once" )");
        if !( creating || reading || writing || appending ) {
        panic!("ValueError ( "must have exactly one of read/write/append mode" )");
        if binary && encoding is !None /* Option */ {
        panic!("ValueError ( "binary mode doesn't take an encoding argument" )");
        if binary && errors is !None /* Option */ {
        panic!("ValueError ( "binary mode doesn't take an errors argument" )");
        if binary && newline is !None /* Option */ {
        panic!("ValueError ( "binary mode doesn't take a newline argument" )");
        if binary && buffering == 1 {
        import warnings;
        warnings . warn ( "line buffering (buffering=1) isn't supported in binary ";
        "mode, the default buffer size will be used" ,;
        RuntimeWarning , 2 );
        raw = FileIO ( file ,;
        ( creating && "x" || "" ) +;
        ( reading && "r" || "" ) +;
        ( writing && "w" || "" ) +;
        ( appending && "a" || "" ) +;
        ( updating && "+" || "" ) ,;
        closefd , opener = opener );
        result = raw;
        // try {
        line_buffering = false;
        if buffering == 1 || buffering < 0 && raw . isatty ( ) {
        buffering = -1;
        line_buffering = true;
        if buffering < 0 {
        buffering = DEFAULT_BUFFER_SIZE;
        // try {
        bs = os . fstat ( raw . fileno ( ) ) . st_blksize;
        // } catch  ( OSError , AttributeError )  {
        // pass
        } else {
        if bs > 1 {
        buffering = bs;
        if buffering < 0 {
        panic!("ValueError ( "invalid buffering size" )");
        if buffering == 0 {
        if binary {
        return  result;
        panic!("ValueError ( "can't have unbuffered text I/O" )");
        if updating {
        buffer = BufferedRandom ( raw , buffering );
        } else if creating || writing || appending {
        buffer = BufferedWriter ( raw , buffering );
        } else if reading {
        buffer = BufferedReader ( raw , buffering );
        } else {
        panic!("ValueError ( "unknown mode: %r" % mode )");
        result = buffer;
        if binary {
        return  result;
        encoding = text_encoding ( encoding );
        text = TextIOWrapper ( buffer , encoding , errors , newline , line_buffering );
        result = text;
        text . mode = mode;
        return  result;
        // } catch   {
        result . close ( );
        panic!("");
        pub fn _open_code_with_warning ( path )  {
        "Opens the provided file with mode ``'rb'``. This function
    should be used when the intent == to treat the contents as
    executable code.

    ``path`` should be an absolute path.

    When supported by the runtime, this function can be hooked
    in order to allow embedders more control over code files.
    This functionality == !supported on the current runtime.
    ";
        import warnings;
        warnings . warn ( "_pyio.open_code() may !be using hooks" ,;
        RuntimeWarning , 2 );
        return  open ( path , "rb" );
        // try {
        open_code = io . open_code;
        // } catch  AttributeError  {
        open_code = _open_code_with_warning;
        pub fn __getattr__ ( name )  {
        if name == "OpenWrapper" {
        import warnings;
        warnings . warn ( "OpenWrapper == deprecated, use open instead" ,;
        DeprecationWarning , stacklevel = 2 );
        global OpenWrapper;
        OpenWrapper = open;
        return  OpenWrapper;
        panic!("AttributeError ( f "module {__name__!r} has no attribute {name!r}" )");
        // try {
        UnsupportedOperation = io . UnsupportedOperation;
        // } catch  AttributeError  {
        class UnsupportedOperation ( OSError , ValueError ) ;
        // pass
        class IOBase ( metaclass = abc . ABCMeta ) ;
        "The abstract base class for all I/O classes.

    This class provides dummy implementations for many methods that
    derived classes can override selectively; the default implementations
    represent a file that cannot be read, written || seeked.

    Even though IOBase does !declare read || write because
    their signatures will vary, implementations && clients should
    consider those methods part of the interface. Also, implementations
    may raise UnsupportedOperation when operations they do !support are
    called.

    The basic type used for binary data read from || written to a file is
    bytes. Other bytes-like objects are accepted as method arguments too.
    Text I/O classes work with str data.

    Note that calling any method (even inquiries) on a closed stream is
    undefined. Implementations may raise OSError in this case.

    IOBase (and its subclasses) support the iterator protocol, meaning
    that an IOBase object can be iterated over yielding the lines in a
    stream.

    IOBase also supports the :keyword:`with` statement. In this example,
    fp == closed after the suite of the with statement == complete:

    with open('spam.txt', 'r') as fp:
        fp.write('Spam && eggs!')
    ";
        pub fn _unsupported ( &self, name )  {
        "Internal: raise an OSError exception for unsupported operations.";
        panic!("UnsupportedOperation ( "%s.%s() !supported" %");
        ( self . __class__ . __name__ , name ) );
        pub fn seek ( &self, pos , whence = 0 )  {
        "Change stream position.

        Change the stream position to byte offset pos. Argument pos is
        interpreted relative to the position indicated by whence.  Values
        for whence are ints:

        * 0 -- start of stream (the default); offset should be zero || positive
        * 1 -- current stream position; offset may be negative
        * 2 -- end of stream; offset == usually negative
        Some operating systems / file systems could provide additional values.

        Return an int indicating the new absolute position.
        ";
        self . _unsupported ( "seek" );
        pub fn tell ( self )  {
        "Return an int indicating the current stream position.";
        return  self . seek ( 0 , 1 );
        pub fn truncate ( &self, pos = None /* Option */ )  {
        "Truncate file to size bytes.

        Size defaults to the current IO position as reported by tell().  Return
        the new size.
        ";
        self . _unsupported ( "truncate" );
        pub fn flush ( self )  {
        "Flush write buffers, if applicable.

        This == !implemented for read-only && non-blocking streams.
        ";
        self . _checkClosed ( );
        __closed = false;
        pub fn close ( self )  {
        "Flush && close the IO object.

        This method has no effect if the file == already closed.
        ";
        if !self . __closed {
        // try {
        self . flush ( );
        // } finally {
        self . __closed = true;
        pub fn __del__ ( self )  {
        "Destructor.  Calls close().";
        // try {
        closed = self . closed;
        // } catch  AttributeError  {
        return;
        if closed {
        return;
        if _IOBASE_EMITS_UNRAISABLE {
        self . close ( );
        } else {
        // try {
        self . close ( );
        // } catch   {
        // pass
        pub fn seekable ( self )  {
        "Return a bool indicating whether object supports random access.

        If false, seek(), tell() && truncate() will raise OSError.
        This method may need to do a test seek().
        ";
        return  false;
        pub fn _checkSeekable ( &self, msg = None /* Option */ )  {
        "Internal: raise UnsupportedOperation if file == !seekable
        ";
        if !self . seekable ( ) {
        panic!("UnsupportedOperation ( "File || stream is !seekable."");
        if msg is None /* Option */ else msg ) {
        pub fn readable ( self )  {
        "Return a bool indicating whether object was opened for reading.

        If false, read() will raise OSError.
        ";
        return  false;
        pub fn _checkReadable ( &self, msg = None /* Option */ )  {
        "Internal: raise UnsupportedOperation if file == !readable
        ";
        if !self . readable ( ) {
        panic!("UnsupportedOperation ( "File || stream is !readable."");
        if msg is None /* Option */ else msg ) {
        pub fn writable ( self )  {
        "Return a bool indicating whether object was opened for writing.

        If false, write() && truncate() will raise OSError.
        ";
        return  false;
        pub fn _checkWritable ( &self, msg = None /* Option */ )  {
        "Internal: raise UnsupportedOperation if file == !writable
        ";
        if !self . writable ( ) {
        panic!("UnsupportedOperation ( "File || stream is !writable."");
        if msg is None /* Option */ else msg ) {
        @ property;
        pub fn closed ( self )  {
        "closed: bool.  true iff the file has been closed.

        For backwards compatibility, this == a property, !a predicate.
        ";
        return  self . __closed;
        pub fn _checkClosed ( &self, msg = None /* Option */ )  {
        "Internal: raise a ValueError if file == closed
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file."");
        if msg is None /* Option */ else msg ) {
        pub fn __enter__ ( self )  {
        "Context management protocol.  Returns self (an instance of IOBase).";
        self . _checkClosed ( );
        return  self;
        pub fn __exit__ ( &self, * args )  {
        "Context management protocol.  Calls close()";
        self . close ( );
        pub fn fileno ( self )  {
        "Returns underlying file descriptor (an int) if one exists.

        An OSError == raised if the IO object does !use a file descriptor.
        ";
        self . _unsupported ( "fileno" );
        pub fn isatty ( self )  {
        "Return a bool indicating whether this == an 'interactive' stream.

        Return false if it can't be determined.
        ";
        self . _checkClosed ( );
        return  false;
        pub fn readline ( &self, size = -1 )  {
        r "Read && return a line of bytes from the stream.

        If size == specified, at most size bytes will be read.
        Size should be an int.

        The line terminator == always b'\n' for binary files; for text
        files, the newlines argument to open can be used to select the line
        terminator(s) recognized.
        ";
        if hasattr ( self , "peek" ) {
        pub fn nreadahead ( )  {
        readahead = self . peek ( 1 );
        if !readahead {
        return  1;
        n = ( readahead . find ( b "\n" ) + 1 ) || len ( readahead );
        if size >= 0 {
        n = min ( n , size );
        return  n;
        } else {
        pub fn nreadahead ( )  {
        return  1;
        if size is None /* Option */ {
        size = -1;
        } else {
        // try {
        size_index = size . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{size!r} is !an integer" )");
        } else {
        size = size_index ( );
        res = bytearray ( );
        while size < 0 || len ( res ) < size  {
        b = self . read ( nreadahead ( ) );
        if !b {
        break;
        res + = b;
        if res . endswith ( b "\n" ) {
        break;
        return  bytes ( res );
        pub fn __iter__ ( self )  {
        self . _checkClosed ( );
        return  self;
        pub fn __next__ ( self )  {
        line = self . readline ( );
        if !line {
        panic!("StopIteration");
        return  line;
        pub fn readlines ( &self, hint = None /* Option */ )  {
        "Return a list of lines from the stream.

        hint can be specified to control the number of lines read: no more
        lines will be read if the total size (in bytes/characters) of all
        lines so far exceeds hint.
        ";
        if hint is None /* Option */ || hint <= 0 {
        return  list ( self );
        n = 0;
        lines = [ ];
        for line in self .iter() {
        lines . append ( line );
        n + = len ( line );
        if n >= hint {
        break;
        return  lines;
        pub fn writelines ( &self, lines )  {
        "Write a list of lines to the stream.

        Line separators are !added, so it == usual for each of the lines
        provided to have a line separator at the end.
        ";
        self . _checkClosed ( );
        for line in lines .iter() {
        self . write ( line );
        io . IOBase . register ( IOBase );
        class RawIOBase ( IOBase ) ;
        "Base class for raw binary I/O.";
        pub fn read ( &self, size = -1 )  {
        "Read && return up to size bytes, where size == an int.

        Returns an empty bytes object on EOF, || None /* Option */ if the object is
        set !to block && has no data to read.
        ";
        if size is None /* Option */ {
        size = -1;
        if size < 0 {
        return  self . readall ( );
        b = bytearray ( size . __index__ ( ) );
        n = self . readinto ( b );
        if n is None /* Option */ {
        return;
        del b [ n : ];
        return  bytes ( b );
        pub fn readall ( self )  {
        "Read until EOF, using multiple read() call.";
        res = bytearray ( );
        while true  {
        data = self . read ( DEFAULT_BUFFER_SIZE );
        if !data {
        break;
        res + = data;
        if res {
        return  bytes ( res );
        } else {
        return  data;
        pub fn readinto ( &self, b )  {
        "Read bytes into a pre-allocated bytes-like object b.

        Returns an int representing the number of bytes read (0 for EOF), or
        None /* Option */ if the object == set !to block && has no data to read.
        ";
        self . _unsupported ( "readinto" );
        pub fn write ( &self, b )  {
        "Write the given buffer to the IO stream.

        Returns the number of bytes written, which may be less than the
        length of b in bytes.
        ";
        self . _unsupported ( "write" );
        io . RawIOBase . register ( RawIOBase );
        from _io import FileIO;
        RawIOBase . register ( FileIO );
        class BufferedIOBase ( IOBase ) ;
        "Base class for buffered IO objects.

    The main difference with RawIOBase == that the read() method
    supports omitting the size argument, && does !have a default
    implementation that defers to readinto().

    In addition, read(), readinto() && write() may raise
    BlockingIOError if the underlying raw stream == in non-blocking
    mode && !ready; unlike their raw counterparts, they will never
    return None /* Option */.

    A typical implementation should !inherit from a RawIOBase
    implementation, but wrap one.
    ";
        pub fn read ( &self, size = -1 )  {
        "Read && return up to size bytes, where size == an int.

        If the argument == omitted, None /* Option */, || negative, reads and
        returns all data until EOF.

        If the argument == positive, && the underlying raw stream is
        !'interactive', multiple raw reads may be issued to satisfy
        the byte count (unless EOF == reached first).  But for
        interactive raw streams (XXX && for pipes?), at most one raw
        read will be issued, && a short result does !imply that
        EOF == imminent.

        Returns an empty bytes array on EOF.

        Raises BlockingIOError if the underlying raw stream has no
        data at the moment.
        ";
        self . _unsupported ( "read" );
        pub fn read1 ( &self, size = -1 )  {
        "Read up to size bytes with at most one read() system call,
        where size == an int.
        ";
        self . _unsupported ( "read1" );
        pub fn readinto ( &self, b )  {
        "Read bytes into a pre-allocated bytes-like object b.

        Like read(), this may issue multiple reads to the underlying raw
        stream, unless the latter == 'interactive'.

        Returns an int representing the number of bytes read (0 for EOF).

        Raises BlockingIOError if the underlying raw stream has no
        data at the moment.
        ";
        return  self . _readinto ( b , read1 = false );
        pub fn readinto1 ( &self, b )  {
        "Read bytes into buffer *b*, using at most one system call

        Returns an int representing the number of bytes read (0 for EOF).

        Raises BlockingIOError if the underlying raw stream has no
        data at the moment.
        ";
        return  self . _readinto ( b , read1 = true );
        pub fn _readinto ( &self, b , read1 )  {
        if !isinstance ( b , memoryview ) {
        b = memoryview ( b );
        b = b . cast ( "B" );
        if read1 {
        data = self . read1 ( len ( b ) );
        } else {
        data = self . read ( len ( b ) );
        n = len ( data );
        b [ : n ] = data;
        return  n;
        pub fn write ( &self, b )  {
        "Write the given bytes buffer to the IO stream.

        Return the number of bytes written, which == always the length of b
        in bytes.

        Raises BlockingIOError if the buffer == full && the
        underlying raw stream cannot accept more data at the moment.
        ";
        self . _unsupported ( "write" );
        pub fn detach ( self )  {
        "
        Separate the underlying raw stream from the buffer && return it.

        After the raw stream has been detached, the buffer == in an unusable
        state.
        ";
        self . _unsupported ( "detach" );
        io . BufferedIOBase . register ( BufferedIOBase );
        class _BufferedIOMixin ( BufferedIOBase ) ;
        "A mixin implementation of BufferedIOBase with an underlying raw stream.

    This passes most requests on to the underlying raw stream.  It
    does *not* provide implementations of read(), readinto() or
    write().
    ";
        pub fn __init__ ( &self, raw )  {
        self . _raw = raw;
        pub fn seek ( &self, pos , whence = 0 )  {
        new_position = self . raw . seek ( pos , whence );
        if new_position < 0 {
        panic!("OSError ( "seek() returned an invalid position" )");
        return  new_position;
        pub fn tell ( self )  {
        pos = self . raw . tell ( );
        if pos < 0 {
        panic!("OSError ( "tell() returned an invalid position" )");
        return  pos;
        pub fn truncate ( &self, pos = None /* Option */ )  {
        self . _checkClosed ( );
        self . _checkWritable ( );
        self . flush ( );
        if pos is None /* Option */ {
        pos = self . tell ( );
        return  self . raw . truncate ( pos );
        pub fn flush ( self )  {
        if self . closed {
        panic!("ValueError ( "flush on closed file" )");
        self . raw . flush ( );
        pub fn close ( self )  {
        if self . raw is !None /* Option */ && !self . closed {
        // try {
        self . flush ( );
        // } finally {
        self . raw . close ( );
        pub fn detach ( self )  {
        if self . raw is None /* Option */ {
        panic!("ValueError ( "raw stream already detached" )");
        self . flush ( );
        raw = self . _raw;
        self . _raw = None /* Option */;
        return  raw;
        pub fn seekable ( self )  {
        return  self . raw . seekable ( );
        @ property;
        pub fn raw ( self )  {
        return  self . _raw;
        @ property;
        pub fn closed ( self )  {
        return  self . raw . closed;
        @ property;
        pub fn name ( self )  {
        return  self . raw . name;
        @ property;
        pub fn mode ( self )  {
        return  self . raw . mode;
        pub fn __getstate__ ( self )  {
        panic!("TypeError ( f "cannot pickle {self.__class__.__name__!r} object" )");
        pub fn __repr__ ( self )  {
        modname = self . __class__ . __module__;
        clsname = self . __class__ . __qualname__;
        // try {
        name = self . name;
        // } catch  AttributeError  {
        return  "<{}.{}>" . format ( modname , clsname );
        } else {
        return  "<{}.{} name={!r}>" . format ( modname , clsname , name );
        pub fn fileno ( self )  {
        return  self . raw . fileno ( );
        pub fn isatty ( self )  {
        return  self . raw . isatty ( );
        class BytesIO ( BufferedIOBase ) ;
        "Buffered I/O implementation using an in-memory bytes buffer.";
        _buffer = None /* Option */;
        pub fn __init__ ( &self, initial_bytes = None /* Option */ )  {
        buf = bytearray ( );
        if initial_bytes is !None /* Option */ {
        buf + = initial_bytes;
        self . _buffer = buf;
        self . _pos = 0;
        pub fn __getstate__ ( self )  {
        if self . closed {
        panic!("ValueError ( "__getstate__ on closed file" )");
        return  self . __dict__ . copy ( );
        pub fn getvalue ( self )  {
        "Return the bytes value (contents) of the buffer
        ";
        if self . closed {
        panic!("ValueError ( "getvalue on closed file" )");
        return  bytes ( self . _buffer );
        pub fn getbuffer ( self )  {
        "Return a readable && writable view of the buffer.
        ";
        if self . closed {
        panic!("ValueError ( "getbuffer on closed file" )");
        return  memoryview ( self . _buffer );
        pub fn close ( self )  {
        if self . _buffer is !None /* Option */ {
        self . _buffer . clear ( );
        super ( ) . close ( );
        pub fn read ( &self, size = -1 )  {
        if self . closed {
        panic!("ValueError ( "read from closed file" )");
        if size is None /* Option */ {
        size = -1;
        } else {
        // try {
        size_index = size . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{size!r} is !an integer" )");
        } else {
        size = size_index ( );
        if size < 0 {
        size = len ( self . _buffer );
        if len ( self . _buffer ) <= self . _pos {
        return  b "";
        newpos = min ( len ( self . _buffer ) , self . _pos + size );
        b = self . _buffer [ self . _pos : newpos ];
        self . _pos = newpos;
        return  bytes ( b );
        pub fn read1 ( &self, size = -1 )  {
        "This == the same as read.
        ";
        return  self . read ( size );
        pub fn write ( &self, b )  {
        if self . closed {
        panic!("ValueError ( "write to closed file" )");
        if isinstance ( b , str ) {
        panic!("TypeError ( "can't write str to binary stream" )");
        // with scope: memoryview ( b ) as view  {
        n = view . nbytes;
        if n == 0 {
        return  0;
        pos = self . _pos;
        if pos > len ( self . _buffer ) {
        padding = b "\x00" * ( pos - len ( self . _buffer ) );
        self . _buffer + = padding;
        self . _buffer [ pos : pos + n ] = b;
        self . _pos + = n;
        return  n;
        pub fn seek ( &self, pos , whence = 0 )  {
        if self . closed {
        panic!("ValueError ( "seek on closed file" )");
        // try {
        pos_index = pos . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{pos!r} is !an integer" )");
        } else {
        pos = pos_index ( );
        if whence == 0 {
        if pos < 0 {
        panic!("ValueError ( "negative seek position %r" % ( pos , ) )");
        self . _pos = pos;
        } else if whence == 1 {
        self . _pos = max ( 0 , self . _pos + pos );
        } else if whence == 2 {
        self . _pos = max ( 0 , len ( self . _buffer ) + pos );
        } else {
        panic!("ValueError ( "unsupported whence value" )");
        return  self . _pos;
        pub fn tell ( self )  {
        if self . closed {
        panic!("ValueError ( "tell on closed file" )");
        return  self . _pos;
        pub fn truncate ( &self, pos = None /* Option */ )  {
        if self . closed {
        panic!("ValueError ( "truncate on closed file" )");
        if pos is None /* Option */ {
        pos = self . _pos;
        } else {
        // try {
        pos_index = pos . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{pos!r} is !an integer" )");
        } else {
        pos = pos_index ( );
        if pos < 0 {
        panic!("ValueError ( "negative truncate position %r" % ( pos , ) )");
        del self . _buffer [ pos : ];
        return  pos;
        pub fn readable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  true;
        pub fn writable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  true;
        pub fn seekable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  true;
        class BufferedReader ( _BufferedIOMixin ) ;
        "BufferedReader(raw[, buffer_size])

    A buffer for a readable, sequential BaseRawIO object.

    The constructor creates a BufferedReader for the given readable raw
    stream && buffer_size. If buffer_size == omitted, DEFAULT_BUFFER_SIZE
    == used.
    ";
        pub fn __init__ ( &self, raw , buffer_size = DEFAULT_BUFFER_SIZE )  {
        "Create a new buffered reader using the given readable raw IO object.
        ";
        if !raw . readable ( ) {
        panic!("OSError ( ""raw" argument must be readable." )");
        _BufferedIOMixin . __init__ ( self , raw );
        if buffer_size <= 0 {
        panic!("ValueError ( "invalid buffer size" )");
        self . buffer_size = buffer_size;
        self . _reset_read_buf ( );
        self . _read_lock = Lock ( );
        pub fn readable ( self )  {
        return  self . raw . readable ( );
        pub fn _reset_read_buf ( self )  {
        self . _read_buf = b "";
        self . _read_pos = 0;
        pub fn read ( &self, size = None /* Option */ )  {
        "Read size bytes.

        Returns exactly size bytes of data unless the underlying raw IO
        stream reaches EOF || if the call would block in non-blocking
        mode. If size == negative, read until EOF || until read() would
        block.
        ";
        if size is !None /* Option */ && size < -1 {
        panic!("ValueError ( "invalid number of bytes to read" )");
        // with scope: self . _read_lock  {
        return  self . _read_unlocked ( size );
        pub fn _read_unlocked ( &self, n = None /* Option */ )  {
        nodata_val = b "";
        empty_values = ( b "" , None /* Option */ );
        buf = self . _read_buf;
        pos = self . _read_pos;
        if n is None /* Option */ || n == -1 {
        self . _reset_read_buf ( );
        if hasattr ( self . raw , "readall" ) {
        chunk = self . raw . readall ( );
        if chunk is None /* Option */ {
        return  buf [ pos : ] || None /* Option */;
        } else {
        return  buf [ pos : ] + chunk;
        chunks = [ buf [ pos : ] ];
        current_size = 0;
        while true  {
        chunk = self . raw . read ( );
        if chunk in empty_values {
        nodata_val = chunk;
        break;
        current_size + = len ( chunk );
        chunks . append ( chunk );
        return  b "" . join ( chunks ) || nodata_val;
        avail = len ( buf ) - pos;
        if n <= avail {
        self . _read_pos + = n;
        return  buf [ pos : pos + n ];
        chunks = [ buf [ pos : ] ];
        wanted = max ( self . buffer_size , n );
        while avail < n  {
        chunk = self . raw . read ( wanted );
        if chunk in empty_values {
        nodata_val = chunk;
        break;
        avail + = len ( chunk );
        chunks . append ( chunk );
        n = min ( n , avail );
        out = b "" . join ( chunks );
        self . _read_buf = out [ n : ];
        self . _read_pos = 0;
        return  out [ : n ] if out else nodata_val;
        pub fn peek ( &self, size = 0 )  {
        "Returns buffered bytes without advancing the position.

        The argument indicates a desired minimal number of bytes; we
        do at most one raw read to satisfy it.  We never return more
        than self.buffer_size.
        ";
        // with scope: self . _read_lock  {
        return  self . _peek_unlocked ( size );
        pub fn _peek_unlocked ( &self, n = 0 )  {
        want = min ( n , self . buffer_size );
        have = len ( self . _read_buf ) - self . _read_pos;
        if have < want || have <= 0 {
        to_read = self . buffer_size - have;
        current = self . raw . read ( to_read );
        if current {
        self . _read_buf = self . _read_buf [ self . _read_pos : ] + current;
        self . _read_pos = 0;
        return  self . _read_buf [ self . _read_pos : ];
        pub fn read1 ( &self, size = -1 )  {
        "Reads up to size bytes, with at most one read() system call.";
        if size < 0 {
        size = self . buffer_size;
        if size == 0 {
        return  b "";
        // with scope: self . _read_lock  {
        self . _peek_unlocked ( 1 );
        return  self . _read_unlocked (;
        min ( size , len ( self . _read_buf ) - self . _read_pos ) );
        pub fn _readinto ( &self, buf , read1 )  {
        "Read data into *buf* with at most one system call.";
        if !isinstance ( buf , memoryview ) {
        buf = memoryview ( buf );
        if buf . nbytes == 0 {
        return  0;
        buf = buf . cast ( "B" );
        written = 0;
        // with scope: self . _read_lock  {
        while written < len ( buf )  {
        avail = min ( len ( self . _read_buf ) - self . _read_pos , len ( buf ) );
        if avail {
        buf [ written : written + avail ] = \;
        self . _read_buf [ self . _read_pos : self . _read_pos + avail ];
        self . _read_pos + = avail;
        written + = avail;
        if written == len ( buf ) {
        break;
        if len ( buf ) - written > self . buffer_size {
        n = self . raw . readinto ( buf [ written : ] );
        if !n {
        break;
        written + = n;
        } else if !( read1 && written ) {
        if !self . _peek_unlocked ( 1 ) {
        break;
        if read1 && written {
        break;
        return  written;
        pub fn tell ( self )  {
        return  max ( _BufferedIOMixin . tell ( self ) - len ( self . _read_buf ) + self . _read_pos , 0 );
        pub fn seek ( &self, pos , whence = 0 )  {
        if whence !in valid_seek_flags {
        panic!("ValueError ( "invalid whence value" )");
        // with scope: self . _read_lock  {
        if whence == 1 {
        pos - = len ( self . _read_buf ) - self . _read_pos;
        pos = _BufferedIOMixin . seek ( self , pos , whence );
        self . _reset_read_buf ( );
        return  pos;
        class BufferedWriter ( _BufferedIOMixin ) ;
        "A buffer for a writeable sequential RawIO object.

    The constructor creates a BufferedWriter for the given writeable raw
    stream. If the buffer_size == !given, it defaults to
    DEFAULT_BUFFER_SIZE.
    ";
        pub fn __init__ ( &self, raw , buffer_size = DEFAULT_BUFFER_SIZE )  {
        if !raw . writable ( ) {
        panic!("OSError ( ""raw" argument must be writable." )");
        _BufferedIOMixin . __init__ ( self , raw );
        if buffer_size <= 0 {
        panic!("ValueError ( "invalid buffer size" )");
        self . buffer_size = buffer_size;
        self . _write_buf = bytearray ( );
        self . _write_lock = Lock ( );
        pub fn writable ( self )  {
        return  self . raw . writable ( );
        pub fn write ( &self, b )  {
        if isinstance ( b , str ) {
        panic!("TypeError ( "can't write str to binary stream" )");
        // with scope: self . _write_lock  {
        if self . closed {
        panic!("ValueError ( "write to closed file" )");
        if len ( self . _write_buf ) > self . buffer_size {
        self . _flush_unlocked ( );
        before = len ( self . _write_buf );
        self . _write_buf . extend ( b );
        written = len ( self . _write_buf ) - before;
        if len ( self . _write_buf ) > self . buffer_size {
        // try {
        self . _flush_unlocked ( );
        // } catch  BlockingIOError as e  {
        if len ( self . _write_buf ) > self . buffer_size {
        overage = len ( self . _write_buf ) - self . buffer_size;
        written - = overage;
        self . _write_buf = self . _write_buf [ : self . buffer_size ];
        panic!("BlockingIOError ( e . errno , e . strerror , written )");
        return  written;
        pub fn truncate ( &self, pos = None /* Option */ )  {
        // with scope: self . _write_lock  {
        self . _flush_unlocked ( );
        if pos is None /* Option */ {
        pos = self . raw . tell ( );
        return  self . raw . truncate ( pos );
        pub fn flush ( self )  {
        // with scope: self . _write_lock  {
        self . _flush_unlocked ( );
        pub fn _flush_unlocked ( self )  {
        if self . closed {
        panic!("ValueError ( "flush on closed file" )");
        while self . _write_buf  {
        // try {
        n = self . raw . write ( self . _write_buf );
        // } catch  BlockingIOError  {
        panic!("RuntimeError ( "self.raw should implement RawIOBase: it "");
        "should !raise BlockingIOError" );
        if n is None /* Option */ {
        panic!("BlockingIOError (");
        errno . EAGAIN ,;
        "write could !complete without blocking" , 0 );
        if n > len ( self . _write_buf ) || n < 0 {
        panic!("OSError ( "write() returned incorrect number of bytes" )");
        del self . _write_buf [ : n ];
        pub fn tell ( self )  {
        return  _BufferedIOMixin . tell ( self ) + len ( self . _write_buf );
        pub fn seek ( &self, pos , whence = 0 )  {
        if whence !in valid_seek_flags {
        panic!("ValueError ( "invalid whence value" )");
        // with scope: self . _write_lock  {
        self . _flush_unlocked ( );
        return  _BufferedIOMixin . seek ( self , pos , whence );
        pub fn close ( self )  {
        // with scope: self . _write_lock  {
        if self . raw is None /* Option */ || self . closed {
        return;
        // try {
        self . flush ( );
        // } finally {
        // with scope: self . _write_lock  {
        self . raw . close ( );
        class BufferedRWPair ( BufferedIOBase ) ;
        "A buffered reader && writer object together.

    A buffered reader object && buffered writer object put together to
    form a sequential IO object that can read && write. This == typically
    used with a socket || two-way pipe.

    reader && writer are RawIOBase objects that are readable and
    writeable respectively. If the buffer_size == omitted it defaults to
    DEFAULT_BUFFER_SIZE.
    ";
        pub fn __init__ ( &self, reader , writer , buffer_size = DEFAULT_BUFFER_SIZE )  {
        "Constructor.

        The arguments are two RawIO instances.
        ";
        if !reader . readable ( ) {
        panic!("OSError ( ""reader" argument must be readable." )");
        if !writer . writable ( ) {
        panic!("OSError ( ""writer" argument must be writable." )");
        self . reader = BufferedReader ( reader , buffer_size );
        self . writer = BufferedWriter ( writer , buffer_size );
        pub fn read ( &self, size = -1 )  {
        if size is None /* Option */ {
        size = -1;
        return  self . reader . read ( size );
        pub fn readinto ( &self, b )  {
        return  self . reader . readinto ( b );
        pub fn write ( &self, b )  {
        return  self . writer . write ( b );
        pub fn peek ( &self, size = 0 )  {
        return  self . reader . peek ( size );
        pub fn read1 ( &self, size = -1 )  {
        return  self . reader . read1 ( size );
        pub fn readinto1 ( &self, b )  {
        return  self . reader . readinto1 ( b );
        pub fn readable ( self )  {
        return  self . reader . readable ( );
        pub fn writable ( self )  {
        return  self . writer . writable ( );
        pub fn flush ( self )  {
        return  self . writer . flush ( );
        pub fn close ( self )  {
        // try {
        self . writer . close ( );
        // } finally {
        self . reader . close ( );
        pub fn isatty ( self )  {
        return  self . reader . isatty ( ) || self . writer . isatty ( );
        @ property;
        pub fn closed ( self )  {
        return  self . writer . closed;
        class BufferedRandom ( BufferedWriter , BufferedReader ) ;
        "A buffered interface to random access streams.

    The constructor creates a reader && writer for a seekable stream,
    raw, given in the first argument. If the buffer_size == omitted it
    defaults to DEFAULT_BUFFER_SIZE.
    ";
        pub fn __init__ ( &self, raw , buffer_size = DEFAULT_BUFFER_SIZE )  {
        raw . _checkSeekable ( );
        BufferedReader . __init__ ( self , raw , buffer_size );
        BufferedWriter . __init__ ( self , raw , buffer_size );
        pub fn seek ( &self, pos , whence = 0 )  {
        if whence !in valid_seek_flags {
        panic!("ValueError ( "invalid whence value" )");
        self . flush ( );
        if self . _read_buf {
        // with scope: self . _read_lock  {
        self . raw . seek ( self . _read_pos - len ( self . _read_buf ) , 1 );
        pos = self . raw . seek ( pos , whence );
        // with scope: self . _read_lock  {
        self . _reset_read_buf ( );
        if pos < 0 {
        panic!("OSError ( "seek() returned invalid position" )");
        return  pos;
        pub fn tell ( self )  {
        if self . _write_buf {
        return  BufferedWriter . tell ( self );
        } else {
        return  BufferedReader . tell ( self );
        pub fn truncate ( &self, pos = None /* Option */ )  {
        if pos is None /* Option */ {
        pos = self . tell ( );
        return  BufferedWriter . truncate ( self , pos );
        pub fn read ( &self, size = None /* Option */ )  {
        if size is None /* Option */ {
        size = -1;
        self . flush ( );
        return  BufferedReader . read ( self , size );
        pub fn readinto ( &self, b )  {
        self . flush ( );
        return  BufferedReader . readinto ( self , b );
        pub fn peek ( &self, size = 0 )  {
        self . flush ( );
        return  BufferedReader . peek ( self , size );
        pub fn read1 ( &self, size = -1 )  {
        self . flush ( );
        return  BufferedReader . read1 ( self , size );
        pub fn readinto1 ( &self, b )  {
        self . flush ( );
        return  BufferedReader . readinto1 ( self , b );
        pub fn write ( &self, b )  {
        if self . _read_buf {
        // with scope: self . _read_lock  {
        self . raw . seek ( self . _read_pos - len ( self . _read_buf ) , 1 );
        self . _reset_read_buf ( );
        return  BufferedWriter . write ( self , b );
        class FileIO ( RawIOBase ) ;
        _fd = -1;
        _created = false;
        _readable = false;
        _writable = false;
        _appending = false;
        _seekable = None /* Option */;
        _closefd = true;
        pub fn __init__ ( &self, file , mode = "r" , closefd = true , opener = None /* Option */ )  {
        "Open a file.  The mode can be 'r' (default), 'w', 'x' || 'a' for reading,
        writing, exclusive creation || appending.  The file will be created if it
        doesn't exist when opened for writing || appending; it will be truncated
        when opened for writing.  A FileExistsError will be raised if it already
        exists when opened for creating. Opening a file for creating implies
        writing so this mode behaves in a similar way to 'w'. Add a '+' to the mode
        to allow simultaneous reading && writing. A custom opener can be used by
        passing a callable as *opener*. The underlying file descriptor for the file
        object == then obtained by calling opener with (*name*, *flags*).
        *opener* must return an open file descriptor (passing os.open as *opener*
        results in functionality similar to passing None /* Option */).
        ";
        if self . _fd >= 0 {
        // try {
        if self . _closefd {
        os . close ( self . _fd );
        // } finally {
        self . _fd = -1;
        if isinstance ( file , float ) {
        panic!("TypeError ( "integer argument expected, got float" )");
        if isinstance ( file , int ) {
        fd = file;
        if fd < 0 {
        panic!("ValueError ( "negative file descriptor" )");
        } else {
        fd = -1;
        if !isinstance ( mode , str ) {
        panic!("TypeError ( "invalid mode: %s" % ( mode , ) )");
        if !set ( mode ) <= set ( "xrwab+" ) {
        panic!("ValueError ( "invalid mode: %s" % ( mode , ) )");
        if sum ( c in "rwax" for c in mode ) != 1 || mode . count ( "+" ) > 1 {
        panic!("ValueError ( "Must have exactly one of create/read/write/append "");
        "mode && at most one plus" );
        if "x" in mode {
        self . _created = true;
        self . _writable = true;
        flags = os . O_EXCL | os . O_CREAT;
        } else if "r" in mode {
        self . _readable = true;
        flags = 0;
        } else if "w" in mode {
        self . _writable = true;
        flags = os . O_CREAT | os . O_TRUNC;
        } else if "a" in mode {
        self . _writable = true;
        self . _appending = true;
        flags = os . O_APPEND | os . O_CREAT;
        if "+" in mode {
        self . _readable = true;
        self . _writable = true;
        if self . _readable && self . _writable {
        flags | = os . O_RDWR;
        } else if self . _readable {
        flags | = os . O_RDONLY;
        } else {
        flags | = os . O_WRONLY;
        flags | = getattr ( os , "O_BINARY" , 0 );
        noinherit_flag = ( getattr ( os , "O_NOINHERIT" , 0 ) or;
        getattr ( os , "O_CLOEXEC" , 0 ) );
        flags | = noinherit_flag;
        owned_fd = None /* Option */;
        // try {
        if fd < 0 {
        if !closefd {
        panic!("ValueError ( "Cannot use closefd=false with file name" )");
        if opener is None /* Option */ {
        fd = os . open ( file , flags , 0 o666 );
        } else {
        fd = opener ( file , flags );
        if !isinstance ( fd , int ) {
        panic!("TypeError ( "expected integer from opener" )");
        if fd < 0 {
        panic!("OSError ( "Negative file descriptor" )");
        owned_fd = fd;
        if !noinherit_flag {
        os . set_inheritable ( fd , false );
        self . _closefd = closefd;
        fdfstat = os . fstat ( fd );
        // try {
        if stat . S_ISDIR ( fdfstat . st_mode ) {
        panic!("IsADirectoryError ( errno . EISDIR ,");
        os . strerror ( errno . EISDIR ) , file );
        // } catch  AttributeError  {
        // pass
        self . _blksize = getattr ( fdfstat , "st_blksize" , 0 );
        if self . _blksize <= 1 {
        self . _blksize = DEFAULT_BUFFER_SIZE;
        if _setmode {
        _setmode ( fd , os . O_BINARY );
        self . name = file;
        if self . _appending {
        // try {
        os . lseek ( fd , 0 , SEEK_END );
        // } catch  OSError as e  {
        if e . errno != errno . ESPIPE {
        panic!("");
        // } catch   {
        if owned_fd is !None /* Option */ {
        os . close ( owned_fd );
        panic!("");
        self . _fd = fd;
        pub fn __del__ ( self )  {
        if self . _fd >= 0 && self . _closefd && !self . closed {
        import warnings;
        warnings . warn ( "unclosed file %r" % ( self , ) , ResourceWarning ,;
        stacklevel = 2 , source = self );
        self . close ( );
        pub fn __getstate__ ( self )  {
        panic!("TypeError ( f "cannot pickle {self.__class__.__name__!r} object" )");
        pub fn __repr__ ( self )  {
        class_name = "%s.%s" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ );
        if self . closed {
        return  "<%s [closed]>" % class_name;
        // try {
        name = self . name;
        // } catch  AttributeError  {
        return  ( "<%s fd=%d mode=%r closefd=%r>" %;
        ( class_name , self . _fd , self . mode , self . _closefd ) );
        } else {
        return  ( "<%s name=%r mode=%r closefd=%r>" %;
        ( class_name , name , self . mode , self . _closefd ) );
        pub fn _checkReadable ( self )  {
        if !self . _readable {
        panic!("UnsupportedOperation ( "File !open for reading" )");
        pub fn _checkWritable ( &self, msg = None /* Option */ )  {
        if !self . _writable {
        panic!("UnsupportedOperation ( "File !open for writing" )");
        pub fn read ( &self, size = None /* Option */ )  {
        "Read at most size bytes, returned as bytes.

        Only makes one system call, so less data may be returned than requested
        In non-blocking mode, returns None /* Option */ if no data == available.
        Return an empty bytes object at EOF.
        ";
        self . _checkClosed ( );
        self . _checkReadable ( );
        if size is None /* Option */ || size < 0 {
        return  self . readall ( );
        // try {
        return  os . read ( self . _fd , size );
        // } catch  BlockingIOError  {
        return;
        pub fn readall ( self )  {
        "Read all data from the file, returned as bytes.

        In non-blocking mode, returns as much as == immediately available,
        || None /* Option */ if no data == available.  Return an empty bytes object at EOF.
        ";
        self . _checkClosed ( );
        self . _checkReadable ( );
        bufsize = DEFAULT_BUFFER_SIZE;
        // try {
        pos = os . lseek ( self . _fd , 0 , SEEK_CUR );
        end = os . fstat ( self . _fd ) . st_size;
        if end >= pos {
        bufsize = end - pos + 1;
        // } catch  OSError  {
        // pass
        result = bytearray ( );
        while true  {
        if len ( result ) >= bufsize {
        bufsize = len ( result );
        bufsize + = max ( bufsize , DEFAULT_BUFFER_SIZE );
        n = bufsize - len ( result );
        // try {
        chunk = os . read ( self . _fd , n );
        // } catch  BlockingIOError  {
        if result {
        break;
        return;
        if !chunk {
        break;
        result + = chunk;
        return  bytes ( result );
        pub fn readinto ( &self, b )  {
        "Same as RawIOBase.readinto().";
        m = memoryview ( b ) . cast ( "B" );
        data = self . read ( len ( m ) );
        n = len ( data );
        m [ : n ] = data;
        return  n;
        pub fn write ( &self, b )  {
        "Write bytes b to file, return number written.

        Only makes one system call, so !all of the data may be written.
        The number of bytes actually written == returned.  In non-blocking mode,
        returns None /* Option */ if the write would block.
        ";
        self . _checkClosed ( );
        self . _checkWritable ( );
        // try {
        return  os . write ( self . _fd , b );
        // } catch  BlockingIOError  {
        return;
        pub fn seek ( &self, pos , whence = SEEK_SET )  {
        "Move to new file position.

        Argument offset == a byte count.  Optional argument whence defaults to
        SEEK_SET || 0 (offset from start of file, offset should be >= 0); other values
        are SEEK_CUR || 1 (move relative to current position, positive || negative),
        && SEEK_END || 2 (move relative to end of file, usually negative, although
        many platforms allow seeking beyond the end of a file).

        Note that !all file objects are seekable.
        ";
        if isinstance ( pos , float ) {
        panic!("TypeError ( "an integer is required" )");
        self . _checkClosed ( );
        return  os . lseek ( self . _fd , pos , whence );
        pub fn tell ( self )  {
        "tell() -> int.  Current file position.

        Can raise OSError for non seekable files.";
        self . _checkClosed ( );
        return  os . lseek ( self . _fd , 0 , SEEK_CUR );
        pub fn truncate ( &self, size = None /* Option */ )  {
        "Truncate the file to at most size bytes.

        Size defaults to the current file position, as returned by tell().
        The current file position == changed to the value of size.
        ";
        self . _checkClosed ( );
        self . _checkWritable ( );
        if size is None /* Option */ {
        size = self . tell ( );
        os . ftruncate ( self . _fd , size );
        return  size;
        pub fn close ( self )  {
        "Close the file.

        A closed file cannot be used for further I/O operations.  close() may be
        called more than once without error.
        ";
        if !self . closed {
        // try {
        if self . _closefd {
        os . close ( self . _fd );
        // } finally {
        super ( ) . close ( );
        pub fn seekable ( self )  {
        "true if file supports random-access.";
        self . _checkClosed ( );
        if self . _seekable is None /* Option */ {
        // try {
        self . tell ( );
        // } catch  OSError  {
        self . _seekable = false;
        } else {
        self . _seekable = true;
        return  self . _seekable;
        pub fn readable ( self )  {
        "true if file was opened in a read mode.";
        self . _checkClosed ( );
        return  self . _readable;
        pub fn writable ( self )  {
        "true if file was opened in a write mode.";
        self . _checkClosed ( );
        return  self . _writable;
        pub fn fileno ( self )  {
        "Return the underlying file descriptor (an integer).";
        self . _checkClosed ( );
        return  self . _fd;
        pub fn isatty ( self )  {
        "true if the file == connected to a TTY device.";
        self . _checkClosed ( );
        return  os . isatty ( self . _fd );
        @ property;
        pub fn closefd ( self )  {
        "true if the file descriptor will be closed by close().";
        return  self . _closefd;
        @ property;
        pub fn mode ( self )  {
        "String giving the file mode";
        if self . _created {
        if self . _readable {
        return  "xb+";
        } else {
        return  "xb";
        } else if self . _appending {
        if self . _readable {
        return  "ab+";
        } else {
        return  "ab";
        } else if self . _readable {
        if self . _writable {
        return  "rb+";
        } else {
        return  "rb";
        } else {
        return  "wb";
        class TextIOBase ( IOBase ) ;
        "Base class for text I/O.

    This class provides a character && line based interface to stream
    I/O.
    ";
        pub fn read ( &self, size = -1 )  {
        "Read at most size characters from stream, where size == an int.

        Read from underlying buffer until we have size characters || we hit EOF.
        If size == negative || omitted, read until EOF.

        Returns a string.
        ";
        self . _unsupported ( "read" );
        pub fn write ( &self, s )  {
        "Write string s to stream && returning an int.";
        self . _unsupported ( "write" );
        pub fn truncate ( &self, pos = None /* Option */ )  {
        "Truncate size to pos, where pos == an int.";
        self . _unsupported ( "truncate" );
        pub fn readline ( self )  {
        "Read until newline || EOF.

        Returns an empty string if EOF == hit immediately.
        ";
        self . _unsupported ( "readline" );
        pub fn detach ( self )  {
        "
        Separate the underlying buffer from the TextIOBase && return it.

        After the underlying buffer has been detached, the TextIO == in an
        unusable state.
        ";
        self . _unsupported ( "detach" );
        @ property;
        pub fn encoding ( self )  {
        "Subclasses should override.";
        return;
        @ property;
        pub fn newlines ( self )  {
        "Line endings translated so far.

        Only line endings translated during reading are considered.

        Subclasses should override.
        ";
        return;
        @ property;
        pub fn errors ( self )  {
        "Error setting of the decoder || encoder.

        Subclasses should override.";
        return;
        io . TextIOBase . register ( TextIOBase );
        class IncrementalNewlineDecoder ( codecs . IncrementalDecoder ) ;
        r "Codec used when reading a file in universal newlines mode.  It wraps
    another incremental decoder, translating \r\n && \r into \n.  It also
    records the types of newlines encountered.  When used with
    translate=false, it ensures that the newline sequence == returned in
    one piece.
    ";
        pub fn __init__ ( &self, decoder , translate , errors = "strict" )  {
        codecs . IncrementalDecoder . __init__ ( self , errors = errors );
        self . translate = translate;
        self . decoder = decoder;
        self . seennl = 0;
        self . pendingcr = false;
        pub fn decode ( &self, input , final = false )  {
        if self . decoder is None /* Option */ {
        output = input;
        } else {
        output = self . decoder . decode ( input , final = final );
        if self . pendingcr && ( output || final ) {
        output = "\r" + output;
        self . pendingcr = false;
        if output . endswith ( "\r" ) && !final {
        output = output [ : -1 ];
        self . pendingcr = true;
        crlf = output . count ( "\r\n" );
        cr = output . count ( "\r" ) - crlf;
        lf = output . count ( "\n" ) - crlf;
        self . seennl | = ( lf && self . _LF ) | ( cr && self . _CR ) \;
        | ( crlf && self . _CRLF );
        if self . translate {
        if crlf {
        output = output . replace ( "\r\n" , "\n" );
        if cr {
        output = output . replace ( "\r" , "\n" );
        return  output;
        pub fn getstate ( self )  {
        if self . decoder is None /* Option */ {
        buf = b "";
        flag = 0;
        } else {
        buf , flag = self . decoder . getstate ( );
        flag < <= 1;
        if self . pendingcr {
        flag | = 1;
        return  buf , flag;
        pub fn setstate ( &self, state )  {
        buf , flag = state;
        self . pendingcr = bool ( flag & 1 );
        if self . decoder is !None /* Option */ {
        self . decoder . setstate ( ( buf , flag > > 1 ) );
        pub fn reset ( self )  {
        self . seennl = 0;
        self . pendingcr = false;
        if self . decoder is !None /* Option */ {
        self . decoder . reset ( );
        _LF = 1;
        _CR = 2;
        _CRLF = 4;
        @ property;
        pub fn newlines ( self )  {
        return  ( None /* Option */ ,;
        "\n" ,;
        "\r" ,;
        ( "\r" , "\n" ) ,;
        "\r\n" ,;
        ( "\n" , "\r\n" ) ,;
        ( "\r" , "\r\n" ) ,;
        ( "\r" , "\n" , "\r\n" );
        ) [ self . seennl ];
        class TextIOWrapper ( TextIOBase ) ;
        r "Character && line based layer over a BufferedIOBase object, buffer.

    encoding gives the name of the encoding that the stream will be
    decoded || encoded with. It defaults to locale.getencoding().

    errors determines the strictness of encoding && decoding (see the
    codecs.register) && defaults to "strict".

    newline can be None /* Option */, '', '\n', '\r', || '\r\n'.  It controls the
    handling of line endings. If it == None /* Option */, universal newlines is
    enabled.  With this enabled, on input, the lines endings '\n', '\r',
    || '\r\n' are translated to '\n' before being returned to the
    caller. Conversely, on output, '\n' == translated to the system
    default line separator, os.linesep. If newline == any other of its
    legal values, that newline becomes the newline when the file == read
    && it == returned untranslated. On output, '\n' == converted to the
    newline.

    If line_buffering == true, a call to flush == implied when a call to
    write contains a newline character.
    ";
        _CHUNK_SIZE = 2048;
        _buffer = None /* Option */;
        pub fn __init__ ( &self, buffer , encoding = None /* Option */ , errors = None /* Option */ , newline = None /* Option */ , {
        line_buffering = false , write_through = false ) ;
        self . _check_newline ( newline );
        encoding = text_encoding ( encoding );
        if encoding == "locale" {
        encoding = self . _get_locale_encoding ( );
        if !isinstance ( encoding , str ) {
        panic!("ValueError ( "invalid encoding: %r" % encoding )");
        if !codecs . lookup ( encoding ) . _is_text_encoding {
        msg = ( "%r == !a text encoding; ";
        "use codecs.open() to handle arbitrary codecs" );
        panic!("LookupError ( msg % encoding )");
        if errors is None /* Option */ {
        errors = "strict";
        } else {
        if !isinstance ( errors , str ) {
        panic!("ValueError ( "invalid errors: %r" % errors )");
        if _CHECK_ERRORS {
        codecs . lookup_error ( errors );
        self . _buffer = buffer;
        self . _decoded_chars = "";
        self . _decoded_chars_used = 0;
        self . _snapshot = None /* Option */;
        self . _seekable = self . _telling = self . buffer . seekable ( );
        self . _has_read1 = hasattr ( self . buffer , "read1" );
        self . _configure ( encoding , errors , newline ,;
        line_buffering , write_through );
        pub fn _check_newline ( &self, newline )  {
        if newline is !None /* Option */ && !isinstance ( newline , str ) {
        panic!("TypeError ( "illegal newline type: %r" % ( type ( newline ) , ) )");
        if newline !in ( None /* Option */ , "" , "\n" , "\r" , "\r\n" ) {
        panic!("ValueError ( "illegal newline value: %r" % ( newline , ) )");
        pub fn _configure ( &self, encoding = None /* Option */ , errors = None /* Option */ , newline = None /* Option */ , {
        line_buffering = false , write_through = false ) ;
        self . _encoding = encoding;
        self . _errors = errors;
        self . _encoder = None /* Option */;
        self . _decoder = None /* Option */;
        self . _b2cratio = 0.0;
        self . _readuniversal = !newline;
        self . _readtranslate = newline is None /* Option */;
        self . _readnl = newline;
        self . _writetranslate = newline != "";
        self . _writenl = newline || os . linesep;
        self . _line_buffering = line_buffering;
        self . _write_through = write_through;
        if self . _seekable && self . writable ( ) {
        position = self . buffer . tell ( );
        if position != 0 {
        // try {
        self . _get_encoder ( ) . setstate ( 0 );
        // } catch  LookupError  {
        // pass
        pub fn __repr__ ( self )  {
        result = "<{}.{}" . format ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ );
        // try {
        name = self . name;
        // } catch  AttributeError  {
        // pass
        } else {
        result + = " name={0!r}" . format ( name );
        // try {
        mode = self . mode;
        // } catch  AttributeError  {
        // pass
        } else {
        result + = " mode={0!r}" . format ( mode );
        return  result + " encoding={0!r}>" . format ( self . encoding );
        @ property;
        pub fn encoding ( self )  {
        return  self . _encoding;
        @ property;
        pub fn errors ( self )  {
        return  self . _errors;
        @ property;
        pub fn line_buffering ( self )  {
        return  self . _line_buffering;
        @ property;
        pub fn write_through ( self )  {
        return  self . _write_through;
        @ property;
        pub fn buffer ( self )  {
        return  self . _buffer;
        pub fn reconfigure ( &self, * , {
        encoding = None /* Option */ , errors = None /* Option */ , newline = Ellipsis ,;
        line_buffering = None /* Option */ , write_through = None /* Option */ ) ;
        "Reconfigure the text stream with new parameters.

        This also flushes the stream.
        ";
        if ( self . _decoder is !None /* Option */ {
        and ( encoding == !None /* Option */ || errors == !None /* Option */;
        or newline == !Ellipsis ) ) ;
        panic!("UnsupportedOperation (");
        "It == !possible to set the encoding || newline of stream ";
        "after the first read" );
        if errors is None /* Option */ {
        if encoding is None /* Option */ {
        errors = self . _errors;
        } else {
        errors = "strict";
        } else if !isinstance ( errors , str ) {
        panic!("TypeError ( "invalid errors: %r" % errors )");
        if encoding is None /* Option */ {
        encoding = self . _encoding;
        } else {
        if !isinstance ( encoding , str ) {
        panic!("TypeError ( "invalid encoding: %r" % encoding )");
        if encoding == "locale" {
        encoding = self . _get_locale_encoding ( );
        if newline is Ellipsis {
        newline = self . _readnl;
        self . _check_newline ( newline );
        if line_buffering is None /* Option */ {
        line_buffering = self . line_buffering;
        if write_through is None /* Option */ {
        write_through = self . write_through;
        self . flush ( );
        self . _configure ( encoding , errors , newline ,;
        line_buffering , write_through );
        pub fn seekable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  self . _seekable;
        pub fn readable ( self )  {
        return  self . buffer . readable ( );
        pub fn writable ( self )  {
        return  self . buffer . writable ( );
        pub fn flush ( self )  {
        self . buffer . flush ( );
        self . _telling = self . _seekable;
        pub fn close ( self )  {
        if self . buffer is !None /* Option */ && !self . closed {
        // try {
        self . flush ( );
        // } finally {
        self . buffer . close ( );
        @ property;
        pub fn closed ( self )  {
        return  self . buffer . closed;
        @ property;
        pub fn name ( self )  {
        return  self . buffer . name;
        pub fn fileno ( self )  {
        return  self . buffer . fileno ( );
        pub fn isatty ( self )  {
        return  self . buffer . isatty ( );
        pub fn write ( &self, s )  {
        "Write data, where s == a str";
        if self . closed {
        panic!("ValueError ( "write to closed file" )");
        if !isinstance ( s , str ) {
        panic!("TypeError ( "can't write %s to text stream" %");
        s . __class__ . __name__ );
        length = len ( s );
        haslf = ( self . _writetranslate || self . _line_buffering ) && "\n" in s;
        if haslf && self . _writetranslate && self . _writenl != "\n" {
        s = s . replace ( "\n" , self . _writenl );
        encoder = self . _encoder || self . _get_encoder ( );
        b = encoder . encode ( s );
        self . buffer . write ( b );
        if self . _line_buffering && ( haslf || "\r" in s ) {
        self . flush ( );
        if self . _snapshot is !None /* Option */ {
        self . _set_decoded_chars ( "" );
        self . _snapshot = None /* Option */;
        if self . _decoder {
        self . _decoder . reset ( );
        return  length;
        pub fn _get_encoder ( self )  {
        make_encoder = codecs . getincrementalencoder ( self . _encoding );
        self . _encoder = make_encoder ( self . _errors );
        return  self . _encoder;
        pub fn _get_decoder ( self )  {
        make_decoder = codecs . getincrementaldecoder ( self . _encoding );
        decoder = make_decoder ( self . _errors );
        if self . _readuniversal {
        decoder = IncrementalNewlineDecoder ( decoder , self . _readtranslate );
        self . _decoder = decoder;
        return  decoder;
        pub fn _set_decoded_chars ( &self, chars )  {
        "Set the _decoded_chars buffer.";
        self . _decoded_chars = chars;
        self . _decoded_chars_used = 0;
        pub fn _get_decoded_chars ( &self, n = None /* Option */ )  {
        "Advance into the _decoded_chars buffer.";
        offset = self . _decoded_chars_used;
        if n is None /* Option */ {
        chars = self . _decoded_chars [ offset : ];
        } else {
        chars = self . _decoded_chars [ offset : offset + n ];
        self . _decoded_chars_used + = len ( chars );
        return  chars;
        pub fn _get_locale_encoding ( self )  {
        // try {
        import locale;
        // } catch  ImportError  {
        return  "utf-8";
        } else {
        return  locale . getencoding ( );
        pub fn _rewind_decoded_chars ( &self, n )  {
        "Rewind the _decoded_chars buffer.";
        if self . _decoded_chars_used < n {
        panic!("AssertionError ( "rewind decoded_chars out of bounds" )");
        self . _decoded_chars_used - = n;
        pub fn _read_chunk ( self )  {
        "
        Read && decode the next chunk of data from the BufferedReader.
        ";
        if self . _decoder is None /* Option */ {
        panic!("ValueError ( "no decoder" )");
        if self . _telling {
        dec_buffer , dec_flags = self . _decoder . getstate ( );
        if self . _has_read1 {
        input_chunk = self . buffer . read1 ( self . _CHUNK_SIZE );
        } else {
        input_chunk = self . buffer . read ( self . _CHUNK_SIZE );
        eof = !input_chunk;
        decoded_chars = self . _decoder . decode ( input_chunk , eof );
        self . _set_decoded_chars ( decoded_chars );
        if decoded_chars {
        self . _b2cratio = len ( input_chunk ) / len ( self . _decoded_chars );
        } else {
        self . _b2cratio = 0.0;
        if self . _telling {
        self . _snapshot = ( dec_flags , dec_buffer + input_chunk );
        return  !eof;
        pub fn _pack_cookie ( &self, position , dec_flags = 0 , {
        bytes_to_feed = 0 , need_eof = false , chars_to_skip = 0 ) ;
        return  ( position | ( dec_flags < < 64 ) | ( bytes_to_feed < < 128 ) |;
        ( chars_to_skip < < 192 ) | bool ( need_eof ) < < 256 );
        pub fn _unpack_cookie ( &self, bigint )  {
        rest , position = divmod ( bigint , 1 < < 64 );
        rest , dec_flags = divmod ( rest , 1 < < 64 );
        rest , bytes_to_feed = divmod ( rest , 1 < < 64 );
        need_eof , chars_to_skip = divmod ( rest , 1 < < 64 );
        return  position , dec_flags , bytes_to_feed , bool ( need_eof ) , chars_to_skip;
        pub fn tell ( self )  {
        if !self . _seekable {
        panic!("UnsupportedOperation ( "underlying stream is !seekable" )");
        if !self . _telling {
        panic!("OSError ( "telling position disabled by next() call" )");
        self . flush ( );
        position = self . buffer . tell ( );
        decoder = self . _decoder;
        if decoder is None /* Option */ || self . _snapshot is None /* Option */ {
        if self . _decoded_chars {
        panic!("AssertionError ( "pending decoded text" )");
        return  position;
        dec_flags , next_input = self . _snapshot;
        position - = len ( next_input );
        chars_to_skip = self . _decoded_chars_used;
        if chars_to_skip == 0 {
        return  self . _pack_cookie ( position , dec_flags );
        saved_state = decoder . getstate ( );
        // try {
        skip_bytes = int ( self . _b2cratio * chars_to_skip );
        skip_back = 1;
        assert skip_bytes <= len ( next_input );
        while skip_bytes > 0  {
        decoder . setstate ( ( b "" , dec_flags ) );
        n = len ( decoder . decode ( next_input [ : skip_bytes ] ) );
        if n <= chars_to_skip {
        b , d = decoder . getstate ( );
        if !b {
        dec_flags = d;
        chars_to_skip - = n;
        break;
        skip_bytes - = len ( b );
        skip_back = 1;
        } else {
        skip_bytes - = skip_back;
        skip_back = skip_back * 2;
        } else {
        skip_bytes = 0;
        decoder . setstate ( ( b "" , dec_flags ) );
        start_pos = position + skip_bytes;
        start_flags = dec_flags;
        if chars_to_skip == 0 {
        return  self . _pack_cookie ( start_pos , start_flags );
        bytes_fed = 0;
        need_eof = false;
        chars_decoded = 0;
        for i in range ( skip_bytes , len ( next_input ) ) .iter() {
        bytes_fed + = 1;
        chars_decoded + = len ( decoder . decode ( next_input [ i : i + 1 ] ) );
        dec_buffer , dec_flags = decoder . getstate ( );
        if !dec_buffer && chars_decoded <= chars_to_skip {
        start_pos + = bytes_fed;
        chars_to_skip - = chars_decoded;
        start_flags , bytes_fed , chars_decoded = dec_flags , 0 , 0;
        if chars_decoded >= chars_to_skip {
        break;
        } else {
        chars_decoded + = len ( decoder . decode ( b "" , final = true ) );
        need_eof = true;
        if chars_decoded < chars_to_skip {
        panic!("OSError ( "can't reconstruct logical file position" )");
        return  self . _pack_cookie (;
        start_pos , start_flags , bytes_fed , need_eof , chars_to_skip );
        // } finally {
        decoder . setstate ( saved_state );
        pub fn truncate ( &self, pos = None /* Option */ )  {
        self . flush ( );
        if pos is None /* Option */ {
        pos = self . tell ( );
        return  self . buffer . truncate ( pos );
        pub fn detach ( self )  {
        if self . buffer is None /* Option */ {
        panic!("ValueError ( "buffer is already detached" )");
        self . flush ( );
        buffer = self . _buffer;
        self . _buffer = None /* Option */;
        return  buffer;
        pub fn seek ( &self, cookie , whence = 0 )  {
        pub fn _reset_encoder ( position )  {
        "Reset the encoder (merely useful for proper BOM handling)";
        // try {
        encoder = self . _encoder || self . _get_encoder ( );
        // } catch  LookupError  {
        // pass
        } else {
        if position != 0 {
        encoder . setstate ( 0 );
        } else {
        encoder . reset ( );
        if self . closed {
        panic!("ValueError ( "tell on closed file" )");
        if !self . _seekable {
        panic!("UnsupportedOperation ( "underlying stream is !seekable" )");
        if whence == SEEK_CUR {
        if cookie != 0 {
        panic!("UnsupportedOperation ( "can't do nonzero cur-relative seeks" )");
        whence = 0;
        cookie = self . tell ( );
        } else if whence == SEEK_END {
        if cookie != 0 {
        panic!("UnsupportedOperation ( "can't do nonzero end-relative seeks" )");
        self . flush ( );
        position = self . buffer . seek ( 0 , whence );
        self . _set_decoded_chars ( "" );
        self . _snapshot = None /* Option */;
        if self . _decoder {
        self . _decoder . reset ( );
        _reset_encoder ( position );
        return  position;
        if whence != 0 {
        panic!("ValueError ( "unsupported whence (%r)" % ( whence , ) )");
        if cookie < 0 {
        panic!("ValueError ( "negative seek position %r" % ( cookie , ) )");
        self . flush ( );
        start_pos , dec_flags , bytes_to_feed , need_eof , chars_to_skip = \;
        self . _unpack_cookie ( cookie );
        self . buffer . seek ( start_pos );
        self . _set_decoded_chars ( "" );
        self . _snapshot = None /* Option */;
        if cookie == 0 && self . _decoder {
        self . _decoder . reset ( );
        } else if self . _decoder || dec_flags || chars_to_skip {
        self . _decoder = self . _decoder || self . _get_decoder ( );
        self . _decoder . setstate ( ( b "" , dec_flags ) );
        self . _snapshot = ( dec_flags , b "" );
        if chars_to_skip {
        input_chunk = self . buffer . read ( bytes_to_feed );
        self . _set_decoded_chars (;
        self . _decoder . decode ( input_chunk , need_eof ) );
        self . _snapshot = ( dec_flags , input_chunk );
        if len ( self . _decoded_chars ) < chars_to_skip {
        panic!("OSError ( "can't restore logical file position" )");
        self . _decoded_chars_used = chars_to_skip;
        _reset_encoder ( cookie );
        return  cookie;
        pub fn read ( &self, size = None /* Option */ )  {
        self . _checkReadable ( );
        if size is None /* Option */ {
        size = -1;
        } else {
        // try {
        size_index = size . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{size!r} is !an integer" )");
        } else {
        size = size_index ( );
        decoder = self . _decoder || self . _get_decoder ( );
        if size < 0 {
        result = ( self . _get_decoded_chars ( ) +;
        decoder . decode ( self . buffer . read ( ) , final = true ) );
        if self . _snapshot is !None /* Option */ {
        self . _set_decoded_chars ( "" );
        self . _snapshot = None /* Option */;
        return  result;
        } else {
        eof = false;
        result = self . _get_decoded_chars ( size );
        while len ( result ) < size && !eof  {
        eof = !self . _read_chunk ( );
        result + = self . _get_decoded_chars ( size - len ( result ) );
        return  result;
        pub fn __next__ ( self )  {
        self . _telling = false;
        line = self . readline ( );
        if !line {
        self . _snapshot = None /* Option */;
        self . _telling = self . _seekable;
        panic!("StopIteration");
        return  line;
        pub fn readline ( &self, size = None /* Option */ )  {
        if self . closed {
        panic!("ValueError ( "read from closed file" )");
        if size is None /* Option */ {
        size = -1;
        } else {
        // try {
        size_index = size . __index__;
        // } catch  AttributeError  {
        panic!("TypeError ( f "{size!r} is !an integer" )");
        } else {
        size = size_index ( );
        line = self . _get_decoded_chars ( );
        start = 0;
        if !self . _decoder {
        self . _get_decoder ( );
        pos = endpos = None /* Option */;
        while true  {
        if self . _readtranslate {
        pos = line . find ( "\n" , start );
        if pos >= 0 {
        endpos = pos + 1;
        break;
        } else {
        start = len ( line );
        } else if self . _readuniversal {
        nlpos = line . find ( "\n" , start );
        crpos = line . find ( "\r" , start );
        if crpos == -1 {
        if nlpos == -1 {
        start = len ( line );
        } else {
        endpos = nlpos + 1;
        break;
        } else if nlpos == -1 {
        endpos = crpos + 1;
        break;
        } else if nlpos < crpos {
        endpos = nlpos + 1;
        break;
        } else if nlpos == crpos + 1 {
        endpos = crpos + 2;
        break;
        } else {
        endpos = crpos + 1;
        break;
        } else {
        pos = line . find ( self . _readnl );
        if pos >= 0 {
        endpos = pos + len ( self . _readnl );
        break;
        if size >= 0 && len ( line ) >= size {
        endpos = size;
        break;
        while self . _read_chunk ( )  {
        if self . _decoded_chars {
        break;
        if self . _decoded_chars {
        line + = self . _get_decoded_chars ( );
        } else {
        self . _set_decoded_chars ( "" );
        self . _snapshot = None /* Option */;
        return  line;
        if size >= 0 && endpos > size {
        endpos = size;
        self . _rewind_decoded_chars ( len ( line ) - endpos );
        return  line [ : endpos ];
        @ property;
        pub fn newlines ( self )  {
        return  self . _decoder . newlines if self . _decoder else None /* Option */;
        class StringIO ( TextIOWrapper ) ;
        "Text I/O implementation using an in-memory buffer.

    The initial_value argument sets the value of object.  The newline
    argument == like the one of TextIOWrapper's constructor.
    ";
        pub fn __init__ ( &self, initial_value = "" , newline = "\n" )  {
        super ( StringIO , self ) . __init__ ( BytesIO ( ) ,;
        encoding = "utf-8" ,;
        errors = "surrogatepass" ,;
        newline = newline );
        if newline is None /* Option */ {
        self . _writetranslate = false;
        if initial_value is !None /* Option */ {
        if !isinstance ( initial_value , str ) {
        panic!("TypeError ( "initial_value must be str || None /* Option */, !{0}"");
        . format ( type ( initial_value ) . __name__ ) );
        self . write ( initial_value );
        self . seek ( 0 );
        pub fn getvalue ( self )  {
        self . flush ( );
        decoder = self . _decoder || self . _get_decoder ( );
        old_state = decoder . getstate ( );
        decoder . reset ( );
        // try {
        return  decoder . decode ( self . buffer . getvalue ( ) , final = true );
        // } finally {
        decoder . setstate ( old_state );
        pub fn __repr__ ( self )  {
        return  object . __repr__ ( self );
        @ property;
        pub fn errors ( self )  {
        return;
        @ property;
        pub fn encoding ( self )  {
        return;
        pub fn detach ( self )  {
        self . _unsupported ( "detach" );
}

