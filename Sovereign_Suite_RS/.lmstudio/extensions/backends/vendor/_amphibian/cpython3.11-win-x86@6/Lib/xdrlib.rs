//! xdrlib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::struct;
// use crate::BytesIO;
// use crate::wraps;

pub const remove: f64 = ( 3 , 13 ) );
pub const __all__: &str = ["Error" ,"Packer" ,"Unpacker" ,"ConversionError" ];
pub struct Error {
    pub msg: String, // TODO: infer type
    pub __buf: String, // TODO: infer type
    pub __pos: String, // TODO: infer type
}

impl Error {
    pub fn new(msg: &str) -> Self {
        self . msg = msg;
        pub fn __repr__ ( self )  {
        return  repr ( self . msg );
        pub fn __str__ ( self )  {
        return  str ( self . msg );
    }

    pub fn raise_conversion_error(&self, function: &str) {
        " Wrap any raised struct.errors in a ConversionError. ";
        @ wraps ( function );
        pub fn result ( &self, value )  {
        // try {
        return  function ( self , value );
        // } catch  struct . error as e  {
        panic!("ConversionError ( e . args [ 0 ] ) from None /* Option */");
        return  result;
        class Packer ;
        "Pack various data representations into a buffer.";
        pub fn __init__ ( self )  {
        self . reset ( );
        pub fn reset ( self )  {
        self . __buf = BytesIO ( );
        pub fn get_buffer ( self )  {
        return  self . __buf . getvalue ( );
        get_buf = get_buffer;
        @ raise_conversion_error;
        pub fn pack_uint ( &self, x )  {
        self . __buf . write ( struct . pack ( ">L" , x ) );
        @ raise_conversion_error;
        pub fn pack_int ( &self, x )  {
        self . __buf . write ( struct . pack ( ">l" , x ) );
        pack_enum = pack_int;
        pub fn pack_bool ( &self, x )  {
        if x { : self . __buf . write ( b "\0\0\0\1" ); }
        } else {
        pub fn pack_uhyper ( &self, x )  {
        // try {
        self . pack_uint ( x > > 32 & 0x ffffffff );
        // } catch  ( TypeError , struct . error ) as e  {
        panic!("ConversionError ( e . args [ 0 ] ) from None /* Option */");
        // try {
        self . pack_uint ( x & 0x ffffffff );
        // } catch  ( TypeError , struct . error ) as e  {
        panic!("ConversionError ( e . args [ 0 ] ) from None /* Option */");
        pack_hyper = pack_uhyper;
        @ raise_conversion_error;
        pub fn pack_float ( &self, x )  {
        self . __buf . write ( struct . pack ( ">f" , x ) );
        @ raise_conversion_error;
        pub fn pack_double ( &self, x )  {
        self . __buf . write ( struct . pack ( ">d" , x ) );
        pub fn pack_fstring ( &self, n , s )  {
        if n < 0 {
        panic!("ValueError ( "fstring size must be nonnegative" )");
        data = s [ : n ];
        n = ( ( n + 3 ) / / 4 ) * 4;
        data = data + ( n - len ( data ) ) * b "\0";
        self . __buf . write ( data );
        pack_fopaque = pack_fstring;
        pub fn pack_string ( &self, s )  {
        n = len ( s );
        self . pack_uint ( n );
        self . pack_fstring ( n , s );
        pack_opaque = pack_string;
        pack_bytes = pack_string;
        pub fn pack_list ( &self, list , pack_item )  {
        for item in list .iter() {
        self . pack_uint ( 1 );
        pack_item ( item );
        self . pack_uint ( 0 );
        pub fn pack_farray ( &self, n , list , pack_item )  {
        if len ( list ) != n {
        panic!("ValueError ( "wrong array size" )");
        for item in list .iter() {
        pack_item ( item );
        pub fn pack_array ( &self, list , pack_item )  {
        n = len ( list );
        self . pack_uint ( n );
        self . pack_farray ( n , list , pack_item );
        class Unpacker ;
        "Unpacks various data representations from the given buffer.";
        pub fn __init__ ( &self, data )  {
        self . reset ( data );
        pub fn reset ( &self, data )  {
        self . __buf = data;
        self . __pos = 0;
        pub fn get_position ( self )  {
        return  self . __pos;
        pub fn set_position ( &self, position )  {
        self . __pos = position;
        pub fn get_buffer ( self )  {
        return  self . __buf;
        pub fn done ( self )  {
        if self . __pos < len ( self . __buf ) {
        panic!("Error ( "unextracted data remains" )");
        pub fn unpack_uint ( self )  {
        i = self . __pos;
        self . __pos = j = i + 4;
        data = self . __buf [ i : j ];
        if len ( data ) < 4 {
        panic!("EOFError");
        return  struct . unpack ( ">L" , data ) [ 0 ];
        pub fn unpack_int ( self )  {
        i = self . __pos;
        self . __pos = j = i + 4;
        data = self . __buf [ i : j ];
        if len ( data ) < 4 {
        panic!("EOFError");
        return  struct . unpack ( ">l" , data ) [ 0 ];
        unpack_enum = unpack_int;
        pub fn unpack_bool ( self )  {
        return  bool ( self . unpack_int ( ) );
        pub fn unpack_uhyper ( self )  {
        hi = self . unpack_uint ( );
        lo = self . unpack_uint ( );
        return  int ( hi ) < < 32 | lo;
        pub fn unpack_hyper ( self )  {
        x = self . unpack_uhyper ( );
        if x >= 0x8000000000000000 {
        x = x - 0x10000000000000000;
        return  x;
        pub fn unpack_float ( self )  {
        i = self . __pos;
        self . __pos = j = i + 4;
        data = self . __buf [ i : j ];
        if len ( data ) < 4 {
        panic!("EOFError");
        return  struct . unpack ( ">f" , data ) [ 0 ];
        pub fn unpack_double ( self )  {
        i = self . __pos;
        self . __pos = j = i + 8;
        data = self . __buf [ i : j ];
        if len ( data ) < 8 {
        panic!("EOFError");
        return  struct . unpack ( ">d" , data ) [ 0 ];
        pub fn unpack_fstring ( &self, n )  {
        if n < 0 {
        panic!("ValueError ( "fstring size must be nonnegative" )");
        i = self . __pos;
        j = i + ( n + 3 ) / / 4 * 4;
        if j > len ( self . __buf ) {
        panic!("EOFError");
        self . __pos = j;
        return  self . __buf [ i : i + n ];
        unpack_fopaque = unpack_fstring;
        pub fn unpack_string ( self )  {
        n = self . unpack_uint ( );
        return  self . unpack_fstring ( n );
        unpack_opaque = unpack_string;
        unpack_bytes = unpack_string;
        pub fn unpack_list ( &self, unpack_item )  {
        list = [ ];
        while 1  {
        x = self . unpack_uint ( );
        if x == 0 { : break; }
        if x != 1 {
        panic!("ConversionError ( "0 || 1 expected, got %r" % ( x , ) )");
        item = unpack_item ( );
        list . append ( item );
        return  list;
        pub fn unpack_farray ( &self, n , unpack_item )  {
        list = [ ];
        for i in range ( n ) .iter() {
        list . append ( unpack_item ( ) );
        return  list;
        pub fn unpack_array ( &self, unpack_item )  {
        n = self . unpack_uint ( );
        return  self . unpack_farray ( n , unpack_item );
    }

}

