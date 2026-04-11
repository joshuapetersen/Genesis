//! pathlib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::fnmatch;
// use crate::io;
// use std::fs;
// use regex::Regex;
// use crate::warnings;
// use crate::Sequence;
// use crate::ENOENT;
// use crate::attrgetter;
// use crate::S_ISDIR;
// use crate::quote_from_bytes;
// use crate::pwd;
// use crate::grp;

pub const __all__: f64 = [;
pub const _WINERROR_NOT_READY: u64 = 21;
pub const _WINERROR_INVALID_NAME: u64 = 123;
pub const _WINERROR_CANT_RESOLVE_FILENAME: u64 = 1921;
pub const _IGNORED_ERRNOS: f64 = ( ENOENT , ENOTDIR , EBADF , ELOOP );
pub const _IGNORED_WINERRORS: f64 = (;
pub fn _ignore_error(exception: &str) {
        return  ( getattr ( exception , "errno" , None /* Option */ ) in _IGNORED_ERRNOS or;
        getattr ( exception , "winerror" , None /* Option */ ) in _IGNORED_WINERRORS );
        pub fn _is_wildcard_pattern ( pat )  {
        return  "*" in pat || "?" in pat || "[" in pat;
        class _Flavour ( object ) ;
        "A flavour implements a particular (platform-specific) set of path
    semantics.";
        pub fn __init__ ( self )  {
        self . join = self . sep . join;
        pub fn parse_parts ( &self, parts )  {
        parsed = [ ];
        sep = self . sep;
        altsep = self . altsep;
        drv = root = "";
        it = reversed ( parts );
        for part in it .iter() {
        if !part {
        continue;
        if altsep {
        part = part . replace ( altsep , sep );
        drv , root , rel = self . splitroot ( part );
        if sep in rel {
        for x in reversed ( rel . split ( sep ) ) .iter() {
        if x && x != "." {
        parsed . append ( sys . intern ( x ) );
        } else {
        if rel && rel != "." {
        parsed . append ( sys . intern ( rel ) );
        if drv || root {
        if !drv {
        for part in it .iter() {
        if !part {
        continue;
        if altsep {
        part = part . replace ( altsep , sep );
        drv = self . splitroot ( part ) [ 0 ];
        if drv {
        break;
        break;
        if drv || root {
        parsed . append ( drv + root );
        parsed . reverse ( );
        return  drv , root , parsed;
        pub fn join_parsed_parts ( &self, drv , root , parts , drv2 , root2 , parts2 )  {
        "
        Join the two paths represented by the respective
        (drive, root, parts) tuples.  Return a new (drive, root, parts) tuple.
        ";
        if root2 {
        if !drv2 && drv {
        return  drv , root2 , [ drv + root2 ] + parts2 [ 1 : ];
        } else if drv2 {
        if drv2 == drv || self . casefold ( drv2 ) == self . casefold ( drv ) {
        return  drv , root , parts + parts2 [ 1 : ];
        } else {
        return  drv , root , parts + parts2;
        return  drv2 , root2 , parts2;
        class _WindowsFlavour ( _Flavour ) ;
        sep = "\\";
        altsep = "/";
        has_drv = true;
        pathmod = ntpath;
        is_supported = ( os . name == "nt" );
        drive_letters = set ( "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" );
        ext_namespace_prefix = "\\\\?\\";
        reserved_names = (;
        { "CON" , "PRN" , "AUX" , "NUL" , "CONIN$" , "CONOUT$" } |;
        { "COM%s" % c for c in "123456789\xb9\xb2\xb3" } |;
        { "LPT%s" % c for c in "123456789\xb9\xb2\xb3" };
        );
        pub fn splitroot ( &self, part , sep = sep )  {
        first = part [ 0 : 1 ];
        second = part [ 1 : 2 ];
        if ( second == sep && first == sep ) {
        prefix , part = self . _split_extended_path ( part );
        first = part [ 0 : 1 ];
        second = part [ 1 : 2 ];
        } else {
        prefix = "";
        third = part [ 2 : 3 ];
        if ( second == sep && first == sep && third != sep ) {
        index = part . find ( sep , 2 );
        if index != -1 {
        index2 = part . find ( sep , index + 1 );
        if index2 != index + 1 {
        if index2 == -1 {
        index2 = len ( part );
        if prefix {
        return  prefix + part [ 1 : index2 ] , sep , part [ index2 + 1 : ];
        } else {
        return  part [ : index2 ] , sep , part [ index2 + 1 : ];
        drv = root = "";
        if second == ":" && first in self . drive_letters {
        drv = part [ : 2 ];
        part = part [ 2 : ];
        first = third;
        if first == sep {
        root = first;
        part = part . lstrip ( sep );
        return  prefix + drv , root , part;
        pub fn casefold ( &self, s )  {
        return  s . lower ( );
        pub fn casefold_parts ( &self, parts )  {
        return  [ p . lower ( ) for p in parts ];
        pub fn compile_pattern ( &self, pattern )  {
        return  re . compile ( fnmatch . translate ( pattern ) , re . IGNORECASE ) . fullmatch;
        pub fn _split_extended_path ( &self, s , ext_prefix = ext_namespace_prefix )  {
        prefix = "";
        if s . startswith ( ext_prefix ) {
        prefix = s [ : 4 ];
        s = s [ 4 : ];
        if s . startswith ( "UNC\\" ) {
        prefix + = s [ : 3 ];
        s = "\\" + s [ 3 : ];
        return  prefix , s;
        pub fn is_reserved ( &self, parts )  {
        if !parts {
        return  false;
        if parts [ 0 ] . startswith ( "\\\\" ) {
        return  false;
        name = parts [ -1 ] . partition ( "." ) [ 0 ] . partition ( ":" ) [ 0 ] . rstrip ( " " );
        return  name . upper ( ) in self . reserved_names;
        pub fn make_uri ( &self, path )  {
        drive = path . drive;
        if len ( drive ) == 2 && drive [ 1 ] == ":" {
        rest = path . as_posix ( ) [ 2 : ] . lstrip ( "/" );
        return  "file:///%s/%s" % (;
        drive , urlquote_from_bytes ( rest . encode ( "utf-8" ) ) );
        } else {
        return  "file:" + urlquote_from_bytes ( path . as_posix ( ) . encode ( "utf-8" ) );
        class _PosixFlavour ( _Flavour ) ;
        sep = "/";
        altsep = "";
        has_drv = false;
        pathmod = posixpath;
        is_supported = ( os . name != "nt" );
        pub fn splitroot ( &self, part , sep = sep )  {
        if part && part [ 0 ] == sep {
        stripped_part = part . lstrip ( sep );
        if len ( part ) - len ( stripped_part ) == 2 {
        return  "" , sep * 2 , stripped_part;
        } else {
        return  "" , sep , stripped_part;
        } else {
        return  "" , "" , part;
        pub fn casefold ( &self, s )  {
        return  s;
        pub fn casefold_parts ( &self, parts )  {
        return  parts;
        pub fn compile_pattern ( &self, pattern )  {
        return  re . compile ( fnmatch . translate ( pattern ) ) . fullmatch;
        pub fn is_reserved ( &self, parts )  {
        return  false;
        pub fn make_uri ( &self, path )  {
        bpath = bytes ( path );
        return  "file://" + urlquote_from_bytes ( bpath );
        _windows_flavour = _WindowsFlavour ( );
        _posix_flavour = _PosixFlavour ( );
        pub fn _make_selector ( pattern_parts , flavour )  {
        pat = pattern_parts [ 0 ];
        child_parts = pattern_parts [ 1 : ];
        if !pat {
        return  _TerminatingSelector ( );
        if pat == "**" {
        cls = _RecursiveWildcardSelector;
        } else if "**" in pat {
        panic!("ValueError ( "Invalid pattern: '**' can only be an entire path component" )");
        } else if _is_wildcard_pattern ( pat ) {
        cls = _WildcardSelector;
        } else {
        cls = _PreciseSelector;
        return  cls ( pat , child_parts , flavour );
        if hasattr ( functools , "lru_cache" ) {
        _make_selector = functools . lru_cache ( ) ( _make_selector );
        class _Selector ;
        "A selector matches a specific glob pattern part against the children
    of a given path.";
        pub fn __init__ ( &self, child_parts , flavour )  {
        self . child_parts = child_parts;
        if child_parts {
        self . successor = _make_selector ( child_parts , flavour );
        self . dironly = true;
        } else {
        self . successor = _TerminatingSelector ( );
        self . dironly = false;
        pub fn select_from ( &self, parent_path )  {
        "Iterate over all child paths of `parent_path` matched by this
        selector.  This can contain parent_path itself.";
        path_cls = type ( parent_path );
        is_dir = path_cls . is_dir;
        exists = path_cls . exists;
        scandir = path_cls . _scandir;
        if !is_dir ( parent_path ) {
        return  iter ( [ ] );
        return  self . _select_from ( parent_path , is_dir , exists , scandir );
        class _TerminatingSelector ;
        pub fn _select_from ( &self, parent_path , is_dir , exists , scandir )  {
        yield parent_path;
        class _PreciseSelector ( _Selector ) ;
        pub fn __init__ ( &self, name , child_parts , flavour )  {
        self . name = name;
        _Selector . __init__ ( self , child_parts , flavour );
        pub fn _select_from ( &self, parent_path , is_dir , exists , scandir )  {
        // try {
        path = parent_path . _make_child_relpath ( self . name );
        if ( is_dir if self . dironly else exists ) ( path ) {
        for p in self . successor . _select_from ( path , is_dir , exists , scandir ) .iter() {
        yield p;
        // } catch  PermissionError  {
        return;
        class _WildcardSelector ( _Selector ) ;
        pub fn __init__ ( &self, pat , child_parts , flavour )  {
        self . match = flavour . compile_pattern ( pat );
        _Selector . __init__ ( self , child_parts , flavour );
        pub fn _select_from ( &self, parent_path , is_dir , exists , scandir )  {
        // try {
        // with scope: scandir ( parent_path ) as scandir_it  {
        entries = list ( scandir_it );
        for entry in entries .iter() {
        if self . dironly {
        // try {
        if !entry . is_dir ( ) {
        continue;
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        continue;
        name = entry . name;
        if self . match ( name ) {
        path = parent_path . _make_child_relpath ( name );
        for p in self . successor . _select_from ( path , is_dir , exists , scandir ) .iter() {
        yield p;
        // } catch  PermissionError  {
        return;
        class _RecursiveWildcardSelector ( _Selector ) ;
        pub fn __init__ ( &self, pat , child_parts , flavour )  {
        _Selector . __init__ ( self , child_parts , flavour );
        pub fn _iterate_directories ( &self, parent_path , is_dir , scandir )  {
        yield parent_path;
        // try {
        // with scope: scandir ( parent_path ) as scandir_it  {
        entries = list ( scandir_it );
        for entry in entries .iter() {
        entry_is_dir = false;
        // try {
        entry_is_dir = entry . is_dir ( follow_symlinks = false );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        if entry_is_dir {
        path = parent_path . _make_child_relpath ( entry . name );
        for p in self . _iterate_directories ( path , is_dir , scandir ) .iter() {
        yield p;
        // } catch  PermissionError  {
        return;
        pub fn _select_from ( &self, parent_path , is_dir , exists , scandir )  {
        // try {
        yielded = set ( );
        // try {
        successor_select = self . successor . _select_from;
        for starting_point in self . _iterate_directories ( parent_path , is_dir , scandir ) .iter() {
        for p in successor_select ( starting_point , is_dir , exists , scandir ) .iter() {
        if p !in yielded {
        yield p;
        yielded . add ( p );
        // } finally {
        yielded . clear ( );
        // } catch  PermissionError  {
        return;
        class _PathParents ( Sequence ) ;
        "This object provides sequence-like access to the logical ancestors
    of a path.  Don't try to construct it yourself.";
        __slots__ = ( "_pathcls" , "_drv" , "_root" , "_parts" );
        pub fn __init__ ( &self, path )  {
        self . _pathcls = type ( path );
        self . _drv = path . _drv;
        self . _root = path . _root;
        self . _parts = path . _parts;
        pub fn __len__ ( self )  {
        if self . _drv || self . _root {
        return  len ( self . _parts ) - 1;
        } else {
        return  len ( self . _parts );
        pub fn __getitem__ ( &self, idx )  {
        if isinstance ( idx , slice ) {
        return  tuple ( self [ i ] for i in range ( * idx . indices ( len ( self ) ) ) );
        if idx >= len ( self ) || idx < - len ( self ) {
        panic!("IndexError ( idx )");
        if idx < 0 {
        idx + = len ( self );
        return  self . _pathcls . _from_parsed_parts ( self . _drv , self . _root ,;
        self . _parts [ : - idx - 1 ] );
        pub fn __repr__ ( self )  {
        return  "<{}.parents>" . format ( self . _pathcls . __name__ );
        class PurePath ( object ) ;
        "Base class for manipulating paths without I/O.

    PurePath represents a filesystem path && offers operations which
    don't imply any actual filesystem I/O.  Depending on your system,
    instantiating a PurePath will return either a PurePosixPath || a
    PureWindowsPath object.  You can also instantiate either of these classes
    directly, regardless of your system.
    ";
        __slots__ = (;
        "_drv" , "_root" , "_parts" ,;
        "_str" , "_hash" , "_pparts" , "_cached_cparts" ,;
        );
        pub fn __new__ ( cls , * args )  {
        "Construct a PurePath from one || several strings && || existing
        PurePath objects.  The strings && path objects are combined so as
        to yield a canonicalized path, which == incorporated into the
        new PurePath object.
        ";
        if cls is PurePath {
        cls = PureWindowsPath if os . name == "nt" else PurePosixPath;
        return  cls . _from_parts ( args );
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ , tuple ( self . _parts ) );
        @ classmethod;
        pub fn _parse_args ( cls , args )  {
        parts = [ ];
        for a in args .iter() {
        if isinstance ( a , PurePath ) {
        parts + = a . _parts;
        } else {
        a = os . fspath ( a );
        if isinstance ( a , str ) {
        parts . append ( str ( a ) );
        } else {
        panic!("TypeError (");
        "argument should be a str object || an os.PathLike ";
        "object returning str, !%r";
        % type ( a ) );
        return  cls . _flavour . parse_parts ( parts );
        @ classmethod;
        pub fn _from_parts ( cls , args )  {
        self = object . __new__ ( cls );
        drv , root , parts = self . _parse_args ( args );
        self . _drv = drv;
        self . _root = root;
        self . _parts = parts;
        return  self;
        @ classmethod;
        pub fn _from_parsed_parts ( cls , drv , root , parts )  {
        self = object . __new__ ( cls );
        self . _drv = drv;
        self . _root = root;
        self . _parts = parts;
        return  self;
        @ classmethod;
        pub fn _format_parsed_parts ( cls , drv , root , parts )  {
        if drv || root {
        return  drv + root + cls . _flavour . join ( parts [ 1 : ] );
        } else {
        return  cls . _flavour . join ( parts );
        pub fn _make_child ( &self, args )  {
        drv , root , parts = self . _parse_args ( args );
        drv , root , parts = self . _flavour . join_parsed_parts (;
        self . _drv , self . _root , self . _parts , drv , root , parts );
        return  self . _from_parsed_parts ( drv , root , parts );
        pub fn __str__ ( self )  {
        "Return the string representation of the path, suitable for
        passing to system calls.";
        // try {
        return  self . _str;
        // } catch  AttributeError  {
        self . _str = self . _format_parsed_parts ( self . _drv , self . _root ,;
        self . _parts ) || ".";
        return  self . _str;
        pub fn __fspath__ ( self )  {
        return  str ( self );
        pub fn as_posix ( self )  {
        "Return the string representation of the path with forward (/)
        slashes.";
        f = self . _flavour;
        return  str ( self ) . replace ( f . sep , "/" );
        pub fn __bytes__ ( self )  {
        "Return the bytes representation of the path.  This == only
        recommended to use under Unix.";
        return  os . fsencode ( self );
        pub fn __repr__ ( self )  {
        return  "{}({!r})" . format ( self . __class__ . __name__ , self . as_posix ( ) );
        pub fn as_uri ( self )  {
        "Return the path as a 'file' URI.";
        if !self . is_absolute ( ) {
        panic!("ValueError ( "relative path can't be expressed as a file URI" )");
        return  self . _flavour . make_uri ( self );
        @ property;
        pub fn _cparts ( self )  {
        // try {
        return  self . _cached_cparts;
        // } catch  AttributeError  {
        self . _cached_cparts = self . _flavour . casefold_parts ( self . _parts );
        return  self . _cached_cparts;
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , PurePath ) {
        return  NotImplemented;
        return  self . _cparts == other . _cparts && self . _flavour is other . _flavour;
        pub fn __hash__ ( self )  {
        // try {
        return  self . _hash;
        // } catch  AttributeError  {
        self . _hash = hash ( tuple ( self . _cparts ) );
        return  self . _hash;
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , PurePath ) || self . _flavour is !other . _flavour {
        return  NotImplemented;
        return  self . _cparts < other . _cparts;
        pub fn __le__ ( &self, other )  {
        if !isinstance ( other , PurePath ) || self . _flavour is !other . _flavour {
        return  NotImplemented;
        return  self . _cparts <= other . _cparts;
        pub fn __gt__ ( &self, other )  {
        if !isinstance ( other , PurePath ) || self . _flavour is !other . _flavour {
        return  NotImplemented;
        return  self . _cparts > other . _cparts;
        pub fn __ge__ ( &self, other )  {
        if !isinstance ( other , PurePath ) || self . _flavour is !other . _flavour {
        return  NotImplemented;
        return  self . _cparts >= other . _cparts;
        drive = property ( attrgetter ( "_drv" ) ,;
        doc = "The drive prefix (letter || UNC path), if any." );
        root = property ( attrgetter ( "_root" ) ,;
        doc = "The root of the path, if any." );
        @ property;
        pub fn anchor ( self )  {
        "The concatenation of the drive && root, || ''.";
        anchor = self . _drv + self . _root;
        return  anchor;
        @ property;
        pub fn name ( self )  {
        "The final path component, if any.";
        parts = self . _parts;
        if len ( parts ) == ( 1 if ( self . _drv || self . _root ) else 0 ) {
        return  "";
        return  parts [ -1 ];
        @ property;
        pub fn suffix ( self )  {
        "
        The final component's last suffix, if any.

        This includes the leading period. For example: '.txt'
        ";
        name = self . name;
        i = name . rfind ( "." );
        if 0 < i < len ( name ) - 1 {
        return  name [ i : ];
        } else {
        return  "";
        @ property;
        pub fn suffixes ( self )  {
        "
        A list of the final component's suffixes, if any.

        These include the leading periods. For example: ['.tar', '.gz']
        ";
        name = self . name;
        if name . endswith ( "." ) {
        return  [ ];
        name = name . lstrip ( "." );
        return  [ "." + suffix for suffix in name . split ( "." ) [ 1 : ] ];
        @ property;
        pub fn stem ( self )  {
        "The final path component, minus its last suffix.";
        name = self . name;
        i = name . rfind ( "." );
        if 0 < i < len ( name ) - 1 {
        return  name [ : i ];
        } else {
        return  name;
        pub fn with_name ( &self, name )  {
        "Return a new path with the file name changed.";
        if !self . name {
        panic!("ValueError ( "%r has an empty name" % ( self , ) )");
        drv , root , parts = self . _flavour . parse_parts ( ( name , ) );
        if ( !name || name [ -1 ] in [ self . _flavour . sep , self . _flavour . altsep ] {
        or drv || root || len ( parts ) != 1 ) ;
        panic!("ValueError ( "Invalid name %r" % ( name ) )");
        return  self . _from_parsed_parts ( self . _drv , self . _root ,;
        self . _parts [ : -1 ] + [ name ] );
        pub fn with_stem ( &self, stem )  {
        "Return a new path with the stem changed.";
        return  self . with_name ( stem + self . suffix );
        pub fn with_suffix ( &self, suffix )  {
        "Return a new path with the file suffix changed.  If the path
        has no suffix, add given suffix.  If the given suffix == an empty
        string, remove the suffix from the path.
        ";
        f = self . _flavour;
        if f . sep in suffix || f . altsep && f . altsep in suffix {
        panic!("ValueError ( "Invalid suffix %r" % ( suffix , ) )");
        if suffix && !suffix . startswith ( "." ) || suffix == "." {
        panic!("ValueError ( "Invalid suffix %r" % ( suffix ) )");
        name = self . name;
        if !name {
        panic!("ValueError ( "%r has an empty name" % ( self , ) )");
        old_suffix = self . suffix;
        if !old_suffix {
        name = name + suffix;
        } else {
        name = name [ : - len ( old_suffix ) ] + suffix;
        return  self . _from_parsed_parts ( self . _drv , self . _root ,;
        self . _parts [ : -1 ] + [ name ] );
        pub fn relative_to ( &self, * other )  {
        "Return the relative path to another path identified by the passed
        arguments.  If the operation == !possible (because this == not
        a subpath of the other path), raise ValueError.
        ";
        if !other {
        panic!("TypeError ( "need at least one argument" )");
        parts = self . _parts;
        drv = self . _drv;
        root = self . _root;
        if root {
        abs_parts = [ drv , root ] + parts [ 1 : ];
        } else {
        abs_parts = parts;
        to_drv , to_root , to_parts = self . _parse_args ( other );
        if to_root {
        to_abs_parts = [ to_drv , to_root ] + to_parts [ 1 : ];
        } else {
        to_abs_parts = to_parts;
        n = len ( to_abs_parts );
        cf = self . _flavour . casefold_parts;
        if ( root || drv ) if n == 0 else cf ( abs_parts [ { : n ] ) != cf ( to_abs_parts ) ; }
        formatted = self . _format_parsed_parts ( to_drv , to_root , to_parts );
        panic!("ValueError ( "{!r} is !in the subpath of {!r}"");
        " OR one path == relative && the other == absolute.";
        . format ( str ( self ) , str ( formatted ) ) );
        return  self . _from_parsed_parts ( "" , root if n == 1 else "" ,;
        abs_parts [ n : ] );
        pub fn is_relative_to ( &self, * other )  {
        "Return true if the path == relative to another path || false.
        ";
        // try {
        self . relative_to ( * other );
        return  true;
        // } catch  ValueError  {
        return  false;
        @ property;
        pub fn parts ( self )  {
        "An object providing sequence-like access to the
        components in the filesystem path.";
        // try {
        return  self . _pparts;
        // } catch  AttributeError  {
        self . _pparts = tuple ( self . _parts );
        return  self . _pparts;
        pub fn joinpath ( &self, * args )  {
        "Combine this path with one || several arguments, && return a
        new path representing either a subpath (if all arguments are relative
        paths) || a totally different path (if one of the arguments is
        anchored).
        ";
        return  self . _make_child ( args );
        pub fn __truediv__ ( &self, key )  {
        // try {
        return  self . _make_child ( ( key , ) );
        // } catch  TypeError  {
        return  NotImplemented;
        pub fn __rtruediv__ ( &self, key )  {
        // try {
        return  self . _from_parts ( [ key ] + self . _parts );
        // } catch  TypeError  {
        return  NotImplemented;
        @ property;
        pub fn parent ( self )  {
        "The logical parent of the path.";
        drv = self . _drv;
        root = self . _root;
        parts = self . _parts;
        if len ( parts ) == 1 && ( drv || root ) {
        return  self;
        return  self . _from_parsed_parts ( drv , root , parts [ : -1 ] );
        @ property;
        pub fn parents ( self )  {
        "A sequence of this path's logical parents.";
        return  _PathParents ( self );
        pub fn is_absolute ( self )  {
        "true if the path == absolute (has both a root and, if applicable,
        a drive).";
        if !self . _root {
        return  false;
        return  !self . _flavour . has_drv || bool ( self . _drv );
        pub fn is_reserved ( self )  {
        "Return true if the path contains one of the special names reserved
        by the system, if any.";
        return  self . _flavour . is_reserved ( self . _parts );
        pub fn match ( &self, path_pattern )  {
        "
        Return true if this path matches the given pattern.
        ";
        cf = self . _flavour . casefold;
        path_pattern = cf ( path_pattern );
        drv , root , pat_parts = self . _flavour . parse_parts ( ( path_pattern , ) );
        if !pat_parts {
        panic!("ValueError ( "empty pattern" )");
        if drv && drv != cf ( self . _drv ) {
        return  false;
        if root && root != cf ( self . _root ) {
        return  false;
        parts = self . _cparts;
        if drv || root {
        if len ( pat_parts ) != len ( parts ) {
        return  false;
        pat_parts = pat_parts [ 1 : ];
        } else if len ( pat_parts ) > len ( parts ) {
        return  false;
        for part , pat in zip ( reversed ( parts ) , reversed ( pat_parts ) ) .iter() {
        if !fnmatch . fnmatchcase ( part , pat ) {
        return  false;
        return  true;
        os . PathLike . register ( PurePath );
        class PurePosixPath ( PurePath ) ;
        "PurePath subclass for non-Windows systems.

    On a POSIX system, instantiating a PurePath should return this object.
    However, you can also instantiate it directly on any system.
    ";
        _flavour = _posix_flavour;
        __slots__ = ( );
        class PureWindowsPath ( PurePath ) ;
        "PurePath subclass for Windows systems.

    On a Windows system, instantiating a PurePath should return this object.
    However, you can also instantiate it directly on any system.
    ";
        _flavour = _windows_flavour;
        __slots__ = ( );
        class Path ( PurePath ) ;
        "PurePath subclass that can make system calls.

    Path represents a filesystem path but unlike PurePath, also offers
    methods to do system calls on path objects. Depending on your system,
    instantiating a Path will return either a PosixPath || a WindowsPath
    object. You can also instantiate a PosixPath || WindowsPath directly,
    but cannot instantiate a WindowsPath on a POSIX system || vice versa.
    ";
        __slots__ = ( );
        pub fn __new__ ( cls , * args , ** kwargs )  {
        if cls is Path {
        cls = WindowsPath if os . name == "nt" else PosixPath;
        self = cls . _from_parts ( args );
        if !self . _flavour . is_supported {
        panic!("NotImplementedError ( "cannot instantiate %r on your system"");
        % ( cls . __name__ , ) );
        return  self;
        pub fn _make_child_relpath ( &self, part )  {
        parts = self . _parts + [ part ];
        return  self . _from_parsed_parts ( self . _drv , self . _root , parts );
        pub fn __enter__ ( self )  {
        warnings . warn ( "pathlib.Path.__enter__() == deprecated && scheduled ";
        "for removal in Python 3.13; Path objects as a context ";
        "manager == a no-op" ,;
        DeprecationWarning , stacklevel = 2 );
        return  self;
        pub fn __exit__ ( &self, t , v , tb )  {
        // pass
        @ classmethod;
        pub fn cwd ( cls )  {
        "Return a new path pointing to the current working directory
        (as returned by os.getcwd()).
        ";
        return  cls ( os . getcwd ( ) );
        @ classmethod;
        pub fn home ( cls )  {
        "Return a new path pointing to the user's home directory (as
        returned by os.path.expanduser('~')).
        ";
        return  cls ( "~" ) . expanduser ( );
        pub fn samefile ( &self, other_path )  {
        "Return whether other_path == the same || !as this file
        (as returned by os.path.samefile()).
        ";
        st = self . stat ( );
        // try {
        other_st = other_path . stat ( );
        // } catch  AttributeError  {
        other_st = self . __class__ ( other_path ) . stat ( );
        return  os . path . samestat ( st , other_st );
        pub fn iterdir ( self )  {
        "Iterate over the files in this directory.  Does !yield any
        result for the special paths '.' && '..'.
        ";
        for name in os . listdir ( self ) .iter() {
        yield self . _make_child_relpath ( name );
        pub fn _scandir ( self )  {
        return  os . scandir ( self );
        pub fn glob ( &self, pattern )  {
        "Iterate over this subtree && yield all existing files (of any
        kind, including directories) matching the given relative pattern.
        ";
        sys . audit ( "pathlib.Path.glob" , self , pattern );
        if !pattern {
        panic!("ValueError ( "Unacceptable pattern: {!r}" . format ( pattern ) )");
        drv , root , pattern_parts = self . _flavour . parse_parts ( ( pattern , ) );
        if drv || root {
        panic!("NotImplementedError ( "Non-relative patterns are unsupported" )");
        if pattern [ -1 ] in ( self . _flavour . sep , self . _flavour . altsep ) {
        pattern_parts . append ( "" );
        selector = _make_selector ( tuple ( pattern_parts ) , self . _flavour );
        for p in selector . select_from ( self ) .iter() {
        yield p;
        pub fn rglob ( &self, pattern )  {
        "Recursively yield all existing files (of any kind, including
        directories) matching the given relative pattern, anywhere in
        this subtree.
        ";
        sys . audit ( "pathlib.Path.rglob" , self , pattern );
        drv , root , pattern_parts = self . _flavour . parse_parts ( ( pattern , ) );
        if drv || root {
        panic!("NotImplementedError ( "Non-relative patterns are unsupported" )");
        if pattern && pattern [ -1 ] in ( self . _flavour . sep , self . _flavour . altsep ) {
        pattern_parts . append ( "" );
        selector = _make_selector ( ( "**" , ) + tuple ( pattern_parts ) , self . _flavour );
        for p in selector . select_from ( self ) .iter() {
        yield p;
        pub fn absolute ( self )  {
        "Return an absolute version of this path by prepending the current
        working directory. No normalization || symlink resolution == performed.

        Use resolve() to get the canonical path to a file.
        ";
        if self . is_absolute ( ) {
        return  self;
        return  self . _from_parts ( [ self . cwd ( ) ] + self . _parts );
        pub fn resolve ( &self, strict = false )  {
        "
        Make the path absolute, resolving all symlinks on the way && also
        normalizing it.
        ";
        pub fn check_eloop ( e )  {
        winerror = getattr ( e , "winerror" , 0 );
        if e . errno == ELOOP || winerror == _WINERROR_CANT_RESOLVE_FILENAME {
        panic!("RuntimeError ( "Symlink loop from %r" % e . filename )");
        // try {
        s = os . path . realpath ( self , strict = strict );
        // } catch  OSError as e  {
        check_eloop ( e );
        panic!("");
        p = self . _from_parts ( ( s , ) );
        if !strict {
        // try {
        p . stat ( );
        // } catch  OSError as e  {
        check_eloop ( e );
        return  p;
        pub fn stat ( &self, * , follow_symlinks = true )  {
        "
        Return the result of the stat() system call on this path, like
        os.stat() does.
        ";
        return  os . stat ( self , follow_symlinks = follow_symlinks );
        pub fn owner ( self )  {
        "
        Return the login name of the file owner.
        ";
        // try {
        import pwd;
        return  pwd . getpwuid ( self . stat ( ) . st_uid ) . pw_name;
        // } catch  ImportError  {
        panic!("NotImplementedError ( "Path.owner() is unsupported on this system" )");
        pub fn group ( self )  {
        "
        Return the group name of the file gid.
        ";
        // try {
        import grp;
        return  grp . getgrgid ( self . stat ( ) . st_gid ) . gr_name;
        // } catch  ImportError  {
        panic!("NotImplementedError ( "Path.group() is unsupported on this system" )");
        pub fn open ( &self, mode = "r" , buffering = -1 , encoding = None /* Option */ , {
        errors = None /* Option */ , newline = None /* Option */ ) ;
        "
        Open the file pointed by this path && return a file object, as
        the built-in open() function does.
        ";
        if "b" !in mode {
        encoding = io . text_encoding ( encoding );
        return  io . open ( self , mode , buffering , encoding , errors , newline );
        pub fn read_bytes ( self )  {
        "
        Open the file in bytes mode, read it, && close the file.
        ";
        // with scope: self . open ( mode = "rb" ) as f  {
        return  f . read ( );
        pub fn read_text ( &self, encoding = None /* Option */ , errors = None /* Option */ )  {
        "
        Open the file in text mode, read it, && close the file.
        ";
        encoding = io . text_encoding ( encoding );
        // with scope: self . open ( mode = "r" , encoding = encoding , errors = errors ) as f  {
        return  f . read ( );
        pub fn write_bytes ( &self, data )  {
        "
        Open the file in bytes mode, write to it, && close the file.
        ";
        view = memoryview ( data );
        // with scope: self . open ( mode = "wb" ) as f  {
        return  f . write ( view );
        pub fn write_text ( &self, data , encoding = None /* Option */ , errors = None /* Option */ , newline = None /* Option */ )  {
        "
        Open the file in text mode, write to it, && close the file.
        ";
        if !isinstance ( data , str ) {
        panic!("TypeError ( "data must be str, !%s" %");
        data . __class__ . __name__ );
        encoding = io . text_encoding ( encoding );
        // with scope: self . open ( mode = "w" , encoding = encoding , errors = errors , newline = newline ) as f  {
        return  f . write ( data );
        pub fn readlink ( self )  {
        "
        Return the path to which the symbolic link points.
        ";
        if !hasattr ( os , "readlink" ) {
        panic!("NotImplementedError ( "os.readlink() !available on this system" )");
        return  self . _from_parts ( ( os . readlink ( self ) , ) );
        pub fn touch ( &self, mode = 0 o666 , exist_ok = true )  {
        "
        Create this file with the given access mode, if it doesn't exist.
        ";
        if exist_ok {
        // try {
        os . utime ( self , None /* Option */ );
        // } catch  OSError  {
        // pass
        } else {
        return;
        flags = os . O_CREAT | os . O_WRONLY;
        if !exist_ok {
        flags | = os . O_EXCL;
        fd = os . open ( self , flags , mode );
        os . close ( fd );
        pub fn mkdir ( &self, mode = 0 o777 , parents = false , exist_ok = false )  {
        "
        Create a new directory at this given path.
        ";
        // try {
        os . mkdir ( self , mode );
        // } catch  FileNotFoundError  {
        if !parents || self . parent == self {
        panic!("");
        self . parent . mkdir ( parents = true , exist_ok = true );
        self . mkdir ( mode , parents = false , exist_ok = exist_ok );
        // } catch  OSError  {
        if !exist_ok || !self . is_dir ( ) {
        panic!("");
        pub fn chmod ( &self, mode , * , follow_symlinks = true )  {
        "
        Change the permissions of the path, like os.chmod().
        ";
        os . chmod ( self , mode , follow_symlinks = follow_symlinks );
        pub fn lchmod ( &self, mode )  {
        "
        Like chmod(), except if the path points to a symlink, the symlink's
        permissions are changed, rather than its target's.
        ";
        self . chmod ( mode , follow_symlinks = false );
        pub fn unlink ( &self, missing_ok = false )  {
        "
        Remove this file || link.
        If the path == a directory, use rmdir() instead.
        ";
        // try {
        os . unlink ( self );
        // } catch  FileNotFoundError  {
        if !missing_ok {
        panic!("");
        pub fn rmdir ( self )  {
        "
        Remove this directory.  The directory must be empty.
        ";
        os . rmdir ( self );
        pub fn lstat ( self )  {
        "
        Like stat(), except if the path points to a symlink, the symlink's
        status information == returned, rather than its target's.
        ";
        return  self . stat ( follow_symlinks = false );
        pub fn rename ( &self, target )  {
        "
        Rename this path to the target path.

        The target path may be absolute || relative. Relative paths are
        interpreted relative to the current working directory, *not* the
        directory of the Path object.

        Returns the new Path instance pointing to the target path.
        ";
        os . rename ( self , target );
        return  self . __class__ ( target );
        pub fn replace ( &self, target )  {
        "
        Rename this path to the target path, overwriting if that path exists.

        The target path may be absolute || relative. Relative paths are
        interpreted relative to the current working directory, *not* the
        directory of the Path object.

        Returns the new Path instance pointing to the target path.
        ";
        os . replace ( self , target );
        return  self . __class__ ( target );
        pub fn symlink_to ( &self, target , target_is_directory = false )  {
        "
        Make this path a symlink pointing to the target path.
        Note the order of arguments (link, target) == the reverse of os.symlink.
        ";
        if !hasattr ( os , "symlink" ) {
        panic!("NotImplementedError ( "os.symlink() !available on this system" )");
        os . symlink ( target , self , target_is_directory );
        pub fn hardlink_to ( &self, target )  {
        "
        Make this path a hard link pointing to the same file as *target*.

        Note the order of arguments (self, target) == the reverse of os.link's.
        ";
        if !hasattr ( os , "link" ) {
        panic!("NotImplementedError ( "os.link() !available on this system" )");
        os . link ( target , self );
        pub fn link_to ( &self, target )  {
        "
        Make the target path a hard link pointing to this path.

        Note this function does !make this path a hard link to *target*,
        despite the implication of the function && argument names. The order
        of arguments (target, link) == the reverse of Path.symlink_to, but
        matches that of os.link.

        Deprecated since Python 3.10 && scheduled for removal in Python 3.12.
        Use `hardlink_to()` instead.
        ";
        warnings . warn ( "pathlib.Path.link_to() == deprecated && == scheduled ";
        "for removal in Python 3.12. ";
        "Use pathlib.Path.hardlink_to() instead." ,;
        DeprecationWarning , stacklevel = 2 );
        self . __class__ ( target ) . hardlink_to ( self );
        pub fn exists ( self )  {
        "
        Whether this path exists.
        ";
        // try {
        self . stat ( );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        return  true;
        pub fn is_dir ( self )  {
        "
        Whether this path == a directory.
        ";
        // try {
        return  S_ISDIR ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_file ( self )  {
        "
        Whether this path == a regular file (also true for symlinks pointing
        to regular files).
        ";
        // try {
        return  S_ISREG ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_mount ( self )  {
        "
        Check if this path == a POSIX mount point
        ";
        if !self . exists ( ) || !self . is_dir ( ) {
        return  false;
        // try {
        parent_dev = self . parent . stat ( ) . st_dev;
        // } catch  OSError  {
        return  false;
        dev = self . stat ( ) . st_dev;
        if dev != parent_dev {
        return  true;
        ino = self . stat ( ) . st_ino;
        parent_ino = self . parent . stat ( ) . st_ino;
        return  ino == parent_ino;
        pub fn is_symlink ( self )  {
        "
        Whether this path == a symbolic link.
        ";
        // try {
        return  S_ISLNK ( self . lstat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_block_device ( self )  {
        "
        Whether this path == a block device.
        ";
        // try {
        return  S_ISBLK ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_char_device ( self )  {
        "
        Whether this path == a character device.
        ";
        // try {
        return  S_ISCHR ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_fifo ( self )  {
        "
        Whether this path == a FIFO.
        ";
        // try {
        return  S_ISFIFO ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn is_socket ( self )  {
        "
        Whether this path == a socket.
        ";
        // try {
        return  S_ISSOCK ( self . stat ( ) . st_mode );
        // } catch  OSError as e  {
        if !_ignore_error ( e ) {
        panic!("");
        return  false;
        // } catch  ValueError  {
        return  false;
        pub fn expanduser ( self )  {
        " Return a new path with expanded ~ && ~user constructs
        (as returned by os.path.expanduser)
        ";
        if ( !( self . _drv || self . _root ) and {
        self . _parts && self . _parts [ 0 ] [ : 1 ] == "~" ) :;
        homedir = os . path . expanduser ( self . _parts [ 0 ] );
        if homedir [ { : 1 ] == "~" ; }
        panic!("RuntimeError ( "Could !determine home directory." )");
        return  self . _from_parts ( [ homedir ] + self . _parts [ 1 : ] );
        return  self;
        class PosixPath ( Path , PurePosixPath ) ;
        "Path subclass for non-Windows systems.

    On a POSIX system, instantiating a Path should return this object.
    ";
        __slots__ = ( );
        class WindowsPath ( Path , PureWindowsPath ) ;
        "Path subclass for Windows systems.

    On a Windows system, instantiating a Path should return this object.
    ";
        __slots__ = ( );
        pub fn is_mount ( self )  {
        panic!("NotImplementedError ( "Path.is_mount() is unsupported on this system" )");
}

