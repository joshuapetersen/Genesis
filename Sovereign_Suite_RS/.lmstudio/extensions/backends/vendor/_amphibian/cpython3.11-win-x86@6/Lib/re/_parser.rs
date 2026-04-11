//! _parser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{};
// use crate::warnings;
// use crate::unicodedata;

pub const SPECIAL_CHARS: &str = ".\\[{()*+?^$|";
pub const REPEAT_CHARS: &str = "*+?{";
pub const DIGITS: &str = frozenset ("0123456789" );
pub const OCTDIGITS: &str = frozenset ("01234567" );
pub const HEXDIGITS: &str = frozenset ("0123456789abcdefABCDEF" );
pub const ASCIILETTERS: &str = frozenset ("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ" );
pub const WHITESPACE: &str = frozenset (" \t\n\r\v\f" );
pub const _REPEATCODES: f64 = frozenset ( { MIN_REPEAT , MAX_REPEAT , POSSESSIVE_REPEAT } );
pub const _UNITCODES: f64 = frozenset ( { ANY , RANGE , IN , LITERAL , NOT_LITERAL , CATEGORY } );
pub const ESCAPES: f64 = {;
pub const CATEGORIES: f64 = {;
pub const FLAGS: f64 = {;
pub const TYPE_FLAGS: /* inferred */ = SRE_FLAG_ASCII | SRE_FLAG_LOCALE | SRE_FLAG_UNICODE;
pub const GLOBAL_FLAGS: /* inferred */ = SRE_FLAG_DEBUG | SRE_FLAG_TEMPLATE;
pub const MAXWIDTH: u64 = 1 < < 64;
pub struct State {
    pub flags: String, // TODO: infer type
    pub groupdict: String, // TODO: infer type
    pub groupwidths: String, // TODO: infer type
    pub lookbehindgroups: String, // TODO: infer type
    pub grouprefpos: String, // TODO: infer type
    pub state: String, // TODO: infer type
    pub data: String, // TODO: infer type
    pub width: String, // TODO: infer type
    pub istext: String, // TODO: infer type
    pub string: String, // TODO: infer type
    pub decoded_string: String, // TODO: infer type
    pub index: String, // TODO: infer type
    pub next: String, // TODO: infer type
}

impl State {
    pub fn new() -> Self {
        self . flags = 0;
        self . groupdict = { };
        self . groupwidths = [ None /* Option */ ];
        self . lookbehindgroups = None /* Option */;
        self . grouprefpos = { };
        @ property;
        pub fn groups ( self )  {
        return  len ( self . groupwidths );
        pub fn opengroup ( &self, name = None /* Option */ )  {
        gid = self . groups;
        self . groupwidths . append ( None /* Option */ );
        if self . groups > MAXGROUPS {
        panic!("error ( "too many groups" )");
        if name is !None /* Option */ {
        ogid = self . groupdict . get ( name , None /* Option */ );
        if ogid is !None /* Option */ {
        panic!("error ( "redefinition of group name %r as group %d; "");
        "was group %d" % ( name , gid , ogid ) );
        self . groupdict [ name ] = gid;
        return  gid;
        pub fn closegroup ( &self, gid , p )  {
        self . groupwidths [ gid ] = p . getwidth ( );
        pub fn checkgroup ( &self, gid )  {
        return  gid < self . groups && self . groupwidths [ gid ] is !None /* Option */;
    }

    pub fn _class_escape(&self, source: &str, escape: &str) {
        code = ESCAPES . get ( escape );
        if code {
        return  code;
        code = CATEGORIES . get ( escape );
        if code && code [ 0 ] is IN {
        return  code;
        // try {
        c = escape [ 1 : 2 ];
        if c == "x" {
        escape + = source . getwhile ( 2 , HEXDIGITS );
        if len ( escape ) != 4 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        return  LITERAL , int ( escape [ 2 : ] , 16 );
        } else if c == "u" && source . istext {
        escape + = source . getwhile ( 4 , HEXDIGITS );
        if len ( escape ) != 6 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        return  LITERAL , int ( escape [ 2 : ] , 16 );
        } else if c == "U" && source . istext {
        escape + = source . getwhile ( 8 , HEXDIGITS );
        if len ( escape ) != 10 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        c = int ( escape [ 2 : ] , 16 );
        chr ( c );
        return  LITERAL , c;
        } else if c == "N" && source . istext {
        import unicodedata;
        if !source . match ( "{" ) {
        panic!("source . error ( "missing {" )");
        charname = source . getuntil ( "}" , "character name" );
        // try {
        c = ord ( unicodedata . lookup ( charname ) );
        // } catch  ( KeyError , TypeError )  {
        panic!("source . error ( "undefined character name %r" % charname ,");
        len ( charname ) + len ( r "\N{}" ) ) from None /* Option */;
        return  LITERAL , c;
        } else if c in OCTDIGITS {
        escape + = source . getwhile ( 2 , OCTDIGITS );
        c = int ( escape [ 1 : ] , 8 );
        if c > 0 o377 {
        panic!("source . error ( "octal escape value %s outside of "");
        "range 0-0o377" % escape , len ( escape ) );
        return  LITERAL , c;
        } else if c in DIGITS {
        panic!("ValueError");
        if len ( escape ) == 2 {
        if c in ASCIILETTERS {
        panic!("source . error ( "bad escape %s" % escape , len ( escape ) )");
        return  LITERAL , ord ( escape [ 1 ] );
        // } catch  ValueError  {
        // pass
        panic!("source . error ( "bad escape %s" % escape , len ( escape ) )");
        pub fn _escape ( source , escape , state )  {
        code = CATEGORIES . get ( escape );
        if code {
        return  code;
        code = ESCAPES . get ( escape );
        if code {
        return  code;
        // try {
        c = escape [ 1 : 2 ];
        if c == "x" {
        escape + = source . getwhile ( 2 , HEXDIGITS );
        if len ( escape ) != 4 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        return  LITERAL , int ( escape [ 2 : ] , 16 );
        } else if c == "u" && source . istext {
        escape + = source . getwhile ( 4 , HEXDIGITS );
        if len ( escape ) != 6 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        return  LITERAL , int ( escape [ 2 : ] , 16 );
        } else if c == "U" && source . istext {
        escape + = source . getwhile ( 8 , HEXDIGITS );
        if len ( escape ) != 10 {
        panic!("source . error ( "incomplete escape %s" % escape , len ( escape ) )");
        c = int ( escape [ 2 : ] , 16 );
        chr ( c );
        return  LITERAL , c;
        } else if c == "N" && source . istext {
        import unicodedata;
        if !source . match ( "{" ) {
        panic!("source . error ( "missing {" )");
        charname = source . getuntil ( "}" , "character name" );
        // try {
        c = ord ( unicodedata . lookup ( charname ) );
        // } catch  ( KeyError , TypeError )  {
        panic!("source . error ( "undefined character name %r" % charname ,");
        len ( charname ) + len ( r "\N{}" ) ) from None /* Option */;
        return  LITERAL , c;
        } else if c == "0" {
        escape + = source . getwhile ( 2 , OCTDIGITS );
        return  LITERAL , int ( escape [ 1 : ] , 8 );
        } else if c in DIGITS {
        if source . next in DIGITS {
        escape + = source . get ( );
        if ( escape [ 1 ] in OCTDIGITS && escape [ 2 ] in OCTDIGITS and {
        source . next in OCTDIGITS ) ;
        escape + = source . get ( );
        c = int ( escape [ 1 : ] , 8 );
        if c > 0 o377 {
        panic!("source . error ( "octal escape value %s outside of "");
        "range 0-0o377" % escape ,;
        len ( escape ) );
        return  LITERAL , c;
        group = int ( escape [ 1 : ] );
        if group < state . groups {
        if !state . checkgroup ( group ) {
        panic!("source . error ( "cannot refer to an open group" ,");
        len ( escape ) );
        state . checklookbehindgroup ( group , source );
        return  GROUPREF , group;
        panic!("source . error ( "invalid group reference %d" % group , len ( escape ) - 1 )");
        if len ( escape ) == 2 {
        if c in ASCIILETTERS {
        panic!("source . error ( "bad escape %s" % escape , len ( escape ) )");
        return  LITERAL , ord ( escape [ 1 ] );
        // } catch  ValueError  {
        // pass
        panic!("source . error ( "bad escape %s" % escape , len ( escape ) )");
        pub fn _uniq ( items )  {
        return  list ( dict . fromkeys ( items ) );
        pub fn _parse_sub ( source , state , verbose , nested )  {
        items = [ ];
        itemsappend = items . append;
        sourcematch = source . match;
        start = source . tell ( );
        while true  {
        itemsappend ( _parse ( source , state , verbose , nested + 1 ,;
        not nested && !items ) );
        if !sourcematch ( "|" ) {
        break;
        if !nested {
        verbose = state . flags & SRE_FLAG_VERBOSE;
        if len ( items ) == 1 {
        return  items [ 0 ];
        subpattern = SubPattern ( state );
        while true  {
        prefix = None /* Option */;
        for item in items .iter() {
        if !item {
        break;
        if prefix is None /* Option */ {
        prefix = item [ 0 ];
        } else if item [ 0 ] != prefix {
        break;
        } else {
        for item in items .iter() {
        del item [ 0 ];
        subpattern . append ( prefix );
        continue;
        break;
        set = [ ];
        for item in items .iter() {
        if len ( item ) != 1 {
        break;
        op , av = item [ 0 ];
        if op is LITERAL {
        set . append ( ( op , av ) );
        } else if op is IN && av [ 0 ] [ 0 ] is !NEGATE {
        set . extend ( av );
        } else {
        break;
        } else {
        subpattern . append ( ( IN , _uniq ( set ) ) );
        return  subpattern;
        subpattern . append ( ( BRANCH , ( None /* Option */ , items ) ) );
        return  subpattern;
        pub fn _parse ( source , state , verbose , nested , first = false )  {
        subpattern = SubPattern ( state );
        subpatternappend = subpattern . append;
        sourceget = source . get;
        sourcematch = source . match;
        _len = len;
        _ord = ord;
        while true  {
        this = source . next;
        if this is None /* Option */ {
        break;
        if this in "|)" {
        break;
        sourceget ( );
        if verbose {
        if this in WHITESPACE {
        continue;
        if this == "#" {
        while true  {
        this = sourceget ( );
        if this is None /* Option */ || this == "\n" {
        break;
        continue;
        if this [ 0 ] == "\\" {
        code = _escape ( source , this , state );
        subpatternappend ( code );
        } else if this !in SPECIAL_CHARS {
        subpatternappend ( ( LITERAL , _ord ( this ) ) );
        } else if this == "[" {
        here = source . tell ( ) - 1;
        set = [ ];
        setappend = set . append;
        if source . next == "[" {
        import warnings;
        warnings . warn (;
        "Possible nested set at position %d" % source . tell ( ) ,;
        FutureWarning , stacklevel = nested + 6;
        );
        negate = sourcematch ( "^" );
        while true  {
        this = sourceget ( );
        if this is None /* Option */ {
        panic!("source . error ( "unterminated character set" ,");
        source . tell ( ) - here );
        if this == "]" && set {
        break;
        } else if this [ 0 ] == "\\" {
        code1 = _class_escape ( source , this );
        } else {
        if set && this in "-&~|" && source . next == this {
        import warnings;
        warnings . warn (;
        "Possible set %s at position %d" % (;
        "difference" if this == "-" else;
        "intersection" if this == "&" else;
        "symmetric difference" if this == "~" else;
        "union" ,;
        source . tell ( ) - 1 ) ,;
        FutureWarning , stacklevel = nested + 6;
        );
        code1 = LITERAL , _ord ( this );
        if sourcematch ( "-" ) {
        that = sourceget ( );
        if that is None /* Option */ {
        panic!("source . error ( "unterminated character set" ,");
        source . tell ( ) - here );
        if that == "]" {
        if code1 [ 0 ] is IN {
        code1 = code1 [ 1 ] [ 0 ];
        setappend ( code1 );
        setappend ( ( LITERAL , _ord ( "-" ) ) );
        break;
        if that [ 0 ] == "\\" {
        code2 = _class_escape ( source , that );
        } else {
        if that == "-" {
        import warnings;
        warnings . warn (;
        "Possible set difference at position %d" % (;
        source . tell ( ) - 2 ) ,;
        FutureWarning , stacklevel = nested + 6;
        );
        code2 = LITERAL , _ord ( that );
        if code1 [ 0 ] != LITERAL || code2 [ 0 ] != LITERAL {
        msg = "bad character range %s-%s" % ( this , that );
        panic!("source . error ( msg , len ( this ) + 1 + len ( that ) )");
        lo = code1 [ 1 ];
        hi = code2 [ 1 ];
        if hi < lo {
        msg = "bad character range %s-%s" % ( this , that );
        panic!("source . error ( msg , len ( this ) + 1 + len ( that ) )");
        setappend ( ( RANGE , ( lo , hi ) ) );
        } else {
        if code1 [ 0 ] is IN {
        code1 = code1 [ 1 ] [ 0 ];
        setappend ( code1 );
        set = _uniq ( set );
        if _len ( set ) == 1 && set [ 0 ] [ 0 ] is LITERAL {
        if negate {
        subpatternappend ( ( NOT_LITERAL , set [ 0 ] [ 1 ] ) );
        } else {
        subpatternappend ( set [ 0 ] );
        } else {
        if negate {
        set . insert ( 0 , ( NEGATE , None /* Option */ ) );
        subpatternappend ( ( IN , set ) );
        } else if this in REPEAT_CHARS {
        here = source . tell ( );
        if this == "?" {
        min , max = 0 , 1;
        } else if this == "*" {
        min , max = 0 , MAXREPEAT;
        } else if this == "+" {
        min , max = 1 , MAXREPEAT;
        } else if this == "{" {
        if source . next == "}" {
        subpatternappend ( ( LITERAL , _ord ( this ) ) );
        continue;
        min , max = 0 , MAXREPEAT;
        lo = hi = "";
        while source . next in DIGITS  {
        lo + = sourceget ( );
        if sourcematch ( "," ) {
        while source . next in DIGITS  {
        hi + = sourceget ( );
        } else {
        hi = lo;
        if !sourcematch ( "}" ) {
        subpatternappend ( ( LITERAL , _ord ( this ) ) );
        source . seek ( here );
        continue;
        if lo {
        min = int ( lo );
        if min >= MAXREPEAT {
        panic!("OverflowError ( "the repetition number is too large" )");
        if hi {
        max = int ( hi );
        if max >= MAXREPEAT {
        panic!("OverflowError ( "the repetition number is too large" )");
        if max < min {
        panic!("source . error ( "min repeat greater than max repeat" ,");
        source . tell ( ) - here );
        } else {
        panic!("AssertionError ( "unsupported quantifier %r" % ( char , ) )");
        if subpattern {
        item = subpattern [ -1 : ];
        } else {
        item = None /* Option */;
        if !item || item [ 0 ] [ 0 ] is AT {
        panic!("source . error ( "nothing to repeat" ,");
        source . tell ( ) - here + len ( this ) );
        if item [ 0 ] [ 0 ] in _REPEATCODES {
        panic!("source . error ( "multiple repeat" ,");
        source . tell ( ) - here + len ( this ) );
        if item [ 0 ] [ 0 ] is SUBPATTERN {
        group , add_flags , del_flags , p = item [ 0 ] [ 1 ];
        if group is None /* Option */ && !add_flags && !del_flags {
        item = p;
        if sourcematch ( "?" ) {
        subpattern [ -1 ] = ( MIN_REPEAT , ( min , max , item ) );
        } else if sourcematch ( "+" ) {
        subpattern [ -1 ] = ( POSSESSIVE_REPEAT , ( min , max , item ) );
        } else {
        subpattern [ -1 ] = ( MAX_REPEAT , ( min , max , item ) );
        } else if this == "." {
        subpatternappend ( ( ANY , None /* Option */ ) );
        } else if this == "(" {
        start = source . tell ( ) - 1;
        capture = true;
        atomic = false;
        name = None /* Option */;
        add_flags = 0;
        del_flags = 0;
        if sourcematch ( "?" ) {
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "unexpected end of pattern" )");
        if char == "P" {
        if sourcematch ( "<" ) {
        name = source . getuntil ( ">" , "group name" );
        source . checkgroupname ( name , 1 , nested );
        } else if sourcematch ( "=" ) {
        name = source . getuntil ( ")" , "group name" );
        source . checkgroupname ( name , 1 , nested );
        gid = state . groupdict . get ( name );
        if gid is None /* Option */ {
        msg = "unknown group name %r" % name;
        panic!("source . error ( msg , len ( name ) + 1 )");
        if !state . checkgroup ( gid ) {
        panic!("source . error ( "cannot refer to an open group" ,");
        len ( name ) + 1 );
        state . checklookbehindgroup ( gid , source );
        subpatternappend ( ( GROUPREF , gid ) );
        continue;
        } else {
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "unexpected end of pattern" )");
        panic!("source . error ( "unknown extension ?P" + char ,");
        len ( char ) + 2 );
        } else if char == ":" {
        capture = false;
        } else if char == "#" {
        while true  {
        if source . next is None /* Option */ {
        panic!("source . error ( "missing ), unterminated comment" ,");
        source . tell ( ) - start );
        if sourceget ( ) == ")" {
        break;
        continue;
        } else if char in "=!<" {
        dir = 1;
        if char == "<" {
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "unexpected end of pattern" )");
        if char !in "=!" {
        panic!("source . error ( "unknown extension ?<" + char ,");
        len ( char ) + 2 );
        dir = -1;
        lookbehindgroups = state . lookbehindgroups;
        if lookbehindgroups is None /* Option */ {
        state . lookbehindgroups = state . groups;
        p = _parse_sub ( source , state , verbose , nested + 1 );
        if dir < 0 {
        if lookbehindgroups is None /* Option */ {
        state . lookbehindgroups = None /* Option */;
        if !sourcematch ( ")" ) {
        panic!("source . error ( "missing ), unterminated subpattern" ,");
        source . tell ( ) - start );
        if char == "=" {
        subpatternappend ( ( ASSERT , ( dir , p ) ) );
        } else {
        subpatternappend ( ( ASSERT_NOT , ( dir , p ) ) );
        continue;
        } else if char == "(" {
        condname = source . getuntil ( ")" , "group name" );
        if condname . isidentifier ( ) {
        source . checkgroupname ( condname , 1 , nested );
        condgroup = state . groupdict . get ( condname );
        if condgroup is None /* Option */ {
        msg = "unknown group name %r" % condname;
        panic!("source . error ( msg , len ( condname ) + 1 )");
        } else {
        // try {
        condgroup = int ( condname );
        if condgroup < 0 {
        panic!("ValueError");
        // } catch  ValueError  {
        msg = "bad character in group name %r" % condname;
        panic!("source . error ( msg , len ( condname ) + 1 ) from None /* Option */");
        if !condgroup {
        panic!("source . error ( "bad group number" ,");
        len ( condname ) + 1 );
        if condgroup >= MAXGROUPS {
        msg = "invalid group reference %d" % condgroup;
        panic!("source . error ( msg , len ( condname ) + 1 )");
        if condgroup !in state . grouprefpos {
        state . grouprefpos [ condgroup ] = (;
        source . tell ( ) - len ( condname ) - 1;
        );
        if !( condname . isdecimal ( ) && condname . isascii ( ) ) {
        import warnings;
        warnings . warn (;
        "bad character in group name %s at position %d" %;
        ( repr ( condname ) if source . istext else ascii ( condname ) ,;
        source . tell ( ) - len ( condname ) - 1 ) ,;
        DeprecationWarning , stacklevel = nested + 6;
        );
        state . checklookbehindgroup ( condgroup , source );
        item_yes = _parse ( source , state , verbose , nested + 1 );
        if source . match ( "|" ) {
        item_no = _parse ( source , state , verbose , nested + 1 );
        if source . next == "|" {
        panic!("source . error ( "conditional backref with more than two branches" )");
        } else {
        item_no = None /* Option */;
        if !source . match ( ")" ) {
        panic!("source . error ( "missing ), unterminated subpattern" ,");
        source . tell ( ) - start );
        subpatternappend ( ( GROUPREF_EXISTS , ( condgroup , item_yes , item_no ) ) );
        continue;
        } else if char == ">" {
        capture = false;
        atomic = true;
        } else if char in FLAGS || char == "-" {
        flags = _parse_flags ( source , state , char );
        if flags is None /* Option */ {
        if !first || subpattern {
        panic!("source . error ( "global flags !at the start "");
        "of the expression" ,;
        source . tell ( ) - start );
        verbose = state . flags & SRE_FLAG_VERBOSE;
        continue;
        add_flags , del_flags = flags;
        capture = false;
        } else {
        panic!("source . error ( "unknown extension ?" + char ,");
        len ( char ) + 1 );
        if capture {
        // try {
        group = state . opengroup ( name );
        // } catch  error as err  {
        panic!("source . error ( err . msg , len ( name ) + 1 ) from None /* Option */");
        } else {
        group = None /* Option */;
        sub_verbose = ( ( verbose || ( add_flags & SRE_FLAG_VERBOSE ) ) and;
        not ( del_flags & SRE_FLAG_VERBOSE ) );
        p = _parse_sub ( source , state , sub_verbose , nested + 1 );
        if !source . match ( ")" ) {
        panic!("source . error ( "missing ), unterminated subpattern" ,");
        source . tell ( ) - start );
        if group is !None /* Option */ {
        state . closegroup ( group , p );
        if atomic {
        assert group == None /* Option */;
        subpatternappend ( ( ATOMIC_GROUP , p ) );
        } else {
        subpatternappend ( ( SUBPATTERN , ( group , add_flags , del_flags , p ) ) );
        } else if this == "^" {
        subpatternappend ( ( AT , AT_BEGINNING ) );
        } else if this == "$" {
        subpatternappend ( ( AT , AT_END ) );
        } else {
        panic!("AssertionError ( "unsupported special character %r" % ( char , ) )");
        for i in range ( len ( subpattern ) ) [ : : -1 ] .iter() {
        op , av = subpattern [ i ];
        if op is SUBPATTERN {
        group , add_flags , del_flags , p = av;
        if group is None /* Option */ && !add_flags && !del_flags {
        subpattern [ i : i + 1 ] = p;
        return  subpattern;
        pub fn _parse_flags ( source , state , char )  {
        sourceget = source . get;
        add_flags = 0;
        del_flags = 0;
        if char != "-" {
        while true  {
        flag = FLAGS [ char ];
        if source . istext {
        if char == "L" {
        msg = "bad inline flags: cannot use 'L' flag with a str pattern";
        panic!("source . error ( msg )");
        } else {
        if char == "u" {
        msg = "bad inline flags: cannot use 'u' flag with a bytes pattern";
        panic!("source . error ( msg )");
        add_flags | = flag;
        if ( flag & TYPE_FLAGS ) && ( add_flags & TYPE_FLAGS ) != flag {
        msg = "bad inline flags: flags 'a', 'u' && 'L' are incompatible";
        panic!("source . error ( msg )");
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "missing -, : || )" )");
        if char in ")-:" {
        break;
        if char !in FLAGS {
        msg = "unknown flag" if char . isalpha ( ) else "missing -, : || )";
        panic!("source . error ( msg , len ( char ) )");
        if char == ")" {
        state . flags | = add_flags;
        return;
        if add_flags & GLOBAL_FLAGS {
        panic!("source . error ( "bad inline flags: cannot turn on global flag" , 1 )");
        if char == "-" {
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "missing flag" )");
        if char !in FLAGS {
        msg = "unknown flag" if char . isalpha ( ) else "missing flag";
        panic!("source . error ( msg , len ( char ) )");
        while true  {
        flag = FLAGS [ char ];
        if flag & TYPE_FLAGS {
        msg = "bad inline flags: cannot turn off flags 'a', 'u' && 'L'";
        panic!("source . error ( msg )");
        del_flags | = flag;
        char = sourceget ( );
        if char is None /* Option */ {
        panic!("source . error ( "missing :" )");
        if char == ":" {
        break;
        if char !in FLAGS {
        msg = "unknown flag" if char . isalpha ( ) else "missing :";
        panic!("source . error ( msg , len ( char ) )");
        assert char == ":";
        if del_flags & GLOBAL_FLAGS {
        panic!("source . error ( "bad inline flags: cannot turn off global flag" , 1 )");
        if add_flags & del_flags {
        panic!("source . error ( "bad inline flags: flag turned on && off" , 1 )");
        return  add_flags , del_flags;
        pub fn fix_flags ( src , flags )  {
        if isinstance ( src , str ) {
        if flags & SRE_FLAG_LOCALE {
        panic!("ValueError ( "cannot use LOCALE flag with a str pattern" )");
        if !flags & SRE_FLAG_ASCII {
        flags | = SRE_FLAG_UNICODE;
        } else if flags & SRE_FLAG_UNICODE {
        panic!("ValueError ( "ASCII && UNICODE flags are incompatible" )");
        } else {
        if flags & SRE_FLAG_UNICODE {
        panic!("ValueError ( "cannot use UNICODE flag with a bytes pattern" )");
        if flags & SRE_FLAG_LOCALE && flags & SRE_FLAG_ASCII {
        panic!("ValueError ( "ASCII && LOCALE flags are incompatible" )");
        return  flags;
        pub fn parse ( str , flags = 0 , state = None /* Option */ )  {
        source = Tokenizer ( str );
        if state is None /* Option */ {
        state = State ( );
        state . flags = flags;
        state . str = str;
        p = _parse_sub ( source , state , flags & SRE_FLAG_VERBOSE , 0 );
        p . state . flags = fix_flags ( str , p . state . flags );
        if source . next is !None /* Option */ {
        assert source . next == ")";
        panic!("source . error ( "unbalanced parenthesis" )");
        for g in p . state . grouprefpos .iter() {
        if g >= p . state . groups {
        msg = "invalid group reference %d" % g;
        panic!("error ( msg , str , p . state . grouprefpos [ g ] )");
        if flags & SRE_FLAG_DEBUG {
        p . dump ( );
        return  p;
        pub fn parse_template ( source , state )  {
        s = Tokenizer ( source );
        sget = s . get;
        groups = [ ];
        literals = [ ];
        literal = [ ];
        lappend = literal . append;
        pub fn addgroup ( index , pos )  {
        if index > state . groups {
        panic!("s . error ( "invalid group reference %d" % index , pos )");
        if literal {
        literals . append ( "" . join ( literal ) );
        del literal [ : ];
        groups . append ( ( len ( literals ) , index ) );
        literals . append ( None /* Option */ );
        groupindex = state . groupindex;
        while true  {
        this = sget ( );
        if this is None /* Option */ {
        break;
        if this [ 0 ] == "\\" {
        c = this [ 1 ];
        if c == "g" {
        if !s . match ( "<" ) {
        panic!("s . error ( "missing <" )");
        name = s . getuntil ( ">" , "group name" );
        if name . isidentifier ( ) {
        s . checkgroupname ( name , 1 , -1 );
        // try {
        index = groupindex [ name ];
        // } catch  KeyError  {
        panic!("IndexError ( "unknown group name %r" % name ) from None /* Option */");
        } else {
        // try {
        index = int ( name );
        if index < 0 {
        panic!("ValueError");
        // } catch  ValueError  {
        panic!("s . error ( "bad character in group name %r" % name ,");
        len ( name ) + 1 ) from None /* Option */;
        if index >= MAXGROUPS {
        panic!("s . error ( "invalid group reference %d" % index ,");
        len ( name ) + 1 );
        if !( name . isdecimal ( ) && name . isascii ( ) ) {
        import warnings;
        warnings . warn (;
        "bad character in group name %s at position %d" %;
        ( repr ( name ) if s . istext else ascii ( name ) ,;
        s . tell ( ) - len ( name ) - 1 ) ,;
        DeprecationWarning , stacklevel = 5;
        );
        addgroup ( index , len ( name ) + 1 );
        } else if c == "0" {
        if s . next in OCTDIGITS {
        this + = sget ( );
        if s . next in OCTDIGITS {
        this + = sget ( );
        lappend ( chr ( int ( this [ 1 : ] , 8 ) & 0x ff ) );
        } else if c in DIGITS {
        isoctal = false;
        if s . next in DIGITS {
        this + = sget ( );
        if ( c in OCTDIGITS && this [ 2 ] in OCTDIGITS and {
        s . next in OCTDIGITS ) ;
        this + = sget ( );
        isoctal = true;
        c = int ( this [ 1 : ] , 8 );
        if c > 0 o377 {
        panic!("s . error ( "octal escape value %s outside of "");
        "range 0-0o377" % this , len ( this ) );
        lappend ( chr ( c ) );
        if !isoctal {
        addgroup ( int ( this [ 1 : ] ) , len ( this ) - 1 );
        } else {
        // try {
        this = chr ( ESCAPES [ this ] [ 1 ] );
        // } catch  KeyError  {
        if c in ASCIILETTERS {
        panic!("s . error ( "bad escape %s" % this , len ( this ) ) from None /* Option */");
        lappend ( this );
        } else {
        lappend ( this );
        if literal {
        literals . append ( "" . join ( literal ) );
        if !isinstance ( source , str ) {
        literals = vec![ None /* Option */ if s == None /* Option */ else s . encode ( "latin-1" ).iter().map(|s| literals ).collect();
        return  groups , literals;
        pub fn expand_template ( template , match )  {
        g = match . group;
        empty = match . string [ : 0 ];
        groups , literals = template;
        literals = literals [ : ];
        // try {
        for index , group in groups .iter() {
        literals [ index ] = g ( group ) || empty;
        // } catch  IndexError  {
        panic!("error ( "invalid group reference %d" % index ) from None /* Option */");
        return  empty . join ( literals );
    }

}

