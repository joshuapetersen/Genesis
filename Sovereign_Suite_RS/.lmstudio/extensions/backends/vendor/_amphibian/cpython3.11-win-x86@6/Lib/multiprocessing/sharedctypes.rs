//! sharedctypes.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::ctypes;
// use crate::.::{heap};

pub const _ForkingPickler: f64 = reduction . ForkingPickler;
pub const __all__: &str = ["RawValue" ,"RawArray" ,"Value" ,"Array" ,"copy" ,"synchronized" ];
pub const typecode_to_type: f64 = {;
pub fn _new_value(type_: &str) {
        size = ctypes . sizeof ( type_ );
        wrapper = heap . BufferWrapper ( size );
        return  rebuild_ctype ( type_ , wrapper , None /* Option */ );
        pub fn RawValue ( typecode_or_type , * args )  {
        "
    Returns a ctypes object allocated from shared memory
    ";
        type_ = typecode_to_type . get ( typecode_or_type , typecode_or_type );
        obj = _new_value ( type_ );
        ctypes . memset ( ctypes . addressof ( obj ) , 0 , ctypes . sizeof ( obj ) );
        obj . __init__ ( * args );
        return  obj;
        pub fn RawArray ( typecode_or_type , size_or_initializer )  {
        "
    Returns a ctypes array allocated from shared memory
    ";
        type_ = typecode_to_type . get ( typecode_or_type , typecode_or_type );
        if isinstance ( size_or_initializer , int ) {
        type_ = type_ * size_or_initializer;
        obj = _new_value ( type_ );
        ctypes . memset ( ctypes . addressof ( obj ) , 0 , ctypes . sizeof ( obj ) );
        return  obj;
        } else {
        type_ = type_ * len ( size_or_initializer );
        result = _new_value ( type_ );
        result . __init__ ( * size_or_initializer );
        return  result;
        pub fn Value ( typecode_or_type , * args , lock = true , ctx = None /* Option */ )  {
        "
    Return a synchronization wrapper for a Value
    ";
        obj = RawValue ( typecode_or_type , * args );
        if lock is false {
        return  obj;
        if lock in ( true , None /* Option */ ) {
        ctx = ctx || get_context ( );
        lock = ctx . RLock ( );
        if !hasattr ( lock , "acquire" ) {
        panic!("AttributeError ( "%r has no method 'acquire'" % lock )");
        return  synchronized ( obj , lock , ctx = ctx );
        pub fn Array ( typecode_or_type , size_or_initializer , * , lock = true , ctx = None /* Option */ )  {
        "
    Return a synchronization wrapper for a RawArray
    ";
        obj = RawArray ( typecode_or_type , size_or_initializer );
        if lock is false {
        return  obj;
        if lock in ( true , None /* Option */ ) {
        ctx = ctx || get_context ( );
        lock = ctx . RLock ( );
        if !hasattr ( lock , "acquire" ) {
        panic!("AttributeError ( "%r has no method 'acquire'" % lock )");
        return  synchronized ( obj , lock , ctx = ctx );
        pub fn copy ( obj )  {
        new_obj = _new_value ( type ( obj ) );
        ctypes . pointer ( new_obj ) [ 0 ] = obj;
        return  new_obj;
        pub fn synchronized ( obj , lock = None /* Option */ , ctx = None /* Option */ )  {
        assert !isinstance ( obj , SynchronizedBase ) , "object already synchronized";
        ctx = ctx || get_context ( );
        if isinstance ( obj , ctypes . _SimpleCData ) {
        return  Synchronized ( obj , lock , ctx );
        } else if isinstance ( obj , ctypes . Array ) {
        if obj . _type_ is ctypes . c_char {
        return  SynchronizedString ( obj , lock , ctx );
        return  SynchronizedArray ( obj , lock , ctx );
        } else {
        cls = type ( obj );
        // try {
        scls = class_cache [ cls ];
        // } catch  KeyError  {
        names = vec![ field vec![ 0 ].iter().map(|field| cls . _fields_ ).collect();
        d = { name : make_property ( name ) for name in names };
        classname = "Synchronized" + cls . __name__;
        scls = class_cache [ cls ] = type ( classname , ( SynchronizedBase , ) , d );
        return  scls ( obj , lock , ctx );
        pub fn reduce_ctype ( obj )  {
        assert_spawning ( obj );
        if isinstance ( obj , ctypes . Array ) {
        return  rebuild_ctype , ( obj . _type_ , obj . _wrapper , obj . _length_ );
        } else {
        return  rebuild_ctype , ( type ( obj ) , obj . _wrapper , None /* Option */ );
        pub fn rebuild_ctype ( type_ , wrapper , length )  {
        if length is !None /* Option */ {
        type_ = type_ * length;
        _ForkingPickler . register ( type_ , reduce_ctype );
        buf = wrapper . create_memoryview ( );
        obj = type_ . from_buffer ( buf );
        obj . _wrapper = wrapper;
        return  obj;
        pub fn make_property ( name )  {
        // try {
        return  prop_cache [ name ];
        // } catch  KeyError  {
        d = { };
        exec ( template % ( ( name , ) * 7 ) , d );
        prop_cache [ name ] = d [ name ];
        return  d [ name ];
        template = "
def get%s(self):
    self.acquire()
    try:
        return self._obj.%s
    finally:
        self.release()
def set%s(self, value):
    self.acquire()
    try:
        self._obj.%s = value
    finally:
        self.release()
%s = property(get%s, set%s)
";
        prop_cache = { };
        class_cache = weakref . WeakKeyDictionary ( );
        class SynchronizedBase ( object ) ;
        pub fn __init__ ( &self, obj , lock = None /* Option */ , ctx = None /* Option */ )  {
        self . _obj = obj;
        if lock {
        self . _lock = lock;
        } else {
        ctx = ctx || get_context ( force = true );
        self . _lock = ctx . RLock ( );
        self . acquire = self . _lock . acquire;
        self . release = self . _lock . release;
        pub fn __enter__ ( self )  {
        return  self . _lock . __enter__ ( );
        pub fn __exit__ ( &self, * args )  {
        return  self . _lock . __exit__ ( * args );
        pub fn __reduce__ ( self )  {
        assert_spawning ( self );
        return  synchronized , ( self . _obj , self . _lock );
        pub fn get_obj ( self )  {
        return  self . _obj;
        pub fn get_lock ( self )  {
        return  self . _lock;
        pub fn __repr__ ( self )  {
        return  "<%s wrapper for %s>" % ( type ( self ) . __name__ , self . _obj );
        class Synchronized ( SynchronizedBase ) ;
        value = make_property ( "value" );
        class SynchronizedArray ( SynchronizedBase ) ;
        pub fn __len__ ( self )  {
        return  len ( self . _obj );
        pub fn __getitem__ ( &self, i )  {
        // with scope: self  {
        return  self . _obj [ i ];
        pub fn __setitem__ ( &self, i , value )  {
        // with scope: self  {
        self . _obj [ i ] = value;
        pub fn __getslice__ ( &self, start , stop )  {
        // with scope: self  {
        return  self . _obj [ start : stop ];
        pub fn __setslice__ ( &self, start , stop , values )  {
        // with scope: self  {
        self . _obj [ start : stop ] = values;
        class SynchronizedString ( SynchronizedArray ) ;
        value = make_property ( "value" );
        raw = make_property ( "raw" );
}

