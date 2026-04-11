//! sunau.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{namedtuple};
// use crate::warnings;
// use crate::builtins;
// use crate::audioop;

pub const remove: f64 = ( 3 , 13 ) );
pub const _sunau_params: &str = namedtuple ("_sunau_params" ,;
pub const AUDIO_FILE_MAGIC: f64 = 0x2e736e64;
pub const AUDIO_FILE_ENCODING_MULAW_8: u64 = 1;
pub const AUDIO_FILE_ENCODING_LINEAR_8: u64 = 2;
pub const AUDIO_FILE_ENCODING_LINEAR_16: u64 = 3;
pub const AUDIO_FILE_ENCODING_LINEAR_24: u64 = 4;
pub const AUDIO_FILE_ENCODING_LINEAR_32: u64 = 5;
pub const AUDIO_FILE_ENCODING_FLOAT: u64 = 6;
pub const AUDIO_FILE_ENCODING_DOUBLE: u64 = 7;
pub const AUDIO_FILE_ENCODING_ADPCM_G721: u64 = 23;
pub const AUDIO_FILE_ENCODING_ADPCM_G722: u64 = 24;
pub const AUDIO_FILE_ENCODING_ADPCM_G723_3: u64 = 25;
pub const AUDIO_FILE_ENCODING_ADPCM_G723_5: u64 = 26;
pub const AUDIO_FILE_ENCODING_ALAW_8: u64 = 27;
pub const AUDIO_UNKNOWN_SIZE: u64 = 0x FFFFFFFF;
pub const _simple_encodings: f64 = [ AUDIO_FILE_ENCODING_MULAW_8 ,;
pub struct Error {
    pub _opened: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _soundpos: String, // TODO: infer type
    pub _hdr_size: String, // TODO: infer type
    pub _data_size: String, // TODO: infer type
    pub _encoding: String, // TODO: infer type
    pub _sampwidth: String, // TODO: infer type
    pub _framesize: String, // TODO: infer type
    pub _framerate: String, // TODO: infer type
    pub _nchannels: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _data_pos: String, // TODO: infer type
    pub _nframes: String, // TODO: infer type
    pub _nframeswritten: String, // TODO: infer type
    pub _datawritten: String, // TODO: infer type
    pub _datalength: String, // TODO: infer type
    pub _comptype: String, // TODO: infer type
    pub _form_length_pos: String, // TODO: infer type
}

impl Error {
}

pub fn _read_u32(file: &str) {
        x = 0;
        for i in range ( 4 ) .iter() {
        byte = file . read ( 1 );
        if !byte {
        panic!("EOFError");
        x = x * 256 + ord ( byte );
        return  x;
        pub fn _write_u32 ( file , x )  {
        data = [ ];
        for i in range ( 4 ) .iter() {
        d , m = divmod ( x , 256 );
        data . insert ( 0 , int ( m ) );
        x = d;
        file . write ( bytes ( data ) );
        class Au_read ;
        pub fn __init__ ( &self, f )  {
        if type ( f ) == type ( "" ) {
        import builtins;
        f = builtins . open ( f , "rb" );
        self . _opened = true;
        } else {
        self . _opened = false;
        self . initfp ( f );
        pub fn __del__ ( self )  {
        if self . _file {
        self . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn initfp ( &self, file )  {
        self . _file = file;
        self . _soundpos = 0;
        magic = int ( _read_u32 ( file ) );
        if magic != AUDIO_FILE_MAGIC {
        panic!("Error ( "bad magic number" )");
        self . _hdr_size = int ( _read_u32 ( file ) );
        if self . _hdr_size < 24 {
        panic!("Error ( "header size too small" )");
        if self . _hdr_size > 100 {
        panic!("Error ( "header size ridiculously large" )");
        self . _data_size = _read_u32 ( file );
        if self . _data_size != AUDIO_UNKNOWN_SIZE {
        self . _data_size = int ( self . _data_size );
        self . _encoding = int ( _read_u32 ( file ) );
        if self . _encoding !in _simple_encodings {
        panic!("Error ( "encoding !(yet) supported" )");
        if self . _encoding in ( AUDIO_FILE_ENCODING_MULAW_8 , {
        AUDIO_FILE_ENCODING_ALAW_8 ) ;
        self . _sampwidth = 2;
        self . _framesize = 1;
        } else if self . _encoding == AUDIO_FILE_ENCODING_LINEAR_8 {
        self . _framesize = self . _sampwidth = 1;
        } else if self . _encoding == AUDIO_FILE_ENCODING_LINEAR_16 {
        self . _framesize = self . _sampwidth = 2;
        } else if self . _encoding == AUDIO_FILE_ENCODING_LINEAR_24 {
        self . _framesize = self . _sampwidth = 3;
        } else if self . _encoding == AUDIO_FILE_ENCODING_LINEAR_32 {
        self . _framesize = self . _sampwidth = 4;
        } else {
        panic!("Error ( "unknown encoding" )");
        self . _framerate = int ( _read_u32 ( file ) );
        self . _nchannels = int ( _read_u32 ( file ) );
        if !self . _nchannels {
        panic!("Error ( "bad # of channels" )");
        self . _framesize = self . _framesize * self . _nchannels;
        if self . _hdr_size > 24 {
        self . _info = file . read ( self . _hdr_size - 24 );
        self . _info , _ , _ = self . _info . partition ( b "\0" );
        } else {
        self . _info = b "";
        // try {
        self . _data_pos = file . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . _data_pos = None /* Option */;
        pub fn getfp ( self )  {
        return  self . _file;
        pub fn getnchannels ( self )  {
        return  self . _nchannels;
        pub fn getsampwidth ( self )  {
        return  self . _sampwidth;
        pub fn getframerate ( self )  {
        return  self . _framerate;
        pub fn getnframes ( self )  {
        if self . _data_size == AUDIO_UNKNOWN_SIZE {
        return  AUDIO_UNKNOWN_SIZE;
        if self . _encoding in _simple_encodings {
        return  self . _data_size / / self . _framesize;
        return  0;
        pub fn getcomptype ( self )  {
        if self . _encoding == AUDIO_FILE_ENCODING_MULAW_8 {
        return  "ULAW";
        } else if self . _encoding == AUDIO_FILE_ENCODING_ALAW_8 {
        return  "ALAW";
        } else {
        return  "NONE";
        pub fn getcompname ( self )  {
        if self . _encoding == AUDIO_FILE_ENCODING_MULAW_8 {
        return  "CCITT G.711 u-law";
        } else if self . _encoding == AUDIO_FILE_ENCODING_ALAW_8 {
        return  "CCITT G.711 A-law";
        } else {
        return  "not compressed";
        pub fn getparams ( self )  {
        return  _sunau_params ( self . getnchannels ( ) , self . getsampwidth ( ) ,;
        self . getframerate ( ) , self . getnframes ( ) ,;
        self . getcomptype ( ) , self . getcompname ( ) );
        pub fn getmarkers ( self )  {
        return;
        pub fn getmark ( &self, id )  {
        panic!("Error ( "no marks" )");
        pub fn readframes ( &self, nframes )  {
        if self . _encoding in _simple_encodings {
        if nframes == AUDIO_UNKNOWN_SIZE {
        data = self . _file . read ( );
        } else {
        data = self . _file . read ( nframes * self . _framesize );
        self . _soundpos + = len ( data ) / / self . _framesize;
        if self . _encoding == AUDIO_FILE_ENCODING_MULAW_8 {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        data = audioop . ulaw2lin ( data , self . _sampwidth );
        return  data;
        return;
        pub fn rewind ( self )  {
        if self . _data_pos is None /* Option */ {
        panic!("OSError ( "cannot seek" )");
        self . _file . seek ( self . _data_pos );
        self . _soundpos = 0;
        pub fn tell ( self )  {
        return  self . _soundpos;
        pub fn setpos ( &self, pos )  {
        if pos < 0 || pos > self . getnframes ( ) {
        panic!("Error ( "position !in range" )");
        if self . _data_pos is None /* Option */ {
        panic!("OSError ( "cannot seek" )");
        self . _file . seek ( self . _data_pos + pos * self . _framesize );
        self . _soundpos = pos;
        pub fn close ( self )  {
        file = self . _file;
        if file {
        self . _file = None /* Option */;
        if self . _opened {
        file . close ( );
        class Au_write ;
        pub fn __init__ ( &self, f )  {
        if type ( f ) == type ( "" ) {
        import builtins;
        f = builtins . open ( f , "wb" );
        self . _opened = true;
        } else {
        self . _opened = false;
        self . initfp ( f );
        pub fn __del__ ( self )  {
        if self . _file {
        self . close ( );
        self . _file = None /* Option */;
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn initfp ( &self, file )  {
        self . _file = file;
        self . _framerate = 0;
        self . _nchannels = 0;
        self . _sampwidth = 0;
        self . _framesize = 0;
        self . _nframes = AUDIO_UNKNOWN_SIZE;
        self . _nframeswritten = 0;
        self . _datawritten = 0;
        self . _datalength = 0;
        self . _info = b "";
        self . _comptype = "ULAW";
        pub fn setnchannels ( &self, nchannels )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if nchannels !in ( 1 , 2 , 4 ) {
        panic!("Error ( "only 1, 2, || 4 channels supported" )");
        self . _nchannels = nchannels;
        pub fn getnchannels ( self )  {
        if !self . _nchannels {
        panic!("Error ( "number of channels !set" )");
        return  self . _nchannels;
        pub fn setsampwidth ( &self, sampwidth )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if sampwidth !in ( 1 , 2 , 3 , 4 ) {
        panic!("Error ( "bad sample width" )");
        self . _sampwidth = sampwidth;
        pub fn getsampwidth ( self )  {
        if !self . _framerate {
        panic!("Error ( "sample width !specified" )");
        return  self . _sampwidth;
        pub fn setframerate ( &self, framerate )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . _framerate = framerate;
        pub fn getframerate ( self )  {
        if !self . _framerate {
        panic!("Error ( "frame rate !set" )");
        return  self . _framerate;
        pub fn setnframes ( &self, nframes )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if nframes < 0 {
        panic!("Error ( "# of frames cannot be negative" )");
        self . _nframes = nframes;
        pub fn getnframes ( self )  {
        return  self . _nframeswritten;
        pub fn setcomptype ( &self, type , name )  {
        if type in ( "NONE" , "ULAW" ) {
        self . _comptype = type;
        } else {
        panic!("Error ( "unknown compression type" )");
        pub fn getcomptype ( self )  {
        return  self . _comptype;
        pub fn getcompname ( self )  {
        if self . _comptype == "ULAW" {
        return  "CCITT G.711 u-law";
        } else if self . _comptype == "ALAW" {
        return  "CCITT G.711 A-law";
        } else {
        return  "not compressed";
        pub fn setparams ( &self, params )  {
        nchannels , sampwidth , framerate , nframes , comptype , compname = params;
        self . setnchannels ( nchannels );
        self . setsampwidth ( sampwidth );
        self . setframerate ( framerate );
        self . setnframes ( nframes );
        self . setcomptype ( comptype , compname );
        pub fn getparams ( self )  {
        return  _sunau_params ( self . getnchannels ( ) , self . getsampwidth ( ) ,;
        self . getframerate ( ) , self . getnframes ( ) ,;
        self . getcomptype ( ) , self . getcompname ( ) );
        pub fn tell ( self )  {
        return  self . _nframeswritten;
        pub fn writeframesraw ( &self, data )  {
        if !isinstance ( data , ( bytes , bytearray ) ) {
        data = memoryview ( data ) . cast ( "B" );
        self . _ensure_header_written ( );
        if self . _comptype == "ULAW" {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        data = audioop . lin2ulaw ( data , self . _sampwidth );
        nframes = len ( data ) / / self . _framesize;
        self . _file . write ( data );
        self . _nframeswritten = self . _nframeswritten + nframes;
        self . _datawritten = self . _datawritten + len ( data );
        pub fn writeframes ( &self, data )  {
        self . writeframesraw ( data );
        if self . _nframeswritten != self . _nframes || \ {
        self . _datalength != self . _datawritten :;
        self . _patchheader ( );
        pub fn close ( self )  {
        if self . _file {
        // try {
        self . _ensure_header_written ( );
        if self . _nframeswritten != self . _nframes || \ {
        self . _datalength != self . _datawritten :;
        self . _patchheader ( );
        self . _file . flush ( );
        // } finally {
        file = self . _file;
        self . _file = None /* Option */;
        if self . _opened {
        file . close ( );
        pub fn _ensure_header_written ( self )  {
        if !self . _nframeswritten {
        if !self . _nchannels {
        panic!("Error ( "# of channels !specified" )");
        if !self . _sampwidth {
        panic!("Error ( "sample width !specified" )");
        if !self . _framerate {
        panic!("Error ( "frame rate !specified" )");
        self . _write_header ( );
        pub fn _write_header ( self )  {
        if self . _comptype == "NONE" {
        if self . _sampwidth == 1 {
        encoding = AUDIO_FILE_ENCODING_LINEAR_8;
        self . _framesize = 1;
        } else if self . _sampwidth == 2 {
        encoding = AUDIO_FILE_ENCODING_LINEAR_16;
        self . _framesize = 2;
        } else if self . _sampwidth == 3 {
        encoding = AUDIO_FILE_ENCODING_LINEAR_24;
        self . _framesize = 3;
        } else if self . _sampwidth == 4 {
        encoding = AUDIO_FILE_ENCODING_LINEAR_32;
        self . _framesize = 4;
        } else {
        panic!("Error ( "internal error" )");
        } else if self . _comptype == "ULAW" {
        encoding = AUDIO_FILE_ENCODING_MULAW_8;
        self . _framesize = 1;
        } else {
        panic!("Error ( "internal error" )");
        self . _framesize = self . _framesize * self . _nchannels;
        _write_u32 ( self . _file , AUDIO_FILE_MAGIC );
        header_size = 25 + len ( self . _info );
        header_size = ( header_size + 7 ) & ~ 7;
        _write_u32 ( self . _file , header_size );
        if self . _nframes == AUDIO_UNKNOWN_SIZE {
        length = AUDIO_UNKNOWN_SIZE;
        } else {
        length = self . _nframes * self . _framesize;
        // try {
        self . _form_length_pos = self . _file . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . _form_length_pos = None /* Option */;
        _write_u32 ( self . _file , length );
        self . _datalength = length;
        _write_u32 ( self . _file , encoding );
        _write_u32 ( self . _file , self . _framerate );
        _write_u32 ( self . _file , self . _nchannels );
        self . _file . write ( self . _info );
        self . _file . write ( b "\0" * ( header_size - len ( self . _info ) - 24 ) );
        pub fn _patchheader ( self )  {
        if self . _form_length_pos is None /* Option */ {
        panic!("OSError ( "cannot seek" )");
        self . _file . seek ( self . _form_length_pos );
        _write_u32 ( self . _file , self . _datawritten );
        self . _datalength = self . _datawritten;
        self . _file . seek ( 0 , 2 );
        pub fn open ( f , mode = None /* Option */ )  {
        if mode is None /* Option */ {
        if hasattr ( f , "mode" ) {
        mode = f . mode;
        } else {
        mode = "rb";
        if mode in ( "r" , "rb" ) {
        return  Au_read ( f );
        } else if mode in ( "w" , "wb" ) {
        return  Au_write ( f );
        } else {
        panic!("Error ( "mode must be 'r', 'rb', 'w', || 'wb'" )");
}

