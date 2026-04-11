//! hex_codec.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub fn hex_encode(input: &str, errors: &str) {
        assert errors == "strict";
        return  ( binascii . b2a_hex ( input ) , len ( input ) );
        pub fn hex_decode ( input , errors = "strict" )  {
        assert errors == "strict";
        return  ( binascii . a2b_hex ( input ) , len ( input ) );
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        return  hex_encode ( input , errors );
        pub fn decode ( &self, input , errors = "strict" )  {
        return  hex_decode ( input , errors );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        assert self . errors == "strict";
        return  binascii . b2a_hex ( input );
        class IncrementalDecoder ( codecs . IncrementalDecoder ) ;
        pub fn decode ( &self, input , final = false )  {
        assert self . errors == "strict";
        return  binascii . a2b_hex ( input );
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        charbuffertype = bytes;
        class StreamReader ( Codec , codecs . StreamReader ) ;
        charbuffertype = bytes;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "hex" ,;
        encode = hex_encode ,;
        decode = hex_decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        _is_text_encoding = false ,;
        );
}

