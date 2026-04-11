//! zipimport.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_frozen_importlib_external;
// use crate::_unpack_uint16;
// use crate::_imp;
// use crate::marshal;
// use std::time;
// use crate::importlib::{ZipReader};
// use crate::zlib::{decompress};

pub const __all__: &str = ["ZipImportError" ,"zipimporter" ];
pub const path_sep: f64 = _bootstrap_external . path_sep;
pub const alt_path_sep: f64 = _bootstrap_external . path_separators [ 1 : ];
pub struct ZipImportError {
    pub _files: String, // TODO: infer type
    pub archive: String, // TODO: infer type
    pub prefix: String, // TODO: infer type
}

impl ZipImportError {
}

pub const _zip_directory_cache: f64 = { };
pub const _module_type: f64 = type ( sys );
pub const END_CENTRAL_DIR_SIZE: u64 = 22;
pub const STRING_END_ARCHIVE: &str = b"PK\x05\x06";
pub const MAX_COMMENT_LEN: f64 = ( 1 < < 16 ) - 1;
pub struct zipimporter {
    pub _files: String, // TODO: infer type
    pub archive: String, // TODO: infer type
    pub prefix: String, // TODO: infer type
}

impl zipimporter {
}

pub const _zip_searchorder: f64 = (;
pub fn _get_module_path(fullname: &str) {
        return  self . prefix + fullname . rpartition ( "." ) [ 2 ];
        pub fn _is_dir ( &self, path )  {
        dirpath = path + path_sep;
        return  dirpath in self . _files;
        pub fn _get_module_info ( &self, fullname )  {
        path = _get_module_path ( self , fullname );
        for suffix , isbytecode , ispackage in _zip_searchorder .iter() {
        fullpath = path + suffix;
        if fullpath in self . _files {
        return  ispackage;
        return;
        pub fn _read_directory ( archive )  {
        // try {
        fp = _io . open_code ( archive );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't open Zip file: {archive!r}" , path = archive )");
        // with scope: fp  {
        start_offset = fp . tell ( );
        // try {
        // try {
        fp . seek ( - END_CENTRAL_DIR_SIZE , 2 );
        header_position = fp . tell ( );
        buffer = fp . read ( END_CENTRAL_DIR_SIZE );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        if len ( buffer ) != END_CENTRAL_DIR_SIZE {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        if buffer [ { : 4 ] != STRING_END_ARCHIVE ; }
        // try {
        fp . seek ( 0 , 2 );
        file_size = fp . tell ( );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" ,");
        path = archive );
        max_comment_start = max ( file_size - MAX_COMMENT_LEN -;
        END_CENTRAL_DIR_SIZE , 0 );
        // try {
        fp . seek ( max_comment_start );
        data = fp . read ( );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" ,");
        path = archive );
        pos = data . rfind ( STRING_END_ARCHIVE );
        if pos < 0 {
        panic!("ZipImportError ( f "not a Zip file: {archive!r}" ,");
        path = archive );
        buffer = data [ pos : pos + END_CENTRAL_DIR_SIZE ];
        if len ( buffer ) != END_CENTRAL_DIR_SIZE {
        panic!("ZipImportError ( f "corrupt Zip file: {archive!r}" ,");
        path = archive );
        header_position = file_size - len ( data ) + pos;
        header_size = _unpack_uint32 ( buffer [ 12 : 16 ] );
        header_offset = _unpack_uint32 ( buffer [ 16 : 20 ] );
        if header_position < header_size {
        panic!("ZipImportError ( f "bad central directory size: {archive!r}" , path = archive )");
        if header_position < header_offset {
        panic!("ZipImportError ( f "bad central directory offset: {archive!r}" , path = archive )");
        header_position - = header_size;
        arc_offset = header_position - header_offset;
        if arc_offset < 0 {
        panic!("ZipImportError ( f "bad central directory size || offset: {archive!r}" , path = archive )");
        files = { };
        count = 0;
        // try {
        fp . seek ( header_position );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        while true  {
        buffer = fp . read ( 46 );
        if len ( buffer ) < 4 {
        panic!("EOFError ( "EOF read where !expected" )");
        if buffer [ { : 4 ] != b "PK\x01\x02" ; }
        break;
        if len ( buffer ) != 46 {
        panic!("EOFError ( "EOF read where !expected" )");
        flags = _unpack_uint16 ( buffer [ 8 : 10 ] );
        compress = _unpack_uint16 ( buffer [ 10 : 12 ] );
        time = _unpack_uint16 ( buffer [ 12 : 14 ] );
        date = _unpack_uint16 ( buffer [ 14 : 16 ] );
        crc = _unpack_uint32 ( buffer [ 16 : 20 ] );
        data_size = _unpack_uint32 ( buffer [ 20 : 24 ] );
        file_size = _unpack_uint32 ( buffer [ 24 : 28 ] );
        name_size = _unpack_uint16 ( buffer [ 28 : 30 ] );
        extra_size = _unpack_uint16 ( buffer [ 30 : 32 ] );
        comment_size = _unpack_uint16 ( buffer [ 32 : 34 ] );
        file_offset = _unpack_uint32 ( buffer [ 42 : 46 ] );
        header_size = name_size + extra_size + comment_size;
        if file_offset > header_offset {
        panic!("ZipImportError ( f "bad local header offset: {archive!r}" , path = archive )");
        file_offset + = arc_offset;
        // try {
        name = fp . read ( name_size );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        if len ( name ) != name_size {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        // try {
        if len ( fp . read ( header_size - name_size ) ) != header_size - name_size {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        if flags & 0x800 {
        name = name . decode ( );
        } else {
        // try {
        name = name . decode ( "ascii" );
        // } catch  UnicodeDecodeError  {
        name = name . decode ( "latin1" ) . translate ( cp437_table );
        name = name . replace ( "/" , path_sep );
        path = _bootstrap_external . _path_join ( archive , name );
        t = ( path , compress , data_size , file_size , file_offset , time , date , crc );
        files [ name ] = t;
        count + = 1;
        // } finally {
        fp . seek ( start_offset );
        _bootstrap . _verbose_message ( "zipimport: found {} names in {!r}" , count , archive );
        return  files;
        cp437_table = (;
        "\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0format!(");
        "\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1format!(");
        " !"#$%&\'()*+,-./";
        "0123456789:;<=>?";
        "@ABCDEFGHIJKLMNO";
        "PQRSTUVWXYZ[\\]^_";
        "`abcdefghijklmno";
        "pqrstuvwxyz{|}~\x7format!(");
        "\xc7\xfc\xe9\xe2\xe4\xe0\xe5\xe7";
        "\xea\xeb\xe8\xef\xee\xec\xc4\xc5";
        "\xc9\xe6\xc6\xf4\xf6\xf2\xfb\xf9";
        "\xff\xd6\xdc\xa2\xa3\xa5\u20a7\u0192";
        "\xe1\xed\xf3\xfa\xf1\xd1\xaa\xba";
        "\xbf\u2310\xac\xbd\xbc\xa1\xab\xbb";
        "\u2591\u2592\u2593\u2502\u2524\u2561\u2562\u2556";
        "\u2555\u2563\u2551\u2557\u255d\u255c\u255b\u2510";
        "\u2514\u2534\u252c\u251c\u2500\u253c\u255e\u255format!(");
        "\u255a\u2554\u2569\u2566\u2560\u2550\u256c\u2567";
        "\u2568\u2564\u2565\u2559\u2558\u2552\u2553\u256b";
        "\u256a\u2518\u250c\u2588\u2584\u258c\u2590\u2580";
        "\u03b1\xdf\u0393\u03c0\u03a3\u03c3\xb5\u03c4";
        "\u03a6\u0398\u03a9\u03b4\u221e\u03c6\u03b5\u2229";
        "\u2261\xb1\u2265\u2264\u2320\u2321\xf7\u2248";
        "\xb0\u2219\xb7\u221a\u207f\xb2\u25a0\xa0";
        );
        _importing_zlib = false;
        pub fn _get_decompress_func ( )  {
        global _importing_zlib;
        if _importing_zlib {
        _bootstrap . _verbose_message ( "zipimport: zlib UNAVAILABLE" );
        panic!("ZipImportError ( "can't decompress data; zlib !available" )");
        _importing_zlib = true;
        // try {
        from zlib import decompress;
        // } catch  Exception  {
        _bootstrap . _verbose_message ( "zipimport: zlib UNAVAILABLE" );
        panic!("ZipImportError ( "can't decompress data; zlib !available" )");
        // } finally {
        _importing_zlib = false;
        _bootstrap . _verbose_message ( "zipimport: zlib available" );
        return  decompress;
        pub fn _get_data ( archive , toc_entry )  {
        datapath , compress , data_size , file_size , file_offset , time , date , crc = toc_entry;
        if data_size < 0 {
        panic!("ZipImportError ( "negative data size" )");
        // with scope: _io . open_code ( archive ) as fp  {
        // try {
        fp . seek ( file_offset );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        buffer = fp . read ( 30 );
        if len ( buffer ) != 30 {
        panic!("EOFError ( "EOF read where !expected" )");
        if buffer [ { : 4 ] != b "PK\x03\x04" ; }
        panic!("ZipImportError ( f "bad local file header: {archive!r}" , path = archive )");
        name_size = _unpack_uint16 ( buffer [ 26 : 28 ] );
        extra_size = _unpack_uint16 ( buffer [ 28 : 30 ] );
        header_size = 30 + name_size + extra_size;
        file_offset + = header_size;
        // try {
        fp . seek ( file_offset );
        // } catch  OSError  {
        panic!("ZipImportError ( f "can't read Zip file: {archive!r}" , path = archive )");
        raw_data = fp . read ( data_size );
        if len ( raw_data ) != data_size {
        panic!("OSError ( "zipimport: can't read data" )");
        if compress == 0 {
        return  raw_data;
        // try {
        decompress = _get_decompress_func ( );
        // } catch  Exception  {
        panic!("ZipImportError ( "can't decompress data; zlib !available" )");
        return  decompress ( raw_data , -15 );
        pub fn _eq_mtime ( t1 , t2 )  {
        return  abs ( t1 - t2 ) <= 1;
        pub fn _unmarshal_code ( &self, pathname , fullpath , fullname , data )  {
        exc_details = {;
        "name" : fullname ,;
        "path" : fullpath ,;
        };
        flags = _bootstrap_external . _classify_pyc ( data , fullname , exc_details );
        hash_based = flags & 0 b1 != 0;
        if hash_based {
        check_source = flags & 0 b10 != 0;
        if ( _imp . check_hash_based_pycs != "never" and {
        ( check_source || _imp . check_hash_based_pycs == "always" ) ) ;
        source_bytes = _get_pyc_source ( self , fullpath );
        if source_bytes is !None /* Option */ {
        source_hash = _imp . source_hash (;
        _bootstrap_external . _RAW_MAGIC_NUMBER ,;
        source_bytes ,;
        );
        _bootstrap_external . _validate_hash_pyc (;
        data , source_hash , fullname , exc_details );
        } else {
        source_mtime , source_size = \;
        _get_mtime_and_size_of_source ( self , fullpath );
        if source_mtime {
        if ( !_eq_mtime ( _unpack_uint32 ( data [ 8 { : 12 ] ) , source_mtime ) or; }
        _unpack_uint32 ( data [ 12 : 16 ] ) != source_size ) ;
        _bootstrap . _verbose_message (;
        format!("bytecode == stale for {fullname!r}" ));
        return;
        code = marshal . loads ( data [ 16 : ] );
        if !isinstance ( code , _code_type ) {
        panic!("TypeError ( f "compiled module {pathname!r} is !a code object" )");
        return  code;
        _code_type = type ( _unmarshal_code . __code__ );
        pub fn _normalize_line_endings ( source )  {
        source = source . replace ( b "\r\n" , b "\n" );
        source = source . replace ( b "\r" , b "\n" );
        return  source;
        pub fn _compile_source ( pathname , source )  {
        source = _normalize_line_endings ( source );
        return  compile ( source , pathname , "exec" , dont_inherit = true );
        pub fn _parse_dostime ( d , t )  {
        return  time . mktime ( (;
        ( d > > 9 ) + 1980 ,;
        ( d > > 5 ) & 0x F ,;
        d & 0x1 F ,;
        t > > 11 ,;
        ( t > > 5 ) & 0x3 F ,;
        ( t & 0x1 F ) * 2 ,;
        -1 , -1 , -1 ) );
        pub fn _get_mtime_and_size_of_source ( &self, path )  {
        // try {
        assert path [ -1 : ] in ( "c" , "o" );
        path = path [ : -1 ];
        toc_entry = self . _files [ path ];
        time = toc_entry [ 5 ];
        date = toc_entry [ 6 ];
        uncompressed_size = toc_entry [ 3 ];
        return  _parse_dostime ( date , time ) , uncompressed_size;
        // } catch  ( KeyError , IndexError , TypeError )  {
        return  0 , 0;
        pub fn _get_pyc_source ( &self, path )  {
        assert path [ -1 : ] in ( "c" , "o" );
        path = path [ : -1 ];
        // try {
        toc_entry = self . _files [ path ];
        // } catch  KeyError  {
        return;
        } else {
        return  _get_data ( self . archive , toc_entry );
        pub fn _get_module_code ( &self, fullname )  {
        path = _get_module_path ( self , fullname );
        import_error = None /* Option */;
        for suffix , isbytecode , ispackage in _zip_searchorder .iter() {
        fullpath = path + suffix;
        _bootstrap . _verbose_message ( "trying {}{}{}" , self . archive , path_sep , fullpath , verbosity = 2 );
        // try {
        toc_entry = self . _files [ fullpath ];
        // } catch  KeyError  {
        // pass
        } else {
        modpath = toc_entry [ 0 ];
        data = _get_data ( self . archive , toc_entry );
        code = None /* Option */;
        if isbytecode {
        // try {
        code = _unmarshal_code ( self , modpath , fullpath , fullname , data );
        // } catch  ImportError as exc  {
        import_error = exc;
        } else {
        code = _compile_source ( modpath , data );
        if code is None /* Option */ {
        continue;
        modpath = toc_entry [ 0 ];
        return  code , ispackage , modpath;
        } else {
        if import_error {
        msg = format!("module load failed: {import_error}");
        panic!("ZipImportError ( msg , name = fullname ) from import_error");
        } else {
        panic!("ZipImportError ( f "can't find module {fullname!r}" , name = fullname )");
}

