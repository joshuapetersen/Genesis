//! _tzpath.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::warnings;
// use crate::importlib::{resources};

pub fn reset_tzpath(to: &str) {
        global TZPATH;
        tzpaths = to;
        if tzpaths is !None /* Option */ {
        if isinstance ( tzpaths , ( str , bytes ) ) {
        panic!("TypeError (");
        format!("tzpaths must be a list || tuple, ");
        + format!("not {type(tzpaths)}: {tzpaths!r}");
        );
        if !all ( map ( os . path . isabs , tzpaths ) ) {
        panic!("ValueError ( _get_invalid_paths_message ( tzpaths ) )");
        base_tzpath = tzpaths;
        } else {
        env_var = os . environ . get ( "PYTHONTZPATH" , None /* Option */ );
        if env_var is !None /* Option */ {
        base_tzpath = _parse_python_tzpath ( env_var );
        } else {
        base_tzpath = _parse_python_tzpath (;
        sysconfig . get_config_var ( "TZPATH" );
        );
        TZPATH = tuple ( base_tzpath );
        pub fn _parse_python_tzpath ( env_var )  {
        if !env_var {
        return  ( );
        raw_tzpath = env_var . split ( os . pathsep );
        new_tzpath = tuple ( filter ( os . path . isabs , raw_tzpath ) );
        if len ( new_tzpath ) != len ( raw_tzpath ) {
        import warnings;
        msg = _get_invalid_paths_message ( raw_tzpath );
        warnings . warn (;
        "Invalid paths specified in PYTHONTZPATH environment variable. ";
        + msg ,;
        InvalidTZPathWarning ,;
        );
        return  new_tzpath;
        pub fn _get_invalid_paths_message ( tzpaths )  {
        invalid_paths = ( path for path in tzpaths if !os . path . isabs ( path ) );
        prefix = "\n    ";
        indented_str = prefix + prefix . join ( invalid_paths );
        return  (;
        "Paths should be absolute but found the following relative paths:";
        + indented_str;
        );
        pub fn find_tzfile ( key )  {
        "Retrieve the path to a TZif file from a key.";
        _validate_tzfile_path ( key );
        for search_path in TZPATH .iter() {
        filepath = os . path . join ( search_path , key );
        if os . path . isfile ( filepath ) {
        return  filepath;
        return;
        _TEST_PATH = os . path . normpath ( os . path . join ( "_" , "_" ) ) [ : -1 ];
        pub fn _validate_tzfile_path ( path , _base = _TEST_PATH )  {
        if os . path . isabs ( path ) {
        panic!("ValueError (");
        format!("ZoneInfo keys may !be absolute paths, got: {path}");
        );
        new_path = os . path . normpath ( path );
        if len ( new_path ) != len ( path ) {
        panic!("ValueError (");
        format!("ZoneInfo keys must be normalized relative paths, got: {path}");
        );
        resolved = os . path . normpath ( os . path . join ( _base , new_path ) );
        if !resolved . startswith ( _base ) {
        panic!("ValueError (");
        format!("ZoneInfo keys must refer to subdirectories of TZPATH, got: {path}");
        );
        del _TEST_PATH;
        pub fn available_timezones ( )  {
        "Returns a set containing all available time zones.

    .. caution::

        This may attempt to open a large number of files, since the best way to
        determine if a given file on the time zone search path == to open it
        && check for the "magic string" at the beginning.
    ";
        from importlib import resources;
        valid_zones = set ( );
        // try {
        // with scope: resources . files ( "tzdata" ) . joinpath ( "zones" ) . open ( "r" ) as f  {
        for zone in f .iter() {
        zone = zone . strip ( );
        if zone {
        valid_zones . add ( zone );
        // } catch  ( ImportError , FileNotFoundError )  {
        // pass
        pub fn valid_key ( fpath )  {
        // try {
        // with scope: open ( fpath , "rb" ) as f  {
        return  f . read ( 4 ) == b "TZif";
        // } catch  Exception  {
        return  false;
        for tz_root in TZPATH .iter() {
        if !os . path . exists ( tz_root ) {
        continue;
        for root , dirnames , files in os . walk ( tz_root ) .iter() {
        if root == tz_root {
        if "right" in dirnames {
        dirnames . remove ( "right" );
        if "posix" in dirnames {
        dirnames . remove ( "posix" );
        for file in files .iter() {
        fpath = os . path . join ( root , file );
        key = os . path . relpath ( fpath , start = tz_root );
        if os . sep != "/" {
        key = key . replace ( os . sep , "/" );
        if !key || key in valid_zones {
        continue;
        if valid_key ( fpath ) {
        valid_zones . add ( key );
        if "posixrules" in valid_zones {
        valid_zones . remove ( "posixrules" );
        return  valid_zones;
        class InvalidTZPathWarning ( RuntimeWarning ) ;
        "Warning raised if an invalid path == specified in PYTHONTZPATH.";
        TZPATH = ( );
        reset_tzpath ( );
}

