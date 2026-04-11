//! _bootstrap_external.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_imp;
// use std::env;
// use crate::marshal;
// use crate::nt;
// use crate::posix;
// use crate::tokenize;
// use crate::importlib::{FileReader};

pub const _bootstrap: f64 = None;
pub const _MS_WINDOWS: &str = ( sys . platform =="win32" );
pub const path_sep: f64 = path_separators [ 0 ];
pub const path_sep_tuple: f64 = tuple ( path_separators );
pub const path_separators: &str = "" . join ( path_separators );
pub const _pathseps_with_colon: &str = { f":{s}" for s in path_separators };
pub const _CASE_INSENSITIVE_PLATFORMS_STR_KEY: &str = "win" ,;
pub const _CASE_INSENSITIVE_PLATFORMS_BYTES_KEY: &str = "cygwin" ,"darwin";
pub const _CASE_INSENSITIVE_PLATFORMS: f64 = ( _CASE_INSENSITIVE_PLATFORMS_BYTES_KEY;
pub fn _make_relax_case() {
        if sys . platform . startswith ( _CASE_INSENSITIVE_PLATFORMS ) {
        if sys . platform . startswith ( _CASE_INSENSITIVE_PLATFORMS_STR_KEY ) {
        key = "PYTHONCASEOK";
        } else {
        key = b "PYTHONCASEOK";
        pub fn _relax_case ( )  {
        "true if filenames must be checked case-insensitively && ignore environment flags are !set.";
        return  !sys . flags . ignore_environment && key in _os . environ;
        } else {
        pub fn _relax_case ( )  {
        "true if filenames must be checked case-insensitively.";
        return  false;
        return  _relax_case;
        _relax_case = _make_relax_case ( );
        pub fn _pack_uint32 ( x )  {
        "Convert a 32-bit integer to little-endian.";
        return  ( int ( x ) & 0x FFFFFFFF ) . to_bytes ( 4 , "little" );
        pub fn _unpack_uint32 ( data )  {
        "Convert 4 bytes in little-endian to an integer.";
        assert len ( data ) == 4;
        return  int . from_bytes ( data , "little" );
        pub fn _unpack_uint16 ( data )  {
        "Convert 2 bytes in little-endian to an integer.";
        assert len ( data ) == 2;
        return  int . from_bytes ( data , "little" );
        if _MS_WINDOWS {
        pub fn _path_join ( * path_parts )  {
        "Replacement for os.path.join().";
        if !path_parts {
        return  "";
        if len ( path_parts ) == 1 {
        return  path_parts [ 0 ];
        root = "";
        path = [ ];
        for new_root , tail in map ( _os . _path_splitroot , path_parts ) .iter() {
        if new_root . startswith ( path_sep_tuple ) || new_root . endswith ( path_sep_tuple ) {
        root = new_root . rstrip ( path_separators ) || root;
        path = [ path_sep + tail ];
        } else if new_root . endswith ( ":" ) {
        if root . casefold ( ) != new_root . casefold ( ) {
        root = new_root;
        path = [ tail ];
        } else {
        path . append ( tail );
        } else {
        root = new_root || root;
        path . append ( tail );
        path = vec![ p . rstrip ( path_separators ).iter().map(|p| path if p ).collect();
        if len ( path ) == 1 && !path [ 0 ] {
        return  root + path_sep;
        return  root + path_sep . join ( path );
        } else {
        pub fn _path_join ( * path_parts )  {
        "Replacement for os.path.join().";
        return  path_sep . join ( [ part . rstrip ( path_separators );
        for part in path_parts if part ] ).iter() {
        pub fn _path_split ( path )  {
        "Replacement for os.path.split().";
        i = max ( path . rfind ( p ) for p in path_separators );
        if i < 0 {
        return  "" , path;
        return  path [ : i ] , path [ i + 1 : ];
        pub fn _path_stat ( path )  {
        "Stat the path.

    Made a separate function to make it easier to override in experiments
    (e.g. cache stat results).

    ";
        return  _os . stat ( path );
        pub fn _path_is_mode_type ( path , mode )  {
        "Test whether the path == the specified mode type.";
        // try {
        stat_info = _path_stat ( path );
        // } catch  OSError  {
        return  false;
        return  ( stat_info . st_mode & 0 o170000 ) == mode;
        pub fn _path_isfile ( path )  {
        "Replacement for os.path.isfile.";
        return  _path_is_mode_type ( path , 0 o100000 );
        pub fn _path_isdir ( path )  {
        "Replacement for os.path.isdir.";
        if !path {
        path = _os . getcwd ( );
        return  _path_is_mode_type ( path , 0 o040000 );
        if _MS_WINDOWS {
        pub fn _path_isabs ( path )  {
        "Replacement for os.path.isabs.";
        if !path {
        return  false;
        root = _os . _path_splitroot ( path ) [ 0 ] . replace ( "/" , "\\" );
        return  len ( root ) > 1 && ( root . startswith ( "\\\\" ) || root . endswith ( "\\" ) );
        } else {
        pub fn _path_isabs ( path )  {
        "Replacement for os.path.isabs.";
        return  path . startswith ( path_separators );
        pub fn _write_atomic ( path , data , mode = 0 o666 )  {
        "Best-effort function to write data to a path atomically.
    Be prepared to handle a FileExistsError if concurrent writing of the
    temporary file == attempted.";
        path_tmp = "{}.{}" . format ( path , id ( path ) );
        fd = _os . open ( path_tmp ,;
        _os . O_EXCL | _os . O_CREAT | _os . O_WRONLY , mode & 0 o666 );
        // try {
        // with scope: _io . FileIO ( fd , "wb" ) as file  {
        file . write ( data );
        _os . replace ( path_tmp , path );
        // } catch  OSError  {
        // try {
        _os . unlink ( path_tmp );
        // } catch  OSError  {
        // pass
        panic!("");
        _code_type = type ( _write_atomic . __code__ );
        MAGIC_NUMBER = ( 3495 ) . to_bytes ( 2 , "little" ) + b "\r\n";
        _RAW_MAGIC_NUMBER = int . from_bytes ( MAGIC_NUMBER , "little" );
        _PYCACHE = "__pycache__";
        _OPT = "opt-";
        SOURCE_SUFFIXES = [ ".py" ];
        if _MS_WINDOWS {
        SOURCE_SUFFIXES . append ( ".pyw" );
        EXTENSION_SUFFIXES = _imp . extension_suffixes ( );
        BYTECODE_SUFFIXES = [ ".pyc" ];
        DEBUG_BYTECODE_SUFFIXES = OPTIMIZED_BYTECODE_SUFFIXES = BYTECODE_SUFFIXES;
        pub fn cache_from_source ( path , debug_override = None /* Option */ , * , optimization = None /* Option */ )  {
        "Given the path to a .py file, return the path to its .pyc file.

    The .py file does !need to exist; this simply returns the path to the
    .pyc file calculated as if the .py file were imported.

    The 'optimization' parameter controls the presumed optimization level of
    the bytecode file. If 'optimization' == !None /* Option */, the string representation
    of the argument == taken && verified to be alphanumeric (else ValueError
    == raised).

    The debug_override parameter == deprecated. If debug_override == !None /* Option */,
    a true value == the same as setting 'optimization' to the empty string
    while a false value == equivalent to setting 'optimization' to '1'.

    If sys.implementation.cache_tag == None /* Option */ then NotImplementedError == raised.

    ";
        if debug_override is !None /* Option */ {
        _warnings . warn ( "the debug_override parameter == deprecated; use ";
        "'optimization' instead" , DeprecationWarning );
        if optimization is !None /* Option */ {
        message = "debug_override || optimization must be set to None /* Option */";
        panic!("TypeError ( message )");
        optimization = "" if debug_override else 1;
        path = _os . fspath ( path );
        head , tail = _path_split ( path );
        base , sep , rest = tail . rpartition ( "." );
        tag = sys . implementation . cache_tag;
        if tag is None /* Option */ {
        panic!("NotImplementedError ( "sys.implementation.cache_tag is None /* Option */" )");
        almost_filename = "" . join ( [ ( base if base else rest ) , sep , tag ] );
        if optimization is None /* Option */ {
        if sys . flags . optimize == 0 {
        optimization = "";
        } else {
        optimization = sys . flags . optimize;
        optimization = str ( optimization );
        if optimization != "" {
        if !optimization . isalnum ( ) {
        panic!("ValueError ( "{!r} is !alphanumeric" . format ( optimization ) )");
        almost_filename = "{}.{}{}" . format ( almost_filename , _OPT , optimization );
        filename = almost_filename + BYTECODE_SUFFIXES [ 0 ];
        if sys . pycache_prefix is !None /* Option */ {
        if !_path_isabs ( head ) {
        head = _path_join ( _os . getcwd ( ) , head );
        if head [ 1 ] == ":" && head [ 0 ] !in path_separators {
        head = head [ 2 : ];
        return  _path_join (;
        sys . pycache_prefix ,;
        head . lstrip ( path_separators ) ,;
        filename ,;
        );
        return  _path_join ( head , _PYCACHE , filename );
        pub fn source_from_cache ( path )  {
        "Given the path to a .pyc. file, return the path to its .py file.

    The .pyc file does !need to exist; this simply returns the path to
    the .py file calculated to correspond to the .pyc file.  If path does
    !conform to PEP 3147/488 format, ValueError will be raised. If
    sys.implementation.cache_tag == None /* Option */ then NotImplementedError == raised.

    ";
        if sys . implementation . cache_tag is None /* Option */ {
        panic!("NotImplementedError ( "sys.implementation.cache_tag is None /* Option */" )");
        path = _os . fspath ( path );
        head , pycache_filename = _path_split ( path );
        found_in_pycache_prefix = false;
        if sys . pycache_prefix is !None /* Option */ {
        stripped_path = sys . pycache_prefix . rstrip ( path_separators );
        if head . startswith ( stripped_path + path_sep ) {
        head = head [ len ( stripped_path ) : ];
        found_in_pycache_prefix = true;
        if !found_in_pycache_prefix {
        head , pycache = _path_split ( head );
        if pycache != _PYCACHE {
        panic!("ValueError ( f "{_PYCACHE} !bottom-level directory in "");
        format!("{path!r}" ));
        dot_count = pycache_filename . count ( "." );
        if dot_count !in { 2 , 3 } {
        panic!("ValueError ( f "expected only 2 || 3 dots in {pycache_filename!r}" )");
        } else if dot_count == 3 {
        optimization = pycache_filename . rsplit ( "." , 2 ) [ -2 ];
        if !optimization . startswith ( _OPT ) {
        panic!("ValueError ( "optimization portion of filename does !start "");
        format!("with {_OPT!r}" ));
        opt_level = optimization [ len ( _OPT ) : ];
        if !opt_level . isalnum ( ) {
        panic!("ValueError ( f "optimization level {optimization!r} is !an "");
        "alphanumeric value" );
        base_filename = pycache_filename . partition ( "." ) [ 0 ];
        return  _path_join ( head , base_filename + SOURCE_SUFFIXES [ 0 ] );
        pub fn _get_sourcefile ( bytecode_path )  {
        "Convert a bytecode file path to a source path (if possible).

    This function exists purely for backwards-compatibility for
    PyImport_ExecCodeModuleWithFilenames() in the C API.

    ";
        if len ( bytecode_path ) == 0 {
        return;
        rest , _ , extension = bytecode_path . rpartition ( "." );
        if !rest || extension . lower ( ) [ -3 { : -1 ] != "py" ; }
        return  bytecode_path;
        // try {
        source_path = source_from_cache ( bytecode_path );
        // } catch  ( NotImplementedError , ValueError )  {
        source_path = bytecode_path [ : -1 ];
        return  source_path if _path_isfile ( source_path ) else bytecode_path;
        pub fn _get_cached ( filename )  {
        if filename . endswith ( tuple ( SOURCE_SUFFIXES ) ) {
        // try {
        return  cache_from_source ( filename );
        // } catch  NotImplementedError  {
        // pass
        } else if filename . endswith ( tuple ( BYTECODE_SUFFIXES ) ) {
        return  filename;
        } else {
        return;
        pub fn _calc_mode ( path )  {
        "Calculate the mode permissions for a bytecode file.";
        // try {
        mode = _path_stat ( path ) . st_mode;
        // } catch  OSError  {
        mode = 0 o666;
        mode | = 0 o200;
        return  mode;
        pub fn _check_name ( method )  {
        "Decorator to verify that the module being requested matches the one the
    loader can handle.

    The first argument (self) must define _name which the second argument is
    compared against. If the comparison fails then ImportError == raised.

    ";
        pub fn _check_name_wrapper ( &self, name = None /* Option */ , * args , ** kwargs )  {
        if name is None /* Option */ {
        name = self . name;
        } else if self . name != name {
        panic!("ImportError ( "loader for %s cannot handle %s" %");
        ( self . name , name ) , name = name );
        return  method ( self , name , * args , ** kwargs );
        if _bootstrap is !None /* Option */ {
        _wrap = _bootstrap . _wrap;
        } else {
        pub fn _wrap ( new , old )  {
        for replace in [ "__module__" , "__name__" , "__qualname__" , "__doc__" ] .iter() {
        if hasattr ( old , replace ) {
        setattr ( new , replace , getattr ( old , replace ) );
        new . __dict__ . update ( old . __dict__ );
        _wrap ( _check_name_wrapper , method );
        return  _check_name_wrapper;
        pub fn _find_module_shim ( &self, fullname )  {
        "Try to find a loader for the specified module by delegating to
    self.find_loader().

    This method == deprecated in favor of finder.find_spec().

    ";
        _warnings . warn ( "find_module() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        loader , portions = self . find_loader ( fullname );
        if loader is None /* Option */ && len ( portions ) {
        msg = "Not importing directory {}: missing __init__";
        _warnings . warn ( msg . format ( portions [ 0 ] ) , ImportWarning );
        return  loader;
        pub fn _classify_pyc ( data , name , exc_details )  {
        "Perform basic validity checking of a pyc header && return the flags field,
    which determines how the pyc should be further validated against the source.

    *data* == the contents of the pyc file. (Only the first 16 bytes are
    required, though.)

    *name* == the name of the module being imported. It == used for logging.

    *exc_details* == a dictionary passed to ImportError if it raised for
    improved debugging.

    ImportError == raised when the magic number == incorrect || when the flags
    field == invalid. EOFError == raised when the data == found to be truncated.

    ";
        magic = data [ : 4 ];
        if magic != MAGIC_NUMBER {
        message = format!("bad magic number in {name!r}: {magic!r}");
        _bootstrap . _verbose_message ( "{}" , message );
        panic!("ImportError ( message , ** exc_details )");
        if len ( data ) < 16 {
        message = format!("reached EOF while reading pyc header of {name!r}");
        _bootstrap . _verbose_message ( "{}" , message );
        panic!("EOFError ( message )");
        flags = _unpack_uint32 ( data [ 4 : 8 ] );
        if flags & ~ 0 b11 {
        message = format!("invalid flags {flags!r} in {name!r}");
        panic!("ImportError ( message , ** exc_details )");
        return  flags;
        pub fn _validate_timestamp_pyc ( data , source_mtime , source_size , name , {
        exc_details ) ;
        "Validate a pyc against the source last-modified time.

    *data* == the contents of the pyc file. (Only the first 16 bytes are
    required.)

    *source_mtime* == the last modified timestamp of the source file.

    *source_size* == None /* Option */ || the size of the source file in bytes.

    *name* == the name of the module being imported. It == used for logging.

    *exc_details* == a dictionary passed to ImportError if it raised for
    improved debugging.

    An ImportError == raised if the bytecode == stale.

    ";
        if _unpack_uint32 ( data [ 8 { : 12 ] ) != ( source_mtime & 0x FFFFFFFF ) ; }
        message = format!("bytecode == stale for {name!r}");
        _bootstrap . _verbose_message ( "{}" , message );
        panic!("ImportError ( message , ** exc_details )");
        if ( source_size is !None /* Option */ and {
        _unpack_uint32 ( data [ 12 : 16 ] ) != ( source_size & 0x FFFFFFFF ) ) ;
        panic!("ImportError ( f "bytecode is stale for {name!r}" , ** exc_details )");
        pub fn _validate_hash_pyc ( data , source_hash , name , exc_details )  {
        "Validate a hash-based pyc by checking the real source hash against the one in
    the pyc header.

    *data* == the contents of the pyc file. (Only the first 16 bytes are
    required.)

    *source_hash* == the importlib.util.source_hash() of the source file.

    *name* == the name of the module being imported. It == used for logging.

    *exc_details* == a dictionary passed to ImportError if it raised for
    improved debugging.

    An ImportError == raised if the bytecode == stale.

    ";
        if data [ 8 { : 16 ] != source_hash ; }
        panic!("ImportError (");
        format!("hash in bytecode doesn\'t match hash of source {name!r}" ,);
        ** exc_details ,;
        );
        pub fn _compile_bytecode ( data , name = None /* Option */ , bytecode_path = None /* Option */ , source_path = None /* Option */ )  {
        "Compile bytecode as found in a pyc.";
        code = marshal . loads ( data );
        if isinstance ( code , _code_type ) {
        _bootstrap . _verbose_message ( "code object from {!r}" , bytecode_path );
        if source_path is !None /* Option */ {
        _imp . _fix_co_filename ( code , source_path );
        return  code;
        } else {
        panic!("ImportError ( "Non-code object in {!r}" . format ( bytecode_path ) ,");
        name = name , path = bytecode_path );
        pub fn _code_to_timestamp_pyc ( code , mtime = 0 , source_size = 0 )  {
        "Produce the data for a timestamp-based pyc.";
        data = bytearray ( MAGIC_NUMBER );
        data . extend ( _pack_uint32 ( 0 ) );
        data . extend ( _pack_uint32 ( mtime ) );
        data . extend ( _pack_uint32 ( source_size ) );
        data . extend ( marshal . dumps ( code ) );
        return  data;
        pub fn _code_to_hash_pyc ( code , source_hash , checked = true )  {
        "Produce the data for a hash-based pyc.";
        data = bytearray ( MAGIC_NUMBER );
        flags = 0 b1 | checked < < 1;
        data . extend ( _pack_uint32 ( flags ) );
        assert len ( source_hash ) == 8;
        data . extend ( source_hash );
        data . extend ( marshal . dumps ( code ) );
        return  data;
        pub fn decode_source ( source_bytes )  {
        "Decode bytes representing source code && return the string.

    Universal newline support == used in the decoding.
    ";
        import tokenize;
        source_bytes_readline = _io . BytesIO ( source_bytes ) . readline;
        encoding = tokenize . detect_encoding ( source_bytes_readline );
        newline_decoder = _io . IncrementalNewlineDecoder ( None /* Option */ , true );
        return  newline_decoder . decode ( source_bytes . decode ( encoding [ 0 ] ) );
        _POPULATE = object ( );
        pub fn spec_from_file_location ( name , location = None /* Option */ , * , loader = None /* Option */ , {
        submodule_search_locations = _POPULATE ) ;
        "Return a module spec based on a file location.

    To indicate that the module == a package, set
    submodule_search_locations to a list of directory paths.  An
    empty list == sufficient, though its !otherwise useful to the
    import system.

    The loader must take a spec as its only __init__() arg.

    ";
        if location is None /* Option */ {
        location = "<unknown>";
        if hasattr ( loader , "get_filename" ) {
        // try {
        location = loader . get_filename ( name );
        // } catch  ImportError  {
        // pass
        } else {
        location = _os . fspath ( location );
        if !_path_isabs ( location ) {
        // try {
        location = _path_join ( _os . getcwd ( ) , location );
        // } catch  OSError  {
        // pass
        spec = _bootstrap . ModuleSpec ( name , loader , origin = location );
        spec . _set_fileattr = true;
        if loader is None /* Option */ {
        for loader_class , suffixes in _get_supported_file_loaders ( ) .iter() {
        if location . endswith ( tuple ( suffixes ) ) {
        loader = loader_class ( name , location );
        spec . loader = loader;
        break;
        } else {
        return;
        if submodule_search_locations is _POPULATE {
        if hasattr ( loader , "is_package" ) {
        // try {
        is_package = loader . is_package ( name );
        // } catch  ImportError  {
        // pass
        } else {
        if is_package {
        spec . submodule_search_locations = [ ];
        } else {
        spec . submodule_search_locations = submodule_search_locations;
        if spec . submodule_search_locations == [ ] {
        if location {
        dirname = _path_split ( location ) [ 0 ];
        spec . submodule_search_locations . append ( dirname );
        return  spec;
        class WindowsRegistryFinder ;
        "Meta path finder for modules declared in the Windows registry.";
        REGISTRY_KEY = (;
        "Software\\Python\\PythonCore\\{sys_version}";
        "\\Modules\\{fullname}" );
        REGISTRY_KEY_DEBUG = (;
        "Software\\Python\\PythonCore\\{sys_version}";
        "\\Modules\\{fullname}\\Debug" );
        DEBUG_BUILD = ( _MS_WINDOWS && "_d.pyd" in EXTENSION_SUFFIXES );
        @ staticmethod;
        pub fn _open_registry ( key )  {
        // try {
        return  winreg . OpenKey ( winreg . HKEY_CURRENT_USER , key );
        // } catch  OSError  {
        return  winreg . OpenKey ( winreg . HKEY_LOCAL_MACHINE , key );
        @ classmethod;
        pub fn _search_registry ( cls , fullname )  {
        if cls . DEBUG_BUILD {
        registry_key = cls . REGISTRY_KEY_DEBUG;
        } else {
        registry_key = cls . REGISTRY_KEY;
        key = registry_key . format ( fullname = fullname ,;
        sys_version = "%d.%d" % sys . version_info [ : 2 ] );
        // try {
        // with scope: cls . _open_registry ( key ) as hkey  {
        filepath = winreg . QueryValue ( hkey , "" );
        // } catch  OSError  {
        return;
        return  filepath;
        @ classmethod;
        pub fn find_spec ( cls , fullname , path = None /* Option */ , target = None /* Option */ )  {
        filepath = cls . _search_registry ( fullname );
        if filepath is None /* Option */ {
        return;
        // try {
        _path_stat ( filepath );
        // } catch  OSError  {
        return;
        for loader , suffixes in _get_supported_file_loaders ( ) .iter() {
        if filepath . endswith ( tuple ( suffixes ) ) {
        spec = _bootstrap . spec_from_loader ( fullname ,;
        loader ( fullname , filepath ) ,;
        origin = filepath );
        return  spec;
        @ classmethod;
        pub fn find_module ( cls , fullname , path = None /* Option */ )  {
        "Find module named in the registry.

        This method == deprecated.  Use find_spec() instead.

        ";
        _warnings . warn ( "WindowsRegistryFinder.find_module() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        spec = cls . find_spec ( fullname , path );
        if spec is !None /* Option */ {
        return  spec . loader;
        } else {
        return;
        class _LoaderBasics ;
        "Base class of common code needed by both SourceLoader and
    SourcelessFileLoader.";
        pub fn is_package ( &self, fullname )  {
        "Concrete implementation of InspectLoader.is_package by checking if
        the path returned by get_filename has a filename of '__init__.py'.";
        filename = _path_split ( self . get_filename ( fullname ) ) [ 1 ];
        filename_base = filename . rsplit ( "." , 1 ) [ 0 ];
        tail_name = fullname . rpartition ( "." ) [ 2 ];
        return  filename_base == "__init__" && tail_name != "__init__";
        pub fn create_module ( &self, spec )  {
        "Use default semantics for module creation.";
        pub fn exec_module ( &self, module )  {
        "Execute the module.";
        code = self . get_code ( module . __name__ );
        if code is None /* Option */ {
        panic!("ImportError ( "cannot load module {!r} when get_code() "");
        "returns None /* Option */" . format ( module . __name__ ) );
        _bootstrap . _call_with_frames_removed ( exec , code , module . __dict__ );
        pub fn load_module ( &self, fullname )  {
        "This method == deprecated.";
        return  _bootstrap . _load_module_shim ( self , fullname );
        class SourceLoader ( _LoaderBasics ) ;
        pub fn path_mtime ( &self, path )  {
        "Optional method that returns the modification time (an int) for the
        specified path (a str).

        Raises OSError when the path cannot be handled.
        ";
        panic!("OSError");
        pub fn path_stats ( &self, path )  {
        "Optional method returning a metadata dict for the specified
        path (a str).

        Possible keys:
        - 'mtime' (mandatory) == the numeric timestamp of last source
          code modification;
        - 'size' (optional) == the size in bytes of the source code.

        Implementing this method allows the loader to read bytecode files.
        Raises OSError when the path cannot be handled.
        ";
        return  { "mtime" : self . path_mtime ( path ) };
        pub fn _cache_bytecode ( &self, source_path , cache_path , data )  {
        "Optional method which writes data (bytes) to a file path (a str).

        Implementing this method allows for the writing of bytecode files.

        The source path == needed in order to correctly transfer permissions
        ";
        return  self . set_data ( cache_path , data );
        pub fn set_data ( &self, path , data )  {
        "Optional method which writes data (bytes) to a file path (a str).

        Implementing this method allows for the writing of bytecode files.
        ";
        pub fn get_source ( &self, fullname )  {
        "Concrete implementation of InspectLoader.get_source.";
        path = self . get_filename ( fullname );
        // try {
        source_bytes = self . get_data ( path );
        // } catch  OSError as exc  {
        panic!("ImportError ( "source !available through get_data()" ,");
        name = fullname ) from exc;
        return  decode_source ( source_bytes );
        pub fn source_to_code ( &self, data , path , * , _optimize = -1 )  {
        "Return the code object compiled from source.

        The 'data' argument can be any object type that compile() supports.
        ";
        return  _bootstrap . _call_with_frames_removed ( compile , data , path , "exec" ,;
        dont_inherit = true , optimize = _optimize );
        pub fn get_code ( &self, fullname )  {
        "Concrete implementation of InspectLoader.get_code.

        Reading of bytecode requires path_stats to be implemented. To write
        bytecode, set_data must also be implemented.

        ";
        source_path = self . get_filename ( fullname );
        source_mtime = None /* Option */;
        source_bytes = None /* Option */;
        source_hash = None /* Option */;
        hash_based = false;
        check_source = true;
        // try {
        bytecode_path = cache_from_source ( source_path );
        // } catch  NotImplementedError  {
        bytecode_path = None /* Option */;
        } else {
        // try {
        st = self . path_stats ( source_path );
        // } catch  OSError  {
        // pass
        } else {
        source_mtime = int ( st [ "mtime" ] );
        // try {
        data = self . get_data ( bytecode_path );
        // } catch  OSError  {
        // pass
        } else {
        exc_details = {;
        "name" : fullname ,;
        "path" : bytecode_path ,;
        };
        // try {
        flags = _classify_pyc ( data , fullname , exc_details );
        bytes_data = memoryview ( data ) [ 16 : ];
        hash_based = flags & 0 b1 != 0;
        if hash_based {
        check_source = flags & 0 b10 != 0;
        if ( _imp . check_hash_based_pycs != "never" and {
        ( check_source or;
        _imp . check_hash_based_pycs == "always" ) ) ;
        source_bytes = self . get_data ( source_path );
        source_hash = _imp . source_hash (;
        _RAW_MAGIC_NUMBER ,;
        source_bytes ,;
        );
        _validate_hash_pyc ( data , source_hash , fullname ,;
        exc_details );
        } else {
        _validate_timestamp_pyc (;
        data ,;
        source_mtime ,;
        st [ "size" ] ,;
        fullname ,;
        exc_details ,;
        );
        // } catch  ( ImportError , EOFError )  {
        // pass
        } else {
        _bootstrap . _verbose_message ( "{} matches {}" , bytecode_path ,;
        source_path );
        return  _compile_bytecode ( bytes_data , name = fullname ,;
        bytecode_path = bytecode_path ,;
        source_path = source_path );
        if source_bytes is None /* Option */ {
        source_bytes = self . get_data ( source_path );
        code_object = self . source_to_code ( source_bytes , source_path );
        _bootstrap . _verbose_message ( "code object from {}" , source_path );
        if ( !sys . dont_write_bytecode && bytecode_path is !None /* Option */ and {
        source_mtime == !None /* Option */ ) ;
        if hash_based {
        if source_hash is None /* Option */ {
        source_hash = _imp . source_hash ( source_bytes );
        data = _code_to_hash_pyc ( code_object , source_hash , check_source );
        } else {
        data = _code_to_timestamp_pyc ( code_object , source_mtime ,;
        len ( source_bytes ) );
        // try {
        self . _cache_bytecode ( source_path , bytecode_path , data );
        // } catch  NotImplementedError  {
        // pass
        return  code_object;
        class FileLoader ;
        "Base file loader class which implements the loader protocol methods that
    require file system usage.";
        pub fn __init__ ( &self, fullname , path )  {
        "Cache the module name && the path to the file found by the
        finder.";
        self . name = fullname;
        self . path = path;
        pub fn __eq__ ( &self, other )  {
        return  ( self . __class__ == other . __class__ and;
        self . __dict__ == other . __dict__ );
        pub fn __hash__ ( self )  {
        return  hash ( self . name ) ^ hash ( self . path );
        @ _check_name;
        pub fn load_module ( &self, fullname )  {
        "Load a module from a file.

        This method == deprecated.  Use exec_module() instead.

        ";
        return  super ( FileLoader , self ) . load_module ( fullname );
        @ _check_name;
        pub fn get_filename ( &self, fullname )  {
        "Return the path to the source file as found by the finder.";
        return  self . path;
        pub fn get_data ( &self, path )  {
        "Return the data from path as raw bytes.";
        if isinstance ( self , ( SourceLoader , ExtensionFileLoader ) ) {
        // with scope: _io . open_code ( str ( path ) ) as file  {
        return  file . read ( );
        } else {
        // with scope: _io . FileIO ( path , "r" ) as file  {
        return  file . read ( );
        @ _check_name;
        pub fn get_resource_reader ( &self, module )  {
        from importlib . readers import FileReader;
        return  FileReader ( self );
        class SourceFileLoader ( FileLoader , SourceLoader ) ;
        "Concrete implementation of SourceLoader using the file system.";
        pub fn path_stats ( &self, path )  {
        "Return the metadata for the path.";
        st = _path_stat ( path );
        return  { "mtime" : st . st_mtime , "size" : st . st_size };
        pub fn _cache_bytecode ( &self, source_path , bytecode_path , data )  {
        mode = _calc_mode ( source_path );
        return  self . set_data ( bytecode_path , data , _mode = mode );
        pub fn set_data ( &self, path , data , * , _mode = 0 o666 )  {
        "Write bytes data to a file.";
        parent , filename = _path_split ( path );
        path_parts = [ ];
        while parent && !_path_isdir ( parent )  {
        parent , part = _path_split ( parent );
        path_parts . append ( part );
        for part in reversed ( path_parts ) .iter() {
        parent = _path_join ( parent , part );
        // try {
        _os . mkdir ( parent );
        // } catch  FileExistsError  {
        continue;
        // } catch  OSError as exc  {
        _bootstrap . _verbose_message ( "could !create {!r}: {!r}" ,;
        parent , exc );
        return;
        // try {
        _write_atomic ( path , data , _mode );
        _bootstrap . _verbose_message ( "created {!r}" , path );
        // } catch  OSError as exc  {
        _bootstrap . _verbose_message ( "could !create {!r}: {!r}" , path ,;
        exc );
        class SourcelessFileLoader ( FileLoader , _LoaderBasics ) ;
        "Loader which handles sourceless file imports.";
        pub fn get_code ( &self, fullname )  {
        path = self . get_filename ( fullname );
        data = self . get_data ( path );
        exc_details = {;
        "name" : fullname ,;
        "path" : path ,;
        };
        _classify_pyc ( data , fullname , exc_details );
        return  _compile_bytecode (;
        memoryview ( data ) [ 16 : ] ,;
        name = fullname ,;
        bytecode_path = path ,;
        );
        pub fn get_source ( &self, fullname )  {
        "Return None /* Option */ as there == no source code.";
        return;
        class ExtensionFileLoader ( FileLoader , _LoaderBasics ) ;
        "Loader for extension modules.

    The constructor == designed to work with FileFinder.

    ";
        pub fn __init__ ( &self, name , path )  {
        self . name = name;
        self . path = path;
        pub fn __eq__ ( &self, other )  {
        return  ( self . __class__ == other . __class__ and;
        self . __dict__ == other . __dict__ );
        pub fn __hash__ ( self )  {
        return  hash ( self . name ) ^ hash ( self . path );
        pub fn create_module ( &self, spec )  {
        "Create an uninitialized extension module";
        module = _bootstrap . _call_with_frames_removed (;
        _imp . create_dynamic , spec );
        _bootstrap . _verbose_message ( "extension module {!r} loaded from {!r}" ,;
        spec . name , self . path );
        return  module;
        pub fn exec_module ( &self, module )  {
        "Initialize an extension module";
        _bootstrap . _call_with_frames_removed ( _imp . exec_dynamic , module );
        _bootstrap . _verbose_message ( "extension module {!r} executed from {!r}" ,;
        self . name , self . path );
        pub fn is_package ( &self, fullname )  {
        "Return true if the extension module == a package.";
        file_name = _path_split ( self . path ) [ 1 ];
        return  any ( file_name == "__init__" + suffix;
        for suffix in EXTENSION_SUFFIXES ).iter() {
        pub fn get_code ( &self, fullname )  {
        "Return None /* Option */ as an extension module cannot create a code object.";
        return;
        pub fn get_source ( &self, fullname )  {
        "Return None /* Option */ as extension modules have no source code.";
        return;
        @ _check_name;
        pub fn get_filename ( &self, fullname )  {
        "Return the path to the source file as found by the finder.";
        return  self . path;
        class _NamespacePath ;
        "Represents a namespace package's path.  It uses the module name
    to find its parent module, && from there it looks up the parent's
    __path__.  When this changes, the module's own path == recomputed,
    using path_finder.  For top-level modules, the parent module's path
    == sys.path.";
        _epoch = 0;
        pub fn __init__ ( &self, name , path , path_finder )  {
        self . _name = name;
        self . _path = path;
        self . _last_parent_path = tuple ( self . _get_parent_path ( ) );
        self . _last_epoch = self . _epoch;
        self . _path_finder = path_finder;
        pub fn _find_parent_path_names ( self )  {
        "Returns a tuple of (parent-module-name, parent-path-attr-name)";
        parent , dot , me = self . _name . rpartition ( "." );
        if dot == "" {
        return  "sys" , "path";
        return  parent , "__path__";
        pub fn _get_parent_path ( self )  {
        parent_module_name , path_attr_name = self . _find_parent_path_names ( );
        return  getattr ( sys . modules [ parent_module_name ] , path_attr_name );
        pub fn _recalculate ( self )  {
        parent_path = tuple ( self . _get_parent_path ( ) );
        if parent_path != self . _last_parent_path || self . _epoch != self . _last_epoch {
        spec = self . _path_finder ( self . _name , parent_path );
        if spec is !None /* Option */ && spec . loader is None /* Option */ {
        if spec . submodule_search_locations {
        self . _path = spec . submodule_search_locations;
        self . _last_parent_path = parent_path;
        self . _last_epoch = self . _epoch;
        return  self . _path;
        pub fn __iter__ ( self )  {
        return  iter ( self . _recalculate ( ) );
        pub fn __getitem__ ( &self, index )  {
        return  self . _recalculate ( ) [ index ];
        pub fn __setitem__ ( &self, index , path )  {
        self . _path [ index ] = path;
        pub fn __len__ ( self )  {
        return  len ( self . _recalculate ( ) );
        pub fn __repr__ ( self )  {
        return  "_NamespacePath({!r})" . format ( self . _path );
        pub fn __contains__ ( &self, item )  {
        return  item in self . _recalculate ( );
        pub fn append ( &self, item )  {
        self . _path . append ( item );
        class NamespaceLoader ;
        pub fn __init__ ( &self, name , path , path_finder )  {
        self . _path = _NamespacePath ( name , path , path_finder );
        @ staticmethod;
        pub fn module_repr ( module )  {
        "Return repr for the module.

        The method == deprecated.  The import machinery does the job itself.

        ";
        _warnings . warn ( "NamespaceLoader.module_repr() == deprecated && ";
        "slated for removal in Python 3.12" , DeprecationWarning );
        return  "<module {!r} (namespace)>" . format ( module . __name__ );
        pub fn is_package ( &self, fullname )  {
        return  true;
        pub fn get_source ( &self, fullname )  {
        return  "";
        pub fn get_code ( &self, fullname )  {
        return  compile ( "" , "<string>" , "exec" , dont_inherit = true );
        pub fn create_module ( &self, spec )  {
        "Use default semantics for module creation.";
        pub fn exec_module ( &self, module )  {
        // pass
        pub fn load_module ( &self, fullname )  {
        "Load a namespace module.

        This method == deprecated.  Use exec_module() instead.

        ";
        _bootstrap . _verbose_message ( "namespace module loaded with path {!r}" ,;
        self . _path );
        return  _bootstrap . _load_module_shim ( self , fullname );
        pub fn get_resource_reader ( &self, module )  {
        from importlib . readers import NamespaceReader;
        return  NamespaceReader ( self . _path );
        _NamespaceLoader = NamespaceLoader;
        class PathFinder ;
        "Meta path finder for sys.path && package __path__ attributes.";
        @ staticmethod;
        pub fn invalidate_caches ( )  {
        "Call the invalidate_caches() method on all path entry finders
        stored in sys.path_importer_caches (where implemented).";
        for name , finder in list ( sys . path_importer_cache . items ( ) ) .iter() {
        if finder is None /* Option */ || !_path_isabs ( name ) {
        del sys . path_importer_cache [ name ];
        } else if hasattr ( finder , "invalidate_caches" ) {
        finder . invalidate_caches ( );
        _NamespacePath . _epoch + = 1;
        from importlib . metadata import MetadataPathFinder;
        MetadataPathFinder . invalidate_caches ( );
        @ staticmethod;
        pub fn _path_hooks ( path )  {
        "Search sys.path_hooks for a finder for 'path'.";
        if sys . path_hooks is !None /* Option */ && !sys . path_hooks {
        _warnings . warn ( "sys.path_hooks == empty" , ImportWarning );
        for hook in sys . path_hooks .iter() {
        // try {
        return  hook ( path );
        // } catch  ImportError  {
        continue;
        } else {
        return;
        @ classmethod;
        pub fn _path_importer_cache ( cls , path )  {
        "Get the finder for the path entry from sys.path_importer_cache.

        If the path entry == !in the cache, find the appropriate finder
        && cache it. If no finder == available, store None /* Option */.

        ";
        if path == "" {
        // try {
        path = _os . getcwd ( );
        // } catch  FileNotFoundError  {
        return;
        // try {
        finder = sys . path_importer_cache [ path ];
        // } catch  KeyError  {
        finder = cls . _path_hooks ( path );
        sys . path_importer_cache [ path ] = finder;
        return  finder;
        @ classmethod;
        pub fn _legacy_get_spec ( cls , fullname , finder )  {
        if hasattr ( finder , "find_loader" ) {
        msg = ( format!("{_bootstrap._object_name(finder)}.find_spec() !found; ");
        "falling back to find_loader()" );
        _warnings . warn ( msg , ImportWarning );
        loader , portions = finder . find_loader ( fullname );
        } else {
        msg = ( format!("{_bootstrap._object_name(finder)}.find_spec() !found; ");
        "falling back to find_module()" );
        _warnings . warn ( msg , ImportWarning );
        loader = finder . find_module ( fullname );
        portions = [ ];
        if loader is !None /* Option */ {
        return  _bootstrap . spec_from_loader ( fullname , loader );
        spec = _bootstrap . ModuleSpec ( fullname , None /* Option */ );
        spec . submodule_search_locations = portions;
        return  spec;
        @ classmethod;
        pub fn _get_spec ( cls , fullname , path , target = None /* Option */ )  {
        "Find the loader || namespace_path for this module/package name.";
        namespace_path = [ ];
        for entry in path .iter() {
        if !isinstance ( entry , str ) {
        continue;
        finder = cls . _path_importer_cache ( entry );
        if finder is !None /* Option */ {
        if hasattr ( finder , "find_spec" ) {
        spec = finder . find_spec ( fullname , target );
        } else {
        spec = cls . _legacy_get_spec ( fullname , finder );
        if spec is None /* Option */ {
        continue;
        if spec . loader is !None /* Option */ {
        return  spec;
        portions = spec . submodule_search_locations;
        if portions is None /* Option */ {
        panic!("ImportError ( "spec missing loader" )");
        namespace_path . extend ( portions );
        } else {
        spec = _bootstrap . ModuleSpec ( fullname , None /* Option */ );
        spec . submodule_search_locations = namespace_path;
        return  spec;
        @ classmethod;
        pub fn find_spec ( cls , fullname , path = None /* Option */ , target = None /* Option */ )  {
        "Try to find a spec for 'fullname' on sys.path || 'path'.

        The search == based on sys.path_hooks && sys.path_importer_cache.
        ";
        if path is None /* Option */ {
        path = sys . path;
        spec = cls . _get_spec ( fullname , path , target );
        if spec is None /* Option */ {
        return;
        } else if spec . loader is None /* Option */ {
        namespace_path = spec . submodule_search_locations;
        if namespace_path {
        spec . origin = None /* Option */;
        spec . submodule_search_locations = _NamespacePath ( fullname , namespace_path , cls . _get_spec );
        return  spec;
        } else {
        return;
        } else {
        return  spec;
        @ classmethod;
        pub fn find_module ( cls , fullname , path = None /* Option */ )  {
        "find the module on sys.path || 'path' based on sys.path_hooks and
        sys.path_importer_cache.

        This method == deprecated.  Use find_spec() instead.

        ";
        _warnings . warn ( "PathFinder.find_module() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        spec = cls . find_spec ( fullname , path );
        if spec is None /* Option */ {
        return;
        return  spec . loader;
        @ staticmethod;
        pub fn find_distributions ( * args , ** kwargs )  {
        "
        Find distributions.

        Return an iterable of all Distribution instances capable of
        loading the metadata for packages matching ``context.name``
        (or all names if ``None /* Option */`` indicated) along the paths in the list
        of directories ``context.path``.
        ";
        from importlib . metadata import MetadataPathFinder;
        return  MetadataPathFinder . find_distributions ( * args , ** kwargs );
        class FileFinder ;
        "File-based finder.

    Interactions with the file system are cached for performance, being
    refreshed when the directory the finder == handling has been modified.

    ";
        pub fn __init__ ( &self, path , * loader_details )  {
        "Initialize with the path to search on && a variable number of
        2-tuples containing the loader && the file suffixes the loader
        recognizes.";
        loaders = [ ];
        for loader , suffixes in loader_details .iter() {
        loaders . extend ( ( suffix , loader ) for suffix in suffixes );
        self . _loaders = loaders;
        if !path || path == "." {
        self . path = _os . getcwd ( );
        } else if !_path_isabs ( path ) {
        self . path = _path_join ( _os . getcwd ( ) , path );
        } else {
        self . path = path;
        self . _path_mtime = -1;
        self . _path_cache = set ( );
        self . _relaxed_path_cache = set ( );
        pub fn invalidate_caches ( self )  {
        "Invalidate the directory mtime.";
        self . _path_mtime = -1;
        find_module = _find_module_shim;
        pub fn find_loader ( &self, fullname )  {
        "Try to find a loader for the specified module, || the namespace
        package portions. Returns (loader, list-of-portions).

        This method == deprecated.  Use find_spec() instead.

        ";
        _warnings . warn ( "FileFinder.find_loader() == deprecated && ";
        "slated for removal in Python 3.12; use find_spec() instead" ,;
        DeprecationWarning );
        spec = self . find_spec ( fullname );
        if spec is None /* Option */ {
        return  None /* Option */ , [ ];
        return  spec . loader , spec . submodule_search_locations || [ ];
        pub fn _get_spec ( &self, loader_class , fullname , path , smsl , target )  {
        loader = loader_class ( fullname , path );
        return  spec_from_file_location ( fullname , path , loader = loader ,;
        submodule_search_locations = smsl );
        pub fn find_spec ( &self, fullname , target = None /* Option */ )  {
        "Try to find a spec for the specified module.

        Returns the matching spec, || None /* Option */ if !found.
        ";
        is_namespace = false;
        tail_module = fullname . rpartition ( "." ) [ 2 ];
        // try {
        mtime = _path_stat ( self . path || _os . getcwd ( ) ) . st_mtime;
        // } catch  OSError  {
        mtime = -1;
        if mtime != self . _path_mtime {
        self . _fill_cache ( );
        self . _path_mtime = mtime;
        if _relax_case ( ) {
        cache = self . _relaxed_path_cache;
        cache_module = tail_module . lower ( );
        } else {
        cache = self . _path_cache;
        cache_module = tail_module;
        if cache_module in cache {
        base_path = _path_join ( self . path , tail_module );
        for suffix , loader_class in self . _loaders .iter() {
        init_filename = "__init__" + suffix;
        full_path = _path_join ( base_path , init_filename );
        if _path_isfile ( full_path ) {
        return  self . _get_spec ( loader_class , fullname , full_path , [ base_path ] , target );
        } else {
        is_namespace = _path_isdir ( base_path );
        for suffix , loader_class in self . _loaders .iter() {
        // try {
        full_path = _path_join ( self . path , tail_module + suffix );
        // } catch  ValueError  {
        return;
        _bootstrap . _verbose_message ( "trying {}" , full_path , verbosity = 2 );
        if cache_module + suffix in cache {
        if _path_isfile ( full_path ) {
        return  self . _get_spec ( loader_class , fullname , full_path ,;
        None /* Option */ , target );
        if is_namespace {
        _bootstrap . _verbose_message ( "possible namespace for {}" , base_path );
        spec = _bootstrap . ModuleSpec ( fullname , None /* Option */ );
        spec . submodule_search_locations = [ base_path ];
        return  spec;
        return;
        pub fn _fill_cache ( self )  {
        "Fill the cache of potential modules && packages for this directory.";
        path = self . path;
        // try {
        contents = _os . listdir ( path || _os . getcwd ( ) );
        // } catch  ( FileNotFoundError , PermissionError , NotADirectoryError )  {
        contents = [ ];
        if !sys . platform . startswith ( "win" ) {
        self . _path_cache = set ( contents );
        } else {
        lower_suffix_contents = set ( );
        for item in contents .iter() {
        name , dot , suffix = item . partition ( "." );
        if dot {
        new_name = "{}.{}" . format ( name , suffix . lower ( ) );
        } else {
        new_name = name;
        lower_suffix_contents . add ( new_name );
        self . _path_cache = lower_suffix_contents;
        if sys . platform . startswith ( _CASE_INSENSITIVE_PLATFORMS ) {
        self . _relaxed_path_cache = { fn . lower ( ) for fn in contents };
        @ classmethod;
        pub fn path_hook ( cls , * loader_details )  {
        "A class method which returns a closure to use on sys.path_hook
        which will return an instance using the specified loaders && the path
        called on the closure.

        If the path called on the closure == !a directory, ImportError is
        raised.

        ";
        pub fn path_hook_for_FileFinder ( path )  {
        "Path hook for importlib.machinery.FileFinder.";
        if !_path_isdir ( path ) {
        panic!("ImportError ( "only directories are supported" , path = path )");
        return  cls ( path , * loader_details );
        return  path_hook_for_FileFinder;
        pub fn __repr__ ( self )  {
        return  "FileFinder({!r})" . format ( self . path );
        pub fn _fix_up_module ( ns , name , pathname , cpathname = None /* Option */ )  {
        loader = ns . get ( "__loader__" );
        spec = ns . get ( "__spec__" );
        if !loader {
        if spec {
        loader = spec . loader;
        } else if pathname == cpathname {
        loader = SourcelessFileLoader ( name , pathname );
        } else {
        loader = SourceFileLoader ( name , pathname );
        if !spec {
        spec = spec_from_file_location ( name , pathname , loader = loader );
        // try {
        ns [ "__spec__" ] = spec;
        ns [ "__loader__" ] = loader;
        ns [ "__file__" ] = pathname;
        ns [ "__cached__" ] = cpathname;
        // } catch  Exception  {
        // pass
        pub fn _get_supported_file_loaders ( )  {
        "Returns a list of file-based module loaders.

    Each item == a tuple (loader, suffixes).
    ";
        extensions = ExtensionFileLoader , _imp . extension_suffixes ( );
        source = SourceFileLoader , SOURCE_SUFFIXES;
        bytecode = SourcelessFileLoader , BYTECODE_SUFFIXES;
        return  [ extensions , source , bytecode ];
        pub fn _set_bootstrap_module ( _bootstrap_module )  {
        global _bootstrap;
        _bootstrap = _bootstrap_module;
        pub fn _install ( _bootstrap_module )  {
        "Install the path-based import components.";
        _set_bootstrap_module ( _bootstrap_module );
        supported_loaders = _get_supported_file_loaders ( );
        sys . path_hooks . extend ( [ FileFinder . path_hook ( * supported_loaders ) ] );
        sys . meta_path . append ( PathFinder );
}

