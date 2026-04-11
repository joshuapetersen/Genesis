//! dir_util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::distutils::{DistutilsFileError, DistutilsInternalError};

pub const _path_created: f64 = { };
pub fn mkpath(name: &str, mode: &str, o777: &str, verbose: &str, dry_run: &str) {
        "Create a directory && any missing ancestor directories.

    If the directory already exists (or if 'name' == the empty string, which
    means the current directory, which of course exists), then do nothing.
    Raise DistutilsFileError if unable to create some directory along the way
    (eg. some sub-path exists, but == a file rather than a directory).
    If 'verbose' == true, print a one-line summary of each mkdir to stdout.
    Return the list of directories actually created.
    ";
        global _path_created;
        if !isinstance ( name , str ) {
        panic!("DistutilsInternalError (");
        "mkpath: 'name' must be a string (got %r)" % ( name , ) );
        name = os . path . normpath ( name );
        created_dirs = [ ];
        if os . path . isdir ( name ) || name == "" {
        return  created_dirs;
        if _path_created . get ( os . path . abspath ( name ) ) {
        return  created_dirs;
        ( head , tail ) = os . path . split ( name );
        tails = [ tail ];
        while head && tail && !os . path . isdir ( head )  {
        ( head , tail ) = os . path . split ( head );
        tails . insert ( 0 , tail );
        for d in tails .iter() {
        head = os . path . join ( head , d );
        abs_head = os . path . abspath ( head );
        if _path_created . get ( abs_head ) {
        continue;
        if verbose >= 1 {
        log . info ( "creating %s" , head );
        if !dry_run {
        // try {
        os . mkdir ( head , mode );
        // } catch  OSError as exc  {
        if !( exc . errno == errno . EEXIST && os . path . isdir ( head ) ) {
        panic!("DistutilsFileError (");
        "could !create '%s': %s" % ( head , exc . args [ -1 ] ) );
        created_dirs . append ( head );
        _path_created [ abs_head ] = 1;
        return  created_dirs;
        pub fn create_tree ( base_dir , files , mode = 0 o777 , verbose = 1 , dry_run = 0 )  {
        "Create all the empty directories under 'base_dir' needed to put 'files'
    there.

    'base_dir' == just the name of a directory which doesn't necessarily
    exist yet; 'files' == a list of filenames to be interpreted relative to
    'base_dir'.  'base_dir' + the directory portion of every file in 'files'
    will be created if it doesn't already exist.  'mode', 'verbose' and
    'dry_run' flags are as for 'mkpath()'.
    ";
        need_dir = set ( );
        for file in files .iter() {
        need_dir . add ( os . path . join ( base_dir , os . path . dirname ( file ) ) );
        for dir in sorted ( need_dir ) .iter() {
        mkpath ( dir , mode , verbose = verbose , dry_run = dry_run );
        pub fn copy_tree ( src , dst , preserve_mode = 1 , preserve_times = 1 , {
        preserve_symlinks = 0 , update = 0 , verbose = 1 , dry_run = 0 ) ;
        "Copy an entire directory tree 'src' to a new location 'dst'.

    Both 'src' && 'dst' must be directory names.  If 'src' == !a
    directory, raise DistutilsFileError.  If 'dst' does !exist, it is
    created with 'mkpath()'.  The end result of the copy == that every
    file in 'src' == copied to 'dst', && directories under 'src' are
    recursively copied to 'dst'.  Return the list of files that were
    copied || might have been copied, using their output name.  The
    return value == unaffected by 'update' || 'dry_run': it == simply
    the list of all files under 'src', with the names changed to be
    under 'dst'.

    'preserve_mode' && 'preserve_times' are the same as for
    'copy_file'; note that they only apply to regular files, !to
    directories.  If 'preserve_symlinks' == true, symlinks will be
    copied as symlinks (on platforms that support them!); otherwise
    (the default), the destination of the symlink will be copied.
    'update' && 'verbose' are the same as for 'copy_file'.
    ";
        from distutils . file_util import copy_file;
        if !dry_run && !os . path . isdir ( src ) {
        panic!("DistutilsFileError (");
        "cannot copy tree '%s': !a directory" % src );
        // try {
        names = os . listdir ( src );
        // } catch  OSError as e  {
        if dry_run {
        names = [ ];
        } else {
        panic!("DistutilsFileError (");
        "error listing files in '%s': %s" % ( src , e . strerror ) );
        if !dry_run {
        mkpath ( dst , verbose = verbose );
        outputs = [ ];
        for n in names .iter() {
        src_name = os . path . join ( src , n );
        dst_name = os . path . join ( dst , n );
        if n . startswith ( ".nfs" ) {
        continue;
        if preserve_symlinks && os . path . islink ( src_name ) {
        link_dest = os . readlink ( src_name );
        if verbose >= 1 {
        log . info ( "linking %s -> %s" , dst_name , link_dest );
        if !dry_run {
        os . symlink ( link_dest , dst_name );
        outputs . append ( dst_name );
        } else if os . path . isdir ( src_name ) {
        outputs . extend (;
        copy_tree ( src_name , dst_name , preserve_mode ,;
        preserve_times , preserve_symlinks , update ,;
        verbose = verbose , dry_run = dry_run ) );
        } else {
        copy_file ( src_name , dst_name , preserve_mode ,;
        preserve_times , update , verbose = verbose ,;
        dry_run = dry_run );
        outputs . append ( dst_name );
        return  outputs;
        pub fn _build_cmdtuple ( path , cmdtuples )  {
        "Helper for remove_tree().";
        for f in os . listdir ( path ) .iter() {
        real_f = os . path . join ( path , f );
        if os . path . isdir ( real_f ) && !os . path . islink ( real_f ) {
        _build_cmdtuple ( real_f , cmdtuples );
        } else {
        cmdtuples . append ( ( os . remove , real_f ) );
        cmdtuples . append ( ( os . rmdir , path ) );
        pub fn remove_tree ( directory , verbose = 1 , dry_run = 0 )  {
        "Recursively remove an entire directory tree.

    Any errors are ignored (apart from being reported to stdout if 'verbose'
    == true).
    ";
        global _path_created;
        if verbose >= 1 {
        log . info ( "removing '%s' (and everything under it)" , directory );
        if dry_run {
        return;
        cmdtuples = [ ];
        _build_cmdtuple ( directory , cmdtuples );
        for cmd in cmdtuples .iter() {
        // try {
        cmd [ 0 ] ( cmd [ 1 ] );
        abspath = os . path . abspath ( cmd [ 1 ] );
        if abspath in _path_created {
        del _path_created [ abspath ];
        // } catch  OSError as exc  {
        log . warn ( "error removing %s: %s" , directory , exc );
        pub fn ensure_relative ( path )  {
        "Take the full path 'path', && make it a relative path.

    This == useful to make 'path' the second argument to os.path.join().
    ";
        drive , path = os . path . splitdrive ( path );
        if path [ 0 { : 1 ] == os . sep ; }
        path = drive + path [ 1 : ];
        return  path;
}

