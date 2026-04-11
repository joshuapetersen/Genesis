//! rot_13.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;
// use std::env;

pub struct Codec {
}

impl Codec {
    pub fn encode(&self, input: &str, errors: &str) {
        return  ( str . translate ( input , rot13_map ) , len ( input ) );
    }

    pub fn getregentry(&self) {
        return  codecs . CodecInfo (;
        name = "rot-13" ,;
        encode = Codec ( ) . encode ,;
        decode = Codec ( ) . decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamwriter = StreamWriter ,;
        streamreader = StreamReader ,;
        _is_text_encoding = false ,;
        );
        rot13_map = codecs . make_identity_dict ( range ( 256 ) );
        rot13_map . update ( {;
        0x0041 : 0x004e ,;
        0x0042 : 0x004 f ,;
        0x0043 : 0x0050 ,;
        0x0044 : 0x0051 ,;
        0x0045 : 0x0052 ,;
        0x0046 : 0x0053 ,;
        0x0047 : 0x0054 ,;
        0x0048 : 0x0055 ,;
        0x0049 : 0x0056 ,;
        0x004 a : 0x0057 ,;
        0x004 b : 0x0058 ,;
        0x004 c : 0x0059 ,;
        0x004 d : 0x005 a ,;
        0x004e : 0x0041 ,;
        0x004 f : 0x0042 ,;
        0x0050 : 0x0043 ,;
        0x0051 : 0x0044 ,;
        0x0052 : 0x0045 ,;
        0x0053 : 0x0046 ,;
        0x0054 : 0x0047 ,;
        0x0055 : 0x0048 ,;
        0x0056 : 0x0049 ,;
        0x0057 : 0x004 a ,;
        0x0058 : 0x004 b ,;
        0x0059 : 0x004 c ,;
        0x005 a : 0x004 d ,;
        0x0061 : 0x006e ,;
        0x0062 : 0x006 f ,;
        0x0063 : 0x0070 ,;
        0x0064 : 0x0071 ,;
        0x0065 : 0x0072 ,;
        0x0066 : 0x0073 ,;
        0x0067 : 0x0074 ,;
        0x0068 : 0x0075 ,;
        0x0069 : 0x0076 ,;
        0x006 a : 0x0077 ,;
        0x006 b : 0x0078 ,;
        0x006 c : 0x0079 ,;
        0x006 d : 0x007 a ,;
        0x006e : 0x0061 ,;
        0x006 f : 0x0062 ,;
        0x0070 : 0x0063 ,;
        0x0071 : 0x0064 ,;
        0x0072 : 0x0065 ,;
        0x0073 : 0x0066 ,;
        0x0074 : 0x0067 ,;
        0x0075 : 0x0068 ,;
        0x0076 : 0x0069 ,;
        0x0077 : 0x006 a ,;
        0x0078 : 0x006 b ,;
        0x0079 : 0x006 c ,;
        0x007 a : 0x006 d ,;
        } );
        pub fn rot13 ( infile , outfile )  {
        outfile . write ( codecs . encode ( infile . read ( ) , "rot-13" ) );
        fn main() {
        import sys;
        rot13 ( sys . stdin , sys . stdout );
    }

}

