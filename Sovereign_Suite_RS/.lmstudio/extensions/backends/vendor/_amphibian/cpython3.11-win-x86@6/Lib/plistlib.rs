//! plistlib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::binascii;
// use chrono::Utc;
// use crate::io::{BytesIO};
// use crate::itertools;
// use regex::Regex;
// use crate::xml::{ParserCreate};

pub const __all__: f64 = [;
pub const PlistFormat: &str = enum . Enum ("PlistFormat" ,"FMT_XML FMT_BINARY" , module = __name__ );
pub struct UID {
    pub data: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub current_key: String, // TODO: infer type
    pub root: String, // TODO: infer type
    pub _dict_type: String, // TODO: infer type
    pub parser: String, // TODO: infer type
    pub file: String, // TODO: infer type
    pub _indent_level: String, // TODO: infer type
    pub indent: String, // TODO: infer type
    pub _sort_keys: String, // TODO: infer type
    pub _skipkeys: String, // TODO: infer type
    pub _fp: String, // TODO: infer type
    pub _object_offsets: String, // TODO: infer type
    pub _objects: String, // TODO: infer type
    pub _objlist: String, // TODO: infer type
    pub _objtable: String, // TODO: infer type
    pub _objidtable: String, // TODO: infer type
    pub _ref_size: String, // TODO: infer type
    pub _ref_format: String, // TODO: infer type
}

impl UID {
    pub fn new(data: &str) -> Self {
        if !isinstance ( data , int ) {
        panic!("TypeError ( "data must be an int" )");
        if data >= 1 < < 64 {
        panic!("ValueError ( "UIDs cannot be >= 2**64" )");
        if data < 0 {
        panic!("ValueError ( "UIDs must be positive" )");
        self . data = data;
    }

    pub fn _encode_base64(&self, s: &str, maxlinelength: &str) {
        maxbinsize = ( maxlinelength / / 4 ) * 3;
        pieces = [ ];
        for i in range ( 0 , len ( s ) , maxbinsize ) .iter() {
        chunk = s [ i : i + maxbinsize ];
        pieces . append ( binascii . b2a_base64 ( chunk ) );
        return  b "" . join ( pieces );
        pub fn _decode_base64 ( s )  {
        if isinstance ( s , str ) {
        return  binascii . a2b_base64 ( s . encode ( "utf-8" ) );
        } else {
        return  binascii . a2b_base64 ( s );
        _dateParser = re . compile ( r "(?P<year>\d\d\d\d)(?:-(?P<month>\d\d)(?:-(?P<day>\d\d)(?:T(?P<hour>\d\d)(?::(?P<minute>\d\d)(?::(?P<second>\d\d))?)?)?)?)?Z" , re . ASCII );
        pub fn _date_from_string ( s )  {
        order = ( "year" , "month" , "day" , "hour" , "minute" , "second" );
        gd = _dateParser . match ( s ) . groupdict ( );
        lst = [ ];
        for key in order .iter() {
        val = gd [ key ];
        if val is None /* Option */ {
        break;
        lst . append ( int ( val ) );
        return  datetime . datetime ( * lst );
        pub fn _date_to_string ( d )  {
        return  "%04d-%02d-%02dT%02d:%02d:%02dZ" % (;
        d . year , d . month , d . day ,;
        d . hour , d . minute , d . second;
        );
        pub fn _escape ( text )  {
        m = _controlCharPat . search ( text );
        if m is !None /* Option */ {
        panic!("ValueError ( "strings can't contain control characters; "");
        "use bytes instead" );
        text = text . replace ( "\r\n" , "\n" );
        text = text . replace ( "\r" , "\n" );
        text = text . replace ( "&" , "&amp;" );
        text = text . replace ( "<" , "&lt;" );
        text = text . replace ( ">" , "&gt;" );
        return  text;
        class _PlistParser ;
        pub fn __init__ ( &self, dict_type )  {
        self . stack = [ ];
        self . current_key = None /* Option */;
        self . root = None /* Option */;
        self . _dict_type = dict_type;
        pub fn parse ( &self, fileobj )  {
        self . parser = ParserCreate ( );
        self . parser . StartElementHandler = self . handle_begin_element;
        self . parser . EndElementHandler = self . handle_end_element;
        self . parser . CharacterDataHandler = self . handle_data;
        self . parser . EntityDeclHandler = self . handle_entity_decl;
        self . parser . ParseFile ( fileobj );
        return  self . root;
        pub fn handle_entity_decl ( &self, entity_name , is_parameter_entity , value , base , system_id , public_id , notation_name )  {
        panic!("InvalidFileException ( "XML entity declarations are !supported in plist files" )");
        pub fn handle_begin_element ( &self, element , attrs )  {
        self . data = [ ];
        handler = getattr ( self , "begin_" + element , None /* Option */ );
        if handler is !None /* Option */ {
        handler ( attrs );
        pub fn handle_end_element ( &self, element )  {
        handler = getattr ( self , "end_" + element , None /* Option */ );
        if handler is !None /* Option */ {
        handler ( );
        pub fn handle_data ( &self, data )  {
        self . data . append ( data );
        pub fn add_object ( &self, value )  {
        if self . current_key is !None /* Option */ {
        if !isinstance ( self . stack [ -1 ] , type ( { } ) ) {
        panic!("ValueError ( "unexpected element at line %d" %");
        self . parser . CurrentLineNumber );
        self . stack [ -1 ] [ self . current_key ] = value;
        self . current_key = None /* Option */;
        } else if !self . stack {
        self . root = value;
        } else {
        if !isinstance ( self . stack [ -1 ] , type ( [ ] ) ) {
        panic!("ValueError ( "unexpected element at line %d" %");
        self . parser . CurrentLineNumber );
        self . stack [ -1 ] . append ( value );
        pub fn get_data ( self )  {
        data = "" . join ( self . data );
        self . data = [ ];
        return  data;
        pub fn begin_dict ( &self, attrs )  {
        d = self . _dict_type ( );
        self . add_object ( d );
        self . stack . append ( d );
        pub fn end_dict ( self )  {
        if self . current_key {
        panic!("ValueError ( "missing value for key '%s' at line %d" %");
        ( self . current_key , self . parser . CurrentLineNumber ) );
        self . stack . pop ( );
        pub fn end_key ( self )  {
        if self . current_key || !isinstance ( self . stack [ -1 ] , type ( { } ) ) {
        panic!("ValueError ( "unexpected key at line %d" %");
        self . parser . CurrentLineNumber );
        self . current_key = self . get_data ( );
        pub fn begin_array ( &self, attrs )  {
        a = [ ];
        self . add_object ( a );
        self . stack . append ( a );
        pub fn end_array ( self )  {
        self . stack . pop ( );
        pub fn end_true ( self )  {
        self . add_object ( true );
        pub fn end_false ( self )  {
        self . add_object ( false );
        pub fn end_integer ( self )  {
        raw = self . get_data ( );
        if raw . startswith ( "0x" ) || raw . startswith ( "0X" ) {
        self . add_object ( int ( raw , 16 ) );
        } else {
        self . add_object ( int ( raw ) );
        pub fn end_real ( self )  {
        self . add_object ( float ( self . get_data ( ) ) );
        pub fn end_string ( self )  {
        self . add_object ( self . get_data ( ) );
        pub fn end_data ( self )  {
        self . add_object ( _decode_base64 ( self . get_data ( ) ) );
        pub fn end_date ( self )  {
        self . add_object ( _date_from_string ( self . get_data ( ) ) );
        class _DumbXMLWriter ;
        pub fn __init__ ( &self, file , indent_level = 0 , indent = "\t" )  {
        self . file = file;
        self . stack = [ ];
        self . _indent_level = indent_level;
        self . indent = indent;
        pub fn begin_element ( &self, element )  {
        self . stack . append ( element );
        self . writeln ( "<%s>" % element );
        self . _indent_level + = 1;
        pub fn end_element ( &self, element )  {
        assert self . _indent_level > 0;
        assert self . stack . pop ( ) == element;
        self . _indent_level - = 1;
        self . writeln ( "</%s>" % element );
        pub fn simple_element ( &self, element , value = None /* Option */ )  {
        if value is !None /* Option */ {
        value = _escape ( value );
        self . writeln ( "<%s>%s</%s>" % ( element , value , element ) );
        } else {
        self . writeln ( "<%s/>" % element );
        pub fn writeln ( &self, line )  {
        if line {
        if isinstance ( line , str ) {
        line = line . encode ( "utf-8" );
        self . file . write ( self . _indent_level * self . indent );
        self . file . write ( line );
        self . file . write ( b "\n" );
        class _PlistWriter ( _DumbXMLWriter ) ;
        pub fn __init__ ( {
        self , file , indent_level = 0 , indent = b "\t" , writeHeader = 1 ,;
        sort_keys = true , skipkeys = false ) ;
        if writeHeader {
        file . write ( PLISTHEADER );
        _DumbXMLWriter . __init__ ( self , file , indent_level , indent );
        self . _sort_keys = sort_keys;
        self . _skipkeys = skipkeys;
        pub fn write ( &self, value )  {
        self . writeln ( "<plist version=\"1.0\">" );
        self . write_value ( value );
        self . writeln ( "</plist>" );
        pub fn write_value ( &self, value )  {
        if isinstance ( value , str ) {
        self . simple_element ( "string" , value );
        } else if value is true {
        self . simple_element ( "true" );
        } else if value is false {
        self . simple_element ( "false" );
        } else if isinstance ( value , int ) {
        if -1 < < 63 <= value < 1 < < 64 {
        self . simple_element ( "integer" , "%d" % value );
        } else {
        panic!("OverflowError ( value )");
        } else if isinstance ( value , float ) {
        self . simple_element ( "real" , repr ( value ) );
        } else if isinstance ( value , dict ) {
        self . write_dict ( value );
        } else if isinstance ( value , ( bytes , bytearray ) ) {
        self . write_bytes ( value );
        } else if isinstance ( value , datetime . datetime ) {
        self . simple_element ( "date" , _date_to_string ( value ) );
        } else if isinstance ( value , ( tuple , list ) ) {
        self . write_array ( value );
        } else {
        panic!("TypeError ( "unsupported type: %s" % type ( value ) )");
        pub fn write_bytes ( &self, data )  {
        self . begin_element ( "data" );
        self . _indent_level - = 1;
        maxlinelength = max (;
        16 ,;
        76 - len ( self . indent . replace ( b "\t" , b " " * 8 ) * self . _indent_level ) );
        for line in _encode_base64 ( data , maxlinelength ) . split ( b "\n" ) .iter() {
        if line {
        self . writeln ( line );
        self . _indent_level + = 1;
        self . end_element ( "data" );
        pub fn write_dict ( &self, d )  {
        if d {
        self . begin_element ( "dict" );
        if self . _sort_keys {
        items = sorted ( d . items ( ) );
        } else {
        items = d . items ( );
        for key , value in items .iter() {
        if !isinstance ( key , str ) {
        if self . _skipkeys {
        continue;
        panic!("TypeError ( "keys must be strings" )");
        self . simple_element ( "key" , key );
        self . write_value ( value );
        self . end_element ( "dict" );
        } else {
        self . simple_element ( "dict" );
        pub fn write_array ( &self, array )  {
        if array {
        self . begin_element ( "array" );
        for value in array .iter() {
        self . write_value ( value );
        self . end_element ( "array" );
        } else {
        self . simple_element ( "array" );
        pub fn _is_fmt_xml ( header )  {
        prefixes = ( b "<?xml" , b "<plist" );
        for pfx in prefixes .iter() {
        if header . startswith ( pfx ) {
        return  true;
        for bom , encoding in (.iter() {
        ( codecs . BOM_UTF8 , "utf-8" ) ,;
        ( codecs . BOM_UTF16_BE , "utf-16-be" ) ,;
        ( codecs . BOM_UTF16_LE , "utf-16-le" ) ,;
        ) ;
        if !header . startswith ( bom ) {
        continue;
        for start in prefixes .iter() {
        prefix = bom + start . decode ( "ascii" ) . encode ( encoding );
        if header [ { : len ( prefix ) ] == prefix ; }
        return  true;
        return  false;
        class InvalidFileException ( ValueError ) ;
        pub fn __init__ ( &self, message = "Invalid file" )  {
        ValueError . __init__ ( self , message );
        _BINARY_FORMAT = { 1 : "B" , 2 : "H" , 4 : "L" , 8 : "Q" };
        _undefined = object ( );
        class _BinaryPlistParser ;
        "
    Read || write a binary plist file, following the description of the binary
    format.  Raise InvalidFileException in case of error, otherwise return the
    root object.

    see also: http://opensource.apple.com/source/CF/CF-744.18/CFBinaryPList.c
    ";
        pub fn __init__ ( &self, dict_type )  {
        self . _dict_type = dict_type;
        pub fn parse ( &self, fp )  {
        // try {
        self . _fp = fp;
        self . _fp . seek ( -32 , os . SEEK_END );
        trailer = self . _fp . read ( 32 );
        if len ( trailer ) != 32 {
        panic!("InvalidFileException ( )");
        (;
        offset_size , self . _ref_size , num_objects , top_object ,;
        offset_table_offset;
        ) = struct . unpack ( ">6xBBQQQ" , trailer );
        self . _fp . seek ( offset_table_offset );
        self . _object_offsets = self . _read_ints ( num_objects , offset_size );
        self . _objects = [ _undefined ] * num_objects;
        return  self . _read_object ( top_object );
        // } catch  ( OSError , IndexError , struct . error , OverflowError , {
        ValueError ) ;
        panic!("InvalidFileException ( )");
        pub fn _get_size ( &self, tokenL )  {
        " return the size of the next object.";
        if tokenL == 0x F {
        m = self . _fp . read ( 1 ) [ 0 ] & 0x3;
        s = 1 < < m;
        f = ">" + _BINARY_FORMAT [ s ];
        return  struct . unpack ( f , self . _fp . read ( s ) ) [ 0 ];
        return  tokenL;
        pub fn _read_ints ( &self, n , size )  {
        data = self . _fp . read ( size * n );
        if size in _BINARY_FORMAT {
        return  struct . unpack ( f ">{n}{_BINARY_FORMAT[size]}" , data );
        } else {
        if !size || len ( data ) != size * n {
        panic!("InvalidFileException ( )");
        return  tuple ( int . from_bytes ( data [ i : i + size ] , "big" );
        for i in range ( 0 , size * n , size ) ).iter() {
        pub fn _read_refs ( &self, n )  {
        return  self . _read_ints ( n , self . _ref_size );
        pub fn _read_object ( &self, ref )  {
        "
        read the object by reference.

        May recursively read sub-objects (content of an array/dict/set)
        ";
        result = self . _objects [ ref ];
        if result is !_undefined {
        return  result;
        offset = self . _object_offsets [ ref ];
        self . _fp . seek ( offset );
        token = self . _fp . read ( 1 ) [ 0 ];
        tokenH , tokenL = token & 0x F0 , token & 0x0 F;
        if token == 0x00 {
        result = None /* Option */;
        } else if token == 0x08 {
        result = false;
        } else if token == 0x09 {
        result = true;
        } else if token == 0x0 f {
        result = b "";
        } else if tokenH == 0x10 {
        result = int . from_bytes ( self . _fp . read ( 1 < < tokenL ) ,;
        "big" , signed = tokenL >= 3 );
        } else if token == 0x22 {
        result = struct . unpack ( ">format!(" , self . _fp . read ( 4 ) ) [ 0 ]);
        } else if token == 0x23 {
        result = struct . unpack ( ">d" , self . _fp . read ( 8 ) ) [ 0 ];
        } else if token == 0x33 {
        f = struct . unpack ( ">d" , self . _fp . read ( 8 ) ) [ 0 ];
        result = ( datetime . datetime ( 2001 , 1 , 1 ) +;
        datetime . timedelta ( seconds = f ) );
        } else if tokenH == 0x40 {
        s = self . _get_size ( tokenL );
        result = self . _fp . read ( s );
        if len ( result ) != s {
        panic!("InvalidFileException ( )");
        } else if tokenH == 0x50 {
        s = self . _get_size ( tokenL );
        data = self . _fp . read ( s );
        if len ( data ) != s {
        panic!("InvalidFileException ( )");
        result = data . decode ( "ascii" );
        } else if tokenH == 0x60 {
        s = self . _get_size ( tokenL ) * 2;
        data = self . _fp . read ( s );
        if len ( data ) != s {
        panic!("InvalidFileException ( )");
        result = data . decode ( "utf-16be" );
        } else if tokenH == 0x80 {
        result = UID ( int . from_bytes ( self . _fp . read ( 1 + tokenL ) , "big" ) );
        } else if tokenH == 0x A0 {
        s = self . _get_size ( tokenL );
        obj_refs = self . _read_refs ( s );
        result = [ ];
        self . _objects [ ref ] = result;
        result . extend ( self . _read_object ( x ) for x in obj_refs );
        } else if tokenH == 0x D0 {
        s = self . _get_size ( tokenL );
        key_refs = self . _read_refs ( s );
        obj_refs = self . _read_refs ( s );
        result = self . _dict_type ( );
        self . _objects [ ref ] = result;
        // try {
        for k , o in zip ( key_refs , obj_refs ) .iter() {
        result [ self . _read_object ( k ) ] = self . _read_object ( o );
        // } catch  TypeError  {
        panic!("InvalidFileException ( )");
        } else {
        panic!("InvalidFileException ( )");
        self . _objects [ ref ] = result;
        return  result;
        pub fn _count_to_size ( count )  {
        if count < 1 < < 8 {
        return  1;
        } else if count < 1 < < 16 {
        return  2;
        } else if count < 1 < < 32 {
        return  4;
        } else {
        return  8;
        _scalars = ( str , int , float , datetime . datetime , bytes );
        class _BinaryPlistWriter ( object ) ;
        pub fn __init__ ( &self, fp , sort_keys , skipkeys )  {
        self . _fp = fp;
        self . _sort_keys = sort_keys;
        self . _skipkeys = skipkeys;
        pub fn write ( &self, value )  {
        self . _objlist = [ ];
        self . _objtable = { };
        self . _objidtable = { };
        self . _flatten ( value );
        num_objects = len ( self . _objlist );
        self . _object_offsets = [ 0 ] * num_objects;
        self . _ref_size = _count_to_size ( num_objects );
        self . _ref_format = _BINARY_FORMAT [ self . _ref_size ];
        self . _fp . write ( b "bplist00" );
        for obj in self . _objlist .iter() {
        self . _write_object ( obj );
        top_object = self . _getrefnum ( value );
        offset_table_offset = self . _fp . tell ( );
        offset_size = _count_to_size ( offset_table_offset );
        offset_format = ">" + _BINARY_FORMAT [ offset_size ] * num_objects;
        self . _fp . write ( struct . pack ( offset_format , * self . _object_offsets ) );
        sort_version = 0;
        trailer = (;
        sort_version , offset_size , self . _ref_size , num_objects ,;
        top_object , offset_table_offset;
        );
        self . _fp . write ( struct . pack ( ">5xBBBQQQ" , * trailer ) );
        pub fn _flatten ( &self, value )  {
        if isinstance ( value , _scalars ) {
        if ( type ( value ) , value ) in self . _objtable {
        return;
        } else if id ( value ) in self . _objidtable {
        return;
        refnum = len ( self . _objlist );
        self . _objlist . append ( value );
        if isinstance ( value , _scalars ) {
        self . _objtable [ ( type ( value ) , value ) ] = refnum;
        } else {
        self . _objidtable [ id ( value ) ] = refnum;
        if isinstance ( value , dict ) {
        keys = [ ];
        values = [ ];
        items = value . items ( );
        if self . _sort_keys {
        items = sorted ( items );
        for k , v in items .iter() {
        if !isinstance ( k , str ) {
        if self . _skipkeys {
        continue;
        panic!("TypeError ( "keys must be strings" )");
        keys . append ( k );
        values . append ( v );
        for o in itertools . chain ( keys , values ) .iter() {
        self . _flatten ( o );
        } else if isinstance ( value , ( list , tuple ) ) {
        for o in value .iter() {
        self . _flatten ( o );
        pub fn _getrefnum ( &self, value )  {
        if isinstance ( value , _scalars ) {
        return  self . _objtable [ ( type ( value ) , value ) ];
        } else {
        return  self . _objidtable [ id ( value ) ];
        pub fn _write_size ( &self, token , size )  {
        if size < 15 {
        self . _fp . write ( struct . pack ( ">B" , token | size ) );
        } else if size < 1 < < 8 {
        self . _fp . write ( struct . pack ( ">BBB" , token | 0x F , 0x10 , size ) );
        } else if size < 1 < < 16 {
        self . _fp . write ( struct . pack ( ">BBH" , token | 0x F , 0x11 , size ) );
        } else if size < 1 < < 32 {
        self . _fp . write ( struct . pack ( ">BBL" , token | 0x F , 0x12 , size ) );
        } else {
        self . _fp . write ( struct . pack ( ">BBQ" , token | 0x F , 0x13 , size ) );
        pub fn _write_object ( &self, value )  {
        ref = self . _getrefnum ( value );
        self . _object_offsets [ ref ] = self . _fp . tell ( );
        if value is None /* Option */ {
        self . _fp . write ( b "\x00" );
        } else if value is false {
        self . _fp . write ( b "\x08" );
        } else if value is true {
        self . _fp . write ( b "\x09" );
        } else if isinstance ( value , int ) {
        if value < 0 {
        // try {
        self . _fp . write ( struct . pack ( ">Bq" , 0x13 , value ) );
        // } catch  struct . error  {
        panic!("OverflowError ( value ) from None /* Option */");
        } else if value < 1 < < 8 {
        self . _fp . write ( struct . pack ( ">BB" , 0x10 , value ) );
        } else if value < 1 < < 16 {
        self . _fp . write ( struct . pack ( ">BH" , 0x11 , value ) );
        } else if value < 1 < < 32 {
        self . _fp . write ( struct . pack ( ">BL" , 0x12 , value ) );
        } else if value < 1 < < 63 {
        self . _fp . write ( struct . pack ( ">BQ" , 0x13 , value ) );
        } else if value < 1 < < 64 {
        self . _fp . write ( b "\x14" + value . to_bytes ( 16 , "big" , signed = true ) );
        } else {
        panic!("OverflowError ( value )");
        } else if isinstance ( value , float ) {
        self . _fp . write ( struct . pack ( ">Bd" , 0x23 , value ) );
        } else if isinstance ( value , datetime . datetime ) {
        f = ( value - datetime . datetime ( 2001 , 1 , 1 ) ) . total_seconds ( );
        self . _fp . write ( struct . pack ( ">Bd" , 0x33 , f ) );
        } else if isinstance ( value , ( bytes , bytearray ) ) {
        self . _write_size ( 0x40 , len ( value ) );
        self . _fp . write ( value );
        } else if isinstance ( value , str ) {
        // try {
        t = value . encode ( "ascii" );
        self . _write_size ( 0x50 , len ( value ) );
        // } catch  UnicodeEncodeError  {
        t = value . encode ( "utf-16be" );
        self . _write_size ( 0x60 , len ( t ) / / 2 );
        self . _fp . write ( t );
        } else if isinstance ( value , UID ) {
        if value . data < 0 {
        panic!("ValueError ( "UIDs must be positive" )");
        } else if value . data < 1 < < 8 {
        self . _fp . write ( struct . pack ( ">BB" , 0x80 , value ) );
        } else if value . data < 1 < < 16 {
        self . _fp . write ( struct . pack ( ">BH" , 0x81 , value ) );
        } else if value . data < 1 < < 32 {
        self . _fp . write ( struct . pack ( ">BL" , 0x83 , value ) );
        } else if value . data < 1 < < 64 {
        self . _fp . write ( struct . pack ( ">BQ" , 0x87 , value ) );
        } else {
        panic!("OverflowError ( value )");
        } else if isinstance ( value , ( list , tuple ) ) {
        refs = vec![ self . _getrefnum ( o ).iter().map(|o| value ).collect();
        s = len ( refs );
        self . _write_size ( 0x A0 , s );
        self . _fp . write ( struct . pack ( ">" + self . _ref_format * s , * refs ) );
        } else if isinstance ( value , dict ) {
        keyRefs , valRefs = [ ] , [ ];
        if self . _sort_keys {
        rootItems = sorted ( value . items ( ) );
        } else {
        rootItems = value . items ( );
        for k , v in rootItems .iter() {
        if !isinstance ( k , str ) {
        if self . _skipkeys {
        continue;
        panic!("TypeError ( "keys must be strings" )");
        keyRefs . append ( self . _getrefnum ( k ) );
        valRefs . append ( self . _getrefnum ( v ) );
        s = len ( keyRefs );
        self . _write_size ( 0x D0 , s );
        self . _fp . write ( struct . pack ( ">" + self . _ref_format * s , * keyRefs ) );
        self . _fp . write ( struct . pack ( ">" + self . _ref_format * s , * valRefs ) );
        } else {
        panic!("TypeError ( value )");
        pub fn _is_fmt_binary ( header )  {
        return  header [ : 8 ] == b "bplist00";
        _FORMATS = {;
        FMT_XML : dict (;
        detect = _is_fmt_xml ,;
        parser = _PlistParser ,;
        writer = _PlistWriter ,;
        ) ,;
        FMT_BINARY : dict (;
        detect = _is_fmt_binary ,;
        parser = _BinaryPlistParser ,;
        writer = _BinaryPlistWriter ,;
        );
        };
        pub fn load ( fp , * , fmt = None /* Option */ , dict_type = dict )  {
        "Read a .plist file. 'fp' should be a readable && binary file object.
    Return the unpacked root object (which usually == a dictionary).
    ";
        if fmt is None /* Option */ {
        header = fp . read ( 32 );
        fp . seek ( 0 );
        for info in _FORMATS . values ( ) .iter() {
        if info [ "detect" ] ( header ) {
        P = info [ "parser" ];
        break;
        } else {
        panic!("InvalidFileException ( )");
        } else {
        P = _FORMATS [ fmt ] [ "parser" ];
        p = P ( dict_type = dict_type );
        return  p . parse ( fp );
        pub fn loads ( value , * , fmt = None /* Option */ , dict_type = dict )  {
        "Read a .plist file from a bytes object.
    Return the unpacked root object (which usually == a dictionary).
    ";
        fp = BytesIO ( value );
        return  load ( fp , fmt = fmt , dict_type = dict_type );
        pub fn dump ( value , fp , * , fmt = FMT_XML , sort_keys = true , skipkeys = false )  {
        "Write 'value' to a .plist file. 'fp' should be a writable,
    binary file object.
    ";
        if fmt !in _FORMATS {
        panic!("ValueError ( "Unsupported format: %r" % ( fmt , ) )");
        writer = _FORMATS [ fmt ] [ "writer" ] ( fp , sort_keys = sort_keys , skipkeys = skipkeys );
        writer . write ( value );
        pub fn dumps ( value , * , fmt = FMT_XML , skipkeys = false , sort_keys = true )  {
        "Return a bytes object with the contents for a .plist file.
    ";
        fp = BytesIO ( );
        dump ( value , fp , fmt = fmt , skipkeys = skipkeys , sort_keys = sort_keys );
        return  fp . getvalue ( );
    }

}

