//! utf_32.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::codecs;

pub const encode: f64 = codecs . utf_32_encode;
pub fn decode(input: &str, errors: &str) {
        return  codecs . utf_32_decode ( input , errors , true );
        class IncrementalEncoder ( codecs . IncrementalEncoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        codecs . IncrementalEncoder . __init__ ( self , errors );
        self . encoder = None /* Option */;
        pub fn encode ( &self, input , final = false )  {
        if self . encoder is None /* Option */ {
        result = codecs . utf_32_encode ( input , self . errors ) [ 0 ];
        if sys . byteorder == "little" {
        self . encoder = codecs . utf_32_le_encode;
        } else {
        self . encoder = codecs . utf_32_be_encode;
        return  result;
        return  self . encoder ( input , self . errors ) [ 0 ];
        pub fn reset ( self )  {
        codecs . IncrementalEncoder . reset ( self );
        self . encoder = None /* Option */;
        pub fn getstate ( self )  {
        return  ( 2 if self . encoder is None /* Option */ else 0 );
        pub fn setstate ( &self, state )  {
        if state {
        self . encoder = None /* Option */;
        } else {
        if sys . byteorder == "little" {
        self . encoder = codecs . utf_32_le_encode;
        } else {
        self . encoder = codecs . utf_32_be_encode;
        class IncrementalDecoder ( codecs . BufferedIncrementalDecoder ) ;
        pub fn __init__ ( &self, errors = "strict" )  {
        codecs . BufferedIncrementalDecoder . __init__ ( self , errors );
        self . decoder = None /* Option */;
        pub fn _buffer_decode ( &self, input , errors , final )  {
        if self . decoder is None /* Option */ {
        ( output , consumed , byteorder ) = \;
        codecs . utf_32_ex_decode ( input , errors , 0 , final );
        if byteorder == -1 {
        self . decoder = codecs . utf_32_le_decode;
        } else if byteorder == 1 {
        self . decoder = codecs . utf_32_be_decode;
        } else if consumed >= 4 {
        panic!("UnicodeError ( "UTF-32 stream does !start with BOM" )");
        return  ( output , consumed );
        return  self . decoder ( input , self . errors , final );
        pub fn reset ( self )  {
        codecs . BufferedIncrementalDecoder . reset ( self );
        self . decoder = None /* Option */;
        pub fn getstate ( self )  {
        state = codecs . BufferedIncrementalDecoder . getstate ( self ) [ 0 ];
        if self . decoder is None /* Option */ {
        return  ( state , 2 );
        addstate = int ( ( sys . byteorder == "big" ) !=;
        ( self . decoder == codecs . utf_32_be_decode ) );
        return  ( state , addstate );
        pub fn setstate ( &self, state )  {
        codecs . BufferedIncrementalDecoder . setstate ( self , state );
        state = state [ 1 ];
        if state == 0 {
        self . decoder = ( codecs . utf_32_be_decode;
        if sys . byteorder == "big" {
        else codecs . utf_32_le_decode );
        } else if state == 1 {
        self . decoder = ( codecs . utf_32_le_decode;
        if sys . byteorder == "big" {
        else codecs . utf_32_be_decode );
        } else {
        self . decoder = None /* Option */;
        class StreamWriter ( codecs . StreamWriter ) ;
        pub fn __init__ ( &self, stream , errors = "strict" )  {
        self . encoder = None /* Option */;
        codecs . StreamWriter . __init__ ( self , stream , errors );
        pub fn reset ( self )  {
        codecs . StreamWriter . reset ( self );
        self . encoder = None /* Option */;
        pub fn encode ( &self, input , errors = "strict" )  {
        if self . encoder is None /* Option */ {
        result = codecs . utf_32_encode ( input , errors );
        if sys . byteorder == "little" {
        self . encoder = codecs . utf_32_le_encode;
        } else {
        self . encoder = codecs . utf_32_be_encode;
        return  result;
        } else {
        return  self . encoder ( input , errors );
        class StreamReader ( codecs . StreamReader ) ;
        pub fn reset ( self )  {
        codecs . StreamReader . reset ( self );
        // try {
        del self . decode;
        // } catch  AttributeError  {
        // pass
        pub fn decode ( &self, input , errors = "strict" )  {
        ( object , consumed , byteorder ) = \;
        codecs . utf_32_ex_decode ( input , errors , 0 , false );
        if byteorder == -1 {
        self . decode = codecs . utf_32_le_decode;
        } else if byteorder == 1 {
        self . decode = codecs . utf_32_be_decode;
        } else if consumed >= 4 {
        panic!("UnicodeError ( "UTF-32 stream does !start with BOM" )");
        return  ( object , consumed );
        pub fn getregentry ( )  {
        return  codecs . CodecInfo (;
        name = "utf-32" ,;
        encode = encode ,;
        decode = decode ,;
        incrementalencoder = IncrementalEncoder ,;
        incrementaldecoder = IncrementalDecoder ,;
        streamreader = StreamReader ,;
        streamwriter = StreamWriter ,;
        );
}

