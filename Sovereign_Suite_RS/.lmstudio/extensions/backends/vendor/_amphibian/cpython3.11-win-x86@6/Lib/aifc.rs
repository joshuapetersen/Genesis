//! aifc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::struct;
// use crate::warnings;
// use std::f64::consts;
// use crate::chunk::{Chunk};
// use std::collections::{namedtuple};
// use crate::audioop;
// use std::env;

pub const __all__: &str = ["Error" ,"open" ];
pub const remove: f64 = ( 3 , 13 ) );
pub struct Error {
    pub _version: String, // TODO: infer type
    pub _convert: String, // TODO: infer type
    pub _markers: String, // TODO: infer type
    pub _soundpos: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _aifc: String, // TODO: infer type
    pub _comm_chunk_read: String, // TODO: infer type
    pub _ssnd_chunk: String, // TODO: infer type
    pub _ssnd_seek_needed: String, // TODO: infer type
    pub _adpcmstate: String, // TODO: infer type
    pub _nchannels: String, // TODO: infer type
    pub _nframes: String, // TODO: infer type
    pub _sampwidth: String, // TODO: infer type
    pub _framerate: String, // TODO: infer type
    pub _framesize: String, // TODO: infer type
    pub _comptype: String, // TODO: infer type
    pub _compname: String, // TODO: infer type
    pub _nframeswritten: String, // TODO: infer type
    pub _datawritten: String, // TODO: infer type
    pub _datalength: String, // TODO: infer type
    pub _marklength: String, // TODO: infer type
    pub _form_length_pos: String, // TODO: infer type
    pub _nframes_pos: String, // TODO: infer type
    pub _ssnd_length_pos: String, // TODO: infer type
}

impl Error {
}

