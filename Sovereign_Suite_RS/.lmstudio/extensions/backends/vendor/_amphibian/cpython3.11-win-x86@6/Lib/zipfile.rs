//! zipfile.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::binascii;
// use crate::io;
// use std::fs;
// use crate::shutil;
// use crate::struct;
// use std::thread;
// use crate::contextlib;
// use crate::zlib;
// use crate::bz2;
// use crate::lzma;
// use crate::warnings;
// use crate::py_compile;
// use crate::argparse;

pub const __all__: &str = ["BadZipFile" ,"BadZipfile" ,"error" ,;
pub struct BadZipFile {
    pub orig_filename: String, // TODO: infer type
    pub filename: String, // TODO: infer type
    pub date_time: String, // TODO: infer type
    pub compress_type: String, // TODO: infer type
    pub _compresslevel: String, // TODO: infer type
    pub comment: String, // TODO: infer type
    pub extra: String, // TODO: infer type
    pub create_system: String, // TODO: infer type
    pub create_version: String, // TODO: infer type
    pub extract_version: String, // TODO: infer type
    pub reserved: String, // TODO: infer type
    pub flag_bits: String, // TODO: infer type
    pub volume: String, // TODO: infer type
    pub internal_attr: String, // TODO: infer type
    pub external_attr: String, // TODO: infer type
    pub compress_size: String, // TODO: infer type
    pub file_size: String, // TODO: infer type
    pub _end_offset: String, // TODO: infer type
    pub _comp: String, // TODO: infer type
    pub _decomp: String, // TODO: infer type
    pub _unconsumed: String, // TODO: infer type
    pub eof: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _close: String, // TODO: infer type
    pub _lock: String, // TODO: infer type
    pub _writing: String, // TODO: infer type
    pub seekable: String, // TODO: infer type
    pub fp: String, // TODO: infer type
    pub offset: String, // TODO: infer type
    pub _fileobj: String, // TODO: infer type
    pub _pwd: String, // TODO: infer type
    pub _close_fileobj: String, // TODO: infer type
    pub _compress_type: String, // TODO: infer type
    pub _compress_left: String, // TODO: infer type
    pub _left: String, // TODO: infer type
    pub _decompressor: String, // TODO: infer type
    pub _eof: String, // TODO: infer type
    pub _readbuffer: String, // TODO: infer type
    pub _offset: String, // TODO: infer type
    pub newlines: String, // TODO: infer type
    pub mode: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub _expected_crc: String, // TODO: infer type
    pub _running_crc: String, // TODO: infer type
    pub _seekable: String, // TODO: infer type
    pub _orig_compress_start: String, // TODO: infer type
    pub _orig_compress_size: String, // TODO: infer type
    pub _orig_file_size: String, // TODO: infer type
    pub _orig_start_crc: String, // TODO: infer type
    pub _decrypter: String, // TODO: infer type
    pub _zinfo: String, // TODO: infer type
    pub _zip64: String, // TODO: infer type
    pub _zipfile: String, // TODO: infer type
    pub _compressor: String, // TODO: infer type
    pub _file_size: String, // TODO: infer type
    pub _compress_size: String, // TODO: infer type
    pub _crc: String, // TODO: infer type
    pub _allowZip64: String, // TODO: infer type
    pub _didModify: String, // TODO: infer type
    pub debug: String, // TODO: infer type
    pub NameToInfo: String, // TODO: infer type
    pub filelist: String, // TODO: infer type
    pub compression: String, // TODO: infer type
    pub compresslevel: String, // TODO: infer type
    pub pwd: String, // TODO: infer type
    pub _comment: String, // TODO: infer type
    pub _strict_timestamps: String, // TODO: infer type
    pub metadata_encoding: String, // TODO: infer type
    pub _filePassed: String, // TODO: infer type
    pub _fileRefCnt: String, // TODO: infer type
    pub start_dir: String, // TODO: infer type
    pub _optimize: String, // TODO: infer type
    pub __names: String, // TODO: infer type
    pub __lookup: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub at: String, // TODO: infer type
}

impl BadZipFile {
}

pub struct LargeZipFile {
    pub orig_filename: String, // TODO: infer type
    pub filename: String, // TODO: infer type
    pub date_time: String, // TODO: infer type
    pub compress_type: String, // TODO: infer type
    pub _compresslevel: String, // TODO: infer type
    pub comment: String, // TODO: infer type
    pub extra: String, // TODO: infer type
    pub create_system: String, // TODO: infer type
    pub create_version: String, // TODO: infer type
    pub extract_version: String, // TODO: infer type
    pub reserved: String, // TODO: infer type
    pub flag_bits: String, // TODO: infer type
    pub volume: String, // TODO: infer type
    pub internal_attr: String, // TODO: infer type
    pub external_attr: String, // TODO: infer type
    pub compress_size: String, // TODO: infer type
    pub file_size: String, // TODO: infer type
    pub _end_offset: String, // TODO: infer type
    pub _comp: String, // TODO: infer type
    pub _decomp: String, // TODO: infer type
    pub _unconsumed: String, // TODO: infer type
    pub eof: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _close: String, // TODO: infer type
    pub _lock: String, // TODO: infer type
    pub _writing: String, // TODO: infer type
    pub seekable: String, // TODO: infer type
    pub fp: String, // TODO: infer type
    pub offset: String, // TODO: infer type
    pub _fileobj: String, // TODO: infer type
    pub _pwd: String, // TODO: infer type
    pub _close_fileobj: String, // TODO: infer type
    pub _compress_type: String, // TODO: infer type
    pub _compress_left: String, // TODO: infer type
    pub _left: String, // TODO: infer type
    pub _decompressor: String, // TODO: infer type
    pub _eof: String, // TODO: infer type
    pub _readbuffer: String, // TODO: infer type
    pub _offset: String, // TODO: infer type
    pub newlines: String, // TODO: infer type
    pub mode: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub _expected_crc: String, // TODO: infer type
    pub _running_crc: String, // TODO: infer type
    pub _seekable: String, // TODO: infer type
    pub _orig_compress_start: String, // TODO: infer type
    pub _orig_compress_size: String, // TODO: infer type
    pub _orig_file_size: String, // TODO: infer type
    pub _orig_start_crc: String, // TODO: infer type
    pub _decrypter: String, // TODO: infer type
    pub _zinfo: String, // TODO: infer type
    pub _zip64: String, // TODO: infer type
    pub _zipfile: String, // TODO: infer type
    pub _compressor: String, // TODO: infer type
    pub _file_size: String, // TODO: infer type
    pub _compress_size: String, // TODO: infer type
    pub _crc: String, // TODO: infer type
    pub _allowZip64: String, // TODO: infer type
    pub _didModify: String, // TODO: infer type
    pub debug: String, // TODO: infer type
    pub NameToInfo: String, // TODO: infer type
    pub filelist: String, // TODO: infer type
    pub compression: String, // TODO: infer type
    pub compresslevel: String, // TODO: infer type
    pub pwd: String, // TODO: infer type
    pub _comment: String, // TODO: infer type
    pub _strict_timestamps: String, // TODO: infer type
    pub metadata_encoding: String, // TODO: infer type
    pub _filePassed: String, // TODO: infer type
    pub _fileRefCnt: String, // TODO: infer type
    pub start_dir: String, // TODO: infer type
    pub _optimize: String, // TODO: infer type
    pub __names: String, // TODO: infer type
    pub __lookup: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub at: String, // TODO: infer type
}

impl LargeZipFile {
}

pub const error: f64 = BadZipfile = BadZipFile;
pub const ZIP64_LIMIT: f64 = ( 1 < < 31 ) - 1;
pub const ZIP_FILECOUNT_LIMIT: f64 = ( 1 < < 16 ) - 1;
pub const ZIP_MAX_COMMENT: f64 = ( 1 < < 16 ) - 1;
pub const ZIP_STORED: u64 = 0;
pub const ZIP_DEFLATED: u64 = 8;
pub const ZIP_BZIP2: u64 = 12;
pub const ZIP_LZMA: u64 = 14;
pub const DEFAULT_VERSION: u64 = 20;
pub const ZIP64_VERSION: u64 = 45;
pub const BZIP2_VERSION: u64 = 46;
pub const LZMA_VERSION: u64 = 63;
pub const MAX_EXTRACT_VERSION: u64 = 63;
pub const structEndArchive: &str = b"<4s4H2LH";
pub const stringEndArchive: &str = b"PK\005\006";
pub const sizeEndCentDir: f64 = struct . calcsize ( structEndArchive );
pub const _ECD_SIGNATURE: u64 = 0;
pub const _ECD_DISK_NUMBER: u64 = 1;
pub const _ECD_DISK_START: u64 = 2;
pub const _ECD_ENTRIES_THIS_DISK: u64 = 3;
pub const _ECD_ENTRIES_TOTAL: u64 = 4;
pub const _ECD_SIZE: u64 = 5;
pub const _ECD_OFFSET: u64 = 6;
pub const _ECD_COMMENT_SIZE: u64 = 7;
pub const _ECD_COMMENT: u64 = 8;
pub const _ECD_LOCATION: u64 = 9;
pub const structCentralDir: &str = "<4s4B4HL2L5H2L";
pub const stringCentralDir: &str = b"PK\001\002";
pub const sizeCentralDir: f64 = struct . calcsize ( structCentralDir );
pub const _CD_SIGNATURE: u64 = 0;
pub const _CD_CREATE_VERSION: u64 = 1;
pub const _CD_CREATE_SYSTEM: u64 = 2;
pub const _CD_EXTRACT_VERSION: u64 = 3;
pub const _CD_EXTRACT_SYSTEM: u64 = 4;
pub const _CD_FLAG_BITS: u64 = 5;
pub const _CD_COMPRESS_TYPE: u64 = 6;
pub const _CD_TIME: u64 = 7;
pub const _CD_DATE: u64 = 8;
pub const _CD_CRC: u64 = 9;
pub const _CD_COMPRESSED_SIZE: u64 = 10;
pub const _CD_UNCOMPRESSED_SIZE: u64 = 11;
pub const _CD_FILENAME_LENGTH: u64 = 12;
pub const _CD_EXTRA_FIELD_LENGTH: u64 = 13;
pub const _CD_COMMENT_LENGTH: u64 = 14;
pub const _CD_DISK_NUMBER_START: u64 = 15;
pub const _CD_INTERNAL_FILE_ATTRIBUTES: u64 = 16;
pub const _CD_EXTERNAL_FILE_ATTRIBUTES: u64 = 17;
pub const _CD_LOCAL_HEADER_OFFSET: u64 = 18;
pub const _MASK_ENCRYPTED: u64 = 1 < < 0;
pub const _MASK_COMPRESS_OPTION_1: u64 = 1 < < 1;
pub const _MASK_USE_DATA_DESCRIPTOR: u64 = 1 < < 3;
pub const _MASK_COMPRESSED_PATCH: u64 = 1 < < 5;
pub const _MASK_STRONG_ENCRYPTION: u64 = 1 < < 6;
pub const _MASK_UTF_FILENAME: u64 = 1 < < 11;
pub const structFileHeader: &str = "<4s2B4HL2L2H";
pub const stringFileHeader: &str = b"PK\003\004";
pub const sizeFileHeader: f64 = struct . calcsize ( structFileHeader );
pub const _FH_SIGNATURE: u64 = 0;
pub const _FH_EXTRACT_VERSION: u64 = 1;
pub const _FH_EXTRACT_SYSTEM: u64 = 2;
pub const _FH_GENERAL_PURPOSE_FLAG_BITS: u64 = 3;
pub const _FH_COMPRESSION_METHOD: u64 = 4;
pub const _FH_LAST_MOD_TIME: u64 = 5;
pub const _FH_LAST_MOD_DATE: u64 = 6;
pub const _FH_CRC: u64 = 7;
pub const _FH_COMPRESSED_SIZE: u64 = 8;
pub const _FH_UNCOMPRESSED_SIZE: u64 = 9;
pub const _FH_FILENAME_LENGTH: u64 = 10;
pub const _FH_EXTRA_FIELD_LENGTH: u64 = 11;
pub const structEndArchive64Locator: &str = "<4sLQL";
pub const stringEndArchive64Locator: &str = b"PK\x06\x07";
pub const sizeEndCentDir64Locator: f64 = struct . calcsize ( structEndArchive64Locator );
pub const structEndArchive64: &str = "<4sQ2H2L4Q";
pub const stringEndArchive64: &str = b"PK\x06\x06";
pub const sizeEndCentDir64: f64 = struct . calcsize ( structEndArchive64 );
pub const _CD64_SIGNATURE: u64 = 0;
pub const _CD64_DIRECTORY_RECSIZE: u64 = 1;
pub const _CD64_CREATE_VERSION: u64 = 2;
pub const _CD64_EXTRACT_VERSION: u64 = 3;
pub const _CD64_DISK_NUMBER: u64 = 4;
pub const _CD64_DISK_NUMBER_START: u64 = 5;
pub const _CD64_NUMBER_ENTRIES_THIS_DISK: u64 = 6;
pub const _CD64_NUMBER_ENTRIES_TOTAL: u64 = 7;
pub const _CD64_DIRECTORY_SIZE: u64 = 8;
pub const _CD64_OFFSET_START_CENTDIR: u64 = 9;
pub const _DD_SIGNATURE: u64 = 0x08074 b50;
pub const _EXTRA_FIELD_STRUCT: &str = struct . Struct ("<HH" );
pub fn _strip_extra(extra: &str, xids: &str) {
        unpack = _EXTRA_FIELD_STRUCT . unpack;
        modified = false;
        buffer = [ ];
        start = i = 0;
        while i + 4 <= len ( extra )  {
        xid , xlen = unpack ( extra [ i : i + 4 ] );
        j = i + 4 + xlen;
        if xid in xids {
        if i != start {
        buffer . append ( extra [ start : i ] );
        start = j;
        modified = true;
        i = j;
        if !modified {
        return  extra;
        if start != len ( extra ) {
        buffer . append ( extra [ start : ] );
        return  b "" . join ( buffer );
        pub fn _check_zipfile ( fp )  {
        // try {
        if _EndRecData ( fp ) {
        return  true;
        // } catch  OSError  {
        // pass
        return  false;
        pub fn is_zipfile ( filename )  {
        "Quickly see if a file == a ZIP file by checking the magic number.

    The filename argument may be a file || file-like object too.
    ";
        result = false;
        // try {
        if hasattr ( filename , "read" ) {
        result = _check_zipfile ( fp = filename );
        } else {
        // with scope: open ( filename , "rb" ) as fp  {
        result = _check_zipfile ( fp );
        // } catch  OSError  {
        // pass
        return  result;
        pub fn _EndRecData64 ( fpin , offset , endrec )  {
        "
    Read the ZIP64 end-of-archive records && use that to update endrec
    ";
        // try {
        fpin . seek ( offset - sizeEndCentDir64Locator , 2 );
        // } catch  OSError  {
        return  endrec;
        data = fpin . read ( sizeEndCentDir64Locator );
        if len ( data ) != sizeEndCentDir64Locator {
        return  endrec;
        sig , diskno , reloff , disks = struct . unpack ( structEndArchive64Locator , data );
        if sig != stringEndArchive64Locator {
        return  endrec;
        if diskno != 0 || disks > 1 {
        panic!("BadZipFile ( "zipfiles that span multiple disks are !supported" )");
        fpin . seek ( offset - sizeEndCentDir64Locator - sizeEndCentDir64 , 2 );
        data = fpin . read ( sizeEndCentDir64 );
        if len ( data ) != sizeEndCentDir64 {
        return  endrec;
        sig , sz , create_version , read_version , disk_num , disk_dir , \;
        dircount , dircount2 , dirsize , diroffset = \;
        struct . unpack ( structEndArchive64 , data );
        if sig != stringEndArchive64 {
        return  endrec;
        endrec [ _ECD_SIGNATURE ] = sig;
        endrec [ _ECD_DISK_NUMBER ] = disk_num;
        endrec [ _ECD_DISK_START ] = disk_dir;
        endrec [ _ECD_ENTRIES_THIS_DISK ] = dircount;
        endrec [ _ECD_ENTRIES_TOTAL ] = dircount2;
        endrec [ _ECD_SIZE ] = dirsize;
        endrec [ _ECD_OFFSET ] = diroffset;
        return  endrec;
        pub fn _EndRecData ( fpin )  {
        "Return data from the "End of Central Directory" record, || None /* Option */.

    The data == a list of the nine items in the ZIP "End of central dir"
    record followed by a tenth item, the file seek offset of this record.";
        fpin . seek ( 0 , 2 );
        filesize = fpin . tell ( );
        // try {
        fpin . seek ( - sizeEndCentDir , 2 );
        // } catch  OSError  {
        return;
        data = fpin . read ( );
        if ( len ( data ) == sizeEndCentDir and {
        data [ 0 : 4 ] == stringEndArchive and;
        data [ -2 : ] == b "\000\000" ) ;
        endrec = struct . unpack ( structEndArchive , data );
        endrec = list ( endrec );
        endrec . append ( b "" );
        endrec . append ( filesize - sizeEndCentDir );
        return  _EndRecData64 ( fpin , - sizeEndCentDir , endrec );
        maxCommentStart = max ( filesize - ( 1 < < 16 ) - sizeEndCentDir , 0 );
        fpin . seek ( maxCommentStart , 0 );
        data = fpin . read ( );
        start = data . rfind ( stringEndArchive );
        if start >= 0 {
        recData = data [ start : start + sizeEndCentDir ];
        if len ( recData ) != sizeEndCentDir {
        return;
        endrec = list ( struct . unpack ( structEndArchive , recData ) );
        commentSize = endrec [ _ECD_COMMENT_SIZE ];
        comment = data [ start + sizeEndCentDir : start + sizeEndCentDir + commentSize ];
        endrec . append ( comment );
        endrec . append ( maxCommentStart + start );
        return  _EndRecData64 ( fpin , maxCommentStart + start - filesize ,;
        endrec );
        return;
        class ZipInfo ( object ) ;
        "Class with attributes describing each file in the ZIP archive.";
        __slots__ = (;
        "orig_filename" ,;
        "filename" ,;
        "date_time" ,;
        "compress_type" ,;
        "_compresslevel" ,;
        "comment" ,;
        "extra" ,;
        "create_system" ,;
        "create_version" ,;
        "extract_version" ,;
        "reserved" ,;
        "flag_bits" ,;
        "volume" ,;
        "internal_attr" ,;
        "external_attr" ,;
        "header_offset" ,;
        "CRC" ,;
        "compress_size" ,;
        "file_size" ,;
        "_raw_time" ,;
        "_end_offset" ,;
        );
        pub fn __init__ ( &self, filename = "NoName" , date_time = ( 1980 , 1 , 1 , 0 , 0 , 0 ) )  {
        self . orig_filename = filename;
        null_byte = filename . find ( chr ( 0 ) );
        if null_byte >= 0 {
        filename = filename [ 0 : null_byte ];
        if os . sep != "/" && os . sep in filename {
        filename = filename . replace ( os . sep , "/" );
        self . filename = filename;
        self . date_time = date_time;
        if date_time [ 0 ] < 1980 {
        panic!("ValueError ( "ZIP does !support timestamps before 1980" )");
        self . compress_type = ZIP_STORED;
        self . _compresslevel = None /* Option */;
        self . comment = b "";
        self . extra = b "";
        if sys . platform == "win32" {
        self . create_system = 0;
        } else {
        self . create_system = 3;
        self . create_version = DEFAULT_VERSION;
        self . extract_version = DEFAULT_VERSION;
        self . reserved = 0;
        self . flag_bits = 0;
        self . volume = 0;
        self . internal_attr = 0;
        self . external_attr = 0;
        self . compress_size = 0;
        self . file_size = 0;
        self . _end_offset = None /* Option */;
        pub fn __repr__ ( self )  {
        result = [ "<%s filename=%r" % ( self . __class__ . __name__ , self . filename ) ];
        if self . compress_type != ZIP_STORED {
        result . append ( " compress_type=%s" %;
        compressor_names . get ( self . compress_type ,;
        self . compress_type ) );
        hi = self . external_attr > > 16;
        lo = self . external_attr & 0x FFFF;
        if hi {
        result . append ( " filemode=%r" % stat . filemode ( hi ) );
        if lo {
        result . append ( " external_attr=%#x" % lo );
        isdir = self . is_dir ( );
        if !isdir || self . file_size {
        result . append ( " file_size=%r" % self . file_size );
        if ( ( !isdir || self . compress_size ) and {
        ( self . compress_type != ZIP_STORED or;
        self . file_size != self . compress_size ) ) :;
        result . append ( " compress_size=%r" % self . compress_size );
        result . append ( ">" );
        return  "" . join ( result );
        pub fn FileHeader ( &self, zip64 = None /* Option */ )  {
        "Return the per-file header as a bytes object.

        When the optional zip64 arg == None /* Option */ rather than a bool, we will
        decide based upon the file_size && compress_size, if known,
        false otherwise.
        ";
        dt = self . date_time;
        dosdate = ( dt [ 0 ] - 1980 ) < < 9 | dt [ 1 ] < < 5 | dt [ 2 ];
        dostime = dt [ 3 ] < < 11 | dt [ 4 ] < < 5 | ( dt [ 5 ] / / 2 );
        if self . flag_bits & _MASK_USE_DATA_DESCRIPTOR {
        CRC = compress_size = file_size = 0;
        } else {
        CRC = self . CRC;
        compress_size = self . compress_size;
        file_size = self . file_size;
        extra = self . extra;
        min_version = 0;
        if zip64 is None /* Option */ {
        zip64 = file_size > ZIP64_LIMIT || compress_size > ZIP64_LIMIT;
        if zip64 {
        fmt = "<HHQQ";
        extra = extra + struct . pack ( fmt ,;
        1 , struct . calcsize ( fmt ) -4 , file_size , compress_size );
        file_size = 0x ffffffff;
        compress_size = 0x ffffffff;
        min_version = ZIP64_VERSION;
        if self . compress_type == ZIP_BZIP2 {
        min_version = max ( BZIP2_VERSION , min_version );
        } else if self . compress_type == ZIP_LZMA {
        min_version = max ( LZMA_VERSION , min_version );
        self . extract_version = max ( min_version , self . extract_version );
        self . create_version = max ( min_version , self . create_version );
        filename , flag_bits = self . _encodeFilenameFlags ( );
        header = struct . pack ( structFileHeader , stringFileHeader ,;
        self . extract_version , self . reserved , flag_bits ,;
        self . compress_type , dostime , dosdate , CRC ,;
        compress_size , file_size ,;
        len ( filename ) , len ( extra ) );
        return  header + filename + extra;
        pub fn _encodeFilenameFlags ( self )  {
        // try {
        return  self . filename . encode ( "ascii" ) , self . flag_bits;
        // } catch  UnicodeEncodeError  {
        return  self . filename . encode ( "utf-8" ) , self . flag_bits | _MASK_UTF_FILENAME;
        pub fn _decodeExtra ( self )  {
        extra = self . extra;
        unpack = struct . unpack;
        while len ( extra ) >= 4  {
        tp , ln = unpack ( "<HH" , extra [ : 4 ] );
        if ln + 4 > len ( extra ) {
        panic!("BadZipFile ( "Corrupt extra field %04x (size=%d)" % ( tp , ln ) )");
        if tp == 0x0001 {
        data = extra [ 4 : ln + 4 ];
        // try {
        if self . file_size in ( 0x FFFF_FFFF_FFFF_FFFF , 0x FFFF_FFFF ) {
        field = "File size";
        self . file_size , = unpack ( "<Q" , data [ : 8 ] );
        data = data [ 8 : ];
        if self . compress_size == 0x FFFF_FFFF {
        field = "Compress size";
        self . compress_size , = unpack ( "<Q" , data [ : 8 ] );
        data = data [ 8 : ];
        if self . header_offset == 0x FFFF_FFFF {
        field = "Header offset";
        self . header_offset , = unpack ( "<Q" , data [ : 8 ] );
        // } catch  struct . error  {
        panic!("BadZipFile ( f "Corrupt zip64 extra field. "");
        format!("{field} !found." ) from None /* Option */);
        extra = extra [ ln + 4 : ];
        @ classmethod;
        pub fn from_file ( cls , filename , arcname = None /* Option */ , * , strict_timestamps = true )  {
        "Construct an appropriate ZipInfo for a file on the filesystem.

        filename should be the path to a file || directory on the filesystem.

        arcname == the name which it will have within the archive (by default,
        this will be the same as filename, but without a drive letter && with
        leading path separators removed).
        ";
        if isinstance ( filename , os . PathLike ) {
        filename = os . fspath ( filename );
        st = os . stat ( filename );
        isdir = stat . S_ISDIR ( st . st_mode );
        mtime = time . localtime ( st . st_mtime );
        date_time = mtime [ 0 : 6 ];
        if !strict_timestamps && date_time [ 0 ] < 1980 {
        date_time = ( 1980 , 1 , 1 , 0 , 0 , 0 );
        } else if !strict_timestamps && date_time [ 0 ] > 2107 {
        date_time = ( 2107 , 12 , 31 , 23 , 59 , 59 );
        if arcname is None /* Option */ {
        arcname = filename;
        arcname = os . path . normpath ( os . path . splitdrive ( arcname ) [ 1 ] );
        while arcname [ 0 ] in ( os . sep , os . altsep )  {
        arcname = arcname [ 1 : ];
        if isdir {
        arcname + = "/";
        zinfo = cls ( arcname , date_time );
        zinfo . external_attr = ( st . st_mode & 0x FFFF ) < < 16;
        if isdir {
        zinfo . file_size = 0;
        zinfo . external_attr | = 0x10;
        } else {
        zinfo . file_size = st . st_size;
        return  zinfo;
        pub fn is_dir ( self )  {
        "Return true if this archive member == a directory.";
        if self . filename . endswith ( "/" ) {
        return  true;
        if os . path . altsep {
        return  self . filename . endswith ( ( os . path . sep , os . path . altsep ) );
        return  false;
        _crctable = None /* Option */;
        pub fn _gen_crc ( crc )  {
        for j in range ( 8 ) .iter() {
        if crc & 1 {
        crc = ( crc > > 1 ) ^ 0x EDB88320;
        } else {
        crc > >= 1;
        return  crc;
        pub fn _ZipDecrypter ( pwd )  {
        key0 = 305419896;
        key1 = 591751049;
        key2 = 878082192;
        global _crctable;
        if _crctable is None /* Option */ {
        _crctable = list ( map ( _gen_crc , range ( 256 ) ) );
        crctable = _crctable;
        pub fn crc32 ( ch , crc )  {
        "Compute the CRC32 primitive on one byte.";
        return  ( crc > > 8 ) ^ crctable [ ( crc ^ ch ) & 0x FF ];
        pub fn update_keys ( c )  {
        nonlocal key0 , key1 , key2;
        key0 = crc32 ( c , key0 );
        key1 = ( key1 + ( key0 & 0x FF ) ) & 0x FFFFFFFF;
        key1 = ( key1 * 134775813 + 1 ) & 0x FFFFFFFF;
        key2 = crc32 ( key1 > > 24 , key2 );
        for p in pwd .iter() {
        update_keys ( p );
        pub fn decrypter ( data )  {
        "Decrypt a bytes object.";
        result = bytearray ( );
        append = result . append;
        for c in data .iter() {
        k = key2 | 2;
        c ^ = ( ( k * ( k ^ 1 ) ) > > 8 ) & 0x FF;
        update_keys ( c );
        append ( c );
        return  bytes ( result );
        return  decrypter;
        class LZMACompressor ;
        pub fn __init__ ( self )  {
        self . _comp = None /* Option */;
        pub fn _init ( self )  {
        props = lzma . _encode_filter_properties ( { "id" : lzma . FILTER_LZMA1 } );
        self . _comp = lzma . LZMACompressor ( lzma . FORMAT_RAW , filters = [;
        lzma . _decode_filter_properties ( lzma . FILTER_LZMA1 , props );
        ] );
        return  struct . pack ( "<BBH" , 9 , 4 , len ( props ) ) + props;
        pub fn compress ( &self, data )  {
        if self . _comp is None /* Option */ {
        return  self . _init ( ) + self . _comp . compress ( data );
        return  self . _comp . compress ( data );
        pub fn flush ( self )  {
        if self . _comp is None /* Option */ {
        return  self . _init ( ) + self . _comp . flush ( );
        return  self . _comp . flush ( );
        class LZMADecompressor ;
        pub fn __init__ ( self )  {
        self . _decomp = None /* Option */;
        self . _unconsumed = b "";
        self . eof = false;
        pub fn decompress ( &self, data )  {
        if self . _decomp is None /* Option */ {
        self . _unconsumed + = data;
        if len ( self . _unconsumed ) <= 4 {
        return  b "";
        psize , = struct . unpack ( "<H" , self . _unconsumed [ 2 : 4 ] );
        if len ( self . _unconsumed ) <= 4 + psize {
        return  b "";
        self . _decomp = lzma . LZMADecompressor ( lzma . FORMAT_RAW , filters = [;
        lzma . _decode_filter_properties ( lzma . FILTER_LZMA1 ,;
        self . _unconsumed [ 4 : 4 + psize ] );
        ] );
        data = self . _unconsumed [ 4 + psize : ];
        del self . _unconsumed;
        result = self . _decomp . decompress ( data );
        self . eof = self . _decomp . eof;
        return  result;
        compressor_names = {;
        0 : "store" ,;
        1 : "shrink" ,;
        2 : "reduce" ,;
        3 : "reduce" ,;
        4 : "reduce" ,;
        5 : "reduce" ,;
        6 : "implode" ,;
        7 : "tokenize" ,;
        8 : "deflate" ,;
        9 : "deflate64" ,;
        10 : "implode" ,;
        12 : "bzip2" ,;
        14 : "lzma" ,;
        18 : "terse" ,;
        19 : "lz77" ,;
        97 : "wavpack" ,;
        98 : "ppmd" ,;
        };
        pub fn _check_compression ( compression )  {
        if compression == ZIP_STORED {
        // pass
        } else if compression == ZIP_DEFLATED {
        if !zlib {
        panic!("RuntimeError (");
        "Compression requires the (missing) zlib module" );
        } else if compression == ZIP_BZIP2 {
        if !bz2 {
        panic!("RuntimeError (");
        "Compression requires the (missing) bz2 module" );
        } else if compression == ZIP_LZMA {
        if !lzma {
        panic!("RuntimeError (");
        "Compression requires the (missing) lzma module" );
        } else {
        panic!("NotImplementedError ( "That compression method is !supported" )");
        pub fn _get_compressor ( compress_type , compresslevel = None /* Option */ )  {
        if compress_type == ZIP_DEFLATED {
        if compresslevel is !None /* Option */ {
        return  zlib . compressobj ( compresslevel , zlib . DEFLATED , -15 );
        return  zlib . compressobj ( zlib . Z_DEFAULT_COMPRESSION , zlib . DEFLATED , -15 );
        } else if compress_type == ZIP_BZIP2 {
        if compresslevel is !None /* Option */ {
        return  bz2 . BZ2Compressor ( compresslevel );
        return  bz2 . BZ2Compressor ( );
        } else if compress_type == ZIP_LZMA {
        return  LZMACompressor ( );
        } else {
        return;
        pub fn _get_decompressor ( compress_type )  {
        _check_compression ( compress_type );
        if compress_type == ZIP_STORED {
        return;
        } else if compress_type == ZIP_DEFLATED {
        return  zlib . decompressobj ( -15 );
        } else if compress_type == ZIP_BZIP2 {
        return  bz2 . BZ2Decompressor ( );
        } else if compress_type == ZIP_LZMA {
        return  LZMADecompressor ( );
        } else {
        descr = compressor_names . get ( compress_type );
        if descr {
        panic!("NotImplementedError ( "compression type %d (%s)" % ( compress_type , descr ) )");
        } else {
        panic!("NotImplementedError ( "compression type %d" % ( compress_type , ) )");
        class _SharedFile ;
        pub fn __init__ ( &self, file , pos , close , lock , writing )  {
        self . _file = file;
        self . _pos = pos;
        self . _close = close;
        self . _lock = lock;
        self . _writing = writing;
        self . seekable = file . seekable;
        pub fn tell ( self )  {
        return  self . _pos;
        pub fn seek ( &self, offset , whence = 0 )  {
        // with scope: self . _lock  {
        if self . _writing ( ) {
        panic!("ValueError ( "Can't reposition in the ZIP file while "");
        "there == an open writing handle on it. ";
        "Close the writing handle before trying to read." );
        self . _file . seek ( offset , whence );
        self . _pos = self . _file . tell ( );
        return  self . _pos;
        pub fn read ( &self, n = -1 )  {
        // with scope: self . _lock  {
        if self . _writing ( ) {
        panic!("ValueError ( "Can't read from the ZIP file while there "");
        "is an open writing handle on it. ";
        "Close the writing handle before trying to read." );
        self . _file . seek ( self . _pos );
        data = self . _file . read ( n );
        self . _pos = self . _file . tell ( );
        return  data;
        pub fn close ( self )  {
        if self . _file is !None /* Option */ {
        fileobj = self . _file;
        self . _file = None /* Option */;
        self . _close ( fileobj );
        class _Tellable ;
        pub fn __init__ ( &self, fp )  {
        self . fp = fp;
        self . offset = 0;
        pub fn write ( &self, data )  {
        n = self . fp . write ( data );
        self . offset + = n;
        return  n;
        pub fn tell ( self )  {
        return  self . offset;
        pub fn flush ( self )  {
        self . fp . flush ( );
        pub fn close ( self )  {
        self . fp . close ( );
        class ZipExtFile ( io . BufferedIOBase ) ;
        "File-like object for reading an archive member.
       Is returned by ZipFile.open().
    ";
        MAX_N = 1 < < 31 - 1;
        MIN_READ_SIZE = 4096;
        MAX_SEEK_READ = 1 < < 24;
        pub fn __init__ ( &self, fileobj , mode , zipinfo , pwd = None /* Option */ , {
        close_fileobj = false ) ;
        self . _fileobj = fileobj;
        self . _pwd = pwd;
        self . _close_fileobj = close_fileobj;
        self . _compress_type = zipinfo . compress_type;
        self . _compress_left = zipinfo . compress_size;
        self . _left = zipinfo . file_size;
        self . _decompressor = _get_decompressor ( self . _compress_type );
        self . _eof = false;
        self . _readbuffer = b "";
        self . _offset = 0;
        self . newlines = None /* Option */;
        self . mode = mode;
        self . name = zipinfo . filename;
        if hasattr ( zipinfo , "CRC" ) {
        self . _expected_crc = zipinfo . CRC;
        self . _running_crc = crc32 ( b "" );
        } else {
        self . _expected_crc = None /* Option */;
        self . _seekable = false;
        // try {
        if fileobj . seekable ( ) {
        self . _orig_compress_start = fileobj . tell ( );
        self . _orig_compress_size = zipinfo . compress_size;
        self . _orig_file_size = zipinfo . file_size;
        self . _orig_start_crc = self . _running_crc;
        self . _seekable = true;
        // } catch  AttributeError  {
        // pass
        self . _decrypter = None /* Option */;
        if pwd {
        if zipinfo . flag_bits & _MASK_USE_DATA_DESCRIPTOR {
        check_byte = ( zipinfo . _raw_time > > 8 ) & 0x ff;
        } else {
        check_byte = ( zipinfo . CRC > > 24 ) & 0x ff;
        h = self . _init_decrypter ( );
        if h != check_byte {
        panic!("RuntimeError ( "Bad password for file %r" % zipinfo . orig_filename )");
        pub fn _init_decrypter ( self )  {
        self . _decrypter = _ZipDecrypter ( self . _pwd );
        header = self . _fileobj . read ( 12 );
        self . _compress_left - = 12;
        return  self . _decrypter ( header ) [ 11 ];
        pub fn __repr__ ( self )  {
        result = [ "<%s.%s" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ) ];
        if !self . closed {
        result . append ( " name=%r mode=%r" % ( self . name , self . mode ) );
        if self . _compress_type != ZIP_STORED {
        result . append ( " compress_type=%s" %;
        compressor_names . get ( self . _compress_type ,;
        self . _compress_type ) );
        } else {
        result . append ( " [closed]" );
        result . append ( ">" );
        return  "" . join ( result );
        pub fn readline ( &self, limit = -1 )  {
        "Read && return a line from the stream.

        If limit == specified, at most limit bytes will be read.
        ";
        if limit < 0 {
        i = self . _readbuffer . find ( b "\n" , self . _offset ) + 1;
        if i > 0 {
        line = self . _readbuffer [ self . _offset : i ];
        self . _offset = i;
        return  line;
        return  io . BufferedIOBase . readline ( self , limit );
        pub fn peek ( &self, n = 1 )  {
        "Returns buffered bytes without advancing the position.";
        if n > len ( self . _readbuffer ) - self . _offset {
        chunk = self . read ( n );
        if len ( chunk ) > self . _offset {
        self . _readbuffer = chunk + self . _readbuffer [ self . _offset : ];
        self . _offset = 0;
        } else {
        self . _offset - = len ( chunk );
        return  self . _readbuffer [ self . _offset : self . _offset + 512 ];
        pub fn readable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  true;
        pub fn read ( &self, n = -1 )  {
        "Read && return up to n bytes.
        If the argument == omitted, None /* Option */, || negative, data == read && returned until EOF == reached.
        ";
        if self . closed {
        panic!("ValueError ( "read from closed file." )");
        if n is None /* Option */ || n < 0 {
        buf = self . _readbuffer [ self . _offset : ];
        self . _readbuffer = b "";
        self . _offset = 0;
        while !self . _eof  {
        buf + = self . _read1 ( self . MAX_N );
        return  buf;
        end = n + self . _offset;
        if end < len ( self . _readbuffer ) {
        buf = self . _readbuffer [ self . _offset : end ];
        self . _offset = end;
        return  buf;
        n = end - len ( self . _readbuffer );
        buf = self . _readbuffer [ self . _offset : ];
        self . _readbuffer = b "";
        self . _offset = 0;
        while n > 0 && !self . _eof  {
        data = self . _read1 ( n );
        if n < len ( data ) {
        self . _readbuffer = data;
        self . _offset = n;
        buf + = data [ : n ];
        break;
        buf + = data;
        n - = len ( data );
        return  buf;
        pub fn _update_crc ( &self, newdata )  {
        if self . _expected_crc is None /* Option */ {
        return;
        self . _running_crc = crc32 ( newdata , self . _running_crc );
        if self . _eof && self . _running_crc != self . _expected_crc {
        panic!("BadZipFile ( "Bad CRC-32 for file %r" % self . name )");
        pub fn read1 ( &self, n )  {
        "Read up to n bytes with at most one read() system call.";
        if n is None /* Option */ || n < 0 {
        buf = self . _readbuffer [ self . _offset : ];
        self . _readbuffer = b "";
        self . _offset = 0;
        while !self . _eof  {
        data = self . _read1 ( self . MAX_N );
        if data {
        buf + = data;
        break;
        return  buf;
        end = n + self . _offset;
        if end < len ( self . _readbuffer ) {
        buf = self . _readbuffer [ self . _offset : end ];
        self . _offset = end;
        return  buf;
        n = end - len ( self . _readbuffer );
        buf = self . _readbuffer [ self . _offset : ];
        self . _readbuffer = b "";
        self . _offset = 0;
        if n > 0 {
        while !self . _eof  {
        data = self . _read1 ( n );
        if n < len ( data ) {
        self . _readbuffer = data;
        self . _offset = n;
        buf + = data [ : n ];
        break;
        if data {
        buf + = data;
        break;
        return  buf;
        pub fn _read1 ( &self, n )  {
        if self . _eof || n <= 0 {
        return  b "";
        if self . _compress_type == ZIP_DEFLATED {
        data = self . _decompressor . unconsumed_tail;
        if n > len ( data ) {
        data + = self . _read2 ( n - len ( data ) );
        } else {
        data = self . _read2 ( n );
        if self . _compress_type == ZIP_STORED {
        self . _eof = self . _compress_left <= 0;
        } else if self . _compress_type == ZIP_DEFLATED {
        n = max ( n , self . MIN_READ_SIZE );
        data = self . _decompressor . decompress ( data , n );
        self . _eof = ( self . _decompressor . eof or;
        self . _compress_left <= 0 and;
        not self . _decompressor . unconsumed_tail );
        if self . _eof {
        data + = self . _decompressor . flush ( );
        } else {
        data = self . _decompressor . decompress ( data );
        self . _eof = self . _decompressor . eof || self . _compress_left <= 0;
        data = data [ : self . _left ];
        self . _left - = len ( data );
        if self . _left <= 0 {
        self . _eof = true;
        self . _update_crc ( data );
        return  data;
        pub fn _read2 ( &self, n )  {
        if self . _compress_left <= 0 {
        return  b "";
        n = max ( n , self . MIN_READ_SIZE );
        n = min ( n , self . _compress_left );
        data = self . _fileobj . read ( n );
        self . _compress_left - = len ( data );
        if !data {
        panic!("EOFError");
        if self . _decrypter is !None /* Option */ {
        data = self . _decrypter ( data );
        return  data;
        pub fn close ( self )  {
        // try {
        if self . _close_fileobj {
        self . _fileobj . close ( );
        // } finally {
        super ( ) . close ( );
        pub fn seekable ( self )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        return  self . _seekable;
        pub fn seek ( &self, offset , whence = 0 )  {
        if self . closed {
        panic!("ValueError ( "seek on closed file." )");
        if !self . _seekable {
        panic!("io . UnsupportedOperation ( "underlying stream is !seekable" )");
        curr_pos = self . tell ( );
        if whence == 0 {
        new_pos = offset;
        } else if whence == 1 {
        new_pos = curr_pos + offset;
        } else if whence == 2 {
        new_pos = self . _orig_file_size + offset;
        } else {
        panic!("ValueError ( "whence must be os.SEEK_SET (0), "");
        "os.SEEK_CUR (1), || os.SEEK_END (2)" );
        if new_pos > self . _orig_file_size {
        new_pos = self . _orig_file_size;
        if new_pos < 0 {
        new_pos = 0;
        read_offset = new_pos - curr_pos;
        buff_offset = read_offset + self . _offset;
        if buff_offset >= 0 && buff_offset < len ( self . _readbuffer ) {
        self . _offset = buff_offset;
        read_offset = 0;
        } else if read_offset < 0 {
        self . _fileobj . seek ( self . _orig_compress_start );
        self . _running_crc = self . _orig_start_crc;
        self . _compress_left = self . _orig_compress_size;
        self . _left = self . _orig_file_size;
        self . _readbuffer = b "";
        self . _offset = 0;
        self . _decompressor = _get_decompressor ( self . _compress_type );
        self . _eof = false;
        read_offset = new_pos;
        if self . _decrypter is !None /* Option */ {
        self . _init_decrypter ( );
        while read_offset > 0  {
        read_len = min ( self . MAX_SEEK_READ , read_offset );
        self . read ( read_len );
        read_offset - = read_len;
        return  self . tell ( );
        pub fn tell ( self )  {
        if self . closed {
        panic!("ValueError ( "tell on closed file." )");
        if !self . _seekable {
        panic!("io . UnsupportedOperation ( "underlying stream is !seekable" )");
        filepos = self . _orig_file_size - self . _left - len ( self . _readbuffer ) + self . _offset;
        return  filepos;
        class _ZipWriteFile ( io . BufferedIOBase ) ;
        pub fn __init__ ( &self, zf , zinfo , zip64 )  {
        self . _zinfo = zinfo;
        self . _zip64 = zip64;
        self . _zipfile = zf;
        self . _compressor = _get_compressor ( zinfo . compress_type ,;
        zinfo . _compresslevel );
        self . _file_size = 0;
        self . _compress_size = 0;
        self . _crc = 0;
        @ property;
        pub fn _fileobj ( self )  {
        return  self . _zipfile . fp;
        pub fn writable ( self )  {
        return  true;
        pub fn write ( &self, data )  {
        if self . closed {
        panic!("ValueError ( "I/O operation on closed file." )");
        if isinstance ( data , ( bytes , bytearray ) ) {
        nbytes = len ( data );
        } else {
        data = memoryview ( data );
        nbytes = data . nbytes;
        self . _file_size + = nbytes;
        self . _crc = crc32 ( data , self . _crc );
        if self . _compressor {
        data = self . _compressor . compress ( data );
        self . _compress_size + = len ( data );
        self . _fileobj . write ( data );
        return  nbytes;
        pub fn close ( self )  {
        if self . closed {
        return;
        // try {
        super ( ) . close ( );
        if self . _compressor {
        buf = self . _compressor . flush ( );
        self . _compress_size + = len ( buf );
        self . _fileobj . write ( buf );
        self . _zinfo . compress_size = self . _compress_size;
        } else {
        self . _zinfo . compress_size = self . _file_size;
        self . _zinfo . CRC = self . _crc;
        self . _zinfo . file_size = self . _file_size;
        if !self . _zip64 {
        if self . _file_size > ZIP64_LIMIT {
        panic!("RuntimeError ( "File size too large, try using force_zip64" )");
        if self . _compress_size > ZIP64_LIMIT {
        panic!("RuntimeError ( "Compressed size too large, try using force_zip64" )");
        if self . _zinfo . flag_bits & _MASK_USE_DATA_DESCRIPTOR {
        fmt = "<LLQQ" if self . _zip64 else "<LLLL";
        self . _fileobj . write ( struct . pack ( fmt , _DD_SIGNATURE , self . _zinfo . CRC ,;
        self . _zinfo . compress_size , self . _zinfo . file_size ) );
        self . _zipfile . start_dir = self . _fileobj . tell ( );
        } else {
        self . _zipfile . start_dir = self . _fileobj . tell ( );
        self . _fileobj . seek ( self . _zinfo . header_offset );
        self . _fileobj . write ( self . _zinfo . FileHeader ( self . _zip64 ) );
        self . _fileobj . seek ( self . _zipfile . start_dir );
        self . _zipfile . filelist . append ( self . _zinfo );
        self . _zipfile . NameToInfo [ self . _zinfo . filename ] = self . _zinfo;
        // } finally {
        self . _zipfile . _writing = false;
        class ZipFile ;
        " Class with methods to open, read, write, close, list zip files.

    z = ZipFile(file, mode="r", compression=ZIP_STORED, allowZip64=true,
                compresslevel=None /* Option */)

    file: Either the path to the file, || a file-like object.
          If it == a path, the file will be opened && closed by ZipFile.
    mode: The mode can be either read 'r', write 'w', exclusive create 'x',
          || append 'a'.
    compression: ZIP_STORED (no compression), ZIP_DEFLATED (requires zlib),
                 ZIP_BZIP2 (requires bz2) || ZIP_LZMA (requires lzma).
    allowZip64: if true ZipFile will create files with ZIP64 extensions when
                needed, otherwise it will raise an exception when this would
                be necessary.
    compresslevel: None /* Option */ (default for the given compression type) || an integer
                   specifying the level to pass to the compressor.
                   When using ZIP_STORED || ZIP_LZMA this keyword has no effect.
                   When using ZIP_DEFLATED integers 0 through 9 are accepted.
                   When using ZIP_BZIP2 integers 1 through 9 are accepted.

    ";
        fp = None /* Option */;
        _windows_illegal_name_trans_table = None /* Option */;
        pub fn __init__ ( &self, file , mode = "r" , compression = ZIP_STORED , allowZip64 = true , {
        compresslevel = None /* Option */ , * , strict_timestamps = true , metadata_encoding = None /* Option */ ) ;
        "Open the ZIP file with mode read 'r', write 'w', exclusive create 'x',
        || append 'a'.";
        if mode !in ( "r" , "w" , "x" , "a" ) {
        panic!("ValueError ( "ZipFile requires mode 'r', 'w', 'x', || 'a'" )");
        _check_compression ( compression );
        self . _allowZip64 = allowZip64;
        self . _didModify = false;
        self . debug = 0;
        self . NameToInfo = { };
        self . filelist = [ ];
        self . compression = compression;
        self . compresslevel = compresslevel;
        self . mode = mode;
        self . pwd = None /* Option */;
        self . _comment = b "";
        self . _strict_timestamps = strict_timestamps;
        self . metadata_encoding = metadata_encoding;
        if self . metadata_encoding && mode != "r" {
        panic!("ValueError (");
        "metadata_encoding == only supported for reading files" );
        if isinstance ( file , os . PathLike ) {
        file = os . fspath ( file );
        if isinstance ( file , str ) {
        self . _filePassed = 0;
        self . filename = file;
        modeDict = { "r" : "rb" , "w" : "w+b" , "x" : "x+b" , "a" : "r+b" ,;
        "r+b" : "w+b" , "w+b" : "wb" , "x+b" : "xb" };
        filemode = modeDict [ mode ];
        while true  {
        // try {
        self . fp = io . open ( file , filemode );
        // } catch  OSError  {
        if filemode in modeDict {
        filemode = modeDict [ filemode ];
        continue;
        panic!("");
        break;
        } else {
        self . _filePassed = 1;
        self . fp = file;
        self . filename = getattr ( file , "name" , None /* Option */ );
        self . _fileRefCnt = 1;
        self . _lock = threading . RLock ( );
        self . _seekable = true;
        self . _writing = false;
        // try {
        if mode == "r" {
        self . _RealGetContents ( );
        } else if mode in ( "w" , "x" ) {
        self . _didModify = true;
        // try {
        self . start_dir = self . fp . tell ( );
        // } catch  ( AttributeError , OSError )  {
        self . fp = _Tellable ( self . fp );
        self . start_dir = 0;
        self . _seekable = false;
        } else {
        // try {
        self . fp . seek ( self . start_dir );
        // } catch  ( AttributeError , OSError )  {
        self . _seekable = false;
        } else if mode == "a" {
        // try {
        self . _RealGetContents ( );
        self . fp . seek ( self . start_dir );
        // } catch  BadZipFile  {
        self . fp . seek ( 0 , 2 );
        self . _didModify = true;
        self . start_dir = self . fp . tell ( );
        } else {
        panic!("ValueError ( "Mode must be 'r', 'w', 'x', || 'a'" )");
        // } catch   {
        fp = self . fp;
        self . fp = None /* Option */;
        self . _fpclose ( fp );
        panic!("");
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, type , value , traceback )  {
        self . close ( );
        pub fn __repr__ ( self )  {
        result = [ "<%s.%s" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ) ];
        if self . fp is !None /* Option */ {
        if self . _filePassed {
        result . append ( " file=%r" % self . fp );
        } else if self . filename is !None /* Option */ {
        result . append ( " filename=%r" % self . filename );
        result . append ( " mode=%r" % self . mode );
        } else {
        result . append ( " [closed]" );
        result . append ( ">" );
        return  "" . join ( result );
        pub fn _RealGetContents ( self )  {
        "Read in the table of contents for the ZIP file.";
        fp = self . fp;
        // try {
        endrec = _EndRecData ( fp );
        // } catch  OSError  {
        panic!("BadZipFile ( "File is !a zip file" )");
        if !endrec {
        panic!("BadZipFile ( "File is !a zip file" )");
        if self . debug > 1 {
        println!( endrec );
        size_cd = endrec [ _ECD_SIZE ];
        offset_cd = endrec [ _ECD_OFFSET ];
        self . _comment = endrec [ _ECD_COMMENT ];
        concat = endrec [ _ECD_LOCATION ] - size_cd - offset_cd;
        if endrec [ _ECD_SIGNATURE ] == stringEndArchive64 {
        concat - = ( sizeEndCentDir64 + sizeEndCentDir64Locator );
        if self . debug > 2 {
        inferred = concat + offset_cd;
        println!( "given, inferred, offset" , offset_cd , inferred , concat );
        self . start_dir = offset_cd + concat;
        if self . start_dir < 0 {
        panic!("BadZipFile ( "Bad offset for central directory" )");
        fp . seek ( self . start_dir , 0 );
        data = fp . read ( size_cd );
        fp = io . BytesIO ( data );
        total = 0;
        while total < size_cd  {
        centdir = fp . read ( sizeCentralDir );
        if len ( centdir ) != sizeCentralDir {
        panic!("BadZipFile ( "Truncated central directory" )");
        centdir = struct . unpack ( structCentralDir , centdir );
        if centdir [ _CD_SIGNATURE ] != stringCentralDir {
        panic!("BadZipFile ( "Bad magic number for central directory" )");
        if self . debug > 2 {
        println!( centdir );
        filename = fp . read ( centdir [ _CD_FILENAME_LENGTH ] );
        flags = centdir [ _CD_FLAG_BITS ];
        if flags & _MASK_UTF_FILENAME {
        filename = filename . decode ( "utf-8" );
        } else {
        filename = filename . decode ( self . metadata_encoding || "cp437" );
        x = ZipInfo ( filename );
        x . extra = fp . read ( centdir [ _CD_EXTRA_FIELD_LENGTH ] );
        x . comment = fp . read ( centdir [ _CD_COMMENT_LENGTH ] );
        x . header_offset = centdir [ _CD_LOCAL_HEADER_OFFSET ];
        ( x . create_version , x . create_system , x . extract_version , x . reserved ,;
        x . flag_bits , x . compress_type , t , d ,;
        x . CRC , x . compress_size , x . file_size ) = centdir [ 1 : 12 ];
        if x . extract_version > MAX_EXTRACT_VERSION {
        panic!("NotImplementedError ( "zip file version %.1f" %");
        ( x . extract_version / 10 ) );
        x . volume , x . internal_attr , x . external_attr = centdir [ 15 : 18 ];
        x . _raw_time = t;
        x . date_time = ( ( d > > 9 ) + 1980 , ( d > > 5 ) & 0x F , d & 0x1 F ,;
        t > > 11 , ( t > > 5 ) & 0x3 F , ( t & 0x1 F ) * 2 );
        x . _decodeExtra ( );
        x . header_offset = x . header_offset + concat;
        self . filelist . append ( x );
        self . NameToInfo [ x . filename ] = x;
        total = ( total + sizeCentralDir + centdir [ _CD_FILENAME_LENGTH ];
        + centdir [ _CD_EXTRA_FIELD_LENGTH ];
        + centdir [ _CD_COMMENT_LENGTH ] );
        if self . debug > 2 {
        println!( "total" , total );
        end_offset = self . start_dir;
        for zinfo in sorted ( self . filelist ,.iter() {
        key = |zinfo | {  zinfo . header_offset , };
        reverse = true ) ;
        zinfo . _end_offset = end_offset;
        end_offset = zinfo . header_offset;
        pub fn namelist ( self )  {
        "Return a list of file names in the archive.";
        return  [ data . filename for data in self . filelist ];
        pub fn infolist ( self )  {
        "Return a list of class ZipInfo instances for files in the
        archive.";
        return  self . filelist;
        pub fn printdir ( &self, file = None /* Option */ )  {
        "Print a table of contents for the zip file.";
        println!( "%-46s %19s %12s" % ( "File Name" , "Modified    " , "Size" ) );
        file = file );
        for zinfo in self . filelist .iter() {
        date = "%d-%02d-%02d %02d:%02d:%02d" % zinfo . date_time [ : 6 ];
        println!( "%-46s %s %12d" % ( zinfo . filename , date , zinfo . file_size ) );
        file = file );
        pub fn testzip ( self )  {
        "Read all the files && check the CRC.";
        chunk_size = 2 ** 20;
        for zinfo in self . filelist .iter() {
        // try {
        // with scope: self . open ( zinfo . filename , "r" ) as f  {
        while f . read ( chunk_size )  {
        // pass
        // } catch  BadZipFile  {
        return  zinfo . filename;
        pub fn getinfo ( &self, name )  {
        "Return the instance of ZipInfo given 'name'.";
        info = self . NameToInfo . get ( name );
        if info is None /* Option */ {
        panic!("KeyError (");
        "There == no item named %r in the archive" % name );
        return  info;
        pub fn setpassword ( &self, pwd )  {
        "Set default password for encrypted files.";
        if pwd && !isinstance ( pwd , bytes ) {
        panic!("TypeError ( "pwd: expected bytes, got %s" % type ( pwd ) . __name__ )");
        if pwd {
        self . pwd = pwd;
        } else {
        self . pwd = None /* Option */;
        @ property;
        pub fn comment ( self )  {
        "The comment text associated with the ZIP file.";
        return  self . _comment;
        @ comment . setter;
        pub fn comment ( &self, comment )  {
        if !isinstance ( comment , bytes ) {
        panic!("TypeError ( "comment: expected bytes, got %s" % type ( comment ) . __name__ )");
        if len ( comment ) > ZIP_MAX_COMMENT {
        import warnings;
        warnings . warn ( "Archive comment == too long; truncating to %d bytes";
        % ZIP_MAX_COMMENT , stacklevel = 2 );
        comment = comment [ : ZIP_MAX_COMMENT ];
        self . _comment = comment;
        self . _didModify = true;
        pub fn read ( &self, name , pwd = None /* Option */ )  {
        "Return file bytes for name.";
        // with scope: self . open ( name , "r" , pwd ) as fp  {
        return  fp . read ( );
        pub fn open ( &self, name , mode = "r" , pwd = None /* Option */ , * , force_zip64 = false )  {
        "Return file-like object for 'name'.

        name == a string for the file name within the ZIP file, || a ZipInfo
        object.

        mode should be 'r' to read a file already in the ZIP file, || 'w' to
        write to a file newly added to the archive.

        pwd == the password to decrypt files (only used for reading).

        When writing, if the file size == !known in advance but may exceed
        2 GiB, pass force_zip64 to use the ZIP64 format, which can handle large
        files.  If the size == known in advance, it == best to pass a ZipInfo
        instance for name, with zinfo.file_size set.
        ";
        if mode !in { "r" , "w" } {
        panic!("ValueError ( "open() requires mode "r" || "w"" )");
        if pwd && ( mode == "w" ) {
        panic!("ValueError ( "pwd is only supported for reading files" )");
        if !self . fp {
        panic!("ValueError (");
        "Attempt to use ZIP archive that was already closed" );
        if isinstance ( name , ZipInfo ) {
        zinfo = name;
        } else if mode == "w" {
        zinfo = ZipInfo ( name );
        zinfo . compress_type = self . compression;
        zinfo . _compresslevel = self . compresslevel;
        } else {
        zinfo = self . getinfo ( name );
        if mode == "w" {
        return  self . _open_to_write ( zinfo , force_zip64 = force_zip64 );
        if self . _writing {
        panic!("ValueError ( "Can't read from the ZIP file while there "");
        "is an open writing handle on it. ";
        "Close the writing handle before trying to read." );
        self . _fileRefCnt + = 1;
        zef_file = _SharedFile ( self . fp , zinfo . header_offset ,;
        self . _fpclose , self . _lock , lambda : self . _writing );
        // try {
        fheader = zef_file . read ( sizeFileHeader );
        if len ( fheader ) != sizeFileHeader {
        panic!("BadZipFile ( "Truncated file header" )");
        fheader = struct . unpack ( structFileHeader , fheader );
        if fheader [ _FH_SIGNATURE ] != stringFileHeader {
        panic!("BadZipFile ( "Bad magic number for file header" )");
        fname = zef_file . read ( fheader [ _FH_FILENAME_LENGTH ] );
        if fheader [ _FH_EXTRA_FIELD_LENGTH ] {
        zef_file . read ( fheader [ _FH_EXTRA_FIELD_LENGTH ] );
        if zinfo . flag_bits & _MASK_COMPRESSED_PATCH {
        panic!("NotImplementedError ( "compressed patched data (flag bit 5)" )");
        if zinfo . flag_bits & _MASK_STRONG_ENCRYPTION {
        panic!("NotImplementedError ( "strong encryption (flag bit 6)" )");
        if fheader [ _FH_GENERAL_PURPOSE_FLAG_BITS ] & _MASK_UTF_FILENAME {
        fname_str = fname . decode ( "utf-8" );
        } else {
        fname_str = fname . decode ( self . metadata_encoding || "cp437" );
        if fname_str != zinfo . orig_filename {
        panic!("BadZipFile (");
        "File name in directory %r && header %r differ.";
        % ( zinfo . orig_filename , fname ) );
        if ( zinfo . _end_offset is !None /* Option */ and {
        zef_file . tell ( ) + zinfo . compress_size > zinfo . _end_offset ) ;
        panic!("BadZipFile ( f "Overlapped entries: {zinfo.orig_filename!r} (possible zip bomb)" )");
        is_encrypted = zinfo . flag_bits & _MASK_ENCRYPTED;
        if is_encrypted {
        if !pwd {
        pwd = self . pwd;
        if pwd && !isinstance ( pwd , bytes ) {
        panic!("TypeError ( "pwd: expected bytes, got %s" % type ( pwd ) . __name__ )");
        if !pwd {
        panic!("RuntimeError ( "File %r is encrypted, password "");
        "required for extraction" % name );
        } else {
        pwd = None /* Option */;
        return  ZipExtFile ( zef_file , mode , zinfo , pwd , true );
        // } catch   {
        zef_file . close ( );
        panic!("");
        pub fn _open_to_write ( &self, zinfo , force_zip64 = false )  {
        if force_zip64 && !self . _allowZip64 {
        panic!("ValueError (");
        "force_zip64 == true, but allowZip64 was false when opening ";
        "the ZIP file.";
        );
        if self . _writing {
        panic!("ValueError ( "Can't write to the ZIP file while there is "");
        "another write handle open on it. ";
        "Close the first handle before opening another." );
        zinfo . compress_size = 0;
        zinfo . CRC = 0;
        zinfo . flag_bits = 0x00;
        if zinfo . compress_type == ZIP_LZMA {
        zinfo . flag_bits | = _MASK_COMPRESS_OPTION_1;
        if !self . _seekable {
        zinfo . flag_bits | = _MASK_USE_DATA_DESCRIPTOR;
        if !zinfo . external_attr {
        zinfo . external_attr = 0 o600 < < 16;
        zip64 = force_zip64 || ( zinfo . file_size * 1.05 > ZIP64_LIMIT );
        if !self . _allowZip64 && zip64 {
        panic!("LargeZipFile ( "Filesize would require ZIP64 extensions" )");
        if self . _seekable {
        self . fp . seek ( self . start_dir );
        zinfo . header_offset = self . fp . tell ( );
        self . _writecheck ( zinfo );
        self . _didModify = true;
        self . fp . write ( zinfo . FileHeader ( zip64 ) );
        self . _writing = true;
        return  _ZipWriteFile ( self , zinfo , zip64 );
        pub fn extract ( &self, member , path = None /* Option */ , pwd = None /* Option */ )  {
        "Extract a member from the archive to the current working directory,
           using its full name. Its file information == extracted as accurately
           as possible. `member' may be a filename || a ZipInfo object. You can
           specify a different directory using `path'.
        ";
        if path is None /* Option */ {
        path = os . getcwd ( );
        } else {
        path = os . fspath ( path );
        return  self . _extract_member ( member , path , pwd );
        pub fn extractall ( &self, path = None /* Option */ , members = None /* Option */ , pwd = None /* Option */ )  {
        "Extract all members from the archive to the current working
           directory. `path' specifies a different directory to extract to.
           `members' == optional && must be a subset of the list returned
           by namelist().
        ";
        if members is None /* Option */ {
        members = self . namelist ( );
        if path is None /* Option */ {
        path = os . getcwd ( );
        } else {
        path = os . fspath ( path );
        for zipinfo in members .iter() {
        self . _extract_member ( zipinfo , path , pwd );
        @ classmethod;
        pub fn _sanitize_windows_name ( cls , arcname , pathsep )  {
        "Replace bad characters && remove trailing dots from parts.";
        table = cls . _windows_illegal_name_trans_table;
        if !table {
        illegal = ":<>|"?*";
        table = str . maketrans ( illegal , "_" * len ( illegal ) );
        cls . _windows_illegal_name_trans_table = table;
        arcname = arcname . translate ( table );
        arcname = ( x . rstrip ( "." ) for x in arcname . split ( pathsep ) );
        arcname = pathsep . join ( x for x in arcname if x );
        return  arcname;
        pub fn _extract_member ( &self, member , targetpath , pwd )  {
        "Extract the ZipInfo object 'member' to a physical
           file on the path targetpath.
        ";
        if !isinstance ( member , ZipInfo ) {
        member = self . getinfo ( member );
        arcname = member . filename . replace ( "/" , os . path . sep );
        if os . path . altsep {
        arcname = arcname . replace ( os . path . altsep , os . path . sep );
        arcname = os . path . splitdrive ( arcname ) [ 1 ];
        invalid_path_parts = ( "" , os . path . curdir , os . path . pardir );
        arcname = os . path . sep . join ( x for x in arcname . split ( os . path . sep );
        if x !in invalid_path_parts ) {
        if os . path . sep == "\\" {
        arcname = self . _sanitize_windows_name ( arcname , os . path . sep );
        targetpath = os . path . join ( targetpath , arcname );
        targetpath = os . path . normpath ( targetpath );
        upperdirs = os . path . dirname ( targetpath );
        if upperdirs && !os . path . exists ( upperdirs ) {
        os . makedirs ( upperdirs );
        if member . is_dir ( ) {
        if !os . path . isdir ( targetpath ) {
        os . mkdir ( targetpath );
        return  targetpath;
        // with scope: self . open ( member , pwd = pwd ) as source , \ {
        open ( targetpath , "wb" ) as target ;
        shutil . copyfileobj ( source , target );
        return  targetpath;
        pub fn _writecheck ( &self, zinfo )  {
        "Check for errors before writing a file to the archive.";
        if zinfo . filename in self . NameToInfo {
        import warnings;
        warnings . warn ( "Duplicate name: %r" % zinfo . filename , stacklevel = 3 );
        if self . mode !in ( "w" , "x" , "a" ) {
        panic!("ValueError ( "write() requires mode 'w', 'x', || 'a'" )");
        if !self . fp {
        panic!("ValueError (");
        "Attempt to write ZIP archive that was already closed" );
        _check_compression ( zinfo . compress_type );
        if !self . _allowZip64 {
        requires_zip64 = None /* Option */;
        if len ( self . filelist ) >= ZIP_FILECOUNT_LIMIT {
        requires_zip64 = "Files count";
        } else if zinfo . file_size > ZIP64_LIMIT {
        requires_zip64 = "Filesize";
        } else if zinfo . header_offset > ZIP64_LIMIT {
        requires_zip64 = "Zipfile size";
        if requires_zip64 {
        panic!("LargeZipFile ( requires_zip64 +");
        " would require ZIP64 extensions" );
        pub fn write ( &self, filename , arcname = None /* Option */ , {
        compress_type = None /* Option */ , compresslevel = None /* Option */ ) ;
        "Put the bytes from filename into the archive under the name
        arcname.";
        if !self . fp {
        panic!("ValueError (");
        "Attempt to write to ZIP archive that was already closed" );
        if self . _writing {
        panic!("ValueError (");
        "Can't write to ZIP archive while an open writing handle exists";
        );
        zinfo = ZipInfo . from_file ( filename , arcname ,;
        strict_timestamps = self . _strict_timestamps );
        if zinfo . is_dir ( ) {
        zinfo . compress_size = 0;
        zinfo . CRC = 0;
        self . mkdir ( zinfo );
        } else {
        if compress_type is !None /* Option */ {
        zinfo . compress_type = compress_type;
        } else {
        zinfo . compress_type = self . compression;
        if compresslevel is !None /* Option */ {
        zinfo . _compresslevel = compresslevel;
        } else {
        zinfo . _compresslevel = self . compresslevel;
        // with scope: open ( filename , "rb" ) as src , self . open ( zinfo , "w" ) as dest  {
        shutil . copyfileobj ( src , dest , 1024 * 8 );
        pub fn writestr ( &self, zinfo_or_arcname , data , {
        compress_type = None /* Option */ , compresslevel = None /* Option */ ) ;
        "Write a file into the archive.  The contents == 'data', which
        may be either a 'str' || a 'bytes' instance; if it == a 'str',
        it == encoded as UTF-8 first.
        'zinfo_or_arcname' == either a ZipInfo instance or
        the name of the file in the archive.";
        if isinstance ( data , str ) {
        data = data . encode ( "utf-8" );
        if !isinstance ( zinfo_or_arcname , ZipInfo ) {
        zinfo = ZipInfo ( filename = zinfo_or_arcname ,;
        date_time = time . localtime ( time . time ( ) ) [ : 6 ] );
        zinfo . compress_type = self . compression;
        zinfo . _compresslevel = self . compresslevel;
        if zinfo . filename [ -1 ] == "/" {
        zinfo . external_attr = 0 o40775 < < 16;
        zinfo . external_attr | = 0x10;
        } else {
        zinfo . external_attr = 0 o600 < < 16;
        } else {
        zinfo = zinfo_or_arcname;
        if !self . fp {
        panic!("ValueError (");
        "Attempt to write to ZIP archive that was already closed" );
        if self . _writing {
        panic!("ValueError (");
        "Can't write to ZIP archive while an open writing handle exists.";
        );
        if compress_type is !None /* Option */ {
        zinfo . compress_type = compress_type;
        if compresslevel is !None /* Option */ {
        zinfo . _compresslevel = compresslevel;
        zinfo . file_size = len ( data );
        // with scope: self . _lock  {
        // with scope: self . open ( zinfo , mode = "w" ) as dest  {
        dest . write ( data );
        pub fn mkdir ( &self, zinfo_or_directory_name , mode = 511 )  {
        "Creates a directory inside the zip archive.";
        if isinstance ( zinfo_or_directory_name , ZipInfo ) {
        zinfo = zinfo_or_directory_name;
        if !zinfo . is_dir ( ) {
        panic!("ValueError ( "The given ZipInfo does !describe a directory" )");
        } else if isinstance ( zinfo_or_directory_name , str ) {
        directory_name = zinfo_or_directory_name;
        if !directory_name . endswith ( "/" ) {
        directory_name + = "/";
        zinfo = ZipInfo ( directory_name );
        zinfo . compress_size = 0;
        zinfo . CRC = 0;
        zinfo . external_attr = ( ( 0 o40000 | mode ) & 0x FFFF ) < < 16;
        zinfo . file_size = 0;
        zinfo . external_attr | = 0x10;
        } else {
        panic!("TypeError ( "Expected type str || ZipInfo" )");
        // with scope: self . _lock  {
        if self . _seekable {
        self . fp . seek ( self . start_dir );
        zinfo . header_offset = self . fp . tell ( );
        if zinfo . compress_type == ZIP_LZMA {
        zinfo . flag_bits | = _MASK_COMPRESS_OPTION_1;
        self . _writecheck ( zinfo );
        self . _didModify = true;
        self . filelist . append ( zinfo );
        self . NameToInfo [ zinfo . filename ] = zinfo;
        self . fp . write ( zinfo . FileHeader ( false ) );
        self . start_dir = self . fp . tell ( );
        pub fn __del__ ( self )  {
        "Call the "close()" method in case the user forgot.";
        self . close ( );
        pub fn close ( self )  {
        "Close the file, && for mode 'w', 'x' && 'a' write the ending
        records.";
        if self . fp is None /* Option */ {
        return;
        if self . _writing {
        panic!("ValueError ( "Can't close the ZIP file while there is "");
        "an open writing handle on it. ";
        "Close the writing handle before closing the zip." );
        // try {
        if self . mode in ( "w" , "x" , "a" ) && self . _didModify {
        // with scope: self . _lock  {
        if self . _seekable {
        self . fp . seek ( self . start_dir );
        self . _write_end_record ( );
        // } finally {
        fp = self . fp;
        self . fp = None /* Option */;
        self . _fpclose ( fp );
        pub fn _write_end_record ( self )  {
        for zinfo in self . filelist .iter() {
        dt = zinfo . date_time;
        dosdate = ( dt [ 0 ] - 1980 ) < < 9 | dt [ 1 ] < < 5 | dt [ 2 ];
        dostime = dt [ 3 ] < < 11 | dt [ 4 ] < < 5 | ( dt [ 5 ] / / 2 );
        extra = [ ];
        if zinfo . file_size > ZIP64_LIMIT \ {
        or zinfo . compress_size > ZIP64_LIMIT ;
        extra . append ( zinfo . file_size );
        extra . append ( zinfo . compress_size );
        file_size = 0x ffffffff;
        compress_size = 0x ffffffff;
        } else {
        file_size = zinfo . file_size;
        compress_size = zinfo . compress_size;
        if zinfo . header_offset > ZIP64_LIMIT {
        extra . append ( zinfo . header_offset );
        header_offset = 0x ffffffff;
        } else {
        header_offset = zinfo . header_offset;
        extra_data = zinfo . extra;
        min_version = 0;
        if extra {
        extra_data = _strip_extra ( extra_data , ( 1 , ) );
        extra_data = struct . pack (;
        "<HH" + "Q" * len ( extra ) ,;
        1 , 8 * len ( extra ) , * extra ) + extra_data;
        min_version = ZIP64_VERSION;
        if zinfo . compress_type == ZIP_BZIP2 {
        min_version = max ( BZIP2_VERSION , min_version );
        } else if zinfo . compress_type == ZIP_LZMA {
        min_version = max ( LZMA_VERSION , min_version );
        extract_version = max ( min_version , zinfo . extract_version );
        create_version = max ( min_version , zinfo . create_version );
        filename , flag_bits = zinfo . _encodeFilenameFlags ( );
        centdir = struct . pack ( structCentralDir ,;
        stringCentralDir , create_version ,;
        zinfo . create_system , extract_version , zinfo . reserved ,;
        flag_bits , zinfo . compress_type , dostime , dosdate ,;
        zinfo . CRC , compress_size , file_size ,;
        len ( filename ) , len ( extra_data ) , len ( zinfo . comment ) ,;
        0 , zinfo . internal_attr , zinfo . external_attr ,;
        header_offset );
        self . fp . write ( centdir );
        self . fp . write ( filename );
        self . fp . write ( extra_data );
        self . fp . write ( zinfo . comment );
        pos2 = self . fp . tell ( );
        centDirCount = len ( self . filelist );
        centDirSize = pos2 - self . start_dir;
        centDirOffset = self . start_dir;
        requires_zip64 = None /* Option */;
        if centDirCount > ZIP_FILECOUNT_LIMIT {
        requires_zip64 = "Files count";
        } else if centDirOffset > ZIP64_LIMIT {
        requires_zip64 = "Central directory offset";
        } else if centDirSize > ZIP64_LIMIT {
        requires_zip64 = "Central directory size";
        if requires_zip64 {
        if !self . _allowZip64 {
        panic!("LargeZipFile ( requires_zip64 +");
        " would require ZIP64 extensions" );
        zip64endrec = struct . pack (;
        structEndArchive64 , stringEndArchive64 ,;
        44 , 45 , 45 , 0 , 0 , centDirCount , centDirCount ,;
        centDirSize , centDirOffset );
        self . fp . write ( zip64endrec );
        zip64locrec = struct . pack (;
        structEndArchive64Locator ,;
        stringEndArchive64Locator , 0 , pos2 , 1 );
        self . fp . write ( zip64locrec );
        centDirCount = min ( centDirCount , 0x FFFF );
        centDirSize = min ( centDirSize , 0x FFFFFFFF );
        centDirOffset = min ( centDirOffset , 0x FFFFFFFF );
        endrec = struct . pack ( structEndArchive , stringEndArchive ,;
        0 , 0 , centDirCount , centDirCount ,;
        centDirSize , centDirOffset , len ( self . _comment ) );
        self . fp . write ( endrec );
        self . fp . write ( self . _comment );
        if self . mode == "a" {
        self . fp . truncate ( );
        self . fp . flush ( );
        pub fn _fpclose ( &self, fp )  {
        assert self . _fileRefCnt > 0;
        self . _fileRefCnt - = 1;
        if !self . _fileRefCnt && !self . _filePassed {
        fp . close ( );
        class PyZipFile ( ZipFile ) ;
        "Class to create ZIP archives with Python library files && packages.";
        pub fn __init__ ( &self, file , mode = "r" , compression = ZIP_STORED , {
        allowZip64 = true , optimize = -1 ) ;
        ZipFile . __init__ ( self , file , mode = mode , compression = compression ,;
        allowZip64 = allowZip64 );
        self . _optimize = optimize;
        pub fn writepy ( &self, pathname , basename = "" , filterfunc = None /* Option */ )  {
        "Add all files from "pathname" to the ZIP archive.

        If pathname == a package directory, search the directory and
        all package subdirectories recursively for all *.py && enter
        the modules into the archive.  If pathname == a plain
        directory, listdir *.py && enter all modules.  Else, pathname
        must be a Python *.py file && the module will be put into the
        archive.  Added modules are always module.pyc.
        This method will compile the module.py into module.pyc if
        necessary.
        If filterfunc(pathname) == given, it == called with every argument.
        When it == false, the file || directory == skipped.
        ";
        pathname = os . fspath ( pathname );
        if filterfunc && !filterfunc ( pathname ) {
        if self . debug {
        label = "path" if os . path . isdir ( pathname ) else "file";
        println!( "%s %r skipped by filterfunc" % ( label , pathname ) );
        return;
        dir , name = os . path . split ( pathname );
        if os . path . isdir ( pathname ) {
        initname = os . path . join ( pathname , "__init__.py" );
        if os . path . isfile ( initname ) {
        if basename {
        basename = "%s/%s" % ( basename , name );
        } else {
        basename = name;
        if self . debug {
        println!( "Adding package in" , pathname , "as" , basename );
        fname , arcname = self . _get_codename ( initname [ 0 : -3 ] , basename );
        if self . debug {
        println!( "Adding" , arcname );
        self . write ( fname , arcname );
        dirlist = sorted ( os . listdir ( pathname ) );
        dirlist . remove ( "__init__.py" );
        for filename in dirlist .iter() {
        path = os . path . join ( pathname , filename );
        root , ext = os . path . splitext ( filename );
        if os . path . isdir ( path ) {
        if os . path . isfile ( os . path . join ( path , "__init__.py" ) ) {
        self . writepy ( path , basename ,;
        filterfunc = filterfunc );
        } else if ext == ".py" {
        if filterfunc && !filterfunc ( path ) {
        if self . debug {
        println!( "file %r skipped by filterfunc" % path );
        continue;
        fname , arcname = self . _get_codename ( path [ 0 : -3 ] ,;
        basename );
        if self . debug {
        println!( "Adding" , arcname );
        self . write ( fname , arcname );
        } else {
        if self . debug {
        println!( "Adding files from directory" , pathname );
        for filename in sorted ( os . listdir ( pathname ) ) .iter() {
        path = os . path . join ( pathname , filename );
        root , ext = os . path . splitext ( filename );
        if ext == ".py" {
        if filterfunc && !filterfunc ( path ) {
        if self . debug {
        println!( "file %r skipped by filterfunc" % path );
        continue;
        fname , arcname = self . _get_codename ( path [ 0 : -3 ] ,;
        basename );
        if self . debug {
        println!( "Adding" , arcname );
        self . write ( fname , arcname );
        } else {
        if pathname [ -3 { : ] != ".py" ; }
        panic!("RuntimeError (");
        "Files added with writepy() must end with ".py"" );
        fname , arcname = self . _get_codename ( pathname [ 0 : -3 ] , basename );
        if self . debug {
        println!( "Adding file" , arcname );
        self . write ( fname , arcname );
        pub fn _get_codename ( &self, pathname , basename )  {
        "Return (filename, archivename) for the path.

        Given a module name path, return the correct file path and
        archive name, compiling if necessary.  For example, given
        /python/lib/string, return (/python/lib/string.pyc, string).
        ";
        pub fn _compile ( file , optimize = -1 )  {
        import py_compile;
        if self . debug {
        println!( "Compiling" , file );
        // try {
        py_compile . compile ( file , doraise = true , optimize = optimize );
        // } catch  py_compile . PyCompileError as err  {
        println!( err . msg );
        return  false;
        return  true;
        file_py = pathname + ".py";
        file_pyc = pathname + ".pyc";
        pycache_opt0 = importlib . util . cache_from_source ( file_py , optimization = "" );
        pycache_opt1 = importlib . util . cache_from_source ( file_py , optimization = 1 );
        pycache_opt2 = importlib . util . cache_from_source ( file_py , optimization = 2 );
        if self . _optimize == -1 {
        if ( os . path . isfile ( file_pyc ) and {
        os . stat ( file_pyc ) . st_mtime >= os . stat ( file_py ) . st_mtime ) ;
        arcname = fname = file_pyc;
        } else if ( os . path . isfile ( pycache_opt0 ) and {
        os . stat ( pycache_opt0 ) . st_mtime >= os . stat ( file_py ) . st_mtime ) ;
        fname = pycache_opt0;
        arcname = file_pyc;
        } else if ( os . path . isfile ( pycache_opt1 ) and {
        os . stat ( pycache_opt1 ) . st_mtime >= os . stat ( file_py ) . st_mtime ) ;
        fname = pycache_opt1;
        arcname = file_pyc;
        } else if ( os . path . isfile ( pycache_opt2 ) and {
        os . stat ( pycache_opt2 ) . st_mtime >= os . stat ( file_py ) . st_mtime ) ;
        fname = pycache_opt2;
        arcname = file_pyc;
        } else {
        if _compile ( file_py ) {
        if sys . flags . optimize == 0 {
        fname = pycache_opt0;
        } else if sys . flags . optimize == 1 {
        fname = pycache_opt1;
        } else {
        fname = pycache_opt2;
        arcname = file_pyc;
        } else {
        fname = arcname = file_py;
        } else {
        if self . _optimize == 0 {
        fname = pycache_opt0;
        arcname = file_pyc;
        } else {
        arcname = file_pyc;
        if self . _optimize == 1 {
        fname = pycache_opt1;
        } else if self . _optimize == 2 {
        fname = pycache_opt2;
        } else {
        msg = "invalid value for 'optimize': {!r}" . format ( self . _optimize );
        panic!("ValueError ( msg )");
        if !( os . path . isfile ( fname ) and {
        os . stat ( fname ) . st_mtime >= os . stat ( file_py ) . st_mtime ) ;
        if !_compile ( file_py , optimize = self . _optimize ) {
        fname = arcname = file_py;
        archivename = os . path . split ( arcname ) [ 1 ];
        if basename {
        archivename = "%s/%s" % ( basename , archivename );
        return  ( fname , archivename );
        pub fn _parents ( path )  {
        "
    Given a path with elements separated by
    posixpath.sep, generate all parents of that path.

    >>> list(_parents('b/d'))
    ['b']
    >>> list(_parents('/b/d/'))
    ['/b']
    >>> list(_parents('b/d/f/'))
    ['b/d', 'b']
    >>> list(_parents('b'))
    []
    >>> list(_parents(''))
    []
    ";
        return  itertools . islice ( _ancestry ( path ) , 1 , None /* Option */ );
        pub fn _ancestry ( path )  {
        "
    Given a path with elements separated by
    posixpath.sep, generate all elements of that path

    >>> list(_ancestry('b/d'))
    ['b/d', 'b']
    >>> list(_ancestry('/b/d/'))
    ['/b/d', '/b']
    >>> list(_ancestry('b/d/f/'))
    ['b/d/f', 'b/d', 'b']
    >>> list(_ancestry('b'))
    ['b']
    >>> list(_ancestry(''))
    []
    ";
        path = path . rstrip ( posixpath . sep );
        while path && path != posixpath . sep  {
        yield path;
        path , tail = posixpath . split ( path );
        _dedupe = dict . fromkeys;
        "Deduplicate an iterable in original order";
        pub fn _difference ( minuend , subtrahend )  {
        "
    Return items in minuend !in subtrahend, retaining order
    with O(1) lookup.
    ";
        return  itertools . filterfalse ( set ( subtrahend ) . __contains__ , minuend );
        class CompleteDirs ( ZipFile ) ;
        "
    A ZipFile subclass that ensures that implied directories
    are always included in the namelist.
    ";
        @ staticmethod;
        pub fn _implied_dirs ( names )  {
        parents = itertools . chain . from_iterable ( map ( _parents , names ) );
        as_dirs = ( p + posixpath . sep for p in parents );
        return  _dedupe ( _difference ( as_dirs , names ) );
        pub fn namelist ( self )  {
        names = super ( CompleteDirs , self ) . namelist ( );
        return  names + list ( self . _implied_dirs ( names ) );
        pub fn _name_set ( self )  {
        return  set ( self . namelist ( ) );
        pub fn resolve_dir ( &self, name )  {
        "
        If the name represents a directory, return that name
        as a directory (with the trailing slash).
        ";
        names = self . _name_set ( );
        dirname = name + "/";
        dir_match = name !in names && dirname in names;
        return  dirname if dir_match else name;
        pub fn getinfo ( &self, name )  {
        "
        Supplement getinfo for implied dirs.
        ";
        // try {
        return  super ( ) . getinfo ( name );
        // } catch  KeyError  {
        if !name . endswith ( "/" ) || name !in self . _name_set ( ) {
        panic!("");
        return  ZipInfo ( filename = name );
        @ classmethod;
        pub fn make ( cls , source )  {
        "
        Given a source (filename || zipfile), return an
        appropriate CompleteDirs subclass.
        ";
        if isinstance ( source , CompleteDirs ) {
        return  source;
        if !isinstance ( source , ZipFile ) {
        return  cls ( source );
        if "r" !in source . mode {
        cls = CompleteDirs;
        source . __class__ = cls;
        return  source;
        class FastLookup ( CompleteDirs ) ;
        "
    ZipFile subclass to ensure implicit
    dirs exist && are resolved rapidly.
    ";
        pub fn namelist ( self )  {
        // with scope: contextlib . suppress ( AttributeError )  {
        return  self . __names;
        self . __names = super ( FastLookup , self ) . namelist ( );
        return  self . __names;
        pub fn _name_set ( self )  {
        // with scope: contextlib . suppress ( AttributeError )  {
        return  self . __lookup;
        self . __lookup = super ( FastLookup , self ) . _name_set ( );
        return  self . __lookup;
        pub fn _extract_text_encoding ( encoding = None /* Option */ , * args , ** kwargs )  {
        return  io . text_encoding ( encoding , 3 ) , args , kwargs;
        class Path ;
        "
    A pathlib-compatible interface for zip files.

    Consider a zip file with this structure::

        .
        ├── a.txt
        └── b
            ├── c.txt
            └── d
                └── e.txt

    >>> data = io.BytesIO()
    >>> zf = ZipFile(data, 'w')
    >>> zf.writestr('a.txt', 'content of a')
    >>> zf.writestr('b/c.txt', 'content of c')
    >>> zf.writestr('b/d/e.txt', 'content of e')
    >>> zf.filename = 'mem/abcde.zip'

    Path accepts the zipfile object itself || a filename

    >>> root = Path(zf)

    From there, several path operations are available.

    Directory iteration (including the zip file itself):

    >>> a, b = root.iterdir()
    >>> a
    Path('mem/abcde.zip', 'a.txt')
    >>> b
    Path('mem/abcde.zip', 'b/')

    name property:

    >>> b.name
    'b'

    join with divide operator:

    >>> c = b / 'c.txt'
    >>> c
    Path('mem/abcde.zip', 'b/c.txt')
    >>> c.name
    'c.txt'

    Read text:

    >>> c.read_text()
    'content of c'

    existence:

    >>> c.exists()
    true
    >>> (b / 'missing.txt').exists()
    false

    Coercion to string:

    >>> import os
    >>> str(c).replace(os.sep, posixpath.sep)
    'mem/abcde.zip/b/c.txt'

    At the root, ``name``, ``filename``, && ``parent``
    resolve to the zipfile. Note these attributes are not
    valid && will raise a ``ValueError`` if the zipfile
    has no filename.

    >>> root.name
    'abcde.zip'
    >>> str(root.filename).replace(os.sep, posixpath.sep)
    'mem/abcde.zip'
    >>> str(root.parent)
    'mem'
    ";
        __repr = "{self.__class__.__name__}({self.root.filename!r}, {self.at!r})";
        pub fn __init__ ( &self, root , at = "" )  {
        "
        Construct a Path from a ZipFile || filename.

        Note: When the source == an existing ZipFile object,
        its type (__class__) will be mutated to a
        specialized type. If the caller wishes to retain the
        original type, the caller should either create a
        separate ZipFile object || pass a filename.
        ";
        self . root = FastLookup . make ( root );
        self . at = at;
        pub fn open ( &self, mode = "r" , * args , pwd = None /* Option */ , ** kwargs )  {
        "
        Open this entry as text || binary following the semantics
        of ``pathlib.Path.open()`` by passing arguments through
        to io.TextIOWrapper().
        ";
        if self . is_dir ( ) {
        panic!("IsADirectoryError ( self )");
        zip_mode = mode [ 0 ];
        if !self . exists ( ) && zip_mode == "r" {
        panic!("FileNotFoundError ( self )");
        stream = self . root . open ( self . at , zip_mode , pwd = pwd );
        if "b" in mode {
        if args || kwargs {
        panic!("ValueError ( "encoding args invalid for binary operation" )");
        return  stream;
        encoding , args , kwargs = _extract_text_encoding ( * args , ** kwargs );
        return  io . TextIOWrapper ( stream , encoding , * args , ** kwargs );
        pub fn _base ( self )  {
        return  pathlib . PurePosixPath ( self . at || self . root . filename );
        @ property;
        pub fn name ( self )  {
        return  self . _base ( ) . name;
        @ property;
        pub fn suffix ( self )  {
        return  self . _base ( ) . suffix;
        @ property;
        pub fn suffixes ( self )  {
        return  self . _base ( ) . suffixes;
        @ property;
        pub fn stem ( self )  {
        return  self . _base ( ) . stem;
        @ property;
        pub fn filename ( self )  {
        return  pathlib . Path ( self . root . filename ) . joinpath ( self . at );
        pub fn read_text ( &self, * args , ** kwargs )  {
        encoding , args , kwargs = _extract_text_encoding ( * args , ** kwargs );
        // with scope: self . open ( "r" , encoding , * args , ** kwargs ) as strm  {
        return  strm . read ( );
        pub fn read_bytes ( self )  {
        // with scope: self . open ( "rb" ) as strm  {
        return  strm . read ( );
        pub fn _is_child ( &self, path )  {
        return  posixpath . dirname ( path . at . rstrip ( "/" ) ) == self . at . rstrip ( "/" );
        pub fn _next ( &self, at )  {
        return  self . __class__ ( self . root , at );
        pub fn is_dir ( self )  {
        return  !self . at || self . at . endswith ( "/" );
        pub fn is_file ( self )  {
        return  self . exists ( ) && !self . is_dir ( );
        pub fn exists ( self )  {
        return  self . at in self . root . _name_set ( );
        pub fn iterdir ( self )  {
        if !self . is_dir ( ) {
        panic!("ValueError ( "Can't listdir a file" )");
        subs = map ( self . _next , self . root . namelist ( ) );
        return  filter ( self . _is_child , subs );
        pub fn __str__ ( self )  {
        return  posixpath . join ( self . root . filename , self . at );
        pub fn __repr__ ( self )  {
        return  self . __repr . format ( self = self );
        pub fn joinpath ( &self, * other )  {
        next = posixpath . join ( self . at , * other );
        return  self . _next ( self . root . resolve_dir ( next ) );
        __truediv__ = joinpath;
        @ property;
        pub fn parent ( self )  {
        if !self . at {
        return  self . filename . parent;
        parent_at = posixpath . dirname ( self . at . rstrip ( "/" ) );
        if parent_at {
        parent_at + = "/";
        return  self . _next ( parent_at );
        pub fn main ( args = None /* Option */ )  {
        import argparse;
        description = "A simple command-line interface for zipfile module.";
        parser = argparse . ArgumentParser ( description = description );
        group = parser . add_mutually_exclusive_group ( required = true );
        group . add_argument ( "-l" , "--list" , metavar = "<zipfile>" ,;
        help = "Show listing of a zipfile" );
        group . add_argument ( "-e" , "--extract" , nargs = 2 ,;
        metavar = ( "<zipfile>" , "<output_dir>" ) ,;
        help = "Extract zipfile into target dir" );
        group . add_argument ( "-c" , "--create" , nargs = "+" ,;
        metavar = ( "<name>" , "<file>" ) ,;
        help = "Create zipfile from sources" );
        group . add_argument ( "-t" , "--test" , metavar = "<zipfile>" ,;
        help = "Test if a zipfile == valid" );
        parser . add_argument ( "--metadata-encoding" , metavar = "<encoding>" ,;
        help = "Specify encoding of member names for -l, -e && -t" );
        args = parser . parse_args ( args );
        encoding = args . metadata_encoding;
        if args . test is !None /* Option */ {
        src = args . test;
        // with scope: ZipFile ( src , "r" , metadata_encoding = encoding ) as zf  {
        badfile = zf . testzip ( );
        if badfile {
        println!( "The following enclosed file is corrupted: {!r}" . format ( badfile ) );
        println!( "Done testing" );
        } else if args . list is !None /* Option */ {
        src = args . list;
        // with scope: ZipFile ( src , "r" , metadata_encoding = encoding ) as zf  {
        zf . printdir ( );
        } else if args . extract is !None /* Option */ {
        src , curdir = args . extract;
        // with scope: ZipFile ( src , "r" , metadata_encoding = encoding ) as zf  {
        zf . extractall ( curdir );
        } else if args . create is !None /* Option */ {
        if encoding {
        println!( "Non-conforming encodings !supported with -c." );
        file = sys . stderr );
        sys . exit ( 1 );
        zip_name = args . create . pop ( 0 );
        files = args . create;
        pub fn addToZip ( zf , path , zippath )  {
        if os . path . isfile ( path ) {
        zf . write ( path , zippath , ZIP_DEFLATED );
        } else if os . path . isdir ( path ) {
        if zippath {
        zf . write ( path , zippath );
        for nm in sorted ( os . listdir ( path ) ) .iter() {
        addToZip ( zf ,;
        os . path . join ( path , nm ) , os . path . join ( zippath , nm ) );
        // with scope: ZipFile ( zip_name , "w" ) as zf  {
        for path in files .iter() {
        zippath = os . path . basename ( path );
        if !zippath {
        zippath = os . path . basename ( os . path . dirname ( path ) );
        if zippath in ( "" , os . curdir , os . pardir ) {
        zippath = "";
        addToZip ( zf , path , zippath );
        fn main() {
        main ( );
}

