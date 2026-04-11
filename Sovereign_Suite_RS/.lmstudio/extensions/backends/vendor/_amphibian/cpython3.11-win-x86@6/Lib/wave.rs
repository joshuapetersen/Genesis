//! wave.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{namedtuple};
// use crate::builtins;
// use std::env;

pub const __all__: &str = ["open" ,"Error" ,"Wave_read" ,"Wave_write" ];
pub struct Error {
    pub closed: String, // TODO: infer type
    pub align: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub chunkname: String, // TODO: infer type
    pub chunksize: String, // TODO: infer type
    pub size_read: String, // TODO: infer type
    pub offset: String, // TODO: infer type
    pub seekable: String, // TODO: infer type
    pub _convert: String, // TODO: infer type
    pub _soundpos: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _fmt_chunk_read: String, // TODO: infer type
    pub _data_chunk: String, // TODO: infer type
    pub _data_seek_needed: String, // TODO: infer type
    pub _nframes: String, // TODO: infer type
    pub _i_opened_the_file: String, // TODO: infer type
    pub _sampwidth: String, // TODO: infer type
    pub _framesize: String, // TODO: infer type
    pub _comptype: String, // TODO: infer type
    pub _compname: String, // TODO: infer type
    pub _nchannels: String, // TODO: infer type
    pub _framerate: String, // TODO: infer type
    pub _nframeswritten: String, // TODO: infer type
    pub _datawritten: String, // TODO: infer type
    pub _datalength: String, // TODO: infer type
    pub _headerwritten: String, // TODO: infer type
    pub _form_length_pos: String, // TODO: infer type
    pub _data_length_pos: String, // TODO: infer type
}

impl Error {
}

