//! chunk.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::warnings;
// use crate::struct;

pub const remove: f64 = ( 3 , 13 ) );
pub struct Chunk {
    pub closed: String, // TODO: infer type
    pub align: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub chunkname: String, // TODO: infer type
    pub chunksize: String, // TODO: infer type
    pub size_read: String, // TODO: infer type
    pub offset: String, // TODO: infer type
    pub seekable: String, // TODO: infer type
}

impl Chunk {
    pub fn new(file: &str, align: &str, bigendian: &str, inclheader: &str) -> Self {
        import struct;
        self . closed = false;
        self . align = align;
        if bigendian {
        strflag = ">";
        } else {
        strflag = "<";
        self . file = file;
        self . chunkname = file . read ( 4 );
        if len ( self . chunkname ) < 4 {
        panic!("EOFError");
        // try {
        self . chunksize = struct . unpack_from ( strflag + "L" , file . read ( 4 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        if inclheader {
        self . chunksize = self . chunksize - 8;
        self . size_read = 0;
        // try {
        self . offset = self . file . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . seekable = false;
        } else {
        self . seekable = true;
    }

}

