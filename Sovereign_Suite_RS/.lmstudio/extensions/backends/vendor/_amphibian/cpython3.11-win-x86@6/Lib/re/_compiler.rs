//! _compiler.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_sre;
// use crate::_parser;
// use crate::_EXTRA_CASES;
// use std::env;

pub const _LITERAL_CODES: f64 = { LITERAL , NOT_LITERAL };
pub const _SUCCESS_CODES: f64 = { SUCCESS , FAILURE };
pub const _ASSERT_CODES: f64 = { ASSERT , ASSERT_NOT };
pub const _UNIT_CODES: f64 = _LITERAL_CODES | { ANY , IN };
pub const _REPEATING_CODES: f64 = {;
pub fn _combine_flags(flags: &str, add_flags: &str, del_flags: &str, TYPE_FLAGS: &str, _parser: &str, TYPE_FLAGS: &str) {
        // pass
}

pub fn _compile(code: &str, pattern: &str, flags: &str) {
        emit = code . append;
        _len = len;
        LITERAL_CODES = _LITERAL_CODES;
        REPEATING_CODES = _REPEATING_CODES;
        SUCCESS_CODES = _SUCCESS_CODES;
        ASSERT_CODES = _ASSERT_CODES;
        iscased = None /* Option */;
        tolower = None /* Option */;
        fixes = None /* Option */;
        if flags & SRE_FLAG_IGNORECASE && !flags & SRE_FLAG_LOCALE {
        if flags & SRE_FLAG_UNICODE {
        iscased = _sre . unicode_iscased;
        tolower = _sre . unicode_tolower;
        fixes = _EXTRA_CASES;
        } else {
        iscased = _sre . ascii_iscased;
        tolower = _sre . ascii_tolower;
        for op , av in pattern .iter() {
        if op in LITERAL_CODES {
        if !flags & SRE_FLAG_IGNORECASE {
        emit ( op );
        emit ( av );
        } else if flags & SRE_FLAG_LOCALE {
        emit ( OP_LOCALE_IGNORE [ op ] );
        emit ( av );
        } else if !iscased ( av ) {
        emit ( op );
        emit ( av );
        } else {
        lo = tolower ( av );
        if !fixes {
        emit ( OP_IGNORE [ op ] );
        emit ( lo );
        } else if lo !in fixes {
        emit ( OP_UNICODE_IGNORE [ op ] );
        emit ( lo );
        } else {
        emit ( IN_UNI_IGNORE );
        skip = _len ( code ) ; emit ( 0 );
        if op is NOT_LITERAL {
        emit ( NEGATE );
        for k in ( lo , ) + fixes [ lo ] .iter() {
        emit ( LITERAL );
        emit ( k );
        emit ( FAILURE );
        code [ skip ] = _len ( code ) - skip;
        } else if op is IN {
        charset , hascased = _optimize_charset ( av , iscased , tolower , fixes );
        if flags & SRE_FLAG_IGNORECASE && flags & SRE_FLAG_LOCALE {
        emit ( IN_LOC_IGNORE );
        } else if !hascased {
        emit ( IN );
        } else if !fixes {
        emit ( IN_IGNORE );
        } else {
        emit ( IN_UNI_IGNORE );
        skip = _len ( code ) ; emit ( 0 );
        _compile_charset ( charset , flags , code );
        code [ skip ] = _len ( code ) - skip;
        } else if op is ANY {
        if flags & SRE_FLAG_DOTALL {
        emit ( ANY_ALL );
        } else {
        emit ( ANY );
        } else if op in REPEATING_CODES {
        if flags & SRE_FLAG_TEMPLATE {
        panic!("error ( "internal: unsupported template operator %r" % ( op , ) )");
        if _simple ( av [ 2 ] ) {
        emit ( REPEATING_CODES [ op ] [ 2 ] );
        skip = _len ( code ) ; emit ( 0 );
        emit ( av [ 0 ] );
        emit ( av [ 1 ] );
        _compile ( code , av [ 2 ] , flags );
        emit ( SUCCESS );
        code [ skip ] = _len ( code ) - skip;
        } else {
        emit ( REPEATING_CODES [ op ] [ 0 ] );
        skip = _len ( code ) ; emit ( 0 );
        emit ( av [ 0 ] );
        emit ( av [ 1 ] );
        _compile ( code , av [ 2 ] , flags );
        code [ skip ] = _len ( code ) - skip;
        emit ( REPEATING_CODES [ op ] [ 1 ] );
        } else if op is SUBPATTERN {
        group , add_flags , del_flags , p = av;
        if group {
        emit ( MARK );
        emit ( ( group -1 ) * 2 );
        _compile ( code , p , _combine_flags ( flags , add_flags , del_flags ) );
        if group {
        emit ( MARK );
        emit ( ( group -1 ) * 2 + 1 );
        } else if op is ATOMIC_GROUP {
        emit ( ATOMIC_GROUP );
        skip = _len ( code ) ; emit ( 0 );
        _compile ( code , av , flags );
        emit ( SUCCESS );
        code [ skip ] = _len ( code ) - skip;
        } else if op in SUCCESS_CODES {
        emit ( op );
        } else if op in ASSERT_CODES {
        emit ( op );
        skip = _len ( code ) ; emit ( 0 );
        if av [ 0 ] >= 0 {
        emit ( 0 );
        } else {
        lo , hi = av [ 1 ] . getwidth ( );
        if lo > MAXCODE {
        panic!("error ( "looks too much behind" )");
        if lo != hi {
        panic!("error ( "look-behind requires fixed-width pattern" )");
        emit ( lo );
        _compile ( code , av [ 1 ] , flags );
        emit ( SUCCESS );
        code [ skip ] = _len ( code ) - skip;
        } else if op is AT {
        emit ( op );
        if flags & SRE_FLAG_MULTILINE {
        av = AT_MULTILINE . get ( av , av );
        if flags & SRE_FLAG_LOCALE {
        av = AT_LOCALE . get ( av , av );
        } else if flags & SRE_FLAG_UNICODE {
        av = AT_UNICODE . get ( av , av );
        emit ( av );
        } else if op is BRANCH {
        emit ( op );
        tail = [ ];
        tailappend = tail . append;
        for av in av [ 1 ] .iter() {
        skip = _len ( code ) ; emit ( 0 );
        _compile ( code , av , flags );
        emit ( JUMP );
        tailappend ( _len ( code ) ) ; emit ( 0 );
        code [ skip ] = _len ( code ) - skip;
        emit ( FAILURE );
        for tail in tail .iter() {
        code [ tail ] = _len ( code ) - tail;
        } else if op is CATEGORY {
        emit ( op );
        if flags & SRE_FLAG_LOCALE {
        av = CH_LOCALE [ av ];
        } else if flags & SRE_FLAG_UNICODE {
        av = CH_UNICODE [ av ];
        emit ( av );
        } else if op is GROUPREF {
        if !flags & SRE_FLAG_IGNORECASE {
        emit ( op );
        } else if flags & SRE_FLAG_LOCALE {
        emit ( GROUPREF_LOC_IGNORE );
        } else if !fixes {
        emit ( GROUPREF_IGNORE );
        } else {
        emit ( GROUPREF_UNI_IGNORE );
        emit ( av -1 );
        } else if op is GROUPREF_EXISTS {
        emit ( op );
        emit ( av [ 0 ] -1 );
        skipyes = _len ( code ) ; emit ( 0 );
        _compile ( code , av [ 1 ] , flags );
        if av [ 2 ] {
        emit ( JUMP );
        skipno = _len ( code ) ; emit ( 0 );
        code [ skipyes ] = _len ( code ) - skipyes + 1;
        _compile ( code , av [ 2 ] , flags );
        code [ skipno ] = _len ( code ) - skipno;
        } else {
        code [ skipyes ] = _len ( code ) - skipyes + 1;
        } else {
        panic!("error ( "internal: unsupported operand type %r" % ( op , ) )");
        pub fn _compile_charset ( charset , flags , code )  {
        emit = code . append;
        for op , av in charset .iter() {
        emit ( op );
        if op is NEGATE {
        // pass
        } else if op is LITERAL {
        emit ( av );
        } else if op is RANGE || op is RANGE_UNI_IGNORE {
        emit ( av [ 0 ] );
        emit ( av [ 1 ] );
        } else if op is CHARSET {
        code . extend ( av );
        } else if op is BIGCHARSET {
        code . extend ( av );
        } else if op is CATEGORY {
        if flags & SRE_FLAG_LOCALE {
        emit ( CH_LOCALE [ av ] );
        } else if flags & SRE_FLAG_UNICODE {
        emit ( CH_UNICODE [ av ] );
        } else {
        emit ( av );
        } else {
        panic!("error ( "internal: unsupported set operator %r" % ( op , ) )");
        emit ( FAILURE );
        pub fn _optimize_charset ( charset , iscased = None /* Option */ , fixup = None /* Option */ , fixes = None /* Option */ )  {
        out = [ ];
        tail = [ ];
        charmap = bytearray ( 256 );
        hascased = false;
        for op , av in charset .iter() {
        while true  {
        // try {
        if op is LITERAL {
        if fixup {
        lo = fixup ( av );
        charmap [ lo ] = 1;
        if fixes && lo in fixes {
        for k in fixes [ lo ] .iter() {
        charmap [ k ] = 1;
        if !hascased && iscased ( av ) {
        hascased = true;
        } else {
        charmap [ av ] = 1;
        } else if op is RANGE {
        r = range ( av [ 0 ] , av [ 1 ] + 1 );
        if fixup {
        if fixes {
        for i in map ( fixup , r ) .iter() {
        charmap [ i ] = 1;
        if i in fixes {
        for k in fixes [ i ] .iter() {
        charmap [ k ] = 1;
        } else {
        for i in map ( fixup , r ) .iter() {
        charmap [ i ] = 1;
        if !hascased {
        hascased = any ( map ( iscased , r ) );
        } else {
        for i in r .iter() {
        charmap [ i ] = 1;
        } else if op is NEGATE {
        out . append ( ( op , av ) );
        } else {
        tail . append ( ( op , av ) );
        // } catch  IndexError  {
        if len ( charmap ) == 256 {
        charmap + = b "\0" * 0x ff00;
        continue;
        if fixup {
        hascased = true;
        if op is RANGE {
        op = RANGE_UNI_IGNORE;
        tail . append ( ( op , av ) );
        break;
        runs = [ ];
        q = 0;
        while true  {
        p = charmap . find ( 1 , q );
        if p < 0 {
        break;
        if len ( runs ) >= 2 {
        runs = None /* Option */;
        break;
        q = charmap . find ( 0 , p );
        if q < 0 {
        runs . append ( ( p , len ( charmap ) ) );
        break;
        runs . append ( ( p , q ) );
        if runs is !None /* Option */ {
        for p , q in runs .iter() {
        if q - p == 1 {
        out . append ( ( LITERAL , p ) );
        } else {
        out . append ( ( RANGE , ( p , q - 1 ) ) );
        out + = tail;
        if hascased || len ( out ) < len ( charset ) {
        return  out , hascased;
        return  charset , hascased;
        if len ( charmap ) == 256 {
        data = _mk_bitmap ( charmap );
        out . append ( ( CHARSET , data ) );
        out + = tail;
        return  out , hascased;
        charmap = bytes ( charmap );
        comps = { };
        mapping = bytearray ( 256 );
        block = 0;
        data = bytearray ( );
        for i in range ( 0 , 65536 , 256 ) .iter() {
        chunk = charmap [ i : i + 256 ];
        if chunk in comps {
        mapping [ i / / 256 ] = comps [ chunk ];
        } else {
        mapping [ i / / 256 ] = comps [ chunk ] = block;
        block + = 1;
        data + = chunk;
        data = _mk_bitmap ( data );
        data [ 0 : 0 ] = [ block ] + _bytes_to_codes ( mapping );
        out . append ( ( BIGCHARSET , data ) );
        out + = tail;
        return  out , hascased;
        _CODEBITS = _sre . CODESIZE * 8;
        MAXCODE = ( 1 < < _CODEBITS ) - 1;
        _BITS_TRANS = b "0" + b "1" * 255;
        pub fn _mk_bitmap ( bits , _CODEBITS = _CODEBITS , _int = int )  {
        s = bits . translate ( _BITS_TRANS ) [ : : -1 ];
        return  [ _int ( s [ i - _CODEBITS : i ] , 2 );
        for i in range ( len ( s ) , 0 , - _CODEBITS ) ].iter() {
        pub fn _bytes_to_codes ( b )  {
        a = memoryview ( b ) . cast ( "I" );
        assert a . itemsize == _sre . CODESIZE;
        assert len ( a ) * a . itemsize == len ( b );
        return  a . tolist ( );
        pub fn _simple ( p )  {
        if len ( p ) != 1 {
        return  false;
        op , av = p [ 0 ];
        if op is SUBPATTERN {
        return  av [ 0 ] is None /* Option */ && _simple ( av [ -1 ] );
        return  op in _UNIT_CODES;
        pub fn _generate_overlap_table ( prefix )  {
        "
    Generate an overlap table.iter().map(|the following prefix.
    An overlap table == a table of the same size as the prefix which
    informs about the potential self-overlap.iter().map(|each index| the prefix:
    - if overlapvec![i] == 0, prefixvec![i:] can't overlap prefixvec![0:...]
    - if overlapvec![i] == k with 0 < k <= i, prefixvec![i-k+1:i+1] overlaps with
      prefixvec![0:k]
    ";
        table = [ 0 ] * len ( prefix );
        for i in range ( 1 , len ( prefix ) ) .iter() {
        idx = table [ i - 1 ];
        while prefix [ i ] != prefix [ idx ]  {
        if idx == 0 {
        table [ i ] = 0;
        break;
        idx = table [ idx - 1 ];
        } else {
        table [ i ] = idx + 1;
        return  table;
        pub fn _get_iscased ( flags )  {
        if !flags & SRE_FLAG_IGNORECASE {
        return;
        } else if flags & SRE_FLAG_UNICODE {
        return  _sre . unicode_iscased;
        } else {
        return  _sre . ascii_iscased;
        pub fn _get_literal_prefix ( pattern , flags )  {
        prefix = [ ];
        prefixappend = prefix . append;
        prefix_skip = None /* Option */;
        iscased = _get_iscased ( flags );
        for op , av in pattern . data .iter() {
        if op is LITERAL {
        if iscased && iscased ( av ) {
        break;
        prefixappend ( av );
        } else if op is SUBPATTERN {
        group , add_flags , del_flags , p = av;
        flags1 = _combine_flags ( flags , add_flags , del_flags );
        if flags1 & SRE_FLAG_IGNORECASE && flags1 & SRE_FLAG_LOCALE {
        break;
        prefix1 , prefix_skip1 , got_all = _get_literal_prefix ( p , flags1 );
        if prefix_skip is None /* Option */ {
        if group is !None /* Option */ {
        prefix_skip = len ( prefix );
        } else if prefix_skip1 is !None /* Option */ {
        prefix_skip = len ( prefix ) + prefix_skip1;
        prefix . extend ( prefix1 );
        if !got_all {
        break;
        } else {
        break;
        } else {
        return  prefix , prefix_skip , true;
        return  prefix , prefix_skip , false;
        pub fn _get_charset_prefix ( pattern , flags )  {
        while true  {
        if !pattern . data {
        return;
        op , av = pattern . data [ 0 ];
        if op is !SUBPATTERN {
        break;
        group , add_flags , del_flags , pattern = av;
        flags = _combine_flags ( flags , add_flags , del_flags );
        if flags & SRE_FLAG_IGNORECASE && flags & SRE_FLAG_LOCALE {
        return;
        iscased = _get_iscased ( flags );
        if op is LITERAL {
        if iscased && iscased ( av ) {
        return;
        return  [ ( op , av ) ];
        } else if op is BRANCH {
        charset = [ ];
        charsetappend = charset . append;
        for p in av [ 1 ] .iter() {
        if !p {
        return;
        op , av = p [ 0 ];
        if op is LITERAL && !( iscased && iscased ( av ) ) {
        charsetappend ( ( op , av ) );
        } else {
        return;
        return  charset;
        } else if op is IN {
        charset = av;
        if iscased {
        for op , av in charset .iter() {
        if op is LITERAL {
        if iscased ( av ) {
        return;
        } else if op is RANGE {
        if av [ 1 ] > 0x ffff {
        return;
        if any ( map ( iscased , range ( av [ 0 ] , av [ 1 ] + 1 ) ) ) {
        return;
        return  charset;
        return;
        pub fn _compile_info ( code , pattern , flags )  {
        lo , hi = pattern . getwidth ( );
        if hi > MAXCODE {
        hi = MAXCODE;
        if lo == 0 {
        code . extend ( [ INFO , 4 , 0 , lo , hi ] );
        return;
        prefix = [ ];
        prefix_skip = 0;
        charset = [ ];
        if !( flags & SRE_FLAG_IGNORECASE && flags & SRE_FLAG_LOCALE ) {
        prefix , prefix_skip , got_all = _get_literal_prefix ( pattern , flags );
        if !prefix {
        charset = _get_charset_prefix ( pattern , flags );
        emit = code . append;
        emit ( INFO );
        skip = len ( code ) ; emit ( 0 );
        mask = 0;
        if prefix {
        mask = SRE_INFO_PREFIX;
        if prefix_skip is None /* Option */ && got_all {
        mask = mask | SRE_INFO_LITERAL;
        } else if charset {
        mask = mask | SRE_INFO_CHARSET;
        emit ( mask );
        if lo < MAXCODE {
        emit ( lo );
        } else {
        emit ( MAXCODE );
        prefix = prefix [ : MAXCODE ];
        emit ( hi );
        if prefix {
        emit ( len ( prefix ) );
        if prefix_skip is None /* Option */ {
        prefix_skip = len ( prefix );
        emit ( prefix_skip );
        code . extend ( prefix );
        code . extend ( _generate_overlap_table ( prefix ) );
        } else if charset {
        charset , hascased = _optimize_charset ( charset );
        assert !hascased;
        _compile_charset ( charset , flags , code );
        code [ skip ] = len ( code ) - skip;
        pub fn isstring ( obj )  {
        return  isinstance ( obj , ( str , bytes ) );
        pub fn _code ( p , flags )  {
        flags = p . state . flags | flags;
        code = [ ];
        _compile_info ( code , p , flags );
        _compile ( code , p . data , flags );
        code . append ( SUCCESS );
        return  code;
        pub fn _hex_code ( code )  {
        return  "[%s]" % ", " . join ( "%#0*x" % ( _sre . CODESIZE * 2 + 2 , x ) for x in code );
        pub fn dis ( code )  {
        import sys;
        labels = set ( );
        level = 0;
        offset_width = len ( str ( len ( code ) - 1 ) );
        pub fn dis_ ( start , end )  {
        pub fn print_ ( * args , to = None /* Option */ )  {
        if to is !None /* Option */ {
        labels . add ( to );
        args + = ( "(to %d)" % ( to , ) , );
        println!( "%*d%s " % ( offset_width , start , ":" if start in labels else "." ) );
        end = "  " * ( level -1 ) );
        println!( * args );
        pub fn print_2 ( * args )  {
        println!( end = " " * ( offset_width + 2 * level ) );
        println!( * args );
        nonlocal level;
        level + = 1;
        i = start;
        while i < end  {
        start = i;
        op = code [ i ];
        i + = 1;
        op = OPCODES [ op ];
        if op in ( SUCCESS , FAILURE , ANY , ANY_ALL , {
        MAX_UNTIL , MIN_UNTIL , NEGATE ) ;
        println!( op );
        } else if op in ( LITERAL , NOT_LITERAL , {
        LITERAL_IGNORE , NOT_LITERAL_IGNORE ,;
        LITERAL_UNI_IGNORE , NOT_LITERAL_UNI_IGNORE ,;
        LITERAL_LOC_IGNORE , NOT_LITERAL_LOC_IGNORE ) ;
        arg = code [ i ];
        i + = 1;
        println!( op , "%#02x (%r)" % ( arg , chr ( arg ) ) );
        } else if op is AT {
        arg = code [ i ];
        i + = 1;
        arg = str ( ATCODES [ arg ] );
        assert arg [ : 3 ] == "AT_";
        println!( op , arg [ 3 : ] );
        } else if op is CATEGORY {
        arg = code [ i ];
        i + = 1;
        arg = str ( CHCODES [ arg ] );
        assert arg [ : 9 ] == "CATEGORY_";
        println!( op , arg [ 9 : ] );
        } else if op in ( IN , IN_IGNORE , IN_UNI_IGNORE , IN_LOC_IGNORE ) {
        skip = code [ i ];
        println!( op , skip , to = i + skip );
        dis_ ( i + 1 , i + skip );
        i + = skip;
        } else if op in ( RANGE , RANGE_UNI_IGNORE ) {
        lo , hi = code [ i : i + 2 ];
        i + = 2;
        println!( op , "%#02x %#02x (%r-%r)" % ( lo , hi , chr ( lo ) , chr ( hi ) ) );
        } else if op is CHARSET {
        println!( op , _hex_code ( code [ i : i + 256 / / _CODEBITS ] ) );
        i + = 256 / / _CODEBITS;
        } else if op is BIGCHARSET {
        arg = code [ i ];
        i + = 1;
        mapping = list ( b "" . join ( x . to_bytes ( _sre . CODESIZE , sys . byteorder );
        for x in code [ i : i + 256 / / _sre . CODESIZE ] ) ).iter() {
        println!( op , arg , mapping );
        i + = 256 / / _sre . CODESIZE;
        level + = 1;
        for j in range ( arg ) .iter() {
        println!( _hex_code ( code [ i : i + 256 / / _CODEBITS ] ) );
        i + = 256 / / _CODEBITS;
        level - = 1;
        } else if op in ( MARK , GROUPREF , GROUPREF_IGNORE , GROUPREF_UNI_IGNORE , {
        GROUPREF_LOC_IGNORE ) ;
        arg = code [ i ];
        i + = 1;
        println!( op , arg );
        } else if op is JUMP {
        skip = code [ i ];
        println!( op , skip , to = i + skip );
        i + = 1;
        } else if op is BRANCH {
        skip = code [ i ];
        println!( op , skip , to = i + skip );
        while skip  {
        dis_ ( i + 1 , i + skip );
        i + = skip;
        start = i;
        skip = code [ i ];
        if skip {
        println!( "branch" , skip , to = i + skip );
        } else {
        println!( FAILURE );
        i + = 1;
        } else if op in ( REPEAT , REPEAT_ONE , MIN_REPEAT_ONE , {
        POSSESSIVE_REPEAT , POSSESSIVE_REPEAT_ONE ) ;
        skip , min , max = code [ i : i + 3 ];
        if max == MAXREPEAT {
        max = "MAXREPEAT";
        println!( op , skip , min , max , to = i + skip );
        dis_ ( i + 3 , i + skip );
        i + = skip;
        } else if op is GROUPREF_EXISTS {
        arg , skip = code [ i : i + 2 ];
        println!( op , arg , skip , to = i + skip );
        i + = 2;
        } else if op in ( ASSERT , ASSERT_NOT ) {
        skip , arg = code [ i : i + 2 ];
        println!( op , skip , arg , to = i + skip );
        dis_ ( i + 2 , i + skip );
        i + = skip;
        } else if op is ATOMIC_GROUP {
        skip = code [ i ];
        println!( op , skip , to = i + skip );
        dis_ ( i + 1 , i + skip );
        i + = skip;
        } else if op is INFO {
        skip , flags , min , max = code [ i : i + 4 ];
        if max == MAXREPEAT {
        max = "MAXREPEAT";
        println!( op , skip , bin ( flags ) , min , max , to = i + skip );
        start = i + 4;
        if flags & SRE_INFO_PREFIX {
        prefix_len , prefix_skip = code [ i + 4 : i + 6 ];
        println!( "  prefix_skip" , prefix_skip );
        start = i + 6;
        prefix = code [ start : start + prefix_len ];
        println!( "  prefix" );
        "vec![%s]" % ", " . join ( "%#02x" % x.iter().map(|x| prefix ) ,;
        "(%r)" % "" . join ( map ( chr , prefix ) ) );
        start + = prefix_len;
        println!( "  overlap" , code [ start : start + prefix_len ] );
        start + = prefix_len;
        if flags & SRE_INFO_CHARSET {
        level + = 1;
        println!( "in" );
        dis_ ( start , i + skip );
        level - = 1;
        i + = skip;
        } else {
        panic!("ValueError ( op )");
        level - = 1;
        dis_ ( 0 , len ( code ) );
        pub fn compile ( p , flags = 0 )  {
        if isstring ( p ) {
        pattern = p;
        p = _parser . parse ( p , flags );
        } else {
        pattern = None /* Option */;
        code = _code ( p , flags );
        if flags & SRE_FLAG_DEBUG {
        println!( );
        dis ( code );
        groupindex = p . state . groupdict;
        indexgroup = [ None /* Option */ ] * p . state . groups;
        for k , i in groupindex . items ( ) .iter() {
        indexgroup [ i ] = k;
        return  _sre . compile (;
        pattern , flags | p . state . flags , code ,;
        p . state . groups -1 ,;
        groupindex , tuple ( indexgroup );
        );
}

