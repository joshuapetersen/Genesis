//! _endian.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;

pub const _array_type: f64 = type ( Array );
pub fn _other_endian(typ: &str) {
        "Return the type with the 'other' byte order.  Simple types like
    c_int && so on already have __ctype_be__ && __ctype_le__
    attributes which contain the types, for more complicated types
    arrays && structures are supported.
    ";
        if hasattr ( typ , _OTHER_ENDIAN ) {
        return  getattr ( typ , _OTHER_ENDIAN );
        if isinstance ( typ , _array_type ) {
        return  _other_endian ( typ . _type_ ) * typ . _length_;
        if issubclass ( typ , ( Structure , Union ) ) {
        return  typ;
        panic!("TypeError ( "This type does !support other endian: %s" % typ )");
        class _swapped_meta ;
        pub fn __setattr__ ( &self, attrname , value )  {
        if attrname == "_fields_" {
        fields = [ ];
        for desc in value .iter() {
        name = desc [ 0 ];
        typ = desc [ 1 ];
        rest = desc [ 2 : ];
        fields . append ( ( name , _other_endian ( typ ) ) + rest );
        value = fields;
        super ( ) . __setattr__ ( attrname , value );
        class _swapped_struct_meta ( _swapped_meta , type ( Structure ) ) : pass;
        class _swapped_union_meta ( _swapped_meta , type ( Union ) ) : pass;
        if sys . byteorder == "little" {
        _OTHER_ENDIAN = "__ctype_be__";
        LittleEndianStructure = Structure;
        class BigEndianStructure ( Structure , metaclass = _swapped_struct_meta ) ;
        "Structure with big endian byte order";
        __slots__ = ( );
        _swappedbytes_ = None /* Option */;
        LittleEndianUnion = Union;
        class BigEndianUnion ( Union , metaclass = _swapped_union_meta ) ;
        "Union with big endian byte order";
        __slots__ = ( );
        _swappedbytes_ = None /* Option */;
        } else if sys . byteorder == "big" {
        _OTHER_ENDIAN = "__ctype_le__";
        BigEndianStructure = Structure;
        class LittleEndianStructure ( Structure , metaclass = _swapped_struct_meta ) ;
        "Structure with little endian byte order";
        __slots__ = ( );
        _swappedbytes_ = None /* Option */;
        BigEndianUnion = Union;
        class LittleEndianUnion ( Union , metaclass = _swapped_union_meta ) ;
        "Union with little endian byte order";
        __slots__ = ( );
        _swappedbytes_ = None /* Option */;
        } else {
        panic!("RuntimeError ( "Invalid byteorder" )");
}

