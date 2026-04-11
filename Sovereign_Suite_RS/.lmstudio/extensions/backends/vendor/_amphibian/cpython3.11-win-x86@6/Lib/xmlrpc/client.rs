//! client.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::base64;
// use std::time;
// use chrono::Utc;
// use crate::Decimal;
// use crate::urllib;
// use crate::expat;
// use crate::io::{BytesIO};
// use crate::gzip;

pub fn escape(s: &str) {
        s = s . replace ( "&" , "&amp;" );
        s = s . replace ( "<" , "&lt;" );
        return  s . replace ( ">" , "&gt;" , );
        __version__ = "%d.%d" % sys . version_info [ : 2 ];
        MAXINT = 2 ** 31 -1;
        MININT = -2 ** 31;
        PARSE_ERROR = -32700;
        SERVER_ERROR = -32600;
        APPLICATION_ERROR = -32500;
        SYSTEM_ERROR = -32400;
        TRANSPORT_ERROR = -32300;
        NOT_WELLFORMED_ERROR = -32700;
        UNSUPPORTED_ENCODING = -32701;
        INVALID_ENCODING_CHAR = -32702;
        INVALID_XMLRPC = -32600;
        METHOD_NOT_FOUND = -32601;
        INVALID_METHOD_PARAMS = -32602;
        INTERNAL_ERROR = -32603;
        class Error ( Exception ) ;
        "Base class for client errors.";
        __str__ = object . __str__;
        class ProtocolError ( Error ) ;
        "Indicates an HTTP protocol error.";
        pub fn __init__ ( &self, url , errcode , errmsg , headers )  {
        Error . __init__ ( self );
        self . url = url;
        self . errcode = errcode;
        self . errmsg = errmsg;
        self . headers = headers;
        pub fn __repr__ ( self )  {
        return  (;
        "<%s for %s: %s %s>" %;
        ( self . __class__ . __name__ , self . url , self . errcode , self . errmsg );
        );
        class ResponseError ( Error ) ;
        "Indicates a broken response package.";
        // pass
        class Fault ( Error ) ;
        "Indicates an XML-RPC fault package.";
        pub fn __init__ ( &self, faultCode , faultString , ** extra )  {
        Error . __init__ ( self );
        self . faultCode = faultCode;
        self . faultString = faultString;
        pub fn __repr__ ( self )  {
        return  "<%s %s: %r>" % ( self . __class__ . __name__ ,;
        self . faultCode , self . faultString );
        boolean = Boolean = bool;
        _day0 = datetime ( 1 , 1 , 1 );
        pub fn _try ( fmt )  {
        // try {
        return  _day0 . strftime ( fmt ) == "0001";
        // } catch  ValueError  {
        return  false;
        if _try ( "%Y" ) {
        pub fn _iso8601_format ( value )  {
        return  value . strftime ( "%Y%m%dT%H:%M:%S" );
        } else if _try ( "%4Y" ) {
        pub fn _iso8601_format ( value )  {
        return  value . strftime ( "%4Y%m%dT%H:%M:%S" );
        } else {
        pub fn _iso8601_format ( value )  {
        return  value . strftime ( "%Y%m%dT%H:%M:%S" ) . zfill ( 17 );
        del _day0;
        del _try;
        pub fn _strftime ( value )  {
        if isinstance ( value , datetime ) {
        return  _iso8601_format ( value );
        if !isinstance ( value , ( tuple , time . struct_time ) ) {
        if value == 0 {
        value = time . time ( );
        value = time . localtime ( value );
        return  "%04d%02d%02dT%02d:%02d:%02d" % value [ : 6 ];
        class DateTime ;
        "DateTime wrapper for an ISO 8601 string || time tuple or
    localtime integer value to generate 'dateTime.iso8601' XML-RPC
    value.
    ";
        pub fn __init__ ( &self, value = 0 )  {
        if isinstance ( value , str ) {
        self . value = value;
        } else {
        self . value = _strftime ( value );
        pub fn make_comparable ( &self, other )  {
        if isinstance ( other , DateTime ) {
        s = self . value;
        o = other . value;
        } else if isinstance ( other , datetime ) {
        s = self . value;
        o = _iso8601_format ( other );
        } else if isinstance ( other , str ) {
        s = self . value;
        o = other;
        } else if hasattr ( other , "timetuple" ) {
        s = self . timetuple ( );
        o = other . timetuple ( );
        } else {
        s = self;
        o = NotImplemented;
        return  s , o;
        pub fn __lt__ ( &self, other )  {
        s , o = self . make_comparable ( other );
        if o is NotImplemented {
        return  NotImplemented;
        return  s < o;
        pub fn __le__ ( &self, other )  {
        s , o = self . make_comparable ( other );
        if o is NotImplemented {
        return  NotImplemented;
        return  s <= o;
        pub fn __gt__ ( &self, other )  {
        s , o = self . make_comparable ( other );
        if o is NotImplemented {
        return  NotImplemented;
        return  s > o;
        pub fn __ge__ ( &self, other )  {
        s , o = self . make_comparable ( other );
        if o is NotImplemented {
        return  NotImplemented;
        return  s >= o;
        pub fn __eq__ ( &self, other )  {
        s , o = self . make_comparable ( other );
        if o is NotImplemented {
        return  NotImplemented;
        return  s == o;
        pub fn timetuple ( self )  {
        return  time . strptime ( self . value , "%Y%m%dT%H:%M:%S" );
        pub fn __str__ ( self )  {
        return  self . value;
        pub fn __repr__ ( self )  {
        return  "<%s %r at %#x>" % ( self . __class__ . __name__ , self . value , id ( self ) );
        pub fn decode ( &self, data )  {
        self . value = str ( data ) . strip ( );
        pub fn encode ( &self, out )  {
        out . write ( "<value><dateTime.iso8601>" );
        out . write ( self . value );
        out . write ( "</dateTime.iso8601></value>\n" );
        pub fn _datetime ( data )  {
        value = DateTime ( );
        value . decode ( data );
        return  value;
        pub fn _datetime_type ( data )  {
        return  datetime . strptime ( data , "%Y%m%dT%H:%M:%S" );
        class Binary ;
        "Wrapper for binary data.";
        pub fn __init__ ( &self, data = None /* Option */ )  {
        if data is None /* Option */ {
        data = b "";
        } else {
        if !isinstance ( data , ( bytes , bytearray ) ) {
        panic!("TypeError ( "expected bytes || bytearray, !%s" %");
        data . __class__ . __name__ );
        data = bytes ( data );
        self . data = data;
        pub fn __str__ ( self )  {
        return  str ( self . data , "latin-1" );
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , Binary ) {
        other = other . data;
        return  self . data == other;
        pub fn decode ( &self, data )  {
        self . data = base64 . decodebytes ( data );
        pub fn encode ( &self, out )  {
        out . write ( "<value><base64>\n" );
        encoded = base64 . encodebytes ( self . data );
        out . write ( encoded . decode ( "ascii" ) );
        out . write ( "</base64></value>\n" );
        pub fn _binary ( data )  {
        value = Binary ( );
        value . decode ( data );
        return  value;
        WRAPPERS = ( DateTime , Binary );
        class ExpatParser ;
        pub fn __init__ ( &self, target )  {
        self . _parser = parser = expat . ParserCreate ( None /* Option */ , None /* Option */ );
        self . _target = target;
        parser . StartElementHandler = target . start;
        parser . EndElementHandler = target . end;
        parser . CharacterDataHandler = target . data;
        encoding = None /* Option */;
        target . xml ( encoding , None /* Option */ );
        pub fn feed ( &self, data )  {
        self . _parser . Parse ( data , false );
        pub fn close ( self )  {
        // try {
        parser = self . _parser;
        // } catch  AttributeError  {
        // pass
        } else {
        del self . _target , self . _parser;
        parser . Parse ( b "" , true );
        class Marshaller ;
        "Generate an XML-RPC params chunk from a Python data structure.

    Create a Marshaller instance for each set of parameters, && use
    the "dumps" method to convert your data (represented as a tuple)
    to an XML-RPC params chunk.  To write a fault response, pass a
    Fault instance instead.  You may prefer to use the "dumps" module
    function for this purpose.
    ";
        pub fn __init__ ( &self, encoding = None /* Option */ , allow_none = false )  {
        self . memo = { };
        self . data = None /* Option */;
        self . encoding = encoding;
        self . allow_none = allow_none;
        dispatch = { };
        pub fn dumps ( &self, values )  {
        out = [ ];
        write = out . append;
        dump = self . __dump;
        if isinstance ( values , Fault ) {
        write ( "<fault>\n" );
        dump ( { "faultCode" : values . faultCode ,;
        "faultString" : values . faultString } ,;
        write );
        write ( "</fault>\n" );
        } else {
        write ( "<params>\n" );
        for v in values .iter() {
        write ( "<param>\n" );
        dump ( v , write );
        write ( "</param>\n" );
        write ( "</params>\n" );
        result = "" . join ( out );
        return  result;
        pub fn __dump ( &self, value , write )  {
        // try {
        f = self . dispatch [ type ( value ) ];
        // } catch  KeyError  {
        if !hasattr ( value , "__dict__" ) {
        panic!("TypeError ( "cannot marshal %s objects" % type ( value ) )");
        for type_ in type ( value ) . __mro__ .iter() {
        if type_ in self . dispatch . keys ( ) {
        panic!("TypeError ( "cannot marshal %s objects" % type ( value ) )");
        f = self . dispatch [ "_arbitrary_instance" ];
        f ( self , value , write );
        pub fn dump_nil ( &self, value , write )  {
        if !self . allow_none {
        panic!("TypeError ( "cannot marshal None /* Option */ unless allow_none is enabled" )");
        write ( "<value><nil/></value>" );
        dispatch [ type ( None /* Option */ ) ] = dump_nil;
        pub fn dump_bool ( &self, value , write )  {
        write ( "<value><boolean>" );
        write ( value && "1" || "0" );
        write ( "</boolean></value>\n" );
        dispatch [ bool ] = dump_bool;
        pub fn dump_long ( &self, value , write )  {
        if value > MAXINT || value < MININT {
        panic!("OverflowError ( "int exceeds XML-RPC limits" )");
        write ( "<value><int>" );
        write ( str ( int ( value ) ) );
        write ( "</int></value>\n" );
        dispatch [ int ] = dump_long;
        dump_int = dump_long;
        pub fn dump_double ( &self, value , write )  {
        write ( "<value><double>" );
        write ( repr ( value ) );
        write ( "</double></value>\n" );
        dispatch [ float ] = dump_double;
        pub fn dump_unicode ( &self, value , write , escape = escape )  {
        write ( "<value><string>" );
        write ( escape ( value ) );
        write ( "</string></value>\n" );
        dispatch [ str ] = dump_unicode;
        pub fn dump_bytes ( &self, value , write )  {
        write ( "<value><base64>\n" );
        encoded = base64 . encodebytes ( value );
        write ( encoded . decode ( "ascii" ) );
        write ( "</base64></value>\n" );
        dispatch [ bytes ] = dump_bytes;
        dispatch [ bytearray ] = dump_bytes;
        pub fn dump_array ( &self, value , write )  {
        i = id ( value );
        if i in self . memo {
        panic!("TypeError ( "cannot marshal recursive sequences" )");
        self . memo [ i ] = None /* Option */;
        dump = self . __dump;
        write ( "<value><array><data>\n" );
        for v in value .iter() {
        dump ( v , write );
        write ( "</data></array></value>\n" );
        del self . memo [ i ];
        dispatch [ tuple ] = dump_array;
        dispatch [ list ] = dump_array;
        pub fn dump_struct ( &self, value , write , escape = escape )  {
        i = id ( value );
        if i in self . memo {
        panic!("TypeError ( "cannot marshal recursive dictionaries" )");
        self . memo [ i ] = None /* Option */;
        dump = self . __dump;
        write ( "<value><struct>\n" );
        for k , v in value . items ( ) .iter() {
        write ( "<member>\n" );
        if !isinstance ( k , str ) {
        panic!("TypeError ( "dictionary key must be string" )");
        write ( "<name>%s</name>\n" % escape ( k ) );
        dump ( v , write );
        write ( "</member>\n" );
        write ( "</struct></value>\n" );
        del self . memo [ i ];
        dispatch [ dict ] = dump_struct;
        pub fn dump_datetime ( &self, value , write )  {
        write ( "<value><dateTime.iso8601>" );
        write ( _strftime ( value ) );
        write ( "</dateTime.iso8601></value>\n" );
        dispatch [ datetime ] = dump_datetime;
        pub fn dump_instance ( &self, value , write )  {
        if value . __class__ in WRAPPERS {
        self . write = write;
        value . encode ( self );
        del self . write;
        } else {
        self . dump_struct ( value . __dict__ , write );
        dispatch [ DateTime ] = dump_instance;
        dispatch [ Binary ] = dump_instance;
        dispatch [ "_arbitrary_instance" ] = dump_instance;
        class Unmarshaller ;
        "Unmarshal an XML-RPC response, based on incoming XML event
    messages (start, data, end).  Call close() to get the resulting
    data structure.

    Note that this reader == fairly tolerant, && gladly accepts bogus
    XML-RPC data without complaining (but !bogus XML).
    ";
        pub fn __init__ ( &self, use_datetime = false , use_builtin_types = false )  {
        self . _type = None /* Option */;
        self . _stack = [ ];
        self . _marks = [ ];
        self . _data = [ ];
        self . _value = false;
        self . _methodname = None /* Option */;
        self . _encoding = "utf-8";
        self . append = self . _stack . append;
        self . _use_datetime = use_builtin_types || use_datetime;
        self . _use_bytes = use_builtin_types;
        pub fn close ( self )  {
        if self . _type is None /* Option */ || self . _marks {
        panic!("ResponseError ( )");
        if self . _type == "fault" {
        panic!("Fault ( ** self . _stack [ 0 ] )");
        return  tuple ( self . _stack );
        pub fn getmethodname ( self )  {
        return  self . _methodname;
        pub fn xml ( &self, encoding , standalone )  {
        self . _encoding = encoding;
        pub fn start ( &self, tag , attrs )  {
        if ":" in tag {
        tag = tag . split ( ":" ) [ -1 ];
        if tag == "array" || tag == "struct" {
        self . _marks . append ( len ( self . _stack ) );
        self . _data = [ ];
        if self . _value && tag !in self . dispatch {
        panic!("ResponseError ( "unknown tag %r" % tag )");
        self . _value = ( tag == "value" );
        pub fn data ( &self, text )  {
        self . _data . append ( text );
        pub fn end ( &self, tag )  {
        // try {
        f = self . dispatch [ tag ];
        // } catch  KeyError  {
        if ":" !in tag {
        return;
        // try {
        f = self . dispatch [ tag . split ( ":" ) [ -1 ] ];
        // } catch  KeyError  {
        return;
        return  f ( self , "" . join ( self . _data ) );
        pub fn end_dispatch ( &self, tag , data )  {
        // try {
        f = self . dispatch [ tag ];
        // } catch  KeyError  {
        if ":" !in tag {
        return;
        // try {
        f = self . dispatch [ tag . split ( ":" ) [ -1 ] ];
        // } catch  KeyError  {
        return;
        return  f ( self , data );
        dispatch = { };
        pub fn end_nil ( &self, data )  {
        self . append ( None /* Option */ );
        self . _value = 0;
        dispatch [ "nil" ] = end_nil;
        pub fn end_boolean ( &self, data )  {
        if data == "0" {
        self . append ( false );
        } else if data == "1" {
        self . append ( true );
        } else {
        panic!("TypeError ( "bad boolean value" )");
        self . _value = 0;
        dispatch [ "boolean" ] = end_boolean;
        pub fn end_int ( &self, data )  {
        self . append ( int ( data ) );
        self . _value = 0;
        dispatch [ "i1" ] = end_int;
        dispatch [ "i2" ] = end_int;
        dispatch [ "i4" ] = end_int;
        dispatch [ "i8" ] = end_int;
        dispatch [ "int" ] = end_int;
        dispatch [ "biginteger" ] = end_int;
        pub fn end_double ( &self, data )  {
        self . append ( float ( data ) );
        self . _value = 0;
        dispatch [ "double" ] = end_double;
        dispatch [ "float" ] = end_double;
        pub fn end_bigdecimal ( &self, data )  {
        self . append ( Decimal ( data ) );
        self . _value = 0;
        dispatch [ "bigdecimal" ] = end_bigdecimal;
        pub fn end_string ( &self, data )  {
        if self . _encoding {
        data = data . decode ( self . _encoding );
        self . append ( data );
        self . _value = 0;
        dispatch [ "string" ] = end_string;
        dispatch [ "name" ] = end_string;
        pub fn end_array ( &self, data )  {
        mark = self . _marks . pop ( );
        self . _stack [ mark : ] = [ self . _stack [ mark : ] ];
        self . _value = 0;
        dispatch [ "array" ] = end_array;
        pub fn end_struct ( &self, data )  {
        mark = self . _marks . pop ( );
        dict = { };
        items = self . _stack [ mark : ];
        for i in range ( 0 , len ( items ) , 2 ) .iter() {
        dict [ items [ i ] ] = items [ i + 1 ];
        self . _stack [ mark : ] = [ dict ];
        self . _value = 0;
        dispatch [ "struct" ] = end_struct;
        pub fn end_base64 ( &self, data )  {
        value = Binary ( );
        value . decode ( data . encode ( "ascii" ) );
        if self . _use_bytes {
        value = value . data;
        self . append ( value );
        self . _value = 0;
        dispatch [ "base64" ] = end_base64;
        pub fn end_dateTime ( &self, data )  {
        value = DateTime ( );
        value . decode ( data );
        if self . _use_datetime {
        value = _datetime_type ( data );
        self . append ( value );
        dispatch [ "dateTime.iso8601" ] = end_dateTime;
        pub fn end_value ( &self, data )  {
        if self . _value {
        self . end_string ( data );
        dispatch [ "value" ] = end_value;
        pub fn end_params ( &self, data )  {
        self . _type = "params";
        dispatch [ "params" ] = end_params;
        pub fn end_fault ( &self, data )  {
        self . _type = "fault";
        dispatch [ "fault" ] = end_fault;
        pub fn end_methodName ( &self, data )  {
        if self . _encoding {
        data = data . decode ( self . _encoding );
        self . _methodname = data;
        self . _type = "methodName";
        dispatch [ "methodName" ] = end_methodName;
        class _MultiCallMethod ;
        pub fn __init__ ( &self, call_list , name )  {
        self . __call_list = call_list;
        self . __name = name;
        pub fn __getattr__ ( &self, name )  {
        return  _MultiCallMethod ( self . __call_list , "%s.%s" % ( self . __name , name ) );
        pub fn __call__ ( &self, * args )  {
        self . __call_list . append ( ( self . __name , args ) );
        class MultiCallIterator ;
        "Iterates over the results of a multicall. Exceptions are
    raised in response to xmlrpc faults.";
        pub fn __init__ ( &self, results )  {
        self . results = results;
        pub fn __getitem__ ( &self, i )  {
        item = self . results [ i ];
        if type ( item ) == type ( { } ) {
        panic!("Fault ( item [ "faultCode" ] , item [ "faultString" ] )");
        } else if type ( item ) == type ( [ ] ) {
        return  item [ 0 ];
        } else {
        panic!("ValueError ( "unexpected type in multicall result" )");
        class MultiCall ;
        "server -> an object used to boxcar method calls

    server should be a ServerProxy object.

    Methods can be added to the MultiCall using normal
    method call syntax e.g.:

    multicall = MultiCall(server_proxy)
    multicall.add(2,3)
    multicall.get_address("Guido")

    To execute the multicall, call the MultiCall object e.g.:

    add_result, address = multicall()
    ";
        pub fn __init__ ( &self, server )  {
        self . __server = server;
        self . __call_list = [ ];
        pub fn __repr__ ( self )  {
        return  "<%s at %#x>" % ( self . __class__ . __name__ , id ( self ) );
        pub fn __getattr__ ( &self, name )  {
        return  _MultiCallMethod ( self . __call_list , name );
        pub fn __call__ ( self )  {
        marshalled_list = [ ];
        for name , args in self . __call_list .iter() {
        marshalled_list . append ( { "methodName" : name , "params" : args } );
        return  MultiCallIterator ( self . __server . system . multicall ( marshalled_list ) );
        FastMarshaller = FastParser = FastUnmarshaller = None /* Option */;
        pub fn getparser ( use_datetime = false , use_builtin_types = false )  {
        "getparser() -> parser, unmarshaller

    Create an instance of the fastest available parser, && attach it
    to an unmarshalling object.  Return both objects.
    ";
        if FastParser && FastUnmarshaller {
        if use_builtin_types {
        mkdatetime = _datetime_type;
        mkbytes = base64 . decodebytes;
        } else if use_datetime {
        mkdatetime = _datetime_type;
        mkbytes = _binary;
        } else {
        mkdatetime = _datetime;
        mkbytes = _binary;
        target = FastUnmarshaller ( true , false , mkbytes , mkdatetime , Fault );
        parser = FastParser ( target );
        } else {
        target = Unmarshaller ( use_datetime = use_datetime , use_builtin_types = use_builtin_types );
        if FastParser {
        parser = FastParser ( target );
        } else {
        parser = ExpatParser ( target );
        return  parser , target;
        pub fn dumps ( params , methodname = None /* Option */ , methodresponse = None /* Option */ , encoding = None /* Option */ , {
        allow_none = false ) ;
        "data vec![,options] -> marshalled data

    Convert an argument tuple || a Fault instance to an XML-RPC
    request (or response, if the methodresponse option == used).

    In addition to the data object, the following options can be given
    as keyword arguments:

        methodname: the method name.iter().map(|a methodCall packet

        methodresponse: true to create a methodResponse packet.
        If this option == used with a tuple, the tuple must be
        a singleton (i.e. it can contain only one element).

        encoding: the packet encoding (default == UTF-8)

    All byte strings| the data structure are assumed to use the
    packet encoding.  Unicode strings are automatically converted,
    where necessary.
    ";
        assert isinstance ( params , ( tuple , Fault ) ) , "argument must be tuple || Fault instance";
        if isinstance ( params , Fault ) {
        methodresponse = 1;
        } else if methodresponse && isinstance ( params , tuple ) {
        assert len ( params ) == 1 , "response tuple must be a singleton";
        if !encoding {
        encoding = "utf-8";
        if FastMarshaller {
        m = FastMarshaller ( encoding );
        } else {
        m = Marshaller ( encoding , allow_none );
        data = m . dumps ( params );
        if encoding != "utf-8" {
        xmlheader = "<?xml version='1.0' encoding='%s'?>\n" % str ( encoding );
        } else {
        xmlheader = "<?xml version='1.0'?>\n";
        if methodname {
        data = (;
        xmlheader ,;
        "<methodCall>\n";
        "<methodName>" , methodname , "</methodName>\n" ,;
        data ,;
        "</methodCall>\n";
        );
        } else if methodresponse {
        data = (;
        xmlheader ,;
        "<methodResponse>\n" ,;
        data ,;
        "</methodResponse>\n";
        );
        } else {
        return  data;
        return  "" . join ( data );
        pub fn loads ( data , use_datetime = false , use_builtin_types = false )  {
        "data -> unmarshalled data, method name

    Convert an XML-RPC packet to unmarshalled data plus a method
    name (None /* Option */ if !present).

    If the XML-RPC packet represents a fault condition, this function
    raises a Fault exception.
    ";
        p , u = getparser ( use_datetime = use_datetime , use_builtin_types = use_builtin_types );
        p . feed ( data );
        p . close ( );
        return  u . close ( ) , u . getmethodname ( );
        pub fn gzip_encode ( data )  {
        "data -> gzip encoded data

    Encode data using the gzip content encoding as described in RFC 1952
    ";
        if !gzip {
        panic!("NotImplementedError");
        f = BytesIO ( );
        // with scope: gzip . GzipFile ( mode = "wb" , fileobj = f , compresslevel = 1 ) as gzf  {
        gzf . write ( data );
        return  f . getvalue ( );
        pub fn gzip_decode ( data , max_decode = 20971520 )  {
        "gzip encoded data -> unencoded data

    Decode data using the gzip content encoding as described in RFC 1952
    ";
        if !gzip {
        panic!("NotImplementedError");
        // with scope: gzip . GzipFile ( mode = "rb" , fileobj = BytesIO ( data ) ) as gzf  {
        // try {
        if max_decode < 0 {
        decoded = gzf . read ( );
        } else {
        decoded = gzf . read ( max_decode + 1 );
        // } catch  OSError  {
        panic!("ValueError ( "invalid data" )");
        if max_decode >= 0 && len ( decoded ) > max_decode {
        panic!("ValueError ( "max gzipped payload length exceeded" )");
        return  decoded;
        class GzipDecodedResponse ( gzip . GzipFile if gzip else object ) ;
        "a file-like object to decode a response encoded with the gzip
    method, as described in RFC 1952.
    ";
        pub fn __init__ ( &self, response )  {
        if !gzip {
        panic!("NotImplementedError");
        self . io = BytesIO ( response . read ( ) );
        gzip . GzipFile . __init__ ( self , mode = "rb" , fileobj = self . io );
        pub fn close ( self )  {
        // try {
        gzip . GzipFile . close ( self );
        // } finally {
        self . io . close ( );
        class _Method ;
        pub fn __init__ ( &self, send , name )  {
        self . __send = send;
        self . __name = name;
        pub fn __getattr__ ( &self, name )  {
        return  _Method ( self . __send , "%s.%s" % ( self . __name , name ) );
        pub fn __call__ ( &self, * args )  {
        return  self . __send ( self . __name , args );
        class Transport ;
        "Handles an HTTP transaction to an XML-RPC server.";
        user_agent = "Python-xmlrpc/%s" % __version__;
        accept_gzip_encoding = true;
        encode_threshold = None /* Option */;
        pub fn __init__ ( &self, use_datetime = false , use_builtin_types = false , {
        * , headers = ( ) ) ;
        self . _use_datetime = use_datetime;
        self . _use_builtin_types = use_builtin_types;
        self . _connection = ( None /* Option */ , None /* Option */ );
        self . _headers = list ( headers );
        self . _extra_headers = [ ];
        pub fn request ( &self, host , handler , request_body , verbose = false )  {
        for i in ( 0 , 1 ) .iter() {
        // try {
        return  self . single_request ( host , handler , request_body , verbose );
        // } catch  http . client . RemoteDisconnected  {
        if i {
        panic!("");
        // } catch  OSError as e  {
        if i || e . errno !in ( errno . ECONNRESET , errno . ECONNABORTED , {
        errno . EPIPE ) ;
        panic!("");
        pub fn single_request ( &self, host , handler , request_body , verbose = false )  {
        // try {
        http_conn = self . send_request ( host , handler , request_body , verbose );
        resp = http_conn . getresponse ( );
        if resp . status == 200 {
        self . verbose = verbose;
        return  self . parse_response ( resp );
        // } catch  Fault  {
        panic!("");
        // } catch  Exception  {
        self . close ( );
        panic!("");
        if resp . getheader ( "content-length" , "" ) {
        resp . read ( );
        panic!("ProtocolError (");
        host + handler ,;
        resp . status , resp . reason ,;
        dict ( resp . getheaders ( ) );
        );
        pub fn getparser ( self )  {
        return  getparser ( use_datetime = self . _use_datetime ,;
        use_builtin_types = self . _use_builtin_types );
        pub fn get_host_info ( &self, host )  {
        x509 = { };
        if isinstance ( host , tuple ) {
        host , x509 = host;
        auth , host = urllib . parse . _splituser ( host );
        if auth {
        auth = urllib . parse . unquote_to_bytes ( auth );
        auth = base64 . encodebytes ( auth ) . decode ( "utf-8" );
        auth = "" . join ( auth . split ( ) );
        extra_headers = [;
        ( "Authorization" , "Basic " + auth );
        ];
        } else {
        extra_headers = [ ];
        return  host , extra_headers , x509;
        pub fn make_connection ( &self, host )  {
        if self . _connection && host == self . _connection [ 0 ] {
        return  self . _connection [ 1 ];
        chost , self . _extra_headers , x509 = self . get_host_info ( host );
        self . _connection = host , http . client . HTTPConnection ( chost );
        return  self . _connection [ 1 ];
        pub fn close ( self )  {
        host , connection = self . _connection;
        if connection {
        self . _connection = ( None /* Option */ , None /* Option */ );
        connection . close ( );
        pub fn send_request ( &self, host , handler , request_body , debug )  {
        connection = self . make_connection ( host );
        headers = self . _headers + self . _extra_headers;
        if debug {
        connection . set_debuglevel ( 1 );
        if self . accept_gzip_encoding && gzip {
        connection . putrequest ( "POST" , handler , skip_accept_encoding = true );
        headers . append ( ( "Accept-Encoding" , "gzip" ) );
        } else {
        connection . putrequest ( "POST" , handler );
        headers . append ( ( "Content-Type" , "text/xml" ) );
        headers . append ( ( "User-Agent" , self . user_agent ) );
        self . send_headers ( connection , headers );
        self . send_content ( connection , request_body );
        return  connection;
        pub fn send_headers ( &self, connection , headers )  {
        for key , val in headers .iter() {
        connection . putheader ( key , val );
        pub fn send_content ( &self, connection , request_body )  {
        if ( self . encode_threshold is !None /* Option */ and {
        self . encode_threshold < len ( request_body ) and;
        gzip ) ;
        connection . putheader ( "Content-Encoding" , "gzip" );
        request_body = gzip_encode ( request_body );
        connection . putheader ( "Content-Length" , str ( len ( request_body ) ) );
        connection . endheaders ( request_body );
        pub fn parse_response ( &self, response )  {
        if hasattr ( response , "getheader" ) {
        if response . getheader ( "Content-Encoding" , "" ) == "gzip" {
        stream = GzipDecodedResponse ( response );
        } else {
        stream = response;
        } else {
        stream = response;
        p , u = self . getparser ( );
        while 1  {
        data = stream . read ( 1024 );
        if !data {
        break;
        if self . verbose {
        println!( "body:" , repr ( data ) );
        p . feed ( data );
        if stream is !response {
        stream . close ( );
        p . close ( );
        return  u . close ( );
        class SafeTransport ( Transport ) ;
        "Handles an HTTPS transaction to an XML-RPC server.";
        pub fn __init__ ( &self, use_datetime = false , use_builtin_types = false , {
        * , headers = ( ) , context = None /* Option */ ) ;
        super ( ) . __init__ ( use_datetime = use_datetime ,;
        use_builtin_types = use_builtin_types ,;
        headers = headers );
        self . context = context;
        pub fn make_connection ( &self, host )  {
        if self . _connection && host == self . _connection [ 0 ] {
        return  self . _connection [ 1 ];
        if !hasattr ( http . client , "HTTPSConnection" ) {
        panic!("NotImplementedError (");
        "your version of http.client doesn't support HTTPS" );
        chost , self . _extra_headers , x509 = self . get_host_info ( host );
        self . _connection = host , http . client . HTTPSConnection ( chost ,;
        None /* Option */ , context = self . context , ** ( x509 || { } ) );
        return  self . _connection [ 1 ];
        class ServerProxy ;
        "uri [,options] -> a logical connection to an XML-RPC server

    uri == the connection point on the server, given as
    scheme://host/target.

    The standard implementation always supports the "http" scheme.  If
    SSL socket support == available (Python 2.0), it also supports
    "https".

    If the target part && the slash preceding it are both omitted,
    "/RPC2" == assumed.

    The following options can be given as keyword arguments:

        transport: a transport factory
        encoding: the request encoding (default == UTF-8)

    All 8-bit strings passed to the server proxy are assumed to use
    the given encoding.
    ";
        pub fn __init__ ( &self, uri , transport = None /* Option */ , encoding = None /* Option */ , verbose = false , {
        allow_none = false , use_datetime = false , use_builtin_types = false ,;
        * , headers = ( ) , context = None /* Option */ ) ;
        p = urllib . parse . urlsplit ( uri );
        if p . scheme !in ( "http" , "https" ) {
        panic!("OSError ( "unsupported XML-RPC protocol" )");
        self . __host = p . netloc;
        self . __handler = urllib . parse . urlunsplit ( [ "" , "" , * p [ 2 : ] ] );
        if !self . __handler {
        self . __handler = "/RPC2";
        if transport is None /* Option */ {
        if p . scheme == "https" {
        handler = SafeTransport;
        extra_kwargs = { "context" : context };
        } else {
        handler = Transport;
        extra_kwargs = { };
        transport = handler ( use_datetime = use_datetime ,;
        use_builtin_types = use_builtin_types ,;
        headers = headers ,;
        ** extra_kwargs );
        self . __transport = transport;
        self . __encoding = encoding || "utf-8";
        self . __verbose = verbose;
        self . __allow_none = allow_none;
        pub fn __close ( self )  {
        self . __transport . close ( );
        pub fn __request ( &self, methodname , params )  {
        request = dumps ( params , methodname , encoding = self . __encoding ,;
        allow_none = self . __allow_none ) . encode ( self . __encoding , "xmlcharrefreplace" );
        response = self . __transport . request (;
        self . __host ,;
        self . __handler ,;
        request ,;
        verbose = self . __verbose;
        );
        if len ( response ) == 1 {
        response = response [ 0 ];
        return  response;
        pub fn __repr__ ( self )  {
        return  (;
        "<%s for %s%s>" %;
        ( self . __class__ . __name__ , self . __host , self . __handler );
        );
        pub fn __getattr__ ( &self, name )  {
        return  _Method ( self . __request , name );
        pub fn __call__ ( &self, attr )  {
        "A workaround to get special attributes on the ServerProxy
           without interfering with the magic __getattr__
        ";
        if attr == "close" {
        return  self . __close;
        } else if attr == "transport" {
        return  self . __transport;
        panic!("AttributeError ( "Attribute %r !found" % ( attr , ) )");
        pub fn __enter__ ( self )  {
        return  self;
        pub fn __exit__ ( &self, * args )  {
        self . __close ( );
        Server = ServerProxy;
        fn main() {
        server = ServerProxy ( "http://localhost:8000" );
        // try {
        println!( server . currentTime . getCurrentTime ( ) );
        // } catch  Error as v  {
        println!( "ERROR" , v );
        multi = MultiCall ( server );
        multi . getData ( );
        multi . pow ( 2 , 9 );
        multi . add ( 1 , 2 );
        // try {
        for response in multi ( ) .iter() {
        println!( response );
        // } catch  Error as v  {
        println!( "ERROR" , v );
}

