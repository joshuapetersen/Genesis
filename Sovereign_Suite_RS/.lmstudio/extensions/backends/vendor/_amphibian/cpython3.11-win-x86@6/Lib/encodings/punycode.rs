//! punycode.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub fn segregate(str: &str) {
        "3.1 Basic code point segregation";
        base = bytearray ( );
        extended = set ( );
        for c in str .iter() {
        if ord ( c ) < 128 {
        base . append ( ord ( c ) );
        } else {
        extended . add ( c );
        extended = sorted ( extended );
        return  bytes ( base ) , extended;
        pub fn selective_len ( str , max )  {
        "Return the length of str, considering only characters below max.";
        res = 0;
        for c in str .iter() {
        if ord ( c ) < max {
        res + = 1;
        return  res;
        pub fn selective_find ( str , char , index , pos )  {
        "Return a pair (index, pos), indicating the next occurrence of
    char in str. index == the position of the character considering
    only ordinals up to && including char, && pos == the position in
    the full string. index/pos == the starting position in the full
    string.";
        l = len ( str );
        while 1  {
        pos + = 1;
        if pos == l {
        return  ( -1 , -1 );
        c = str [ pos ];
        if c == char {
        return  index + 1 , pos;
        } else if c < char {
        index + = 1;
        pub fn insertion_unsort ( str , extended )  {
        "3.2 Insertion unsort coding";
        oldchar = 0x80;
        result = [ ];
        oldindex = -1;
        for c in extended .iter() {
        index = pos = -1;
        char = ord ( c );
        curlen = selective_len ( str , char );
        delta = ( curlen + 1 ) * ( char - oldchar );
        while 1  {
        index , pos = selective_find ( str , c , index , pos );
        if index == -1 {
        break;
        delta + = index - oldindex;
        result . append ( delta -1 );
        oldindex = index;
        delta = 0;
        oldchar = char;
        return  result;
        pub fn T ( j , bias )  {
        res = 36 * ( j + 1 ) - bias;
        if res < 1 { : return 1; }
        if res > 26 { : return 26; }
        return  res;
        digits = b "abcdefghijklmnopqrstuvwxyz0123456789";
        pub fn generate_generalized_integer ( N , bias )  {
        "3.3 Generalized variable-length integers";
        result = bytearray ( );
        j = 0;
        while 1  {
        t = T ( j , bias );
        if N < t {
        result . append ( digits [ N ] );
        return  bytes ( result );
        result . append ( digits [ t + ( ( N - t ) % ( 36 - t ) ) ] );
        N = ( N - t ) / / ( 36 - t );
        j + = 1;
        pub fn adapt ( delta , first , numchars )  {
        if first {
        delta / / = 700;
        } else {
        delta / / = 2;
        delta + = delta / / numchars;
        divisions = 0;
        while delta > 455  {
        delta = delta / / 35;
        divisions + = 36;
        bias = divisions + ( 36 * delta / / ( delta + 38 ) );
        return  bias;
        pub fn generate_integers ( baselen , deltas )  {
        "3.4 Bias adaptation";
        result = bytearray ( );
        bias = 72;
        for points , delta in enumerate ( deltas ) .iter() {
        s = generate_generalized_integer ( delta , bias );
        result . extend ( s );
        bias = adapt ( delta , points == 0 , baselen + points + 1 );
        return  bytes ( result );
        pub fn punycode_encode ( text )  {
        base , extended = segregate ( text );
        deltas = insertion_unsort ( text , extended );
        extended = generate_integers ( len ( base ) , deltas );
        if base {
        return  base + b "-" + extended;
        return  extended;
        pub fn decode_generalized_number ( extended , extpos , bias , errors )  {
        "3.3 Generalized variable-length integers";
        result = 0;
        w = 1;
        j = 0;
        while 1  {
        // try {
        char = ord ( extended [ extpos ] );
        // } catch  IndexError  {
        if errors == "strict" {
        panic!("UnicodeError ( "incomplete punicode string" )");
        return  extpos + 1 , None /* Option */;
        extpos + = 1;
        if 0x41 <= char <= 0x5 A {
        digit = char - 0x41;
        } else if 0x30 <= char <= 0x39 {
        digit = char - 22;
        } else if errors == "strict" {
        panic!("UnicodeError ( "Invalid extended code point '%s'"");
        % extended [ extpos -1 ] );
        } else {
        return  extpos , None /* Option */;
        t = T ( j , bias );
        result + = digit * w;
        if digit < t {
        return  extpos , result;
        w = w * ( 36 - t );
        j + = 1;
        pub fn insertion_sort ( base , extended , errors )  {
        "3.2 Insertion unsort coding";
        char = 0x80;
        pos = -1;
        bias = 72;
        extpos = 0;
        while extpos < len ( extended )  {
        newpos , delta = decode_generalized_number ( extended , extpos ,;
        bias , errors );
        if delta is None /* Option */ {
        return  base;
        pos + = delta + 1;
        char + = pos / / ( len ( base ) + 1 );
        if char > 0x10 FFFF {
        if errors == "strict" {
        panic!("UnicodeError ( "Invalid character U+%x" % char )");
        char = ord ( "?" );
        pos = pos % ( len ( base ) + 1 );
        base = base [ : pos ] + chr ( char ) + base [ pos : ];
        bias = adapt ( delta , ( extpos == 0 ) , len ( base ) );
        extpos = newpos;
        return  base;
        pub fn punycode_decode ( text , errors )  {
        if isinstance ( text , str ) {
        text = text . encode ( "ascii" );
        if isinstance ( text , memoryview ) {
        text = bytes ( text );
        pos = text . rfind ( b "-" );
        if pos == -1 {
        base = "";
        extended = str ( text , "ascii" ) . upper ( );
        } else {
        base = str ( text [ : pos ] , "ascii" , errors );
        extended = str ( text [ pos + 1 : ] , "ascii" ) . upper ( );
        return  insertion_sort ( base , extended , errors );
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        res = punycode_encode ( input );
        return  res , len ( input );
        pub fn decode ( &self, input , errors = "strict" )  {
        if errors !in ( "strict" , "replace" , "ignore" ) {
        panic!("UnicodeError ( "Unsupported error handling " + errors )");
        res = punycode_decode ( input , errors );
        return  res , len ( input );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        return  punycode_encode ( input );
        class IncrementalDecoder ( codecs . IncrementalDecoder ) ;
        pub fn decode ( &self, input , final = false )  {
        if self . errors !in ( "strict" , "replace" , "ignore" ) {
        panic!("UnicodeError ( "Unsupported error handling " + self . errors )");
        return  punycode_decode ( input , self . errors );
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        // pass
        class StreamReader ( Codec , codecs . StreamReader ) ;
        // pass
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "punycode" ,;
        encode = Codec ( ) . encode ,;
        decode = Codec ( ) . decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        );
}

