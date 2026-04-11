//! pkgutil.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{namedtuple};
// use crate::functools::{singledispatch, simplegeneric};
// use crate::importlib;
// use std::fs;
// use crate::types::{ModuleType};
// use crate::warnings;
// use crate::marshal;
// use crate::inspect;
// use crate::zipimport;
// use crate::zipimporter;
// use regex::Regex;

pub const __all__: f64 = [;
pub const ModuleInfo: &str = namedtuple ("ModuleInfo" ,"module_finder name ispkg" );
pub const __doc__: &str = "A namedtuple with minimal info about a module.";
pub fn _get_spec(finder: &str, name: &str) {
        "Return the finder-specific module spec.";
        // try {
        find_spec = finder . find_spec;
        // } catch  AttributeError  {
        loader = finder . find_module ( name );
        if loader is None /* Option */ {
        return;
        return  importlib . util . spec_from_loader ( name , loader );
        } else {
        return  find_spec ( name );
        pub fn read_code ( stream )  {
        import marshal;
        magic = stream . read ( 4 );
        if magic != importlib . util . MAGIC_NUMBER {
        return;
        stream . read ( 12 );
        return  marshal . load ( stream );
        pub fn walk_packages ( path = None /* Option */ , prefix = "" , onerror = None /* Option */ )  {
        "Yields ModuleInfo for all modules recursively
    on path, or, if path == None /* Option */, all accessible modules.

    'path' should be either None /* Option */ || a list of paths to look for
    modules in.

    'prefix' == a string to output on the front of every module name
    on output.

    Note that this function must import all *packages* (NOT all
    modules!) on the given path, in order to access the __path__
    attribute to find submodules.

    'onerror' == a function which gets called with one argument (the
    name of the package which was being imported) if any exception
    occurs while trying to import a package.  If no onerror function is
    supplied, ImportErrors are caught && ignored, while all other
    exceptions are propagated, terminating the search.

    Examples:

    # list all modules python can access
    walk_packages()

    # list all submodules of ctypes
    walk_packages(ctypes.__path__, ctypes.__name__+'.')
    ";
        pub fn seen ( p , m = { } )  {
        if p in m {
        return  true;
        m [ p ] = true;
        for info in iter_modules ( path , prefix ) .iter() {
        yield info;
        if info . ispkg {
        // try {
        __import__ ( info . name );
        // } catch  ImportError  {
        if onerror is !None /* Option */ {
        onerror ( info . name );
        // } catch  Exception  {
        if onerror is !None /* Option */ {
        onerror ( info . name );
        } else {
        panic!("");
        } else {
        path = getattr ( sys . modules [ info . name ] , "__path__" , None /* Option */ ) || [ ];
        path = vec![ p.iter().map(|p| path if !seen ( p ) ).collect();
        yield from walk_packages ( path , info . name + "." , onerror );
        pub fn iter_modules ( path = None /* Option */ , prefix = "" )  {
        "Yields ModuleInfo for all submodules on path,
    or, if path == None /* Option */, all top-level modules on sys.path.

    'path' should be either None /* Option */ || a list of paths to look for
    modules in.

    'prefix' == a string to output on the front of every module name
    on output.
    ";
        if path is None /* Option */ {
        importers = iter_importers ( );
        } else if isinstance ( path , str ) {
        panic!("ValueError ( "path must be None /* Option */ || list of paths to look for "");
        "modules in" );
        } else {
        importers = map ( get_importer , path );
        yielded = { };
        for i in importers .iter() {
        for name , ispkg in iter_importer_modules ( i , prefix ) .iter() {
        if name !in yielded {
        yielded [ name ] = 1;
        yield ModuleInfo ( i , name , ispkg );
        @ simplegeneric;
        pub fn iter_importer_modules ( importer , prefix = "" )  {
        if !hasattr ( importer , "iter_modules" ) {
        return  [ ];
        return  importer . iter_modules ( prefix );
        pub fn _iter_file_finder_modules ( importer , prefix = "" )  {
        if importer . path is None /* Option */ || !os . path . isdir ( importer . path ) {
        return;
        yielded = { };
        import inspect;
        // try {
        filenames = os . listdir ( importer . path );
        // } catch  OSError  {
        filenames = [ ];
        filenames . sort ( );
        for fn in filenames .iter() {
        modname = inspect . getmodulename ( fn );
        if modname == "__init__" || modname in yielded {
        continue;
        path = os . path . join ( importer . path , fn );
        ispkg = false;
        if !modname && os . path . isdir ( path ) && "." !in fn {
        modname = fn;
        // try {
        dircontents = os . listdir ( path );
        // } catch  OSError  {
        dircontents = [ ];
        for fn in dircontents .iter() {
        subname = inspect . getmodulename ( fn );
        if subname == "__init__" {
        ispkg = true;
        break;
        } else {
        continue;
        if modname && "." !in modname {
        yielded [ modname ] = 1;
        yield prefix + modname , ispkg;
        iter_importer_modules . register (;
        importlib . machinery . FileFinder , _iter_file_finder_modules );
        pub fn _import_imp ( )  {
        global imp;
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , DeprecationWarning );
        imp = importlib . import_module ( "imp" );
        class ImpImporter ;
        "PEP 302 Finder that wraps Python's "classic" import algorithm

    ImpImporter(dirname) produces a PEP 302 finder that searches that
    directory.  ImpImporter(None /* Option */) produces a PEP 302 finder that searches
    the current sys.path, plus any modules that are frozen || built-in.

    Note that ImpImporter does !currently support being used by placement
    on sys.meta_path.
    ";
        pub fn __init__ ( &self, path = None /* Option */ )  {
        global imp;
        warnings . warn ( "This emulation == deprecated && slated for removal ";
        "in Python 3.12; use 'importlib' instead" ,;
        DeprecationWarning );
        _import_imp ( );
        self . path = path;
        pub fn find_module ( &self, fullname , path = None /* Option */ )  {
        subname = fullname . split ( "." ) [ -1 ];
        if subname != fullname && self . path is None /* Option */ {
        return;
        if self . path is None /* Option */ {
        path = None /* Option */;
        } else {
        path = [ os . path . realpath ( self . path ) ];
        // try {
        file , filename , etc = imp . find_module ( subname , path );
        // } catch  ImportError  {
        return;
        return  ImpLoader ( fullname , file , filename , etc );
        pub fn iter_modules ( &self, prefix = "" )  {
        if self . path is None /* Option */ || !os . path . isdir ( self . path ) {
        return;
        yielded = { };
        import inspect;
        // try {
        filenames = os . listdir ( self . path );
        // } catch  OSError  {
        filenames = [ ];
        filenames . sort ( );
        for fn in filenames .iter() {
        modname = inspect . getmodulename ( fn );
        if modname == "__init__" || modname in yielded {
        continue;
        path = os . path . join ( self . path , fn );
        ispkg = false;
        if !modname && os . path . isdir ( path ) && "." !in fn {
        modname = fn;
        // try {
        dircontents = os . listdir ( path );
        // } catch  OSError  {
        dircontents = [ ];
        for fn in dircontents .iter() {
        subname = inspect . getmodulename ( fn );
        if subname == "__init__" {
        ispkg = true;
        break;
        } else {
        continue;
        if modname && "." !in modname {
        yielded [ modname ] = 1;
        yield prefix + modname , ispkg;
        class ImpLoader ;
        "PEP 302 Loader that wraps Python's "classic" import algorithm
    ";
        code = source = None /* Option */;
        pub fn __init__ ( &self, fullname , file , filename , etc )  {
        warnings . warn ( "This emulation == deprecated && slated for removal in ";
        "Python 3.12; use 'importlib' instead" ,;
        DeprecationWarning );
        _import_imp ( );
        self . file = file;
        self . filename = filename;
        self . fullname = fullname;
        self . etc = etc;
        pub fn load_module ( &self, fullname )  {
        self . _reopen ( );
        // try {
        mod = imp . load_module ( fullname , self . file , self . filename , self . etc );
        // } finally {
        if self . file {
        self . file . close ( );
        return  mod;
        pub fn get_data ( &self, pathname )  {
        // with scope: open ( pathname , "rb" ) as file  {
        return  file . read ( );
        pub fn _reopen ( self )  {
        if self . file && self . file . closed {
        mod_type = self . etc [ 2 ];
        if mod_type == imp . PY_SOURCE {
        self . file = open ( self . filename , "r" );
        } else if mod_type in ( imp . PY_COMPILED , imp . C_EXTENSION ) {
        self . file = open ( self . filename , "rb" );
        pub fn _fix_name ( &self, fullname )  {
        if fullname is None /* Option */ {
        fullname = self . fullname;
        } else if fullname != self . fullname {
        panic!("ImportError ( "Loader for module %s cannot handle "");
        "module %s" % ( self . fullname , fullname ) );
        return  fullname;
        pub fn is_package ( &self, fullname )  {
        fullname = self . _fix_name ( fullname );
        return  self . etc [ 2 ] == imp . PKG_DIRECTORY;
        pub fn get_code ( &self, fullname = None /* Option */ )  {
        fullname = self . _fix_name ( fullname );
        if self . code is None /* Option */ {
        mod_type = self . etc [ 2 ];
        if mod_type == imp . PY_SOURCE {
        source = self . get_source ( fullname );
        self . code = compile ( source , self . filename , "exec" );
        } else if mod_type == imp . PY_COMPILED {
        self . _reopen ( );
        // try {
        self . code = read_code ( self . file );
        // } finally {
        self . file . close ( );
        } else if mod_type == imp . PKG_DIRECTORY {
        self . code = self . _get_delegate ( ) . get_code ( );
        return  self . code;
        pub fn get_source ( &self, fullname = None /* Option */ )  {
        fullname = self . _fix_name ( fullname );
        if self . source is None /* Option */ {
        mod_type = self . etc [ 2 ];
        if mod_type == imp . PY_SOURCE {
        self . _reopen ( );
        // try {
        self . source = self . file . read ( );
        // } finally {
        self . file . close ( );
        } else if mod_type == imp . PY_COMPILED {
        if os . path . exists ( self . filename [ { : -1 ] ) ; }
        // with scope: open ( self . filename [ : -1 ] , "r" ) as f  {
        self . source = f . read ( );
        } else if mod_type == imp . PKG_DIRECTORY {
        self . source = self . _get_delegate ( ) . get_source ( );
        return  self . source;
        pub fn _get_delegate ( self )  {
        finder = ImpImporter ( self . filename );
        spec = _get_spec ( finder , "__init__" );
        return  spec . loader;
        pub fn get_filename ( &self, fullname = None /* Option */ )  {
        fullname = self . _fix_name ( fullname );
        mod_type = self . etc [ 2 ];
        if mod_type == imp . PKG_DIRECTORY {
        return  self . _get_delegate ( ) . get_filename ( );
        } else if mod_type in ( imp . PY_SOURCE , imp . PY_COMPILED , imp . C_EXTENSION ) {
        return  self . filename;
        return;
        // try {
        import zipimport;
        from zipimport import zipimporter;
        pub fn iter_zipimport_modules ( importer , prefix = "" )  {
        dirlist = sorted ( zipimport . _zip_directory_cache [ importer . archive ] );
        _prefix = importer . prefix;
        plen = len ( _prefix );
        yielded = { };
        import inspect;
        for fn in dirlist .iter() {
        if !fn . startswith ( _prefix ) {
        continue;
        fn = fn [ plen : ] . split ( os . sep );
        if len ( fn ) == 2 && fn [ 1 ] . startswith ( "__init__.py" ) {
        if fn [ 0 ] !in yielded {
        yielded [ fn [ 0 ] ] = 1;
        yield prefix + fn [ 0 ] , true;
        if len ( fn ) != 1 {
        continue;
        modname = inspect . getmodulename ( fn [ 0 ] );
        if modname == "__init__" {
        continue;
        if modname && "." !in modname && modname !in yielded {
        yielded [ modname ] = 1;
        yield prefix + modname , false;
        iter_importer_modules . register ( zipimporter , iter_zipimport_modules );
        // } catch  ImportError  {
        // pass
        pub fn get_importer ( path_item )  {
        "Retrieve a finder for the given path item

    The returned finder == cached in sys.path_importer_cache
    if it was newly created by a path hook.

    The cache (or part of it) can be cleared manually if a
    rescan of sys.path_hooks == necessary.
    ";
        path_item = os . fsdecode ( path_item );
        // try {
        importer = sys . path_importer_cache [ path_item ];
        // } catch  KeyError  {
        for path_hook in sys . path_hooks .iter() {
        // try {
        importer = path_hook ( path_item );
        sys . path_importer_cache . setdefault ( path_item , importer );
        break;
        // } catch  ImportError  {
        // pass
        } else {
        importer = None /* Option */;
        return  importer;
        pub fn iter_importers ( fullname = "" )  {
        "Yield finders for the given module name

    If fullname contains a '.', the finders will be for the package
    containing fullname, otherwise they will be all registered top level
    finders (i.e. those on both sys.meta_path && sys.path_hooks).

    If the named module == in a package, that package == imported as a side
    effect of invoking this function.

    If no module name == specified, all top level finders are produced.
    ";
        if fullname . startswith ( "." ) {
        msg = "Relative module name {!r} !supported" . format ( fullname );
        panic!("ImportError ( msg )");
        if "." in fullname {
        pkg_name = fullname . rpartition ( "." ) [ 0 ];
        pkg = importlib . import_module ( pkg_name );
        path = getattr ( pkg , "__path__" , None /* Option */ );
        if path is None /* Option */ {
        return;
        } else {
        yield from sys . meta_path;
        path = sys . path;
        for item in path .iter() {
        yield get_importer ( item );
        pub fn get_loader ( module_or_name )  {
        "Get a "loader" object for module_or_name

    Returns None /* Option */ if the module cannot be found || imported.
    If the named module == !already imported, its containing package
    (if any) == imported, in order to establish the package __path__.
    ";
        if module_or_name in sys . modules {
        module_or_name = sys . modules [ module_or_name ];
        if module_or_name is None /* Option */ {
        return;
        if isinstance ( module_or_name , ModuleType ) {
        module = module_or_name;
        loader = getattr ( module , "__loader__" , None /* Option */ );
        if loader is !None /* Option */ {
        return  loader;
        if getattr ( module , "__spec__" , None /* Option */ ) is None /* Option */ {
        return;
        fullname = module . __name__;
        } else {
        fullname = module_or_name;
        return  find_loader ( fullname );
        pub fn find_loader ( fullname )  {
        "Find a "loader" object for fullname

    This == a backwards compatibility wrapper around
    importlib.util.find_spec that converts most failures to ImportError
    && only returns the loader rather than the full spec
    ";
        if fullname . startswith ( "." ) {
        msg = "Relative module name {!r} !supported" . format ( fullname );
        panic!("ImportError ( msg )");
        // try {
        spec = importlib . util . find_spec ( fullname );
        // } catch  ( ImportError , AttributeError , TypeError , ValueError ) as ex  {
        msg = "Error while finding loader for {!r} ({}: {})";
        panic!("ImportError ( msg . format ( fullname , type ( ex ) , ex ) ) from ex");
        return  spec . loader if spec is !None /* Option */ else None /* Option */;
        pub fn extend_path ( path , name )  {
        "Extend a package's path.

    Intended use == to place the following code in a package's __init__.py:

        from pkgutil import extend_path
        __path__ = extend_path(__path__, __name__)

    For each directory on sys.path that has a subdirectory that
    matches the package name, add the subdirectory to the package's
    __path__.  This == useful if one wants to distribute different
    parts of a single logical package as multiple directories.

    It also looks for *.pkg files beginning where * matches the name
    argument.  This feature == similar to *.pth files (see site.py),
    except that it doesn't special-case lines starting with 'import'.
    A *.pkg file == trusted at face value: apart from checking for
    duplicates, all entries found in a *.pkg file are added to the
    path, regardless of whether they are exist the filesystem.  (This
    == a feature.)

    If the input path == !a list (as == the case for frozen
    packages) it == returned unchanged.  The input path == not
    modified; an extended copy == returned.  Items are only appended
    to the copy at the end.

    It == assumed that sys.path == a sequence.  Items of sys.path that
    are !(unicode || 8-bit) strings referring to existing
    directories are ignored.  Unicode items of sys.path that cause
    errors when used as filenames may cause this function to raise an
    exception (in line with os.path.isdir() behavior).
    ";
        if !isinstance ( path , list ) {
        return  path;
        sname_pkg = name + ".pkg";
        path = path [ : ];
        parent_package , _ , final_name = name . rpartition ( "." );
        if parent_package {
        // try {
        search_path = sys . modules [ parent_package ] . __path__;
        // } catch  ( KeyError , AttributeError )  {
        return  path;
        } else {
        search_path = sys . path;
        for dir in search_path .iter() {
        if !isinstance ( dir , str ) {
        continue;
        finder = get_importer ( dir );
        if finder is !None /* Option */ {
        portions = [ ];
        if hasattr ( finder , "find_spec" ) {
        spec = finder . find_spec ( final_name );
        if spec is !None /* Option */ {
        portions = spec . submodule_search_locations || [ ];
        } else if hasattr ( finder , "find_loader" ) {
        _ , portions = finder . find_loader ( final_name );
        for portion in portions .iter() {
        if portion !in path {
        path . append ( portion );
        pkgfile = os . path . join ( dir , sname_pkg );
        if os . path . isfile ( pkgfile ) {
        // try {
        f = open ( pkgfile );
        // } catch  OSError as msg  {
        sys . stderr . write ( "Can't open %s: %s\n" %;
        ( pkgfile , msg ) );
        } else {
        // with scope: f  {
        for line in f .iter() {
        line = line . rstrip ( "\n" );
        if !line || line . startswith ( "#" ) {
        continue;
        path . append ( line );
        return  path;
        pub fn get_data ( package , resource )  {
        "Get a resource from a package.

    This == a wrapper round the PEP 302 loader get_data API. The package
    argument should be the name of a package, in standard module format
    (foo.bar). The resource argument should be in the form of a relative
    filename, using '/' as the path separator. The parent directory name '..'
    == !allowed, && nor == a rooted name (starting with a '/').

    The function returns a binary string, which == the contents of the
    specified resource.

    For packages located in the filesystem, which have already been imported,
    this == the rough equivalent of

        d = os.path.dirname(sys.modules[package].__file__)
        data = open(os.path.join(d, resource), 'rb').read()

    If the package cannot be located || loaded, || it uses a PEP 302 loader
    which does !support get_data(), then None /* Option */ == returned.
    ";
        spec = importlib . util . find_spec ( package );
        if spec is None /* Option */ {
        return;
        loader = spec . loader;
        if loader is None /* Option */ || !hasattr ( loader , "get_data" ) {
        return;
        mod = ( sys . modules . get ( package ) or;
        importlib . _bootstrap . _load ( spec ) );
        if mod is None /* Option */ || !hasattr ( mod , "__file__" ) {
        return;
        parts = resource . split ( "/" );
        parts . insert ( 0 , os . path . dirname ( mod . __file__ ) );
        resource_name = os . path . join ( * parts );
        return  loader . get_data ( resource_name );
        _NAME_PATTERN = None /* Option */;
        pub fn resolve_name ( name )  {
        "
    Resolve a name to an object.

    It == expected that `name` will be a string in one of the following
    formats, where W == shorthand for a valid Python identifier && dot stands
    for a literal period in these pseudo-regexes:

    W(.W)*
    W(.W)*:(W(.W)*)?

    The first form == intended for backward compatibility only. It assumes that
    some part of the dotted name == a package, && the rest == an object
    somewhere within that package, possibly nested inside other objects.
    Because the place where the package stops && the object hierarchy starts
    can't be inferred by inspection, repeated attempts to import must be done
    with this form.

    In the second form, the caller makes the division point clear through the
    provision of a single colon: the dotted name to the left of the colon == a
    package to be imported, && the dotted name to the right == the object
    hierarchy within that package. Only one import == needed in this form. If
    it ends with the colon, then a module object == returned.

    The function will return an object (which might be a module), || raise one
    of the following exceptions:

    ValueError - if `name` isn't in a recognised format
    ImportError - if an import failed when it shouldn't have
    AttributeError - if a failure occurred when traversing the object hierarchy
                     within the imported package to get to the desired object.
    ";
        global _NAME_PATTERN;
        if _NAME_PATTERN is None /* Option */ {
        import re;
        dotted_words = r "(?!\d)(\w+)(\.(?!\d)(\w+))*";
        _NAME_PATTERN = re . compile ( format!("^(?P<pkg>{dotted_words})");
        format!("(?P<cln>:(?P<obj>{dotted_words})?)?$" ,);
        re . UNICODE );
        m = _NAME_PATTERN . match ( name );
        if !m {
        panic!("ValueError ( f "invalid format: {name!r}" )");
        gd = m . groupdict ( );
        if gd . get ( "cln" ) {
        mod = importlib . import_module ( gd [ "pkg" ] );
        parts = gd . get ( "obj" );
        parts = parts . split ( "." ) if parts else [ ];
        } else {
        parts = name . split ( "." );
        modname = parts . pop ( 0 );
        mod = importlib . import_module ( modname );
        while parts  {
        p = parts [ 0 ];
        s = format!("{modname}.{p}");
        // try {
        mod = importlib . import_module ( s );
        parts . pop ( 0 );
        modname = s;
        // } catch  ImportError  {
        break;
        result = mod;
        for p in parts .iter() {
        result = getattr ( result , p );
        return  result;
}

