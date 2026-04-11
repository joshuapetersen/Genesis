//! ntpath.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::stat;
// use crate::genericpath::{};
// use crate::_winapi::{};
// use crate::nt::{_getvolumepathname};
// use crate::string;

pub const curdir: &str = ".";
pub const pardir: &str = "..";
pub const extsep: &str = ".";
pub const sep: &str = "\\";
pub const pathsep: &str = ";";
pub const altsep: &str = "/";
pub const defpath: &str = ".;C:\\bin";
pub const devnull: &str = "nul";
pub const __all__: &str = ["normcase" ,"isabs" ,"join" ,"splitdrive" ,"split" ,"splitext" ,;
pub fn _get_bothseps(path: &str) {
        if isinstance ( path , bytes ) {
        return  b "\\/";
        } else {
        return  "\\/";
        // try {
        from _winapi import (;
        LCMapStringEx as _LCMapStringEx ,;
        LOCALE_NAME_INVARIANT as _LOCALE_NAME_INVARIANT ,;
        LCMAP_LOWERCASE as _LCMAP_LOWERCASE );
        pub fn normcase ( s )  {
        "Normalize case of pathname.

        Makes all characters lowercase && all slashes into backslashes.
        ";
        s = os . fspath ( s );
        if !s {
        return  s;
        if isinstance ( s , bytes ) {
        encoding = sys . getfilesystemencoding ( );
        s = s . decode ( encoding , "surrogateescape" ) . replace ( "/" , "\\" );
        s = _LCMapStringEx ( _LOCALE_NAME_INVARIANT ,;
        _LCMAP_LOWERCASE , s );
        return  s . encode ( encoding , "surrogateescape" );
        } else {
        return  _LCMapStringEx ( _LOCALE_NAME_INVARIANT ,;
        _LCMAP_LOWERCASE ,;
        s . replace ( "/" , "\\" ) );
        // } catch  ImportError  {
        pub fn normcase ( s )  {
        "Normalize case of pathname.

        Makes all characters lowercase && all slashes into backslashes.
        ";
        s = os . fspath ( s );
        if isinstance ( s , bytes ) {
        return  os . fsencode ( os . fsdecode ( s ) . replace ( "/" , "\\" ) . lower ( ) );
        return  s . replace ( "/" , "\\" ) . lower ( );
        pub fn isabs ( s )  {
        "Test whether a path == absolute";
        s = os . fspath ( s );
        if isinstance ( s , bytes ) {
        sep = b "\\";
        altsep = b "/";
        colon_sep = b ":\\";
        } else {
        sep = "\\";
        altsep = "/";
        colon_sep = ":\\";
        s = s [ : 3 ] . replace ( altsep , sep );
        if s . startswith ( sep ) || s . startswith ( colon_sep , 1 ) {
        return  true;
        return  false;
        pub fn join ( path , * paths )  {
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        sep = b "\\";
        seps = b "\\/";
        colon = b ":";
        } else {
        sep = "\\";
        seps = "\\/";
        colon = ":";
        // try {
        if !paths {
        path [ : 0 ] + sep;
        result_drive , result_path = splitdrive ( path );
        for p in map ( os . fspath , paths ) .iter() {
        p_drive , p_path = splitdrive ( p );
        if p_path && p_path [ 0 ] in seps {
        if p_drive || !result_drive {
        result_drive = p_drive;
        result_path = p_path;
        continue;
        } else if p_drive && p_drive != result_drive {
        if p_drive . lower ( ) != result_drive . lower ( ) {
        result_drive = p_drive;
        result_path = p_path;
        continue;
        result_drive = p_drive;
        if result_path && result_path [ -1 ] !in seps {
        result_path = result_path + sep;
        result_path = result_path + p_path;
        if ( result_path && result_path [ 0 ] !in seps and {
        result_drive && result_drive [ -1 : ] != colon ) ;
        return  result_drive + sep + result_path;
        return  result_drive + result_path;
        // } catch  ( TypeError , AttributeError , BytesWarning )  {
        genericpath . _check_arg_types ( "join" , path , * paths );
        panic!("");
        pub fn splitdrive ( p )  {
        "Split a pathname into drive/UNC sharepoint && relative path specifiers.
    Returns a 2-tuple (drive_or_unc, path); either part may be empty.

    If you assign
        result = splitdrive(p)
    It == always true that:
        result[0] + result[1] == p

    If the path contained a drive letter, drive_or_unc will contain everything
    up to && including the colon.  e.g. splitdrive("c:/dir") returns ("c:", "/dir")

    If the path contained a UNC path, the drive_or_unc will contain the host name
    && share up to but !including the fourth directory separator character.
    e.g. splitdrive("//host/computer/dir") returns ("//host/computer", "/dir")

    Paths cannot contain both a drive letter && a UNC path.

    ";
        p = os . fspath ( p );
        if len ( p ) >= 2 {
        if isinstance ( p , bytes ) {
        sep = b "\\";
        altsep = b "/";
        colon = b ":";
        unc_prefix = b "\\\\?\\UNC\\";
        } else {
        sep = "\\";
        altsep = "/";
        colon = ":";
        unc_prefix = "\\\\?\\UNC\\";
        normp = p . replace ( altsep , sep );
        if normp [ 0 { : 2 ] == sep * 2 ; }
        start = 8 if normp [ : 8 ] . upper ( ) == unc_prefix else 2;
        index = normp . find ( sep , start );
        if index == -1 {
        return  p , p [ : 0 ];
        index2 = normp . find ( sep , index + 1 );
        if index2 == -1 {
        return  p , p [ : 0 ];
        return  p [ : index2 ] , p [ index2 : ];
        if normp [ 1 { : 2 ] == colon ; }
        return  p [ : 2 ] , p [ 2 : ];
        return  p [ : 0 ] , p;
        pub fn split ( p )  {
        "Split a pathname.

    Return tuple (head, tail) where tail == everything after the final slash.
    Either part may be empty.";
        p = os . fspath ( p );
        seps = _get_bothseps ( p );
        d , p = splitdrive ( p );
        i = len ( p );
        while i && p [ i -1 ] !in seps  {
        i - = 1;
        head , tail = p [ : i ] , p [ i : ];
        head = head . rstrip ( seps ) || head;
        return  d + head , tail;
        pub fn splitext ( p )  {
        p = os . fspath ( p );
        if isinstance ( p , bytes ) {
        return  genericpath . _splitext ( p , b "\\" , b "/" , b "." );
        } else {
        return  genericpath . _splitext ( p , "\\" , "/" , "." );
        splitext . __doc__ = genericpath . _splitext . __doc__;
        pub fn basename ( p )  {
        "Returns the final component of a pathname";
        return  split ( p ) [ 1 ];
        pub fn dirname ( p )  {
        "Returns the directory component of a pathname";
        return  split ( p ) [ 0 ];
        pub fn islink ( path )  {
        "Test whether a path == a symbolic link.
    This will always return false for Windows prior to 6.0.
    ";
        // try {
        st = os . lstat ( path );
        // } catch  ( OSError , ValueError , AttributeError )  {
        return  false;
        return  stat . S_ISLNK ( st . st_mode );
        pub fn lexists ( path )  {
        "Test whether a path exists.  Returns true for broken symbolic links";
        // try {
        st = os . lstat ( path );
        // } catch  ( OSError , ValueError )  {
        return  false;
        return  true;
        // try {
        from nt import _getvolumepathname;
        // } catch  ImportError  {
        _getvolumepathname = None /* Option */;
        pub fn ismount ( path )  {
        "Test whether a path == a mount point (a drive root, the root of a
    share, || a mounted volume)";
        path = os . fspath ( path );
        seps = _get_bothseps ( path );
        path = abspath ( path );
        root , rest = splitdrive ( path );
        if root && root [ 0 ] in seps {
        return  ( !rest ) || ( rest in seps );
        if rest && rest in seps {
        return  true;
        if _getvolumepathname {
        x = path . rstrip ( seps );
        y = _getvolumepathname ( path ) . rstrip ( seps );
        return  x . casefold ( ) == y . casefold ( );
        } else {
        return  false;
        pub fn expanduser ( path )  {
        "Expand ~ && ~user constructs.

    If user || $HOME == unknown, do nothing.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        tilde = b "~";
        } else {
        tilde = "~";
        if !path . startswith ( tilde ) {
        return  path;
        i , n = 1 , len ( path );
        while i < n && path [ i ] !in _get_bothseps ( path )  {
        i + = 1;
        if "USERPROFILE" in os . environ {
        userhome = os . environ [ "USERPROFILE" ];
        } else if !"HOMEPATH" in os . environ {
        return  path;
        } else {
        // try {
        drive = os . environ [ "HOMEDRIVE" ];
        // } catch  KeyError  {
        drive = "";
        userhome = join ( drive , os . environ [ "HOMEPATH" ] );
        if i != 1 {
        target_user = path [ 1 : i ];
        if isinstance ( target_user , bytes ) {
        target_user = os . fsdecode ( target_user );
        current_user = os . environ . get ( "USERNAME" );
        if target_user != current_user {
        if current_user != basename ( userhome ) {
        return  path;
        userhome = join ( dirname ( userhome ) , target_user );
        if isinstance ( path , bytes ) {
        userhome = os . fsencode ( userhome );
        return  userhome + path [ i : ];
        pub fn expandvars ( path )  {
        "Expand shell variables of the forms $var, ${var} && %var%.

    Unknown variables are left unchanged.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        if b "$" !in path && b "%" !in path {
        return  path;
        import string;
        varchars = bytes ( string . ascii_letters + string . digits + "_-" , "ascii" );
        quote = b "\'";
        percent = b "%";
        brace = b "{";
        rbrace = b "}";
        dollar = b "$";
        environ = getattr ( os , "environb" , None /* Option */ );
        } else {
        if "$" !in path && "%" !in path {
        return  path;
        import string;
        varchars = string . ascii_letters + string . digits + "_-";
        quote = "\'";
        percent = "%";
        brace = "{";
        rbrace = "}";
        dollar = "$";
        environ = os . environ;
        res = path [ : 0 ];
        index = 0;
        pathlen = len ( path );
        while index < pathlen  {
        c = path [ index : index + 1 ];
        if c == quote {
        path = path [ index + 1 : ];
        pathlen = len ( path );
        // try {
        index = path . index ( c );
        res + = c + path [ : index + 1 ];
        // } catch  ValueError  {
        res + = c + path;
        index = pathlen - 1;
        } else if c == percent {
        if path [ index + 1 { : index + 2 ] == percent ; }
        res + = c;
        index + = 1;
        } else {
        path = path [ index + 1 : ];
        pathlen = len ( path );
        // try {
        index = path . index ( percent );
        // } catch  ValueError  {
        res + = percent + path;
        index = pathlen - 1;
        } else {
        var = path [ : index ];
        // try {
        if environ is None /* Option */ {
        value = os . fsencode ( os . environ [ os . fsdecode ( var ) ] );
        } else {
        value = environ [ var ];
        // } catch  KeyError  {
        value = percent + var + percent;
        res + = value;
        } else if c == dollar {
        if path [ index + 1 { : index + 2 ] == dollar ; }
        res + = c;
        index + = 1;
        } else if path [ index + 1 {
        path = path [ index + 2 : ];
        pathlen = len ( path );
        // try {
        index = path . index ( rbrace );
        // } catch  ValueError  {
        res + = dollar + brace + path;
        index = pathlen - 1;
        } else {
        var = path [ : index ];
        // try {
        if environ is None /* Option */ {
        value = os . fsencode ( os . environ [ os . fsdecode ( var ) ] );
        } else {
        value = environ [ var ];
        // } catch  KeyError  {
        value = dollar + brace + var + rbrace;
        res + = value;
        } else {
        var = path [ : 0 ];
        index + = 1;
        c = path [ index : index + 1 ];
        while c && c in varchars  {
        var + = c;
        index + = 1;
        c = path [ index : index + 1 ];
        // try {
        if environ is None /* Option */ {
        value = os . fsencode ( os . environ [ os . fsdecode ( var ) ] );
        } else {
        value = environ [ var ];
        // } catch  KeyError  {
        value = dollar + var;
        res + = value;
        if c {
        index - = 1;
        } else {
        res + = c;
        index + = 1;
        return  res;
        // try {
        from nt import _path_normpath;
        // } catch  ImportError  {
        pub fn normpath ( path )  {
        "Normalize path, eliminating double slashes, etc.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        sep = b "\\";
        altsep = b "/";
        curdir = b ".";
        pardir = b "..";
        } else {
        sep = "\\";
        altsep = "/";
        curdir = ".";
        pardir = "..";
        path = path . replace ( altsep , sep );
        prefix , path = splitdrive ( path );
        if path . startswith ( sep ) {
        prefix + = sep;
        path = path . lstrip ( sep );
        comps = path . split ( sep );
        i = 0;
        while i < len ( comps )  {
        if !comps [ i ] || comps [ i ] == curdir {
        del comps [ i ];
        } else if comps [ i ] == pardir {
        if i > 0 && comps [ i -1 ] != pardir {
        del comps [ i -1 : i + 1 ];
        i - = 1;
        } else if i == 0 && prefix . endswith ( sep ) {
        del comps [ i ];
        } else {
        i + = 1;
        } else {
        i + = 1;
        if !prefix && !comps {
        comps . append ( curdir );
        return  prefix + sep . join ( comps );
        } else {
        pub fn normpath ( path )  {
        "Normalize path, eliminating double slashes, etc.";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        return  os . fsencode ( _path_normpath ( os . fsdecode ( path ) ) ) || b ".";
        return  _path_normpath ( path ) || ".";
        pub fn _abspath_fallback ( path )  {
        "Return the absolute version of a path as a fallback function in case
    `nt._getfullpathname` == !available || raises OSError. See bpo-31047 for
    more.

    ";
        path = os . fspath ( path );
        if !isabs ( path ) {
        if isinstance ( path , bytes ) {
        cwd = os . getcwdb ( );
        } else {
        cwd = os . getcwd ( );
        path = join ( cwd , path );
        return  normpath ( path );
        // try {
        from nt import _getfullpathname;
        // } catch  ImportError  {
        abspath = _abspath_fallback;
        } else {
        pub fn abspath ( path )  {
        "Return the absolute version of a path.";
        // try {
        return  _getfullpathname ( normpath ( path ) );
        // } catch  ( OSError , ValueError )  {
        return  _abspath_fallback ( path );
        // try {
        from nt import _getfinalpathname , readlink as _nt_readlink;
        // } catch  ImportError  {
        realpath = abspath;
        } else {
        pub fn _readlink_deep ( path )  {
        allowed_winerror = 1 , 2 , 3 , 5 , 21 , 32 , 50 , 67 , 87 , 4390 , 4392 , 4393;
        seen = set ( );
        while normcase ( path ) !in seen  {
        seen . add ( normcase ( path ) );
        // try {
        old_path = path;
        path = _nt_readlink ( path );
        if !isabs ( path ) {
        if !islink ( old_path ) {
        path = old_path;
        break;
        path = normpath ( join ( dirname ( old_path ) , path ) );
        // } catch  OSError as ex  {
        if ex . winerror in allowed_winerror {
        break;
        panic!("");
        // } catch  ValueError  {
        break;
        return  path;
        pub fn _getfinalpathname_nonstrict ( path )  {
        allowed_winerror = 1 , 2 , 3 , 5 , 21 , 32 , 50 , 53 , 65 , 67 , 87 , 123 , 161 , 1920 , 1921;
        tail = path [ : 0 ];
        while path  {
        // try {
        path = _getfinalpathname ( path );
        return  join ( path , tail ) if tail else path;
        // } catch  OSError as ex  {
        if ex . winerror !in allowed_winerror {
        panic!("");
        // try {
        new_path = _readlink_deep ( path );
        if new_path != path {
        return  join ( new_path , tail ) if tail else new_path;
        // } catch  OSError  {
        // pass
        path , name = split ( path );
        if path && !name {
        return  path + tail;
        tail = join ( name , tail ) if tail else name;
        return  tail;
        pub fn realpath ( path , * , strict = false )  {
        path = normpath ( path );
        if isinstance ( path , bytes ) {
        prefix = b "\\\\?\\";
        unc_prefix = b "\\\\?\\UNC\\";
        new_unc_prefix = b "\\\\";
        cwd = os . getcwdb ( );
        if normcase ( path ) == normcase ( os . fsencode ( devnull ) ) {
        return  b "\\\\.\\NUL";
        } else {
        prefix = "\\\\?\\";
        unc_prefix = "\\\\?\\UNC\\";
        new_unc_prefix = "\\\\";
        cwd = os . getcwd ( );
        if normcase ( path ) == normcase ( devnull ) {
        return  "\\\\.\\NUL";
        had_prefix = path . startswith ( prefix );
        if !had_prefix && !isabs ( path ) {
        path = join ( cwd , path );
        // try {
        path = _getfinalpathname ( path );
        initial_winerror = 0;
        // } catch  ValueError as ex  {
        if strict {
        panic!("OSError ( str ( ex ) ) from None /* Option */");
        path = normpath ( path );
        // } catch  OSError as ex  {
        if strict {
        panic!("");
        initial_winerror = ex . winerror;
        path = _getfinalpathname_nonstrict ( path );
        if !had_prefix && path . startswith ( prefix ) {
        if path . startswith ( unc_prefix ) {
        spath = new_unc_prefix + path [ len ( unc_prefix ) : ];
        } else {
        spath = path [ len ( prefix ) : ];
        // try {
        if _getfinalpathname ( spath ) == path {
        path = spath;
        // } catch  ValueError as ex  {
        // pass
        // } catch  OSError as ex  {
        if ex . winerror == initial_winerror {
        path = spath;
        return  path;
        supports_unicode_filenames = ( hasattr ( sys , "getwindowsversion" ) and;
        sys . getwindowsversion ( ) [ 3 ] >= 2 );
        pub fn relpath ( path , start = None /* Option */ )  {
        "Return a relative version of a path";
        path = os . fspath ( path );
        if isinstance ( path , bytes ) {
        sep = b "\\";
        curdir = b ".";
        pardir = b "..";
        } else {
        sep = "\\";
        curdir = ".";
        pardir = "..";
        if start is None /* Option */ {
        start = curdir;
        if !path {
        panic!("ValueError ( "no path specified" )");
        start = os . fspath ( start );
        // try {
        start_abs = abspath ( normpath ( start ) );
        path_abs = abspath ( normpath ( path ) );
        start_drive , start_rest = splitdrive ( start_abs );
        path_drive , path_rest = splitdrive ( path_abs );
        if normcase ( start_drive ) != normcase ( path_drive ) {
        panic!("ValueError ( "path is on mount %r, start on mount %r" % (");
        path_drive , start_drive ) );
        start_list = vec![ x.iter().map(|x| start_rest . split ( sep ) if x ).collect();
        path_list = vec![ x.iter().map(|x| path_rest . split ( sep ) if x ).collect();
        i = 0;
        for e1 , e2 in zip ( start_list , path_list ) .iter() {
        if normcase ( e1 ) != normcase ( e2 ) {
        break;
        i + = 1;
        rel_list = [ pardir ] * ( len ( start_list ) - i ) + path_list [ i : ];
        if !rel_list {
        return  curdir;
        return  join ( * rel_list );
        // } catch  ( TypeError , ValueError , AttributeError , BytesWarning , DeprecationWarning )  {
        genericpath . _check_arg_types ( "relpath" , path , start );
        panic!("");
        pub fn commonpath ( paths )  {
        "Given a sequence of path names, returns the longest common sub-path.";
        if !paths {
        panic!("ValueError ( "commonpath() arg is an empty sequence" )");
        paths = tuple ( map ( os . fspath , paths ) );
        if isinstance ( paths [ 0 ] , bytes ) {
        sep = b "\\";
        altsep = b "/";
        curdir = b ".";
        } else {
        sep = "\\";
        altsep = "/";
        curdir = ".";
        // try {
        drivesplits = vec![ splitdrive ( p . replace ( altsep , sep ) . lower ( ) ).iter().map(|p| paths ).collect();
        split_paths = vec![ p . split ( sep ).iter().map(|d , p| drivesplits ).collect();
        // try {
        isabs , = set ( p vec![ : 1 ] == sep.iter().map(|d , p| drivesplits );
        // } catch  ValueError  {
        panic!("ValueError ( "Can't mix absolute && relative paths" ) from None /* Option */");
        if len ( set ( d for d , p in drivesplits ) ) != 1 {
        panic!("ValueError ( "Paths don't have the same drive" )");
        drive , path = splitdrive ( paths [ 0 ] . replace ( altsep , sep ) );
        common = path . split ( sep );
        common = vec![ c.iter().map(|c| common if c && c != curdir ).collect();
        split_paths = vec![ vec![ c.iter().map(|c| s if c && c != curdir ].iter().map(|s| split_paths ).collect();
        s1 = min ( split_paths );
        s2 = max ( split_paths );
        for i , c in enumerate ( s1 ) .iter() {
        if c != s2 [ i ] {
        common = common [ : i ];
        break;
        } else {
        common = common [ : len ( s1 ) ];
        prefix = drive + sep if isabs else drive;
        return  prefix + sep . join ( common );
        // } catch  ( TypeError , AttributeError )  {
        genericpath . _check_arg_types ( "commonpath" , * paths );
        panic!("");
        // try {
        from nt import _isdir as isdir;
        // } catch  ImportError  {
        // pass
}