pub const WAVE_FORMAT_PCM: u64 = 0x0001;
pub const _array_fmts: &str = None ,"b" ,"h" , None ,"i";
pub const _wave_params: &str = namedtuple ("_wave_params" ,;
pub fn _byteswap(data: &str, width: &str) {
        swapped_data = bytearray ( len ( data ) );
        for i in range ( 0 , len ( data ) , width ) .iter() {
        for j in range ( width ) .iter() {
        swapped_data [ i + width - 1 - j ] = data [ i + j ];
        return  bytes ( swapped_data );
        class _Chunk ;
        pub fn __init__ ( &self, file , align = true , bigendian = true , inclheader = false )  {
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
        pub fn getname ( self )  {
        "Return the name (ID) of the current chunk.";
        return  self . chunkname;
        pub fn close ( self )  {
        if !self . closed {
        // try {
        self . skip ( );
        // } finally {
        self . closed = true;
        pub fn seek ( &self, pos , whence = 0 )  {
        "Seek to specified position into the chunk.
        Default position == 0 (start of chunk).
        If the file == !seekable, this will result in an error.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file" )");
        if !self . seekable {
        panic!("OSError ( "cannot seek" )");
        if whence == 1 {
        pos = pos + self . size_read;
        } else if whence == 2 {
        pos = pos + self . chunksize;
        if pos < 0 || pos > self . chunksize {
        panic!("RuntimeError");
        self . file . seek ( self . offset + pos , 0 );
        self . size_read = pos;
        pub fn tell ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file" )");
        return  self . size_read;
        pub fn read ( &self, size = -1 )  {
        "Read at most size bytes from the chunk.
        If size == omitted || negative, read until the end
        of the chunk.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file" )");
        if self . size_read >= self . chunksize {
        return  b "";
        if size < 0 {
        size = self . chunksize - self . size_read;
        if size > self . chunksize - self . size_read {
        size = self . chunksize - self . size_read;
        data = self . file . read ( size );
        self . size_read = self . size_read + len ( data );
        if self . size_read == self . chunksize && \ {
        self . align && \;
        ( self . chunksize & 1 ) ;
        dummy = self . file . read ( 1 );
        self . size_read = self . size_read + len ( dummy );
        return  data;
        pub fn skip ( self )  {
        "Skip the rest of the chunk.
        If you are !interested in the contents of the chunk,
        this method should be called so that the file points to
        the start of the next chunk.
        ";
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file" )");
        if self . seekable {
        // try {
        n = self . chunksize - self . size_read;
        if self . align && ( self . chunksize & 1 ) {
        n = n + 1;
        self . file . seek ( n , 1 );
        self . size_read = self . size_read + n;
        return;
        // } catch  OSError  {
        // pass
        while self . size_read < self . chunksize  {
        n = min ( 8192 , self . chunksize - self . size_read );
        dummy = self . read ( n );
        if !dummy {
        panic!("EOFError");
        class Wave_read ;
        "Variables used in this class:

    These variables are available to the user though appropriate
    methods of this class:
    _file -- the open file with methods read(), close(), && seek()
              set through the __init__() method
    _nchannels -- the number of audio channels
              available through the getnchannels() method
    _nframes -- the number of audio frames
              available through the getnframes() method
    _sampwidth -- the number of bytes per audio sample
              available through the getsampwidth() method
    _framerate -- the sampling frequency
              available through the getframerate() method
    _comptype -- the AIFF-C compression type ('NONE' if AIFF)
              available through the getcomptype() method
    _compname -- the human-readable AIFF-C compression type
              available through the getcomptype() method
    _soundpos -- the position in the audio stream
              available through the tell() method, set through the
              setpos() method

    These variables are used internally only:
    _fmt_chunk_read -- 1 iff the FMT chunk has been read
    _data_seek_needed -- 1 iff positioned correctly in audio
              file for readframes()
    _data_chunk -- instantiation of a chunk class for the DATA chunk
    _framesize -- size of one frame in the file
    ";
        pub fn initfp ( &self, file )  {
        self . _convert = None /* Option */;
        self . _soundpos = 0;
        self . _file = _Chunk ( file , bigendian = 0 );
        if self . _file . getname ( ) != b "RIFF" {
        panic!("Error ( "file does !start with RIFF id" )");
        if self . _file . read ( 4 ) != b "WAVE" {
        panic!("Error ( "not a WAVE file" )");
        self . _fmt_chunk_read = 0;
        self . _data_chunk = None /* Option */;
        while 1  {
        self . _data_seek_needed = 1;
        // try {
        chunk = _Chunk ( self . _file , bigendian = 0 );
        // } catch  EOFError  {
        break;
        chunkname = chunk . getname ( );
        if chunkname == b "fmt " {
        self . _read_fmt_chunk ( chunk );
        self . _fmt_chunk_read = 1;
        } else if chunkname == b "data" {
        if !self . _fmt_chunk_read {
        panic!("Error ( "data chunk before fmt chunk" )");
        self . _data_chunk = chunk;
        self . _nframes = chunk . chunksize / / self . _framesize;
        self . _data_seek_needed = 0;
        break;
        chunk . skip ( );
        if !self . _fmt_chunk_read || !self . _data_chunk {
        panic!("Error ( "fmt chunk and/or data chunk missing" )");
        pub fn __init__ ( &self, f )  {
        self . _i_opened_the_file = None /* Option */;
        if isinstance ( f , str ) {
        f = builtins . open ( f , "rb" );
        self . _i_opened_the_file = f;
        // try {
        self . initfp ( f );
        // } catch   {
        if self . _i_opened_the_file {
        f . close ( );
        panic!("");
        pub fn __del__ ( self )  {
        self . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn getfp ( self )  {
        return  self . _file;
        pub fn rewind ( self )  {
        self . _data_seek_needed = 1;
        self . _soundpos = 0;
        pub fn close ( self )  {
        self . _file = None /* Option */;
        file = self . _i_opened_the_file;
        if file {
        self . _i_opened_the_file = None /* Option */;
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
        return  _wave_params ( self . getnchannels ( ) , self . getsampwidth ( ) ,;
        self . getframerate ( ) , self . getnframes ( ) ,;
        self . getcomptype ( ) , self . getcompname ( ) );
        pub fn getmarkers ( self )  {
        return;
        pub fn getmark ( &self, id )  {
        panic!("Error ( "no marks" )");
        pub fn setpos ( &self, pos )  {
        if pos < 0 || pos > self . _nframes {
        panic!("Error ( "position !in range" )");
        self . _soundpos = pos;
        self . _data_seek_needed = 1;
        pub fn readframes ( &self, nframes )  {
        if self . _data_seek_needed {
        self . _data_chunk . seek ( 0 , 0 );
        pos = self . _soundpos * self . _framesize;
        if pos {
        self . _data_chunk . seek ( pos , 0 );
        self . _data_seek_needed = 0;
        if nframes == 0 {
        return  b "";
        data = self . _data_chunk . read ( nframes * self . _framesize );
        if self . _sampwidth != 1 && sys . byteorder == "big" {
        data = _byteswap ( data , self . _sampwidth );
        if self . _convert && data {
        data = self . _convert ( data );
        self . _soundpos = self . _soundpos + len ( data ) / / ( self . _nchannels * self . _sampwidth );
        return  data;
        pub fn _read_fmt_chunk ( &self, chunk )  {
        // try {
        wFormatTag , self . _nchannels , self . _framerate , dwAvgBytesPerSec , wBlockAlign = struct . unpack_from ( "<HHLLH" , chunk . read ( 14 ) );
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        if wFormatTag == WAVE_FORMAT_PCM {
        // try {
        sampwidth = struct . unpack_from ( "<H" , chunk . read ( 2 ) ) [ 0 ];
        // } catch  struct . error  {
        panic!("EOFError from None /* Option */");
        self . _sampwidth = ( sampwidth + 7 ) / / 8;
        if !self . _sampwidth {
        panic!("Error ( "bad sample width" )");
        } else {
        panic!("Error ( "unknown format: %r" % ( wFormatTag , ) )");
        if !self . _nchannels {
        panic!("Error ( "bad # of channels" )");
        self . _framesize = self . _nchannels * self . _sampwidth;
        self . _comptype = "NONE";
        self . _compname = "not compressed";
        class Wave_write ;
        "Variables used in this class:

    These variables are user settable through appropriate methods
    of this class:
    _file -- the open file with methods write(), close(), tell(), seek()
              set through the __init__() method
    _comptype -- the AIFF-C compression type ('NONE' in AIFF)
              set through the setcomptype() || setparams() method
    _compname -- the human-readable AIFF-C compression type
              set through the setcomptype() || setparams() method
    _nchannels -- the number of audio channels
              set through the setnchannels() || setparams() method
    _sampwidth -- the number of bytes per audio sample
              set through the setsampwidth() || setparams() method
    _framerate -- the sampling frequency
              set through the setframerate() || setparams() method
    _nframes -- the number of audio frames written to the header
              set through the setnframes() || setparams() method

    These variables are used internally only:
    _datalength -- the size of the audio samples written to the header
    _nframeswritten -- the number of frames actually written
    _datawritten -- the size of the audio samples actually written
    ";
        pub fn __init__ ( &self, f )  {
        self . _i_opened_the_file = None /* Option */;
        if isinstance ( f , str ) {
        f = builtins . open ( f , "wb" );
        self . _i_opened_the_file = f;
        // try {
        self . initfp ( f );
        // } catch   {
        if self . _i_opened_the_file {
        f . close ( );
        panic!("");
        pub fn initfp ( &self, file )  {
        self . _file = file;
        self . _convert = None /* Option */;
        self . _nchannels = 0;
        self . _sampwidth = 0;
        self . _framerate = 0;
        self . _nframes = 0;
        self . _nframeswritten = 0;
        self . _datawritten = 0;
        self . _datalength = 0;
        self . _headerwritten = false;
        pub fn __del__ ( self )  {
        self . close ( );
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . close ( );
        pub fn setnchannels ( &self, nchannels )  {
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if nchannels < 1 {
        panic!("Error ( "bad # of channels" )");
        self . _nchannels = nchannels;
        pub fn getnchannels ( self )  {
        if !self . _nchannels {
        panic!("Error ( "number of channels !set" )");
        return  self . _nchannels;
        pub fn setsampwidth ( &self, sampwidth )  {
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if sampwidth < 1 || sampwidth > 4 {
        panic!("Error ( "bad sample width" )");
        self . _sampwidth = sampwidth;
        pub fn getsampwidth ( self )  {
        if !self . _sampwidth {
        panic!("Error ( "sample width !set" )");
        return  self . _sampwidth;
        pub fn setframerate ( &self, framerate )  {
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if framerate <= 0 {
        panic!("Error ( "bad frame rate" )");
        self . _framerate = int ( round ( framerate ) );
        pub fn getframerate ( self )  {
        if !self . _framerate {
        panic!("Error ( "frame rate !set" )");
        return  self . _framerate;
        pub fn setnframes ( &self, nframes )  {
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . _nframes = nframes;
        pub fn getnframes ( self )  {
        return  self . _nframeswritten;
        pub fn setcomptype ( &self, comptype , compname )  {
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        if comptype !in ( "NONE" , ) {
        panic!("Error ( "unsupported compression type" )");
        self . _comptype = comptype;
        self . _compname = compname;
        pub fn getcomptype ( self )  {
        return  self . _comptype;
        pub fn getcompname ( self )  {
        return  self . _compname;
        pub fn setparams ( &self, params )  {
        nchannels , sampwidth , framerate , nframes , comptype , compname = params;
        if self . _datawritten {
        panic!("Error ( "cannot change parameters after starting to write" )");
        self . setnchannels ( nchannels );
        self . setsampwidth ( sampwidth );
        self . setframerate ( framerate );
        self . setnframes ( nframes );
        self . setcomptype ( comptype , compname );
        pub fn getparams ( self )  {
        if !self . _nchannels || !self . _sampwidth || !self . _framerate {
        panic!("Error ( "not all parameters set" )");
        return  _wave_params ( self . _nchannels , self . _sampwidth , self . _framerate ,;
        self . _nframes , self . _comptype , self . _compname );
        pub fn setmark ( &self, id , pos , name )  {
        panic!("Error ( "setmark() !supported" )");
        pub fn getmark ( &self, id )  {
        panic!("Error ( "no marks" )");
        pub fn getmarkers ( self )  {
        return;
        pub fn tell ( self )  {
        return  self . _nframeswritten;
        pub fn writeframesraw ( &self, data )  {
        if !isinstance ( data , ( bytes , bytearray ) ) {
        data = memoryview ( data ) . cast ( "B" );
        self . _ensure_header_written ( len ( data ) );
        nframes = len ( data ) / / ( self . _sampwidth * self . _nchannels );
        if self . _convert {
        data = self . _convert ( data );
        if self . _sampwidth != 1 && sys . byteorder == "big" {
        data = _byteswap ( data , self . _sampwidth );
        self . _file . write ( data );
        self . _datawritten + = len ( data );
        self . _nframeswritten = self . _nframeswritten + nframes;
        pub fn writeframes ( &self, data )  {
        self . writeframesraw ( data );
        if self . _datalength != self . _datawritten {
        self . _patchheader ( );
        pub fn close ( self )  {
        // try {
        if self . _file {
        self . _ensure_header_written ( 0 );
        if self . _datalength != self . _datawritten {
        self . _patchheader ( );
        self . _file . flush ( );
        // } finally {
        self . _file = None /* Option */;
        file = self . _i_opened_the_file;
        if file {
        self . _i_opened_the_file = None /* Option */;
        file . close ( );
        pub fn _ensure_header_written ( &self, datasize )  {
        if !self . _headerwritten {
        if !self . _nchannels {
        panic!("Error ( "# channels !specified" )");
        if !self . _sampwidth {
        panic!("Error ( "sample width !specified" )");
        if !self . _framerate {
        panic!("Error ( "sampling rate !specified" )");
        self . _write_header ( datasize );
        pub fn _write_header ( &self, initlength )  {
        assert !self . _headerwritten;
        self . _file . write ( b "RIFF" );
        if !self . _nframes {
        self . _nframes = initlength / / ( self . _nchannels * self . _sampwidth );
        self . _datalength = self . _nframes * self . _nchannels * self . _sampwidth;
        // try {
        self . _form_length_pos = self . _file . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . _form_length_pos = None /* Option */;
        self . _file . write ( struct . pack ( "<L4s4sLHHLLHH4s" ,;
        36 + self . _datalength , b "WAVE" , b "fmt " , 16 ,;
        WAVE_FORMAT_PCM , self . _nchannels , self . _framerate ,;
        self . _nchannels * self . _framerate * self . _sampwidth ,;
        self . _nchannels * self . _sampwidth ,;
        self . _sampwidth * 8 , b "data" ) );
        if self . _form_length_pos is !None /* Option */ {
        self . _data_length_pos = self . _file . tell ( );
        self . _file . write ( struct . pack ( "<L" , self . _datalength ) );
        self . _headerwritten = true;
        pub fn _patchheader ( self )  {
        assert self . _headerwritten;
        if self . _datawritten == self . _datalength {
        return;
        curpos = self . _file . tell ( );
        self . _file . seek ( self . _form_length_pos , 0 );
        self . _file . write ( struct . pack ( "<L" , 36 + self . _datawritten ) );
        self . _file . seek ( self . _data_length_pos , 0 );
        self . _file . write ( struct . pack ( "<L" , self . _datawritten ) );
        self . _file . seek ( curpos , 0 );
        self . _datalength = self . _datawritten;
        pub fn open ( f , mode = None /* Option */ )  {
        if mode is None /* Option */ {
        if hasattr ( f , "mode" ) {
        mode = f . mode;
        } else {
        mode = "rb";
        if mode in ( "r" , "rb" ) {
        return  Wave_read ( f );
        } else if mode in ( "w" , "wb" ) {
        return  Wave_write ( f );
        } else {
        panic!("Error ( "mode must be 'r', 'rb', 'w', || 'wb'" )");
}

