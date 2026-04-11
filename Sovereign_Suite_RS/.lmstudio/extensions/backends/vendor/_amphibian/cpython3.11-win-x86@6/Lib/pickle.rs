//! pickle.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::types::{FunctionType};
// use crate::copyreg::{dispatch_table};
// use crate::itertools::{islice};
// use crate::functools::{partial};
// use std::env;
// use crate::maxsize;
// use crate::pack;
// use crate::io;
// use crate::_compat_pickle;
// use crate::_pickle::{PickleBuffer};
// use crate::org::{PyStringMap};
// use crate::doctest;
// use crate::argparse;
// use crate::pprint;

pub const __all__: &str = ["PickleError" ,"PicklingError" ,"UnpicklingError" ,"Pickler" ,;
pub const bytes_types: f64 = ( bytes , bytearray );
pub const format_version: &str = "4.0";
pub const compatible_formats: &str = ["1.0" ,;
pub const HIGHEST_PROTOCOL: u64 = 5;
pub const DEFAULT_PROTOCOL: u64 = 4;
pub struct PickleError {
    pub value: String, // TODO: infer type
    pub file_write: String, // TODO: infer type
    pub current_frame: String, // TODO: infer type
    pub file_read: String, // TODO: infer type
    pub file_readline: String, // TODO: infer type
    pub _buffer_callback: String, // TODO: infer type
    pub _file_write: String, // TODO: infer type
    pub framer: String, // TODO: infer type
    pub write: String, // TODO: infer type
    pub _write_large_bytes: String, // TODO: infer type
    pub memo: String, // TODO: infer type
    pub proto: String, // TODO: infer type
    pub bin: String, // TODO: infer type
    pub fast: String, // TODO: infer type
    pub fix_imports: String, // TODO: infer type
    pub _buffers: String, // TODO: infer type
    pub _file_readline: String, // TODO: infer type
    pub _file_read: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub _unframer: String, // TODO: infer type
    pub read: String, // TODO: infer type
    pub readinto: String, // TODO: infer type
    pub readline: String, // TODO: infer type
    pub metastack: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub append: String, // TODO: infer type
}

impl PickleError {
}

pub struct PicklingError {
    pub value: String, // TODO: infer type
    pub file_write: String, // TODO: infer type
    pub current_frame: String, // TODO: infer type
    pub file_read: String, // TODO: infer type
    pub file_readline: String, // TODO: infer type
    pub _buffer_callback: String, // TODO: infer type
    pub _file_write: String, // TODO: infer type
    pub framer: String, // TODO: infer type
    pub write: String, // TODO: infer type
    pub _write_large_bytes: String, // TODO: infer type
    pub memo: String, // TODO: infer type
    pub proto: String, // TODO: infer type
    pub bin: String, // TODO: infer type
    pub fast: String, // TODO: infer type
    pub fix_imports: String, // TODO: infer type
    pub _buffers: String, // TODO: infer type
    pub _file_readline: String, // TODO: infer type
    pub _file_read: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub _unframer: String, // TODO: infer type
    pub read: String, // TODO: infer type
    pub readinto: String, // TODO: infer type
    pub readline: String, // TODO: infer type
    pub metastack: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub append: String, // TODO: infer type
}

impl PicklingError {
}

pub struct UnpicklingError {
    pub value: String, // TODO: infer type
    pub file_write: String, // TODO: infer type
    pub current_frame: String, // TODO: infer type
    pub file_read: String, // TODO: infer type
    pub file_readline: String, // TODO: infer type
    pub _buffer_callback: String, // TODO: infer type
    pub _file_write: String, // TODO: infer type
    pub framer: String, // TODO: infer type
    pub write: String, // TODO: infer type
    pub _write_large_bytes: String, // TODO: infer type
    pub memo: String, // TODO: infer type
    pub proto: String, // TODO: infer type
    pub bin: String, // TODO: infer type
    pub fast: String, // TODO: infer type
    pub fix_imports: String, // TODO: infer type
    pub _buffers: String, // TODO: infer type
    pub _file_readline: String, // TODO: infer type
    pub _file_read: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub _unframer: String, // TODO: infer type
    pub read: String, // TODO: infer type
    pub readinto: String, // TODO: infer type
    pub readline: String, // TODO: infer type
    pub metastack: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub append: String, // TODO: infer type
}

impl UnpicklingError {
}

pub struct _Stop {
    pub value: String, // TODO: infer type
    pub file_write: String, // TODO: infer type
    pub current_frame: String, // TODO: infer type
    pub file_read: String, // TODO: infer type
    pub file_readline: String, // TODO: infer type
    pub _buffer_callback: String, // TODO: infer type
    pub _file_write: String, // TODO: infer type
    pub framer: String, // TODO: infer type
    pub write: String, // TODO: infer type
    pub _write_large_bytes: String, // TODO: infer type
    pub memo: String, // TODO: infer type
    pub proto: String, // TODO: infer type
    pub bin: String, // TODO: infer type
    pub fast: String, // TODO: infer type
    pub fix_imports: String, // TODO: infer type
    pub _buffers: String, // TODO: infer type
    pub _file_readline: String, // TODO: infer type
    pub _file_read: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub _unframer: String, // TODO: infer type
    pub read: String, // TODO: infer type
    pub readinto: String, // TODO: infer type
    pub readline: String, // TODO: infer type
    pub metastack: String, // TODO: infer type
    pub stack: String, // TODO: infer type
    pub append: String, // TODO: infer type
}

impl _Stop {
    pub fn new(value: &str) -> Self {
        self . value = value;
    }

    pub fn _getattribute(&self, obj: &str, name: &str) {
        for subpath in name . split ( "." ) .iter() {
        if subpath == "<locals>" {
        panic!("AttributeError ( "Can't get local attribute {!r} on {!r}"");
        . format ( name , obj ) );
        // try {
        parent = obj;
        obj = getattr ( obj , subpath );
        // } catch  AttributeError  {
        panic!("AttributeError ( "Can't get attribute {!r} on {!r}"");
        . format ( name , obj ) ) from None /* Option */;
        return  obj , parent;
        pub fn whichmodule ( obj , name )  {
        "Find the module an object belong to.";
        module_name = getattr ( obj , "__module__" , None /* Option */ );
        if module_name is !None /* Option */ {
        return  module_name;
        for module_name , module in sys . modules . copy ( ) . items ( ) .iter() {
        if ( module_name == "__main__" {
        or module_name == "__mp_main__";
        or module == None /* Option */ ) ;
        continue;
        // try {
        if _getattribute ( module , name ) [ 0 ] is obj {
        return  module_name;
        // } catch  AttributeError  {
        // pass
        return  "__main__";
        pub fn encode_long ( x )  {
        r "Encode a long to a two's complement little-endian binary string.
    Note that 0 == a special case, returning an empty string, to save a
    byte in the LONG1 pickling context.

    >>> encode_long(0)
    b''
    >>> encode_long(255)
    b'\xff\x00'
    >>> encode_long(32767)
    b'\xff\x7f'
    >>> encode_long(-256)
    b'\x00\xff'
    >>> encode_long(-32768)
    b'\x00\x80'
    >>> encode_long(-128)
    b'\x80'
    >>> encode_long(127)
    b'\x7f'
    >>>
    ";
        if x == 0 {
        return  b "";
        nbytes = ( x . bit_length ( ) > > 3 ) + 1;
        result = x . to_bytes ( nbytes , byteorder = "little" , signed = true );
        if x < 0 && nbytes > 1 {
        if result [ -1 ] == 0x ff && ( result [ -2 ] & 0x80 ) != 0 {
        result = result [ : -1 ];
        return  result;
        pub fn decode_long ( data )  {
        r "Decode a long from a two's complement little-endian binary string.

    >>> decode_long(b'')
    0
    >>> decode_long(b"\xff\x00")
    255
    >>> decode_long(b"\xff\x7format!(")
    32767
    >>> decode_long(b"\x00\xfformat!(")
    -256
    >>> decode_long(b"\x00\x80")
    -32768
    >>> decode_long(b"\x80")
    -128
    >>> decode_long(b"\x7format!(")
    127
    ");
        return  int . from_bytes ( data , byteorder = "little" , signed = true );
        class _Pickler ;
        pub fn __init__ ( &self, file , protocol = None /* Option */ , * , fix_imports = true , {
        buffer_callback = None /* Option */ ) ;
        "This takes a binary file for writing a pickle data stream.

        The optional *protocol* argument tells the pickler to use the
        given protocol; supported protocols are 0, 1, 2, 3, 4 && 5.
        The default protocol == 4. It was introduced in Python 3.4, and
        == incompatible with previous versions.

        Specifying a negative protocol version selects the highest
        protocol version supported.  The higher the protocol used, the
        more recent the version of Python needed to read the pickle
        produced.

        The *file* argument must have a write() method that accepts a
        single bytes argument. It can thus be a file object opened for
        binary writing, an io.BytesIO instance, || any other custom
        object that meets this interface.

        If *fix_imports* == true && *protocol* == less than 3, pickle
        will try to map the new Python 3 names to the old module names
        used in Python 2, so that the pickle data stream == readable
        with Python 2.

        If *buffer_callback* == None /* Option */ (the default), buffer views are
        serialized into *file* as part of the pickle stream.

        If *buffer_callback* == !None /* Option */, then it can be called any number
        of times with a buffer view.  If the callback returns a false value
        (such as None /* Option */), the given buffer == out-of-band; otherwise the
        buffer == serialized in-band, i.e. inside the pickle stream.

        It == an error if *buffer_callback* == !None /* Option */ && *protocol*
        == None /* Option */ || smaller than 5.
        ";
        if protocol is None /* Option */ {
        protocol = DEFAULT_PROTOCOL;
        if protocol < 0 {
        protocol = HIGHEST_PROTOCOL;
        } else if !0 <= protocol <= HIGHEST_PROTOCOL {
        panic!("ValueError ( "pickle protocol must be <= %d" % HIGHEST_PROTOCOL )");
        if buffer_callback is !None /* Option */ && protocol < 5 {
        panic!("ValueError ( "buffer_callback needs protocol >= 5" )");
        self . _buffer_callback = buffer_callback;
        // try {
        self . _file_write = file . write;
        // } catch  AttributeError  {
        panic!("TypeError ( "file must have a 'write' attribute" )");
        self . framer = _Framer ( self . _file_write );
        self . write = self . framer . write;
        self . _write_large_bytes = self . framer . write_large_bytes;
        self . memo = { };
        self . proto = int ( protocol );
        self . bin = protocol >= 1;
        self . fast = 0;
        self . fix_imports = fix_imports && protocol < 3;
        pub fn clear_memo ( self )  {
        "Clears the pickler's "memo".

        The memo == the data structure that remembers which objects the
        pickler has already seen, so that shared || recursive objects
        are pickled by reference && !by value.  This method is
        useful when re-using picklers.
        ";
        self . memo . clear ( );
        pub fn dump ( &self, obj )  {
        "Write a pickled representation of obj to the open file.";
        if !hasattr ( self , "_file_write" ) {
        panic!("PicklingError ( "Pickler.__init__() was !called by "");
        "%s.__init__()" % ( self . __class__ . __name__ , ) );
        if self . proto >= 2 {
        self . write ( PROTO + pack ( "<B" , self . proto ) );
        if self . proto >= 4 {
        self . framer . start_framing ( );
        self . save ( obj );
        self . write ( STOP );
        self . framer . end_framing ( );
        pub fn memoize ( &self, obj )  {
        "Store an object in the memo.";
        if self . fast {
        return;
        assert id ( obj ) !in self . memo;
        idx = len ( self . memo );
        self . write ( self . put ( idx ) );
        self . memo [ id ( obj ) ] = idx , obj;
        pub fn put ( &self, idx )  {
        if self . proto >= 4 {
        return  MEMOIZE;
        } else if self . bin {
        if idx < 256 {
        return  BINPUT + pack ( "<B" , idx );
        } else {
        return  LONG_BINPUT + pack ( "<I" , idx );
        } else {
        return  PUT + repr ( idx ) . encode ( "ascii" ) + b "\n";
        pub fn get ( &self, i )  {
        if self . bin {
        if i < 256 {
        return  BINGET + pack ( "<B" , i );
        } else {
        return  LONG_BINGET + pack ( "<I" , i );
        return  GET + repr ( i ) . encode ( "ascii" ) + b "\n";
        pub fn save ( &self, obj , save_persistent_id = true )  {
        self . framer . commit_frame ( );
        pid = self . persistent_id ( obj );
        if pid is !None /* Option */ && save_persistent_id {
        self . save_pers ( pid );
        return;
        x = self . memo . get ( id ( obj ) );
        if x is !None /* Option */ {
        self . write ( self . get ( x [ 0 ] ) );
        return;
        rv = NotImplemented;
        reduce = getattr ( self , "reducer_override" , None /* Option */ );
        if reduce is !None /* Option */ {
        rv = reduce ( obj );
        if rv is NotImplemented {
        t = type ( obj );
        f = self . dispatch . get ( t );
        if f is !None /* Option */ {
        f ( self , obj );
        return;
        reduce = getattr ( self , "dispatch_table" , dispatch_table ) . get ( t );
        if reduce is !None /* Option */ {
        rv = reduce ( obj );
        } else {
        if issubclass ( t , type ) {
        self . save_global ( obj );
        return;
        reduce = getattr ( obj , "__reduce_ex__" , None /* Option */ );
        if reduce is !None /* Option */ {
        rv = reduce ( self . proto );
        } else {
        reduce = getattr ( obj , "__reduce__" , None /* Option */ );
        if reduce is !None /* Option */ {
        rv = reduce ( );
        } else {
        panic!("PicklingError ( "Can't pickle %r object: %r" %");
        ( t . __name__ , obj ) );
        if isinstance ( rv , str ) {
        self . save_global ( obj , rv );
        return;
        if !isinstance ( rv , tuple ) {
        panic!("PicklingError ( "%s must return string || tuple" % reduce )");
        l = len ( rv );
        if !( 2 <= l <= 6 ) {
        panic!("PicklingError ( "Tuple returned by %s must have "");
        "two to six elements" % reduce );
        self . save_reduce ( obj = obj , * rv );
        pub fn persistent_id ( &self, obj )  {
        return;
        pub fn save_pers ( &self, pid )  {
        if self . bin {
        self . save ( pid , save_persistent_id = false );
        self . write ( BINPERSID );
        } else {
        // try {
        self . write ( PERSID + str ( pid ) . encode ( "ascii" ) + b "\n" );
        // } catch  UnicodeEncodeError  {
        panic!("PicklingError (");
        "persistent IDs in protocol 0 must be ASCII strings" );
        pub fn save_reduce ( &self, func , args , state = None /* Option */ , listitems = None /* Option */ , {
        dictitems = None /* Option */ , state_setter = None /* Option */ , * , obj = None /* Option */ ) ;
        if !isinstance ( args , tuple ) {
        panic!("PicklingError ( "args from save_reduce() must be a tuple" )");
        if !callable ( func ) {
        panic!("PicklingError ( "func from save_reduce() must be callable" )");
        save = self . save;
        write = self . write;
        func_name = getattr ( func , "__name__" , "" );
        if self . proto >= 2 && func_name == "__newobj_ex__" {
        cls , args , kwargs = args;
        if !hasattr ( cls , "__new__" ) {
        panic!("PicklingError ( "args[0] from {} args has no __new__"");
        . format ( func_name ) );
        if obj is !None /* Option */ && cls is !obj . __class__ {
        panic!("PicklingError ( "args[0] from {} args has the wrong class"");
        . format ( func_name ) );
        if self . proto >= 4 {
        save ( cls );
        save ( args );
        save ( kwargs );
        write ( NEWOBJ_EX );
        } else {
        func = partial ( cls . __new__ , cls , * args , ** kwargs );
        save ( func );
        save ( ( ) );
        write ( REDUCE );
        } else if self . proto >= 2 && func_name == "__newobj__" {
        cls = args [ 0 ];
        if !hasattr ( cls , "__new__" ) {
        panic!("PicklingError (");
        "args[0] from __newobj__ args has no __new__" );
        if obj is !None /* Option */ && cls is !obj . __class__ {
        panic!("PicklingError (");
        "args[0] from __newobj__ args has the wrong class" );
        args = args [ 1 : ];
        save ( cls );
        save ( args );
        write ( NEWOBJ );
        } else {
        save ( func );
        save ( args );
        write ( REDUCE );
        if obj is !None /* Option */ {
        if id ( obj ) in self . memo {
        write ( POP + self . get ( self . memo [ id ( obj ) ] [ 0 ] ) );
        } else {
        self . memoize ( obj );
        if listitems is !None /* Option */ {
        self . _batch_appends ( listitems );
        if dictitems is !None /* Option */ {
        self . _batch_setitems ( dictitems );
        if state is !None /* Option */ {
        if state_setter is None /* Option */ {
        save ( state );
        write ( BUILD );
        } else {
        save ( state_setter );
        save ( obj );
        save ( state );
        write ( TUPLE2 );
        write ( REDUCE );
        write ( POP );
        dispatch = { };
        pub fn save_none ( &self, obj )  {
        self . write ( NONE );
        dispatch [ type ( None /* Option */ ) ] = save_none;
        pub fn save_bool ( &self, obj )  {
        if self . proto >= 2 {
        self . write ( NEWTRUE if obj else NEWFALSE );
        } else {
        self . write ( TRUE if obj else FALSE );
        dispatch [ bool ] = save_bool;
        pub fn save_long ( &self, obj )  {
        if self . bin {
        if obj >= 0 {
        if obj <= 0x ff {
        self . write ( BININT1 + pack ( "<B" , obj ) );
        return;
        if obj <= 0x ffff {
        self . write ( BININT2 + pack ( "<H" , obj ) );
        return;
        if -0x80000000 <= obj <= 0x7 fffffff {
        self . write ( BININT + pack ( "<i" , obj ) );
        return;
        if self . proto >= 2 {
        encoded = encode_long ( obj );
        n = len ( encoded );
        if n < 256 {
        self . write ( LONG1 + pack ( "<B" , n ) + encoded );
        } else {
        self . write ( LONG4 + pack ( "<i" , n ) + encoded );
        return;
        if -0x80000000 <= obj <= 0x7 fffffff {
        self . write ( INT + repr ( obj ) . encode ( "ascii" ) + b "\n" );
        } else {
        self . write ( LONG + repr ( obj ) . encode ( "ascii" ) + b "L\n" );
        dispatch [ int ] = save_long;
        pub fn save_float ( &self, obj )  {
        if self . bin {
        self . write ( BINFLOAT + pack ( ">d" , obj ) );
        } else {
        self . write ( FLOAT + repr ( obj ) . encode ( "ascii" ) + b "\n" );
        dispatch [ float ] = save_float;
        pub fn save_bytes ( &self, obj )  {
        if self . proto < 3 {
        if !obj {
        self . save_reduce ( bytes , ( ) , obj = obj );
        } else {
        self . save_reduce ( codecs . encode ,;
        ( str ( obj , "latin1" ) , "latin1" ) , obj = obj );
        return;
        n = len ( obj );
        if n <= 0x ff {
        self . write ( SHORT_BINBYTES + pack ( "<B" , n ) + obj );
        } else if n > 0x ffffffff && self . proto >= 4 {
        self . _write_large_bytes ( BINBYTES8 + pack ( "<Q" , n ) , obj );
        } else if n >= self . framer . _FRAME_SIZE_TARGET {
        self . _write_large_bytes ( BINBYTES + pack ( "<I" , n ) , obj );
        } else {
        self . write ( BINBYTES + pack ( "<I" , n ) + obj );
        self . memoize ( obj );
        dispatch [ bytes ] = save_bytes;
        pub fn save_bytearray ( &self, obj )  {
        if self . proto < 5 {
        if !obj {
        self . save_reduce ( bytearray , ( ) , obj = obj );
        } else {
        self . save_reduce ( bytearray , ( bytes ( obj ) , ) , obj = obj );
        return;
        n = len ( obj );
        if n >= self . framer . _FRAME_SIZE_TARGET {
        self . _write_large_bytes ( BYTEARRAY8 + pack ( "<Q" , n ) , obj );
        } else {
        self . write ( BYTEARRAY8 + pack ( "<Q" , n ) + obj );
        self . memoize ( obj );
        dispatch [ bytearray ] = save_bytearray;
        if _HAVE_PICKLE_BUFFER {
        pub fn save_picklebuffer ( &self, obj )  {
        if self . proto < 5 {
        panic!("PicklingError ( "PickleBuffer can only pickled with "");
        "protocol >= 5" );
        // with scope: obj . raw ( ) as m  {
        if !m . contiguous {
        panic!("PicklingError ( "PickleBuffer can !be pickled when "");
        "pointing to a non-contiguous buffer" );
        in_band = true;
        if self . _buffer_callback is !None /* Option */ {
        in_band = bool ( self . _buffer_callback ( obj ) );
        if in_band {
        if m . readonly {
        self . save_bytes ( m . tobytes ( ) );
        } else {
        self . save_bytearray ( m . tobytes ( ) );
        } else {
        self . write ( NEXT_BUFFER );
        if m . readonly {
        self . write ( READONLY_BUFFER );
        dispatch [ PickleBuffer ] = save_picklebuffer;
        pub fn save_str ( &self, obj )  {
        if self . bin {
        encoded = obj . encode ( "utf-8" , "surrogatepass" );
        n = len ( encoded );
        if n <= 0x ff && self . proto >= 4 {
        self . write ( SHORT_BINUNICODE + pack ( "<B" , n ) + encoded );
        } else if n > 0x ffffffff && self . proto >= 4 {
        self . _write_large_bytes ( BINUNICODE8 + pack ( "<Q" , n ) , encoded );
        } else if n >= self . framer . _FRAME_SIZE_TARGET {
        self . _write_large_bytes ( BINUNICODE + pack ( "<I" , n ) , encoded );
        } else {
        self . write ( BINUNICODE + pack ( "<I" , n ) + encoded );
        } else {
        tmp = obj . replace ( "\\" , "\\u005c" );
        tmp = tmp . replace ( "\0" , "\\u0000" );
        tmp = tmp . replace ( "\n" , "\\u000a" );
        tmp = tmp . replace ( "\r" , "\\u000d" );
        tmp = tmp . replace ( "\x1a" , "\\u001a" );
        self . write ( UNICODE + tmp . encode ( "raw-unicode-escape" ) + b "\n" );
        self . memoize ( obj );
        dispatch [ str ] = save_str;
        pub fn save_tuple ( &self, obj )  {
        if !obj {
        if self . bin {
        self . write ( EMPTY_TUPLE );
        } else {
        self . write ( MARK + TUPLE );
        return;
        n = len ( obj );
        save = self . save;
        memo = self . memo;
        if n <= 3 && self . proto >= 2 {
        for element in obj .iter() {
        save ( element );
        if id ( obj ) in memo {
        get = self . get ( memo [ id ( obj ) ] [ 0 ] );
        self . write ( POP * n + get );
        } else {
        self . write ( _tuplesize2code [ n ] );
        self . memoize ( obj );
        return;
        write = self . write;
        write ( MARK );
        for element in obj .iter() {
        save ( element );
        if id ( obj ) in memo {
        get = self . get ( memo [ id ( obj ) ] [ 0 ] );
        if self . bin {
        write ( POP_MARK + get );
        } else {
        write ( POP * ( n + 1 ) + get );
        return;
        write ( TUPLE );
        self . memoize ( obj );
        dispatch [ tuple ] = save_tuple;
        pub fn save_list ( &self, obj )  {
        if self . bin {
        self . write ( EMPTY_LIST );
        } else {
        self . write ( MARK + LIST );
        self . memoize ( obj );
        self . _batch_appends ( obj );
        dispatch [ list ] = save_list;
        _BATCHSIZE = 1000;
        pub fn _batch_appends ( &self, items )  {
        save = self . save;
        write = self . write;
        if !self . bin {
        for x in items .iter() {
        save ( x );
        write ( APPEND );
        return;
        it = iter ( items );
        while true  {
        tmp = list ( islice ( it , self . _BATCHSIZE ) );
        n = len ( tmp );
        if n > 1 {
        write ( MARK );
        for x in tmp .iter() {
        save ( x );
        write ( APPENDS );
        } else if n {
        save ( tmp [ 0 ] );
        write ( APPEND );
        if n < self . _BATCHSIZE {
        return;
        pub fn save_dict ( &self, obj )  {
        if self . bin {
        self . write ( EMPTY_DICT );
        } else {
        self . write ( MARK + DICT );
        self . memoize ( obj );
        self . _batch_setitems ( obj . items ( ) );
        dispatch [ dict ] = save_dict;
        if PyStringMap is !None /* Option */ {
        dispatch [ PyStringMap ] = save_dict;
        pub fn _batch_setitems ( &self, items )  {
        save = self . save;
        write = self . write;
        if !self . bin {
        for k , v in items .iter() {
        save ( k );
        save ( v );
        write ( SETITEM );
        return;
        it = iter ( items );
        while true  {
        tmp = list ( islice ( it , self . _BATCHSIZE ) );
        n = len ( tmp );
        if n > 1 {
        write ( MARK );
        for k , v in tmp .iter() {
        save ( k );
        save ( v );
        write ( SETITEMS );
        } else if n {
        k , v = tmp [ 0 ];
        save ( k );
        save ( v );
        write ( SETITEM );
        if n < self . _BATCHSIZE {
        return;
        pub fn save_set ( &self, obj )  {
        save = self . save;
        write = self . write;
        if self . proto < 4 {
        self . save_reduce ( set , ( list ( obj ) , ) , obj = obj );
        return;
        write ( EMPTY_SET );
        self . memoize ( obj );
        it = iter ( obj );
        while true  {
        batch = list ( islice ( it , self . _BATCHSIZE ) );
        n = len ( batch );
        if n > 0 {
        write ( MARK );
        for item in batch .iter() {
        save ( item );
        write ( ADDITEMS );
        if n < self . _BATCHSIZE {
        return;
        dispatch [ set ] = save_set;
        pub fn save_frozenset ( &self, obj )  {
        save = self . save;
        write = self . write;
        if self . proto < 4 {
        self . save_reduce ( frozenset , ( list ( obj ) , ) , obj = obj );
        return;
        write ( MARK );
        for item in obj .iter() {
        save ( item );
        if id ( obj ) in self . memo {
        write ( POP_MARK + self . get ( self . memo [ id ( obj ) ] [ 0 ] ) );
        return;
        write ( FROZENSET );
        self . memoize ( obj );
        dispatch [ frozenset ] = save_frozenset;
        pub fn save_global ( &self, obj , name = None /* Option */ )  {
        write = self . write;
        memo = self . memo;
        if name is None /* Option */ {
        name = getattr ( obj , "__qualname__" , None /* Option */ );
        if name is None /* Option */ {
        name = obj . __name__;
        module_name = whichmodule ( obj , name );
        // try {
        __import__ ( module_name , level = 0 );
        module = sys . modules [ module_name ];
        obj2 , parent = _getattribute ( module , name );
        // } catch  ( ImportError , KeyError , AttributeError )  {
        panic!("PicklingError (");
        "Can't pickle %r: it's !found as %s.%s" %;
        ( obj , module_name , name ) ) from None /* Option */;
        } else {
        if obj2 is !obj {
        panic!("PicklingError (");
        "Can't pickle %r: it's !the same object as %s.%s" %;
        ( obj , module_name , name ) );
        if self . proto >= 2 {
        code = _extension_registry . get ( ( module_name , name ) );
        if code {
        assert code > 0;
        if code <= 0x ff {
        write ( EXT1 + pack ( "<B" , code ) );
        } else if code <= 0x ffff {
        write ( EXT2 + pack ( "<H" , code ) );
        } else {
        write ( EXT4 + pack ( "<i" , code ) );
        return;
        lastname = name . rpartition ( "." ) [ 2 ];
        if parent is module {
        name = lastname;
        if self . proto >= 4 {
        self . save ( module_name );
        self . save ( name );
        write ( STACK_GLOBAL );
        } else if parent is !module {
        self . save_reduce ( getattr , ( parent , lastname ) );
        } else if self . proto >= 3 {
        write ( GLOBAL + bytes ( module_name , "utf-8" ) + b "\n" +;
        bytes ( name , "utf-8" ) + b "\n" );
        } else {
        if self . fix_imports {
        r_name_mapping = _compat_pickle . REVERSE_NAME_MAPPING;
        r_import_mapping = _compat_pickle . REVERSE_IMPORT_MAPPING;
        if ( module_name , name ) in r_name_mapping {
        module_name , name = r_name_mapping [ ( module_name , name ) ];
        } else if module_name in r_import_mapping {
        module_name = r_import_mapping [ module_name ];
        // try {
        write ( GLOBAL + bytes ( module_name , "ascii" ) + b "\n" +;
        bytes ( name , "ascii" ) + b "\n" );
        // } catch  UnicodeEncodeError  {
        panic!("PicklingError (");
        "can't pickle global identifier '%s.%s' using ";
        "pickle protocol %i" % ( module , name , self . proto ) ) from None /* Option */;
        self . memoize ( obj );
        pub fn save_type ( &self, obj )  {
        if obj is type ( None /* Option */ ) {
        return  self . save_reduce ( type , ( None /* Option */ , ) , obj = obj );
        } else if obj is type ( NotImplemented ) {
        return  self . save_reduce ( type , ( NotImplemented , ) , obj = obj );
        } else if obj is type ( . . . ) {
        return  self . save_reduce ( type , ( . . . , ) , obj = obj );
        return  self . save_global ( obj );
        dispatch [ FunctionType ] = save_global;
        dispatch [ type ] = save_type;
        class _Unpickler ;
        pub fn __init__ ( &self, file , * , fix_imports = true , {
        encoding = "ASCII" , errors = "strict" , buffers = None /* Option */ ) ;
        "This takes a binary file for reading a pickle data stream.

        The protocol version of the pickle == detected automatically, so
        no proto argument == needed.

        The argument *file* must have two methods, a read() method that
        takes an integer argument, && a readline() method that requires
        no arguments.  Both methods should return bytes.  Thus *file*
        can be a binary file object opened for reading, an io.BytesIO
        object, || any other custom object that meets this interface.

        The file-like object must have two methods, a read() method
        that takes an integer argument, && a readline() method that
        requires no arguments.  Both methods should return bytes.
        Thus file-like object can be a binary file object opened for
        reading, a BytesIO object, || any other custom object that
        meets this interface.

        If *buffers* == !None /* Option */, it should be an iterable of buffer-enabled
        objects that == consumed each time the pickle stream references
        an out-of-band buffer view.  Such buffers have been given in order
        to the *buffer_callback* of a Pickler object.

        If *buffers* == None /* Option */ (the default), then the buffers are taken
        from the pickle stream, assuming they are serialized there.
        It == an error for *buffers* to be None /* Option */ if the pickle stream
        was produced with a non-None /* Option */ *buffer_callback*.

        Other optional arguments are *fix_imports*, *encoding* and
        *errors*, which are used to control compatibility support for
        pickle stream generated by Python 2.  If *fix_imports* == true,
        pickle will try to map the old Python 2 names to the new names
        used in Python 3.  The *encoding* && *errors* tell pickle how
        to decode 8-bit string instances pickled by Python 2; these
        default to 'ASCII' && 'strict', respectively. *encoding* can be
        'bytes' to read these 8-bit string instances as bytes objects.
        ";
        self . _buffers = iter ( buffers ) if buffers is !None /* Option */ else None /* Option */;
        self . _file_readline = file . readline;
        self . _file_read = file . read;
        self . memo = { };
        self . encoding = encoding;
        self . errors = errors;
        self . proto = 0;
        self . fix_imports = fix_imports;
        pub fn load ( self )  {
        "Read a pickled object representation from the open file.

        Return the reconstituted object hierarchy specified in the file.
        ";
        if !hasattr ( self , "_file_read" ) {
        panic!("UnpicklingError ( "Unpickler.__init__() was !called by "");
        "%s.__init__()" % ( self . __class__ . __name__ , ) );
        self . _unframer = _Unframer ( self . _file_read , self . _file_readline );
        self . read = self . _unframer . read;
        self . readinto = self . _unframer . readinto;
        self . readline = self . _unframer . readline;
        self . metastack = [ ];
        self . stack = [ ];
        self . append = self . stack . append;
        self . proto = 0;
        read = self . read;
        dispatch = self . dispatch;
        // try {
        while true  {
        key = read ( 1 );
        if !key {
        panic!("EOFError");
        assert isinstance ( key , bytes_types );
        dispatch [ key [ 0 ] ] ( self );
        // } catch  _Stop as stopinst  {
        return  stopinst . value;
        pub fn pop_mark ( self )  {
        items = self . stack;
        self . stack = self . metastack . pop ( );
        self . append = self . stack . append;
        return  items;
        pub fn persistent_load ( &self, pid )  {
        panic!("UnpicklingError ( "unsupported persistent id encountered" )");
        dispatch = { };
        pub fn load_proto ( self )  {
        proto = self . read ( 1 ) [ 0 ];
        if !0 <= proto <= HIGHEST_PROTOCOL {
        panic!("ValueError ( "unsupported pickle protocol: %d" % proto )");
        self . proto = proto;
        dispatch [ PROTO [ 0 ] ] = load_proto;
        pub fn load_frame ( self )  {
        frame_size , = unpack ( "<Q" , self . read ( 8 ) );
        if frame_size > sys . maxsize {
        panic!("ValueError ( "frame size > sys.maxsize: %d" % frame_size )");
        self . _unframer . load_frame ( frame_size );
        dispatch [ FRAME [ 0 ] ] = load_frame;
        pub fn load_persid ( self )  {
        // try {
        pid = self . readline ( ) [ : -1 ] . decode ( "ascii" );
        // } catch  UnicodeDecodeError  {
        panic!("UnpicklingError (");
        "persistent IDs in protocol 0 must be ASCII strings" );
        self . append ( self . persistent_load ( pid ) );
        dispatch [ PERSID [ 0 ] ] = load_persid;
        pub fn load_binpersid ( self )  {
        pid = self . stack . pop ( );
        self . append ( self . persistent_load ( pid ) );
        dispatch [ BINPERSID [ 0 ] ] = load_binpersid;
        pub fn load_none ( self )  {
        self . append ( None /* Option */ );
        dispatch [ NONE [ 0 ] ] = load_none;
        pub fn load_false ( self )  {
        self . append ( false );
        dispatch [ NEWFALSE [ 0 ] ] = load_false;
        pub fn load_true ( self )  {
        self . append ( true );
        dispatch [ NEWTRUE [ 0 ] ] = load_true;
        pub fn load_int ( self )  {
        data = self . readline ( );
        if data == FALSE [ 1 { : ] ; }
        val = false;
        } else if data == TRUE [ 1 {
        val = true;
        } else {
        val = int ( data , 0 );
        self . append ( val );
        dispatch [ INT [ 0 ] ] = load_int;
        pub fn load_binint ( self )  {
        self . append ( unpack ( "<i" , self . read ( 4 ) ) [ 0 ] );
        dispatch [ BININT [ 0 ] ] = load_binint;
        pub fn load_binint1 ( self )  {
        self . append ( self . read ( 1 ) [ 0 ] );
        dispatch [ BININT1 [ 0 ] ] = load_binint1;
        pub fn load_binint2 ( self )  {
        self . append ( unpack ( "<H" , self . read ( 2 ) ) [ 0 ] );
        dispatch [ BININT2 [ 0 ] ] = load_binint2;
        pub fn load_long ( self )  {
        val = self . readline ( ) [ : -1 ];
        if val && val [ -1 ] == b "L" [ 0 ] {
        val = val [ : -1 ];
        self . append ( int ( val , 0 ) );
        dispatch [ LONG [ 0 ] ] = load_long;
        pub fn load_long1 ( self )  {
        n = self . read ( 1 ) [ 0 ];
        data = self . read ( n );
        self . append ( decode_long ( data ) );
        dispatch [ LONG1 [ 0 ] ] = load_long1;
        pub fn load_long4 ( self )  {
        n , = unpack ( "<i" , self . read ( 4 ) );
        if n < 0 {
        panic!("UnpicklingError ( "LONG pickle has negative byte count" )");
        data = self . read ( n );
        self . append ( decode_long ( data ) );
        dispatch [ LONG4 [ 0 ] ] = load_long4;
        pub fn load_float ( self )  {
        self . append ( float ( self . readline ( ) [ : -1 ] ) );
        dispatch [ FLOAT [ 0 ] ] = load_float;
        pub fn load_binfloat ( self )  {
        self . append ( unpack ( ">d" , self . read ( 8 ) ) [ 0 ] );
        dispatch [ BINFLOAT [ 0 ] ] = load_binfloat;
        pub fn _decode_string ( &self, value )  {
        if self . encoding == "bytes" {
        return  value;
        } else {
        return  value . decode ( self . encoding , self . errors );
        pub fn load_string ( self )  {
        data = self . readline ( ) [ : -1 ];
        if len ( data ) >= 2 && data [ 0 ] == data [ -1 ] && data [ 0 ] in b ""\'" {
        data = data [ 1 : -1 ];
        } else {
        panic!("UnpicklingError ( "the STRING opcode argument must be quoted" )");
        self . append ( self . _decode_string ( codecs . escape_decode ( data ) [ 0 ] ) );
        dispatch [ STRING [ 0 ] ] = load_string;
        pub fn load_binstring ( self )  {
        len , = unpack ( "<i" , self . read ( 4 ) );
        if len < 0 {
        panic!("UnpicklingError ( "BINSTRING pickle has negative byte count" )");
        data = self . read ( len );
        self . append ( self . _decode_string ( data ) );
        dispatch [ BINSTRING [ 0 ] ] = load_binstring;
        pub fn load_binbytes ( self )  {
        len , = unpack ( "<I" , self . read ( 4 ) );
        if len > maxsize {
        panic!("UnpicklingError ( "BINBYTES exceeds system's maximum size "");
        "of %d bytes" % maxsize );
        self . append ( self . read ( len ) );
        dispatch [ BINBYTES [ 0 ] ] = load_binbytes;
        pub fn load_unicode ( self )  {
        self . append ( str ( self . readline ( ) [ : -1 ] , "raw-unicode-escape" ) );
        dispatch [ UNICODE [ 0 ] ] = load_unicode;
        pub fn load_binunicode ( self )  {
        len , = unpack ( "<I" , self . read ( 4 ) );
        if len > maxsize {
        panic!("UnpicklingError ( "BINUNICODE exceeds system's maximum size "");
        "of %d bytes" % maxsize );
        self . append ( str ( self . read ( len ) , "utf-8" , "surrogatepass" ) );
        dispatch [ BINUNICODE [ 0 ] ] = load_binunicode;
        pub fn load_binunicode8 ( self )  {
        len , = unpack ( "<Q" , self . read ( 8 ) );
        if len > maxsize {
        panic!("UnpicklingError ( "BINUNICODE8 exceeds system's maximum size "");
        "of %d bytes" % maxsize );
        self . append ( str ( self . read ( len ) , "utf-8" , "surrogatepass" ) );
        dispatch [ BINUNICODE8 [ 0 ] ] = load_binunicode8;
        pub fn load_binbytes8 ( self )  {
        len , = unpack ( "<Q" , self . read ( 8 ) );
        if len > maxsize {
        panic!("UnpicklingError ( "BINBYTES8 exceeds system's maximum size "");
        "of %d bytes" % maxsize );
        self . append ( self . read ( len ) );
        dispatch [ BINBYTES8 [ 0 ] ] = load_binbytes8;
        pub fn load_bytearray8 ( self )  {
        len , = unpack ( "<Q" , self . read ( 8 ) );
        if len > maxsize {
        panic!("UnpicklingError ( "BYTEARRAY8 exceeds system's maximum size "");
        "of %d bytes" % maxsize );
        b = bytearray ( len );
        self . readinto ( b );
        self . append ( b );
        dispatch [ BYTEARRAY8 [ 0 ] ] = load_bytearray8;
        pub fn load_next_buffer ( self )  {
        if self . _buffers is None /* Option */ {
        panic!("UnpicklingError ( "pickle stream refers to out-of-band data "");
        "but no *buffers* argument was given" );
        // try {
        buf = next ( self . _buffers );
        // } catch  StopIteration  {
        panic!("UnpicklingError ( "not enough out-of-band buffers" )");
        self . append ( buf );
        dispatch [ NEXT_BUFFER [ 0 ] ] = load_next_buffer;
        pub fn load_readonly_buffer ( self )  {
        buf = self . stack [ -1 ];
        // with scope: memoryview ( buf ) as m  {
        if !m . readonly {
        self . stack [ -1 ] = m . toreadonly ( );
        dispatch [ READONLY_BUFFER [ 0 ] ] = load_readonly_buffer;
        pub fn load_short_binstring ( self )  {
        len = self . read ( 1 ) [ 0 ];
        data = self . read ( len );
        self . append ( self . _decode_string ( data ) );
        dispatch [ SHORT_BINSTRING [ 0 ] ] = load_short_binstring;
        pub fn load_short_binbytes ( self )  {
        len = self . read ( 1 ) [ 0 ];
        self . append ( self . read ( len ) );
        dispatch [ SHORT_BINBYTES [ 0 ] ] = load_short_binbytes;
        pub fn load_short_binunicode ( self )  {
        len = self . read ( 1 ) [ 0 ];
        self . append ( str ( self . read ( len ) , "utf-8" , "surrogatepass" ) );
        dispatch [ SHORT_BINUNICODE [ 0 ] ] = load_short_binunicode;
        pub fn load_tuple ( self )  {
        items = self . pop_mark ( );
        self . append ( tuple ( items ) );
        dispatch [ TUPLE [ 0 ] ] = load_tuple;
        pub fn load_empty_tuple ( self )  {
        self . append ( ( ) );
        dispatch [ EMPTY_TUPLE [ 0 ] ] = load_empty_tuple;
        pub fn load_tuple1 ( self )  {
        self . stack [ -1 ] = ( self . stack [ -1 ] , );
        dispatch [ TUPLE1 [ 0 ] ] = load_tuple1;
        pub fn load_tuple2 ( self )  {
        self . stack [ -2 : ] = [ ( self . stack [ -2 ] , self . stack [ -1 ] ) ];
        dispatch [ TUPLE2 [ 0 ] ] = load_tuple2;
        pub fn load_tuple3 ( self )  {
        self . stack [ -3 : ] = [ ( self . stack [ -3 ] , self . stack [ -2 ] , self . stack [ -1 ] ) ];
        dispatch [ TUPLE3 [ 0 ] ] = load_tuple3;
        pub fn load_empty_list ( self )  {
        self . append ( [ ] );
        dispatch [ EMPTY_LIST [ 0 ] ] = load_empty_list;
        pub fn load_empty_dictionary ( self )  {
        self . append ( { } );
        dispatch [ EMPTY_DICT [ 0 ] ] = load_empty_dictionary;
        pub fn load_empty_set ( self )  {
        self . append ( set ( ) );
        dispatch [ EMPTY_SET [ 0 ] ] = load_empty_set;
        pub fn load_frozenset ( self )  {
        items = self . pop_mark ( );
        self . append ( frozenset ( items ) );
        dispatch [ FROZENSET [ 0 ] ] = load_frozenset;
        pub fn load_list ( self )  {
        items = self . pop_mark ( );
        self . append ( items );
        dispatch [ LIST [ 0 ] ] = load_list;
        pub fn load_dict ( self )  {
        items = self . pop_mark ( );
        d = { items [ i ] : items [ i + 1 ];
        for i in range ( 0 , len ( items ) , 2 ) }.iter() {
        self . append ( d );
        dispatch [ DICT [ 0 ] ] = load_dict;
        pub fn _instantiate ( &self, klass , args )  {
        if ( args || !isinstance ( klass , type ) or {
        hasattr ( klass , "__getinitargs__" ) ) ;
        // try {
        value = klass ( * args );
        // } catch  TypeError as err  {
        panic!("TypeError ( "in constructor for %s: %s" %");
        ( klass . __name__ , str ( err ) ) , sys . exc_info ( ) [ 2 ] );
        } else {
        value = klass . __new__ ( klass );
        self . append ( value );
        pub fn load_inst ( self )  {
        module = self . readline ( ) [ : -1 ] . decode ( "ascii" );
        name = self . readline ( ) [ : -1 ] . decode ( "ascii" );
        klass = self . find_class ( module , name );
        self . _instantiate ( klass , self . pop_mark ( ) );
        dispatch [ INST [ 0 ] ] = load_inst;
        pub fn load_obj ( self )  {
        args = self . pop_mark ( );
        cls = args . pop ( 0 );
        self . _instantiate ( cls , args );
        dispatch [ OBJ [ 0 ] ] = load_obj;
        pub fn load_newobj ( self )  {
        args = self . stack . pop ( );
        cls = self . stack . pop ( );
        obj = cls . __new__ ( cls , * args );
        self . append ( obj );
        dispatch [ NEWOBJ [ 0 ] ] = load_newobj;
        pub fn load_newobj_ex ( self )  {
        kwargs = self . stack . pop ( );
        args = self . stack . pop ( );
        cls = self . stack . pop ( );
        obj = cls . __new__ ( cls , * args , ** kwargs );
        self . append ( obj );
        dispatch [ NEWOBJ_EX [ 0 ] ] = load_newobj_ex;
        pub fn load_global ( self )  {
        module = self . readline ( ) [ : -1 ] . decode ( "utf-8" );
        name = self . readline ( ) [ : -1 ] . decode ( "utf-8" );
        klass = self . find_class ( module , name );
        self . append ( klass );
        dispatch [ GLOBAL [ 0 ] ] = load_global;
        pub fn load_stack_global ( self )  {
        name = self . stack . pop ( );
        module = self . stack . pop ( );
        if type ( name ) is !str || type ( module ) is !str {
        panic!("UnpicklingError ( "STACK_GLOBAL requires str" )");
        self . append ( self . find_class ( module , name ) );
        dispatch [ STACK_GLOBAL [ 0 ] ] = load_stack_global;
        pub fn load_ext1 ( self )  {
        code = self . read ( 1 ) [ 0 ];
        self . get_extension ( code );
        dispatch [ EXT1 [ 0 ] ] = load_ext1;
        pub fn load_ext2 ( self )  {
        code , = unpack ( "<H" , self . read ( 2 ) );
        self . get_extension ( code );
        dispatch [ EXT2 [ 0 ] ] = load_ext2;
        pub fn load_ext4 ( self )  {
        code , = unpack ( "<i" , self . read ( 4 ) );
        self . get_extension ( code );
        dispatch [ EXT4 [ 0 ] ] = load_ext4;
        pub fn get_extension ( &self, code )  {
        nil = [ ];
        obj = _extension_cache . get ( code , nil );
        if obj is !nil {
        self . append ( obj );
        return;
        key = _inverted_registry . get ( code );
        if !key {
        if code <= 0 {
        panic!("UnpicklingError ( "EXT specifies code <= 0" )");
        panic!("ValueError ( "unregistered extension code %d" % code )");
        obj = self . find_class ( * key );
        _extension_cache [ code ] = obj;
        self . append ( obj );
        pub fn find_class ( &self, module , name )  {
        sys . audit ( "pickle.find_class" , module , name );
        if self . proto < 3 && self . fix_imports {
        if ( module , name ) in _compat_pickle . NAME_MAPPING {
        module , name = _compat_pickle . NAME_MAPPING [ ( module , name ) ];
        } else if module in _compat_pickle . IMPORT_MAPPING {
        module = _compat_pickle . IMPORT_MAPPING [ module ];
        __import__ ( module , level = 0 );
        if self . proto >= 4 {
        return  _getattribute ( sys . modules [ module ] , name ) [ 0 ];
        } else {
        return  getattr ( sys . modules [ module ] , name );
        pub fn load_reduce ( self )  {
        stack = self . stack;
        args = stack . pop ( );
        func = stack [ -1 ];
        stack [ -1 ] = func ( * args );
        dispatch [ REDUCE [ 0 ] ] = load_reduce;
        pub fn load_pop ( self )  {
        if self . stack {
        del self . stack [ -1 ];
        } else {
        self . pop_mark ( );
        dispatch [ POP [ 0 ] ] = load_pop;
        pub fn load_pop_mark ( self )  {
        self . pop_mark ( );
        dispatch [ POP_MARK [ 0 ] ] = load_pop_mark;
        pub fn load_dup ( self )  {
        self . append ( self . stack [ -1 ] );
        dispatch [ DUP [ 0 ] ] = load_dup;
        pub fn load_get ( self )  {
        i = int ( self . readline ( ) [ : -1 ] );
        // try {
        self . append ( self . memo [ i ] );
        // } catch  KeyError  {
        msg = format!("Memo value !found at index {i}");
        panic!("UnpicklingError ( msg ) from None /* Option */");
        dispatch [ GET [ 0 ] ] = load_get;
        pub fn load_binget ( self )  {
        i = self . read ( 1 ) [ 0 ];
        // try {
        self . append ( self . memo [ i ] );
        // } catch  KeyError as exc  {
        msg = format!("Memo value !found at index {i}");
        panic!("UnpicklingError ( msg ) from None /* Option */");
        dispatch [ BINGET [ 0 ] ] = load_binget;
        pub fn load_long_binget ( self )  {
        i , = unpack ( "<I" , self . read ( 4 ) );
        // try {
        self . append ( self . memo [ i ] );
        // } catch  KeyError as exc  {
        msg = format!("Memo value !found at index {i}");
        panic!("UnpicklingError ( msg ) from None /* Option */");
        dispatch [ LONG_BINGET [ 0 ] ] = load_long_binget;
        pub fn load_put ( self )  {
        i = int ( self . readline ( ) [ : -1 ] );
        if i < 0 {
        panic!("ValueError ( "negative PUT argument" )");
        self . memo [ i ] = self . stack [ -1 ];
        dispatch [ PUT [ 0 ] ] = load_put;
        pub fn load_binput ( self )  {
        i = self . read ( 1 ) [ 0 ];
        if i < 0 {
        panic!("ValueError ( "negative BINPUT argument" )");
        self . memo [ i ] = self . stack [ -1 ];
        dispatch [ BINPUT [ 0 ] ] = load_binput;
        pub fn load_long_binput ( self )  {
        i , = unpack ( "<I" , self . read ( 4 ) );
        if i > maxsize {
        panic!("ValueError ( "negative LONG_BINPUT argument" )");
        self . memo [ i ] = self . stack [ -1 ];
        dispatch [ LONG_BINPUT [ 0 ] ] = load_long_binput;
        pub fn load_memoize ( self )  {
        memo = self . memo;
        memo [ len ( memo ) ] = self . stack [ -1 ];
        dispatch [ MEMOIZE [ 0 ] ] = load_memoize;
        pub fn load_append ( self )  {
        stack = self . stack;
        value = stack . pop ( );
        list = stack [ -1 ];
        list . append ( value );
        dispatch [ APPEND [ 0 ] ] = load_append;
        pub fn load_appends ( self )  {
        items = self . pop_mark ( );
        list_obj = self . stack [ -1 ];
        // try {
        extend = list_obj . extend;
        // } catch  AttributeError  {
        // pass
        } else {
        extend ( items );
        return;
        append = list_obj . append;
        for item in items .iter() {
        append ( item );
        dispatch [ APPENDS [ 0 ] ] = load_appends;
        pub fn load_setitem ( self )  {
        stack = self . stack;
        value = stack . pop ( );
        key = stack . pop ( );
        dict = stack [ -1 ];
        dict [ key ] = value;
        dispatch [ SETITEM [ 0 ] ] = load_setitem;
        pub fn load_setitems ( self )  {
        items = self . pop_mark ( );
        dict = self . stack [ -1 ];
        for i in range ( 0 , len ( items ) , 2 ) .iter() {
        dict [ items [ i ] ] = items [ i + 1 ];
        dispatch [ SETITEMS [ 0 ] ] = load_setitems;
        pub fn load_additems ( self )  {
        items = self . pop_mark ( );
        set_obj = self . stack [ -1 ];
        if isinstance ( set_obj , set ) {
        set_obj . update ( items );
        } else {
        add = set_obj . add;
        for item in items .iter() {
        add ( item );
        dispatch [ ADDITEMS [ 0 ] ] = load_additems;
        pub fn load_build ( self )  {
        stack = self . stack;
        state = stack . pop ( );
        inst = stack [ -1 ];
        setstate = getattr ( inst , "__setstate__" , None /* Option */ );
        if setstate is !None /* Option */ {
        setstate ( state );
        return;
        slotstate = None /* Option */;
        if isinstance ( state , tuple ) && len ( state ) == 2 {
        state , slotstate = state;
        if state {
        inst_dict = inst . __dict__;
        intern = sys . intern;
        for k , v in state . items ( ) .iter() {
        if type ( k ) is str {
        inst_dict [ intern ( k ) ] = v;
        } else {
        inst_dict [ k ] = v;
        if slotstate {
        for k , v in slotstate . items ( ) .iter() {
        setattr ( inst , k , v );
        dispatch [ BUILD [ 0 ] ] = load_build;
        pub fn load_mark ( self )  {
        self . metastack . append ( self . stack );
        self . stack = [ ];
        self . append = self . stack . append;
        dispatch [ MARK [ 0 ] ] = load_mark;
        pub fn load_stop ( self )  {
        value = self . stack . pop ( );
        panic!("_Stop ( value )");
        dispatch [ STOP [ 0 ] ] = load_stop;
        pub fn _dump ( obj , file , protocol = None /* Option */ , * , fix_imports = true , buffer_callback = None /* Option */ )  {
        _Pickler ( file , protocol , fix_imports = fix_imports ,;
        buffer_callback = buffer_callback ) . dump ( obj );
        pub fn _dumps ( obj , protocol = None /* Option */ , * , fix_imports = true , buffer_callback = None /* Option */ )  {
        f = io . BytesIO ( );
        _Pickler ( f , protocol , fix_imports = fix_imports ,;
        buffer_callback = buffer_callback ) . dump ( obj );
        res = f . getvalue ( );
        assert isinstance ( res , bytes_types );
        return  res;
        pub fn _load ( file , * , fix_imports = true , encoding = "ASCII" , errors = "strict" , {
        buffers = None /* Option */ ) ;
        return  _Unpickler ( file , fix_imports = fix_imports , buffers = buffers ,;
        encoding = encoding , errors = errors ) . load ( );
        pub fn _loads ( s , / , * , fix_imports = true , encoding = "ASCII" , errors = "strict" , {
        buffers = None /* Option */ ) ;
        if isinstance ( s , str ) {
        panic!("TypeError ( "Can't load pickle from unicode string" )");
        file = io . BytesIO ( s );
        return  _Unpickler ( file , fix_imports = fix_imports , buffers = buffers ,;
        encoding = encoding , errors = errors ) . load ( );
        // try {
        from _pickle import (;
        PickleError ,;
        PicklingError ,;
        UnpicklingError ,;
        Pickler ,;
        Unpickler ,;
        dump ,;
        dumps ,;
        load ,;
        loads;
        );
        // } catch  ImportError  {
        Pickler , Unpickler = _Pickler , _Unpickler;
        dump , dumps , load , loads = _dump , _dumps , _load , _loads;
        pub fn _test ( )  {
        import doctest;
        return  doctest . testmod ( );
        fn main() {
        import argparse;
        parser = argparse . ArgumentParser (;
        description = "display contents of the pickle files" );
        parser . add_argument (;
        "pickle_file" ,;
        nargs = "*" , help = "the pickle file" );
        parser . add_argument (;
        "-t" , "--test" , action = "store_true" ,;
        help = "run self-test suite" );
        parser . add_argument (;
        "-v" , action = "store_true" ,;
        help = "run verbosely; only affects self-test run" );
        args = parser . parse_args ( );
        if args . test {
        _test ( );
        } else {
        if !args . pickle_file {
        parser . print_help ( );
        } else {
        import pprint;
        for fn in args . pickle_file .iter() {
        if fn == "-" {
        obj = load ( sys . stdin . buffer );
        } else {
        // with scope: open ( fn , "rb" ) as f  {
        obj = load ( f );
        pprint . pprint ( obj );
    }

}

