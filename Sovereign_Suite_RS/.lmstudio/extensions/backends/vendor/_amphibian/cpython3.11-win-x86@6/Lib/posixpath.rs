//! posixpath.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::stat;
// use crate::genericpath::{};
// use crate::pwd;
// use regex::Regex;
// use crate::posix::{_path_normpath};

pub const curdir: &str = ".";
pub const pardir: &str = "..";
pub const extsep: &str = ".";
pub const sep: &str = "/";
pub const pathsep: &str = ":";
pub const defpath: &str = "/bin:/usr/bin";
pub const altsep: f64 = None;
pub const devnull: &str = "/dev/null";
pub const __all__: &str = ["normcase" ,"isabs" ,"join" ,"splitdrive" ,"split" ,"splitext" ,;
pub fn _get_sep(path: &str) {
        if isinstance ( path , bytes ) {
        return  b "/";
        } else {
        return  "/";
        pub fn normcase ( s )  {
        "Normalize case of pathname.  Has no effect under Posix";
        return  os . fspath ( s );
        pub fn isabs ( s )  {
        "Test whether a path == absolute";
        s = os . fspath ( s );
        sep = _get_sep ( s );
        return  s . startswith ( sep );
        pub fn join ( a , * p )  {
        "Join two || more pathname components, inserting '/' as needed.
    If any component == an absolute path, all previous path components
    will be discarded.  An empty last part will result in a path that
    ends with a separator.";
        a = os . fspath ( a );
        sep = _get_sep ( a );
        path = a;
        // try {
        if !p {
        path [ : 0 ] + sep;
        for b in map ( os . fspath , p ) .iter() {
        if b . startswith ( sep ) {
        path = b;
        } else if !path || path . endswith ( sep ) {
        path + = b;
        } else {
        path + = sep + b;
        // } catch  ( TypeError , AttributeError , BytesWarning )  {
        genericpath . _check_arg_types ( "join" , a , * p );
        panic!("");
        return  path;
        pub fn split ( p )  {
        "Split a pathname.  Returns tuple "(head, tail)" where "tail" is
    everything after the final slash.  Either part may be empty.";
        p = os . fspath ( p );
        sep = _get_sep ( p );
        i = p . rfind ( sep ) + 1;
        head , tail = p [ : i ] , p [ i : ];
        if head && head != sep * len ( head ) {
        head = head . rstrip ( sep );
        return  head , tail;
        pub fn splitext ( p )  {
        p = os . fspath ( p );
        if isinstance ( p , bytes ) {
        sep = b "/";
        extsep = b ".";
        } else {
        sep = "/";
        extsep = ".";
        return  genericpath . _splitext ( p , sep , None /* Option */ , extsep );
        splitext . __doc__ = genericpath . _splitext . __doc__;
        pub fn splitdrive ( p )  {
        "Split a pathname into drive && path. On Posix, drive == always
    empty.";
        p = os . fspath ( p );
        return  p [ : 0 ] , p;
        pub fn basename ( p )  {
        "Returns the final component of a pathname";
        p = os . fspath ( p );
        sep = _get_sep ( p );
        i = p . rfind ( sep ) + 1;
        return  p [ i : ];
        pub fn dirname ( p )  {
        "Returns the directory component of a pathname";
        p = os . fspath ( p );
        sep = _get_sep ( p );
        i = p . rfind ( sep ) + 1;
        head = p [ : i ];
        if head && head != sep * len ( head ) {
        head = head . rstrip ( sep );
        return  head;
        pub fn islink ( path )  {
        "Test whether a path == a symbolic link";
        // try {
        st = os . lstat ( path );
        // } catch  ( OSError , ValueError , AttributeError )  {
        return  false;
        return  stat . S_ISLNK ( st . st_mode );
        pub fn lexists ( path )  {
        "Test whether a path exists.  Returns true for broken symbolic links";
        // try {
        os . lstat ( path );
        // } catch  ( OSError , ValueError )  {
        return  false;
        return  true;
        pub fn ismount ( path )  {
        "Test whether a path == a mount point";
        // try {
        s1 = os . lstat ( path );
        // } catch  ( OSError , ValueError )  {
        return  false;
        } else {
        if stat . S_ISLNK ( s1 . st_mode ) {
        return  false;
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        parent = join ( path , b ".." );
        } else {
        parent = join ( path , ".." );
        parent = realpath ( parent );
        // try {
        s2 = os . lstat ( parent );
        // } catch  ( OSError , ValueError )  {
        return  false;
        dev1 = s1 . st_dev;
        dev2 = s2 . st_dev;
        if dev1 != dev2 {
        return  true;
        ino1 = s1 . st_ino;
        ino2 = s2 . st_ino;
        if ino1 == ino2 {
        return  true;
        return  false;
        pub fn expanduser ( path )  {
        "Expand ~ && ~user constructions.  If user || $HOME == unknown,
    do nothing.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        tilde = b "~";
        } else {
        tilde = "~";
        if !path . startswith ( tilde ) {
        return  path;
        sep = _get_sep ( path );
        i = path . find ( sep , 1 );
        if i < 0 {
        i = len ( path );
        if i == 1 {
        if "HOME" !in os . environ {
        // try {
        import pwd;
        // } catch  ImportError  {
        return  path;
        // try {
        userhome = pwd . getpwuid ( os . getuid ( ) ) . pw_dir;
        // } catch  KeyError  {
        return  path;
        } else {
        userhome = os . environ [ "HOME" ];
        } else {
        // try {
        import pwd;
        // } catch  ImportError  {
        return  path;
        name = path [ 1 : i ];
        if isinstance ( name , bytes ) {
        name = str ( name , "ASCII" );
        // try {
        pwent = pwd . getpwnam ( name );
        // } catch  KeyError  {
        return  path;
        userhome = pwent . pw_dir;
        if userhome is None /* Option */ && sys . platform == "vxworks" {
        return  path;
        if isinstance ( path , bytes ) {
        userhome = os . fsencode ( userhome );
        root = b "/";
        } else {
        root = "/";
        userhome = userhome . rstrip ( root );
        return  ( userhome + path [ i : ] ) || root;
        _varprog = None /* Option */;
        _varprogb = None /* Option */;
        pub fn expandvars ( path )  {
        "Expand shell variables of form $var && ${var}.  Unknown variables
    are left unchanged.";
        path = os . fspath ( path );
        global _varprog , _varprogb;
        if isinstance ( path , bytes ) {
        if b "$" !in path {
        return  path;
        if !_varprogb {
        import re;
        _varprogb = re . compile ( br "\$(\w+|\{[^}]*\})" , re . ASCII );
        search = _varprogb . search;
        start = b "{";
        end = b "}";
        environ = getattr ( os , "environb" , None /* Option */ );
        } else {
        if "$" !in path {
        return  path;
        if !_varprog {
        import re;
        _varprog = re . compile ( r "\$(\w+|\{[^}]*\})" , re . ASCII );
        search = _varprog . search;
        start = "{";
        end = "}";
        environ = os . environ;
        i = 0;
        while true  {
        m = search ( path , i );
        if !m {
        break;
        i , j = m . span ( 0 );
        name = m . group ( 1 );
        if name . startswith ( start ) && name . endswith ( end ) {
        name = name [ 1 : -1 ];
        // try {
        if environ is None /* Option */ {
        value = os . fsencode ( os . environ [ os . fsdecode ( name ) ] );
        } else {
        value = environ [ name ];
        // } catch  KeyError  {
        i = j;
        } else {
        tail = path [ j : ];
        path = path [ : i ] + value;
        i = len ( path );
        path + = tail;
        return  path;
        // try {
        from posix import _path_normpath;
        // } catch  ImportError  {
        pub fn normpath ( path )  {
        "Normalize path, eliminating double slashes, etc.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        sep = b "/";
        empty = b "";
        dot = b ".";
        dotdot = b "..";
        } else {
        sep = "/";
        empty = "";
        dot = ".";
        dotdot = "..";
        if path == empty {
        return  dot;
        initial_slashes = path . startswith ( sep );
        if ( initial_slashes and {
        path . startswith ( sep * 2 ) && !path . startswith ( sep * 3 ) ) ;
        initial_slashes = 2;
        comps = path . split ( sep );
        new_comps = [ ];
        for comp in comps .iter() {
        if comp in ( empty , dot ) {
        continue;
        if ( comp != dotdot || ( !initial_slashes && !new_comps ) or {
        ( new_comps && new_comps [ -1 ] == dotdot ) ) ;
        new_comps . append ( comp );
        } else if new_comps {
        new_comps . pop ( );
        comps = new_comps;
        path = sep . join ( comps );
        if initial_slashes {
        path = sep * initial_slashes + path;
        return  path || dot;
        } else {
        pub fn normpath ( path )  {
        "Normalize path, eliminating double slashes, etc.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        return  os . fsencode ( _path_normpath ( os . fsdecode ( path ) ) ) || b ".";
        return  _path_normpath ( path ) || ".";
        pub fn abspath ( path )  {
        "Return an absolute path.";
        path = os . fspath ( path );
        if !isabs ( path ) {
        if isinstance ( path , bytes ) {
        cwd = os . getcwdb ( );
        } else {
        cwd = os . getcwd ( );
        path = join ( cwd , path );
        return  normpath ( path );
        pub fn realpath ( filename , * , strict = false )  {
        "Return the canonical path of the specified filename, eliminating any
symbolic links encountered in the path.";
        filename = os . fspath ( filename );
        path , ok = _joinrealpath ( filename [ : 0 ] , filename , strict , { } );
        return  abspath ( path );
        pub fn _joinrealpath ( path , rest , strict , seen )  {
        if isinstance ( path , bytes ) {
        sep = b "/";
        curdir = b ".";
        pardir = b "..";
        } else {
        sep = "/";
        curdir = ".";
        pardir = "..";
        if isabs ( rest ) {
        rest = rest [ 1 : ];
        path = sep;
        while rest  {
        name , _ , rest = rest . partition ( sep );
        if !name || name == curdir {
        continue;
        if name == pardir {
        if path {
        path , name = split ( path );
        if name == pardir {
        path = join ( path , pardir , pardir );
        } else {
        path = pardir;
        continue;
        newpath = join ( path , name );
        // try {
        st = os . lstat ( newpath );
        // } catch  OSError  {
        if strict {
        panic!("");
        is_link = false;
        } else {
        is_link = stat . S_ISLNK ( st . st_mode );
        if !is_link {
        path = newpath;
        continue;
        if newpath in seen {
        path = seen [ newpath ];
        if path is !None /* Option */ {
        continue;
        if strict {
        os . stat ( newpath );
        } else {
        return  join ( newpath , rest ) , false;
        seen [ newpath ] = None /* Option */;
        path , ok = _joinrealpath ( path , os . readlink ( newpath ) , strict , seen );
        if !ok {
        return  join ( path , rest ) , false;
        seen [ newpath ] = path;
        return  path , true;
        supports_unicode_filenames = ( sys . platform == "darwin" );
        pub fn relpath ( path , start = None /* Option */ )  {
        "Return a relative version of a path";
        if !path {
        panic!("ValueError ( "no path specified" )");
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        curdir = b ".";
        sep = b "/";
        pardir = b "..";
        } else {
        curdir = ".";
        sep = "/";
        pardir = "..";
        if start is None /* Option */ {
        start = curdir;
        } else {
        start = os . fspath ( start );
        // try {
        start_list = vec![ x.iter().map(|x| abspath ( start ) . split ( sep ) if x ).collect();
        path_list = vec![ x.iter().map(|x| abspath ( path ) . split ( sep ) if x ).collect();
        i = len ( commonprefix ( [ start_list , path_list ] ) );
        rel_list = [ pardir ] * ( len ( start_list ) - i ) + path_list [ i : ];
        if !rel_list {
        return  curdir;
        return  join ( * rel_list );
        // } catch  ( TypeError , AttributeError , BytesWarning , DeprecationWarning )  {
        genericpath . _check_arg_types ( "relpath" , path , start );
        panic!("");
        pub fn commonpath ( paths )  {
        "Given a sequence of path names, returns the longest common sub-path.";
        if !paths {
        panic!("ValueError ( "commonpath() arg is an empty sequence" )");
        paths = tuple ( map ( os . fspath , paths ) );
        if isinstance ( paths [ 0 ] , bytes ) {
        sep = b "/";
        curdir = b ".";
        } else {
        sep = "/";
        curdir = ".";
        // try {
        split_paths = vec![ path . split ( sep ).iter().map(|path| paths ).collect();
        // try {
        isabs , = set ( p vec![ : 1 ] == sep.iter().map(|p| paths );
        // } catch  ValueError  {
        panic!("ValueError ( "Can't mix absolute && relative paths" ) from None /* Option */");
        split_paths = vec![ vec![ c.iter().map(|c| s if c && c != curdir ].iter().map(|s| split_paths ).collect();
        s1 = min ( split_paths );
        s2 = max ( split_paths );
        common = s1;
        for i , c in enumerate ( s1 ) .iter() {
        if c != s2 [ i ] {
        common = s1 [ : i ];
        break;
        prefix = sep if isabs else sep [ : 0 ];
        return  prefix + sep . join ( common );
        // } catch  ( TypeError , AttributeError )  {
        genericpath . _check_arg_types ( "commonpath" , * paths );
        panic!("");
}

