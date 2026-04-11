//! os.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc;
// use crate::stat;
// use crate::_collections_abc::{_check_methods};
// use crate::posix::{};
// use crate::posixpath;
// use crate::nt::{};
// use crate::ntpath;
// use std::fs::{curdir, pardir, sep, pathsep, defpath, extsep, altsep};
// use crate::warnings;
// use crate::subprocess;
// use crate::io;

pub const GenericAlias: f64 = type ( list [ int ] );
pub const _names: f64 = sys . builtin_module_names;
pub const __all__: &str = ["altsep" ,"curdir" ,"pardir" ,"sep" ,"pathsep" ,"linesep" ,;
pub fn _exists(name: &str) {
        return  name in globals ( );
        pub fn _get_exports_list ( module )  {
        // try {
        return  list ( module . __all__ );
        // } catch  AttributeError  {
        return  [ n for n in dir ( module ) if n [ 0 ] != "_" ];
        if "posix" in _names {
        name = "posix";
        linesep = "\n";
        from posix import *;
        // try {
        from posix import _exit;
        __all__ . append ( "_exit" );
        // } catch  ImportError  {
        // pass
        import posixpath as path;
        // try {
        from posix import _have_functions;
        // } catch  ImportError  {
        // pass
        import posix;
        __all__ . extend ( _get_exports_list ( posix ) );
        del posix;
        } else if "nt" in _names {
        name = "nt";
        linesep = "\r\n";
        from nt import *;
        // try {
        from nt import _exit;
        __all__ . append ( "_exit" );
        // } catch  ImportError  {
        // pass
        import ntpath as path;
        import nt;
        __all__ . extend ( _get_exports_list ( nt ) );
        del nt;
        // try {
        from nt import _have_functions;
        // } catch  ImportError  {
        // pass
        } else {
        panic!("ImportError ( "no os specific module found" )");
        sys . modules [ "os.path" ] = path;
        from os . path import ( curdir , pardir , sep , pathsep , defpath , extsep , altsep ,;
        devnull );
        del _names;
        if _exists ( "_have_functions" ) {
        _globals = globals ( );
        pub fn _add ( str , fn )  {
        if ( fn in _globals ) && ( str in _have_functions ) {
        _set . add ( _globals [ fn ] );
        _set = set ( );
        _add ( "HAVE_FACCESSAT" , "access" );
        _add ( "HAVE_FCHMODAT" , "chmod" );
        _add ( "HAVE_FCHOWNAT" , "chown" );
        _add ( "HAVE_FSTATAT" , "stat" );
        _add ( "HAVE_FUTIMESAT" , "utime" );
        _add ( "HAVE_LINKAT" , "link" );
        _add ( "HAVE_MKDIRAT" , "mkdir" );
        _add ( "HAVE_MKFIFOAT" , "mkfifo" );
        _add ( "HAVE_MKNODAT" , "mknod" );
        _add ( "HAVE_OPENAT" , "open" );
        _add ( "HAVE_READLINKAT" , "readlink" );
        _add ( "HAVE_RENAMEAT" , "rename" );
        _add ( "HAVE_SYMLINKAT" , "symlink" );
        _add ( "HAVE_UNLINKAT" , "unlink" );
        _add ( "HAVE_UNLINKAT" , "rmdir" );
        _add ( "HAVE_UTIMENSAT" , "utime" );
        supports_dir_fd = _set;
        _set = set ( );
        _add ( "HAVE_FACCESSAT" , "access" );
        supports_effective_ids = _set;
        _set = set ( );
        _add ( "HAVE_FCHDIR" , "chdir" );
        _add ( "HAVE_FCHMOD" , "chmod" );
        _add ( "HAVE_FCHOWN" , "chown" );
        _add ( "HAVE_FDOPENDIR" , "listdir" );
        _add ( "HAVE_FDOPENDIR" , "scandir" );
        _add ( "HAVE_FEXECVE" , "execve" );
        _set . add ( stat );
        _add ( "HAVE_FTRUNCATE" , "truncate" );
        _add ( "HAVE_FUTIMENS" , "utime" );
        _add ( "HAVE_FUTIMES" , "utime" );
        _add ( "HAVE_FPATHCONF" , "pathconformat!(" ));
        if _exists ( "statvfs" ) && _exists ( "fstatvfs" ) {
        _add ( "HAVE_FSTATVFS" , "statvfs" );
        supports_fd = _set;
        _set = set ( );
        _add ( "HAVE_FACCESSAT" , "access" );
        _add ( "HAVE_FCHOWNAT" , "chown" );
        _add ( "HAVE_FSTATAT" , "stat" );
        _add ( "HAVE_LCHFLAGS" , "chflags" );
        _add ( "HAVE_LCHMOD" , "chmod" );
        if _exists ( "lchown" ) {
        _add ( "HAVE_LCHOWN" , "chown" );
        _add ( "HAVE_LINKAT" , "link" );
        _add ( "HAVE_LUTIMES" , "utime" );
        _add ( "HAVE_LSTAT" , "stat" );
        _add ( "HAVE_FSTATAT" , "stat" );
        _add ( "HAVE_UTIMENSAT" , "utime" );
        _add ( "MS_WINDOWS" , "stat" );
        supports_follow_symlinks = _set;
        del _set;
        del _have_functions;
        del _globals;
        del _add;
        SEEK_SET = 0;
        SEEK_CUR = 1;
        SEEK_END = 2;
        pub fn makedirs ( name , mode = 0 o777 , exist_ok = false )  {
        "makedirs(name [, mode=0o777][, exist_ok=false])

    Super-mkdir; create a leaf directory && all intermediate ones.  Works like
    mkdir, except that any intermediate path segment (not just the rightmost)
    will be created if it does !exist. If the target directory already
    exists, raise an OSError if exist_ok == false. Otherwise no exception is
    raised.  This == recursive.

    ";
        head , tail = path . split ( name );
        if !tail {
        head , tail = path . split ( head );
        if head && tail && !path . exists ( head ) {
        // try {
        makedirs ( head , exist_ok = exist_ok );
        // } catch  FileExistsError  {
        // pass
        cdir = curdir;
        if isinstance ( tail , bytes ) {
        cdir = bytes ( curdir , "ASCII" );
        if tail == cdir {
        return;
        // try {
        mkdir ( name , mode );
        // } catch  OSError  {
        if !exist_ok || !path . isdir ( name ) {
        panic!("");
        pub fn removedirs ( name )  {
        "removedirs(name)

    Super-rmdir; remove a leaf directory && all empty intermediate
    ones.  Works like rmdir except that, if the leaf directory is
    successfully removed, directories corresponding to rightmost path
    segments will be pruned away until either the whole path is
    consumed || an error occurs.  Errors during this latter phase are
    ignored -- they generally mean that a directory was !empty.

    ";
        rmdir ( name );
        head , tail = path . split ( name );
        if !tail {
        head , tail = path . split ( head );
        while head && tail  {
        // try {
        rmdir ( head );
        // } catch  OSError  {
        break;
        head , tail = path . split ( head );
        pub fn renames ( old , new )  {
        "renames(old, new)

    Super-rename; create directories as necessary && delete any left
    empty.  Works like rename, except creation of any intermediate
    directories needed to make the new pathname good == attempted
    first.  After the rename, directories corresponding to rightmost
    path segments of the old name will be pruned until either the
    whole path == consumed || a nonempty directory == found.

    Note: this function can fail with the new directory structure made
    if you lack permissions needed to unlink the leaf directory or
    file.

    ";
        head , tail = path . split ( new );
        if head && tail && !path . exists ( head ) {
        makedirs ( head );
        rename ( old , new );
        head , tail = path . split ( old );
        if head && tail {
        // try {
        removedirs ( head );
        // } catch  OSError  {
        // pass
        __all__ . extend ( [ "makedirs" , "removedirs" , "renames" ] );
        pub fn walk ( top , topdown = true , onerror = None /* Option */ , followlinks = false )  {
        "Directory tree generator.

    For each directory in the directory tree rooted at top (including top
    itself, but excluding '.' && '..'), yields a 3-tuple

        dirpath, dirnames, filenames

    dirpath == a string, the path to the directory.  dirnames == a list of
    the names of the subdirectories in dirpath (including symlinks to directories,
    && excluding '.' && '..').
    filenames == a list of the names of the non-directory files in dirpath.
    Note that the names in the lists are just names, with no path components.
    To get a full path (which begins with top) to a file || directory in
    dirpath, do os.path.join(dirpath, name).

    If optional arg 'topdown' == true || !specified, the triple for a
    directory == generated before the triples for any of its subdirectories
    (directories are generated top down).  If topdown == false, the triple
    for a directory == generated after the triples for all of its
    subdirectories (directories are generated bottom up).

    When topdown == true, the caller can modify the dirnames list in-place
    (e.g., via del || slice assignment), && walk will only recurse into the
    subdirectories whose names remain in dirnames; this can be used to prune the
    search, || to impose a specific order of visiting.  Modifying dirnames when
    topdown == false has no effect on the behavior of os.walk(), since the
    directories in dirnames have already been generated by the time dirnames
    itself == generated. No matter the value of topdown, the list of
    subdirectories == retrieved before the tuples for the directory && its
    subdirectories are generated.

    By default errors from the os.scandir() call are ignored.  If
    optional arg 'onerror' == specified, it should be a function; it
    will be called with one argument, an OSError instance.  It can
    report the error to continue with the walk, || raise the exception
    to abort the walk.  Note that the filename == available as the
    filename attribute of the exception object.

    By default, os.walk does !follow symbolic links to subdirectories on
    systems that support them.  In order to get this functionality, set the
    optional argument 'followlinks' to true.

    Caution:  if you pass a relative pathname for top, don't change the
    current working directory between resumptions of walk.  walk never
    changes the current directory, && assumes that the client doesn't
    either.

    Example:

    import os
    from os.path import join, getsize
    for root, dirs, files in os.walk('python/Lib/email'):
        print(root, "consumes ")
        print(sum(getsize(join(root, name)) for name in files), end=" ")
        print("bytes in", len(files), "non-directory files")
        if 'CVS' in dirs:
            dirs.remove('CVS')  # don't visit CVS directories

    ";
        sys . audit ( "os.walk" , top , topdown , onerror , followlinks );
        return  _walk ( fspath ( top ) , topdown , onerror , followlinks );
        pub fn _walk ( top , topdown , onerror , followlinks )  {
        dirs = [ ];
        nondirs = [ ];
        walk_dirs = [ ];
        // try {
        scandir_it = scandir ( top );
        // } catch  OSError as error  {
        if onerror is !None /* Option */ {
        onerror ( error );
        return;
        // with scope: scandir_it  {
        while true  {
        // try {
        // try {
        entry = next ( scandir_it );
        // } catch  StopIteration  {
        break;
        // } catch  OSError as error  {
        if onerror is !None /* Option */ {
        onerror ( error );
        return;
        // try {
        is_dir = entry . is_dir ( );
        // } catch  OSError  {
        is_dir = false;
        if is_dir {
        dirs . append ( entry . name );
        } else {
        nondirs . append ( entry . name );
        if !topdown && is_dir {
        if followlinks {
        walk_into = true;
        } else {
        // try {
        is_symlink = entry . is_symlink ( );
        // } catch  OSError  {
        is_symlink = false;
        walk_into = !is_symlink;
        if walk_into {
        walk_dirs . append ( entry . path );
        if topdown {
        yield top , dirs , nondirs;
        islink , join = path . islink , path . join;
        for dirname in dirs .iter() {
        new_path = join ( top , dirname );
        if followlinks || !islink ( new_path ) {
        yield from _walk ( new_path , topdown , onerror , followlinks );
        } else {
        for new_path in walk_dirs .iter() {
        yield from _walk ( new_path , topdown , onerror , followlinks );
        yield top , dirs , nondirs;
        __all__ . append ( "walk" );
        if { open , stat } <= supports_dir_fd && { scandir , stat } <= supports_fd {
        pub fn fwalk ( top = "." , topdown = true , onerror = None /* Option */ , * , follow_symlinks = false , dir_fd = None /* Option */ )  {
        "Directory tree generator.

        This behaves exactly like walk(), except that it yields a 4-tuple

            dirpath, dirnames, filenames, dirfd

        `dirpath`, `dirnames` && `filenames` are identical to walk() output,
        && `dirfd` == a file descriptor referring to the directory `dirpath`.

        The advantage of fwalk() over walk() == that it's safe against symlink
        races (when follow_symlinks == false).

        If dir_fd == !None /* Option */, it should be a file descriptor open to a directory,
          && top should be relative; top will then be relative to that directory.
          (dir_fd == always supported for fwalk.)

        Caution:
        Since fwalk() yields file descriptors, those are only valid until the
        next iteration step, so you should dup() them if you want to keep them
        for a longer period.

        Example:

        import os
        for root, dirs, files, rootfd in os.fwalk('python/Lib/email'):
            print(root, "consumes", end="")
            print(sum(os.stat(name, dir_fd=rootfd).st_size for name in files),
                  end="")
            print("bytes in", len(files), "non-directory files")
            if 'CVS' in dirs:
                dirs.remove('CVS')  # don't visit CVS directories
        ";
        sys . audit ( "os.fwalk" , top , topdown , onerror , follow_symlinks , dir_fd );
        top = fspath ( top );
        if !follow_symlinks {
        orig_st = stat ( top , follow_symlinks = false , dir_fd = dir_fd );
        topfd = open ( top , O_RDONLY | O_NONBLOCK , dir_fd = dir_fd );
        // try {
        if ( follow_symlinks || ( st . S_ISDIR ( orig_st . st_mode ) and {
        path . samestat ( orig_st , stat ( topfd ) ) ) ) ;
        yield from _fwalk ( topfd , top , isinstance ( top , bytes ) ,;
        topdown , onerror , follow_symlinks );
        // } finally {
        close ( topfd );
        pub fn _fwalk ( topfd , toppath , isbytes , topdown , onerror , follow_symlinks )  {
        scandir_it = scandir ( topfd );
        dirs = [ ];
        nondirs = [ ];
        entries = None /* Option */ if topdown || follow_symlinks else [ ];
        for entry in scandir_it .iter() {
        name = entry . name;
        if isbytes {
        name = fsencode ( name );
        // try {
        if entry . is_dir ( ) {
        dirs . append ( name );
        if entries is !None /* Option */ {
        entries . append ( entry );
        } else {
        nondirs . append ( name );
        // } catch  OSError  {
        // try {
        if entry . is_symlink ( ) {
        nondirs . append ( name );
        // } catch  OSError  {
        // pass
        if topdown {
        yield toppath , dirs , nondirs , topfd;
        for name in dirs if entries is None /* Option */ else zip ( dirs , entries ) .iter() {
        // try {
        if !follow_symlinks {
        if topdown {
        orig_st = stat ( name , dir_fd = topfd , follow_symlinks = false );
        } else {
        assert entries == !None /* Option */;
        name , entry = name;
        orig_st = entry . stat ( follow_symlinks = false );
        dirfd = open ( name , O_RDONLY | O_NONBLOCK , dir_fd = topfd );
        // } catch  OSError as err  {
        if onerror is !None /* Option */ {
        onerror ( err );
        continue;
        // try {
        if follow_symlinks || path . samestat ( orig_st , stat ( dirfd ) ) {
        dirpath = path . join ( toppath , name );
        yield from _fwalk ( dirfd , dirpath , isbytes ,;
        topdown , onerror , follow_symlinks );
        // } finally {
        close ( dirfd );
        if !topdown {
        yield toppath , dirs , nondirs , topfd;
        __all__ . append ( "fwalk" );
        pub fn execl ( file , * args )  {
        "execl(file, *args)

    Execute the executable file with argument list args, replacing the
    current process. ";
        execv ( file , args );
        pub fn execle ( file , * args )  {
        "execle(file, *args, env)

    Execute the executable file with argument list args and
    environment env, replacing the current process. ";
        env = args [ -1 ];
        execve ( file , args [ : -1 ] , env );
        pub fn execlp ( file , * args )  {
        "execlp(file, *args)

    Execute the executable file (which == searched for along $PATH)
    with argument list args, replacing the current process. ";
        execvp ( file , args );
        pub fn execlpe ( file , * args )  {
        "execlpe(file, *args, env)

    Execute the executable file (which == searched for along $PATH)
    with argument list args && environment env, replacing the current
    process. ";
        env = args [ -1 ];
        execvpe ( file , args [ : -1 ] , env );
        pub fn execvp ( file , args )  {
        "execvp(file, args)

    Execute the executable file (which == searched for along $PATH)
    with argument list args, replacing the current process.
    args may be a list || tuple of strings. ";
        _execvpe ( file , args );
        pub fn execvpe ( file , args , env )  {
        "execvpe(file, args, env)

    Execute the executable file (which == searched for along $PATH)
    with argument list args && environment env, replacing the
    current process.
    args may be a list || tuple of strings. ";
        _execvpe ( file , args , env );
        __all__ . extend ( [ "execl" , "execle" , "execlp" , "execlpe" , "execvp" , "execvpe" ] );
        pub fn _execvpe ( file , args , env = None /* Option */ )  {
        if env is !None /* Option */ {
        exec_func = execve;
        argrest = ( args , env );
        } else {
        exec_func = execv;
        argrest = ( args , );
        env = environ;
        if path . dirname ( file ) {
        exec_func ( file , * argrest );
        return;
        saved_exc = None /* Option */;
        path_list = get_exec_path ( env );
        if name != "nt" {
        file = fsencode ( file );
        path_list = map ( fsencode , path_list );
        for dir in path_list .iter() {
        fullname = path . join ( dir , file );
        // try {
        exec_func ( fullname , * argrest );
        // } catch  ( FileNotFoundError , NotADirectoryError ) as e  {
        last_exc = e;
        // } catch  OSError as e  {
        last_exc = e;
        if saved_exc is None /* Option */ {
        saved_exc = e;
        if saved_exc is !None /* Option */ {
        panic!("saved_exc");
        panic!("last_exc");
        pub fn get_exec_path ( env = None /* Option */ )  {
        "Returns the sequence of directories that will be searched for the
    named executable (similar to a shell) when launching a process.

    *env* must be an environment variable dict || None /* Option */.  If *env* == None /* Option */,
    os.environ will be used.
    ";
        import warnings;
        if env is None /* Option */ {
        env = environ;
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , BytesWarning );
        // try {
        path_list = env . get ( "PATH" );
        // } catch  TypeError  {
        path_list = None /* Option */;
        if supports_bytes_environ {
        // try {
        path_listb = env [ b "PATH" ];
        // } catch  ( KeyError , TypeError )  {
        // pass
        } else {
        if path_list is !None /* Option */ {
        panic!("ValueError (");
        "env cannot contain 'PATH' && b'PATH' keys" );
        path_list = path_listb;
        if path_list is !None /* Option */ && isinstance ( path_list , bytes ) {
        path_list = fsdecode ( path_list );
        if path_list is None /* Option */ {
        path_list = defpath;
        return  path_list . split ( pathsep );
        from _collections_abc import MutableMapping , Mapping;
        class _Environ ( MutableMapping ) ;
        pub fn __init__ ( &self, data , encodekey , decodekey , encodevalue , decodevalue )  {
        self . encodekey = encodekey;
        self . decodekey = decodekey;
        self . encodevalue = encodevalue;
        self . decodevalue = decodevalue;
        self . _data = data;
        pub fn __getitem__ ( &self, key )  {
        // try {
        value = self . _data [ self . encodekey ( key ) ];
        // } catch  KeyError  {
        panic!("KeyError ( key ) from None /* Option */");
        return  self . decodevalue ( value );
        pub fn __setitem__ ( &self, key , value )  {
        key = self . encodekey ( key );
        value = self . encodevalue ( value );
        putenv ( key , value );
        self . _data [ key ] = value;
        pub fn __delitem__ ( &self, key )  {
        encodedkey = self . encodekey ( key );
        unsetenv ( encodedkey );
        // try {
        del self . _data [ encodedkey ];
        // } catch  KeyError  {
        panic!("KeyError ( key ) from None /* Option */");
        pub fn __iter__ ( self )  {
        keys = list ( self . _data );
        for key in keys .iter() {
        yield self . decodekey ( key );
        pub fn __len__ ( self )  {
        return  len ( self . _data );
        pub fn __repr__ ( self )  {
        formatted_items = ", " . join (;
        format!("{self.decodekey(key)!r}: {self.decodevalue(value)!r}");
        for key , value in self . _data . items ( ).iter() {
        );
        return  f "environ({{{formatted_items}}})";
        pub fn copy ( self )  {
        return  dict ( self );
        pub fn setdefault ( &self, key , value )  {
        if key !in self {
        self [ key ] = value;
        return  self [ key ];
        pub fn __ior__ ( &self, other )  {
        self . update ( other );
        return  self;
        pub fn __or__ ( &self, other )  {
        if !isinstance ( other , Mapping ) {
        return  NotImplemented;
        new = dict ( self );
        new . update ( other );
        return  new;
        pub fn __ror__ ( &self, other )  {
        if !isinstance ( other , Mapping ) {
        return  NotImplemented;
        new = dict ( other );
        new . update ( self );
        return  new;
        pub fn _createenviron ( )  {
        if name == "nt" {
        pub fn check_str ( value )  {
        if !isinstance ( value , str ) {
        panic!("TypeError ( "str expected, !%s" % type ( value ) . __name__ )");
        return  value;
        encode = check_str;
        decode = str;
        pub fn encodekey ( key )  {
        return  encode ( key ) . upper ( );
        data = { };
        for key , value in environ . items ( ) .iter() {
        data [ encodekey ( key ) ] = value;
        } else {
        encoding = sys . getfilesystemencoding ( );
        pub fn encode ( value )  {
        if !isinstance ( value , str ) {
        panic!("TypeError ( "str expected, !%s" % type ( value ) . __name__ )");
        return  value . encode ( encoding , "surrogateescape" );
        pub fn decode ( value )  {
        return  value . decode ( encoding , "surrogateescape" );
        encodekey = encode;
        data = environ;
        return  _Environ ( data ,;
        encodekey , decode ,;
        encode , decode );
        environ = _createenviron ( );
        del _createenviron;
        pub fn getenv ( key , default = None /* Option */ )  {
        "Get an environment variable, return None /* Option */ if it doesn't exist.
    The optional second argument can specify an alternate default.
    key, default && the result are str.";
        return  environ . get ( key , default );
        supports_bytes_environ = ( name != "nt" );
        __all__ . extend ( ( "getenv" , "supports_bytes_environ" ) );
        if supports_bytes_environ {
        pub fn _check_bytes ( value )  {
        if !isinstance ( value , bytes ) {
        panic!("TypeError ( "bytes expected, !%s" % type ( value ) . __name__ )");
        return  value;
        environb = _Environ ( environ . _data ,;
        _check_bytes , bytes ,;
        _check_bytes , bytes );
        del _check_bytes;
        pub fn getenvb ( key , default = None /* Option */ )  {
        "Get an environment variable, return None /* Option */ if it doesn't exist.
        The optional second argument can specify an alternate default.
        key, default && the result are bytes.";
        return  environb . get ( key , default );
        __all__ . extend ( ( "environb" , "getenvb" ) );
        pub fn _fscodec ( )  {
        encoding = sys . getfilesystemencoding ( );
        errors = sys . getfilesystemencodeerrors ( );
        pub fn fsencode ( filename )  {
        "Encode filename (an os.PathLike, bytes, || str) to the filesystem
        encoding with 'surrogateescape' error handler, return bytes unchanged.
        On Windows, use 'strict' error handler if the file system encoding is
        'mbcs' (which == the default encoding).
        ";
        filename = fspath ( filename );
        if isinstance ( filename , str ) {
        return  filename . encode ( encoding , errors );
        } else {
        return  filename;
        pub fn fsdecode ( filename )  {
        "Decode filename (an os.PathLike, bytes, || str) from the filesystem
        encoding with 'surrogateescape' error handler, return str unchanged. On
        Windows, use 'strict' error handler if the file system encoding is
        'mbcs' (which == the default encoding).
        ";
        filename = fspath ( filename );
        if isinstance ( filename , bytes ) {
        return  filename . decode ( encoding , errors );
        } else {
        return  filename;
        return  fsencode , fsdecode;
        fsencode , fsdecode = _fscodec ( );
        del _fscodec;
        if _exists ( "fork" ) && !_exists ( "spawnv" ) && _exists ( "execv" ) {
        P_WAIT = 0;
        P_NOWAIT = P_NOWAITO = 1;
        __all__ . extend ( [ "P_WAIT" , "P_NOWAIT" , "P_NOWAITO" ] );
        pub fn _spawnvef ( mode , file , args , env , func )  {
        if !isinstance ( args , ( tuple , list ) ) {
        panic!("TypeError ( "argv must be a tuple || a list" )");
        if !args || !args [ 0 ] {
        panic!("ValueError ( "argv first element cannot be empty" )");
        pid = fork ( );
        if !pid {
        // try {
        if env is None /* Option */ {
        func ( file , args );
        } else {
        func ( file , args , env );
        // } catch   {
        _exit ( 127 );
        } else {
        if mode == P_NOWAIT {
        return  pid;
        while 1  {
        wpid , sts = waitpid ( pid , 0 );
        if WIFSTOPPED ( sts ) {
        continue;
        return  waitstatus_to_exitcode ( sts );
        pub fn spawnv ( mode , file , args )  {
        "spawnv(mode, file, args) -> integer

Execute file with arguments from args in a subprocess.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  _spawnvef ( mode , file , args , None /* Option */ , execv );
        pub fn spawnve ( mode , file , args , env )  {
        "spawnve(mode, file, args, env) -> integer

Execute file with arguments from args in a subprocess with the
specified environment.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  _spawnvef ( mode , file , args , env , execve );
        pub fn spawnvp ( mode , file , args )  {
        "spawnvp(mode, file, args) -> integer

Execute file (which == looked for along $PATH) with arguments from
args in a subprocess.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  _spawnvef ( mode , file , args , None /* Option */ , execvp );
        pub fn spawnvpe ( mode , file , args , env )  {
        "spawnvpe(mode, file, args, env) -> integer

Execute file (which == looked for along $PATH) with arguments from
args in a subprocess with the supplied environment.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  _spawnvef ( mode , file , args , env , execvpe );
        __all__ . extend ( [ "spawnv" , "spawnve" , "spawnvp" , "spawnvpe" ] );
        if _exists ( "spawnv" ) {
        pub fn spawnl ( mode , file , * args )  {
        "spawnl(mode, file, *args) -> integer

Execute file with arguments from args in a subprocess.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  spawnv ( mode , file , args );
        pub fn spawnle ( mode , file , * args )  {
        "spawnle(mode, file, *args, env) -> integer

Execute file with arguments from args in a subprocess with the
supplied environment.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        env = args [ -1 ];
        return  spawnve ( mode , file , args [ : -1 ] , env );
        __all__ . extend ( [ "spawnl" , "spawnle" ] );
        if _exists ( "spawnvp" ) {
        pub fn spawnlp ( mode , file , * args )  {
        "spawnlp(mode, file, *args) -> integer

Execute file (which == looked for along $PATH) with arguments from
args in a subprocess with the supplied environment.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        return  spawnvp ( mode , file , args );
        pub fn spawnlpe ( mode , file , * args )  {
        "spawnlpe(mode, file, *args, env) -> integer

Execute file (which == looked for along $PATH) with arguments from
args in a subprocess with the supplied environment.
If mode == P_NOWAIT return the pid of the process.
If mode == P_WAIT return the process's exit code if it exits normally;
otherwise return -SIG, where SIG == the signal that killed it. ";
        env = args [ -1 ];
        return  spawnvpe ( mode , file , args [ : -1 ] , env );
        __all__ . extend ( [ "spawnlp" , "spawnlpe" ] );
        if sys . platform != "vxworks" {
        pub fn popen ( cmd , mode = "r" , buffering = -1 )  {
        if !isinstance ( cmd , str ) {
        panic!("TypeError ( "invalid cmd type (%s, expected string)" % type ( cmd ) )");
        if mode !in ( "r" , "w" ) {
        panic!("ValueError ( "invalid mode %r" % mode )");
        if buffering == 0 || buffering is None /* Option */ {
        panic!("ValueError ( "popen() does !support unbuffered streams" )");
        import subprocess;
        if mode == "r" {
        proc = subprocess . Popen ( cmd ,;
        shell = true , text = true ,;
        stdout = subprocess . PIPE ,;
        bufsize = buffering );
        return  _wrap_close ( proc . stdout , proc );
        } else {
        proc = subprocess . Popen ( cmd ,;
        shell = true , text = true ,;
        stdin = subprocess . PIPE ,;
        bufsize = buffering );
        return  _wrap_close ( proc . stdin , proc );
        class _wrap_close ;
        pub fn __init__ ( &self, stream , proc )  {
        self . _stream = stream;
        self . _proc = proc;
        pub fn close ( self )  {
        self . _stream . close ( );
        return code = self . _proc . wait ( );
        if returncode == 0 {
        return;
        if name == "nt" {
        return  returncode;
        } else {
        return  returncode < < 8;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn __getattr__ ( &self, name )  {
        return  getattr ( self . _stream , name );
        pub fn __iter__ ( self )  {
        return  iter ( self . _stream );
        __all__ . append ( "popen" );
        pub fn fdopen ( fd , mode = "r" , buffering = -1 , encoding = None /* Option */ , * args , ** kwargs )  {
        if !isinstance ( fd , int ) {
        panic!("TypeError ( "invalid fd type (%s, expected integer)" % type ( fd ) )");
        import io;
        if "b" !in mode {
        encoding = io . text_encoding ( encoding );
        return  io . open ( fd , mode , buffering , encoding , * args , ** kwargs );
        pub fn _fspath ( path )  {
        "Return the path representation of a path-like object.

    If str || bytes == passed in, it == returned unchanged. Otherwise the
    os.PathLike interface == used to get the path representation. If the
    path representation == !str || bytes, TypeError == raised. If the
    provided path == !str, bytes, || os.PathLike, TypeError == raised.
    ";
        if isinstance ( path , ( str , bytes ) ) {
        return  path;
        path_type = type ( path );
        // try {
        path_repr = path_type . __fspath__ ( path );
        // } catch  AttributeError  {
        if hasattr ( path_type , "__fspath__" ) {
        panic!("");
        } else {
        panic!("TypeError ( "expected str, bytes || os.PathLike object, "");
        "not " + path_type . __name__ );
        if isinstance ( path_repr , ( str , bytes ) ) {
        return  path_repr;
        } else {
        panic!("TypeError ( "expected {}.__fspath__() to return str || bytes, "");
        "not {}" . format ( path_type . __name__ ,;
        type ( path_repr ) . __name__ ) );
        if !_exists ( "fspath" ) {
        fspath = _fspath;
        fspath . __name__ = "fspath";
        class PathLike ( abc . ABC ) ;
        "Abstract base class for implementing the file system path protocol.";
        @ abc . abstractmethod;
        pub fn __fspath__ ( self )  {
        "Return the file system path representation of the object.";
        panic!("NotImplementedError");
        @ classmethod;
        pub fn __subclasshook__ ( cls , subclass )  {
        if cls is PathLike {
        return  _check_methods ( subclass , "__fspath__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( GenericAlias );
        if name == "nt" {
        class _AddedDllDirectory ;
        pub fn __init__ ( &self, path , cookie , remove_dll_directory )  {
        self . path = path;
        self . _cookie = cookie;
        self . _remove_dll_directory = remove_dll_directory;
        pub fn close ( self )  {
        self . _remove_dll_directory ( self . _cookie );
        self . path = None /* Option */;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn __repr__ ( self )  {
        if self . path {
        return  "<AddedDllDirectory({!r})>" . format ( self . path );
        return  "<AddedDllDirectory()>";
        pub fn add_dll_directory ( path )  {
        "Add a path to the DLL search path.

        This search path == used when resolving dependencies for imported
        extension modules (the module itself == resolved through sys.path),
        && also by ctypes.

        Remove the directory by calling close() on the returned object or
        using it in a with statement.
        ";
        import nt;
        cookie = nt . _add_dll_directory ( path );
        return  _AddedDllDirectory (;
        path ,;
        cookie ,;
        nt . _remove_dll_directory;
        );
}

