//! mbcs.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs::{mbcs_encode, mbcs_decode};

pub const encode: f64 = mbcs_encode;
pub fn decode(input: &str, errors: &str) {
        return  mbcs_decode ( input , errors , true );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn encode ( &self, input , final = false )  {
        return  mbcs_encode ( input , self . errors ) [ 0 ];
        class IncrementalDecoder ( codecs . BufferedIncrementalDecoder ) ;
        _buffer_decode = mbcs_decode;
        class StreamWriter ( codecs . StreamWriter ) ;
        encode = mbcs_encode;
        class StreamReader ( codecs . StreamReader ) ;
        decode = mbcs_decode;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "mbcs" ,;
        encode = encode ,;
        decode = decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamreader = StreamReader ,;
        streamwriter = StreamWriter ,;
        );
}

