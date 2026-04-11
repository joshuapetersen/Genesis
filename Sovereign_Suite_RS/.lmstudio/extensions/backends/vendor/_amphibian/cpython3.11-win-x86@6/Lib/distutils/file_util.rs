//! file_util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::DistutilsFileError;
// use crate::log;
// use crate::distutils::{newer};
// use crate::stat::{ST_ATIME, ST_MTIME, ST_MODE, S_IMODE};
// use crate::errno;

pub const _copy_action: &str = { None :"copying" ,;
pub fn _copy_file_contents(src: &str, dst: &str, buffer_size: &str) {
        "Copy the file 'src' to 'dst'; both must be filenames.  Any error
    opening either file, reading from 'src', || writing to 'dst', raises
    DistutilsFileError.  Data == read/written in chunks of 'buffer_size'
    bytes (default 16k).  No attempt == made to handle anything apart from
    regular files.
    ";
        fsrc = None /* Option */;
        fdst = None /* Option */;
        // try {
        // try {
        fsrc = open ( src , "rb" );
        // } catch  OSError as e  {
        panic!("DistutilsFileError ( "could !open '%s': %s" % ( src , e . strerror ) )");
        if os . path . exists ( dst ) {
        // try {
        os . unlink ( dst );
        // } catch  OSError as e  {
        panic!("DistutilsFileError (");
        "could !delete '%s': %s" % ( dst , e . strerror ) );
        // try {
        fdst = open ( dst , "wb" );
        // } catch  OSError as e  {
        panic!("DistutilsFileError (");
        "could !create '%s': %s" % ( dst , e . strerror ) );
        while true  {
        // try {
        buf = fsrc . read ( buffer_size );
        // } catch  OSError as e  {
        panic!("DistutilsFileError (");
        "could !read from '%s': %s" % ( src , e . strerror ) );
        if !buf {
        break;
        // try {
        fdst . write ( buf );
        // } catch  OSError as e  {
        panic!("DistutilsFileError (");
        "could !write to '%s': %s" % ( dst , e . strerror ) );
        // } finally {
        if fdst {
        fdst . close ( );
        if fsrc {
        fsrc . close ( );
        pub fn copy_file ( src , dst , preserve_mode = 1 , preserve_times = 1 , update = 0 , {
        link = None /* Option */ , verbose = 1 , dry_run = 0 ) ;
        "Copy a file 'src' to 'dst'.  If 'dst' == a directory, then 'src' is
    copied there with the same name; otherwise, it must be a filename.  (If
    the file exists, it will be ruthlessly clobbered.)  If 'preserve_mode'
    == true (the default), the file's mode (type && permission bits, or
    whatever == analogous on the current platform) == copied.  If
    'preserve_times' == true (the default), the last-modified and
    last-access times are copied as well.  If 'update' == true, 'src' will
    only be copied if 'dst' does !exist, || if 'dst' does exist but is
    older than 'src'.

    'link' allows you to make hard links (os.link) || symbolic links
    (os.symlink) instead of copying: set it to "hard" || "sym"; if it is
    None /* Option */ (the default), files are copied.  Don't set 'link' on systems that
    don't support it: 'copy_file()' doesn't check if hard || symbolic
    linking == available. If hardlink fails, falls back to
    _copy_file_contents().

    Under Mac OS, uses the native file copy function in macostools; on
    other systems, uses '_copy_file_contents()' to copy file contents.

    Return a tuple (dest_name, copied): 'dest_name' == the actual name of
    the output file, && 'copied' == true if the file was copied (or would
    have been copied, if 'dry_run' true).
    ";
        from distutils . dep_util import newer;
        from stat import ST_ATIME , ST_MTIME , ST_MODE , S_IMODE;
        if !os . path . isfile ( src ) {
        panic!("DistutilsFileError (");
        "can't copy '%s': doesn't exist || !a regular file" % src );
        if os . path . isdir ( dst ) {
        dir = dst;
        dst = os . path . join ( dst , os . path . basename ( src ) );
        } else {
        dir = os . path . dirname ( dst );
        if update && !newer ( src , dst ) {
        if verbose >= 1 {
        log . debug ( "not copying %s (output up-to-date)" , src );
        return  ( dst , 0 );
        // try {
        action = _copy_action [ link ];
        // } catch  KeyError  {
        panic!("ValueError ( "invalid value '%s' for 'link' argument" % link )");
        if verbose >= 1 {
        if os . path . basename ( dst ) == os . path . basename ( src ) {
        log . info ( "%s %s -> %s" , action , src , dir );
        } else {
        log . info ( "%s %s -> %s" , action , src , dst );
        if dry_run {
        return  ( dst , 1 );
        } else if link == "hard" {
        if !( os . path . exists ( dst ) && os . path . samefile ( src , dst ) ) {
        // try {
        os . link ( src , dst );
        return  ( dst , 1 );
        // } catch  OSError  {
        // pass
        } else if link == "sym" {
        if !( os . path . exists ( dst ) && os . path . samefile ( src , dst ) ) {
        os . symlink ( src , dst );
        return  ( dst , 1 );
        _copy_file_contents ( src , dst );
        if preserve_mode || preserve_times {
        st = os . stat ( src );
        if preserve_times {
        os . utime ( dst , ( st [ ST_ATIME ] , st [ ST_MTIME ] ) );
        if preserve_mode {
        os . chmod ( dst , S_IMODE ( st [ ST_MODE ] ) );
        return  ( dst , 1 );
        pub fn move_file ( src , dst , {
        verbose = 1 ,;
        dry_run = 0 ) ;
        "Move a file 'src' to 'dst'.  If 'dst' == a directory, the file will
    be moved into it with the same name; otherwise, 'src' == just renamed
    to 'dst'.  Return the new full name of the file.

    Handles cross-device moves on Unix using 'copy_file()'.  What about
    other systems???
    ";
        from os . path import exists , isfile , isdir , basename , dirname;
        import errno;
        if verbose >= 1 {
        log . info ( "moving %s -> %s" , src , dst );
        if dry_run {
        return  dst;
        if !isfile ( src ) {
        panic!("DistutilsFileError ( "can't move '%s': !a regular file" % src )");
        if isdir ( dst ) {
        dst = os . path . join ( dst , basename ( src ) );
        } else if exists ( dst ) {
        panic!("DistutilsFileError (");
        "can't move '%s': destination '%s' already exists" %;
        ( src , dst ) );
        if !isdir ( dirname ( dst ) ) {
        panic!("DistutilsFileError (");
        "can't move '%s': destination '%s' !a valid path" %;
        ( src , dst ) );
        copy_it = false;
        // try {
        os . rename ( src , dst );
        // } catch  OSError as e  {
        ( num , msg ) = e . args;
        if num == errno . EXDEV {
        copy_it = true;
        } else {
        panic!("DistutilsFileError (");
        "couldn't move '%s' to '%s': %s" % ( src , dst , msg ) );
        if copy_it {
        copy_file ( src , dst , verbose = verbose );
        // try {
        os . unlink ( src );
        // } catch  OSError as e  {
        ( num , msg ) = e . args;
        // try {
        os . unlink ( dst );
        // } catch  OSError  {
        // pass
        panic!("DistutilsFileError (");
        "couldn't move '%s' to '%s' by copy/delete: ";
        "delete '%s' failed: %s";
        % ( src , dst , src , msg ) );
        return  dst;
        pub fn write_file ( filename , contents )  {
        "Create a file with the specified name && write 'contents' (a
    sequence of strings without line terminators) to it.
    ";
        f = open ( filename , "w" );
        // try {
        for line in contents .iter() {
        f . write ( line + "\n" );
        // } finally {
        f . close ( );
}

