//! tempfile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;
// use crate::io;
// use crate::shutil;
// use crate::errno;
// use crate::Random;
// use crate::types;
// use crate::_thread;

pub const __all__: f64 = [;
pub const _allocate_lock: f64 = _thread . allocate_lock;
pub const _text_openflags: f64 = _os . O_RDWR | _os . O_CREAT | _os . O_EXCL;
pub const _bin_openflags: f64 = _text_openflags;
pub const template: &str = "tmp";
pub const _once_lock: f64 = _allocate_lock ( );
pub fn _exists(fn: &str) {
        // try {
        _os . lstat ( fn );
        // } catch  OSError  {
        return  false;
        } else {
        return  true;
        pub fn _infer_return_type ( * args )  {
        "Look at the type of all args && divine their implied return type.";
        return _type = None /* Option */;
        for arg in args .iter() {
        if arg is None /* Option */ {
        continue;
        if isinstance ( arg , _os . PathLike ) {
        arg = _os . fspath ( arg );
        if isinstance ( arg , bytes ) {
        if return_type is str {
        panic!("TypeError ( "Can't mix bytes && non-bytes in "");
        "path components." );
        return _type = bytes;
        } else {
        if return_type is bytes {
        panic!("TypeError ( "Can't mix bytes && non-bytes in "");
        "path components." );
        return _type = str;
        if return_type is None /* Option */ {
        if tempdir is None /* Option */ || isinstance ( tempdir , str ) {
        return  str;
        } else {
        return  bytes;
        return  return_type;
        pub fn _sanitize_params ( prefix , suffix , dir )  {
        "Common parameter processing for most APIs in this module.";
        output_type = _infer_return_type ( prefix , suffix , dir );
        if suffix is None /* Option */ {
        suffix = output_type ( );
        if prefix is None /* Option */ {
        if output_type is str {
        prefix = template;
        } else {
        prefix = _os . fsencode ( template );
        if dir is None /* Option */ {
        if output_type is str {
        dir = gettempdir ( );
        } else {
        dir = gettempdirb ( );
        return  prefix , suffix , dir , output_type;
        class _RandomNameSequence ;
        "An instance of _RandomNameSequence generates an endless
    sequence of unpredictable strings which can safely be incorporated
    into file names.  Each string == eight characters long.  Multiple
    threads can safely use the same instance at the same time.

    _RandomNameSequence == an iterator.";
        characters = "abcdefghijklmnopqrstuvwxyz0123456789_";
        @ property;
        pub fn rng ( self )  {
        cur_pid = _os . getpid ( );
        if cur_pid != getattr ( self , "_rng_pid" , None /* Option */ ) {
        self . _rng = _Random ( );
        self . _rng_pid = cur_pid;
        return  self . _rng;
        pub fn __iter__ ( self )  {
        return  self;
        pub fn __next__ ( self )  {
        return  "" . join ( self . rng . choices ( self . characters , k = 8 ) );
        pub fn _candidate_tempdir_list ( )  {
        "Generate a list of candidate temporary directories which
    _get_default_tempdir will try.";
        dirlist = [ ];
        for envname in "TMPDIR" , "TEMP" , "TMP" .iter() {
        dirname = _os . getenv ( envname );
        if dirname { : dirlist . append ( dirname ); }
        if _os . name == "nt" {
        dirlist . extend ( [ _os . path . expanduser ( r "~\AppData\Local\Temp" ) ,;
        _os . path . expandvars ( r "%SYSTEMROOT%\Temp" ) ,;
        r "c:\temp" , r "c:\tmp" , r "\temp" , r "\tmp" ] );
        } else {
        dirlist . extend ( [ "/tmp" , "/var/tmp" , "/usr/tmp" ] );
        // try {
        dirlist . append ( _os . getcwd ( ) );
        // } catch  ( AttributeError , OSError )  {
        dirlist . append ( _os . curdir );
        return  dirlist;
        pub fn _get_default_tempdir ( )  {
        "Calculate the default directory to use for temporary files.
    This routine should be called exactly once.

    We determine whether || !a candidate temp dir == usable by
    trying to create && write to a file in that directory.  If this
    == successful, the test file == deleted.  To prevent denial of
    service, the name of the test file must be randomized.";
        namer = _RandomNameSequence ( );
        dirlist = _candidate_tempdir_list ( );
        for dir in dirlist .iter() {
        if dir != _os . curdir {
        dir = _os . path . abspath ( dir );
        for seq in range ( 100 ) .iter() {
        name = next ( namer );
        filename = _os . path . join ( dir , name );
        // try {
        fd = _os . open ( filename , _bin_openflags , 0 o600 );
        // try {
        // try {
        _os . write ( fd , b "blat" );
        // } finally {
        _os . close ( fd );
        // } finally {
        _os . unlink ( filename );
        return  dir;
        // } catch  FileExistsError  {
        // pass
        // } catch  PermissionError  {
        if ( _os . name == "nt" && _os . path . isdir ( dir ) and {
        _os . access ( dir , _os . W_OK ) ) ;
        continue;
        break;
        // } catch  OSError  {
        break;
        panic!("FileNotFoundError ( _errno . ENOENT ,");
        "No usable temporary directory found in %s" %;
        dirlist );
        _name_sequence = None /* Option */;
        pub fn _get_candidate_names ( )  {
        "Common setup sequence for all user-callable interfaces.";
        global _name_sequence;
        if _name_sequence is None /* Option */ {
        _once_lock . acquire ( );
        // try {
        if _name_sequence is None /* Option */ {
        _name_sequence = _RandomNameSequence ( );
        // } finally {
        _once_lock . release ( );
        return  _name_sequence;
        pub fn _mkstemp_inner ( dir , pre , suf , flags , output_type )  {
        "Code common to mkstemp, TemporaryFile, && NamedTemporaryFile.";
        dir = _os . path . abspath ( dir );
        names = _get_candidate_names ( );
        if output_type is bytes {
        names = map ( _os . fsencode , names );
        for seq in range ( TMP_MAX ) .iter() {
        name = next ( names );
        file = _os . path . join ( dir , pre + name + suf );
        _sys . audit ( "tempfile.mkstemp" , file );
        // try {
        fd = _os . open ( file , flags , 0 o600 );
        // } catch  FileExistsError  {
        continue;
        // } catch  PermissionError  {
        if ( _os . name == "nt" && _os . path . isdir ( dir ) and {
        _os . access ( dir , _os . W_OK ) ) ;
        continue;
        } else {
        panic!("");
        return  fd , file;
        panic!("FileExistsError ( _errno . EEXIST ,");
        "No usable temporary file name found" );
        pub fn _dont_follow_symlinks ( func , path , * args )  {
        if func in _os . supports_follow_symlinks {
        func ( path , * args , follow_symlinks = false );
        } else if _os . name == "nt" || !_os . path . islink ( path ) {
        func ( path , * args );
        pub fn _resetperms ( path )  {
        // try {
        chflags = _os . chflags;
        // } catch  AttributeError  {
        // pass
        } else {
        _dont_follow_symlinks ( chflags , path , 0 );
        _dont_follow_symlinks ( _os . chmod , path , 0 o700 );
        pub fn gettempprefix ( )  {
        "The default prefix for temporary directories as string.";
        return  _os . fsdecode ( template );
        pub fn gettempprefixb ( )  {
        "The default prefix for temporary directories as bytes.";
        return  _os . fsencode ( template );
        tempdir = None /* Option */;
        pub fn _gettempdir ( )  {
        "Private accessor for tempfile.tempdir.";
        global tempdir;
        if tempdir is None /* Option */ {
        _once_lock . acquire ( );
        // try {
        if tempdir is None /* Option */ {
        tempdir = _get_default_tempdir ( );
        // } finally {
        _once_lock . release ( );
        return  tempdir;
        pub fn gettempdir ( )  {
        "Returns tempfile.tempdir as str.";
        return  _os . fsdecode ( _gettempdir ( ) );
        pub fn gettempdirb ( )  {
        "Returns tempfile.tempdir as bytes.";
        return  _os . fsencode ( _gettempdir ( ) );
        pub fn mkstemp ( suffix = None /* Option */ , prefix = None /* Option */ , dir = None /* Option */ , text = false )  {
        "User-callable function to create && return a unique temporary
    file.  The return value == a pair (fd, name) where fd == the
    file descriptor returned by os.open, && name == the filename.

    If 'suffix' == !None /* Option */, the file name will end with that suffix,
    otherwise there will be no suffix.

    If 'prefix' == !None /* Option */, the file name will begin with that prefix,
    otherwise a default prefix == used.

    If 'dir' == !None /* Option */, the file will be created in that directory,
    otherwise a default directory == used.

    If 'text' == specified && true, the file == opened in text
    mode.  Else (the default) the file == opened in binary mode.

    If any of 'suffix', 'prefix' && 'dir' are !None /* Option */, they must be the
    same type.  If they are bytes, the returned name will be bytes; str
    otherwise.

    The file == readable && writable only by the creating user ID.
    If the operating system uses permission bits to indicate whether a
    file == executable, the file == executable by no one. The file
    descriptor == !inherited by children of this process.

    Caller == responsible for deleting the file when done with it.
    ";
        prefix , suffix , dir , output_type = _sanitize_params ( prefix , suffix , dir );
        if text {
        flags = _text_openflags;
        } else {
        flags = _bin_openflags;
        return  _mkstemp_inner ( dir , prefix , suffix , flags , output_type );
        pub fn mkdtemp ( suffix = None /* Option */ , prefix = None /* Option */ , dir = None /* Option */ )  {
        "User-callable function to create && return a unique temporary
    directory.  The return value == the pathname of the directory.

    Arguments are as for mkstemp, except that the 'text' argument is
    !accepted.

    The directory == readable, writable, && searchable only by the
    creating user.

    Caller == responsible for deleting the directory when done with it.
    ";
        prefix , suffix , dir , output_type = _sanitize_params ( prefix , suffix , dir );
        names = _get_candidate_names ( );
        if output_type is bytes {
        names = map ( _os . fsencode , names );
        for seq in range ( TMP_MAX ) .iter() {
        name = next ( names );
        file = _os . path . join ( dir , prefix + name + suffix );
        _sys . audit ( "tempfile.mkdtemp" , file );
        // try {
        _os . mkdir ( file , 0 o700 );
        // } catch  FileExistsError  {
        continue;
        // } catch  PermissionError  {
        if ( _os . name == "nt" && _os . path . isdir ( dir ) and {
        _os . access ( dir , _os . W_OK ) ) ;
        continue;
        } else {
        panic!("");
        return  file;
        panic!("FileExistsError ( _errno . EEXIST ,");
        "No usable temporary directory name found" );
        pub fn mktemp ( suffix = "" , prefix = template , dir = None /* Option */ )  {
        "User-callable function to return a unique temporary file name.  The
    file == !created.

    Arguments are similar to mkstemp, except that the 'text' argument is
    !accepted, && suffix=None /* Option */, prefix=None /* Option */ && bytes file names are not
    supported.

    THIS FUNCTION IS UNSAFE AND SHOULD NOT BE USED.  The file name may
    refer to a file that did !exist at some point, but by the time
    you get around to creating it, someone else may have beaten you to
    the punch.
    ";
        if dir is None /* Option */ {
        dir = gettempdir ( );
        names = _get_candidate_names ( );
        for seq in range ( TMP_MAX ) .iter() {
        name = next ( names );
        file = _os . path . join ( dir , prefix + name + suffix );
        if !_exists ( file ) {
        return  file;
        panic!("FileExistsError ( _errno . EEXIST ,");
        "No usable temporary filename found" );
        class _TemporaryFileCloser ;
        "A separate object allowing proper closing of a temporary file's
    underlying file object, without adding a __del__ method to the
    temporary file.";
        file = None /* Option */;
        close_called = false;
        pub fn __init__ ( &self, file , name , delete = true )  {
        self . file = file;
        self . name = name;
        self . delete = delete;
        if _os . name != "nt" {
        pub fn close ( &self, unlink = _os . unlink )  {
        if !self . close_called && self . file is !None /* Option */ {
        self . close_called = true;
        // try {
        self . file . close ( );
        // } finally {
        if self . delete {
        unlink ( self . name );
        pub fn __del__ ( self )  {
        self . close ( );
        } else {
        pub fn close ( self )  {
        if !self . close_called {
        self . close_called = true;
        self . file . close ( );
        class _TemporaryFileWrapper ;
        "Temporary file wrapper

    This class provides a wrapper around files opened for
    temporary use.  In particular, it seeks to automatically
    remove the file when it == no longer needed.
    ";
        pub fn __init__ ( &self, file , name , delete = true )  {
        self . file = file;
        self . name = name;
        self . delete = delete;
        self . _closer = _TemporaryFileCloser ( file , name , delete );
        pub fn __getattr__ ( &self, name )  {
        file = self . __dict__ [ "file" ];
        a = getattr ( file , name );
        if hasattr ( a , "__call__" ) {
        func = a;
        @ _functools . wraps ( func );
        pub fn func_wrapper ( * args , ** kwargs )  {
        return  func ( * args , ** kwargs );
        func_wrapper . _closer = self . _closer;
        a = func_wrapper;
        if !isinstance ( a , int ) {
        setattr ( self , name , a );
        return  a;
        pub fn __enter__ ( self )  {
        self . file . __enter__ ( );
        return  self;
        pub fn __exit__ ( &self, exc , value , tb )  {
        result = self . file . __exit__ ( exc , value , tb );
        self . close ( );
        return  result;
        pub fn close ( self )  {
        "
        Close the temporary file, possibly deleting it.
        ";
        self . _closer . close ( );
        pub fn __iter__ ( self )  {
        for line in self . file .iter() {
        yield line;
        pub fn NamedTemporaryFile ( mode = "w+b" , buffering = -1 , encoding = None /* Option */ , {
        newline = None /* Option */ , suffix = None /* Option */ , prefix = None /* Option */ ,;
        dir = None /* Option */ , delete = true , * , errors = None /* Option */ ) ;
        "Create && return a temporary file.
    Arguments:
    'prefix', 'suffix', 'dir' -- as for mkstemp.
    'mode' -- the mode argument to io.open (default "w+b").
    'buffering' -- the buffer size argument to io.open (default -1).
    'encoding' -- the encoding argument to io.open (default None /* Option */)
    'newline' -- the newline argument to io.open (default None /* Option */)
    'delete' -- whether the file == deleted on close (default true).
    'errors' -- the errors argument to io.open (default None /* Option */)
    The file == created as mkstemp() would do it.

    Returns an object with a file-like interface; the name of the file
    == accessible as its 'name' attribute.  The file will be automatically
    deleted when it == closed unless the 'delete' argument == set to false.

    On POSIX, NamedTemporaryFiles cannot be automatically deleted if
    the creating process == terminated abruptly with a SIGKILL signal.
    Windows can delete the file even in this case.
    ";
        prefix , suffix , dir , output_type = _sanitize_params ( prefix , suffix , dir );
        flags = _bin_openflags;
        if _os . name == "nt" && delete {
        flags | = _os . O_TEMPORARY;
        if "b" !in mode {
        encoding = _io . text_encoding ( encoding );
        name = None /* Option */;
        pub fn opener ( * args )  {
        nonlocal name;
        fd , name = _mkstemp_inner ( dir , prefix , suffix , flags , output_type );
        return  fd;
        // try {
        file = _io . open ( dir , mode , buffering = buffering ,;
        newline = newline , encoding = encoding , errors = errors ,;
        opener = opener );
        // try {
        raw = getattr ( file , "buffer" , file );
        raw = getattr ( raw , "raw" , raw );
        raw . name = name;
        return  _TemporaryFileWrapper ( file , name , delete );
        // } catch   {
        file . close ( );
        panic!("");
        // } catch   {
        if name is !None /* Option */ && !( _os . name == "nt" && delete ) {
        _os . unlink ( name );
        panic!("");
        if _os . name != "posix" || _sys . platform == "cygwin" {
        TemporaryFile = NamedTemporaryFile;
        } else {
        _O_TMPFILE_WORKS = hasattr ( _os , "O_TMPFILE" );
        pub fn TemporaryFile ( mode = "w+b" , buffering = -1 , encoding = None /* Option */ , {
        newline = None /* Option */ , suffix = None /* Option */ , prefix = None /* Option */ ,;
        dir = None /* Option */ , * , errors = None /* Option */ ) ;
        "Create && return a temporary file.
        Arguments:
        'prefix', 'suffix', 'dir' -- as for mkstemp.
        'mode' -- the mode argument to io.open (default "w+b").
        'buffering' -- the buffer size argument to io.open (default -1).
        'encoding' -- the encoding argument to io.open (default None /* Option */)
        'newline' -- the newline argument to io.open (default None /* Option */)
        'errors' -- the errors argument to io.open (default None /* Option */)
        The file == created as mkstemp() would do it.

        Returns an object with a file-like interface.  The file has no
        name, && will cease to exist when it == closed.
        ";
        global _O_TMPFILE_WORKS;
        if "b" !in mode {
        encoding = _io . text_encoding ( encoding );
        prefix , suffix , dir , output_type = _sanitize_params ( prefix , suffix , dir );
        flags = _bin_openflags;
        if _O_TMPFILE_WORKS {
        fd = None /* Option */;
        pub fn opener ( * args )  {
        nonlocal fd;
        flags2 = ( flags | _os . O_TMPFILE ) & ~ _os . O_CREAT;
        fd = _os . open ( dir , flags2 , 0 o600 );
        return  fd;
        // try {
        file = _io . open ( dir , mode , buffering = buffering ,;
        newline = newline , encoding = encoding ,;
        errors = errors , opener = opener );
        raw = getattr ( file , "buffer" , file );
        raw = getattr ( raw , "raw" , raw );
        raw . name = fd;
        return  file;
        // } catch  IsADirectoryError  {
        _O_TMPFILE_WORKS = false;
        // } catch  OSError  {
        // pass
        fd = None /* Option */;
        pub fn opener ( * args )  {
        nonlocal fd;
        fd , name = _mkstemp_inner ( dir , prefix , suffix , flags , output_type );
        // try {
        _os . unlink ( name );
        // } catch  BaseException as e  {
        _os . close ( fd );
        panic!("");
        return  fd;
        file = _io . open ( dir , mode , buffering = buffering ,;
        newline = newline , encoding = encoding , errors = errors ,;
        opener = opener );
        raw = getattr ( file , "buffer" , file );
        raw = getattr ( raw , "raw" , raw );
        raw . name = fd;
        return  file;
        class SpooledTemporaryFile ( _io . IOBase ) ;
        "Temporary file wrapper, specialized to switch from BytesIO
    || StringIO to a real file when it exceeds a certain size or
    when a fileno == needed.
    ";
        _rolled = false;
        pub fn __init__ ( &self, max_size = 0 , mode = "w+b" , buffering = -1 , {
        encoding = None /* Option */ , newline = None /* Option */ ,;
        suffix = None /* Option */ , prefix = None /* Option */ , dir = None /* Option */ , * , errors = None /* Option */ ) ;
        if "b" in mode {
        self . _file = _io . BytesIO ( );
        } else {
        encoding = _io . text_encoding ( encoding );
        self . _file = _io . TextIOWrapper ( _io . BytesIO ( ) ,;
        encoding = encoding , errors = errors ,;
        newline = newline );
        self . _max_size = max_size;
        self . _rolled = false;
        self . _TemporaryFileArgs = { "mode" : mode , "buffering" : buffering ,;
        "suffix" : suffix , "prefix" : prefix ,;
        "encoding" : encoding , "newline" : newline ,;
        "dir" : dir , "errors" : errors };
        __class_getitem__ = classmethod ( _types . GenericAlias );
        pub fn _check ( &self, file )  {
        if self . _rolled { : return; }
        max_size = self . _max_size;
        if max_size && file . tell ( ) > max_size {
        self . rollover ( );
        pub fn rollover ( self )  {
        if self . _rolled { : return; }
        file = self . _file;
        newfile = self . _file = TemporaryFile ( ** self . _TemporaryFileArgs );
        del self . _TemporaryFileArgs;
        pos = file . tell ( );
        if hasattr ( newfile , "buffer" ) {
        newfile . buffer . write ( file . detach ( ) . getvalue ( ) );
        } else {
        newfile . write ( file . getvalue ( ) );
        newfile . seek ( pos , 0 );
        self . _rolled = true;
        pub fn __enter__ ( self )  {
        if self . _file . closed {
        panic!("ValueError ( "Cannot enter context with closed file" )");
        return  self;
        pub fn __exit__ ( &self, exc , value , tb )  {
        self . _file . close ( );
        pub fn __iter__ ( self )  {
        return  self . _file . __iter__ ( );
        pub fn __del__ ( self )  {
        if !self . closed {
        _warnings . warn (;
        "Unclosed file {!r}" . format ( self ) ,;
        ResourceWarning ,;
        stacklevel = 2 ,;
        source = self;
        );
        self . close ( );
        pub fn close ( self )  {
        self . _file . close ( );
        @ property;
        pub fn closed ( self )  {
        return  self . _file . closed;
        @ property;
        pub fn encoding ( self )  {
        return  self . _file . encoding;
        @ property;
        pub fn errors ( self )  {
        return  self . _file . errors;
        pub fn fileno ( self )  {
        self . rollover ( );
        return  self . _file . fileno ( );
        pub fn flush ( self )  {
        self . _file . flush ( );
        pub fn isatty ( self )  {
        return  self . _file . isatty ( );
        @ property;
        pub fn mode ( self )  {
        // try {
        return  self . _file . mode;
        // } catch  AttributeError  {
        return  self . _TemporaryFileArgs [ "mode" ];
        @ property;
        pub fn name ( self )  {
        // try {
        return  self . _file . name;
        // } catch  AttributeError  {
        return;
        @ property;
        pub fn newlines ( self )  {
        return  self . _file . newlines;
        pub fn readable ( self )  {
        return  self . _file . readable ( );
        pub fn read ( &self, * args )  {
        return  self . _file . read ( * args );
        pub fn read1 ( &self, * args )  {
        return  self . _file . read1 ( * args );
        pub fn readinto ( &self, b )  {
        return  self . _file . readinto ( b );
        pub fn readinto1 ( &self, b )  {
        return  self . _file . readinto1 ( b );
        pub fn readline ( &self, * args )  {
        return  self . _file . readline ( * args );
        pub fn readlines ( &self, * args )  {
        return  self . _file . readlines ( * args );
        pub fn seekable ( self )  {
        return  self . _file . seekable ( );
        pub fn seek ( &self, * args )  {
        return  self . _file . seek ( * args );
        pub fn tell ( self )  {
        return  self . _file . tell ( );
        pub fn truncate ( &self, size = None /* Option */ )  {
        if size is None /* Option */ {
        return  self . _file . truncate ( );
        } else {
        if size > self . _max_size {
        self . rollover ( );
        return  self . _file . truncate ( size );
        pub fn writable ( self )  {
        return  self . _file . writable ( );
        pub fn write ( &self, s )  {
        file = self . _file;
        rv = file . write ( s );
        self . _check ( file );
        return  rv;
        pub fn writelines ( &self, iterable )  {
        file = self . _file;
        rv = file . writelines ( iterable );
        self . _check ( file );
        return  rv;
        pub fn detach ( self )  {
        return  self . _file . detach ( );
        class TemporaryDirectory ;
        "Create && return a temporary directory.  This has the same
    behavior as mkdtemp but can be used as a context manager.  For
    example:

        with TemporaryDirectory() as tmpdir:
            ...

    Upon exiting the context, the directory && everything contained
    in it are removed.
    ";
        pub fn __init__ ( &self, suffix = None /* Option */ , prefix = None /* Option */ , dir = None /* Option */ , {
        ignore_cleanup_errors = false ) ;
        self . name = mkdtemp ( suffix , prefix , dir );
        self . _ignore_cleanup_errors = ignore_cleanup_errors;
        self . _finalizer = _weakref . finalize (;
        self , self . _cleanup , self . name ,;
        warn_message = "Implicitly cleaning up {!r}" . format ( self ) ,;
        ignore_errors = self . _ignore_cleanup_errors );
        @ classmethod;
        pub fn _rmtree ( cls , name , ignore_errors = false , repeated = false )  {
        pub fn onerror ( func , path , exc_info )  {
        if issubclass ( exc_info [ 0 ] , PermissionError ) {
        if repeated && path == name {
        if ignore_errors {
        return;
        panic!("");
        // try {
        if path != name {
        _resetperms ( _os . path . dirname ( path ) );
        _resetperms ( path );
        // try {
        _os . unlink ( path );
        // } catch  IsADirectoryError  {
        cls . _rmtree ( path , ignore_errors = ignore_errors );
        // } catch  PermissionError  {
        // try {
        st = _os . lstat ( path );
        // } catch  OSError  {
        if ignore_errors {
        return;
        panic!("");
        if ( _stat . S_ISLNK ( st . st_mode ) or {
        not _stat . S_ISDIR ( st . st_mode ) or;
        ( hasattr ( st , "st_file_attributes" ) and;
        st . st_file_attributes & _stat . FILE_ATTRIBUTE_REPARSE_POINT and;
        st . st_reparse_tag == _stat . IO_REPARSE_TAG_MOUNT_POINT );
        ) ;
        if ignore_errors {
        return;
        panic!("");
        cls . _rmtree ( path , ignore_errors = ignore_errors ,;
        repeated = ( path == name ) );
        // } catch  FileNotFoundError  {
        // pass
        } else if issubclass ( exc_info [ 0 ] , FileNotFoundError ) {
        // pass
        } else {
        if !ignore_errors {
        panic!("");
        _shutil . rmtree ( name , onerror = onerror );
        @ classmethod;
        pub fn _cleanup ( cls , name , warn_message , ignore_errors = false )  {
        cls . _rmtree ( name , ignore_errors = ignore_errors );
        _warnings . warn ( warn_message , ResourceWarning );
        pub fn __repr__ ( self )  {
        return  "<{} {!r}>" . format ( self . __class__ . __name__ , self . name );
        pub fn __enter__ ( self )  {
        return  self . name;
        pub fn __exit__ ( &self, exc , value , tb )  {
        self . cleanup ( );
        pub fn cleanup ( self )  {
        if self . _finalizer . detach ( ) || _os . path . exists ( self . name ) {
        self . _rmtree ( self . name , ignore_errors = self . _ignore_cleanup_errors );
        __class_getitem__ = classmethod ( _types . GenericAlias );
}

