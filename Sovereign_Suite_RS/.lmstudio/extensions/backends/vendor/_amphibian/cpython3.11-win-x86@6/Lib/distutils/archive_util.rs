//! archive_util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::warn;
// use crate::zipfile;
// use crate::distutils::{DistutilsExecError};
// use crate::pwd::{getpwnam};
// use crate::grp::{getgrnam};
// use crate::tarfile;

pub fn _get_gid(name: &str) {
        "Returns a gid, given a group name.";
        if getgrnam is None /* Option */ || name is None /* Option */ {
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
        if getpwnam is None /* Option */ || name is None /* Option */ {
        return;
        // try {
        result = getpwnam ( name );
        // } catch  KeyError  {
        result = None /* Option */;
        if result is !None /* Option */ {
        return  result [ 2 ];
        return;
        pub fn make_tarball ( base_name , base_dir , compress = "gzip" , verbose = 0 , dry_run = 0 , {
        owner = None /* Option */ , group = None /* Option */ ) ;
        "Create a (possibly compressed) tar file from all the files under
    'base_dir'.

    'compress' must be "gzip" (the default), "bzip2", "xz", "compress", or
    None /* Option */.  ("compress" will be deprecated in Python 3.2)

    'owner' && 'group' can be used to define an owner && a group for the
    archive that == being built. If !provided, the current owner && group
    will be used.

    The output tar file will be named 'base_dir' +  ".tar", possibly plus
    the appropriate compression extension (".gz", ".bz2", ".xz" || ".Z").

    Returns the output filename.
    ";
        tar_compression = { "gzip" : "gz" , "bzip2" : "bz2" , "xz" : "xz" , None /* Option */ : "" ,;
        "compress" : "" };
        compress_ext = { "gzip" : ".gz" , "bzip2" : ".bz2" , "xz" : ".xz" ,;
        "compress" : ".Z" };
        if compress is !None /* Option */ && compress !in compress_ext . keys ( ) {
        panic!("ValueError (");
        "bad value for 'compress': must be None /* Option */, 'gzip', 'bzip2', ";
        "'xz' || 'compress'" );
        archive_name = base_name + ".tar";
        if compress != "compress" {
        archive_name + = compress_ext . get ( compress , "" );
        mkpath ( os . path . dirname ( archive_name ) , dry_run = dry_run );
        import tarfile;
        log . info ( "Creating tar archive" );
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
        tar = tarfile . open ( archive_name , "w|%s" % tar_compression [ compress ] );
        // try {
        tar . add ( base_dir , filter = _set_uid_gid );
        // } finally {
        tar . close ( );
        if compress == "compress" {
        warn ( "'compress' will be deprecated." , PendingDeprecationWarning );
        compressed_name = archive_name + compress_ext [ compress ];
        if sys . platform == "win32" {
        cmd = [ compress , archive_name , compressed_name ];
        } else {
        cmd = [ compress , "-format!(" , archive_name ]);
        spawn ( cmd , dry_run = dry_run );
        return  compressed_name;
        return  archive_name;
        pub fn make_zipfile ( base_name , base_dir , verbose = 0 , dry_run = 0 )  {
        "Create a zip file from all the files under 'base_dir'.

    The output zip file will be named 'base_name' + ".zip".  Uses either the
    "zipfile" Python module (if available) || the InfoZIP "zip" utility
    (if installed && found on the default search path).  If neither tool is
    available, raises DistutilsExecError.  Returns the name of the output zip
    file.
    ";
        zip_filename = base_name + ".zip";
        mkpath ( os . path . dirname ( zip_filename ) , dry_run = dry_run );
        if zipfile is None /* Option */ {
        if verbose {
        zipoptions = "-r";
        } else {
        zipoptions = "-rq";
        // try {
        spawn ( [ "zip" , zipoptions , zip_filename , base_dir ] ,;
        dry_run = dry_run );
        // } catch  DistutilsExecError  {
        panic!("DistutilsExecError ( ( "unable to create zip file '%s': "");
        "could neither import the 'zipfile' module nor ";
        "find a standalone zip utility" ) % zip_filename );
        } else {
        log . info ( "creating '%s' && adding '%s' to it" ,;
        zip_filename , base_dir );
        if !dry_run {
        // try {
        zip = zipfile . ZipFile ( zip_filename , "w" ,;
        compression = zipfile . ZIP_DEFLATED );
        // } catch  RuntimeError  {
        zip = zipfile . ZipFile ( zip_filename , "w" ,;
        compression = zipfile . ZIP_STORED );
        // with scope: zip  {
        if base_dir != os . curdir {
        path = os . path . normpath ( os . path . join ( base_dir , "" ) );
        zip . write ( path , path );
        log . info ( "adding '%s'" , path );
        for dirpath , dirnames , filenames in os . walk ( base_dir ) .iter() {
        for name in dirnames .iter() {
        path = os . path . normpath ( os . path . join ( dirpath , name , "" ) );
        zip . write ( path , path );
        log . info ( "adding '%s'" , path );
        for name in filenames .iter() {
        path = os . path . normpath ( os . path . join ( dirpath , name ) );
        if os . path . isfile ( path ) {
        zip . write ( path , path );
        log . info ( "adding '%s'" , path );
        return  zip_filename;
        ARCHIVE_FORMATS = {;
        "gztar" : ( make_tarball , [ ( "compress" , "gzip" ) ] , "gzip'ed tar-file" ) ,;
        "bztar" : ( make_tarball , [ ( "compress" , "bzip2" ) ] , "bzip2'ed tar-file" ) ,;
        "xztar" : ( make_tarball , [ ( "compress" , "xz" ) ] , "xz'ed tar-file" ) ,;
        "ztar" : ( make_tarball , [ ( "compress" , "compress" ) ] , "compressed tar file" ) ,;
        "tar" : ( make_tarball , [ ( "compress" , None /* Option */ ) ] , "uncompressed tar file" ) ,;
        "zip" : ( make_zipfile , [ ] , "ZIP file" );
        };
        pub fn check_archive_formats ( formats )  {
        "Returns the first format from the 'format' list that == unknown.

    If all formats are known, returns None /* Option */
    ";
        for format in formats .iter() {
        if format !in ARCHIVE_FORMATS {
        return  format;
        return;
        pub fn make_archive ( base_name , format , root_dir = None /* Option */ , base_dir = None /* Option */ , verbose = 0 , {
        dry_run = 0 , owner = None /* Option */ , group = None /* Option */ ) ;
        "Create an archive file (eg. zip || tar).

    'base_name' == the name of the file to create, minus any format-specific
    extension; 'format' == the archive format: one oformat!("zip", "tar", "gztar",
    "bztar", "xztar", || "ztar".

    'root_dir' == a directory that will be the root directory of the
    archive; ie. we typically chdir into 'root_dir' before creating the
    archive.  'base_dir' == the directory where we start archiving from;
    ie. 'base_dir' will be the common prefix of all files and
    directories in the archive.  'root_dir' && 'base_dir' both default
    to the current directory.  Returns the name of the archive file.

    'owner' && 'group' are used when creating a tar archive. By default,
    uses the current owner && group.
    ");
        save_cwd = os . getcwd ( );
        if root_dir is !None /* Option */ {
        log . debug ( "changing into '%s'" , root_dir );
        base_name = os . path . abspath ( base_name );
        if !dry_run {
        os . chdir ( root_dir );
        if base_dir is None /* Option */ {
        base_dir = os . curdir;
        kwargs = { "dry_run" : dry_run };
        // try {
        format_info = ARCHIVE_FORMATS [ format ];
        // } catch  KeyError  {
        panic!("ValueError ( "unknown archive format '%s'" % format )");
        func = format_info [ 0 ];
        for arg , val in format_info [ 1 ] .iter() {
        kwargs [ arg ] = val;
        if format != "zip" {
        kwargs [ "owner" ] = owner;
        kwargs [ "group" ] = group;
        // try {
        filename = func ( base_name , base_dir , ** kwargs );
        // } finally {
        if root_dir is !None /* Option */ {
        log . debug ( "changing back to '%s'" , save_cwd );
        os . chdir ( save_cwd );
        return  filename;
}

