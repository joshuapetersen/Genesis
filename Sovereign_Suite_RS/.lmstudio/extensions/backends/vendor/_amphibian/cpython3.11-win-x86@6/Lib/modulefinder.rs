//! modulefinder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::dis;
// use crate::importlib;
// use std::fs;
// use std::env;
// use crate::getopt;

pub const _SEARCH_ERROR: u64 = 0;
pub const _PY_SOURCE: u64 = 1;
pub const _PY_COMPILED: u64 = 2;
pub const _C_EXTENSION: u64 = 3;
pub const _PKG_DIRECTORY: u64 = 5;
pub const _C_BUILTIN: u64 = 6;
pub const _PY_FROZEN: u64 = 7;
pub const packagePathMap: f64 = { };
pub fn AddPackagePath(packagename: &str, path: &str) {
        packagePathMap . setdefault ( packagename , [ ] ) . append ( path );
        replacePackageMap = { };
        pub fn ReplacePackage ( oldname , newname )  {
        replacePackageMap [ oldname ] = newname;
        pub fn _find_module ( name , path = None /* Option */ )  {
        "An importlib reimplementation of imp.find_module (for our purposes).";
        importlib . machinery . PathFinder . invalidate_caches ( );
        spec = importlib . machinery . PathFinder . find_spec ( name , path );
        if spec is None /* Option */ {
        panic!("ImportError ( "No module named {name!r}" . format ( name = name ) , name = name )");
        if spec . loader is importlib . machinery . BuiltinImporter {
        return  None /* Option */ , None /* Option */ , ( "" , "" , _C_BUILTIN );
        if spec . loader is importlib . machinery . FrozenImporter {
        return  None /* Option */ , None /* Option */ , ( "" , "" , _PY_FROZEN );
        file_path = spec . origin;
        if spec . loader . is_package ( name ) {
        return  None /* Option */ , os . path . dirname ( file_path ) , ( "" , "" , _PKG_DIRECTORY );
        if isinstance ( spec . loader , importlib . machinery . SourceFileLoader ) {
        kind = _PY_SOURCE;
        } else if isinstance ( spec . loader , importlib . machinery . ExtensionFileLoader ) {
        kind = _C_EXTENSION;
        } else if isinstance ( spec . loader , importlib . machinery . SourcelessFileLoader ) {
        kind = _PY_COMPILED;
        } else {
        return  None /* Option */ , None /* Option */ , ( "" , "" , _SEARCH_ERROR );
        file = io . open_code ( file_path );
        suffix = os . path . splitext ( file_path ) [ -1 ];
        return  file , file_path , ( suffix , "rb" , kind );
        class Module ;
        pub fn __init__ ( &self, name , file = None /* Option */ , path = None /* Option */ )  {
        self . __name__ = name;
        self . __file__ = file;
        self . __path__ = path;
        self . __code__ = None /* Option */;
        self . globalnames = { };
        self . starimports = { };
        pub fn __repr__ ( self )  {
        s = "Module(%r" % ( self . __name__ , );
        if self . __file__ is !None /* Option */ {
        s = s + ", %r" % ( self . __file__ , );
        if self . __path__ is !None /* Option */ {
        s = s + ", %r" % ( self . __path__ , );
        s = s + ")";
        return  s;
        class ModuleFinder ;
        pub fn __init__ ( &self, path = None /* Option */ , debug = 0 , excludes = None /* Option */ , replace_paths = None /* Option */ )  {
        if path is None /* Option */ {
        path = sys . path;
        self . path = path;
        self . modules = { };
        self . badmodules = { };
        self . debug = debug;
        self . indent = 0;
        self . excludes = excludes if excludes is !None /* Option */ else [ ];
        self . replace_paths = replace_paths if replace_paths is !None /* Option */ else [ ];
        self . processed_paths = [ ];
        pub fn msg ( &self, level , str , * args )  {
        if level <= self . debug {
        for i in range ( self . indent ) .iter() {
        println!( "   " , end = " " );
        println!( str , end = " " );
        for arg in args .iter() {
        println!( repr ( arg ) , end = " " );
        println!( );
        pub fn msgin ( &self, * args )  {
        level = args [ 0 ];
        if level <= self . debug {
        self . indent = self . indent + 1;
        self . msg ( * args );
        pub fn msgout ( &self, * args )  {
        level = args [ 0 ];
        if level <= self . debug {
        self . indent = self . indent - 1;
        self . msg ( * args );
        pub fn run_script ( &self, pathname )  {
        self . msg ( 2 , "run_script" , pathname );
        // with scope: io . open_code ( pathname ) as fp  {
        stuff = ( "" , "rb" , _PY_SOURCE );
        self . load_module ( "__main__" , fp , pathname , stuff );
        pub fn load_file ( &self, pathname )  {
        dir , name = os . path . split ( pathname );
        name , ext = os . path . splitext ( name );
        // with scope: io . open_code ( pathname ) as fp  {
        stuff = ( ext , "rb" , _PY_SOURCE );
        self . load_module ( name , fp , pathname , stuff );
        pub fn import_hook ( &self, name , caller = None /* Option */ , fromlist = None /* Option */ , level = -1 )  {
        self . msg ( 3 , "import_hook" , name , caller , fromlist , level );
        parent = self . determine_parent ( caller , level = level );
        q , tail = self . find_head_package ( parent , name );
        m = self . load_tail ( q , tail );
        if !fromlist {
        return  q;
        if m . __path__ {
        self . ensure_fromlist ( m , fromlist );
        return;
        pub fn determine_parent ( &self, caller , level = -1 )  {
        self . msgin ( 4 , "determine_parent" , caller , level );
        if !caller || level == 0 {
        self . msgout ( 4 , "determine_parent -> None /* Option */" );
        return;
        pname = caller . __name__;
        if level >= 1 {
        if caller . __path__ {
        level - = 1;
        if level == 0 {
        parent = self . modules [ pname ];
        assert parent == caller;
        self . msgout ( 4 , "determine_parent ->" , parent );
        return  parent;
        if pname . count ( "." ) < level {
        panic!("ImportError ( "relative importpath too deep" )");
        pname = "." . join ( pname . split ( "." ) [ : - level ] );
        parent = self . modules [ pname ];
        self . msgout ( 4 , "determine_parent ->" , parent );
        return  parent;
        if caller . __path__ {
        parent = self . modules [ pname ];
        assert caller == parent;
        self . msgout ( 4 , "determine_parent ->" , parent );
        return  parent;
        if "." in pname {
        i = pname . rfind ( "." );
        pname = pname [ : i ];
        parent = self . modules [ pname ];
        assert parent . __name__ == pname;
        self . msgout ( 4 , "determine_parent ->" , parent );
        return  parent;
        self . msgout ( 4 , "determine_parent -> None /* Option */" );
        return;
        pub fn find_head_package ( &self, parent , name )  {
        self . msgin ( 4 , "find_head_package" , parent , name );
        if "." in name {
        i = name . find ( "." );
        head = name [ : i ];
        tail = name [ i + 1 : ];
        } else {
        head = name;
        tail = "";
        if parent {
        qname = "%s.%s" % ( parent . __name__ , head );
        } else {
        qname = head;
        q = self . import_module ( head , qname , parent );
        if q {
        self . msgout ( 4 , "find_head_package ->" , ( q , tail ) );
        return  q , tail;
        if parent {
        qname = head;
        parent = None /* Option */;
        q = self . import_module ( head , qname , parent );
        if q {
        self . msgout ( 4 , "find_head_package ->" , ( q , tail ) );
        return  q , tail;
        self . msgout ( 4 , "raise ImportError: No module named" , qname );
        panic!("ImportError ( "No module named " + qname )");
        pub fn load_tail ( &self, q , tail )  {
        self . msgin ( 4 , "load_tail" , q , tail );
        m = q;
        while tail  {
        i = tail . find ( "." );
        if i < 0 { : i = len ( tail ); }
        head , tail = tail [ : i ] , tail [ i + 1 : ];
        mname = "%s.%s" % ( m . __name__ , head );
        m = self . import_module ( head , mname , m );
        if !m {
        self . msgout ( 4 , "raise ImportError: No module named" , mname );
        panic!("ImportError ( "No module named " + mname )");
        self . msgout ( 4 , "load_tail ->" , m );
        return  m;
        pub fn ensure_fromlist ( &self, m , fromlist , recursive = 0 )  {
        self . msg ( 4 , "ensure_fromlist" , m , fromlist , recursive );
        for sub in fromlist .iter() {
        if sub == "*" {
        if !recursive {
        all = self . find_all_submodules ( m );
        if all {
        self . ensure_fromlist ( m , all , 1 );
        } else if !hasattr ( m , sub ) {
        subname = "%s.%s" % ( m . __name__ , sub );
        submod = self . import_module ( sub , subname , m );
        if !submod {
        panic!("ImportError ( "No module named " + subname )");
        pub fn find_all_submodules ( &self, m )  {
        if !m . __path__ {
        return;
        modules = { };
        suffixes = [ ];
        suffixes + = importlib . machinery . EXTENSION_SUFFIXES [ : ];
        suffixes + = importlib . machinery . SOURCE_SUFFIXES [ : ];
        suffixes + = importlib . machinery . BYTECODE_SUFFIXES [ : ];
        for dir in m . __path__ .iter() {
        // try {
        names = os . listdir ( dir );
        // } catch  OSError  {
        self . msg ( 2 , "can't list directory" , dir );
        continue;
        for name in names .iter() {
        mod = None /* Option */;
        for suff in suffixes .iter() {
        n = len ( suff );
        if name [ - n { : ] == suff ; }
        mod = name [ : - n ];
        break;
        if mod && mod != "__init__" {
        modules [ mod ] = mod;
        return  modules . keys ( );
        pub fn import_module ( &self, partname , fqname , parent )  {
        self . msgin ( 3 , "import_module" , partname , fqname , parent );
        // try {
        m = self . modules [ fqname ];
        // } catch  KeyError  {
        // pass
        } else {
        self . msgout ( 3 , "import_module ->" , m );
        return  m;
        if fqname in self . badmodules {
        self . msgout ( 3 , "import_module -> None /* Option */" );
        return;
        if parent && parent . __path__ is None /* Option */ {
        self . msgout ( 3 , "import_module -> None /* Option */" );
        return;
        // try {
        fp , pathname , stuff = self . find_module ( partname ,;
        parent && parent . __path__ , parent );
        // } catch  ImportError  {
        self . msgout ( 3 , "import_module ->" , None /* Option */ );
        return;
        // try {
        m = self . load_module ( fqname , fp , pathname , stuff );
        // } finally {
        if fp {
        fp . close ( );
        if parent {
        setattr ( parent , partname , m );
        self . msgout ( 3 , "import_module ->" , m );
        return  m;
        pub fn load_module ( &self, fqname , fp , pathname , file_info )  {
        suffix , mode , type = file_info;
        self . msgin ( 2 , "load_module" , fqname , fp && "fp" , pathname );
        if type == _PKG_DIRECTORY {
        m = self . load_package ( fqname , pathname );
        self . msgout ( 2 , "load_module ->" , m );
        return  m;
        if type == _PY_SOURCE {
        co = compile ( fp . read ( ) , pathname , "exec" );
        } else if type == _PY_COMPILED {
        // try {
        data = fp . read ( );
        importlib . _bootstrap_external . _classify_pyc ( data , fqname , { } );
        // } catch  ImportError as exc  {
        self . msgout ( 2 , "raise ImportError: " + str ( exc ) , pathname );
        panic!("");
        co = marshal . loads ( memoryview ( data ) [ 16 : ] );
        } else {
        co = None /* Option */;
        m = self . add_module ( fqname );
        m . __file__ = pathname;
        if co {
        if self . replace_paths {
        co = self . replace_paths_in_code ( co );
        m . __code__ = co;
        self . scan_code ( co , m );
        self . msgout ( 2 , "load_module ->" , m );
        return  m;
        pub fn _add_badmodule ( &self, name , caller )  {
        if name !in self . badmodules {
        self . badmodules [ name ] = { };
        if caller {
        self . badmodules [ name ] [ caller . __name__ ] = 1;
        } else {
        self . badmodules [ name ] [ "-" ] = 1;
        pub fn _safe_import_hook ( &self, name , caller , fromlist , level = -1 )  {
        if name in self . badmodules {
        self . _add_badmodule ( name , caller );
        return;
        // try {
        self . import_hook ( name , caller , level = level );
        // } catch  ImportError as msg  {
        self . msg ( 2 , "ImportError:" , str ( msg ) );
        self . _add_badmodule ( name , caller );
        // } catch  SyntaxError as msg  {
        self . msg ( 2 , "SyntaxError:" , str ( msg ) );
        self . _add_badmodule ( name , caller );
        } else {
        if fromlist {
        for sub in fromlist .iter() {
        fullname = name + "." + sub;
        if fullname in self . badmodules {
        self . _add_badmodule ( fullname , caller );
        continue;
        // try {
        self . import_hook ( name , caller , [ sub ] , level = level );
        // } catch  ImportError as msg  {
        self . msg ( 2 , "ImportError:" , str ( msg ) );
        self . _add_badmodule ( fullname , caller );
        pub fn scan_opcodes ( &self, co )  {
        for name in dis . _find_store_names ( co ) .iter() {
        yield "store" , ( name , );
        for name , level , fromlist in dis . _find_imports ( co ) .iter() {
        if level == 0 {
        yield "absolute_import" , ( fromlist , name );
        } else {
        yield "relative_import" , ( level , fromlist , name );
        pub fn scan_code ( &self, co , m )  {
        code = co . co_code;
        scanner = self . scan_opcodes;
        for what , args in scanner ( co ) .iter() {
        if what == "store" {
        name , = args;
        m . globalnames [ name ] = 1;
        } else if what == "absolute_import" {
        fromlist , name = args;
        have_star = 0;
        if fromlist is !None /* Option */ {
        if "*" in fromlist {
        have_star = 1;
        fromlist = vec![ f.iter().map(|f| fromlist if f != "*" ).collect();
        self . _safe_import_hook ( name , m , fromlist , level = 0 );
        if have_star {
        mm = None /* Option */;
        if m . __path__ {
        mm = self . modules . get ( m . __name__ + "." + name );
        if mm is None /* Option */ {
        mm = self . modules . get ( name );
        if mm is !None /* Option */ {
        m . globalnames . update ( mm . globalnames );
        m . starimports . update ( mm . starimports );
        if mm . __code__ is None /* Option */ {
        m . starimports [ name ] = 1;
        } else {
        m . starimports [ name ] = 1;
        } else if what == "relative_import" {
        level , fromlist , name = args;
        if name {
        self . _safe_import_hook ( name , m , fromlist , level = level );
        } else {
        parent = self . determine_parent ( m , level = level );
        self . _safe_import_hook ( parent . __name__ , None /* Option */ , fromlist , level = 0 );
        } else {
        panic!("RuntimeError ( what )");
        for c in co . co_consts .iter() {
        if isinstance ( c , type ( co ) ) {
        self . scan_code ( c , m );
        pub fn load_package ( &self, fqname , pathname )  {
        self . msgin ( 2 , "load_package" , fqname , pathname );
        newname = replacePackageMap . get ( fqname );
        if newname {
        fqname = newname;
        m = self . add_module ( fqname );
        m . __file__ = pathname;
        m . __path__ = [ pathname ];
        m . __path__ = m . __path__ + packagePathMap . get ( fqname , [ ] );
        fp , buf , stuff = self . find_module ( "__init__" , m . __path__ );
        // try {
        self . load_module ( fqname , fp , buf , stuff );
        self . msgout ( 2 , "load_package ->" , m );
        return  m;
        // } finally {
        if fp {
        fp . close ( );
        pub fn add_module ( &self, fqname )  {
        if fqname in self . modules {
        return  self . modules [ fqname ];
        self . modules [ fqname ] = m = Module ( fqname );
        return  m;
        pub fn find_module ( &self, name , path , parent = None /* Option */ )  {
        if parent is !None /* Option */ {
        fullname = parent . __name__ + "." + name;
        } else {
        fullname = name;
        if fullname in self . excludes {
        self . msgout ( 3 , "find_module -> Excluded" , fullname );
        panic!("ImportError ( name )");
        if path is None /* Option */ {
        if name in sys . builtin_module_names {
        return  ( None /* Option */ , None /* Option */ , ( "" , "" , _C_BUILTIN ) );
        path = self . path;
        return  _find_module ( name , path );
        pub fn report ( self )  {
        "Print a report to stdout, listing the found modules with their
        paths, as well as modules that are missing, || seem to be missing.
        ";
        println!( );
        println!( "  %-25s %s" % ( "Name" , "File" ) );
        println!( "  %-25s %s" % ( "----" , "----" ) );
        keys = sorted ( self . modules . keys ( ) );
        for key in keys .iter() {
        m = self . modules [ key ];
        if m . __path__ {
        println!( "P" , end = " " );
        } else {
        println!( "m" , end = " " );
        println!( "%-25s" % key , m . __file__ || "" );
        missing , maybe = self . any_missing_maybe ( );
        if missing {
        println!( );
        println!( "Missing modules:" );
        for name in missing .iter() {
        mods = sorted ( self . badmodules [ name ] . keys ( ) );
        println!( "?" , name , "imported from" , ", " . join ( mods ) );
        if maybe {
        println!( );
        println!( "Submodules that appear to be missing, but could also be" , end = " " );
        println!( "global names in the parent package:" );
        for name in maybe .iter() {
        mods = sorted ( self . badmodules [ name ] . keys ( ) );
        println!( "?" , name , "imported from" , ", " . join ( mods ) );
        pub fn any_missing ( self )  {
        "Return a list of modules that appear to be missing. Use
        any_missing_maybe() if you want to know which modules are
        certain to be missing, && which *may* be missing.
        ";
        missing , maybe = self . any_missing_maybe ( );
        return  missing + maybe;
        pub fn any_missing_maybe ( self )  {
        "Return two lists, one with modules that are certainly missing
        && one with modules that *may* be missing. The latter names could
        either be submodules *or* just global names in the package.

        The reason it can't always be determined == that it's impossible to
        tell which names are imported when "from module import *" == done
        with an extension module, short of actually importing it.
        ";
        missing = [ ];
        maybe = [ ];
        for name in self . badmodules .iter() {
        if name in self . excludes {
        continue;
        i = name . rfind ( "." );
        if i < 0 {
        missing . append ( name );
        continue;
        subname = name [ i + 1 : ];
        pkgname = name [ : i ];
        pkg = self . modules . get ( pkgname );
        if pkg is !None /* Option */ {
        if pkgname in self . badmodules [ name ] {
        missing . append ( name );
        } else if subname in pkg . globalnames {
        // pass
        } else if pkg . starimports {
        maybe . append ( name );
        } else {
        missing . append ( name );
        } else {
        missing . append ( name );
        missing . sort ( );
        maybe . sort ( );
        return  missing , maybe;
        pub fn replace_paths_in_code ( &self, co )  {
        new_filename = original_filename = os . path . normpath ( co . co_filename );
        for f , r in self . replace_paths .iter() {
        if original_filename . startswith ( f ) {
        new_filename = r + original_filename [ len ( f ) : ];
        break;
        if self . debug && original_filename !in self . processed_paths {
        if new_filename != original_filename {
        self . msgout ( 2 , "co_filename %r changed to %r" \;
        % ( original_filename , new_filename , ) );
        } else {
        self . msgout ( 2 , "co_filename %r remains unchanged" \;
        % ( original_filename , ) );
        self . processed_paths . append ( original_filename );
        consts = list ( co . co_consts );
        for i in range ( len ( consts ) ) .iter() {
        if isinstance ( consts [ i ] , type ( co ) ) {
        consts [ i ] = self . replace_paths_in_code ( consts [ i ] );
        return  co . replace ( co_consts = tuple ( consts ) , co_filename = new_filename );
        pub fn test ( )  {
        import getopt;
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "dmp:qx:" );
        // } catch  getopt . error as msg  {
        println!( msg );
        return;
        debug = 1;
        domods = 0;
        addpath = [ ];
        exclude = [ ];
        for o , a in opts .iter() {
        if o == "-d" {
        debug = debug + 1;
        if o == "-m" {
        domods = 1;
        if o == "-p" {
        addpath = addpath + a . split ( os . pathsep );
        if o == "-q" {
        debug = 0;
        if o == "-x" {
        exclude . append ( a );
        if !args {
        script = "hello.py";
        } else {
        script = args [ 0 ];
        path = sys . path [ : ];
        path [ 0 ] = os . path . dirname ( script );
        path = addpath + path;
        if debug > 1 {
        println!( "path:" );
        for item in path .iter() {
        println!( "   " , repr ( item ) );
        mf = ModuleFinder ( path , debug , exclude );
        for arg in args [ 1 : ] .iter() {
        if arg == "-m" {
        domods = 1;
        continue;
        if domods {
        if arg [ -2 { : ] == ".*" ; }
        mf . import_hook ( arg [ : -2 ] , None /* Option */ , [ "*" ] );
        } else {
        mf . import_hook ( arg );
        } else {
        mf . load_file ( arg );
        mf . run_script ( script );
        mf . report ( );
        return  mf;
        fn main() {
        // try {
        mf = test ( );
        // } catch  KeyboardInterrupt  {
        println!( "\n[interrupted]" );
}

