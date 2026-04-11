//! quopri_codec.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;
// use crate::io::{BytesIO};

pub fn quopri_encode(input: &str, errors: &str) {
        assert errors == "strict";
        f = BytesIO ( input );
        g = BytesIO ( );
        quopri . encode ( f , g , quotetabs = true );
        return  ( g . getvalue ( ) , len ( input ) );
        pub fn quopri_decode ( input , errors = "strict" )  {
        assert errors == "strict";
        f = BytesIO ( input );
        g = BytesIO ( );
        quopri . decode ( f , g );
        return  ( g . getvalue ( ) , len ( input ) );
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        return  quopri_encode ( input , errors );
        pub fn decode ( &self, input , errors = "strict" )  {
        return  quopri_decode ( input , errors );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        return  quopri_encode ( input , self . errors ) [ 0 ];
        class IncrementalDecoder ( codecs . IncrementalDecoder ) ;
        pub fn decode ( &self, input , final = false )  {
        return  quopri_decode ( input , self . errors ) [ 0 ];
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        charbuffertype = bytes;
        class StreamReader ( Codec , codecs . StreamReader ) ;
        charbuffertype = bytes;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "quopri" ,;
        encode = quopri_encode ,;
        decode = quopri_decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        _is_text_encoding = false ,;
        );
}

