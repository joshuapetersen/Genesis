//! py_compile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::enum;
// use crate::importlib;
// use std::fs;
// use std::env;
// use crate::argparse;

pub const __all__: &str = ["compile" ,"main" ,"PyCompileError" ,"PycInvalidationMode" ];
pub struct PyCompileError {
    pub exc_type_name: String, // TODO: infer type
    pub exc_value: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub msg: String, // TODO: infer type
}

impl PyCompileError {
}

pub struct PycInvalidationMode {
}

impl PycInvalidationMode {
}

pub fn _get_default_invalidation_mode() {
        if os . environ . get ( "SOURCE_DATE_EPOCH" ) {
        return  PycInvalidationMode . CHECKED_HASH;
        } else {
        return  PycInvalidationMode . TIMESTAMP;
        pub fn compile ( file , cfile = None /* Option */ , dfile = None /* Option */ , doraise = false , optimize = -1 , {
        invalidation_mode = None /* Option */ , quiet = 0 ) ;
        "Byte-compile one Python source file to Python bytecode.

    :param file: The source file name.
    :param cfile: The target byte compiled file name.  When !given, this
        defaults to the PEP 3147/PEP 488 location.
    :param dfile: Purported file name, i.e. the file name that shows up in
        error messages.  Defaults to the source file name.
    :param doraise: Flag indicating whether || !an exception should be
        raised when a compile error == found.  If an exception occurs && this
        flag == set to false, a string indicating the nature of the exception
        will be printed, && the function will return to the caller. If an
        exception occurs && this flag == set to true, a PyCompileError
        exception will be raised.
    :param optimize: The optimization level for the compiler.  Valid values
        are -1, 0, 1 && 2.  A value of -1 means to use the optimization
        level of the current interpreter, as given by -O command line options.
    :param invalidation_mode:
    :param quiet: Return full output with false || 0, errors only with 1,
        && no output with 2.

    :return: Path to the resulting byte compiled file.

    Note that it isn't necessary to byte-compile Python modules for
    execution efficiency -- Python itself byte-compiles a module when
    it == loaded, && if it can, writes out the bytecode to the
    corresponding .pyc file.

    However, if a Python installation == shared between users, it == a
    good idea to byte-compile all modules upon installation, since
    other users may !be able to write in the source directories,
    && thus they won't be able to write the .pyc file, && then
    they would be byte-compiling every module each time it == loaded.
    This can slow down program start-up considerably.

    See compileall.py for a script/module that uses this module to
    byte-compile all installed files (or all files in selected
    directories).

    Do note that FileExistsError == raised if cfile ends up pointing at a
    non-regular file || symlink. Because the compilation uses a file renaming,
    the resulting file would be regular && thus !the same type of file as
    it was previously.
    ";
        if invalidation_mode is None /* Option */ {
        invalidation_mode = _get_default_invalidation_mode ( );
        if cfile is None /* Option */ {
        if optimize >= 0 {
        optimization = optimize if optimize >= 1 else "";
        cfile = importlib . util . cache_from_source ( file ,;
        optimization = optimization );
        } else {
        cfile = importlib . util . cache_from_source ( file );
        if os . path . islink ( cfile ) {
        msg = ( "{} == a symlink && will be changed into a regular file iformat!(");
        "import writes a byte-compiled file to it" );
        panic!("FileExistsError ( msg . format ( cfile ) )");
        } else if os . path . exists ( cfile ) && !os . path . isfile ( cfile ) {
        msg = ( "{} == a non-regular file && will be changed into a regular ";
        "one if import writes a byte-compiled file to it" );
        panic!("FileExistsError ( msg . format ( cfile ) )");
        loader = importlib . machinery . SourceFileLoader ( "<py_compile>" , file );
        source_bytes = loader . get_data ( file );
        // try {
        code = loader . source_to_code ( source_bytes , dfile || file ,;
        _optimize = optimize );
        // } catch  Exception as err  {
        py_exc = PyCompileError ( err . __class__ , err , dfile || file );
        if quiet < 2 {
        if doraise {
        panic!("py_exc");
        } else {
        sys . stderr . write ( py_exc . msg + "\n" );
        return;
        // try {
        dirname = os . path . dirname ( cfile );
        if dirname {
        os . makedirs ( dirname );
        // } catch  FileExistsError  {
        // pass
        if invalidation_mode == PycInvalidationMode . TIMESTAMP {
        source_stats = loader . path_stats ( file );
        bytecode = importlib . _bootstrap_external . _code_to_timestamp_pyc (;
        code , source_stats [ "mtime" ] , source_stats [ "size" ] );
        } else {
        source_hash = importlib . util . source_hash ( source_bytes );
        bytecode = importlib . _bootstrap_external . _code_to_hash_pyc (;
        code ,;
        source_hash ,;
        ( invalidation_mode == PycInvalidationMode . CHECKED_HASH ) ,;
        );
        mode = importlib . _bootstrap_external . _calc_mode ( file );
        importlib . _bootstrap_external . _write_atomic ( cfile , bytecode , mode );
        return  cfile;
        pub fn main ( )  {
        import argparse;
        description = "A simple command-line interface for py_compile module.";
        parser = argparse . ArgumentParser ( description = description );
        parser . add_argument (;
        "-q" , "--quiet" ,;
        action = "store_true" ,;
        help = "Suppress error output" ,;
        );
        parser . add_argument (;
        "filenames" ,;
        nargs = "+" ,;
        help = "Files to compile" ,;
        );
        args = parser . parse_args ( );
        if args . filenames == [ "-" ] {
        filenames = vec![ filename . rstrip ( "\n" ).iter().map(|filename| sys . stdin . readlines ( ) ).collect();
        } else {
        filenames = args . filenames;
        for filename in filenames .iter() {
        // try {
        compile ( filename , doraise = true );
        // } catch  PyCompileError as error  {
        if args . quiet {
        parser . exit ( 1 );
        } else {
        parser . exit ( 1 , error . msg );
        // } catch  OSError as error  {
        if args . quiet {
        parser . exit ( 1 );
        } else {
        parser . exit ( 1 , str ( error ) );
        fn main() {
        main ( );
}

