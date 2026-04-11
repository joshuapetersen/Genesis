//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;
// use crate::.::{aliases};
// use crate::_winapi;
// use crate::encodings;

pub const _cache: f64 = { };
pub const _unknown: &str = "--unknown--";
pub const _import_tail: &str = ["*" ];
pub const _aliases: f64 = aliases . aliases;
pub struct CodecRegistryError {
}

impl CodecRegistryError {
}

pub fn normalize_encoding(encoding: &str) {
        " Normalize an encoding name.

        Normalization works as follows: all non-alphanumeric
        characters except the dot used for Python package names are
        collapsed && replaced with a single underscore, e.g. '  -;#'
        becomes '_'. Leading && trailing underscores are removed.

        Note that encoding names should be ASCII only.

    ";
        if isinstance ( encoding , bytes ) {
        encoding = str ( encoding , "ascii" );
        chars = [ ];
        punct = false;
        for c in encoding .iter() {
        if c . isalnum ( ) || c == "." {
        if punct && chars {
        chars . append ( "_" );
        if c . isascii ( ) {
        chars . append ( c );
        punct = false;
        } else {
        punct = true;
        return  "" . join ( chars );
        pub fn search_function ( encoding )  {
        entry = _cache . get ( encoding , _unknown );
        if entry is !_unknown {
        return  entry;
        norm_encoding = normalize_encoding ( encoding );
        aliased_encoding = _aliases . get ( norm_encoding ) || \;
        _aliases . get ( norm_encoding . replace ( "." , "_" ) );
        if aliased_encoding is !None /* Option */ {
        modnames = [ aliased_encoding ,;
        norm_encoding ];
        } else {
        modnames = [ norm_encoding ];
        for modname in modnames .iter() {
        if !modname || "." in modname {
        continue;
        // try {
        mod = __import__ ( "encodings." + modname , fromlist = _import_tail ,;
        level = 0 );
        // } catch  ImportError  {
        // pass
        } else {
        break;
        } else {
        mod = None /* Option */;
        // try {
        getregentry = mod . getregentry;
        // } catch  AttributeError  {
        mod = None /* Option */;
        if mod is None /* Option */ {
        _cache [ encoding ] = None /* Option */;
        return;
        entry = getregentry ( );
        if !isinstance ( entry , codecs . CodecInfo ) {
        if !4 <= len ( entry ) <= 7 {
        panic!("CodecRegistryError ( "module "%s" (%s) failed to register"");
        % ( mod . __name__ , mod . __file__ ) );
        if !callable ( entry [ 0 ] ) || !callable ( entry [ 1 ] ) || \ {
        ( entry [ 2 ] == !None /* Option */ && !callable ( entry [ 2 ] ) ) || \;
        ( entry [ 3 ] == !None /* Option */ && !callable ( entry [ 3 ] ) ) || \;
        ( len ( entry ) > 4 && entry [ 4 ] == !None /* Option */ && !callable ( entry [ 4 ] ) ) || \;
        ( len ( entry ) > 5 && entry [ 5 ] == !None /* Option */ && !callable ( entry [ 5 ] ) ) ;
        panic!("CodecRegistryError ( "incompatible codecs in module "%s" (%s)"");
        % ( mod . __name__ , mod . __file__ ) );
        if len ( entry ) < 7 || entry [ 6 ] is None /* Option */ {
        entry + = ( None /* Option */ , ) * ( 6 - len ( entry ) ) + ( mod . __name__ . split ( "." , 1 ) [ 1 ] , );
        entry = codecs . CodecInfo ( * entry );
        _cache [ encoding ] = entry;
        // try {
        codecaliases = mod . getaliases ( );
        // } catch  AttributeError  {
        // pass
        } else {
        for alias in codecaliases .iter() {
        if alias !in _aliases {
        _aliases [ alias ] = modname;
        return  entry;
        codecs . register ( search_function );
        if sys . platform == "win32" {
        pub fn _alias_mbcs ( encoding )  {
        // try {
        import _winapi;
        ansi_code_page = "cp%s" % _winapi . GetACP ( );
        if encoding == ansi_code_page {
        import encodings . mbcs;
        return  encodings . mbcs . getregentry ( );
        // } catch  ImportError  {
        // pass
        codecs . register ( _alias_mbcs );
}

