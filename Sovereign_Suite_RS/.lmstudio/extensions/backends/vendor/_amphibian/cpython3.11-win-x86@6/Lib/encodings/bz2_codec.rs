//! bz2_codec.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub fn bz2_encode(input: &str, errors: &str) {
        assert errors == "strict";
        return  ( bz2 . compress ( input ) , len ( input ) );
        pub fn bz2_decode ( input , errors = "strict" )  {
        assert errors == "strict";
        return  ( bz2 . decompress ( input ) , len ( input ) );
        class Codec ( codecs . Codec ) ;
        pub fn encode ( &self, input , errors = "strict" )  {
        return  bz2_encode ( input , errors );
        pub fn decode ( &self, input , errors = "strict" )  {
        return  bz2_decode ( input , errors );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        assert errors == "strict";
        self . errors = errors;
        self . compressobj = bz2 . BZ2Compressor ( );
        pub fn encode ( &self, input , final = false )  {
        if final {
        c = self . compressobj . compress ( input );
        return  c + self . compressobj . flush ( );
        } else {
        return  self . compressobj . compress ( input );
        pub fn reset ( self )  {
        self . compressobj = bz2 . BZ2Compressor ( );
        class IncrementalDecoder ( codecs . IncrementalDecoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        assert errors == "strict";
        self . errors = errors;
        self . decompressobj = bz2 . BZ2Decompressor ( );
        pub fn decode ( &self, input , final = false )  {
        // try {
        return  self . decompressobj . decompress ( input );
        // } catch  EOFError  {
        return  "";
        pub fn reset ( self )  {
        self . decompressobj = bz2 . BZ2Decompressor ( );
        class StreamWriter ( Codec , codecs . StreamWriter ) ;
        charbuffertype = bytes;
        class StreamReader ( Codec , codecs . StreamReader ) ;
        charbuffertype = bytes;
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "bz2" ,;
        encode = bz2_encode ,;
        decode = bz2_decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        _is_text_encoding = false ,;
        );
}

