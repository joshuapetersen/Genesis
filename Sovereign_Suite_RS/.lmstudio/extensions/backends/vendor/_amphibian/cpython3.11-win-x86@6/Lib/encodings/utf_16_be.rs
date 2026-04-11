//! utf_16_be.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub const encode: f64 = codecs . utf_16_be_encode;
pub fn decode(input: &str, errors: &str) {
        return  codecs . utf_16_be_decode ( input , errors , true );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        return  codecs . utf_16_be_encode ( input , self . errors ) [ 0 ];
        class IncrementalDecoder ( codecs . BufferedIncrementalDecoder ) ;
        _buffer_decode = codecs . utf_16_be_decode;
        class StreamWriter ( codecs . StreamWriter ) ;
        encode = codecs . utf_16_be_encode;
        class StreamReader ( codecs . StreamReader ) ;
        decode = codecs . utf_16_be_decode;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "utf-16-be" ,;
        encode = encode ,;
        decode = decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamreader = StreamReader ,;
        streamwriter = StreamWriter ,;
        );
}

