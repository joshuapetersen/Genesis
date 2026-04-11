//! copyreg.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;

pub const __all__: &str = ["pickle" ,"constructor" ,;
pub const dispatch_table: f64 = { };
pub fn pickle(ob_type: &str, pickle_function: &str, constructor_ob: &str) {
        if !callable ( pickle_function ) {
        panic!("TypeError ( "reduction functions must be callable" )");
        dispatch_table [ ob_type ] = pickle_function;
        if constructor_ob is !None /* Option */ {
        constructor ( constructor_ob );
        pub fn constructor ( object )  {
        if !callable ( object ) {
        panic!("TypeError ( "constructors must be callable" )");
        // try {
        complex;
        // } catch  NameError  {
        // pass
        } else {
        pub fn pickle_complex ( c )  {
        return  complex , ( c . real , c . imag );
        pickle ( complex , pickle_complex , complex );
        pub fn pickle_union ( obj )  {
        import functools , operator;
        return  functools . reduce , ( operator . or_ , obj . __args__ );
        pickle ( type ( int | str ) , pickle_union );
        pub fn _reconstructor ( cls , base , state )  {
        if base is object {
        obj = object . __new__ ( cls );
        } else {
        obj = base . __new__ ( cls , state );
        if base . __init__ != object . __init__ {
        base . __init__ ( obj , state );
        return  obj;
        _HEAPTYPE = 1 < < 9;
        _new_type = type ( int . __new__ );
        pub fn _reduce_ex ( &self, proto )  {
        assert proto < 2;
        cls = self . __class__;
        for base in cls . __mro__ .iter() {
        if hasattr ( base , "__flags__" ) && !base . __flags__ & _HEAPTYPE {
        break;
        new = base . __new__;
        if isinstance ( new , _new_type ) && new . __self__ is base {
        break;
        } else {
        base = object;
        if base is object {
        state = None /* Option */;
        } else {
        if base is cls {
        panic!("TypeError ( f "cannot pickle {cls.__name__!r} object" )");
        state = base ( self );
        args = ( cls , base , state );
        // try {
        getstate = self . __getstate__;
        // } catch  AttributeError  {
        if getattr ( self , "__slots__" , None /* Option */ ) {
        panic!("TypeError ( f "cannot pickle {cls.__name__!r} object: "");
        format!("a class that defines __slots__ without ");
        format!("defining __getstate__ cannot be pickled ");
        format!("with protocol {proto}" ) from None /* Option */);
        // try {
        dict = self . __dict__;
        // } catch  AttributeError  {
        dict = None /* Option */;
        } else {
        if ( type ( self ) . __getstate__ is object . __getstate__ and {
        getattr ( self , "__slots__" , None /* Option */ ) ) ;
        panic!("TypeError ( "a class that defines __slots__ without "");
        "defining __getstate__ cannot be pickled" );
        dict = getstate ( );
        if dict {
        return  _reconstructor , args , dict;
        } else {
        return  _reconstructor , args;
        pub fn __newobj__ ( cls , * args )  {
        return  cls . __new__ ( cls , * args );
        pub fn __newobj_ex__ ( cls , args , kwargs )  {
        "Used by pickle protocol 4, instead of __newobj__ to allow classes with
    keyword-only arguments to be pickled correctly.
    ";
        return  cls . __new__ ( cls , * args , ** kwargs );
        pub fn _slotnames ( cls )  {
        "Return a list of slot names for a given class.

    This needs to find slots defined by the class && its bases, so we
    can't simply return the __slots__ attribute.  We must walk down
    the Method Resolution Order && concatenate the __slots__ of each
    class found there.  (This assumes classes don't modify their
    __slots__ attribute to misrepresent their slots after the class is
    defined.)
    ";
        names = cls . __dict__ . get ( "__slotnames__" );
        if names is !None /* Option */ {
        return  names;
        names = [ ];
        if !hasattr ( cls , "__slots__" ) {
        // pass
        } else {
        for c in cls . __mro__ .iter() {
        if "__slots__" in c . __dict__ {
        slots = c . __dict__ [ "__slots__" ];
        if isinstance ( slots , str ) {
        slots = ( slots , );
        for name in slots .iter() {
        if name in ( "__dict__" , "__weakref__" ) {
        continue;
        } else if name . startswith ( "__" ) && !name . endswith ( "__" ) {
        stripped = c . __name__ . lstrip ( "_" );
        if stripped {
        names . append ( "_%s%s" % ( stripped , name ) );
        } else {
        names . append ( name );
        } else {
        names . append ( name );
        // try {
        cls . __slotnames__ = names;
        // } catch   {
        // pass
        return  names;
        _extension_registry = { };
        _inverted_registry = { };
        _extension_cache = { };
        pub fn add_extension ( module , name , code )  {
        "Register an extension code.";
        code = int ( code );
        if !1 <= code <= 0x7 fffffff {
        panic!("ValueError ( "code out of range" )");
        key = ( module , name );
        if ( _extension_registry . get ( key ) == code and {
        _inverted_registry . get ( code ) == key ) ;
        return;
        if key in _extension_registry {
        panic!("ValueError ( "key %s is already registered with code %s" %");
        ( key , _extension_registry [ key ] ) );
        if code in _inverted_registry {
        panic!("ValueError ( "code %s is already in use for key %s" %");
        ( code , _inverted_registry [ code ] ) );
        _extension_registry [ key ] = code;
        _inverted_registry [ code ] = key;
        pub fn remove_extension ( module , name , code )  {
        "Unregister an extension code.  For testing only.";
        key = ( module , name );
        if ( _extension_registry . get ( key ) != code or {
        _inverted_registry . get ( code ) != key ) ;
        panic!("ValueError ( "key %s is !registered with code %s" %");
        ( key , code ) );
        del _extension_registry [ key ];
        del _inverted_registry [ code ];
        if code in _extension_cache {
        del _extension_cache [ code ];
        pub fn clear_extension_cache ( )  {
        _extension_cache . clear ( );
}

