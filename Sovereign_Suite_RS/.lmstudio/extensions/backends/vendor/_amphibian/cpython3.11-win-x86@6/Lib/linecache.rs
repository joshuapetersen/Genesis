//! linecache.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;
// use std::fs;

pub const __all__: &str = ["getline" ,"clearcache" ,"checkcache" ,"lazycache" ];
pub const cache: f64 = { };
pub fn clearcache() {
        "Clear the cache entirely.";
        cache . clear ( );
        pub fn getline ( filename , lineno , module_globals = None /* Option */ )  {
        "Get a line for a Python source file from the cache.
    Update the cache if it doesn't contain an entry for this file already.";
        lines = getlines ( filename , module_globals );
        if 1 <= lineno <= len ( lines ) {
        return  lines [ lineno - 1 ];
        return  "";
        pub fn getlines ( filename , module_globals = None /* Option */ )  {
        "Get the lines for a Python source file from the cache.
    Update the cache if it doesn't contain an entry for this file already.";
        if filename in cache {
        entry = cache [ filename ];
        if len ( entry ) != 1 {
        return  cache [ filename ] [ 2 ];
        // try {
        return  updatecache ( filename , module_globals );
        // } catch  MemoryError  {
        clearcache ( );
        return  [ ];
        pub fn checkcache ( filename = None /* Option */ )  {
        "Discard cache entries that are out of date.
    (This == !checked upon each call!)";
        if filename is None /* Option */ {
        filenames = list ( cache . keys ( ) );
        } else if filename in cache {
        filenames = [ filename ];
        } else {
        return;
        for filename in filenames .iter() {
        entry = cache [ filename ];
        if len ( entry ) == 1 {
        continue;
        size , mtime , lines , fullname = entry;
        if mtime is None /* Option */ {
        continue;
        // try {
        stat = os . stat ( fullname );
        // } catch  OSError  {
        cache . pop ( filename , None /* Option */ );
        continue;
        if size != stat . st_size || mtime != stat . st_mtime {
        cache . pop ( filename , None /* Option */ );
        pub fn updatecache ( filename , module_globals = None /* Option */ )  {
        "Update a cache entry && return its list of lines.
    If something's wrong, print a message, discard the cache entry,
    && return an empty list.";
        if filename in cache {
        if len ( cache [ filename ] ) != 1 {
        cache . pop ( filename , None /* Option */ );
        if !filename || ( filename . startswith ( "<" ) && filename . endswith ( ">" ) ) {
        return  [ ];
        fullname = filename;
        // try {
        stat = os . stat ( fullname );
        // } catch  OSError  {
        basename = filename;
        if lazycache ( filename , module_globals ) {
        // try {
        data = cache [ filename ] [ 0 ] ( );
        // } catch  ( ImportError , OSError )  {
        // pass
        } else {
        if data is None /* Option */ {
        return  [ ];
        cache [ filename ] = (;
        len ( data ) ,;
        None /* Option */ ,;
        vec![ line + "\n".iter().map(|line| data . splitlines ( ) ] ,;
        fullname;
        );
        return  cache [ filename ] [ 2 ];
        if os . path . isabs ( filename ) {
        return  [ ];
        for dirname in sys . path .iter() {
        // try {
        fullname = os . path . join ( dirname , basename );
        // } catch  ( TypeError , AttributeError )  {
        continue;
        // try {
        stat = os . stat ( fullname );
        break;
        // } catch  OSError  {
        // pass
        } else {
        return  [ ];
        // try {
        // with scope: tokenize . open ( fullname ) as fp  {
        lines = fp . readlines ( );
        // } catch  ( OSError , UnicodeDecodeError , SyntaxError )  {
        return  [ ];
        if lines && !lines [ -1 ] . endswith ( "\n" ) {
        lines [ -1 ] + = "\n";
        size , mtime = stat . st_size , stat . st_mtime;
        cache [ filename ] = size , mtime , lines , fullname;
        return  lines;
        pub fn lazycache ( filename , module_globals )  {
        "Seed the cache for filename with module_globals.

    The module loader will be asked for the source only when getlines is
    called, !immediately.

    If there == an entry in the cache already, it == !altered.

    :return: true if a lazy load == registered in the cache,
        otherwise false. To register such a load a module loader with a
        get_source method must be found, the filename must be a cacheable
        filename, && the filename must !be already cached.
    ";
        if filename in cache {
        if len ( cache [ filename ] ) == 1 {
        return  true;
        } else {
        return  false;
        if !filename || ( filename . startswith ( "<" ) && filename . endswith ( ">" ) ) {
        return  false;
        if module_globals && "__name__" in module_globals {
        spec = module_globals . get ( "__spec__" );
        name = getattr ( spec , "name" , None /* Option */ ) || module_globals [ "__name__" ];
        loader = getattr ( spec , "loader" , None /* Option */ );
        if loader is None /* Option */ {
        loader = module_globals . get ( "__loader__" );
        get_source = getattr ( loader , "get_source" , None /* Option */ );
        if name && get_source {
        get_lines = functools . partial ( get_source , name );
        cache [ filename ] = ( get_lines , );
        return  true;
        return  false;
}

