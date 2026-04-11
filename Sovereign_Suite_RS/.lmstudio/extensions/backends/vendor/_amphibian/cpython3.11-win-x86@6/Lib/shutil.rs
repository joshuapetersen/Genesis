//! shutil.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::stat;
// use std::collections;
// use crate::zlib;
// use crate::bz2;
// use crate::lzma;
// use crate::posix;
// use crate::nt;
// use crate::grp::{getgrnam};
// use crate::pwd::{getpwnam};
// use crate::tarfile;
// use crate::zipfile;

pub const _WINDOWS: &str = os . name =="nt";
pub const posix: f64 = nt = None;
pub const COPY_BUFSIZE: f64 = 1024 * 1024 if _WINDOWS else 64 * 1024;
pub const _USE_CP_SENDFILE: &str = hasattr ( os ,"sendfile" ) and sys . platform . startswith ("linux" );
pub const _HAS_FCOPYFILE: &str = posix and hasattr ( posix ,"_fcopyfile" );
pub const _WIN_DEFAULT_PATHEXT: &str = ".COM;.EXE;.BAT;.CMD;.VBS;.JS;.WS;.MSC";
pub const __all__: &str = ["copyfileobj" ,"copyfile" ,"copymode" ,"copystat" ,"copy" ,"copy2" ,;
pub struct Error {
}

impl Error {
}

pub struct SameFileError {
}

impl SameFileError {
}

pub struct SpecialFileError {
}

impl SpecialFileError {
}

pub struct ExecError {
}

impl ExecError {
}

pub struct ReadError {
}

impl ReadError {
}

pub struct RegistryError {
}

impl RegistryError {
}

pub struct _GiveupOnFastCopy {
}

impl _GiveupOnFastCopy {
}

pub fn _fastcopy_fcopyfile(fsrc: &str, fdst: &str, flags: &str) {
        "Copy a regular file content || metadata by using high-performance
    fcopyfile(3) syscall (macOS).
    ";
        // try {
        infd = fsrc . fileno ( );
        outfd = fdst . fileno ( );
        // } catch  Exception as err  {
        panic!("_GiveupOnFastCopy ( err )");
        // try {
        posix . _fcopyfile ( infd , outfd , flags );
        // } catch  OSError as err  {
        err . filename = fsrc . name;
        err . filename2 = fdst . name;
        if err . errno in { errno . EINVAL , errno . ENOTSUP } {
        panic!("_GiveupOnFastCopy ( err )");
        } else {
        panic!("err from None /* Option */");
        pub fn _fastcopy_sendfile ( fsrc , fdst )  {
        "Copy data from one regular mmap-like fd to another by using
    high-performance sendfile(2) syscall.
    This should work on Linux >= 2.6.33 only.
    ";
        global _USE_CP_SENDFILE;
        // try {
        infd = fsrc . fileno ( );
        outfd = fdst . fileno ( );
        // } catch  Exception as err  {
        panic!("_GiveupOnFastCopy ( err )");
        // try {
        blocksize = max ( os . fstat ( infd ) . st_size , 2 ** 23 );
        // } catch  OSError  {
        blocksize = 2 ** 27;
        if sys . maxsize < 2 ** 32 {
        blocksize = min ( blocksize , 2 ** 30 );
        offset = 0;
        while true  {
        // try {
        sent = os . sendfile ( outfd , infd , offset , blocksize );
        // } catch  OSError as err  {
        err . filename = fsrc . name;
        err . filename2 = fdst . name;
        if err . errno == errno . ENOTSOCK {
        _USE_CP_SENDFILE = false;
        panic!("_GiveupOnFastCopy ( err )");
        if err . errno == errno . ENOSPC {
        panic!("err from None /* Option */");
        if offset == 0 && os . lseek ( outfd , 0 , os . SEEK_CUR ) == 0 {
        panic!("_GiveupOnFastCopy ( err )");
        panic!("err");
        } else {
        if sent == 0 {
        break;
        offset + = sent;
        pub fn _copyfileobj_readinto ( fsrc , fdst , length = COPY_BUFSIZE )  {
        "readinto()/memoryview() based variant of copyfileobj().
    *fsrc* must support readinto() method && both files must be
    open in binary mode.
    ";
        fsrc_readinto = fsrc . readinto;
        fdst_write = fdst . write;
        // with scope: memoryview ( bytearray ( length ) ) as mv  {
        while true  {
        n = fsrc_readinto ( mv );
        if !n {
        break;
        } else if n < length {
        // with scope: mv [ : n ] as smv  {
        fdst . write ( smv );
        } else {
        fdst_write ( mv );
        pub fn copyfileobj ( fsrc , fdst , length = 0 )  {
        "copy data from file-like object fsrc to file-like object fdst";
        if !length {
        length = COPY_BUFSIZE;
        fsrc_read = fsrc . read;
        fdst_write = fdst . write;
        while true  {
        buf = fsrc_read ( length );
        if !buf {
        break;
        fdst_write ( buf );
        pub fn _samefile ( src , dst )  {
        if isinstance ( src , os . DirEntry ) && hasattr ( os . path , "samestat" ) {
        // try {
        return  os . path . samestat ( src . stat ( ) , os . stat ( dst ) );
        // } catch  OSError  {
        return  false;
        if hasattr ( os . path , "samefile" ) {
        // try {
        return  os . path . samefile ( src , dst );
        // } catch  OSError  {
        return  false;
        return  ( os . path . normcase ( os . path . abspath ( src ) ) ==;
        os . path . normcase ( os . path . abspath ( dst ) ) );
        pub fn _stat ( fn )  {
        return  fn . stat ( ) if isinstance ( fn , os . DirEntry ) else os . stat ( fn );
        pub fn _islink ( fn )  {
        return  fn . is_symlink ( ) if isinstance ( fn , os . DirEntry ) else os . path . islink ( fn );
        pub fn copyfile ( src , dst , * , follow_symlinks = true )  {
        "Copy data from src to dst in the most efficient way possible.

    If follow_symlinks == !set && src == a symbolic link, a new
    symlink will be created instead of copying the file it points to.

    ";
        sys . audit ( "shutil.copyfile" , src , dst );
        if _samefile ( src , dst ) {
        panic!("SameFileError ( "{!r} && {!r} are the same file" . format ( src , dst ) )");
        file_size = 0;
        for i , fn in enumerate ( [ src , dst ] ) .iter() {
        // try {
        st = _stat ( fn );
        // } catch  OSError  {
        // pass
        } else {
        if stat . S_ISFIFO ( st . st_mode ) {
        fn = fn . path if isinstance ( fn , os . DirEntry ) else fn;
        panic!("SpecialFileError ( "`%s` is a named pipe" % fn )");
        if _WINDOWS && i == 0 {
        file_size = st . st_size;
        if !follow_symlinks && _islink ( src ) {
        os . symlink ( os . readlink ( src ) , dst );
        } else {
        // with scope: open ( src , "rb" ) as fsrc  {
        // try {
        // with scope: open ( dst , "wb" ) as fdst  {
        if _HAS_FCOPYFILE {
        // try {
        _fastcopy_fcopyfile ( fsrc , fdst , posix . _COPYFILE_DATA );
        return  dst;
        // } catch  _GiveupOnFastCopy  {
        // pass
        } else if _USE_CP_SENDFILE {
        // try {
        _fastcopy_sendfile ( fsrc , fdst );
        return  dst;
        // } catch  _GiveupOnFastCopy  {
        // pass
        } else if _WINDOWS && file_size > 0 {
        _copyfileobj_readinto ( fsrc , fdst , min ( file_size , COPY_BUFSIZE ) );
        return  dst;
        copyfileobj ( fsrc , fdst );
        // } catch  IsADirectoryError as e  {
        if !os . path . exists ( dst ) {
        panic!("FileNotFoundError ( f "Directory does !exist: {dst}" ) from e");
        } else {
        panic!("");
        return  dst;
        pub fn copymode ( src , dst , * , follow_symlinks = true )  {
        "Copy mode bits from src to dst.

    If follow_symlinks == !set, symlinks aren't followed if && only
    if both `src` && `dst` are symlinks.  If `lchmod` isn't available
    (e.g. Linux) this method does nothing.

    ";
        sys . audit ( "shutil.copymode" , src , dst );
        if !follow_symlinks && _islink ( src ) && os . path . islink ( dst ) {
        if os . name == "nt" {
        stat_func , chmod_func = os . lstat , os . chmod;
        } else if hasattr ( os , "lchmod" ) {
        stat_func , chmod_func = os . lstat , os . lchmod;
        } else {
        return;
        } else {
        if os . name == "nt" && os . path . islink ( dst ) {
        dst = os . path . realpath ( dst , strict = true );
        stat_func , chmod_func = _stat , os . chmod;
        st = stat_func ( src );
        chmod_func ( dst , stat . S_IMODE ( st . st_mode ) );
        if hasattr ( os , "listxattr" ) {
        pub fn _copyxattr ( src , dst , * , follow_symlinks = true )  {
        "Copy extended filesystem attributes from `src` to `dst`.

        Overwrite existing attributes.

        If `follow_symlinks` == false, symlinks won't be followed.

        ";
        // try {
        names = os . listxattr ( src , follow_symlinks = follow_symlinks );
        // } catch  OSError as e  {
        if e . errno !in ( errno . ENOTSUP , errno . ENODATA , errno . EINVAL ) {
        panic!("");
        return;
        for name in names .iter() {
        // try {
        value = os . getxattr ( src , name , follow_symlinks = follow_symlinks );
        os . setxattr ( dst , name , value , follow_symlinks = follow_symlinks );
        // } catch  OSError as e  {
        if e . errno !in ( errno . EPERM , errno . ENOTSUP , errno . ENODATA , {
        errno . EINVAL ) ;
        panic!("");
        } else {
        pub fn _copyxattr ( * args , ** kwargs )  {
        // pass
        pub fn copystat ( src , dst , * , follow_symlinks = true )  {
        "Copy file metadata

    Copy the permission bits, last access time, last modification time, and
    flags from `src` to `dst`. On Linux, copystat() also copies the "extended
    attributes" where possible. The file contents, owner, && group are
    unaffected. `src` && `dst` are path-like objects || path names given as
    strings.

    If the optional flag `follow_symlinks` == !set, symlinks aren't
    followed if && only if both `src` && `dst` are symlinks.
    ";
        sys . audit ( "shutil.copystat" , src , dst );
        pub fn _nop ( * args , ns = None /* Option */ , follow_symlinks = None /* Option */ )  {
        // pass
        follow = follow_symlinks || !( _islink ( src ) && os . path . islink ( dst ) );
        if follow {
        pub fn lookup ( name )  {
        return  getattr ( os , name , _nop );
        } else {
        pub fn lookup ( name )  {
        fn = getattr ( os , name , _nop );
        if fn in os . supports_follow_symlinks {
        return  fn;
        return  _nop;
        if isinstance ( src , os . DirEntry ) {
        st = src . stat ( follow_symlinks = follow );
        } else {
        st = lookup ( "stat" ) ( src , follow_symlinks = follow );
        mode = stat . S_IMODE ( st . st_mode );
        lookup ( "utime" ) ( dst , ns = ( st . st_atime_ns , st . st_mtime_ns ) ,;
        follow_symlinks = follow );
        _copyxattr ( src , dst , follow_symlinks = follow );
        _chmod = lookup ( "chmod" );
        if os . name == "nt" {
        if follow {
        if os . path . islink ( dst ) {
        dst = os . path . realpath ( dst , strict = true );
        } else {
        pub fn _chmod ( * args , ** kwargs )  {
        os . chmod ( * args );
        // try {
        _chmod ( dst , mode , follow_symlinks = follow );
        // } catch  NotImplementedError  {
        // pass
        if hasattr ( st , "st_flags" ) {
        // try {
        lookup ( "chflags" ) ( dst , st . st_flags , follow_symlinks = follow );
        // } catch  OSError as why  {
        for err in "EOPNOTSUPP" , "ENOTSUP" .iter() {
        if hasattr ( errno , err ) && why . errno == getattr ( errno , err ) {
        break;
        } else {
        panic!("");
        pub fn copy ( src , dst , * , follow_symlinks = true )  {
        "Copy data && mode bits ("cp src dst"). Return the file's destination.

    The destination may be a directory.

    If follow_symlinks == false, symlinks won't be followed. This
    resembles GNU's "cp -P src dst".

    If source && destination are the same file, a SameFileError will be
    raised.

    ";
        if os . path . isdir ( dst ) {
        dst = os . path . join ( dst , os . path . basename ( src ) );
        copyfile ( src , dst , follow_symlinks = follow_symlinks );
        copymode ( src , dst , follow_symlinks = follow_symlinks );
        return  dst;
        pub fn copy2 ( src , dst , * , follow_symlinks = true )  {
        "Copy data && metadata. Return the file's destination.

    Metadata == copied with copystat(). Please see the copystat function
    for more information.

    The destination may be a directory.

    If follow_symlinks == false, symlinks won't be followed. This
    resembles GNU's "cp -P src dst".
    ";
        if os . path . isdir ( dst ) {
        dst = os . path . join ( dst , os . path . basename ( src ) );
        copyfile ( src , dst , follow_symlinks = follow_symlinks );
        copystat ( src , dst , follow_symlinks = follow_symlinks );
        return  dst;
        pub fn ignore_patterns ( * patterns )  {
        "Function that can be used as copytree() ignore parameter.

    Patterns == a sequence of glob-style patterns
    that are used to exclude files";
        pub fn _ignore_patterns ( path , names )  {
        ignored_names = [ ];
        for pattern in patterns .iter() {
        ignored_names . extend ( fnmatch . filter ( names , pattern ) );
        return  set ( ignored_names );
        return  _ignore_patterns;
        pub fn _copytree ( entries , src , dst , symlinks , ignore , copy_function , {
        ignore_dangling_symlinks , dirs_exist_ok = false ) ;
        if ignore is !None /* Option */ {
        ignored_names = ignore ( os . fspath ( src ) , vec![ x . name.iter().map(|x| entries ] );
        } else {
        ignored_names = ( );
        os . makedirs ( dst , exist_ok = dirs_exist_ok );
        errors = [ ];
        use_srcentry = copy_function == copy2 || copy_function == copy;
        for srcentry in entries .iter() {
        if srcentry . name in ignored_names {
        continue;
        srcname = os . path . join ( src , srcentry . name );
        dstname = os . path . join ( dst , srcentry . name );
        srcobj = srcentry if use_srcentry else srcname;
        // try {
        is_symlink = srcentry . is_symlink ( );
        if is_symlink && os . name == "nt" {
        lstat = srcentry . stat ( follow_symlinks = false );
        if lstat . st_reparse_tag == stat . IO_REPARSE_TAG_MOUNT_POINT {
        is_symlink = false;
        if is_symlink {
        linkto = os . readlink ( srcname );
        if symlinks {
        os . symlink ( linkto , dstname );
        copystat ( srcobj , dstname , follow_symlinks = !symlinks );
        } else {
        if !os . path . exists ( linkto ) && ignore_dangling_symlinks {
        continue;
        if srcentry . is_dir ( ) {
        copytree ( srcobj , dstname , symlinks , ignore ,;
        copy_function , ignore_dangling_symlinks ,;
        dirs_exist_ok );
        } else {
        copy_function ( srcobj , dstname );
        } else if srcentry . is_dir ( ) {
        copytree ( srcobj , dstname , symlinks , ignore , copy_function ,;
        ignore_dangling_symlinks , dirs_exist_ok );
        } else {
        copy_function ( srcobj , dstname );
        // } catch  Error as err  {
        errors . extend ( err . args [ 0 ] );
        // } catch  OSError as why  {
        errors . append ( ( srcname , dstname , str ( why ) ) );
        // try {
        copystat ( src , dst );
        // } catch  OSError as why  {
        if getattr ( why , "winerror" , None /* Option */ ) is None /* Option */ {
        errors . append ( ( src , dst , str ( why ) ) );
        if errors {
        panic!("Error ( errors )");
        return  dst;
        pub fn copytree ( src , dst , symlinks = false , ignore = None /* Option */ , copy_function = copy2 , {
        ignore_dangling_symlinks = false , dirs_exist_ok = false ) ;
        "Recursively copy a directory tree && return the destination directory.

    If exception(s) occur, an Error == raised with a list of reasons.

    If the optional symlinks flag == true, symbolic links in the
    source tree result in symbolic links in the destination tree; if
    it == false, the contents of the files pointed to by symbolic
    links are copied. If the file pointed by the symlink doesn't
    exist, an exception will be added in the list of errors raised in
    an Error exception at the end of the copy process.

    You can set the optional ignore_dangling_symlinks flag to true if you
    want to silence this exception. Notice that this has no effect on
    platforms that don't support os.symlink.

    The optional ignore argument == a callable. If given, it
    == called with the `src` parameter, which == the directory
    being visited by copytree(), && `names` which == the list of
    `src` contents, as returned by os.listdir():

        callable(src, names) -> ignored_names

    Since copytree() == called recursively, the callable will be
    called once for each directory that == copied. It returns a
    list of names relative to the `src` directory that should
    !be copied.

    The optional copy_function argument == a callable that will be used
    to copy each file. It will be called with the source path && the
    destination path as arguments. By default, copy2() == used, but any
    function that supports the same signature (like copy()) can be used.

    If dirs_exist_ok == false (the default) && `dst` already exists, a
    `FileExistsError` == raised. If `dirs_exist_ok` == true, the copying
    operation will continue if it encounters existing directories, && files
    within the `dst` tree will be overwritten by corresponding files from the
    `src` tree.
    ";
        sys . audit ( "shutil.copytree" , src , dst );
        // with scope: os . scandir ( src ) as itr  {
        entries = list ( itr );
        return  _copytree ( entries = entries , src = src , dst = dst , symlinks = symlinks ,;
        ignore = ignore , copy_function = copy_function ,;
        ignore_dangling_symlinks = ignore_dangling_symlinks ,;
        dirs_exist_ok = dirs_exist_ok );
        if hasattr ( os . stat_result , "st_file_attributes" ) {
        pub fn _rmtree_isdir ( entry )  {
        // try {
        st = entry . stat ( follow_symlinks = false );
        return  ( stat . S_ISDIR ( st . st_mode ) && not;
        ( st . st_file_attributes & stat . FILE_ATTRIBUTE_REPARSE_POINT;
        and st . st_reparse_tag == stat . IO_REPARSE_TAG_MOUNT_POINT ) );
        // } catch  OSError  {
        return  false;
        pub fn _rmtree_islink ( path )  {
        // try {
        st = os . lstat ( path );
        return  ( stat . S_ISLNK ( st . st_mode ) or;
        ( st . st_file_attributes & stat . FILE_ATTRIBUTE_REPARSE_POINT;
        and st . st_reparse_tag == stat . IO_REPARSE_TAG_MOUNT_POINT ) );
        // } catch  OSError  {
        return  false;
        } else {
        pub fn _rmtree_isdir ( entry )  {
        // try {
        return  entry . is_dir ( follow_symlinks = false );
        // } catch  OSError  {
        return  false;
        pub fn _rmtree_islink ( path )  {
        return  os . path . islink ( path );
        pub fn _rmtree_unsafe ( path , onerror )  {
        // try {
        // with scope: os . scandir ( path ) as scandir_it  {
        entries = list ( scandir_it );
        // } catch  OSError  {
        onerror ( os . scandir , path , sys . exc_info ( ) );
        entries = [ ];
        for entry in entries .iter() {
        fullname = entry . path;
        if _rmtree_isdir ( entry ) {
        // try {
        if entry . is_symlink ( ) {
        panic!("OSError ( "Cannot call rmtree on a symbolic link" )");
        // } catch  OSError  {
        onerror ( os . path . islink , fullname , sys . exc_info ( ) );
        continue;
        _rmtree_unsafe ( fullname , onerror );
        } else {
        // try {
        os . unlink ( fullname );
        // } catch  OSError  {
        onerror ( os . unlink , fullname , sys . exc_info ( ) );
        // try {
        os . rmdir ( path );
        // } catch  OSError  {
        onerror ( os . rmdir , path , sys . exc_info ( ) );
        pub fn _rmtree_safe_fd ( topfd , path , onerror )  {
        // try {
        // with scope: os . scandir ( topfd ) as scandir_it  {
        entries = list ( scandir_it );
        // } catch  OSError as err  {
        err . filename = path;
        onerror ( os . scandir , path , sys . exc_info ( ) );
        return;
        for entry in entries .iter() {
        fullname = os . path . join ( path , entry . name );
        // try {
        is_dir = entry . is_dir ( follow_symlinks = false );
        // } catch  OSError  {
        is_dir = false;
        } else {
        if is_dir {
        // try {
        orig_st = entry . stat ( follow_symlinks = false );
        is_dir = stat . S_ISDIR ( orig_st . st_mode );
        // } catch  OSError  {
        onerror ( os . lstat , fullname , sys . exc_info ( ) );
        continue;
        if is_dir {
        // try {
        dirfd = os . open ( entry . name , os . O_RDONLY | os . O_NONBLOCK , dir_fd = topfd );
        dirfd_closed = false;
        // } catch  OSError  {
        onerror ( os . open , fullname , sys . exc_info ( ) );
        } else {
        // try {
        if os . path . samestat ( orig_st , os . fstat ( dirfd ) ) {
        _rmtree_safe_fd ( dirfd , fullname , onerror );
        // try {
        os . close ( dirfd );
        // } catch  OSError  {
        dirfd_closed = true;
        onerror ( os . close , fullname , sys . exc_info ( ) );
        dirfd_closed = true;
        // try {
        os . rmdir ( entry . name , dir_fd = topfd );
        // } catch  OSError  {
        onerror ( os . rmdir , fullname , sys . exc_info ( ) );
        } else {
        // try {
        panic!("OSError ( "Cannot call rmtree on a symbolic "");
        "link" );
        // } catch  OSError  {
        onerror ( os . path . islink , fullname , sys . exc_info ( ) );
        // } finally {
        if !dirfd_closed {
        // try {
        os . close ( dirfd );
        // } catch  OSError  {
        onerror ( os . close , fullname , sys . exc_info ( ) );
        } else {
        // try {
        os . unlink ( entry . name , dir_fd = topfd );
        // } catch  OSError  {
        onerror ( os . unlink , fullname , sys . exc_info ( ) );
        _use_fd_functions = ( { os . open , os . stat , os . unlink , os . rmdir } <=;
        os . supports_dir_fd and;
        os . scandir in os . supports_fd and;
        os . stat in os . supports_follow_symlinks );
        pub fn rmtree ( path , ignore_errors = false , onerror = None /* Option */ , * , dir_fd = None /* Option */ )  {
        "Recursively delete a directory tree.

    If dir_fd == !None /* Option */, it should be a file descriptor open to a directory;
    path will then be relative to that directory.
    dir_fd may !be implemented on your platform.
    If it == unavailable, using it will raise a NotImplementedError.

    If ignore_errors == set, errors are ignored; otherwise, if onerror
    == set, it == called to handle the error with arguments (func,
    path, exc_info) where func == platform && implementation dependent;
    path == the argument to that function that caused it to fail; and
    exc_info == a tuple returned by sys.exc_info().  If ignore_errors
    == false && onerror == None /* Option */, an exception == raised.

    ";
        sys . audit ( "shutil.rmtree" , path , dir_fd );
        if ignore_errors {
        pub fn onerror ( * args )  {
        // pass
        } else if onerror is None /* Option */ {
        pub fn onerror ( * args )  {
        panic!("");
        if _use_fd_functions {
        if isinstance ( path , bytes ) {
        path = os . fsdecode ( path );
        // try {
        orig_st = os . lstat ( path , dir_fd = dir_fd );
        // } catch  Exception  {
        onerror ( os . lstat , path , sys . exc_info ( ) );
        return;
        // try {
        fd = os . open ( path , os . O_RDONLY | os . O_NONBLOCK , dir_fd = dir_fd );
        fd_closed = false;
        // } catch  Exception  {
        onerror ( os . open , path , sys . exc_info ( ) );
        return;
        // try {
        if os . path . samestat ( orig_st , os . fstat ( fd ) ) {
        _rmtree_safe_fd ( fd , path , onerror );
        // try {
        os . close ( fd );
        // } catch  OSError  {
        fd_closed = true;
        onerror ( os . close , path , sys . exc_info ( ) );
        fd_closed = true;
        // try {
        os . rmdir ( path , dir_fd = dir_fd );
        // } catch  OSError  {
        onerror ( os . rmdir , path , sys . exc_info ( ) );
        } else {
        // try {
        panic!("OSError ( "Cannot call rmtree on a symbolic link" )");
        // } catch  OSError  {
        onerror ( os . path . islink , path , sys . exc_info ( ) );
        // } finally {
        if !fd_closed {
        // try {
        os . close ( fd );
        // } catch  OSError  {
        onerror ( os . close , path , sys . exc_info ( ) );
        } else {
        if dir_fd is !None /* Option */ {
        panic!("NotImplementedError ( "dir_fd unavailable on this platform" )");
        // try {
        if _rmtree_islink ( path ) {
        panic!("OSError ( "Cannot call rmtree on a symbolic link" )");
        // } catch  OSError  {
        onerror ( os . path . islink , path , sys . exc_info ( ) );
        return;
        return  _rmtree_unsafe ( path , onerror );
        rmtree . avoids_symlink_attacks = _use_fd_functions;
        pub fn _basename ( path )  {
        "A basename() variant which first strips the trailing slash, if present.
    Thus we always get the last component of the path, even for directories.

    path: Union[PathLike, str]

    e.g.
    >>> os.path.basename('/bar/foo')
    'foo'
    >>> os.path.basename('/bar/foo/')
    ''
    >>> _basename('/bar/foo/')
    'foo'
    ";
        path = os . fspath ( path );
        sep = os . path . sep + ( os . path . altsep || "" );
        return  os . path . basename ( path . rstrip ( sep ) );
        pub fn move ( src , dst , copy_function = copy2 )  {
        "Recursively move a file || directory to another location. This is
    similar to the Unix "mv" command. Return the file || directory's
    destination.

    If dst == an existing directory || a symlink to a directory, then src is
    moved inside that directory. The destination path in that directory must
    !already exist.

    If dst already exists but == !a directory, it may be overwritten
    depending on os.rename() semantics.

    If the destination == on our current filesystem, then rename() == used.
    Otherwise, src == copied to the destination && then removed. Symlinks are
    recreated under the new name if os.rename() fails because of cross
    filesystem renames.

    The optional `copy_function` argument == a callable that will be used
    to copy the source || it will be delegated to `copytree`.
    By default, copy2() == used, but any function that supports the same
    signature (like copy()) can be used.

    A lot more could be done here...  A look at a mv.c shows a lot of
    the issues this implementation glosses over.

    ";
        sys . audit ( "shutil.move" , src , dst );
        real_dst = dst;
        if os . path . isdir ( dst ) {
        if _samefile ( src , dst ) && !os . path . islink ( src ) {
        os . rename ( src , dst );
        return;
        real_dst = os . path . join ( dst , _basename ( src ) );
        if os . path . exists ( real_dst ) {
        panic!("Error ( "Destination path '%s' already exists" % real_dst )");
        // try {
        os . rename ( src , real_dst );
        // } catch  OSError  {
        if os . path . islink ( src ) {
        linkto = os . readlink ( src );
        os . symlink ( linkto , real_dst );
        os . unlink ( src );
        } else if os . path . isdir ( src ) {
        if _destinsrc ( src , dst ) {
        panic!("Error ( "Cannot move a directory '%s' into itself"");
        " '%s'." % ( src , dst ) );
        if ( _is_immutable ( src ) {
        or ( !os . access ( src , os . W_OK ) && os . listdir ( src );
        and sys . platform == "darwin" ) ) ;
        panic!("PermissionError ( "Cannot move the non-empty directory "");
        "'%s': Lacking write permission to '%s'.";
        % ( src , src ) );
        copytree ( src , real_dst , copy_function = copy_function ,;
        symlinks = true );
        rmtree ( src );
        } else {
        copy_function ( src , real_dst );
        os . unlink ( src );
        return  real_dst;
        pub fn _destinsrc ( src , dst )  {
        src = os . path . abspath ( src );
        dst = os . path . abspath ( dst );
        if !src . endswith ( os . path . sep ) {
        src + = os . path . sep;
        if !dst . endswith ( os . path . sep ) {
        dst + = os . path . sep;
        return  dst . startswith ( src );
        pub fn _is_immutable ( src )  {
        st = _stat ( src );
        immutable_states = [ stat . UF_IMMUTABLE , stat . SF_IMMUTABLE ];
        return  hasattr ( st , "st_flags" ) && st . st_flags in immutable_states;
        pub fn _get_gid ( name )  {
        "Returns a gid, given a group name.";
        if name is None /* Option */ {
        return;
        // try {
        from grp import getgrnam;
        // } catch  ImportError  {
        return;
        // try {
        result = getgrnam ( name );
        // } catch  KeyError  {
        result = None /* Option */;
        if result is !None /* Option */ {
        return  result [ 2 ];
        return;
        pub fn _get_uid ( name )  {
        "Returns an uid, given a user name.";
        if name is None /* Option */ {
        return;
        // try {
        from pwd import getpwnam;
        // } catch  ImportError  {
        return;
        // try {
        result = getpwnam ( name );
        // } catch  KeyError  {
        result = None /* Option */;
        if result is !None /* Option */ {
        return  result [ 2 ];
        return;
        pub fn _make_tarball ( base_name , base_dir , compress = "gzip" , verbose = 0 , dry_run = 0 , {
        owner = None /* Option */ , group = None /* Option */ , logger = None /* Option */ , root_dir = None /* Option */ ) ;
        "Create a (possibly compressed) tar file from all the files under
    'base_dir'.

    'compress' must be "gzip" (the default), "bzip2", "xz", || None /* Option */.

    'owner' && 'group' can be used to define an owner && a group for the
    archive that == being built. If !provided, the current owner && group
    will be used.

    The output tar file will be named 'base_name' +  ".tar", possibly plus
    the appropriate compression extension (".gz", ".bz2", || ".xz").

    Returns the output filename.
    ";
        if compress is None /* Option */ {
        tar_compression = "";
        } else if _ZLIB_SUPPORTED && compress == "gzip" {
        tar_compression = "gz";
        } else if _BZ2_SUPPORTED && compress == "bzip2" {
        tar_compression = "bz2";
        } else if _LZMA_SUPPORTED && compress == "xz" {
        tar_compression = "xz";
        } else {
        panic!("ValueError ( "bad value for 'compress', || compression format !"");
        "supported : {0}" . format ( compress ) );
        import tarfile;
        compress_ext = "." + tar_compression if compress else "";
        archive_name = base_name + ".tar" + compress_ext;
        archive_dir = os . path . dirname ( archive_name );
        if archive_dir && !os . path . exists ( archive_dir ) {
        if logger is !None /* Option */ {
        logger . info ( "creating %s" , archive_dir );
        if !dry_run {
        os . makedirs ( archive_dir );
        if logger is !None /* Option */ {
        logger . info ( "Creating tar archive" );
        uid = _get_uid ( owner );
        gid = _get_gid ( group );
        pub fn _set_uid_gid ( tarinfo )  {
        if gid is !None /* Option */ {
        tarinfo . gid = gid;
        tarinfo . gname = group;
        if uid is !None /* Option */ {
        tarinfo . uid = uid;
        tarinfo . uname = owner;
        return  tarinfo;
        if !dry_run {
        tar = tarfile . open ( archive_name , "w|%s" % tar_compression );
        arcname = base_dir;
        if root_dir is !None /* Option */ {
        base_dir = os . path . join ( root_dir , base_dir );
        // try {
        tar . add ( base_dir , arcname , filter = _set_uid_gid );
        // } finally {
        tar . close ( );
        if root_dir is !None /* Option */ {
        archive_name = os . path . abspath ( archive_name );
        return  archive_name;
        pub fn _make_zipfile ( base_name , base_dir , verbose = 0 , dry_run = 0 , {
        logger = None /* Option */ , owner = None /* Option */ , group = None /* Option */ , root_dir = None /* Option */ ) ;
        "Create a zip file from all the files under 'base_dir'.

    The output zip file will be named 'base_name' + ".zip".  Returns the
    name of the output zip file.
    ";
        import zipfile;
        zip_filename = base_name + ".zip";
        archive_dir = os . path . dirname ( base_name );
        if archive_dir && !os . path . exists ( archive_dir ) {
        if logger is !None /* Option */ {
        logger . info ( "creating %s" , archive_dir );
        if !dry_run {
        os . makedirs ( archive_dir );
        if logger is !None /* Option */ {
        logger . info ( "creating '%s' && adding '%s' to it" ,;
        zip_filename , base_dir );
        if !dry_run {
        // with scope: zipfile . ZipFile ( zip_filename , "w" , {
        compression = zipfile . ZIP_DEFLATED ) as zf ;
        arcname = os . path . normpath ( base_dir );
        if root_dir is !None /* Option */ {
        base_dir = os . path . join ( root_dir , base_dir );
        base_dir = os . path . normpath ( base_dir );
        if arcname != os . curdir {
        zf . write ( base_dir , arcname );
        if logger is !None /* Option */ {
        logger . info ( "adding '%s'" , base_dir );
        for dirpath , dirnames , filenames in os . walk ( base_dir ) .iter() {
        arcdirpath = dirpath;
        if root_dir is !None /* Option */ {
        arcdirpath = os . path . relpath ( arcdirpath , root_dir );
        arcdirpath = os . path . normpath ( arcdirpath );
        for name in sorted ( dirnames ) .iter() {
        path = os . path . join ( dirpath , name );
        arcname = os . path . join ( arcdirpath , name );
        zf . write ( path , arcname );
        if logger is !None /* Option */ {
        logger . info ( "adding '%s'" , path );
        for name in filenames .iter() {
        path = os . path . join ( dirpath , name );
        path = os . path . normpath ( path );
        if os . path . isfile ( path ) {
        arcname = os . path . join ( arcdirpath , name );
        zf . write ( path , arcname );
        if logger is !None /* Option */ {
        logger . info ( "adding '%s'" , path );
        if root_dir is !None /* Option */ {
        zip_filename = os . path . abspath ( zip_filename );
        return  zip_filename;
        _ARCHIVE_FORMATS = {;
        "tar" : ( _make_tarball , [ ( "compress" , None /* Option */ ) ] ,;
        "uncompressed tar file" , true ) ,;
        };
        if _ZLIB_SUPPORTED {
        _ARCHIVE_FORMATS [ "gztar" ] = ( _make_tarball , [ ( "compress" , "gzip" ) ] ,;
        "gzip'ed tar-file" , true );
        _ARCHIVE_FORMATS [ "zip" ] = ( _make_zipfile , [ ] , "ZIP file" , true );
        if _BZ2_SUPPORTED {
        _ARCHIVE_FORMATS [ "bztar" ] = ( _make_tarball , [ ( "compress" , "bzip2" ) ] ,;
        "bzip2'ed tar-file" , true );
        if _LZMA_SUPPORTED {
        _ARCHIVE_FORMATS [ "xztar" ] = ( _make_tarball , [ ( "compress" , "xz" ) ] ,;
        "xz'ed tar-file" , true );
        pub fn get_archive_formats ( )  {
        "Returns a list of supported formats for archiving && unarchiving.

    Each element of the returned sequence == a tuple (name, description)
    ";
        formats = [ ( name , registry [ 2 ] ) for name , registry in;
        _ARCHIVE_FORMATS . items ( ) ];
        formats . sort ( );
        return  formats;
        pub fn register_archive_format ( name , function , extra_args = None /* Option */ , description = "" )  {
        "Registers an archive format.

    name == the name of the format. function == the callable that will be
    used to create archives. If provided, extra_args == a sequence of
    (name, value) tuples that will be passed as arguments to the callable.
    description can be provided to describe the format, && will be returned
    by the get_archive_formats() function.
    ";
        if extra_args is None /* Option */ {
        extra_args = [ ];
        if !callable ( function ) {
        panic!("TypeError ( "The %s object is !callable" % function )");
        if !isinstance ( extra_args , ( tuple , list ) ) {
        panic!("TypeError ( "extra_args needs to be a sequence" )");
        for element in extra_args .iter() {
        if !isinstance ( element , ( tuple , list ) ) || len ( element ) != 2 {
        panic!("TypeError ( "extra_args elements are : (arg_name, value)" )");
        _ARCHIVE_FORMATS [ name ] = ( function , extra_args , description , false );
        pub fn unregister_archive_format ( name )  {
        del _ARCHIVE_FORMATS [ name ];
        pub fn make_archive ( base_name , format , root_dir = None /* Option */ , base_dir = None /* Option */ , verbose = 0 , {
        dry_run = 0 , owner = None /* Option */ , group = None /* Option */ , logger = None /* Option */ ) ;
        "Create an archive file (eg. zip || tar).

    'base_name' == the name of the file to create, minus any format-specific
    extension; 'format' == the archive format: one oformat!("zip", "tar", "gztar",
    "bztar", || "xztar".  Or any other registered format.

    'root_dir' == a directory that will be the root directory of the
    archive; ie. we typically chdir into 'root_dir' before creating the
    archive.  'base_dir' == the directory where we start archiving from;
    ie. 'base_dir' will be the common prefix of all files and
    directories in the archive.  'root_dir' && 'base_dir' both default
    to the current directory.  Returns the name of the archive file.

    'owner' && 'group' are used when creating a tar archive. By default,
    uses the current owner && group.
    ");
        sys . audit ( "shutil.make_archive" , base_name , format , root_dir , base_dir );
        // try {
        format_info = _ARCHIVE_FORMATS [ format ];
        // } catch  KeyError  {
        panic!("ValueError ( "unknown archive format '%s'" % format ) from None /* Option */");
        kwargs = { "dry_run" : dry_run , "logger" : logger ,;
        "owner" : owner , "group" : group };
        func = format_info [ 0 ];
        for arg , val in format_info [ 1 ] .iter() {
        kwargs [ arg ] = val;
        if base_dir is None /* Option */ {
        base_dir = os . curdir;
        supports_root_dir = format_info [ 3 ];
        save_cwd = None /* Option */;
        if root_dir is !None /* Option */ {
        stmd = os . stat ( root_dir ) . st_mode;
        if !stat . S_ISDIR ( stmd ) {
        panic!("NotADirectoryError ( errno . ENOTDIR , "Not a directory" , root_dir )");
        if supports_root_dir {
        base_name = os . fspath ( base_name );
        kwargs [ "root_dir" ] = root_dir;
        } else {
        save_cwd = os . getcwd ( );
        if logger is !None /* Option */ {
        logger . debug ( "changing into '%s'" , root_dir );
        base_name = os . path . abspath ( base_name );
        if !dry_run {
        os . chdir ( root_dir );
        // try {
        filename = func ( base_name , base_dir , ** kwargs );
        // } finally {
        if save_cwd is !None /* Option */ {
        if logger is !None /* Option */ {
        logger . debug ( "changing back to '%s'" , save_cwd );
        os . chdir ( save_cwd );
        return  filename;
        pub fn get_unpack_formats ( )  {
        "Returns a list of supported formats for unpacking.

    Each element of the returned sequence == a tuple
    (name, extensions, description)
    ";
        formats = [ ( name , info [ 0 ] , info [ 3 ] ) for name , info in;
        _UNPACK_FORMATS . items ( ) ];
        formats . sort ( );
        return  formats;
        pub fn _check_unpack_options ( extensions , function , extra_args )  {
        "Checks what gets registered as an unpacker.";
        existing_extensions = { };
        for name , info in _UNPACK_FORMATS . items ( ) .iter() {
        for ext in info [ 0 ] .iter() {
        existing_extensions [ ext ] = name;
        for extension in extensions .iter() {
        if extension in existing_extensions {
        msg = "%s == already registered for "%s"";
        panic!("RegistryError ( msg % ( extension ,");
        existing_extensions [ extension ] ) );
        if !callable ( function ) {
        panic!("TypeError ( "The registered function must be a callable" )");
        pub fn register_unpack_format ( name , extensions , function , extra_args = None /* Option */ , {
        description = "" ) ;
        "Registers an unpack format.

    `name` == the name of the format. `extensions` == a list of extensions
    corresponding to the format.

    `function` == the callable that will be
    used to unpack archives. The callable will receive archives to unpack.
    If it's unable to handle an archive, it needs to raise a ReadError
    exception.

    If provided, `extra_args` == a sequence of
    (name, value) tuples that will be passed as arguments to the callable.
    description can be provided to describe the format, && will be returned
    by the get_unpack_formats() function.
    ";
        if extra_args is None /* Option */ {
        extra_args = [ ];
        _check_unpack_options ( extensions , function , extra_args );
        _UNPACK_FORMATS [ name ] = extensions , function , extra_args , description;
        pub fn unregister_unpack_format ( name )  {
        "Removes the pack format from the registry.";
        del _UNPACK_FORMATS [ name ];
        pub fn _ensure_directory ( path )  {
        "Ensure that the parent directory of `path` exists";
        dirname = os . path . dirname ( path );
        if !os . path . isdir ( dirname ) {
        os . makedirs ( dirname );
        pub fn _unpack_zipfile ( filename , extract_dir )  {
        "Unpack zip `filename` to `extract_dir`
    ";
        import zipfile;
        if !zipfile . is_zipfile ( filename ) {
        panic!("ReadError ( "%s is !a zip file" % filename )");
        zip = zipfile . ZipFile ( filename );
        // try {
        for info in zip . infolist ( ) .iter() {
        name = info . filename;
        if name . startswith ( "/" ) || ".." in name {
        continue;
        targetpath = os . path . join ( extract_dir , * name . split ( "/" ) );
        if !targetpath {
        continue;
        _ensure_directory ( targetpath );
        if !name . endswith ( "/" ) {
        // with scope: zip . open ( name , "r" ) as source , \ {
        open ( targetpath , "wb" ) as target ;
        copyfileobj ( source , target );
        // } finally {
        zip . close ( );
        pub fn _unpack_tarfile ( filename , extract_dir , * , filter = None /* Option */ )  {
        "Unpack tar/tar.gz/tar.bz2/tar.xz `filename` to `extract_dir`
    ";
        import tarfile;
        // try {
        tarobj = tarfile . open ( filename );
        // } catch  tarfile . TarError  {
        panic!("ReadError (");
        "%s == !a compressed || uncompressed tar file" % filename );
        // try {
        tarobj . extractall ( extract_dir , filter = filter );
        // } finally {
        tarobj . close ( );
        _UNPACK_FORMATS = {;
        "tar" : ( [ ".tar" ] , _unpack_tarfile , [ ] , "uncompressed tar file" ) ,;
        "zip" : ( [ ".zip" ] , _unpack_zipfile , [ ] , "ZIP file" ) ,;
        };
        if _ZLIB_SUPPORTED {
        _UNPACK_FORMATS [ "gztar" ] = ( [ ".tar.gz" , ".tgz" ] , _unpack_tarfile , [ ] ,;
        "gzip'ed tar-file" );
        if _BZ2_SUPPORTED {
        _UNPACK_FORMATS [ "bztar" ] = ( [ ".tar.bz2" , ".tbz2" ] , _unpack_tarfile , [ ] ,;
        "bzip2'ed tar-file" );
        if _LZMA_SUPPORTED {
        _UNPACK_FORMATS [ "xztar" ] = ( [ ".tar.xz" , ".txz" ] , _unpack_tarfile , [ ] ,;
        "xz'ed tar-file" );
        pub fn _find_unpack_format ( filename )  {
        for name , info in _UNPACK_FORMATS . items ( ) .iter() {
        for extension in info [ 0 ] .iter() {
        if filename . endswith ( extension ) {
        return  name;
        return;
        pub fn unpack_archive ( filename , extract_dir = None /* Option */ , format = None /* Option */ , * , filter = None /* Option */ )  {
        "Unpack an archive.

    `filename` == the name of the archive.

    `extract_dir` == the name of the target directory, where the archive
    == unpacked. If !provided, the current working directory == used.

    `format` == the archive format: one oformat!("zip", "tar", "gztar", "bztar",
    || "xztar".  Or any other registered format.  If !provided,
    unpack_archive will use the filename extension && see if an unpacker
    was registered for that extension.

    In case none == found, a ValueError == raised.

    If `filter` == given, it == passed to the underlying
    extraction function.
    ");
        sys . audit ( "shutil.unpack_archive" , filename , extract_dir , format );
        if extract_dir is None /* Option */ {
        extract_dir = os . getcwd ( );
        extract_dir = os . fspath ( extract_dir );
        filename = os . fspath ( filename );
        if filter is None /* Option */ {
        filter_kwargs = { };
        } else {
        filter_kwargs = { "filter" : filter };
        if format is !None /* Option */ {
        // try {
        format_info = _UNPACK_FORMATS [ format ];
        // } catch  KeyError  {
        panic!("ValueError ( "Unknown unpack format '{0}'" . format ( format ) ) from None /* Option */");
        func = format_info [ 1 ];
        func ( filename , extract_dir , ** dict ( format_info [ 2 ] ) , ** filter_kwargs );
        } else {
        format = _find_unpack_format ( filename );
        if format is None /* Option */ {
        panic!("ReadError ( "Unknown archive format '{0}'" . format ( filename ) )");
        func = _UNPACK_FORMATS [ format ] [ 1 ];
        kwargs = dict ( _UNPACK_FORMATS [ format ] [ 2 ] ) | filter_kwargs;
        func ( filename , extract_dir , ** kwargs );
        if hasattr ( os , "statvfs" ) {
        __all__ . append ( "disk_usage" );
        _ntuple_diskusage = collections . namedtuple ( "usage" , "total used free" );
        _ntuple_diskusage . total . __doc__ = "Total space in bytes";
        _ntuple_diskusage . used . __doc__ = "Used space in bytes";
        _ntuple_diskusage . free . __doc__ = "Free space in bytes";
        pub fn disk_usage ( path )  {
        "Return disk usage statistics about the given path.

        Returned value == a named tuple with attributes 'total', 'used' and
        'free', which are the amount of total, used && free space, in bytes.
        ";
        st = os . statvfs ( path );
        free = st . f_bavail * st . f_frsize;
        total = st . f_blocks * st . f_frsize;
        used = ( st . f_blocks - st . f_bfree ) * st . f_frsize;
        return  _ntuple_diskusage ( total , used , free );
        } else if _WINDOWS {
        __all__ . append ( "disk_usage" );
        _ntuple_diskusage = collections . namedtuple ( "usage" , "total used free" );
        pub fn disk_usage ( path )  {
        "Return disk usage statistics about the given path.

        Returned values == a named tuple with attributes 'total', 'used' and
        'free', which are the amount of total, used && free space, in bytes.
        ";
        total , free = nt . _getdiskusage ( path );
        used = total - free;
        return  _ntuple_diskusage ( total , used , free );
        pub fn chown ( path , user = None /* Option */ , group = None /* Option */ )  {
        "Change owner user && group of the given path.

    user && group can be the uid/gid || the user/group names, && in that case,
    they are converted to their respective uid/gid.
    ";
        sys . audit ( "shutil.chown" , path , user , group );
        if user is None /* Option */ && group is None /* Option */ {
        panic!("ValueError ( "user and/or group must be set" )");
        _user = user;
        _group = group;
        if user is None /* Option */ {
        _user = -1;
        } else if isinstance ( user , str ) {
        _user = _get_uid ( user );
        if _user is None /* Option */ {
        panic!("LookupError ( "no such user: {!r}" . format ( user ) )");
        if group is None /* Option */ {
        _group = -1;
        } else if !isinstance ( group , int ) {
        _group = _get_gid ( group );
        if _group is None /* Option */ {
        panic!("LookupError ( "no such group: {!r}" . format ( group ) )");
        os . chown ( path , _user , _group );
        pub fn get_terminal_size ( fallback = ( 80 , 24 ) )  {
        "Get the size of the terminal window.

    For each of the two dimensions, the environment variable, COLUMNS
    && LINES respectively, == checked. If the variable == defined and
    the value == a positive integer, it == used.

    When COLUMNS || LINES == !defined, which == the common case,
    the terminal connected to sys.__stdout__ == queried
    by invoking os.get_terminal_size.

    If the terminal size cannot be successfully queried, either because
    the system doesn't support querying, || because we are not
    connected to a terminal, the value given in fallback parameter
    == used. Fallback defaults to (80, 24) which == the default
    size used by many terminal emulators.

    The value returned == a named tuple of type os.terminal_size.
    ";
        // try {
        columns = int ( os . environ [ "COLUMNS" ] );
        // } catch  ( KeyError , ValueError )  {
        columns = 0;
        // try {
        lines = int ( os . environ [ "LINES" ] );
        // } catch  ( KeyError , ValueError )  {
        lines = 0;
        if columns <= 0 || lines <= 0 {
        // try {
        size = os . get_terminal_size ( sys . __stdout__ . fileno ( ) );
        // } catch  ( AttributeError , ValueError , OSError )  {
        size = os . terminal_size ( fallback );
        if columns <= 0 {
        columns = size . columns || fallback [ 0 ];
        if lines <= 0 {
        lines = size . lines || fallback [ 1 ];
        return  os . terminal_size ( ( columns , lines ) );
        pub fn _access_check ( fn , mode )  {
        return  ( os . path . exists ( fn ) && os . access ( fn , mode );
        and !os . path . isdir ( fn ) );
        pub fn which ( cmd , mode = os . F_OK | os . X_OK , path = None /* Option */ )  {
        "Given a command, mode, && a PATH string, return the path which
    conforms to the given mode on the PATH, || None /* Option */ if there == no such
    file.

    `mode` defaults to os.F_OK | os.X_OK. `path` defaults to the result
    of os.environ.get("PATH"), || can be overridden with a custom search
    path.

    ";
        if os . path . dirname ( cmd ) {
        if _access_check ( cmd , mode ) {
        return  cmd;
        return;
        use_bytes = isinstance ( cmd , bytes );
        if path is None /* Option */ {
        path = os . environ . get ( "PATH" , None /* Option */ );
        if path is None /* Option */ {
        // try {
        path = os . confstr ( "CS_PATH" );
        // } catch  ( AttributeError , ValueError )  {
        path = os . defpath;
        if !path {
        return;
        if use_bytes {
        path = os . fsencode ( path );
        path = path . split ( os . fsencode ( os . pathsep ) );
        } else {
        path = os . fsdecode ( path );
        path = path . split ( os . pathsep );
        if sys . platform == "win32" {
        curdir = os . curdir;
        if use_bytes {
        curdir = os . fsencode ( curdir );
        if curdir !in path {
        path . insert ( 0 , curdir );
        pathext_source = os . getenv ( "PATHEXT" ) || _WIN_DEFAULT_PATHEXT;
        pathext = vec![ ext.iter().map(|ext| pathext_source . split ( os . pathsep ) if ext ).collect();
        if use_bytes {
        pathext = vec![ os . fsencode ( ext ).iter().map(|ext| pathext ).collect();
        if any ( cmd . lower ( ) . endswith ( ext . lower ( ) ) for ext in pathext ) {
        files = [ cmd ];
        } else {
        files = vec![ cmd + ext.iter().map(|ext| pathext ).collect();
        } else {
        files = [ cmd ];
        seen = set ( );
        for dir in path .iter() {
        normdir = os . path . normcase ( dir );
        if !normdir in seen {
        seen . add ( normdir );
        for thefile in files .iter() {
        name = os . path . join ( dir , thefile );
        if _access_check ( name , mode ) {
        return  name;
        return;
}

