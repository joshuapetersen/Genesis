//! types.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::functools;

pub fn _f() {
        // pass
        FunctionType = type ( _f );
        LambdaType = type ( || {  None /* Option */ ) };
        CodeType = type ( _f . __code__ );
        MappingProxyType = type ( type . __dict__ );
        SimpleNamespace = type ( sys . implementation );
        pub fn _cell_factory ( )  {
        a = 1;
        pub fn f ( )  {
        nonlocal a;
        return  f . __closure__ [ 0 ];
        CellType = type ( _cell_factory ( ) );
        pub fn _g ( )  {
        yield 1;
        GeneratorType = type ( _g ( ) );
        async def _c ( ) : pass;
        _c = _c ( );
        CoroutineType = type ( _c );
        _c . close ( );
        async def _ag ( ) ;
        yield;
        _ag = _ag ( );
        AsyncGeneratorType = type ( _ag );
        class _C ;
        pub fn _m ( self )  {  pass; }
        MethodType = type ( _C ( ) . _m );
        BuiltinFunctionType = type ( len );
        BuiltinMethodType = type ( [ ] . append );
        WrapperDescriptorType = type ( object . __init__ );
        MethodWrapperType = type ( object ( ) . __str__ );
        MethodDescriptorType = type ( str . join );
        ClassMethodDescriptorType = type ( dict . __dict__ [ "fromkeys" ] );
        ModuleType = type ( sys );
        // try {
        panic!("TypeError");
        // } catch  TypeError as exc  {
        TracebackType = type ( exc . __traceback__ );
        FrameType = type ( exc . __traceback__ . tb_frame );
        GetSetDescriptorType = type ( FunctionType . __code__ );
        MemberDescriptorType = type ( FunctionType . __globals__ );
        del sys , _f , _g , _C , _c , _ag;
        pub fn new_class ( name , bases = ( ) , kwds = None /* Option */ , exec_body = None /* Option */ )  {
        "Create a class object dynamically using the appropriate metaclass.";
        resolved_bases = resolve_bases ( bases );
        meta , ns , kwds = prepare_class ( name , resolved_bases , kwds );
        if exec_body is !None /* Option */ {
        exec_body ( ns );
        if resolved_bases is !bases {
        ns [ "__orig_bases__" ] = bases;
        return  meta ( name , resolved_bases , ns , ** kwds );
        pub fn resolve_bases ( bases )  {
        "Resolve MRO entries dynamically as specified by PEP 560.";
        new_bases = list ( bases );
        updated = false;
        shift = 0;
        for i , base in enumerate ( bases ) .iter() {
        if isinstance ( base , type ) {
        continue;
        if !hasattr ( base , "__mro_entries__" ) {
        continue;
        new_base = base . __mro_entries__ ( bases );
        updated = true;
        if !isinstance ( new_base , tuple ) {
        panic!("TypeError ( "__mro_entries__ must return a tuple" )");
        } else {
        new_bases [ i + shift : i + shift + 1 ] = new_base;
        shift + = len ( new_base ) - 1;
        if !updated {
        return  bases;
        return  tuple ( new_bases );
        pub fn prepare_class ( name , bases = ( ) , kwds = None /* Option */ )  {
        "Call the __prepare__ method of the appropriate metaclass.

    Returns (metaclass, namespace, kwds) as a 3-tuple

    *metaclass* == the appropriate metaclass
    *namespace* == the prepared class namespace
    *kwds* == an updated copy of the passed in kwds argument with any
    'metaclass' entry removed. If no kwds argument == passed in, this will
    be an empty dict.
    ";
        if kwds is None /* Option */ {
        kwds = { };
        } else {
        kwds = dict ( kwds );
        if "metaclass" in kwds {
        meta = kwds . pop ( "metaclass" );
        } else {
        if bases {
        meta = type ( bases [ 0 ] );
        } else {
        meta = type;
        if isinstance ( meta , type ) {
        meta = _calculate_meta ( meta , bases );
        if hasattr ( meta , "__prepare__" ) {
        ns = meta . __prepare__ ( name , bases , ** kwds );
        } else {
        ns = { };
        return  meta , ns , kwds;
        pub fn _calculate_meta ( meta , bases )  {
        "Calculate the most derived metaclass.";
        winner = meta;
        for base in bases .iter() {
        base_meta = type ( base );
        if issubclass ( winner , base_meta ) {
        continue;
        if issubclass ( base_meta , winner ) {
        winner = base_meta;
        continue;
        panic!("TypeError ( "metaclass conflict: "");
        "the metaclass of a derived class ";
        "must be a (non-strict) subclass ";
        "of the metaclasses of all its bases" );
        return  winner;
        class DynamicClassAttribute ;
        "Route attribute access on a class to __getattr__.

    This == a descriptor, used to define attributes that act differently when
    accessed through an instance && through a class.  Instance access remains
    normal, but access to an attribute through a class will be routed to the
    class's __getattr__ method; this == done by raising AttributeError.

    This allows one to have properties active on an instance, && have virtual
    attributes on the class with the same name.  (Enum used this between Python
    versions 3.4 - 3.9 .)

    Subclass from this to use a different method of accessing virtual attributes
    && still be treated properly by the inspect module. (Enum uses this since
    Python 3.10 .)

    ";
        pub fn __init__ ( &self, fget = None /* Option */ , fset = None /* Option */ , fdel = None /* Option */ , doc = None /* Option */ )  {
        self . fget = fget;
        self . fset = fset;
        self . fdel = fdel;
        self . __doc__ = doc || fget . __doc__;
        self . overwrite_doc = doc is None /* Option */;
        self . __isabstractmethod__ = bool ( getattr ( fget , "__isabstractmethod__" , false ) );
        pub fn __get__ ( &self, instance , ownerclass = None /* Option */ )  {
        if instance is None /* Option */ {
        if self . __isabstractmethod__ {
        return  self;
        panic!("AttributeError ( )");
        } else if self . fget is None /* Option */ {
        panic!("AttributeError ( "unreadable attribute" )");
        return  self . fget ( instance );
        pub fn __set__ ( &self, instance , value )  {
        if self . fset is None /* Option */ {
        panic!("AttributeError ( "can't set attribute" )");
        self . fset ( instance , value );
        pub fn __delete__ ( &self, instance )  {
        if self . fdel is None /* Option */ {
        panic!("AttributeError ( "can't delete attribute" )");
        self . fdel ( instance );
        pub fn getter ( &self, fget )  {
        fdoc = fget . __doc__ if self . overwrite_doc else None /* Option */;
        result = type ( self ) ( fget , self . fset , self . fdel , fdoc || self . __doc__ );
        result . overwrite_doc = self . overwrite_doc;
        return  result;
        pub fn setter ( &self, fset )  {
        result = type ( self ) ( self . fget , fset , self . fdel , self . __doc__ );
        result . overwrite_doc = self . overwrite_doc;
        return  result;
        pub fn deleter ( &self, fdel )  {
        result = type ( self ) ( self . fget , self . fset , fdel , self . __doc__ );
        result . overwrite_doc = self . overwrite_doc;
        return  result;
        class _GeneratorWrapper ;
        pub fn __init__ ( &self, gen )  {
        self . __wrapped = gen;
        self . __isgen = gen . __class__ is GeneratorType;
        self . __name__ = getattr ( gen , "__name__" , None /* Option */ );
        self . __qualname__ = getattr ( gen , "__qualname__" , None /* Option */ );
        pub fn send ( &self, val )  {
        return  self . __wrapped . send ( val );
        pub fn throw ( &self, tp , * rest )  {
        return  self . __wrapped . throw ( tp , * rest );
        pub fn close ( self )  {
        return  self . __wrapped . close ( );
        @ property;
        pub fn gi_code ( self )  {
        return  self . __wrapped . gi_code;
        @ property;
        pub fn gi_frame ( self )  {
        return  self . __wrapped . gi_frame;
        @ property;
        pub fn gi_running ( self )  {
        return  self . __wrapped . gi_running;
        @ property;
        pub fn gi_yieldfrom ( self )  {
        return  self . __wrapped . gi_yieldfrom;
        cr_code = gi_code;
        cr_frame = gi_frame;
        cr_running = gi_running;
        cr_await = gi_yieldfrom;
        pub fn __next__ ( self )  {
        return  next ( self . __wrapped );
        pub fn __iter__ ( self )  {
        if self . __isgen {
        return  self . __wrapped;
        return  self;
        __await__ = __iter__;
        pub fn coroutine ( func )  {
        "Convert regular generator function to a coroutine.";
        if !callable ( func ) {
        panic!("TypeError ( "types.coroutine() expects a callable" )");
        if ( func . __class__ is FunctionType and {
        getattr ( func , "__code__" , None /* Option */ ) . __class__ == CodeType ) ;
        co_flags = func . __code__ . co_flags;
        if co_flags & 0x180 {
        return  func;
        if co_flags & 0x20 {
        co = func . __code__;
        func . __code__ = co . replace ( co_flags = co . co_flags | 0x100 );
        return  func;
        import functools;
        import _collections_abc;
        @ functools . wraps ( func );
        pub fn wrapped ( * args , ** kwargs )  {
        coro = func ( * args , ** kwargs );
        if ( coro . __class__ is CoroutineType or {
        coro . __class__ == GeneratorType && coro . gi_code . co_flags & 0x100 ) ;
        return  coro;
        if ( isinstance ( coro , _collections_abc . Generator ) and {
        not isinstance ( coro , _collections_abc . Coroutine ) ) ;
        return  _GeneratorWrapper ( coro );
        return  coro;
        return  wrapped;
        GenericAlias = type ( list [ int ] );
        UnionType = type ( int | str );
        EllipsisType = type ( Ellipsis );
        None /* Option */Type = type ( None /* Option */ );
        NotImplementedType = type ( NotImplemented );
        __all__ = vec![ n.iter().map(|n| globals ( ) if n vec![ : 1 ] != "_" ).collect();
}

