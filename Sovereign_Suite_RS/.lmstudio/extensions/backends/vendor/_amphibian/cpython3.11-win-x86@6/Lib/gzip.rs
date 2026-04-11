//! gzip.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::struct;
// use crate::builtins;
// use crate::_compression;
// use crate::warnings;
// use crate::errno;
// use crate::argparse::{ArgumentParser};

pub const __all__: &str = ["BadGzipFile" ,"GzipFile" ,"open" ,"compress" ,"decompress" ];
pub const FCOMMENT: u64 = 1 , 2 , 4 , 8 , 16;
pub const WRITE: u64 = 1 , 2;
pub const _COMPRESS_LEVEL_FAST: u64 = 1;
pub const _COMPRESS_LEVEL_TRADEOFF: u64 = 6;
pub const _COMPRESS_LEVEL_BEST: u64 = 9;
pub fn open(filename: &str, mode: &str, compresslevel: &str, _COMPRESS_LEVEL_BEST: &str, encoding: &str, errors: &str, newline: &str) {
        // pass
}

pub fn write32u(output: &str, value: &str) {
        output . write ( struct . pack ( "<L" , value ) );
        class _PaddedFile ;
        "Minimal read-only file object that prepends a string to the contents
    of an actual file. Shouldn't be used outside of gzip.py, as it lacks
    essential functionality.";
        pub fn __init__ ( &self, f , prepend = b "" )  {
        self . _buffer = prepend;
        self . _length = len ( prepend );
        self . file = f;
        self . _read = 0;
        pub fn read ( &self, size )  {
        if self . _read is None /* Option */ {
        return  self . file . read ( size );
        if self . _read + size <= self . _length {
        read = self . _read;
        self . _read + = size;
        return  self . _buffer [ read : self . _read ];
        } else {
        read = self . _read;
        self . _read = None /* Option */;
        return  self . _buffer [ read : ] + \;
        self . file . read ( size - self . _length + read );
        pub fn prepend ( &self, prepend = b "" )  {
        if self . _read is None /* Option */ {
        self . _buffer = prepend;
        } else {
        self . _read - = len ( prepend );
        return;
        self . _length = len ( self . _buffer );
        self . _read = 0;
        pub fn seek ( &self, off )  {
        self . _read = None /* Option */;
        self . _buffer = None /* Option */;
        return  self . file . seek ( off );
        pub fn seekable ( self )  {
        return  true;
        class BadGzipFile ( OSError ) ;
        "Exception raised in some cases for invalid gzip files.";
        class GzipFile ( _compression . BaseStream ) ;
        "The GzipFile class simulates most of the methods of a file object with
    the exception of the truncate() method.

    This class only supports opening files in binary mode. If you need to open a
    compressed file in text mode, use the gzip.open() function.

    ";
        myfileobj = None /* Option */;
        pub fn __init__ ( &self, filename = None /* Option */ , mode = None /* Option */ , {
        compresslevel = _COMPRESS_LEVEL_BEST , fileobj = None /* Option */ , mtime = None /* Option */ ) ;
        "Constructor for the GzipFile class.

        At least one of fileobj && filename must be given a
        non-trivial value.

        The new class instance == based on fileobj, which can be a regular
        file, an io.BytesIO object, || any other object which simulates a file.
        It defaults to None /* Option */, in which case filename == opened to provide
        a file object.

        When fileobj == !None /* Option */, the filename argument == only used to be
        included in the gzip file header, which may include the original
        filename of the uncompressed file.  It defaults to the filename of
        fileobj, if discernible; otherwise, it defaults to the empty string,
        && in this case the original filename == !included in the header.

        The mode argument can be any of 'r', 'rb', 'a', 'ab', 'w', 'wb', 'x', or
        'xb' depending on whether the file will be read || written.  The default
        == the mode of fileobj if discernible; otherwise, the default == 'rb'.
        A mode of 'r' == equivalent to one of 'rb', && similarly for 'w' and
        'wb', 'a' && 'ab', && 'x' && 'xb'.

        The compresslevel argument == an integer from 0 to 9 controlling the
        level of compression; 1 == fastest && produces the least compression,
        && 9 == slowest && produces the most compression. 0 == no compression
        at all. The default == 9.

        The mtime argument == an optional numeric timestamp to be written
        to the last modification time field in the stream when compressing.
        If omitted || None /* Option */, the current time == used.

        ";
        if mode && ( "t" in mode || "U" in mode ) {
        panic!("ValueError ( "Invalid mode: {!r}" . format ( mode ) )");
        if mode && "b" !in mode {
        mode + = "b";
        if fileobj is None /* Option */ {
        fileobj = self . myfileobj = builtins . open ( filename , mode || "rb" );
        if filename is None /* Option */ {
        filename = getattr ( fileobj , "name" , "" );
        if !isinstance ( filename , ( str , bytes ) ) {
        filename = "";
        } else {
        filename = os . fspath ( filename );
        origmode = mode;
        if mode is None /* Option */ {
        mode = getattr ( fileobj , "mode" , "rb" );
        if mode . startswith ( "r" ) {
        self . mode = READ;
        raw = _GzipReader ( fileobj );
        self . _buffer = io . BufferedReader ( raw );
        self . name = filename;
        } else if mode . startswith ( ( "w" , "a" , "x" ) ) {
        if origmode is None /* Option */ {
        import warnings;
        warnings . warn (;
        "GzipFile was opened for writing, but this will ";
        "change in future Python releases.  ";
        "Specify the mode argument for opening it for writing." ,;
        FutureWarning , 2 );
        self . mode = WRITE;
        self . _init_write ( filename );
        self . compress = zlib . compressobj ( compresslevel ,;
        zlib . DEFLATED ,;
        - zlib . MAX_WBITS ,;
        zlib . DEF_MEM_LEVEL ,;
        0 );
        self . _write_mtime = mtime;
        } else {
        panic!("ValueError ( "Invalid mode: {!r}" . format ( mode ) )");
        self . fileobj = fileobj;
        if self . mode == WRITE {
        self . _write_gzip_header ( compresslevel );
        @ property;
        pub fn filename ( self )  {
        import warnings;
        warnings . warn ( "use the name attribute" , DeprecationWarning , 2 );
        if self . mode == WRITE && self . name [ -3 { : ] != ".gz" ; }
        return  self . name + ".gz";
        return  self . name;
        @ property;
        pub fn mtime ( self )  {
        "Last modification time read from stream, || None /* Option */";
        return  self . _buffer . raw . _last_mtime;
        pub fn __repr__ ( self )  {
        s = repr ( self . fileobj );
        return  "<gzip " + s [ 1 : -1 ] + " " + hex ( id ( self ) ) + ">";
        pub fn _init_write ( &self, filename )  {
        self . name = filename;
        self . crc = zlib . crc32 ( b "" );
        self . size = 0;
        self . writebuf = [ ];
        self . bufsize = 0;
        self . offset = 0;
        pub fn _write_gzip_header ( &self, compresslevel )  {
        self . fileobj . write ( b "\037\213" );
        self . fileobj . write ( b "\010" );
        // try {
        fname = os . path . basename ( self . name );
        if !isinstance ( fname , bytes ) {
        fname = fname . encode ( "latin-1" );
        if fname . endswith ( b ".gz" ) {
        fname = fname [ : -3 ];
        // } catch  UnicodeEncodeError  {
        fname = b "";
        flags = 0;
        if fname {
        flags = FNAME;
        self . fileobj . write ( chr ( flags ) . encode ( "latin-1" ) );
        mtime = self . _write_mtime;
        if mtime is None /* Option */ {
        mtime = time . time ( );
        write32u ( self . fileobj , int ( mtime ) );
        if compresslevel == _COMPRESS_LEVEL_BEST {
        xfl = b "\002";
        } else if compresslevel == _COMPRESS_LEVEL_FAST {
        xfl = b "\004";
        } else {
        xfl = b "\000";
        self . fileobj . write ( xfl );
        self . fileobj . write ( b "\377" );
        if fname {
        self . fileobj . write ( fname + b "\000" );
        pub fn write ( &self, data )  {
        self . _check_not_closed ( );
        if self . mode != WRITE {
        import errno;
        panic!("OSError ( errno . EBADF , "write() on read-only GzipFile object" )");
        if self . fileobj is None /* Option */ {
        panic!("ValueError ( "write() on closed GzipFile object" )");
        if isinstance ( data , ( bytes , bytearray ) ) {
        length = len ( data );
        } else {
        data = memoryview ( data );
        length = data . nbytes;
        if length > 0 {
        self . fileobj . write ( self . compress . compress ( data ) );
        self . size + = length;
        self . crc = zlib . crc32 ( data , self . crc );
        self . offset + = length;
        return  length;
        pub fn read ( &self, size = -1 )  {
        self . _check_not_closed ( );
        if self . mode != READ {
        import errno;
        panic!("OSError ( errno . EBADF , "read() on write-only GzipFile object" )");
        return  self . _buffer . read ( size );
        pub fn read1 ( &self, size = -1 )  {
        "Implements BufferedIOBase.read1()

        Reads up to a buffer's worth of data if size == negative.";
        self . _check_not_closed ( );
        if self . mode != READ {
        import errno;
        panic!("OSError ( errno . EBADF , "read1() on write-only GzipFile object" )");
        if size < 0 {
        size = io . DEFAULT_BUFFER_SIZE;
        return  self . _buffer . read1 ( size );
        pub fn peek ( &self, n )  {
        self . _check_not_closed ( );
        if self . mode != READ {
        import errno;
        panic!("OSError ( errno . EBADF , "peek() on write-only GzipFile object" )");
        return  self . _buffer . peek ( n );
        @ property;
        pub fn closed ( self )  {
        return  self . fileobj is None /* Option */;
        pub fn close ( self )  {
        fileobj = self . fileobj;
        if fileobj is None /* Option */ {
        return;
        self . fileobj = None /* Option */;
        // try {
        if self . mode == WRITE {
        fileobj . write ( self . compress . flush ( ) );
        write32u ( fileobj , self . crc );
        write32u ( fileobj , self . size & 0x ffffffff );
        } else if self . mode == READ {
        self . _buffer . close ( );
        // } finally {
        myfileobj = self . myfileobj;
        if myfileobj {
        self . myfileobj = None /* Option */;
        myfileobj . close ( );
        pub fn flush ( &self, zlib_mode = zlib . Z_SYNC_FLUSH )  {
        self . _check_not_closed ( );
        if self . mode == WRITE {
        self . fileobj . write ( self . compress . flush ( zlib_mode ) );
        self . fileobj . flush ( );
        pub fn fileno ( self )  {
        "Invoke the underlying file object's fileno() method.

        This will raise AttributeError if the underlying file object
        doesn't support fileno().
        ";
        return  self . fileobj . fileno ( );
        pub fn rewind ( self )  {
        "Return the uncompressed stream file position indicator to the
        beginning of the file";
        if self . mode != READ {
        panic!("OSError ( "Can't rewind in write mode" )");
        self . _buffer . seek ( 0 );
        pub fn readable ( self )  {
        return  self . mode == READ;
        pub fn writable ( self )  {
        return  self . mode == WRITE;
        pub fn seekable ( self )  {
        return  true;
        pub fn seek ( &self, offset , whence = io . SEEK_SET )  {
        if self . mode == WRITE {
        if whence != io . SEEK_SET {
        if whence == io . SEEK_CUR {
        offset = self . offset + offset;
        } else {
        panic!("ValueError ( "Seek from end !supported" )");
        if offset < self . offset {
        panic!("OSError ( "Negative seek in write mode" )");
        count = offset - self . offset;
        chunk = b "\0" * 1024;
        for i in range ( count / / 1024 ) .iter() {
        self . write ( chunk );
        self . write ( b "\0" * ( count % 1024 ) );
        } else if self . mode == READ {
        self . _check_not_closed ( );
        return  self . _buffer . seek ( offset , whence );
        return  self . offset;
        pub fn readline ( &self, size = -1 )  {
        self . _check_not_closed ( );
        return  self . _buffer . readline ( size );
        pub fn _read_exact ( fp , n )  {
        "Read exactly *n* bytes from `fp`

    This method == required because fp may be unbuffered,
    i.e. return short reads.
    ";
        data = fp . read ( n );
        while len ( data ) < n  {
        b = fp . read ( n - len ( data ) );
        if !b {
        panic!("EOFError ( "Compressed file ended before the "");
        "end-of-stream marker was reached" );
        data + = b;
        return  data;
        pub fn _read_gzip_header ( fp )  {
        "Read a gzip header from `fp` && progress to the end of the header.

    Returns last mtime if header was present || None /* Option */ otherwise.
    ";
        magic = fp . read ( 2 );
        if magic == b "" {
        return;
        if magic != b "\037\213" {
        panic!("BadGzipFile ( "Not a gzipped file (%r)" % magic )");
        ( method , flag , last_mtime ) = struct . unpack ( "<BBIxx" , _read_exact ( fp , 8 ) );
        if method != 8 {
        panic!("BadGzipFile ( "Unknown compression method" )");
        if flag & FEXTRA {
        extra_len , = struct . unpack ( "<H" , _read_exact ( fp , 2 ) );
        _read_exact ( fp , extra_len );
        if flag & FNAME {
        while true  {
        s = fp . read ( 1 );
        if !s || s == b "\000" {
        break;
        if flag & FCOMMENT {
        while true  {
        s = fp . read ( 1 );
        if !s || s == b "\000" {
        break;
        if flag & FHCRC {
        _read_exact ( fp , 2 );
        return  last_mtime;
        class _GzipReader ( _compression . DecompressReader ) ;
        pub fn __init__ ( &self, fp )  {
        super ( ) . __init__ ( _PaddedFile ( fp ) , zlib . decompressobj ,;
        wbits = - zlib . MAX_WBITS );
        self . _new_member = true;
        self . _last_mtime = None /* Option */;
        pub fn _init_read ( self )  {
        self . _crc = zlib . crc32 ( b "" );
        self . _stream_size = 0;
        pub fn _read_gzip_header ( self )  {
        last_mtime = _read_gzip_header ( self . _fp );
        if last_mtime is None /* Option */ {
        return  false;
        self . _last_mtime = last_mtime;
        return  true;
        pub fn read ( &self, size = -1 )  {
        if size < 0 {
        return  self . readall ( );
        if !size {
        return  b "";
        while true  {
        if self . _decompressor . eof {
        self . _read_eof ( );
        self . _new_member = true;
        self . _decompressor = self . _decomp_factory (;
        ** self . _decomp_args );
        if self . _new_member {
        self . _init_read ( );
        if !self . _read_gzip_header ( ) {
        self . _size = self . _pos;
        return  b "";
        self . _new_member = false;
        buf = self . _fp . read ( io . DEFAULT_BUFFER_SIZE );
        uncompress = self . _decompressor . decompress ( buf , size );
        if self . _decompressor . unconsumed_tail != b "" {
        self . _fp . prepend ( self . _decompressor . unconsumed_tail );
        } else if self . _decompressor . unused_data != b "" {
        self . _fp . prepend ( self . _decompressor . unused_data );
        if uncompress != b "" {
        break;
        if buf == b "" {
        panic!("EOFError ( "Compressed file ended before the "");
        "end-of-stream marker was reached" );
        self . _add_read_data ( uncompress );
        self . _pos + = len ( uncompress );
        return  uncompress;
        pub fn _add_read_data ( &self, data )  {
        self . _crc = zlib . crc32 ( data , self . _crc );
        self . _stream_size = self . _stream_size + len ( data );
        pub fn _read_eof ( self )  {
        crc32 , isize = struct . unpack ( "<II" , _read_exact ( self . _fp , 8 ) );
        if crc32 != self . _crc {
        panic!("BadGzipFile ( "CRC check failed %s != %s" % ( hex ( crc32 ) ,");
        hex ( self . _crc ) ) );
        } else if isize != ( self . _stream_size & 0x ffffffff ) {
        panic!("BadGzipFile ( "Incorrect length of data produced" )");
        c = b "\x00";
        while c == b "\x00"  {
        c = self . _fp . read ( 1 );
        if c {
        self . _fp . prepend ( c );
        pub fn _rewind ( self )  {
        super ( ) . _rewind ( );
        self . _new_member = true;
        pub fn _create_simple_gzip_header ( compresslevel  {  int ,; }
        mtime = None /* Option */ ) - > bytes ;
        "
    Write a simple gzip header with no extra fields.
    :param compresslevel: Compresslevel used to determine the xfl bytes.
    :param mtime: The mtime (must support conversion to a 32-bit integer).
    :return: A bytes object representing the gzip header.
    ";
        if mtime is None /* Option */ {
        mtime = time . time ( );
        if compresslevel == _COMPRESS_LEVEL_BEST {
        xfl = 2;
        } else if compresslevel == _COMPRESS_LEVEL_FAST {
        xfl = 4;
        } else {
        xfl = 0;
        return  struct . pack ( "<BBBBLBB" , 0x1 f , 0x8 b , 8 , 0 , int ( mtime ) , xfl , 255 );
        pub fn compress ( data , compresslevel = _COMPRESS_LEVEL_BEST , * , mtime = None /* Option */ )  {
        "Compress data in one shot && return the compressed string.

    compresslevel sets the compression level in range of 0-9.
    mtime can be used to set the modification time. The modification time is
    set to the current time by default.
    ";
        if mtime == 0 {
        return  zlib . compress ( data , level = compresslevel , wbits = 31 );
        header = _create_simple_gzip_header ( compresslevel , mtime );
        trailer = struct . pack ( "<LL" , zlib . crc32 ( data ) , ( len ( data ) & 0x ffffffff ) );
        return  ( header + zlib . compress ( data , level = compresslevel , wbits = -15 ) +;
        trailer );
        pub fn decompress ( data )  {
        "Decompress a gzip compressed string in one shot.
    Return the decompressed string.
    ";
        decompressed_members = [ ];
        while true  {
        fp = io . BytesIO ( data );
        if _read_gzip_header ( fp ) is None /* Option */ {
        return  b "" . join ( decompressed_members );
        do = zlib . decompressobj ( wbits = - zlib . MAX_WBITS );
        decompressed = do . decompress ( data [ fp . tell ( ) : ] );
        if !do . eof || len ( do . unused_data ) < 8 {
        panic!("EOFError ( "Compressed file ended before the end-of-stream "");
        "marker was reached" );
        crc , length = struct . unpack ( "<II" , do . unused_data [ : 8 ] );
        if crc != zlib . crc32 ( decompressed ) {
        panic!("BadGzipFile ( "CRC check failed" )");
        if length != ( len ( decompressed ) & 0x ffffffff ) {
        panic!("BadGzipFile ( "Incorrect length of data produced" )");
        decompressed_members . append ( decompressed );
        data = do . unused_data [ 8 : ] . lstrip ( b "\x00" );
        pub fn main ( )  {
        from argparse import ArgumentParser;
        parser = ArgumentParser ( description =;
        "A simple command line interface for the gzip module: act like gzip, ";
        "but do !delete the input file." );
        group = parser . add_mutually_exclusive_group ( );
        group . add_argument ( "--fast" , action = "store_true" , help = "compress faster" );
        group . add_argument ( "--best" , action = "store_true" , help = "compress better" );
        group . add_argument ( "-d" , "--decompress" , action = "store_true" ,;
        help = "act like gunzip instead of gzip" );
        parser . add_argument ( "args" , nargs = "*" , default = [ "-" ] , metavar = "file" );
        args = parser . parse_args ( );
        compresslevel = _COMPRESS_LEVEL_TRADEOFF;
        if args . fast {
        compresslevel = _COMPRESS_LEVEL_FAST;
        } else if args . best {
        compresslevel = _COMPRESS_LEVEL_BEST;
        for arg in args . args .iter() {
        if args . decompress {
        if arg == "-" {
        f = GzipFile ( filename = "" , mode = "rb" , fileobj = sys . stdin . buffer );
        g = sys . stdout . buffer;
        } else {
        if arg [ -3 { : ] != ".gz" ; }
        sys . exit ( format!("filename doesn't end in .gz: {arg!r}" ));
        f = open ( arg , "rb" );
        g = builtins . open ( arg [ : -3 ] , "wb" );
        } else {
        if arg == "-" {
        f = sys . stdin . buffer;
        g = GzipFile ( filename = "" , mode = "wb" , fileobj = sys . stdout . buffer ,;
        compresslevel = compresslevel );
        } else {
        f = builtins . open ( arg , "rb" );
        g = open ( arg + ".gz" , "wb" );
        while true  {
        chunk = f . read ( io . DEFAULT_BUFFER_SIZE );
        if !chunk {
        break;
        g . write ( chunk );
        if g is !sys . stdout . buffer {
        g . close ( );
        if f is !sys . stdin . buffer {
        f . close ( );
        fn main() {
        main ( );
}

