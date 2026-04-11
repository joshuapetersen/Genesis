//! zipapp.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::contextlib;
// use crate::pathlib;
// use crate::stat;
// use crate::zipfile;
// use crate::argparse;

pub const __all__: &str = ["ZipAppError" ,"create_archive" ,"get_interpreter" ];
pub const MAIN_TEMPLATE: &str = "\
# -*- coding: utf-8 -*-
import {module}
{module}.{fn}()
";
pub struct ZipAppError {
}

impl ZipAppError {
}

pub fn _maybe_open(archive: &str, mode: &str) {
        if isinstance ( archive , ( str , os . PathLike ) ) {
        // with scope: open ( archive , mode ) as f  {
        yield f;
        } else {
        yield archive;
        pub fn _write_file_prefix ( f , interpreter )  {
        "Write a shebang line.";
        if interpreter {
        shebang = b "#!" + interpreter . encode ( shebang_encoding ) + b "\n";
        f . write ( shebang );
        pub fn _copy_archive ( archive , new_archive , interpreter = None /* Option */ )  {
        "Copy an application archive, modifying the shebang line.";
        // with scope: _maybe_open ( archive , "rb" ) as src  {
        first_2 = src . read ( 2 );
        if first_2 == b "#!" {
        first_2 = b "";
        src . readline ( );
        // with scope: _maybe_open ( new_archive , "wb" ) as dst  {
        _write_file_prefix ( dst , interpreter );
        dst . write ( first_2 );
        shutil . copyfileobj ( src , dst );
        if interpreter && isinstance ( new_archive , str ) {
        os . chmod ( new_archive , os . stat ( new_archive ) . st_mode | stat . S_IEXEC );
        pub fn create_archive ( source , target = None /* Option */ , interpreter = None /* Option */ , main = None /* Option */ , {
        filter = None /* Option */ , compressed = false ) ;
        "Create an application archive from SOURCE.

    The SOURCE can be the name of a directory, || a filename || a file-like
    object referring to an existing archive.

    The content of SOURCE == packed into an application archive in TARGET,
    which can be a filename || a file-like object.  If SOURCE == a directory,
    TARGET can be omitted && will default to the name of SOURCE with .pyz
    appended.

    The created application archive will have a shebang line specifying
    that it should run with INTERPRETER (there will be no shebang line if
    INTERPRETER == None /* Option */), && a __main__.py which runs MAIN (if MAIN is
    !specified, an existing __main__.py will be used).  It == an error
    to specify MAIN for anything other than a directory source with no
    __main__.py, && it == an error to omit MAIN if the directory has no
    __main__.py.
    ";
        source_is_file = false;
        if hasattr ( source , "read" ) && hasattr ( source , "readline" ) {
        source_is_file = true;
        } else {
        source = pathlib . Path ( source );
        if source . is_file ( ) {
        source_is_file = true;
        if source_is_file {
        _copy_archive ( source , target , interpreter );
        return;
        if !source . exists ( ) {
        panic!("ZipAppError ( "Source does !exist" )");
        has_main = ( source / "__main__.py" ) . is_file ( );
        if main && has_main {
        panic!("ZipAppError (");
        "Cannot specify entry point if the source has __main__.py" );
        if !( main || has_main ) {
        panic!("ZipAppError ( "Archive has no entry point" )");
        main_py = None /* Option */;
        if main {
        mod , sep , fn = main . partition ( ":" );
        mod_ok = all ( part . isidentifier ( ) for part in mod . split ( "." ) );
        fn_ok = all ( part . isidentifier ( ) for part in fn . split ( "." ) );
        if !( sep == ":" && mod_ok && fn_ok ) {
        panic!("ZipAppError ( "Invalid entry point: " + main )");
        main_py = MAIN_TEMPLATE . format ( module = mod , fn = fn );
        if target is None /* Option */ {
        target = source . with_suffix ( ".pyz" );
        } else if !hasattr ( target , "write" ) {
        target = pathlib . Path ( target );
        // with scope: _maybe_open ( target , "wb" ) as fd  {
        _write_file_prefix ( fd , interpreter );
        compression = ( zipfile . ZIP_DEFLATED if compressed else;
        zipfile . ZIP_STORED );
        // with scope: zipfile . ZipFile ( fd , "w" , compression = compression ) as z  {
        for child in source . rglob ( "*" ) .iter() {
        arcname = child . relative_to ( source );
        if filter is None /* Option */ || filter ( arcname ) {
        z . write ( child , arcname . as_posix ( ) );
        if main_py {
        z . writestr ( "__main__.py" , main_py . encode ( "utf-8" ) );
        if interpreter && !hasattr ( target , "write" ) {
        target . chmod ( target . stat ( ) . st_mode | stat . S_IEXEC );
        pub fn get_interpreter ( archive )  {
        // with scope: _maybe_open ( archive , "rb" ) as f  {
        if f . read ( 2 ) == b "#!" {
        return  f . readline ( ) . strip ( ) . decode ( shebang_encoding );
        pub fn main ( args = None /* Option */ )  {
        "Run the zipapp command line interface.

    The ARGS parameter lets you specify the argument list directly.
    Omitting ARGS (or setting it to None /* Option */) works as for argparse, using
    sys.argv[1:] as the argument list.
    ";
        import argparse;
        parser = argparse . ArgumentParser ( );
        parser . add_argument ( "--output" , "-o" , default = None /* Option */ ,;
        help = "The name of the output archive. ";
        "Required if SOURCE == an archive." );
        parser . add_argument ( "--python" , "-p" , default = None /* Option */ ,;
        help = "The name of the Python interpreter to use ";
        "(default: no shebang line)." );
        parser . add_argument ( "--main" , "-m" , default = None /* Option */ ,;
        help = "The main function of the application ";
        "(default: use an existing __main__.py)." );
        parser . add_argument ( "--compress" , "-c" , action = "store_true" ,;
        help = "Compress files with the deflate method. ";
        "Files are stored uncompressed by default." );
        parser . add_argument ( "--info" , default = false , action = "store_true" ,;
        help = "Display the interpreter from the archive." );
        parser . add_argument ( "source" ,;
        help = "Source directory (or existing archive)." );
        args = parser . parse_args ( args );
        if args . info {
        if !os . path . isfile ( args . source ) {
        panic!("SystemExit ( "Can only get info for an archive file" )");
        interpreter = get_interpreter ( args . source );
        println!( "Interpreter: {}" . format ( interpreter || "<none>" ) );
        sys . exit ( 0 );
        if os . path . isfile ( args . source ) {
        if args . output is None /* Option */ || ( os . path . exists ( args . output ) and {
        os . path . samefile ( args . source , args . output ) ) ;
        panic!("SystemExit ( "In-place editing of archives is !supported" )");
        if args . main {
        panic!("SystemExit ( "Cannot change the main function when copying" )");
        create_archive ( args . source , args . output ,;
        interpreter = args . python , main = args . main ,;
        compressed = args . compress );
        fn main() {
        main ( );
}

