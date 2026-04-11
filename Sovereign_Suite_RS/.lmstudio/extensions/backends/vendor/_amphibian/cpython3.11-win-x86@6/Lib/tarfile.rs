//! tarfile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::builtins::{open, bltn_open};
// use std::env;
// use crate::io;
// use crate::stat;
// use crate::struct;
// use regex::Regex;
// use crate::pwd;
// use crate::grp;
// use crate::zlib;
// use crate::bz2;
// use crate::lzma;
// use crate::gzip::{GzipFile};
// use crate::argparse;

pub const version: &str = "0.9.0";
pub const __author__: &str = "Lars Gust\u00e4bel (lars@gustaebel.de)";
pub const __credits__: &str = "Gustavo Niemeyer, Niels Gust\u00e4bel, Richard Townsend.";
pub const symlink_exception: f64 = ( AttributeError , NotImplementedError );
pub const __all__: &str = ["TarFile" ,"TarInfo" ,"is_tarfile" ,"TarError" ,"ReadError" ,;
pub const NUL: &str = b"\0";
pub const BLOCKSIZE: u64 = 512;
pub const RECORDSIZE: /* inferred */ = BLOCKSIZE * 20;
pub const GNU_MAGIC: &str = b"ustar  \0";
pub const POSIX_MAGIC: &str = b"ustar\x0000";
pub const LENGTH_NAME: u64 = 100;
pub const LENGTH_LINK: u64 = 100;
pub const LENGTH_PREFIX: u64 = 155;
pub const REGTYPE: &str = b"0";
pub const AREGTYPE: &str = b"\0";
pub const LNKTYPE: &str = b"1";
pub const SYMTYPE: &str = b"2";
pub const CHRTYPE: &str = b"3";
pub const BLKTYPE: &str = b"4";
pub const DIRTYPE: &str = b"5";
pub const FIFOTYPE: &str = b"6";
pub const CONTTYPE: &str = b"7";
pub const GNUTYPE_LONGNAME: &str = b"L";
pub const GNUTYPE_LONGLINK: &str = b"K";
pub const GNUTYPE_SPARSE: &str = b"S";
pub const XHDTYPE: &str = b"x";
pub const XGLTYPE: &str = b"g";
pub const SOLARIS_XHDTYPE: &str = b"X";
pub const USTAR_FORMAT: u64 = 0;
pub const GNU_FORMAT: u64 = 1;
pub const PAX_FORMAT: u64 = 2;
pub const DEFAULT_FORMAT: /* inferred */ = PAX_FORMAT;
pub const SUPPORTED_TYPES: f64 = ( REGTYPE , AREGTYPE , LNKTYPE ,;
pub const REGULAR_TYPES: f64 = ( REGTYPE , AREGTYPE ,;
pub const GNU_TYPES: f64 = ( GNUTYPE_LONGNAME , GNUTYPE_LONGLINK ,;
pub const PAX_FIELDS: &str = ("path" ,"linkpath" ,"size" ,"mtime" ,;
pub const PAX_NAME_FIELDS: &str = {"path" ,"linkpath" ,"uname" ,"gname" };
pub const PAX_NUMBER_FIELDS: f64 = {;
pub fn stn(s: &str, length: &str, encoding: &str, errors: &str) {
        "Convert a string to a null-terminated bytes object.
    ";
        if s is None /* Option */ {
        panic!("ValueError ( "metadata cannot contain None /* Option */" )");
        s = s . encode ( encoding , errors );
        return  s [ : length ] + ( length - len ( s ) ) * NUL;
        pub fn nts ( s , encoding , errors )  {
        "Convert a null-terminated bytes object to a string.
    ";
        p = s . find ( b "\0" );
        if p != -1 {
        s = s [ : p ];
        return  s . decode ( encoding , errors );
        pub fn nti ( s )  {
        "Convert a number field to a python number.
    ";
        if s [ 0 ] in ( 0 o200 , 0 o377 ) {
        n = 0;
        for i in range ( len ( s ) - 1 ) .iter() {
        n < <= 8;
        n + = s [ i + 1 ];
        if s [ 0 ] == 0 o377 {
        n = - ( 256 ** ( len ( s ) - 1 ) - n );
        } else {
        // try {
        s = nts ( s , "ascii" , "strict" );
        n = int ( s . strip ( ) || "0" , 8 );
        // } catch  ValueError  {
        panic!("InvalidHeaderError ( "invalid header" )");
        return  n;
        pub fn itn ( n , digits = 8 , format = DEFAULT_FORMAT )  {
        "Convert a python number to a number field.
    ";
        original_n = n;
        n = int ( n );
        if 0 <= n < 8 ** ( digits - 1 ) {
        s = bytes ( "%0*o" % ( digits - 1 , n ) , "ascii" ) + NUL;
        } else if format == GNU_FORMAT && -256 ** ( digits - 1 ) <= n < 256 ** ( digits - 1 ) {
        if n >= 0 {
        s = bytearray ( [ 0 o200 ] );
        } else {
        s = bytearray ( [ 0 o377 ] );
        n = 256 ** digits + n;
        for i in range ( digits - 1 ) .iter() {
        s . insert ( 1 , n & 0 o377 );
        n > >= 8;
        } else {
        panic!("ValueError ( "overflow in number field" )");
        return  s;
        pub fn calc_chksums ( buf )  {
        "Calculate the checksum for a member's header by summing up all
       characters except for the chksum field which == treated as if
       it was filled with spaces. According to the GNU tar sources,
       some tars (Sun && NeXT) calculate chksum with signed char,
       which will be different if there are chars in the buffer with
       the high bit set. So we calculate two checksums, unsigned and
       signed.
    ";
        unsigned_chksum = 256 + sum ( struct . unpack_from ( "148B8x356B" , buf ) );
        signed_chksum = 256 + sum ( struct . unpack_from ( "148b8x356b" , buf ) );
        return  unsigned_chksum , signed_chksum;
        pub fn copyfileobj ( src , dst , length = None /* Option */ , exception = OSError , bufsize = None /* Option */ )  {
        "Copy length bytes from fileobj src to fileobj dst.
       If length == None /* Option */, copy the entire content.
    ";
        bufsize = bufsize || 16 * 1024;
        if length == 0 {
        return;
        if length is None /* Option */ {
        shutil . copyfileobj ( src , dst , bufsize );
        return;
        blocks , remainder = divmod ( length , bufsize );
        for b in range ( blocks ) .iter() {
        buf = src . read ( bufsize );
        if len ( buf ) < bufsize {
        panic!("exception ( "unexpected end of data" )");
        dst . write ( buf );
        if remainder != 0 {
        buf = src . read ( remainder );
        if len ( buf ) < remainder {
        panic!("exception ( "unexpected end of data" )");
        dst . write ( buf );
        return;
        pub fn _safe_print ( s )  {
        encoding = getattr ( sys . stdout , "encoding" , None /* Option */ );
        if encoding is !None /* Option */ {
        s = s . encode ( encoding , "backslashreplace" ) . decode ( encoding );
        println!( s , end = " " );
        class TarError ( Exception ) ;
        "Base exception.";
        // pass
        class ExtractError ( TarError ) ;
        "General exception for extract errors.";
        // pass
        class ReadError ( TarError ) ;
        "Exception for unreadable tar archives.";
        // pass
        class CompressionError ( TarError ) ;
        "Exception for unavailable compression methods.";
        // pass
        class StreamError ( TarError ) ;
        "Exception for unsupported operations on stream-like TarFiles.";
        // pass
        class HeaderError ( TarError ) ;
        "Base exception for header errors.";
        // pass
        class EmptyHeaderError ( HeaderError ) ;
        "Exception for empty headers.";
        // pass
        class TruncatedHeaderError ( HeaderError ) ;
        "Exception for truncated headers.";
        // pass
        class EOFHeaderError ( HeaderError ) ;
        "Exception for end of file headers.";
        // pass
        class InvalidHeaderError ( HeaderError ) ;
        "Exception for invalid headers.";
        // pass
        class SubsequentHeaderError ( HeaderError ) ;
        "Exception for missing && invalid extended headers.";
        // pass
        class _LowLevelFile ;
        "Low-level file object. Supports reading && writing.
       It == used instead of a regular file object for streaming
       access.
    ";
        pub fn __init__ ( &self, name , mode )  {
        mode = {;
        "r" : os . O_RDONLY ,;
        "w" : os . O_WRONLY | os . O_CREAT | os . O_TRUNC ,;
        } [ mode ];
        if hasattr ( os , "O_BINARY" ) {
        mode | = os . O_BINARY;
        self . fd = os . open ( name , mode , 0 o666 );
        pub fn close ( self )  {
        os . close ( self . fd );
        pub fn read ( &self, size )  {
        return  os . read ( self . fd , size );
        pub fn write ( &self, s )  {
        os . write ( self . fd , s );
        class _Stream ;
        "Class that serves as an adapter between TarFile and
       a stream-like object.  The stream-like object only
       needs to have a read() || write() method that works with bytes,
       && the method == accessed blockwise.
       Use of gzip || bzip2 compression == possible.
       A stream-like object could be for example: sys.stdin.buffer,
       sys.stdout.buffer, a socket, a tape device etc.

       _Stream == intended to be used only internally.
    ";
        pub fn __init__ ( &self, name , mode , comptype , fileobj , bufsize )  {
        "Construct a _Stream object.
        ";
        self . _extfileobj = true;
        if fileobj is None /* Option */ {
        fileobj = _LowLevelFile ( name , mode );
        self . _extfileobj = false;
        if comptype == "*" {
        fileobj = _StreamProxy ( fileobj );
        comptype = fileobj . getcomptype ( );
        self . name = name || "";
        self . mode = mode;
        self . comptype = comptype;
        self . fileobj = fileobj;
        self . bufsize = bufsize;
        self . buf = b "";
        self . pos = 0;
        self . closed = false;
        // try {
        if comptype == "gz" {
        // try {
        import zlib;
        // } catch  ImportError  {
        panic!("CompressionError ( "zlib module is !available" ) from None /* Option */");
        self . zlib = zlib;
        self . crc = zlib . crc32 ( b "" );
        if mode == "r" {
        self . exception = zlib . error;
        self . _init_read_gz ( );
        } else {
        self . _init_write_gz ( );
        } else if comptype == "bz2" {
        // try {
        import bz2;
        // } catch  ImportError  {
        panic!("CompressionError ( "bz2 module is !available" ) from None /* Option */");
        if mode == "r" {
        self . dbuf = b "";
        self . cmp = bz2 . BZ2Decompressor ( );
        self . exception = OSError;
        } else {
        self . cmp = bz2 . BZ2Compressor ( );
        } else if comptype == "xz" {
        // try {
        import lzma;
        // } catch  ImportError  {
        panic!("CompressionError ( "lzma module is !available" ) from None /* Option */");
        if mode == "r" {
        self . dbuf = b "";
        self . cmp = lzma . LZMADecompressor ( );
        self . exception = lzma . LZMAError;
        } else {
        self . cmp = lzma . LZMACompressor ( );
        } else if comptype != "tar" {
        panic!("CompressionError ( "unknown compression type %r" % comptype )");
        // } catch   {
        if !self . _extfileobj {
        self . fileobj . close ( );
        self . closed = true;
        panic!("");
        pub fn __del__ ( self )  {
        if hasattr ( self , "closed" ) && !self . closed {
        self . close ( );
        pub fn _init_write_gz ( self )  {
        "Initialize for writing with gzip compression.
        ";
        self . cmp = self . zlib . compressobj ( 9 , self . zlib . DEFLATED ,;
        - self . zlib . MAX_WBITS ,;
        self . zlib . DEF_MEM_LEVEL ,;
        0 );
        timestamp = struct . pack ( "<L" , int ( time . time ( ) ) );
        self . __write ( b "\037\213\010\010" + timestamp + b "\002\377" );
        if self . name . endswith ( ".gz" ) {
        self . name = self . name [ : -3 ];
        self . name = os . path . basename ( self . name );
        self . __write ( self . name . encode ( "iso-8859-1" , "replace" ) + NUL );
        pub fn write ( &self, s )  {
        "Write string s to the stream.
        ";
        if self . comptype == "gz" {
        self . crc = self . zlib . crc32 ( s , self . crc );
        self . pos + = len ( s );
        if self . comptype != "tar" {
        s = self . cmp . compress ( s );
        self . __write ( s );
        pub fn __write ( &self, s )  {
        "Write string s to the stream if a whole new block
           == ready to be written.
        ";
        self . buf + = s;
        while len ( self . buf ) > self . bufsize  {
        self . fileobj . write ( self . buf [ : self . bufsize ] );
        self . buf = self . buf [ self . bufsize : ];
        pub fn close ( self )  {
        "Close the _Stream object. No operation should be
           done on it afterwards.
        ";
        if self . closed {
        return;
        self . closed = true;
        // try {
        if self . mode == "w" && self . comptype != "tar" {
        self . buf + = self . cmp . flush ( );
        if self . mode == "w" && self . buf {
        self . fileobj . write ( self . buf );
        self . buf = b "";
        if self . comptype == "gz" {
        self . fileobj . write ( struct . pack ( "<L" , self . crc ) );
        self . fileobj . write ( struct . pack ( "<L" , self . pos & 0x ffffFFFF ) );
        // } finally {
        if !self . _extfileobj {
        self . fileobj . close ( );
        pub fn _init_read_gz ( self )  {
        "Initialize for reading a gzip compressed fileobj.
        ";
        self . cmp = self . zlib . decompressobj ( - self . zlib . MAX_WBITS );
        self . dbuf = b "";
        if self . __read ( 2 ) != b "\037\213" {
        panic!("ReadError ( "not a gzip file" )");
        if self . __read ( 1 ) != b "\010" {
        panic!("CompressionError ( "unsupported compression method" )");
        flag = ord ( self . __read ( 1 ) );
        self . __read ( 6 );
        if flag & 4 {
        xlen = ord ( self . __read ( 1 ) ) + 256 * ord ( self . __read ( 1 ) );
        self . read ( xlen );
        if flag & 8 {
        while true  {
        s = self . __read ( 1 );
        if !s || s == NUL {
        break;
        if flag & 16 {
        while true  {
        s = self . __read ( 1 );
        if !s || s == NUL {
        break;
        if flag & 2 {
        self . __read ( 2 );
        pub fn tell ( self )  {
        "Return the stream's file pointer position.
        ";
        return  self . pos;
        pub fn seek ( &self, pos = 0 )  {
        "Set the stream's file pointer to pos. Negative seeking
           == forbidden.
        ";
        if pos - self . pos >= 0 {
        blocks , remainder = divmod ( pos - self . pos , self . bufsize );
        for i in range ( blocks ) .iter() {
        self . read ( self . bufsize );
        self . read ( remainder );
        } else {
        panic!("StreamError ( "seeking backwards is !allowed" )");
        return  self . pos;
        pub fn read ( &self, size )  {
        "Return the next size number of bytes from the stream.";
        assert size == !None /* Option */;
        buf = self . _read ( size );
        self . pos + = len ( buf );
        return  buf;
        pub fn _read ( &self, size )  {
        "Return size bytes from the stream.
        ";
        if self . comptype == "tar" {
        return  self . __read ( size );
        c = len ( self . dbuf );
        t = [ self . dbuf ];
        while c < size  {
        if self . buf {
        buf = self . buf;
        self . buf = b "";
        } else {
        buf = self . fileobj . read ( self . bufsize );
        if !buf {
        break;
        // try {
        buf = self . cmp . decompress ( buf );
        // } catch  self . exception as e  {
        panic!("ReadError ( "invalid compressed data" ) from e");
        t . append ( buf );
        c + = len ( buf );
        t = b "" . join ( t );
        self . dbuf = t [ size : ];
        return  t [ : size ];
        pub fn __read ( &self, size )  {
        "Return size bytes from stream. If internal buffer == empty,
           read another block from the stream.
        ";
        c = len ( self . buf );
        t = [ self . buf ];
        while c < size  {
        buf = self . fileobj . read ( self . bufsize );
        if !buf {
        break;
        t . append ( buf );
        c + = len ( buf );
        t = b "" . join ( t );
        self . buf = t [ size : ];
        return  t [ : size ];
        class _StreamProxy ( object ) ;
        "Small proxy class that enables transparent compression
       detection for the Stream interface (mode 'r|*').
    ";
        pub fn __init__ ( &self, fileobj )  {
        self . fileobj = fileobj;
        self . buf = self . fileobj . read ( BLOCKSIZE );
        pub fn read ( &self, size )  {
        self . read = self . fileobj . read;
        return  self . buf;
        pub fn getcomptype ( self )  {
        if self . buf . startswith ( b "\x1f\x8b\x08" ) {
        return  "gz";
        } else if self . buf [ 0 {
        return  "bz2";
        } else if self . buf . startswith ( ( b "\x5d\x00\x00\x80" , b "\xfd7zXZ" ) ) {
        return  "xz";
        } else {
        return  "tar";
        pub fn close ( self )  {
        self . fileobj . close ( );
        class _FileInFile ( object ) ;
        "A thin wrapper around an existing file object that
       provides a part of its data as an individual file
       object.
    ";
        pub fn __init__ ( &self, fileobj , offset , size , blockinfo = None /* Option */ )  {
        self . fileobj = fileobj;
        self . offset = offset;
        self . size = size;
        self . position = 0;
        self . name = getattr ( fileobj , "name" , None /* Option */ );
        self . closed = false;
        if blockinfo is None /* Option */ {
        blockinfo = [ ( 0 , size ) ];
        self . map_index = 0;
        self . map = [ ];
        lastpos = 0;
        realpos = self . offset;
        for offset , size in blockinfo .iter() {
        if offset > lastpos {
        self . map . append ( ( false , lastpos , offset , None /* Option */ ) );
        self . map . append ( ( true , offset , offset + size , realpos ) );
        realpos + = size;
        lastpos = offset + size;
        if lastpos < self . size {
        self . map . append ( ( false , lastpos , self . size , None /* Option */ ) );
        pub fn flush ( self )  {
        // pass
        pub fn readable ( self )  {
        return  true;
        pub fn writable ( self )  {
        return  false;
        pub fn seekable ( self )  {
        return  self . fileobj . seekable ( );
        pub fn tell ( self )  {
        "Return the current file position.
        ";
        return  self . position;
        pub fn seek ( &self, position , whence = io . SEEK_SET )  {
        "Seek to a position in the file.
        ";
        if whence == io . SEEK_SET {
        self . position = min ( max ( position , 0 ) , self . size );
        } else if whence == io . SEEK_CUR {
        if position < 0 {
        self . position = max ( self . position + position , 0 );
        } else {
        self . position = min ( self . position + position , self . size );
        } else if whence == io . SEEK_END {
        self . position = max ( min ( self . size + position , self . size ) , 0 );
        } else {
        panic!("ValueError ( "Invalid argument" )");
        return  self . position;
        pub fn read ( &self, size = None /* Option */ )  {
        "Read data from the file.
        ";
        if size is None /* Option */ {
        size = self . size - self . position;
        } else {
        size = min ( size , self . size - self . position );
        buf = b "";
        while size > 0  {
        while true  {
        data , start , stop , offset = self . map [ self . map_index ];
        if start <= self . position < stop {
        break;
        } else {
        self . map_index + = 1;
        if self . map_index == len ( self . map ) {
        self . map_index = 0;
        length = min ( size , stop - self . position );
        if data {
        self . fileobj . seek ( offset + ( self . position - start ) );
        b = self . fileobj . read ( length );
        if len ( b ) != length {
        panic!("ReadError ( "unexpected end of data" )");
        buf + = b;
        } else {
        buf + = NUL * length;
        size - = length;
        self . position + = length;
        return  buf;
        pub fn readinto ( &self, b )  {
        buf = self . read ( len ( b ) );
        b [ : len ( buf ) ] = buf;
        return  len ( buf );
        pub fn close ( self )  {
        self . closed = true;
        class ExFileObject ( io . BufferedReader ) ;
        pub fn __init__ ( &self, tarfile , tarinfo )  {
        fileobj = _FileInFile ( tarfile . fileobj , tarinfo . offset_data ,;
        tarinfo . size , tarinfo . sparse );
        super ( ) . __init__ ( fileobj );
        class FilterError ( TarError ) ;
        // pass
        class AbsolutePathError ( FilterError ) ;
        pub fn __init__ ( &self, tarinfo )  {
        self . tarinfo = tarinfo;
        super ( ) . __init__ ( format!("member {tarinfo.name!r} has an absolute path" ));
        class OutsideDestinationError ( FilterError ) ;
        pub fn __init__ ( &self, tarinfo , path )  {
        self . tarinfo = tarinfo;
        self . _path = path;
        super ( ) . __init__ ( format!("{tarinfo.name!r} would be extracted to {path!r}, ");
        + "which == outside the destination" );
        class SpecialFileError ( FilterError ) ;
        pub fn __init__ ( &self, tarinfo )  {
        self . tarinfo = tarinfo;
        super ( ) . __init__ ( format!("{tarinfo.name!r} == a special file" ));
        class AbsoluteLinkError ( FilterError ) ;
        pub fn __init__ ( &self, tarinfo )  {
        self . tarinfo = tarinfo;
        super ( ) . __init__ ( format!("{tarinfo.name!r} == a link to an absolute path" ));
        class LinkOutsideDestinationError ( FilterError ) ;
        pub fn __init__ ( &self, tarinfo , path )  {
        self . tarinfo = tarinfo;
        self . _path = path;
        super ( ) . __init__ ( format!("{tarinfo.name!r} would link to {path!r}, ");
        + "which == outside the destination" );
        pub fn _get_filtered_attrs ( member , dest_path , for_data = true )  {
        new_attrs = { };
        name = member . name;
        dest_path = os . path . realpath ( dest_path );
        if name . startswith ( ( "/" , os . sep ) ) {
        name = new_attrs [ "name" ] = member . path . lstrip ( "/" + os . sep );
        if os . path . isabs ( name ) {
        panic!("AbsolutePathError ( member )");
        target_path = os . path . realpath ( os . path . join ( dest_path , name ) );
        if os . path . commonpath ( [ target_path , dest_path ] ) != dest_path {
        panic!("OutsideDestinationError ( member , target_path )");
        mode = member . mode;
        if mode is !None /* Option */ {
        mode = mode & 0 o755;
        if for_data {
        if member . isreg ( ) || member . islnk ( ) {
        if !mode & 0 o100 {
        mode & = ~ 0 o111;
        mode | = 0 o600;
        } else if member . isdir ( ) || member . issym ( ) {
        mode = None /* Option */;
        } else {
        panic!("SpecialFileError ( member )");
        if mode != member . mode {
        new_attrs [ "mode" ] = mode;
        if for_data {
        if member . uid is !None /* Option */ {
        new_attrs [ "uid" ] = None /* Option */;
        if member . gid is !None /* Option */ {
        new_attrs [ "gid" ] = None /* Option */;
        if member . uname is !None /* Option */ {
        new_attrs [ "uname" ] = None /* Option */;
        if member . gname is !None /* Option */ {
        new_attrs [ "gname" ] = None /* Option */;
        if member . islnk ( ) || member . issym ( ) {
        if os . path . isabs ( member . linkname ) {
        panic!("AbsoluteLinkError ( member )");
        if member . issym ( ) {
        target_path = os . path . join ( dest_path ,;
        os . path . dirname ( name ) ,;
        member . linkname );
        } else {
        target_path = os . path . join ( dest_path ,;
        member . linkname );
        target_path = os . path . realpath ( target_path );
        if os . path . commonpath ( [ target_path , dest_path ] ) != dest_path {
        panic!("LinkOutsideDestinationError ( member , target_path )");
        return  new_attrs;
        pub fn fully_trusted_filter ( member , dest_path )  {
        return  member;
        pub fn tar_filter ( member , dest_path )  {
        new_attrs = _get_filtered_attrs ( member , dest_path , false );
        if new_attrs {
        return  member . replace ( ** new_attrs , deep = false );
        return  member;
        pub fn data_filter ( member , dest_path )  {
        new_attrs = _get_filtered_attrs ( member , dest_path , true );
        if new_attrs {
        return  member . replace ( ** new_attrs , deep = false );
        return  member;
        _NAMED_FILTERS = {;
        "fully_trusted" : fully_trusted_filter ,;
        "tar" : tar_filter ,;
        "data" : data_filter ,;
        };
        _KEEP = object ( );
        class TarInfo ( object ) ;
        "Informational class which holds the details about an
       archive member given by a tar header block.
       TarInfo objects are returned by TarFile.getmember(),
       TarFile.getmembers() && TarFile.gettarinfo() && are
       usually created internally.
    ";
        __slots__ = dict (;
        name = "Name of the archive member." ,;
        mode = "Permission bits." ,;
        uid = "User ID of the user who originally stored this member." ,;
        gid = "Group ID of the user who originally stored this member." ,;
        size = "Size in bytes." ,;
        mtime = "Time of last modification." ,;
        chksum = "Header checksum." ,;
        type = ( "File type. type == usually one of these constants: ";
        "REGTYPE, AREGTYPE, LNKTYPE, SYMTYPE, DIRTYPE, FIFOTYPE, ";
        "CONTTYPE, CHRTYPE, BLKTYPE, GNUTYPE_SPARSE." ) ,;
        linkname = ( "Name of the target file name, which == only present ";
        "in TarInfo objects of type LNKTYPE && SYMTYPE." ) ,;
        uname = "User name." ,;
        gname = "Group name." ,;
        devmajor = "Device major number." ,;
        devminor = "Device minor number." ,;
        offset = "The tar header starts here." ,;
        offset_data = "The file's data starts here." ,;
        pax_headers = ( "A dictionary containing key-value pairs of an ";
        "associated pax extended header." ) ,;
        sparse = "Sparse member information." ,;
        tarfile = None /* Option */ ,;
        _sparse_structs = None /* Option */ ,;
        _link_target = None /* Option */ ,;
        );
        pub fn __init__ ( &self, name = "" )  {
        "Construct a TarInfo object. name == the optional name
           of the member.
        ";
        self . name = name;
        self . mode = 0 o644;
        self . uid = 0;
        self . gid = 0;
        self . size = 0;
        self . mtime = 0;
        self . chksum = 0;
        self . type = REGTYPE;
        self . linkname = "";
        self . uname = "";
        self . gname = "";
        self . devmajor = 0;
        self . devminor = 0;
        self . offset = 0;
        self . offset_data = 0;
        self . sparse = None /* Option */;
        self . pax_headers = { };
        @ property;
        pub fn path ( self )  {
        "In pax headers, "name" == called "path".";
        return  self . name;
        @ path . setter;
        pub fn path ( &self, name )  {
        self . name = name;
        @ property;
        pub fn linkpath ( self )  {
        "In pax headers, "linkname" == called "linkpath".";
        return  self . linkname;
        @ linkpath . setter;
        pub fn linkpath ( &self, linkname )  {
        self . linkname = linkname;
        pub fn __repr__ ( self )  {
        return  "<%s %r at %#x>" % ( self . __class__ . __name__ , self . name , id ( self ) );
        pub fn replace ( &self, * , {
        name = _KEEP , mtime = _KEEP , mode = _KEEP , linkname = _KEEP ,;
        uid = _KEEP , gid = _KEEP , uname = _KEEP , gname = _KEEP ,;
        deep = true , _KEEP = _KEEP ) ;
        "Return a deep copy of self with the given attributes replaced.
        ";
        if deep {
        result = copy . deepcopy ( self );
        } else {
        result = copy . copy ( self );
        if name is !_KEEP {
        result . name = name;
        if mtime is !_KEEP {
        result . mtime = mtime;
        if mode is !_KEEP {
        result . mode = mode;
        if linkname is !_KEEP {
        result . linkname = linkname;
        if uid is !_KEEP {
        result . uid = uid;
        if gid is !_KEEP {
        result . gid = gid;
        if uname is !_KEEP {
        result . uname = uname;
        if gname is !_KEEP {
        result . gname = gname;
        return  result;
        pub fn get_info ( self )  {
        "Return the TarInfo's attributes as a dictionary.
        ";
        if self . mode is None /* Option */ {
        mode = None /* Option */;
        } else {
        mode = self . mode & 0 o7777;
        info = {;
        "name" : self . name ,;
        "mode" : mode ,;
        "uid" : self . uid ,;
        "gid" : self . gid ,;
        "size" : self . size ,;
        "mtime" : self . mtime ,;
        "chksum" : self . chksum ,;
        "type" : self . type ,;
        "linkname" : self . linkname ,;
        "uname" : self . uname ,;
        "gname" : self . gname ,;
        "devmajor" : self . devmajor ,;
        "devminor" : self . devminor;
        };
        if info [ "type" ] == DIRTYPE && !info [ "name" ] . endswith ( "/" ) {
        info [ "name" ] + = "/";
        return  info;
        pub fn tobuf ( &self, format = DEFAULT_FORMAT , encoding = ENCODING , errors = "surrogateescape" )  {
        "Return a tar header as a string of 512 byte blocks.
        ";
        info = self . get_info ( );
        for name , value in info . items ( ) .iter() {
        if value is None /* Option */ {
        panic!("ValueError ( "%s may !be None /* Option */" % name )");
        if format == USTAR_FORMAT {
        return  self . create_ustar_header ( info , encoding , errors );
        } else if format == GNU_FORMAT {
        return  self . create_gnu_header ( info , encoding , errors );
        } else if format == PAX_FORMAT {
        return  self . create_pax_header ( info , encoding );
        } else {
        panic!("ValueError ( "invalid format" )");
        pub fn create_ustar_header ( &self, info , encoding , errors )  {
        "Return the object as a ustar header block.
        ";
        info [ "magic" ] = POSIX_MAGIC;
        if len ( info [ "linkname" ] . encode ( encoding , errors ) ) > LENGTH_LINK {
        panic!("ValueError ( "linkname is too long" )");
        if len ( info [ "name" ] . encode ( encoding , errors ) ) > LENGTH_NAME {
        info [ "prefix" ] , info [ "name" ] = self . _posix_split_name ( info [ "name" ] , encoding , errors );
        return  self . _create_header ( info , USTAR_FORMAT , encoding , errors );
        pub fn create_gnu_header ( &self, info , encoding , errors )  {
        "Return the object as a GNU header block sequence.
        ";
        info [ "magic" ] = GNU_MAGIC;
        buf = b "";
        if len ( info [ "linkname" ] . encode ( encoding , errors ) ) > LENGTH_LINK {
        buf + = self . _create_gnu_long_header ( info [ "linkname" ] , GNUTYPE_LONGLINK , encoding , errors );
        if len ( info [ "name" ] . encode ( encoding , errors ) ) > LENGTH_NAME {
        buf + = self . _create_gnu_long_header ( info [ "name" ] , GNUTYPE_LONGNAME , encoding , errors );
        return  buf + self . _create_header ( info , GNU_FORMAT , encoding , errors );
        pub fn create_pax_header ( &self, info , encoding )  {
        "Return the object as a ustar header block. If it cannot be
           represented this way, prepend a pax extended header sequence
           with supplement information.
        ";
        info [ "magic" ] = POSIX_MAGIC;
        pax_headers = self . pax_headers . copy ( );
        for name , hname , length in (.iter() {
        ( "name" , "path" , LENGTH_NAME ) , ( "linkname" , "linkpath" , LENGTH_LINK ) ,;
        ( "uname" , "uname" , 32 ) , ( "gname" , "gname" , 32 ) ) ;
        if hname in pax_headers {
        continue;
        // try {
        info [ name ] . encode ( "ascii" , "strict" );
        // } catch  UnicodeEncodeError  {
        pax_headers [ hname ] = info [ name ];
        continue;
        if len ( info [ name ] ) > length {
        pax_headers [ hname ] = info [ name ];
        for name , digits in ( ( "uid" , 8 ) , ( "gid" , 8 ) , ( "size" , 12 ) , ( "mtime" , 12 ) ) .iter() {
        needs_pax = false;
        val = info [ name ];
        val_is_float = isinstance ( val , float );
        val_int = round ( val ) if val_is_float else val;
        if !0 <= val_int < 8 ** ( digits - 1 ) {
        info [ name ] = 0;
        needs_pax = true;
        } else if val_is_float {
        info [ name ] = val_int;
        needs_pax = true;
        if needs_pax && name !in pax_headers {
        pax_headers [ name ] = str ( val );
        if pax_headers {
        buf = self . _create_pax_generic_header ( pax_headers , XHDTYPE , encoding );
        } else {
        buf = b "";
        return  buf + self . _create_header ( info , USTAR_FORMAT , "ascii" , "replace" );
        @ classmethod;
        pub fn create_pax_global_header ( cls , pax_headers )  {
        "Return the object as a pax global header block sequence.
        ";
        return  cls . _create_pax_generic_header ( pax_headers , XGLTYPE , "utf-8" );
        pub fn _posix_split_name ( &self, name , encoding , errors )  {
        "Split a name longer than 100 chars into a prefix
           && a name part.
        ";
        components = name . split ( "/" );
        for i in range ( 1 , len ( components ) ) .iter() {
        prefix = "/" . join ( components [ : i ] );
        name = "/" . join ( components [ i : ] );
        if len ( prefix . encode ( encoding , errors ) ) <= LENGTH_PREFIX && \ {
        len ( name . encode ( encoding , errors ) ) <= LENGTH_NAME ;
        break;
        } else {
        panic!("ValueError ( "name is too long" )");
        return  prefix , name;
        @ staticmethod;
        pub fn _create_header ( info , format , encoding , errors )  {
        "Return a header block. info == a dictionary with file
           information, format must be one of the *_FORMAT constants.
        ";
        has_device_fields = info . get ( "type" ) in ( CHRTYPE , BLKTYPE );
        if has_device_fields {
        devmajor = itn ( info . get ( "devmajor" , 0 ) , 8 , format );
        devminor = itn ( info . get ( "devminor" , 0 ) , 8 , format );
        } else {
        devmajor = stn ( "" , 8 , encoding , errors );
        devminor = stn ( "" , 8 , encoding , errors );
        filetype = info . get ( "type" , REGTYPE );
        if filetype is None /* Option */ {
        panic!("ValueError ( "TarInfo.type must !be None /* Option */" )");
        parts = [;
        stn ( info . get ( "name" , "" ) , 100 , encoding , errors ) ,;
        itn ( info . get ( "mode" , 0 ) & 0 o7777 , 8 , format ) ,;
        itn ( info . get ( "uid" , 0 ) , 8 , format ) ,;
        itn ( info . get ( "gid" , 0 ) , 8 , format ) ,;
        itn ( info . get ( "size" , 0 ) , 12 , format ) ,;
        itn ( info . get ( "mtime" , 0 ) , 12 , format ) ,;
        b "        " ,;
        filetype ,;
        stn ( info . get ( "linkname" , "" ) , 100 , encoding , errors ) ,;
        info . get ( "magic" , POSIX_MAGIC ) ,;
        stn ( info . get ( "uname" , "" ) , 32 , encoding , errors ) ,;
        stn ( info . get ( "gname" , "" ) , 32 , encoding , errors ) ,;
        devmajor ,;
        devminor ,;
        stn ( info . get ( "prefix" , "" ) , 155 , encoding , errors );
        ];
        buf = struct . pack ( "%ds" % BLOCKSIZE , b "" . join ( parts ) );
        chksum = calc_chksums ( buf [ - BLOCKSIZE : ] ) [ 0 ];
        buf = buf [ : -364 ] + bytes ( "%06o\0" % chksum , "ascii" ) + buf [ -357 : ];
        return  buf;
        @ staticmethod;
        pub fn _create_payload ( payload )  {
        "Return the string payload filled with zero bytes
           up to the next 512 byte border.
        ";
        blocks , remainder = divmod ( len ( payload ) , BLOCKSIZE );
        if remainder > 0 {
        payload + = ( BLOCKSIZE - remainder ) * NUL;
        return  payload;
        @ classmethod;
        pub fn _create_gnu_long_header ( cls , name , type , encoding , errors )  {
        "Return a GNUTYPE_LONGNAME || GNUTYPE_LONGLINK sequence
           for name.
        ";
        name = name . encode ( encoding , errors ) + NUL;
        info = { };
        info [ "name" ] = "././@LongLink";
        info [ "type" ] = type;
        info [ "size" ] = len ( name );
        info [ "magic" ] = GNU_MAGIC;
        return  cls . _create_header ( info , USTAR_FORMAT , encoding , errors ) + \;
        cls . _create_payload ( name );
        @ classmethod;
        pub fn _create_pax_generic_header ( cls , pax_headers , type , encoding )  {
        "Return a POSIX.1-2008 extended || global header sequence
           that contains a list of keyword, value pairs. The values
           must be strings.
        ";
        binary = false;
        for keyword , value in pax_headers . items ( ) .iter() {
        // try {
        value . encode ( "utf-8" , "strict" );
        // } catch  UnicodeEncodeError  {
        binary = true;
        break;
        records = b "";
        if binary {
        records + = b "21 hdrcharset=BINARY\n";
        for keyword , value in pax_headers . items ( ) .iter() {
        keyword = keyword . encode ( "utf-8" );
        if binary {
        value = value . encode ( encoding , "surrogateescape" );
        } else {
        value = value . encode ( "utf-8" );
        l = len ( keyword ) + len ( value ) + 3;
        n = p = 0;
        while true  {
        n = l + len ( str ( p ) );
        if n == p {
        break;
        p = n;
        records + = bytes ( str ( p ) , "ascii" ) + b " " + keyword + b "=" + value + b "\n";
        info = { };
        info [ "name" ] = "././@PaxHeader";
        info [ "type" ] = type;
        info [ "size" ] = len ( records );
        info [ "magic" ] = POSIX_MAGIC;
        return  cls . _create_header ( info , USTAR_FORMAT , "ascii" , "replace" ) + \;
        cls . _create_payload ( records );
        @ classmethod;
        pub fn frombuf ( cls , buf , encoding , errors )  {
        "Construct a TarInfo object from a 512 byte bytes object.
        ";
        if len ( buf ) == 0 {
        panic!("EmptyHeaderError ( "empty header" )");
        if len ( buf ) != BLOCKSIZE {
        panic!("TruncatedHeaderError ( "truncated header" )");
        if buf . count ( NUL ) == BLOCKSIZE {
        panic!("EOFHeaderError ( "end of file header" )");
        chksum = nti ( buf [ 148 : 156 ] );
        if chksum !in calc_chksums ( buf ) {
        panic!("InvalidHeaderError ( "bad checksum" )");
        obj = cls ( );
        obj . name = nts ( buf [ 0 : 100 ] , encoding , errors );
        obj . mode = nti ( buf [ 100 : 108 ] );
        obj . uid = nti ( buf [ 108 : 116 ] );
        obj . gid = nti ( buf [ 116 : 124 ] );
        obj . size = nti ( buf [ 124 : 136 ] );
        obj . mtime = nti ( buf [ 136 : 148 ] );
        obj . chksum = chksum;
        obj . type = buf [ 156 : 157 ];
        obj . linkname = nts ( buf [ 157 : 257 ] , encoding , errors );
        obj . uname = nts ( buf [ 265 : 297 ] , encoding , errors );
        obj . gname = nts ( buf [ 297 : 329 ] , encoding , errors );
        obj . devmajor = nti ( buf [ 329 : 337 ] );
        obj . devminor = nti ( buf [ 337 : 345 ] );
        prefix = nts ( buf [ 345 : 500 ] , encoding , errors );
        if obj . type == AREGTYPE && obj . name . endswith ( "/" ) {
        obj . type = DIRTYPE;
        if obj . type == GNUTYPE_SPARSE {
        pos = 386;
        structs = [ ];
        for i in range ( 4 ) .iter() {
        // try {
        offset = nti ( buf [ pos : pos + 12 ] );
        numbytes = nti ( buf [ pos + 12 : pos + 24 ] );
        // } catch  ValueError  {
        break;
        structs . append ( ( offset , numbytes ) );
        pos + = 24;
        isextended = bool ( buf [ 482 ] );
        origsize = nti ( buf [ 483 : 495 ] );
        obj . _sparse_structs = ( structs , isextended , origsize );
        if obj . isdir ( ) {
        obj . name = obj . name . rstrip ( "/" );
        if prefix && obj . type !in GNU_TYPES {
        obj . name = prefix + "/" + obj . name;
        return  obj;
        @ classmethod;
        pub fn fromtarfile ( cls , tarfile )  {
        "Return the next TarInfo object from TarFile object
           tarfile.
        ";
        buf = tarfile . fileobj . read ( BLOCKSIZE );
        obj = cls . frombuf ( buf , tarfile . encoding , tarfile . errors );
        obj . offset = tarfile . fileobj . tell ( ) - BLOCKSIZE;
        return  obj . _proc_member ( tarfile );
        pub fn _proc_member ( &self, tarfile )  {
        "Choose the right processing method depending on
           the type && call it.
        ";
        if self . type in ( GNUTYPE_LONGNAME , GNUTYPE_LONGLINK ) {
        return  self . _proc_gnulong ( tarfile );
        } else if self . type == GNUTYPE_SPARSE {
        return  self . _proc_sparse ( tarfile );
        } else if self . type in ( XHDTYPE , XGLTYPE , SOLARIS_XHDTYPE ) {
        return  self . _proc_pax ( tarfile );
        } else {
        return  self . _proc_builtin ( tarfile );
        pub fn _proc_builtin ( &self, tarfile )  {
        "Process a builtin type || an unknown type which
           will be treated as a regular file.
        ";
        self . offset_data = tarfile . fileobj . tell ( );
        offset = self . offset_data;
        if self . isreg ( ) || self . type !in SUPPORTED_TYPES {
        offset + = self . _block ( self . size );
        tarfile . offset = offset;
        self . _apply_pax_info ( tarfile . pax_headers , tarfile . encoding , tarfile . errors );
        if self . isdir ( ) {
        self . name = self . name . rstrip ( "/" );
        return  self;
        pub fn _proc_gnulong ( &self, tarfile )  {
        "Process the blocks that hold a GNU longname
           || longlink member.
        ";
        buf = tarfile . fileobj . read ( self . _block ( self . size ) );
        // try {
        next = self . fromtarfile ( tarfile );
        // } catch  HeaderError as e  {
        panic!("SubsequentHeaderError ( str ( e ) ) from None /* Option */");
        next . offset = self . offset;
        if self . type == GNUTYPE_LONGNAME {
        next . name = nts ( buf , tarfile . encoding , tarfile . errors );
        } else if self . type == GNUTYPE_LONGLINK {
        next . linkname = nts ( buf , tarfile . encoding , tarfile . errors );
        if next . isdir ( ) {
        next . name = next . name . removesuffix ( "/" );
        return  next;
        pub fn _proc_sparse ( &self, tarfile )  {
        "Process a GNU sparse header plus extra headers.
        ";
        structs , isextended , origsize = self . _sparse_structs;
        del self . _sparse_structs;
        while isextended  {
        buf = tarfile . fileobj . read ( BLOCKSIZE );
        pos = 0;
        for i in range ( 21 ) .iter() {
        // try {
        offset = nti ( buf [ pos : pos + 12 ] );
        numbytes = nti ( buf [ pos + 12 : pos + 24 ] );
        // } catch  ValueError  {
        break;
        if offset && numbytes {
        structs . append ( ( offset , numbytes ) );
        pos + = 24;
        isextended = bool ( buf [ 504 ] );
        self . sparse = structs;
        self . offset_data = tarfile . fileobj . tell ( );
        tarfile . offset = self . offset_data + self . _block ( self . size );
        self . size = origsize;
        return  self;
        pub fn _proc_pax ( &self, tarfile )  {
        "Process an extended || global header as described in
           POSIX.1-2008.
        ";
        buf = tarfile . fileobj . read ( self . _block ( self . size ) );
        if self . type == XGLTYPE {
        pax_headers = tarfile . pax_headers;
        } else {
        pax_headers = tarfile . pax_headers . copy ( );
        match = re . search ( br "\d+ hdrcharset=([^\n]+)\n" , buf );
        if match is !None /* Option */ {
        pax_headers [ "hdrcharset" ] = match . group ( 1 ) . decode ( "utf-8" );
        hdrcharset = pax_headers . get ( "hdrcharset" );
        if hdrcharset == "BINARY" {
        encoding = tarfile . encoding;
        } else {
        encoding = "utf-8";
        regex = re . compile ( br "(\d+) ([^=]+)=" );
        pos = 0;
        while true  {
        match = regex . match ( buf , pos );
        if !match {
        break;
        length , keyword = match . groups ( );
        length = int ( length );
        if length == 0 {
        panic!("InvalidHeaderError ( "invalid header" )");
        value = buf [ match . end ( 2 ) + 1 : match . start ( 1 ) + length - 1 ];
        keyword = self . _decode_pax_field ( keyword , "utf-8" , "utf-8" ,;
        tarfile . errors );
        if keyword in PAX_NAME_FIELDS {
        value = self . _decode_pax_field ( value , encoding , tarfile . encoding ,;
        tarfile . errors );
        } else {
        value = self . _decode_pax_field ( value , "utf-8" , "utf-8" ,;
        tarfile . errors );
        pax_headers [ keyword ] = value;
        pos + = length;
        // try {
        next = self . fromtarfile ( tarfile );
        // } catch  HeaderError as e  {
        panic!("SubsequentHeaderError ( str ( e ) ) from None /* Option */");
        if "GNU.sparse.map" in pax_headers {
        self . _proc_gnusparse_01 ( next , pax_headers );
        } else if "GNU.sparse.size" in pax_headers {
        self . _proc_gnusparse_00 ( next , pax_headers , buf );
        } else if pax_headers . get ( "GNU.sparse.major" ) == "1" && pax_headers . get ( "GNU.sparse.minor" ) == "0" {
        self . _proc_gnusparse_10 ( next , pax_headers , tarfile );
        if self . type in ( XHDTYPE , SOLARIS_XHDTYPE ) {
        next . _apply_pax_info ( pax_headers , tarfile . encoding , tarfile . errors );
        next . offset = self . offset;
        if "size" in pax_headers {
        offset = next . offset_data;
        if next . isreg ( ) || next . type !in SUPPORTED_TYPES {
        offset + = next . _block ( next . size );
        tarfile . offset = offset;
        return  next;
        pub fn _proc_gnusparse_00 ( &self, next , pax_headers , buf )  {
        "Process a GNU tar extended sparse header, version 0.0.
        ";
        offsets = [ ];
        for match in re . finditer ( br "\d+ GNU.sparse.offset=(\d+)\n" , buf ) .iter() {
        offsets . append ( int ( match . group ( 1 ) ) );
        numbytes = [ ];
        for match in re . finditer ( br "\d+ GNU.sparse.numbytes=(\d+)\n" , buf ) .iter() {
        numbytes . append ( int ( match . group ( 1 ) ) );
        next . sparse = list ( zip ( offsets , numbytes ) );
        pub fn _proc_gnusparse_01 ( &self, next , pax_headers )  {
        "Process a GNU tar extended sparse header, version 0.1.
        ";
        sparse = vec![ int ( x ).iter().map(|x| pax_headers vec![ "GNU.sparse.map" ] . split ( "," ) ).collect();
        next . sparse = list ( zip ( sparse [ : : 2 ] , sparse [ 1 : : 2 ] ) );
        pub fn _proc_gnusparse_10 ( &self, next , pax_headers , tarfile )  {
        "Process a GNU tar extended sparse header, version 1.0.
        ";
        fields = None /* Option */;
        sparse = [ ];
        buf = tarfile . fileobj . read ( BLOCKSIZE );
        fields , buf = buf . split ( b "\n" , 1 );
        fields = int ( fields );
        while len ( sparse ) < fields * 2  {
        if b "\n" !in buf {
        buf + = tarfile . fileobj . read ( BLOCKSIZE );
        number , buf = buf . split ( b "\n" , 1 );
        sparse . append ( int ( number ) );
        next . offset_data = tarfile . fileobj . tell ( );
        next . sparse = list ( zip ( sparse [ : : 2 ] , sparse [ 1 : : 2 ] ) );
        pub fn _apply_pax_info ( &self, pax_headers , encoding , errors )  {
        "Replace fields with supplemental information from a previous
           pax extended || global header.
        ";
        for keyword , value in pax_headers . items ( ) .iter() {
        if keyword == "GNU.sparse.name" {
        setattr ( self , "path" , value );
        } else if keyword == "GNU.sparse.size" {
        setattr ( self , "size" , int ( value ) );
        } else if keyword == "GNU.sparse.realsize" {
        setattr ( self , "size" , int ( value ) );
        } else if keyword in PAX_FIELDS {
        if keyword in PAX_NUMBER_FIELDS {
        // try {
        value = PAX_NUMBER_FIELDS [ keyword ] ( value );
        // } catch  ValueError  {
        value = 0;
        if keyword == "path" {
        value = value . rstrip ( "/" );
        setattr ( self , keyword , value );
        self . pax_headers = pax_headers . copy ( );
        pub fn _decode_pax_field ( &self, value , encoding , fallback_encoding , fallback_errors )  {
        "Decode a single field from a pax record.
        ";
        // try {
        return  value . decode ( encoding , "strict" );
        // } catch  UnicodeDecodeError  {
        return  value . decode ( fallback_encoding , fallback_errors );
        pub fn _block ( &self, count )  {
        "Round up a byte count by BLOCKSIZE && return it,
           e.g. _block(834) => 1024.
        ";
        blocks , remainder = divmod ( count , BLOCKSIZE );
        if remainder {
        blocks + = 1;
        return  blocks * BLOCKSIZE;
        pub fn isreg ( self )  {
        "Return true if the Tarinfo object == a regular file.";
        return  self . type in REGULAR_TYPES;
        pub fn isfile ( self )  {
        "Return true if the Tarinfo object == a regular file.";
        return  self . isreg ( );
        pub fn isdir ( self )  {
        "Return true if it == a directory.";
        return  self . type == DIRTYPE;
        pub fn issym ( self )  {
        "Return true if it == a symbolic link.";
        return  self . type == SYMTYPE;
        pub fn islnk ( self )  {
        "Return true if it == a hard link.";
        return  self . type == LNKTYPE;
        pub fn ischr ( self )  {
        "Return true if it == a character device.";
        return  self . type == CHRTYPE;
        pub fn isblk ( self )  {
        "Return true if it == a block device.";
        return  self . type == BLKTYPE;
        pub fn isfifo ( self )  {
        "Return true if it == a FIFO.";
        return  self . type == FIFOTYPE;
        pub fn issparse ( self )  {
        return  self . sparse is !None /* Option */;
        pub fn isdev ( self )  {
        "Return true if it == one of character device, block device || FIFO.";
        return  self . type in ( CHRTYPE , BLKTYPE , FIFOTYPE );
        class TarFile ( object ) ;
        "The TarFile Class provides an interface to tar archives.
    ";
        debug = 0;
        dereference = false;
        ignore_zeros = false;
        errorlevel = 1;
        format = DEFAULT_FORMAT;
        encoding = ENCODING;
        errors = None /* Option */;
        tarinfo = TarInfo;
        fileobject = ExFileObject;
        extraction_filter = None /* Option */;
        pub fn __init__ ( &self, name = None /* Option */ , mode = "r" , fileobj = None /* Option */ , format = None /* Option */ , {
        tarinfo = None /* Option */ , dereference = None /* Option */ , ignore_zeros = None /* Option */ , encoding = None /* Option */ ,;
        errors = "surrogateescape" , pax_headers = None /* Option */ , debug = None /* Option */ ,;
        errorlevel = None /* Option */ , copybufsize = None /* Option */ ) ;
        "Open an (uncompressed) tar archive `name'. `mode' == either 'r' to
           read from an existing archive, 'a' to append data to an existing
           file || 'w' to create a new file overwriting an existing one. `mode'
           defaults to 'r'.
           If `fileobj' == given, it == used for reading || writing data. If it
           can be determined, `mode' == overridden by `fileobj's mode.
           `fileobj' == !closed, when TarFile == closed.
        ";
        modes = { "r" : "rb" , "a" : "r+b" , "w" : "wb" , "x" : "xb" };
        if mode !in modes {
        panic!("ValueError ( "mode must be 'r', 'a', 'w' || 'x'" )");
        self . mode = mode;
        self . _mode = modes [ mode ];
        if !fileobj {
        if self . mode == "a" && !os . path . exists ( name ) {
        self . mode = "w";
        self . _mode = "wb";
        fileobj = bltn_open ( name , self . _mode );
        self . _extfileobj = false;
        } else {
        if ( name is None /* Option */ && hasattr ( fileobj , "name" ) and {
        isinstance ( fileobj . name , ( str , bytes ) ) ) ;
        name = fileobj . name;
        if hasattr ( fileobj , "mode" ) {
        self . _mode = fileobj . mode;
        self . _extfileobj = true;
        self . name = os . path . abspath ( name ) if name else None /* Option */;
        self . fileobj = fileobj;
        if format is !None /* Option */ {
        self . format = format;
        if tarinfo is !None /* Option */ {
        self . tarinfo = tarinfo;
        if dereference is !None /* Option */ {
        self . dereference = dereference;
        if ignore_zeros is !None /* Option */ {
        self . ignore_zeros = ignore_zeros;
        if encoding is !None /* Option */ {
        self . encoding = encoding;
        self . errors = errors;
        if pax_headers is !None /* Option */ && self . format == PAX_FORMAT {
        self . pax_headers = pax_headers;
        } else {
        self . pax_headers = { };
        if debug is !None /* Option */ {
        self . debug = debug;
        if errorlevel is !None /* Option */ {
        self . errorlevel = errorlevel;
        self . copybufsize = copybufsize;
        self . closed = false;
        self . members = [ ];
        self . _loaded = false;
        self . offset = self . fileobj . tell ( );
        self . inodes = { };
        // try {
        if self . mode == "r" {
        self . firstmember = None /* Option */;
        self . firstmember = self . next ( );
        if self . mode == "a" {
        while true  {
        self . fileobj . seek ( self . offset );
        // try {
        tarinfo = self . tarinfo . fromtarfile ( self );
        self . members . append ( tarinfo );
        // } catch  EOFHeaderError  {
        self . fileobj . seek ( self . offset );
        break;
        // } catch  HeaderError as e  {
        panic!("ReadError ( str ( e ) ) from None /* Option */");
        if self . mode in ( "a" , "w" , "x" ) {
        self . _loaded = true;
        if self . pax_headers {
        buf = self . tarinfo . create_pax_global_header ( self . pax_headers . copy ( ) );
        self . fileobj . write ( buf );
        self . offset + = len ( buf );
        // } catch   {
        if !self . _extfileobj {
        self . fileobj . close ( );
        self . closed = true;
        panic!("");
        @ classmethod;
        pub fn open ( cls , name = None /* Option */ , mode = "r" , fileobj = None /* Option */ , bufsize = RECORDSIZE , ** kwargs )  {
        "Open a tar archive for reading, writing || appending. Return
           an appropriate TarFile class.

           mode:
           'r' || 'r:*' open for reading with transparent compression
           'r:'         open for reading exclusively uncompressed
           'r:gz'       open for reading with gzip compression
           'r:bz2'      open for reading with bzip2 compression
           'r:xz'       open for reading with lzma compression
           'a' || 'a:'  open for appending, creating the file if necessary
           'w' || 'w:'  open for writing without compression
           'w:gz'       open for writing with gzip compression
           'w:bz2'      open for writing with bzip2 compression
           'w:xz'       open for writing with lzma compression

           'x' || 'x:'  create a tarfile exclusively without compression, raise
                        an exception if the file == already created
           'x:gz'       create a gzip compressed tarfile, raise an exception
                        if the file == already created
           'x:bz2'      create a bzip2 compressed tarfile, raise an exception
                        if the file == already created
           'x:xz'       create an lzma compressed tarfile, raise an exception
                        if the file == already created

           'r|*'        open a stream of tar blocks with transparent compression
           'r|'         open an uncompressed stream of tar blocks for reading
           'r|gz'       open a gzip compressed stream of tar blocks
           'r|bz2'      open a bzip2 compressed stream of tar blocks
           'r|xz'       open an lzma compressed stream of tar blocks
           'w|'         open an uncompressed stream for writing
           'w|gz'       open a gzip compressed stream for writing
           'w|bz2'      open a bzip2 compressed stream for writing
           'w|xz'       open an lzma compressed stream for writing
        ";
        if !name && !fileobj {
        panic!("ValueError ( "nothing to open" )");
        if mode in ( "r" , "r:*" ) {
        pub fn not_compressed ( comptype )  {
        return  cls . OPEN_METH [ comptype ] == "taropen";
        error_msgs = [ ];
        for comptype in sorted ( cls . OPEN_METH , key = not_compressed ) .iter() {
        func = getattr ( cls , cls . OPEN_METH [ comptype ] );
        if fileobj is !None /* Option */ {
        saved_pos = fileobj . tell ( );
        // try {
        return  func ( name , "r" , fileobj , ** kwargs );
        // } catch  ( ReadError , CompressionError ) as e  {
        error_msgs . append ( format!("- method {comptype}: {e!r}" ));
        if fileobj is !None /* Option */ {
        fileobj . seek ( saved_pos );
        continue;
        error_msgs_summary = "\n" . join ( error_msgs );
        panic!("ReadError ( f "file could !be opened successfully:\n{error_msgs_summary}" )");
        } else if ":" in mode {
        filemode , comptype = mode . split ( ":" , 1 );
        filemode = filemode || "r";
        comptype = comptype || "tar";
        if comptype in cls . OPEN_METH {
        func = getattr ( cls , cls . OPEN_METH [ comptype ] );
        } else {
        panic!("CompressionError ( "unknown compression type %r" % comptype )");
        return  func ( name , filemode , fileobj , ** kwargs );
        } else if "|" in mode {
        filemode , comptype = mode . split ( "|" , 1 );
        filemode = filemode || "r";
        comptype = comptype || "tar";
        if filemode !in ( "r" , "w" ) {
        panic!("ValueError ( "mode must be 'r' || 'w'" )");
        stream = _Stream ( name , filemode , comptype , fileobj , bufsize );
        // try {
        t = cls ( name , filemode , stream , ** kwargs );
        // } catch   {
        stream . close ( );
        panic!("");
        t . _extfileobj = false;
        return  t;
        } else if mode in ( "a" , "w" , "x" ) {
        return  cls . taropen ( name , mode , fileobj , ** kwargs );
        panic!("ValueError ( "undiscernible mode" )");
        @ classmethod;
        pub fn taropen ( cls , name , mode = "r" , fileobj = None /* Option */ , ** kwargs )  {
        "Open uncompressed tar archive name for reading || writing.
        ";
        if mode !in ( "r" , "a" , "w" , "x" ) {
        panic!("ValueError ( "mode must be 'r', 'a', 'w' || 'x'" )");
        return  cls ( name , mode , fileobj , ** kwargs );
        @ classmethod;
        pub fn gzopen ( cls , name , mode = "r" , fileobj = None /* Option */ , compresslevel = 9 , ** kwargs )  {
        "Open gzip compressed tar archive name for reading || writing.
           Appending == !allowed.
        ";
        if mode !in ( "r" , "w" , "x" ) {
        panic!("ValueError ( "mode must be 'r', 'w' || 'x'" )");
        // try {
        from gzip import GzipFile;
        // } catch  ImportError  {
        panic!("CompressionError ( "gzip module is !available" ) from None /* Option */");
        // try {
        fileobj = GzipFile ( name , mode + "b" , compresslevel , fileobj );
        // } catch  OSError as e  {
        if fileobj is !None /* Option */ && mode == "r" {
        panic!("ReadError ( "not a gzip file" ) from e");
        panic!("");
        // try {
        t = cls . taropen ( name , mode , fileobj , ** kwargs );
        // } catch  OSError as e  {
        fileobj . close ( );
        if mode == "r" {
        panic!("ReadError ( "not a gzip file" ) from e");
        panic!("");
        // } catch   {
        fileobj . close ( );
        panic!("");
        t . _extfileobj = false;
        return  t;
        @ classmethod;
        pub fn bz2open ( cls , name , mode = "r" , fileobj = None /* Option */ , compresslevel = 9 , ** kwargs )  {
        "Open bzip2 compressed tar archive name for reading || writing.
           Appending == !allowed.
        ";
        if mode !in ( "r" , "w" , "x" ) {
        panic!("ValueError ( "mode must be 'r', 'w' || 'x'" )");
        // try {
        from bz2 import BZ2File;
        // } catch  ImportError  {
        panic!("CompressionError ( "bz2 module is !available" ) from None /* Option */");
        fileobj = BZ2File ( fileobj || name , mode , compresslevel = compresslevel );
        // try {
        t = cls . taropen ( name , mode , fileobj , ** kwargs );
        // } catch  ( OSError , EOFError ) as e  {
        fileobj . close ( );
        if mode == "r" {
        panic!("ReadError ( "not a bzip2 file" ) from e");
        panic!("");
        // } catch   {
        fileobj . close ( );
        panic!("");
        t . _extfileobj = false;
        return  t;
        @ classmethod;
        pub fn xzopen ( cls , name , mode = "r" , fileobj = None /* Option */ , preset = None /* Option */ , ** kwargs )  {
        "Open lzma compressed tar archive name for reading || writing.
           Appending == !allowed.
        ";
        if mode !in ( "r" , "w" , "x" ) {
        panic!("ValueError ( "mode must be 'r', 'w' || 'x'" )");
        // try {
        from lzma import LZMAFile , LZMAError;
        // } catch  ImportError  {
        panic!("CompressionError ( "lzma module is !available" ) from None /* Option */");
        fileobj = LZMAFile ( fileobj || name , mode , preset = preset );
        // try {
        t = cls . taropen ( name , mode , fileobj , ** kwargs );
        // } catch  ( LZMAError , EOFError ) as e  {
        fileobj . close ( );
        if mode == "r" {
        panic!("ReadError ( "not an lzma file" ) from e");
        panic!("");
        // } catch   {
        fileobj . close ( );
        panic!("");
        t . _extfileobj = false;
        return  t;
        OPEN_METH = {;
        "tar" : "taropen" ,;
        "gz" : "gzopen" ,;
        "bz2" : "bz2open" ,;
        "xz" : "xzopen";
        };
        pub fn close ( self )  {
        "Close the TarFile. In write-mode, two finishing zero blocks are
           appended to the archive.
        ";
        if self . closed {
        return;
        self . closed = true;
        // try {
        if self . mode in ( "a" , "w" , "x" ) {
        self . fileobj . write ( NUL * ( BLOCKSIZE * 2 ) );
        self . offset + = ( BLOCKSIZE * 2 );
        blocks , remainder = divmod ( self . offset , RECORDSIZE );
        if remainder > 0 {
        self . fileobj . write ( NUL * ( RECORDSIZE - remainder ) );
        // } finally {
        if !self . _extfileobj {
        self . fileobj . close ( );
        pub fn getmember ( &self, name )  {
        "Return a TarInfo object for member `name'. If `name' can !be
           found in the archive, KeyError == raised. If a member occurs more
           than once in the archive, its last occurrence == assumed to be the
           most up-to-date version.
        ";
        tarinfo = self . _getmember ( name . rstrip ( "/" ) );
        if tarinfo is None /* Option */ {
        panic!("KeyError ( "filename %r !found" % name )");
        return  tarinfo;
        pub fn getmembers ( self )  {
        "Return the members of the archive as a list of TarInfo objects. The
           list has the same order as the members in the archive.
        ";
        self . _check ( );
        if !self . _loaded {
        self . _load ( );
        return  self . members;
        pub fn getnames ( self )  {
        "Return the members of the archive as a list of their names. It has
           the same order as the list returned by getmembers().
        ";
        return  [ tarinfo . name for tarinfo in self . getmembers ( ) ];
        pub fn gettarinfo ( &self, name = None /* Option */ , arcname = None /* Option */ , fileobj = None /* Option */ )  {
        "Create a TarInfo object from the result of os.stat || equivalent
           on an existing file. The file == either named by `name', or
           specified as a file object `fileobj' with a file descriptor. If
           given, `arcname' specifies an alternative name for the file in the
           archive, otherwise, the name == taken from the 'name' attribute of
           'fileobj', || the 'name' argument. The name should be a text
           string.
        ";
        self . _check ( "awx" );
        if fileobj is !None /* Option */ {
        name = fileobj . name;
        if arcname is None /* Option */ {
        arcname = name;
        drv , arcname = os . path . splitdrive ( arcname );
        arcname = arcname . replace ( os . sep , "/" );
        arcname = arcname . lstrip ( "/" );
        tarinfo = self . tarinfo ( );
        tarinfo . tarfile = self;
        if fileobj is None /* Option */ {
        if !self . dereference {
        statres = os . lstat ( name );
        } else {
        statres = os . stat ( name );
        } else {
        statres = os . fstat ( fileobj . fileno ( ) );
        linkname = "";
        stmd = statres . st_mode;
        if stat . S_ISREG ( stmd ) {
        inode = ( statres . st_ino , statres . st_dev );
        if !self . dereference && statres . st_nlink > 1 && \ {
        inode in self . inodes && arcname != self . inodes [ inode ] ;
        type = LNKTYPE;
        linkname = self . inodes [ inode ];
        } else {
        type = REGTYPE;
        if inode [ 0 ] {
        self . inodes [ inode ] = arcname;
        } else if stat . S_ISDIR ( stmd ) {
        type = DIRTYPE;
        } else if stat . S_ISFIFO ( stmd ) {
        type = FIFOTYPE;
        } else if stat . S_ISLNK ( stmd ) {
        type = SYMTYPE;
        linkname = os . readlink ( name );
        } else if stat . S_ISCHR ( stmd ) {
        type = CHRTYPE;
        } else if stat . S_ISBLK ( stmd ) {
        type = BLKTYPE;
        } else {
        return;
        tarinfo . name = arcname;
        tarinfo . mode = stmd;
        tarinfo . uid = statres . st_uid;
        tarinfo . gid = statres . st_gid;
        if type == REGTYPE {
        tarinfo . size = statres . st_size;
        } else {
        tarinfo . size = 0;
        tarinfo . mtime = statres . st_mtime;
        tarinfo . type = type;
        tarinfo . linkname = linkname;
        if pwd {
        // try {
        tarinfo . uname = pwd . getpwuid ( tarinfo . uid ) [ 0 ];
        // } catch  KeyError  {
        // pass
        if grp {
        // try {
        tarinfo . gname = grp . getgrgid ( tarinfo . gid ) [ 0 ];
        // } catch  KeyError  {
        // pass
        if type in ( CHRTYPE , BLKTYPE ) {
        if hasattr ( os , "major" ) && hasattr ( os , "minor" ) {
        tarinfo . devmajor = os . major ( statres . st_rdev );
        tarinfo . devminor = os . minor ( statres . st_rdev );
        return  tarinfo;
        pub fn list ( &self, verbose = true , * , members = None /* Option */ )  {
        "Print a table of contents to sys.stdout. If `verbose' == false, only
           the names of the members are printed. If it == true, an `ls -l'-like
           output == produced. `members' == optional && must be a subset of the
           list returned by getmembers().
        ";
        self . _check ( );
        if members is None /* Option */ {
        members = self;
        for tarinfo in members .iter() {
        if verbose {
        if tarinfo . mode is None /* Option */ {
        _safe_print ( "??????????" );
        } else {
        _safe_print ( stat . filemode ( tarinfo . mode ) );
        _safe_print ( "%s/%s" % ( tarinfo . uname || tarinfo . uid ,;
        tarinfo . gname || tarinfo . gid ) );
        if tarinfo . ischr ( ) || tarinfo . isblk ( ) {
        _safe_print ( "%10s" %;
        ( "%d,%d" % ( tarinfo . devmajor , tarinfo . devminor ) ) );
        } else {
        _safe_print ( "%10d" % tarinfo . size );
        if tarinfo . mtime is None /* Option */ {
        _safe_print ( "????-??-?? ??:??:??" );
        } else {
        _safe_print ( "%d-%02d-%02d %02d:%02d:%02d" \;
        % time . localtime ( tarinfo . mtime ) [ : 6 ] );
        _safe_print ( tarinfo . name + ( "/" if tarinfo . isdir ( ) else "" ) );
        if verbose {
        if tarinfo . issym ( ) {
        _safe_print ( "-> " + tarinfo . linkname );
        if tarinfo . islnk ( ) {
        _safe_print ( "link to " + tarinfo . linkname );
        println!( );
        pub fn add ( &self, name , arcname = None /* Option */ , recursive = true , * , filter = None /* Option */ )  {
        "Add the file `name' to the archive. `name' may be any type of file
           (directory, fifo, symbolic link, etc.). If given, `arcname'
           specifies an alternative name for the file in the archive.
           Directories are added recursively by default. This can be avoided by
           setting `recursive' to false. `filter' == a function
           that expects a TarInfo object argument && returns the changed
           TarInfo object, if it returns None /* Option */ the TarInfo object will be
           excluded from the archive.
        ";
        self . _check ( "awx" );
        if arcname is None /* Option */ {
        arcname = name;
        if self . name is !None /* Option */ && os . path . abspath ( name ) == self . name {
        self . _dbg ( 2 , "tarfile: Skipped %r" % name );
        return;
        self . _dbg ( 1 , name );
        tarinfo = self . gettarinfo ( name , arcname );
        if tarinfo is None /* Option */ {
        self . _dbg ( 1 , "tarfile: Unsupported type %r" % name );
        return;
        if filter is !None /* Option */ {
        tarinfo = filter ( tarinfo );
        if tarinfo is None /* Option */ {
        self . _dbg ( 2 , "tarfile: Excluded %r" % name );
        return;
        if tarinfo . isreg ( ) {
        // with scope: bltn_open ( name , "rb" ) as f  {
        self . addfile ( tarinfo , f );
        } else if tarinfo . isdir ( ) {
        self . addfile ( tarinfo );
        if recursive {
        for f in sorted ( os . listdir ( name ) ) .iter() {
        self . add ( os . path . join ( name , f ) , os . path . join ( arcname , f ) ,;
        recursive , filter = filter );
        } else {
        self . addfile ( tarinfo );
        pub fn addfile ( &self, tarinfo , fileobj = None /* Option */ )  {
        "Add the TarInfo object `tarinfo' to the archive. If `fileobj' is
           given, it should be a binary file, && tarinfo.size bytes are read
           from it && added to the archive. You can create TarInfo objects
           directly, || by using gettarinfo().
        ";
        self . _check ( "awx" );
        tarinfo = copy . copy ( tarinfo );
        buf = tarinfo . tobuf ( self . format , self . encoding , self . errors );
        self . fileobj . write ( buf );
        self . offset + = len ( buf );
        bufsize = self . copybufsize;
        if fileobj is !None /* Option */ {
        copyfileobj ( fileobj , self . fileobj , tarinfo . size , bufsize = bufsize );
        blocks , remainder = divmod ( tarinfo . size , BLOCKSIZE );
        if remainder > 0 {
        self . fileobj . write ( NUL * ( BLOCKSIZE - remainder ) );
        blocks + = 1;
        self . offset + = blocks * BLOCKSIZE;
        self . members . append ( tarinfo );
        pub fn _get_filter_function ( &self, filter )  {
        if filter is None /* Option */ {
        filter = self . extraction_filter;
        if filter is None /* Option */ {
        return  fully_trusted_filter;
        if isinstance ( filter , str ) {
        panic!("TypeError (");
        "String names are !supported for ";
        + "TarFile.extraction_filter. Use a function such as ";
        + "tarfile.data_filter directly." );
        return  filter;
        if callable ( filter ) {
        return  filter;
        // try {
        return  _NAMED_FILTERS [ filter ];
        // } catch  KeyError  {
        panic!("ValueError ( f "filter {filter!r} !found" ) from None /* Option */");
        pub fn extractall ( &self, path = "." , members = None /* Option */ , * , numeric_owner = false , {
        filter = None /* Option */ ) ;
        "Extract all members from the archive to the current working
           directory && set owner, modification time && permissions on
           directories afterwards. `path' specifies a different directory
           to extract to. `members' == optional && must be a subset of the
           list returned by getmembers(). If `numeric_owner` == true, only
           the numbers for user/group names are used && !the names.

           The `filter` function will be called on each member just
           before extraction.
           It can return a changed TarInfo || None /* Option */ to skip the member.
           String names of common filters are accepted.
        ";
        directories = [ ];
        filter_function = self . _get_filter_function ( filter );
        if members is None /* Option */ {
        members = self;
        for member in members .iter() {
        tarinfo = self . _get_extract_tarinfo ( member , filter_function , path );
        if tarinfo is None /* Option */ {
        continue;
        if tarinfo . isdir ( ) {
        directories . append ( tarinfo );
        self . _extract_one ( tarinfo , path , set_attrs = !tarinfo . isdir ( ) ,;
        numeric_owner = numeric_owner );
        directories . sort ( key = |a | {  a . name , reverse = true ) };
        for tarinfo in directories .iter() {
        dirpath = os . path . join ( path , tarinfo . name );
        // try {
        self . chown ( tarinfo , dirpath , numeric_owner = numeric_owner );
        self . utime ( tarinfo , dirpath );
        self . chmod ( tarinfo , dirpath );
        // } catch  ExtractError as e  {
        self . _handle_nonfatal_error ( e );
        pub fn extract ( &self, member , path = "" , set_attrs = true , * , numeric_owner = false , {
        filter = None /* Option */ ) ;
        "Extract a member from the archive to the current working directory,
           using its full name. Its file information == extracted as accurately
           as possible. `member' may be a filename || a TarInfo object. You can
           specify a different directory using `path'. File attributes (owner,
           mtime, mode) are set unless `set_attrs' == false. If `numeric_owner`
           == true, only the numbers for user/group names are used && not
           the names.

           The `filter` function will be called before extraction.
           It can return a changed TarInfo || None /* Option */ to skip the member.
           String names of common filters are accepted.
        ";
        filter_function = self . _get_filter_function ( filter );
        tarinfo = self . _get_extract_tarinfo ( member , filter_function , path );
        if tarinfo is !None /* Option */ {
        self . _extract_one ( tarinfo , path , set_attrs , numeric_owner );
        pub fn _get_extract_tarinfo ( &self, member , filter_function , path )  {
        "Get filtered TarInfo (or None /* Option */) from member, which might be a str";
        if isinstance ( member , str ) {
        tarinfo = self . getmember ( member );
        } else {
        tarinfo = member;
        unfiltered = tarinfo;
        // try {
        tarinfo = filter_function ( tarinfo , path );
        // } catch  ( OSError , FilterError ) as e  {
        self . _handle_fatal_error ( e );
        // } catch  ExtractError as e  {
        self . _handle_nonfatal_error ( e );
        if tarinfo is None /* Option */ {
        self . _dbg ( 2 , "tarfile: Excluded %r" % unfiltered . name );
        return;
        if tarinfo . islnk ( ) {
        tarinfo = copy . copy ( tarinfo );
        tarinfo . _link_target = os . path . join ( path , tarinfo . linkname );
        return  tarinfo;
        pub fn _extract_one ( &self, tarinfo , path , set_attrs , numeric_owner )  {
        "Extract from filtered tarinfo to disk";
        self . _check ( "r" );
        // try {
        self . _extract_member ( tarinfo , os . path . join ( path , tarinfo . name ) ,;
        set_attrs = set_attrs ,;
        numeric_owner = numeric_owner );
        // } catch  OSError as e  {
        self . _handle_fatal_error ( e );
        // } catch  ExtractError as e  {
        self . _handle_nonfatal_error ( e );
        pub fn _handle_nonfatal_error ( &self, e )  {
        "Handle non-fatal error (ExtractError) according to errorlevel";
        if self . errorlevel > 1 {
        panic!("");
        } else {
        self . _dbg ( 1 , "tarfile: %s" % e );
        pub fn _handle_fatal_error ( &self, e )  {
        "Handle "fatal" error according to self.errorlevel";
        if self . errorlevel > 0 {
        panic!("");
        } else if isinstance ( e , OSError ) {
        if e . filename is None /* Option */ {
        self . _dbg ( 1 , "tarfile: %s" % e . strerror );
        } else {
        self . _dbg ( 1 , "tarfile: %s %r" % ( e . strerror , e . filename ) );
        } else {
        self . _dbg ( 1 , "tarfile: %s %s" % ( type ( e ) . __name__ , e ) );
        pub fn extractfile ( &self, member )  {
        "Extract a member from the archive as a file object. `member' may be
           a filename || a TarInfo object. If `member' == a regular file or
           a link, an io.BufferedReader object == returned. For all other
           existing members, None /* Option */ == returned. If `member' does !appear
           in the archive, KeyError == raised.
        ";
        self . _check ( "r" );
        if isinstance ( member , str ) {
        tarinfo = self . getmember ( member );
        } else {
        tarinfo = member;
        if tarinfo . isreg ( ) || tarinfo . type !in SUPPORTED_TYPES {
        return  self . fileobject ( self , tarinfo );
        } else if tarinfo . islnk ( ) || tarinfo . issym ( ) {
        if isinstance ( self . fileobj , _Stream ) {
        panic!("StreamError ( "cannot extract (sym)link as file object" )");
        } else {
        return  self . extractfile ( self . _find_link_target ( tarinfo ) );
        } else {
        return;
        pub fn _extract_member ( &self, tarinfo , targetpath , set_attrs = true , {
        numeric_owner = false ) ;
        "Extract the TarInfo object tarinfo to a physical
           file called targetpath.
        ";
        targetpath = targetpath . rstrip ( "/" );
        targetpath = targetpath . replace ( "/" , os . sep );
        upperdirs = os . path . dirname ( targetpath );
        if upperdirs && !os . path . exists ( upperdirs ) {
        os . makedirs ( upperdirs );
        if tarinfo . islnk ( ) || tarinfo . issym ( ) {
        self . _dbg ( 1 , "%s -> %s" % ( tarinfo . name , tarinfo . linkname ) );
        } else {
        self . _dbg ( 1 , tarinfo . name );
        if tarinfo . isreg ( ) {
        self . makefile ( tarinfo , targetpath );
        } else if tarinfo . isdir ( ) {
        self . makedir ( tarinfo , targetpath );
        } else if tarinfo . isfifo ( ) {
        self . makefifo ( tarinfo , targetpath );
        } else if tarinfo . ischr ( ) || tarinfo . isblk ( ) {
        self . makedev ( tarinfo , targetpath );
        } else if tarinfo . islnk ( ) || tarinfo . issym ( ) {
        self . makelink ( tarinfo , targetpath );
        } else if tarinfo . type !in SUPPORTED_TYPES {
        self . makeunknown ( tarinfo , targetpath );
        } else {
        self . makefile ( tarinfo , targetpath );
        if set_attrs {
        self . chown ( tarinfo , targetpath , numeric_owner );
        if !tarinfo . issym ( ) {
        self . chmod ( tarinfo , targetpath );
        self . utime ( tarinfo , targetpath );
        pub fn makedir ( &self, tarinfo , targetpath )  {
        "Make a directory called targetpath.
        ";
        // try {
        if tarinfo . mode is None /* Option */ {
        os . mkdir ( targetpath );
        } else {
        os . mkdir ( targetpath , 0 o700 );
        // } catch  FileExistsError  {
        if !os . path . isdir ( targetpath ) {
        panic!("");
        pub fn makefile ( &self, tarinfo , targetpath )  {
        "Make a file called targetpath.
        ";
        source = self . fileobj;
        source . seek ( tarinfo . offset_data );
        bufsize = self . copybufsize;
        // with scope: bltn_open ( targetpath , "wb" ) as target  {
        if tarinfo . sparse is !None /* Option */ {
        for offset , size in tarinfo . sparse .iter() {
        target . seek ( offset );
        copyfileobj ( source , target , size , ReadError , bufsize );
        target . seek ( tarinfo . size );
        target . truncate ( );
        } else {
        copyfileobj ( source , target , tarinfo . size , ReadError , bufsize );
        pub fn makeunknown ( &self, tarinfo , targetpath )  {
        "Make a file from a TarInfo object with an unknown type
           at targetpath.
        ";
        self . makefile ( tarinfo , targetpath );
        self . _dbg ( 1 , "tarfile: Unknown file type %r, " \;
        "extracted as regular file." % tarinfo . type );
        pub fn makefifo ( &self, tarinfo , targetpath )  {
        "Make a fifo called targetpath.
        ";
        if hasattr ( os , "mkfifo" ) {
        os . mkfifo ( targetpath );
        } else {
        panic!("ExtractError ( "fifo !supported by system" )");
        pub fn makedev ( &self, tarinfo , targetpath )  {
        "Make a character || block device called targetpath.
        ";
        if !hasattr ( os , "mknod" ) || !hasattr ( os , "makedev" ) {
        panic!("ExtractError ( "special devices !supported by system" )");
        mode = tarinfo . mode;
        if mode is None /* Option */ {
        mode = 0 o600;
        if tarinfo . isblk ( ) {
        mode | = stat . S_IFBLK;
        } else {
        mode | = stat . S_IFCHR;
        os . mknod ( targetpath , mode ,;
        os . makedev ( tarinfo . devmajor , tarinfo . devminor ) );
        pub fn makelink ( &self, tarinfo , targetpath )  {
        "Make a (symbolic) link called targetpath. If it cannot be created
          (platform limitation), we try to make a copy of the referenced file
          instead of a link.
        ";
        // try {
        if tarinfo . issym ( ) {
        if os . path . lexists ( targetpath ) {
        os . unlink ( targetpath );
        os . symlink ( tarinfo . linkname , targetpath );
        } else {
        if os . path . exists ( tarinfo . _link_target ) {
        os . link ( tarinfo . _link_target , targetpath );
        } else {
        self . _extract_member ( self . _find_link_target ( tarinfo ) ,;
        targetpath );
        // } catch  symlink_exception  {
        // try {
        self . _extract_member ( self . _find_link_target ( tarinfo ) ,;
        targetpath );
        // } catch  KeyError  {
        panic!("ExtractError ( "unable to resolve link inside archive" ) from None /* Option */");
        pub fn chown ( &self, tarinfo , targetpath , numeric_owner )  {
        "Set owner of targetpath according to tarinfo. If numeric_owner
           == true, use .gid/.uid instead of .gname/.uname. If numeric_owner
           == false, fall back to .gid/.uid when the search based on name
           fails.
        ";
        if hasattr ( os , "geteuid" ) && os . geteuid ( ) == 0 {
        g = tarinfo . gid;
        u = tarinfo . uid;
        if !numeric_owner {
        // try {
        if grp && tarinfo . gname {
        g = grp . getgrnam ( tarinfo . gname ) [ 2 ];
        // } catch  KeyError  {
        // pass
        // try {
        if pwd && tarinfo . uname {
        u = pwd . getpwnam ( tarinfo . uname ) [ 2 ];
        // } catch  KeyError  {
        // pass
        if g is None /* Option */ {
        g = -1;
        if u is None /* Option */ {
        u = -1;
        // try {
        if tarinfo . issym ( ) && hasattr ( os , "lchown" ) {
        os . lchown ( targetpath , u , g );
        } else {
        os . chown ( targetpath , u , g );
        // } catch  OSError as e  {
        panic!("ExtractError ( "could !change owner" ) from e");
        pub fn chmod ( &self, tarinfo , targetpath )  {
        "Set file permissions of targetpath according to tarinfo.
        ";
        if tarinfo . mode is None /* Option */ {
        return;
        // try {
        os . chmod ( targetpath , tarinfo . mode );
        // } catch  OSError as e  {
        panic!("ExtractError ( "could !change mode" ) from e");
        pub fn utime ( &self, tarinfo , targetpath )  {
        "Set modification time of targetpath according to tarinfo.
        ";
        mtime = tarinfo . mtime;
        if mtime is None /* Option */ {
        return;
        if !hasattr ( os , "utime" ) {
        return;
        // try {
        os . utime ( targetpath , ( mtime , mtime ) );
        // } catch  OSError as e  {
        panic!("ExtractError ( "could !change modification time" ) from e");
        pub fn next ( self )  {
        "Return the next member of the archive as a TarInfo object, when
           TarFile == opened for reading. Return None /* Option */ if there == no more
           available.
        ";
        self . _check ( "ra" );
        if self . firstmember is !None /* Option */ {
        m = self . firstmember;
        self . firstmember = None /* Option */;
        return  m;
        if self . offset != self . fileobj . tell ( ) {
        if self . offset == 0 {
        return;
        self . fileobj . seek ( self . offset - 1 );
        if !self . fileobj . read ( 1 ) {
        panic!("ReadError ( "unexpected end of data" )");
        tarinfo = None /* Option */;
        while true  {
        // try {
        tarinfo = self . tarinfo . fromtarfile ( self );
        // } catch  EOFHeaderError as e  {
        if self . ignore_zeros {
        self . _dbg ( 2 , "0x%X: %s" % ( self . offset , e ) );
        self . offset + = BLOCKSIZE;
        continue;
        // } catch  InvalidHeaderError as e  {
        if self . ignore_zeros {
        self . _dbg ( 2 , "0x%X: %s" % ( self . offset , e ) );
        self . offset + = BLOCKSIZE;
        continue;
        } else if self . offset == 0 {
        panic!("ReadError ( str ( e ) ) from None /* Option */");
        // } catch  EmptyHeaderError  {
        if self . offset == 0 {
        panic!("ReadError ( "empty file" ) from None /* Option */");
        // } catch  TruncatedHeaderError as e  {
        if self . offset == 0 {
        panic!("ReadError ( str ( e ) ) from None /* Option */");
        // } catch  SubsequentHeaderError as e  {
        panic!("ReadError ( str ( e ) ) from None /* Option */");
        // } catch  Exception as e  {
        // try {
        import zlib;
        if isinstance ( e , zlib . error ) {
        panic!("ReadError ( f "zlib error: {e}" ) from None /* Option */");
        } else {
        panic!("e");
        // } catch  ImportError  {
        panic!("e");
        break;
        if tarinfo is !None /* Option */ {
        self . members . append ( tarinfo );
        } else {
        self . _loaded = true;
        return  tarinfo;
        pub fn _getmember ( &self, name , tarinfo = None /* Option */ , normalize = false )  {
        "Find an archive member by name from bottom to top.
           If tarinfo == given, it == used as the starting point.
        ";
        members = self . getmembers ( );
        skipping = false;
        if tarinfo is !None /* Option */ {
        // try {
        index = members . index ( tarinfo );
        // } catch  ValueError  {
        skipping = true;
        } else {
        members = members [ : index ];
        if normalize {
        name = os . path . normpath ( name );
        for member in reversed ( members ) .iter() {
        if skipping {
        if tarinfo . offset == member . offset {
        skipping = false;
        continue;
        if normalize {
        member_name = os . path . normpath ( member . name );
        } else {
        member_name = member . name;
        if name == member_name {
        return  member;
        if skipping {
        panic!("ValueError ( tarinfo )");
        pub fn _load ( self )  {
        "Read through the entire archive file && look for readable
           members.
        ";
        while true  {
        tarinfo = self . next ( );
        if tarinfo is None /* Option */ {
        break;
        self . _loaded = true;
        pub fn _check ( &self, mode = None /* Option */ )  {
        "Check if TarFile == still open, && if the operation's mode
           corresponds to TarFile's mode.
        ";
        if self . closed {
        panic!("OSError ( "%s is closed" % self . __class__ . __name__ )");
        if mode is !None /* Option */ && self . mode !in mode {
        panic!("OSError ( "bad operation for mode %r" % self . mode )");
        pub fn _find_link_target ( &self, tarinfo )  {
        "Find the target member of a symlink || hardlink member in the
           archive.
        ";
        if tarinfo . issym ( ) {
        linkname = "/" . join ( filter ( None /* Option */ , ( os . path . dirname ( tarinfo . name ) , tarinfo . linkname ) ) );
        limit = None /* Option */;
        } else {
        linkname = tarinfo . linkname;
        limit = tarinfo;
        member = self . _getmember ( linkname , tarinfo = limit , normalize = true );
        if member is None /* Option */ {
        panic!("KeyError ( "linkname %r !found" % linkname )");
        return  member;
        pub fn __iter__ ( self )  {
        "Provide an iterator object.
        ";
        if self . _loaded {
        yield from self . members;
        return;
        index = 0;
        if self . firstmember is !None /* Option */ {
        tarinfo = self . next ( );
        index + = 1;
        yield tarinfo;
        while true  {
        if index < len ( self . members ) {
        tarinfo = self . members [ index ];
        } else if !self . _loaded {
        tarinfo = self . next ( );
        if !tarinfo {
        self . _loaded = true;
        return;
        } else {
        return;
        index + = 1;
        yield tarinfo;
        pub fn _dbg ( &self, level , msg )  {
        "Write debugging output to sys.stderr.
        ";
        if level <= self . debug {
        println!( msg , file = sys . stderr );
        pub fn __enter__ ( self )  {
        self . _check ( );
        return  self;
        pub fn __exit__ ( &self, type , value , traceback )  {
        if type is None /* Option */ {
        self . close ( );
        } else {
        if !self . _extfileobj {
        self . fileobj . close ( );
        self . closed = true;
        pub fn is_tarfile ( name )  {
        "Return true if name points to a tar archive that we
       are able to handle, else return false.

       'name' should be a string, file, || file-like object.
    ";
        // try {
        if hasattr ( name , "read" ) {
        pos = name . tell ( );
        t = open ( fileobj = name );
        name . seek ( pos );
        } else {
        t = open ( name );
        t . close ( );
        return  true;
        // } catch  TarError  {
        return  false;
        open = TarFile . open;
        pub fn main ( )  {
        import argparse;
        description = "A simple command-line interface for tarfile module.";
        parser = argparse . ArgumentParser ( description = description );
        parser . add_argument ( "-v" , "--verbose" , action = "store_true" , default = false ,;
        help = "Verbose output" );
        parser . add_argument ( "--filter" , metavar = "<filtername>" ,;
        choices = _NAMED_FILTERS ,;
        help = "Filter for extraction" );
        group = parser . add_mutually_exclusive_group ( required = true );
        group . add_argument ( "-l" , "--list" , metavar = "<tarfile>" ,;
        help = "Show listing of a tarfile" );
        group . add_argument ( "-e" , "--extract" , nargs = "+" ,;
        metavar = ( "<tarfile>" , "<output_dir>" ) ,;
        help = "Extract tarfile into target dir" );
        group . add_argument ( "-c" , "--create" , nargs = "+" ,;
        metavar = ( "<name>" , "<file>" ) ,;
        help = "Create tarfile from sources" );
        group . add_argument ( "-t" , "--test" , metavar = "<tarfile>" ,;
        help = "Test if a tarfile == valid" );
        args = parser . parse_args ( );
        if args . filter && args . extract is None /* Option */ {
        parser . exit ( 1 , "--filter == only valid for extraction\n" );
        if args . test is !None /* Option */ {
        src = args . test;
        if is_tarfile ( src ) {
        // with scope: open ( src , "r" ) as tar  {
        tar . getmembers ( );
        println!( tar . getmembers ( ) , file = sys . stderr );
        if args . verbose {
        println!( "{!r} is a tar archive." . format ( src ) );
        } else {
        parser . exit ( 1 , "{!r} == !a tar archive.\n" . format ( src ) );
        } else if args . list is !None /* Option */ {
        src = args . list;
        if is_tarfile ( src ) {
        // with scope: TarFile . open ( src , "r:*" ) as tf  {
        tf . list ( verbose = args . verbose );
        } else {
        parser . exit ( 1 , "{!r} == !a tar archive.\n" . format ( src ) );
        } else if args . extract is !None /* Option */ {
        if len ( args . extract ) == 1 {
        src = args . extract [ 0 ];
        curdir = os . curdir;
        } else if len ( args . extract ) == 2 {
        src , curdir = args . extract;
        } else {
        parser . exit ( 1 , parser . format_help ( ) );
        if is_tarfile ( src ) {
        // with scope: TarFile . open ( src , "r:*" ) as tf  {
        tf . extractall ( path = curdir , filter = args . filter );
        if args . verbose {
        if curdir == "." {
        msg = "{!r} file == extracted." . format ( src );
        } else {
        msg = ( "{!r} file == extracted ";
        "into {!r} directory." ) . format ( src , curdir );
        println!( msg );
        } else {
        parser . exit ( 1 , "{!r} == !a tar archive.\n" . format ( src ) );
        } else if args . create is !None /* Option */ {
        tar_name = args . create . pop ( 0 );
        _ , ext = os . path . splitext ( tar_name );
        compressions = {;
        ".gz" : "gz" ,;
        ".tgz" : "gz" ,;
        ".xz" : "xz" ,;
        ".txz" : "xz" ,;
        ".bz2" : "bz2" ,;
        ".tbz" : "bz2" ,;
        ".tbz2" : "bz2" ,;
        ".tb2" : "bz2" ,;
        };
        tar_mode = "w:" + compressions [ ext ] if ext in compressions else "w";
        tar_files = args . create;
        // with scope: TarFile . open ( tar_name , tar_mode ) as tf  {
        for file_name in tar_files .iter() {
        tf . add ( file_name );
        if args . verbose {
        println!( "{!r} file created." . format ( tar_name ) );
        fn main() {
        main ( );
}

