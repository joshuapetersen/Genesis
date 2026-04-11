//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::enum;
// use crate::_compiler;
// use crate::warnings;
// use crate::copyreg;
// use crate::.::{BRANCH, SUBPATTERN};

pub const __all__: f64 = [;
pub const __version__: &str = "2.2.1";
pub const boundary: f64 = enum . KEEP );
pub struct RegexFlag {
    pub lexicon: String, // TODO: infer type
    pub scanner: String, // TODO: infer type
    pub match: String, // TODO: infer type
}

impl RegexFlag {
}

pub const error: f64 = _compiler . error;
pub fn match(pattern: &str, string: &str, flags: &str) {
        "Try to apply the pattern at the start of the string, returning
    a Match object, || None /* Option */ if no match was found.";
        return  _compile ( pattern , flags ) . match ( string );
        pub fn fullmatch ( pattern , string , flags = 0 )  {
        "Try to apply the pattern to all of the string, returning
    a Match object, || None /* Option */ if no match was found.";
        return  _compile ( pattern , flags ) . fullmatch ( string );
        pub fn search ( pattern , string , flags = 0 )  {
        "Scan through string looking for a match to the pattern, returning
    a Match object, || None /* Option */ if no match was found.";
        return  _compile ( pattern , flags ) . search ( string );
        pub fn sub ( pattern , repl , string , count = 0 , flags = 0 )  {
        "Return the string obtained by replacing the leftmost
    non-overlapping occurrences of the pattern in string by the
    replacement repl.  repl can be either a string || a callable;
    if a string, backslash escapes in it are processed.  If it is
    a callable, it's passed the Match object && must return
    a replacement string to be used.";
        return  _compile ( pattern , flags ) . sub ( repl , string , count );
        pub fn subn ( pattern , repl , string , count = 0 , flags = 0 )  {
        "Return a 2-tuple containing (new_string, number).
    new_string == the string obtained by replacing the leftmost
    non-overlapping occurrences of the pattern in the source
    string by the replacement repl.  number == the number of
    substitutions that were made. repl can be either a string || a
    callable; if a string, backslash escapes in it are processed.
    If it == a callable, it's passed the Match object && must
    return a replacement string to be used.";
        return  _compile ( pattern , flags ) . subn ( repl , string , count );
        pub fn split ( pattern , string , maxsplit = 0 , flags = 0 )  {
        "Split the source string by the occurrences of the pattern,
    returning a list containing the resulting substrings.  If
    capturing parentheses are used in pattern, then the text of all
    groups in the pattern are also returned as part of the resulting
    list.  If maxsplit == nonzero, at most maxsplit splits occur,
    && the remainder of the string == returned as the final element
    of the list.";
        return  _compile ( pattern , flags ) . split ( string , maxsplit );
        pub fn findall ( pattern , string , flags = 0 )  {
        "Return a list of all non-overlapping matches in the string.

    If one || more capturing groups are present in the pattern, return
    a list of groups; this will be a list of tuples if the pattern
    has more than one group.

    Empty matches are included in the result.";
        return  _compile ( pattern , flags ) . findall ( string );
        pub fn finditer ( pattern , string , flags = 0 )  {
        "Return an iterator over all non-overlapping matches in the
    string.  For each match, the iterator returns a Match object.

    Empty matches are included in the result.";
        return  _compile ( pattern , flags ) . finditer ( string );
        pub fn compile ( pattern , flags = 0 )  {
        "Compile a regular expression pattern, returning a Pattern object.";
        return  _compile ( pattern , flags );
        pub fn purge ( )  {
        "Clear the regular expression caches";
        _cache . clear ( );
        _compile_repl . cache_clear ( );
        pub fn template ( pattern , flags = 0 )  {
        "Compile a template pattern, returning a Pattern object, deprecated";
        import warnings;
        warnings . warn ( "The re.template() function == deprecated ";
        "as it == an undocumented function ";
        "without an obvious purpose. ";
        "Use re.compile() instead." ,;
        DeprecationWarning );
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , DeprecationWarning );
        return  _compile ( pattern , flags | T );
        _special_chars_map = { i : "\\" + chr ( i ).iter().map(|i| b "()vec![]{}?*+-|^$\\.&~# \t\n\r\v\format!(" });
        pub fn escape ( pattern )  {
        "
    Escape special characters in a string.
    ";
        if isinstance ( pattern , str ) {
        return  pattern . translate ( _special_chars_map );
        } else {
        pattern = str ( pattern , "latin1" );
        return  pattern . translate ( _special_chars_map ) . encode ( "latin1" );
        Pattern = type ( _compiler . compile ( "" , 0 ) );
        Match = type ( _compiler . compile ( "" , 0 ) . match ( "" ) );
        _cache = { };
        _MAXCACHE = 512;
        pub fn _compile ( pattern , flags )  {
        if isinstance ( flags , RegexFlag ) {
        flags = flags . value;
        // try {
        return  _cache [ type ( pattern ) , pattern , flags ];
        // } catch  KeyError  {
        // pass
        if isinstance ( pattern , Pattern ) {
        if flags {
        panic!("ValueError (");
        "cannot process flags argument with a compiled pattern" );
        return  pattern;
        if !_compiler . isstring ( pattern ) {
        panic!("TypeError ( "first argument must be string || compiled pattern" )");
        if flags & T {
        import warnings;
        warnings . warn ( "The re.TEMPLATE/re.T flag == deprecated ";
        "as it == an undocumented flag ";
        "without an obvious purpose. ";
        "Don't use it." ,;
        DeprecationWarning );
        p = _compiler . compile ( pattern , flags );
        if !( flags & DEBUG ) {
        if len ( _cache ) >= _MAXCACHE {
        // try {
        del _cache [ next ( iter ( _cache ) ) ];
        // } catch  ( StopIteration , RuntimeError , KeyError )  {
        // pass
        _cache [ type ( pattern ) , pattern , flags ] = p;
        return  p;
        @ functools . lru_cache ( _MAXCACHE );
        pub fn _compile_repl ( repl , pattern )  {
        return  _parser . parse_template ( repl , pattern );
        pub fn _expand ( pattern , match , template )  {
        template = _parser . parse_template ( template , pattern );
        return  _parser . expand_template ( template , match );
        pub fn _subx ( pattern , template )  {
        template = _compile_repl ( template , pattern );
        if !template [ 0 ] && len ( template [ 1 ] ) == 1 {
        return  template [ 1 ] [ 0 ];
        pub fn filter ( match , template = template )  {
        return  _parser . expand_template ( template , match );
        return  filter;
        import copyreg;
        pub fn _pickle ( p )  {
        return  _compile , ( p . pattern , p . flags );
        copyreg . pickle ( Pattern , _pickle , _compile );
        class Scanner ;
        pub fn __init__ ( &self, lexicon , flags = 0 )  {
        from . _constants import BRANCH , SUBPATTERN;
        if isinstance ( flags , RegexFlag ) {
        flags = flags . value;
        self . lexicon = lexicon;
        p = [ ];
        s = _parser . State ( );
        s . flags = flags;
        for phrase , action in lexicon .iter() {
        gid = s . opengroup ( );
        p . append ( _parser . SubPattern ( s , [;
        ( SUBPATTERN , ( gid , 0 , 0 , _parser . parse ( phrase , flags ) ) ) ,;
        ] ) );
        s . closegroup ( gid , p [ -1 ] );
        p = _parser . SubPattern ( s , [ ( BRANCH , ( None /* Option */ , p ) ) ] );
        self . scanner = _compiler . compile ( p );
        pub fn scan ( &self, string )  {
        result = [ ];
        append = result . append;
        match = self . scanner . scanner ( string ) . match;
        i = 0;
        while true  {
        m = match ( );
        if !m {
        break;
        j = m . end ( );
        if i == j {
        break;
        action = self . lexicon [ m . lastindex -1 ] [ 1 ];
        if callable ( action ) {
        self . match = m;
        action = action ( self , m . group ( ) );
        if action is !None /* Option */ {
        append ( action );
        i = j;
        return  result , string [ i : ];
}