pub const _AIFC_version: u64 = 0x A2805140;
pub fn _read_long(file: &str) {
        // try {
        return  struct . unpack ( ">l" , file . read ( 4 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        pub fn _read_ulong ( file )  {
        // try {
        return  struct . unpack ( ">L" , file . read ( 4 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        pub fn _read_short ( file )  {
        // try {
        return  struct . unpack ( ">h" , file . read ( 2 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        pub fn _read_ushort ( file )  {
        // try {
        return  struct . unpack ( ">H" , file . read ( 2 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        pub fn _read_string ( file )  {
        length = ord ( file . read ( 1 ) );
        if length == 0 {
        data = b "";
        } else {
        data = file . read ( length );
        if length & 1 == 0 {
        dummy = file . read ( 1 );
        return  data;
        _HUGE_VAL = 1.79769313486231e + 308;
        pub fn _read_float ( f )  {
        expon = _read_short ( f );
        sign = 1;
        if expon < 0 {
        sign = -1;
        expon = expon + 0x8000;
        himant = _read_ulong ( f );
        lomant = _read_ulong ( f );
        if expon == himant == lomant == 0 {
        f = 0.0;
        } else if expon == 0x7 FFF {
        f = _HUGE_VAL;
        } else {
        expon = expon - 16383;
        f = ( himant * 0x100000000 + lomant ) * pow ( 2.0 , expon - 63 );
        return  sign * f;
        pub fn _write_short ( f , x )  {
        f . write ( struct . pack ( ">h" , x ) );
        pub fn _write_ushort ( f , x )  {
        f . write ( struct . pack ( ">H" , x ) );
        pub fn _write_long ( f , x )  {
        f . write ( struct . pack ( ">l" , x ) );
        pub fn _write_ulong ( f , x )  {
        f . write ( struct . pack ( ">L" , x ) );
        pub fn _write_string ( f , s )  {
        if len ( s ) > 255 {
        panic!("ValueError ( "string exceeds maximum pstring length" )");
        f . write ( struct . pack ( "B" , len ( s ) ) );
        f . write ( s );
        if len ( s ) & 1 == 0 {
        f . write ( b "\x00" );
        pub fn _write_float ( f , x )  {
        import math;
        if x < 0 {
        sign = 0x8000;
        x = x * -1;
        } else {
        sign = 0;
        if x == 0 {
        expon = 0;
        himant = 0;
        lomant = 0;
        } else {
        fmant , expon = math . frexp ( x );
        if expon > 16384 || fmant >= 1 || fmant != fmant {
        expon = sign | 0x7 FFF;
        himant = 0;
        lomant = 0;
        } else {
        expon = expon + 16382;
        if expon < 0 {
        fmant = math . ldexp ( fmant , expon );
        expon = 0;
        expon = expon | sign;
        fmant = math . ldexp ( fmant , 32 );
        fsmant = math . floor ( fmant );
        himant = int ( fsmant );
        fmant = math . ldexp ( fmant - fsmant , 32 );
        fsmant = math . floor ( fmant );
        lomant = int ( fsmant );
        _write_ushort ( f , expon );
        _write_ulong ( f , himant );
        _write_ulong ( f , lomant );
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , DeprecationWarning );
        from chunk import Chunk;
        from collections import namedtuple;
        _aifc_params = namedtuple ( "_aifc_params" ,;
        "nchannels sampwidth framerate nframes comptype compname" );
        _aifc_params . nchannels . __doc__ = "Number of audio channels (1 for mono, 2 for stereo)";
        _aifc_params . sampwidth . __doc__ = "Sample width in bytes";
        _aifc_params . framerate . __doc__ = "Sampling frequency";
        _aifc_params . nframes . __doc__ = "Number of audio frames";
        _aifc_params . comptype . __doc__ = "Compression type ("NONE" for AIFF files)";
        _aifc_params . compname . __doc__ = ( "\
A human-readable version of the compression type
('not compressed' for AIFF files)" );
        class Aifc_read ;
        _file = None /* Option */;
        pub fn initfp ( &self, file )  {
        self . _version = 0;
        self . _convert = None /* Option */;
        self . _markers = [ ];
        self . _soundpos = 0;
        self . _file = file;
        chunk = Chunk ( file );
        if chunk . getname ( ) != b "FORM" {
        panic!("Error ( "file does !start with FORM id" )");
        formdata = chunk . read ( 4 );
        if formdata == b "AIFF" {
        self . _aifc = 0;
        } else if formdata == b "AIFC" {
        self . _aifc = 1;
        } else {
        panic!("Error ( "not an AIFF || AIFF-C file" )");
        self . _comm_chunk_read = 0;
        self . _ssnd_chunk = None /* Option */;
        while 1  {
        self . _ssnd_seek_needed = 1;
        // try {
        chunk = Chunk ( self . _file );
        // } catch  EOFError  {
        break;
        chunkname = chunk . getname ( );
        if chunkname == b "COMM" {
        self . _read_comm_chunk ( chunk );
        self . _comm_chunk_read = 1;
        } else if chunkname == b "SSND" {
        self . _ssnd_chunk = chunk;
        dummy = chunk . read ( 8 );
        self . _ssnd_seek_needed = 0;
        } else if chunkname == b "FVER" {
        self . _version = _read_ulong ( chunk );
        } else if chunkname == b "MARK" {
        self . _readmark ( chunk );
        chunk . skip ( );
        if !self . _comm_chunk_read || !self . _ssnd_chunk {
        panic!("Error ( "COMM chunk and/or SSND chunk missing" )");
        pub fn __init__ ( &self, f )  {
        if isinstance ( f , str ) {
        file_object = builtins . open ( f , "rb" );
        // try {
        self . initfp ( file_object );
        // } catch   {
        file_object . close ( );
        panic!("");
        } else {
        self . initfp ( f );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn getfp ( self )  {
        return  self . _file;
        pub fn rewind ( self )  {
        self . _ssnd_seek_needed = 1;
        self . _soundpos = 0;
        pub fn close ( self )  {
        file = self . _file;
        if file is !None /* Option */ {
        self . _file = None /* Option */;
        file . close ( );
        pub fn tell ( self )  {
        return  self . _soundpos;
        pub fn getnchannels ( self )  {
        return  self . _nchannels;
        pub fn getnframes ( self )  {
        return  self . _nframes;
        pub fn getsampwidth ( self )  {
        return  self . _sampwidth;
        pub fn getframerate ( self )  {
        return  self . _framerate;
        pub fn getcomptype ( self )  {
        return  self . _comptype;
        pub fn getcompname ( self )  {
        return  self . _compname;
        pub fn getparams ( self )  {
        return  _aifc_params ( self . getnchannels ( ) , self . getsampwidth ( ) ,;
        self . getframerate ( ) , self . getnframes ( ) ,;
        self . getcomptype ( ) , self . getcompname ( ) );
        pub fn getmarkers ( self )  {
        if len ( self . _markers ) == 0 {
        return;
        return  self . _markers;
        pub fn getmark ( &self, id )  {
        for marker in self . _markers .iter() {
        if id == marker [ 0 ] {
        return  marker;
        panic!("Error ( "marker {0!r} does !exist" . format ( id ) )");
        pub fn setpos ( &self, pos )  {
        if pos < 0 || pos > self . _nframes {
        panic!("Error ( "position !in range" )");
        self . _soundpos = pos;
        self . _ssnd_seek_needed = 1;
        pub fn readframes ( &self, nframes )  {
        if self . _ssnd_seek_needed {
        self . _ssnd_chunk . seek ( 0 );
        dummy = self . _ssnd_chunk . read ( 8 );
        pos = self . _soundpos * self . _framesize;
        if pos {
        self . _ssnd_chunk . seek ( pos + 8 );
        self . _ssnd_seek_needed = 0;
        if nframes == 0 {
        return  b "";
        data = self . _ssnd_chunk . read ( nframes * self . _framesize );
        if self . _convert && data {
        data = self . _convert ( data );
        self . _soundpos = self . _soundpos + len ( data ) / / ( self . _nchannels;
        * self . _sampwidth );
        return  data;
        pub fn _alaw2lin ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . alaw2lin ( data , 2 );
        pub fn _ulaw2lin ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . ulaw2lin ( data , 2 );
        pub fn _adpcm2lin ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        if !hasattr ( self , "_adpcmstate" ) {
        self . _adpcmstate = None /* Option */;
        data , self . _adpcmstate = audioop . adpcm2lin ( data , 2 , self . _adpcmstate );
        return  data;
        pub fn _sowt2lin ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . byteswap ( data , 2 );
        pub fn _read_comm_chunk ( &self, chunk )  {
        self . _nchannels = _read_short ( chunk );
        self . _nframes = _read_long ( chunk );
        self . _sampwidth = ( _read_short ( chunk ) + 7 ) / / 8;
        self . _framerate = int ( _read_float ( chunk ) );
        if self . _sampwidth <= 0 {
        panic!("Error ( "bad sample width" )");
        if self . _nchannels <= 0 {
        panic!("Error ( "bad # of channels" )");
        self . _framesize = self . _nchannels * self . _sampwidth;
        if self . _aifc {
        kludge = 0;
        if chunk . chunksize == 18 {
        kludge = 1;
        warnings . warn ( "Warning: bad COMM chunk size" );
        chunk . chunksize = 23;
        self . _comptype = chunk . read ( 4 );
        if kludge {
        length = ord ( chunk . file . read ( 1 ) );
        if length & 1 == 0 {
        length = length + 1;
        chunk . chunksize = chunk . chunksize + length;
        chunk . file . seek ( -1 , 1 );
        self . _compname = _read_string ( chunk );
        if self . _comptype != b "NONE" {
        if self . _comptype == b "G722" {
        self . _convert = self . _adpcm2lin;
        } else if self . _comptype in ( b "ulaw" , b "ULAW" ) {
        self . _convert = self . _ulaw2lin;
        } else if self . _comptype in ( b "alaw" , b "ALAW" ) {
        self . _convert = self . _alaw2lin;
        } else if self . _comptype in ( b "sowt" , b "SOWT" ) {
        self . _convert = self . _sowt2lin;
        } else {
        panic!("Error ( "unsupported compression type" )");
        self . _sampwidth = 2;
        } else {
        self . _comptype = b "NONE";
        self . _compname = b "not compressed";
        pub fn _readmark ( &self, chunk )  {
        nmarkers = _read_short ( chunk );
        // try {
        for i in range ( nmarkers ) .iter() {
        id = _read_short ( chunk );
        pos = _read_long ( chunk );
        name = _read_string ( chunk );
        if pos || name {
        self . _markers . append ( ( id , pos , name ) );
        // } catch  EOFError  {
        w = ( "Warning: MARK chunk contains only %s marker%s instead of %s" %;
        ( len ( self . _markers ) , "" if len ( self . _markers ) == 1 else "s" ,;
        nmarkers ) );
        warnings . warn ( w );
        class Aifc_write ;
        _file = None /* Option */;
        pub fn __init__ ( &self, f )  {
        if isinstance ( f , str ) {
        file_object = builtins . open ( f , "wb" );
        // try {
        self . initfp ( file_object );
        // } catch   {
        file_object . close ( );
        panic!("");
        if f . endswith ( ".aiff" ) {
        self . _aifc = 0;
        } else {
        self . initfp ( f );
        pub fn initfp ( &self, file )  {
        self . _file = file;
        self . _version = _AIFC_version;
        self . _comptype = b "NONE";
        self . _compname = b "not compressed";
        self . _convert = None /* Option */;
        self . _nchannels = 0;
        self . _sampwidth = 0;
        self . _framerate = 0;
        self . _nframes = 0;
        self . _nframeswritten = 0;
        self . _datawritten = 0;
        self . _datalength = 0;
        self . _markers = [ ];
        self . _marklength = 0;
        self . _aifc = 1;
        pub fn __del__ ( self )  {
        self . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn aiff ( self )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . _aifc = 0;
        pub fn aifc ( self )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . _aifc = 1;
        pub fn setnchannels ( &self, nchannels )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if nchannels < 1 {
        panic!("Error ( "bad # of channels" )");
        self . _nchannels = nchannels;
        pub fn getnchannels ( self )  {
        if !self . _nchannels {
        panic!("Error ( "number of channels !set" )");
        return  self . _nchannels;
        pub fn setsampwidth ( &self, sampwidth )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if sampwidth < 1 || sampwidth > 4 {
        panic!("Error ( "bad sample width" )");
        self . _sampwidth = sampwidth;
        pub fn getsampwidth ( self )  {
        if !self . _sampwidth {
        panic!("Error ( "sample width !set" )");
        return  self . _sampwidth;
        pub fn setframerate ( &self, framerate )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if framerate <= 0 {
        panic!("Error ( "bad frame rate" )");
        self . _framerate = framerate;
        pub fn getframerate ( self )  {
        if !self . _framerate {
        panic!("Error ( "frame rate !set" )");
        return  self . _framerate;
        pub fn setnframes ( &self, nframes )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . _nframes = nframes;
        pub fn getnframes ( self )  {
        return  self . _nframeswritten;
        pub fn setcomptype ( &self, comptype , compname )  {
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if comptype !in ( b "NONE" , b "ulaw" , b "ULAW" , {
        b "alaw" , b "ALAW" , b "G722" , b "sowt" , b "SOWT" ) ;
        panic!("Error ( "unsupported compression type" )");
        self . _comptype = comptype;
        self . _compname = compname;
        pub fn getcomptype ( self )  {
        return  self . _comptype;
        pub fn getcompname ( self )  {
        return  self . _compname;
        pub fn setparams ( &self, params )  {
        nchannels , sampwidth , framerate , nframes , comptype , compname = params;
        if self . _nframeswritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if comptype !in ( b "NONE" , b "ulaw" , b "ULAW" , {
        b "alaw" , b "ALAW" , b "G722" , b "sowt" , b "SOWT" ) ;
        panic!("Error ( "unsupported compression type" )");
        self . setnchannels ( nchannels );
        self . setsampwidth ( sampwidth );
        self . setframerate ( framerate );
        self . setnframes ( nframes );
        self . setcomptype ( comptype , compname );
        pub fn getparams ( self )  {
        if !self . _nchannels || !self . _sampwidth || !self . _framerate {
        panic!("Error ( "not all parameters set" )");
        return  _aifc_params ( self . _nchannels , self . _sampwidth , self . _framerate ,;
        self . _nframes , self . _comptype , self . _compname );
        pub fn setmark ( &self, id , pos , name )  {
        if id <= 0 {
        panic!("Error ( "marker ID must be > 0" )");
        if pos < 0 {
        panic!("Error ( "marker position must be >= 0" )");
        if !isinstance ( name , bytes ) {
        panic!("Error ( "marker name must be bytes" )");
        for i in range ( len ( self . _markers ) ) .iter() {
        if id == self . _markers [ i ] [ 0 ] {
        self . _markers [ i ] = id , pos , name;
        return;
        self . _markers . append ( ( id , pos , name ) );
        pub fn getmark ( &self, id )  {
        for marker in self . _markers .iter() {
        if id == marker [ 0 ] {
        return  marker;
        panic!("Error ( "marker {0!r} does !exist" . format ( id ) )");
        pub fn getmarkers ( self )  {
        if len ( self . _markers ) == 0 {
        return;
        return  self . _markers;
        pub fn tell ( self )  {
        return  self . _nframeswritten;
        pub fn writeframesraw ( &self, data )  {
        if !isinstance ( data , ( bytes , bytearray ) ) {
        data = memoryview ( data ) . cast ( "B" );
        self . _ensure_header_written ( len ( data ) );
        nframes = len ( data ) / / ( self . _sampwidth * self . _nchannels );
        if self . _convert {
        data = self . _convert ( data );
        self . _file . write ( data );
        self . _nframeswritten = self . _nframeswritten + nframes;
        self . _datawritten = self . _datawritten + len ( data );
        pub fn writeframes ( &self, data )  {
        self . writeframesraw ( data );
        if self . _nframeswritten != self . _nframes || \ {
        self . _datalength != self . _datawritten :;
        self . _patchheader ( );
        pub fn close ( self )  {
        if self . _file is None /* Option */ {
        return;
        // try {
        self . _ensure_header_written ( 0 );
        if self . _datawritten & 1 {
        self . _file . write ( b "\x00" );
        self . _datawritten = self . _datawritten + 1;
        self . _writemarkers ( );
        if self . _nframeswritten != self . _nframes || \ {
        self . _datalength != self . _datawritten || \;
        self . _marklength :;
        self . _patchheader ( );
        // } finally {
        self . _convert = None /* Option */;
        f = self . _file;
        self . _file = None /* Option */;
        f . close ( );
        pub fn _lin2alaw ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . lin2alaw ( data , 2 );
        pub fn _lin2ulaw ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . lin2ulaw ( data , 2 );
        pub fn _lin2adpcm ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        if !hasattr ( self , "_adpcmstate" ) {
        self . _adpcmstate = None /* Option */;
        data , self . _adpcmstate = audioop . lin2adpcm ( data , 2 , self . _adpcmstate );
        return  data;
        pub fn _lin2sowt ( &self, data )  {
        // with scope: warnings . catch_warnings ( )  {
        warnings . simplefilter ( "ignore" , category = DeprecationWarning );
        import audioop;
        return  audioop . byteswap ( data , 2 );
        pub fn _ensure_header_written ( &self, datasize )  {
        if !self . _nframeswritten {
        if self . _comptype in ( b "ULAW" , b "ulaw" , {
        b "ALAW" , b "alaw" , b "G722" ,;
        b "sowt" , b "SOWT" ) ;
        if !self . _sampwidth {
        self . _sampwidth = 2;
        if self . _sampwidth != 2 {
        panic!("Error ( "sample width must be 2 when compressing "");
        "with ulaw/ULAW, alaw/ALAW, sowt/SOWT ";
        "or G7.22 (ADPCM)" );
        if !self . _nchannels {
        panic!("Error ( "# channels !specified" )");
        if !self . _sampwidth {
        panic!("Error ( "sample width !specified" )");
        if !self . _framerate {
        panic!("Error ( "sampling rate !specified" )");
        self . _write_header ( datasize );
        pub fn _init_compression ( self )  {
        if self . _comptype == b "G722" {
        self . _convert = self . _lin2adpcm;
        } else if self . _comptype in ( b "ulaw" , b "ULAW" ) {
        self . _convert = self . _lin2ulaw;
        } else if self . _comptype in ( b "alaw" , b "ALAW" ) {
        self . _convert = self . _lin2alaw;
        } else if self . _comptype in ( b "sowt" , b "SOWT" ) {
        self . _convert = self . _lin2sowt;
        pub fn _write_header ( &self, initlength )  {
        if self . _aifc && self . _comptype != b "NONE" {
        self . _init_compression ( );
        self . _file . write ( b "FORM" );
        if !self . _nframes {
        self . _nframes = initlength / / ( self . _nchannels * self . _sampwidth );
        self . _datalength = self . _nframes * self . _nchannels * self . _sampwidth;
        if self . _datalength & 1 {
        self . _datalength = self . _datalength + 1;
        if self . _aifc {
        if self . _comptype in ( b "ulaw" , b "ULAW" , b "alaw" , b "ALAW" ) {
        self . _datalength = self . _datalength / / 2;
        if self . _datalength & 1 {
        self . _datalength = self . _datalength + 1;
        } else if self . _comptype == b "G722" {
        self . _datalength = ( self . _datalength + 3 ) / / 4;
        if self . _datalength & 1 {
        self . _datalength = self . _datalength + 1;
        // try {
        self . _form_length_pos = self . _file . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . _form_length_pos = None /* Option */;
        commlength = self . _write_form_length ( self . _datalength );
        if self . _aifc {
        self . _file . write ( b "AIFC" );
        self . _file . write ( b "FVER" );
        _write_ulong ( self . _file , 4 );
        _write_ulong ( self . _file , self . _version );
        } else {
        self . _file . write ( b "AIFF" );
        self . _file . write ( b "COMM" );
        _write_ulong ( self . _file , commlength );
        _write_short ( self . _file , self . _nchannels );
        if self . _form_length_pos is !None /* Option */ {
        self . _nframes_pos = self . _file . tell ( );
        _write_ulong ( self . _file , self . _nframes );
        if self . _comptype in ( b "ULAW" , b "ulaw" , b "ALAW" , b "alaw" , b "G722" ) {
        _write_short ( self . _file , 8 );
        } else {
        _write_short ( self . _file , self . _sampwidth * 8 );
        _write_float ( self . _file , self . _framerate );
        if self . _aifc {
        self . _file . write ( self . _comptype );
        _write_string ( self . _file , self . _compname );
        self . _file . write ( b "SSND" );
        if self . _form_length_pos is !None /* Option */ {
        self . _ssnd_length_pos = self . _file . tell ( );
        _write_ulong ( self . _file , self . _datalength + 8 );
        _write_ulong ( self . _file , 0 );
        _write_ulong ( self . _file , 0 );
        pub fn _write_form_length ( &self, datalength )  {
        if self . _aifc {
        commlength = 18 + 5 + len ( self . _compname );
        if commlength & 1 {
        commlength = commlength + 1;
        verslength = 12;
        } else {
        commlength = 18;
        verslength = 0;
        _write_ulong ( self . _file , 4 + verslength + self . _marklength + \;
        8 + commlength + 16 + datalength );
        return  commlength;
        pub fn _patchheader ( self )  {
        curpos = self . _file . tell ( );
        if self . _datawritten & 1 {
        datalength = self . _datawritten + 1;
        self . _file . write ( b "\x00" );
        } else {
        datalength = self . _datawritten;
        if datalength == self . _datalength && \ {
        self . _nframes == self . _nframeswritten && \;
        self . _marklength == 0 :;
        self . _file . seek ( curpos , 0 );
        return;
        self . _file . seek ( self . _form_length_pos , 0 );
        dummy = self . _write_form_length ( datalength );
        self . _file . seek ( self . _nframes_pos , 0 );
        _write_ulong ( self . _file , self . _nframeswritten );
        self . _file . seek ( self . _ssnd_length_pos , 0 );
        _write_ulong ( self . _file , datalength + 8 );
        self . _file . seek ( curpos , 0 );
        self . _nframes = self . _nframeswritten;
        self . _datalength = datalength;
        pub fn _writemarkers ( self )  {
        if len ( self . _markers ) == 0 {
        return;
        self . _file . write ( b "MARK" );
        length = 2;
        for marker in self . _markers .iter() {
        id , pos , name = marker;
        length = length + len ( name ) + 1 + 6;
        if len ( name ) & 1 == 0 {
        length = length + 1;
        _write_ulong ( self . _file , length );
        self . _marklength = length + 8;
        _write_short ( self . _file , len ( self . _markers ) );
        for marker in self . _markers .iter() {
        id , pos , name = marker;
        _write_short ( self . _file , id );
        _write_ulong ( self . _file , pos );
        _write_string ( self . _file , name );
        pub fn open ( f , mode = None /* Option */ )  {
        if mode is None /* Option */ {
        if hasattr ( f , "mode" ) {
        mode = f . mode;
        } else {
        mode = "rb";
        if mode in ( "r" , "rb" ) {
        return  Aifc_read ( f );
        } else if mode in ( "w" , "wb" ) {
        return  Aifc_write ( f );
        } else {
        panic!("Error ( "mode must be 'r', 'rb', 'w', || 'wb'" )");
        fn main() {
        import sys;
        if !sys . argv [ 1 { : ] ; }
        sys . argv . append ( "/usr/demos/data/audio/bach.aifformat!(" ));
        fn = sys . argv [ 1 ];
        // with scope: open ( fn , "r" ) as f  {
        println!( "Reading" , fn );
        println!( "nchannels =" , f . getnchannels ( ) );
        println!( "nframes   =" , f . getnframes ( ) );
        println!( "sampwidth =" , f . getsampwidth ( ) );
        println!( "framerate =" , f . getframerate ( ) );
        println!( "comptype  =" , f . getcomptype ( ) );
        println!( "compname  =" , f . getcompname ( ) );
        if sys . argv [ 2 { : ] ; }
        gn = sys . argv [ 2 ];
        println!( "Writing" , gn );
        // with scope: open ( gn , "w" ) as g  {
        g . setparams ( f . getparams ( ) );
        while 1  {
        data = f . readframes ( 1024 );
        if !data {
        break;
        g . writeframes ( data );
        println!( "Done." );
}

