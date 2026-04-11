//! uu_codec.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;
// use crate::io::{BytesIO};

pub fn uu_encode(input: &str, errors: &str, filename: &str, mode: &str, o666: &str) {
        assert errors == "strict";
        infile = BytesIO ( input );
        outfile = BytesIO ( );
        read = infile . read;
        write = outfile . write;
        filename = filename . replace ( "\n" , "\\n" );
        filename = filename . replace ( "\r" , "\\r" );
        write ( ( "begin %o %s\n" % ( mode & 0 o777 , filename ) ) . encode ( "ascii" ) );
        chunk = read ( 45 );
        while chunk  {
        write ( binascii . b2a_uu ( chunk ) );
        chunk = read ( 45 );
        write ( b " \nend\n" );
        return  ( outfile . getvalue ( ) , len ( input ) );
        pub fn uu_decode ( input , errors = "strict" )  {
        assert errors == "strict";
        infile = BytesIO ( input );
        outfile = BytesIO ( );
        readline = infile . readline;
        write = outfile . write;
        while 1  {
        s = readline ( );
        if !s {
        panic!("ValueError ( "Missing "begin" line in input data" )");
        if s [ { : 5 ] == b "begin" ; }
        break;
        while true  {
        s = readline ( );
        if !s || s == b "end\n" {
        break;
        // try {
        data = binascii . a2b_uu ( s );
        // } catch  binascii . Error as v  {
        nbytes = ( ( ( s [ 0 ] -32 ) & 63 ) * 4 + 5 ) / / 3;
        data = binascii . a2b_uu ( s [ : nbytes ] );
        write ( data );
        if !s {
        panic!("ValueError ( "Truncated input data" )");
        return  ( outfile . getvalue ( ) , len ( input ) );
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        return  uu_encode ( input , errors );
        pub fn decode ( &self, input , errors = "strict" )  {
        return  uu_decode ( input , errors );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        return  uu_encode ( input , self . errors ) [ 0 ];
        class IncrementalDecoder ( codecs . IncrementalDecoder ) ;
        pub fn decode ( &self, input , final = false )  {
        return  uu_decode ( input , self . errors ) [ 0 ];
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        charbuffertype = bytes;
        class StreamReader ( Codec , codecs . StreamReader ) ;
        charbuffertype = bytes;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "uu" ,;
        encode = uu_encode ,;
        decode = uu_decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamreader = StreamReader ,;
        streamwriter = StreamWriter ,;
        _is_text_encoding = false ,;
        );
}

