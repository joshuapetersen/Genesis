//! utf_8_sig.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub fn encode(input: &str, errors: &str) {
        return  ( codecs . BOM_UTF8 + codecs . utf_8_encode ( input , errors ) [ 0 ] ,;
        len ( input ) );
        pub fn decode ( input , errors = "strict" )  {
        prefix = 0;
        if input [ { : 3 ] == codecs . BOM_UTF8 ; }
        input = input [ 3 : ];
        prefix = 3;
        ( output , consumed ) = codecs . utf_8_decode ( input , errors , true );
        return  ( output , consumed + prefix );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        codecs . IncrementalEncoder . __init__ ( self , errors );
        self . first = 1;
        pub fn encode ( &self, input , final = false )  {
        if self . first {
        self . first = 0;
        return  codecs . BOM_UTF8 + \;
        codecs . utf_8_encode ( input , self . errors ) [ 0 ];
        } else {
        return  codecs . utf_8_encode ( input , self . errors ) [ 0 ];
        pub fn reset ( self )  {
        codecs . IncrementalEncoder . reset ( self );
        self . first = 1;
        pub fn getstate ( self )  {
        return  self . first;
        pub fn setstate ( &self, state )  {
        self . first = state;
        class IncrementalDecoder ( codecs . BufferedIncrementalDecoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        codecs . BufferedIncrementalDecoder . __init__ ( self , errors );
        self . first = 1;
        pub fn _buffer_decode ( &self, input , errors , final )  {
        if self . first {
        if len ( input ) < 3 {
        if codecs . BOM_UTF8 . startswith ( input ) {
        return  ( "" , 0 );
        } else {
        self . first = 0;
        } else {
        self . first = 0;
        if input [ { : 3 ] == codecs . BOM_UTF8 ; }
        ( output , consumed ) = \;
        codecs . utf_8_decode ( input [ 3 : ] , errors , final );
        return  ( output , consumed + 3 );
        return  codecs . utf_8_decode ( input , errors , final );
        pub fn reset ( self )  {
        codecs . BufferedIncrementalDecoder . reset ( self );
        self . first = 1;
        pub fn getstate ( self )  {
        state = codecs . BufferedIncrementalDecoder . getstate ( self );
        return  ( state [ 0 ] , self . first );
        pub fn setstate ( &self, state )  {
        codecs . BufferedIncrementalDecoder . setstate ( self , state );
        self . first = state [ 1 ];
        class StreamWriter ( codecs . StreamWriter ) ;
        pub fn reset ( self )  {
        codecs . StreamWriter . reset ( self );
        // try {
        del self . encode;
        // } catch  AttributeError  {
        // pass
        pub fn encode ( &self, input , errors = "strict" )  {
        self . encode = codecs . utf_8_encode;
        return  encode ( input , errors );
        class StreamReader ( codecs . StreamReader ) ;
        pub fn reset ( self )  {
        codecs . StreamReader . reset ( self );
        // try {
        del self . decode;
        // } catch  AttributeError  {
        // pass
        pub fn decode ( &self, input , errors = "strict" )  {
        if len ( input ) < 3 {
        if codecs . BOM_UTF8 . startswith ( input ) {
        return  ( "" , 0 );
        } else if input [ {
        self . decode = codecs . utf_8_decode;
        ( output , consumed ) = codecs . utf_8_decode ( input [ 3 : ] , errors );
        return  ( output , consumed + 3 );
        self . decode = codecs . utf_8_decode;
        return  codecs . utf_8_decode ( input , errors );
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "utf-8-sig" ,;
        encode = encode ,;
        decode = decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamreader = StreamReader ,;
        streamwriter = StreamWriter ,;
        );
}

