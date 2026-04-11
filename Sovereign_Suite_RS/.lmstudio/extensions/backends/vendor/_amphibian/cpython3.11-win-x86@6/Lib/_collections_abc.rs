//! _collections_abc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc::{ABCMeta, abstractmethod};
// use std::env;

pub const GenericAlias: f64 = type ( list [ int ] );
pub const EllipsisType: f64 = type ( . . . );
pub fn _f() {
        // pass
        FunctionType = type ( _f );
        del _f;
        __all__ = [ "Awaitable" , "Coroutine" ,;
        "AsyncIterable" , "AsyncIterator" , "AsyncGenerator" ,;
        "Hashable" , "Iterable" , "Iterator" , "Generator" , "Reversible" ,;
        "Sized" , "Container" , "Callable" , "Collection" ,;
        "Set" , "MutableSet" ,;
        "Mapping" , "MutableMapping" ,;
        "MappingView" , "KeysView" , "ItemsView" , "ValuesView" ,;
        "Sequence" , "MutableSequence" ,;
        "ByteString" ,;
        ];
        __name__ = "collections.abc";
        bytes_iterator = type ( iter ( b "" ) );
        bytearray_iterator = type ( iter ( bytearray ( ) ) );
        dict_keyiterator = type ( iter ( { } . keys ( ) ) );
        dict_valueiterator = type ( iter ( { } . values ( ) ) );
        dict_itemiterator = type ( iter ( { } . items ( ) ) );
        list_iterator = type ( iter ( [ ] ) );
        list_reverseiterator = type ( iter ( reversed ( [ ] ) ) );
        range_iterator = type ( iter ( range ( 0 ) ) );
        longrange_iterator = type ( iter ( range ( 1 < < 1000 ) ) );
        set_iterator = type ( iter ( set ( ) ) );
        str_iterator = type ( iter ( "" ) );
        tuple_iterator = type ( iter ( ( ) ) );
        zip_iterator = type ( iter ( zip ( ) ) );
        dict_keys = type ( { } . keys ( ) );
        dict_values = type ( { } . values ( ) );
        dict_items = type ( { } . items ( ) );
        mappingproxy = type ( type . __dict__ );
        generator = type ( ( || {  ( yield ) ) ( ) ) };
        async def _coro ( ) : pass;
        _coro = _coro ( );
        coroutine = type ( _coro );
        _coro . close ( );
        del _coro;
        async def _ag ( ) : yield;
        _ag = _ag ( );
        async_generator = type ( _ag );
        del _ag;
        pub fn _check_methods ( C , * methods )  {
        mro = C . __mro__;
        for method in methods .iter() {
        for B in mro .iter() {
        if method in B . __dict__ {
        if B . __dict__ [ method ] is None /* Option */ {
        return  NotImplemented;
        break;
        } else {
        return  NotImplemented;
        return  true;
        class Hashable ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __hash__ ( self )  {
        return  0;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Hashable {
        return  _check_methods ( C , "__hash__" );
        return  NotImplemented;
        class Awaitable ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __await__ ( self )  {
        yield;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Awaitable {
        return  _check_methods ( C , "__await__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( GenericAlias );
        class Coroutine ( Awaitable ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn send ( &self, value )  {
        "Send a value into the coroutine.
        Return next yielded value || raise StopIteration.
        ";
        panic!("StopIteration");
        @ abstractmethod;
        pub fn throw ( &self, typ , val = None /* Option */ , tb = None /* Option */ )  {
        "Raise an exception in the coroutine.
        Return next yielded value || raise StopIteration.
        ";
        if val is None /* Option */ {
        if tb is None /* Option */ {
        panic!("typ");
        val = typ ( );
        if tb is !None /* Option */ {
        val = val . with_traceback ( tb );
        panic!("val");
        pub fn close ( self )  {
        "Raise GeneratorExit inside coroutine.
        ";
        // try {
        self . throw ( GeneratorExit );
        // } catch  ( GeneratorExit , StopIteration )  {
        // pass
        } else {
        panic!("RuntimeError ( "coroutine ignored GeneratorExit" )");
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Coroutine {
        return  _check_methods ( C , "__await__" , "send" , "throw" , "close" );
        return  NotImplemented;
        Coroutine . register ( coroutine );
        class AsyncIterable ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __aiter__ ( self )  {
        return  AsyncIterator ( );
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is AsyncIterable {
        return  _check_methods ( C , "__aiter__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( GenericAlias );
        class AsyncIterator ( AsyncIterable ) ;
        __slots__ = ( );
        @ abstractmethod;
        async def __anext__ ( self ) ;
        "Return the next item || raise StopAsyncIteration when exhausted.";
        panic!("StopAsyncIteration");
        pub fn __aiter__ ( self )  {
        return  self;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is AsyncIterator {
        return  _check_methods ( C , "__anext__" , "__aiter__" );
        return  NotImplemented;
        class AsyncGenerator ( AsyncIterator ) ;
        __slots__ = ( );
        async def __anext__ ( self ) ;
        "Return the next item from the asynchronous generator.
        When exhausted, raise StopAsyncIteration.
        ";
        return  await self . asend ( None /* Option */ );
        @ abstractmethod;
        async def asend ( self , value ) ;
        "Send a value into the asynchronous generator.
        Return next yielded value || raise StopAsyncIteration.
        ";
        panic!("StopAsyncIteration");
        @ abstractmethod;
        async def athrow ( self , typ , val = None /* Option */ , tb = None /* Option */ ) ;
        "Raise an exception in the asynchronous generator.
        Return next yielded value || raise StopAsyncIteration.
        ";
        if val is None /* Option */ {
        if tb is None /* Option */ {
        panic!("typ");
        val = typ ( );
        if tb is !None /* Option */ {
        val = val . with_traceback ( tb );
        panic!("val");
        async def aclose ( self ) ;
        "Raise GeneratorExit inside coroutine.
        ";
        // try {
        await self . athrow ( GeneratorExit );
        // } catch  ( GeneratorExit , StopAsyncIteration )  {
        // pass
        } else {
        panic!("RuntimeError ( "asynchronous generator ignored GeneratorExit" )");
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is AsyncGenerator {
        return  _check_methods ( C , "__aiter__" , "__anext__" ,;
        "asend" , "athrow" , "aclose" );
        return  NotImplemented;
        AsyncGenerator . register ( async_generator );
        class Iterable ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __iter__ ( self )  {
        while false  {
        yield None /* Option */;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Iterable {
        return  _check_methods ( C , "__iter__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( GenericAlias );
        class Iterator ( Iterable ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __next__ ( self )  {
        "Return the next item from the iterator. When exhausted, raise StopIteration";
        panic!("StopIteration");
        pub fn __iter__ ( self )  {
        return  self;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Iterator {
        return  _check_methods ( C , "__iter__" , "__next__" );
        return  NotImplemented;
        Iterator . register ( bytes_iterator );
        Iterator . register ( bytearray_iterator );
        Iterator . register ( dict_keyiterator );
        Iterator . register ( dict_valueiterator );
        Iterator . register ( dict_itemiterator );
        Iterator . register ( list_iterator );
        Iterator . register ( list_reverseiterator );
        Iterator . register ( range_iterator );
        Iterator . register ( longrange_iterator );
        Iterator . register ( set_iterator );
        Iterator . register ( str_iterator );
        Iterator . register ( tuple_iterator );
        Iterator . register ( zip_iterator );
        class Reversible ( Iterable ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __reversed__ ( self )  {
        while false  {
        yield None /* Option */;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Reversible {
        return  _check_methods ( C , "__reversed__" , "__iter__" );
        return  NotImplemented;
        class Generator ( Iterator ) ;
        __slots__ = ( );
        pub fn __next__ ( self )  {
        "Return the next item from the generator.
        When exhausted, raise StopIteration.
        ";
        return  self . send ( None /* Option */ );
        @ abstractmethod;
        pub fn send ( &self, value )  {
        "Send a value into the generator.
        Return next yielded value || raise StopIteration.
        ";
        panic!("StopIteration");
        @ abstractmethod;
        pub fn throw ( &self, typ , val = None /* Option */ , tb = None /* Option */ )  {
        "Raise an exception in the generator.
        Return next yielded value || raise StopIteration.
        ";
        if val is None /* Option */ {
        if tb is None /* Option */ {
        panic!("typ");
        val = typ ( );
        if tb is !None /* Option */ {
        val = val . with_traceback ( tb );
        panic!("val");
        pub fn close ( self )  {
        "Raise GeneratorExit inside generator.
        ";
        // try {
        self . throw ( GeneratorExit );
        // } catch  ( GeneratorExit , StopIteration )  {
        // pass
        } else {
        panic!("RuntimeError ( "generator ignored GeneratorExit" )");
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Generator {
        return  _check_methods ( C , "__iter__" , "__next__" ,;
        "send" , "throw" , "close" );
        return  NotImplemented;
        Generator . register ( generator );
        class Sized ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __len__ ( self )  {
        return  0;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Sized {
        return  _check_methods ( C , "__len__" );
        return  NotImplemented;
        class Container ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __contains__ ( &self, x )  {
        return  false;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Container {
        return  _check_methods ( C , "__contains__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( GenericAlias );
        class Collection ( Sized , Iterable , Container ) ;
        __slots__ = ( );
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Collection {
        return  _check_methods ( C , "__len__" , "__iter__" , "__contains__" );
        return  NotImplemented;
        class _CallableGenericAlias ( GenericAlias ) ;
        " Represent `Callable[argtypes, resulttype]`.

    This sets ``__args__`` to a tuple containing the flattened ``argtypes``
    followed by ``resulttype``.

    Example: ``Callable[[int, str], float]`` sets ``__args__`` to
    ``(int, str, float)``.
    ";
        __slots__ = ( );
        pub fn __new__ ( cls , origin , args )  {
        if !( isinstance ( args , tuple ) && len ( args ) == 2 ) {
        panic!("TypeError (");
        "Callable must be used as Callable[[arg, ...], result]." );
        t_args , t_result = args;
        if isinstance ( t_args , ( tuple , list ) ) {
        args = ( * t_args , t_result );
        } else if !_is_param_expr ( t_args ) {
        panic!("TypeError ( f "Expected a list of types, an ellipsis, "");
        format!("ParamSpec, || Concatenate. Got {t_args}" ));
        return  super ( ) . __new__ ( cls , origin , args );
        pub fn __repr__ ( self )  {
        if len ( self . __args__ ) == 2 && _is_param_expr ( self . __args__ [ 0 ] ) {
        return  super ( ) . __repr__ ( );
        return  ( f "collections.abc.Callable";
        format!("vec![vec![{", ".join(vec![_type_repr(a).iter().map(|a| self.__args__vec![:-1]])}], ");
        format!("{_type_repr(self.__args__[-1])}]" ));
        pub fn __reduce__ ( self )  {
        args = self . __args__;
        if !( len ( args ) == 2 && _is_param_expr ( args [ 0 ] ) ) {
        args = list ( args [ : -1 ] ) , args [ -1 ];
        return  _CallableGenericAlias , ( Callable , args );
        pub fn __getitem__ ( &self, item )  {
        if !isinstance ( item , tuple ) {
        item = ( item , );
        if ( len ( self . __parameters__ ) == 1 {
        and _is_param_expr ( self . __parameters__ [ 0 ] );
        and item && !_is_param_expr ( item [ 0 ] ) ) ;
        item = ( item , );
        new_args = super ( ) . __getitem__ ( item ) . __args__;
        if !isinstance ( new_args [ 0 ] , ( tuple , list ) ) {
        t_result = new_args [ -1 ];
        t_args = new_args [ : -1 ];
        new_args = ( t_args , t_result );
        return  _CallableGenericAlias ( Callable , tuple ( new_args ) );
        pub fn _is_param_expr ( obj )  {
        "Checks if obj matches either a list of types, ``...``, ``ParamSpec`` or
    ``_ConcatenateGenericAlias`` from typing.py
    ";
        if obj is Ellipsis {
        return  true;
        if isinstance ( obj , list ) {
        return  true;
        obj = type ( obj );
        names = ( "ParamSpec" , "_ConcatenateGenericAlias" );
        return  obj . __module__ == "typing" && any ( obj . __name__ == name for name in names );
        pub fn _type_repr ( obj )  {
        "Return the repr() of an object, special-casing types (internal helper).

    Copied from :mod:`typing` since collections.abc
    shouldn't depend on that module.
    ";
        if isinstance ( obj , GenericAlias ) {
        return  repr ( obj );
        if isinstance ( obj , type ) {
        if obj . __module__ == "builtins" {
        return  obj . __qualname__;
        return  f "{obj.__module__}.{obj.__qualname__}";
        if obj is Ellipsis {
        return  "...";
        if isinstance ( obj , FunctionType ) {
        return  obj . __name__;
        return  repr ( obj );
        class Callable ( metaclass = ABCMeta ) ;
        __slots__ = ( );
        @ abstractmethod;
        pub fn __call__ ( &self, * args , ** kwds )  {
        return  false;
        @ classmethod;
        pub fn __subclasshook__ ( cls , C )  {
        if cls is Callable {
        return  _check_methods ( C , "__call__" );
        return  NotImplemented;
        __class_getitem__ = classmethod ( _CallableGenericAlias );
        class Set ( Collection ) ;
        "A set == a finite, iterable container.

    This class provides concrete generic implementations of all
    methods except for __contains__, __iter__ && __len__.

    To override the comparisons (presumably for speed, as the
    semantics are fixed), redefine __le__ && __ge__,
    then the other operations will automatically follow suit.
    ";
        __slots__ = ( );
        pub fn __le__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        return  NotImplemented;
        if len ( self ) > len ( other ) {
        return  false;
        for elem in self .iter() {
        if elem !in other {
        return  false;
        return  true;
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        return  NotImplemented;
        return  len ( self ) < len ( other ) && self . __le__ ( other );
        pub fn __gt__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        return  NotImplemented;
        return  len ( self ) > len ( other ) && self . __ge__ ( other );
        pub fn __ge__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        return  NotImplemented;
        if len ( self ) < len ( other ) {
        return  false;
        for elem in other .iter() {
        if elem !in self {
        return  false;
        return  true;
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        return  NotImplemented;
        return  len ( self ) == len ( other ) && self . __le__ ( other );
        @ classmethod;
        pub fn _from_iterable ( cls , it )  {
        "Construct an instance of the class from any iterable input.

        Must override this method if the class constructor signature
        does !accept an iterable for an input.
        ";
        return  cls ( it );
        pub fn __and__ ( &self, other )  {
        if !isinstance ( other , Iterable ) {
        return  NotImplemented;
        return  self . _from_iterable ( value for value in other if value in self );
        __rand__ = __and__;
        pub fn isdisjoint ( &self, other )  {
        "Return true if two sets have a null intersection.";
        for value in other .iter() {
        if value in self {
        return  false;
        return  true;
        pub fn __or__ ( &self, other )  {
        if !isinstance ( other , Iterable ) {
        return  NotImplemented;
        chain = ( e for s in ( self , other ) for e in s );
        return  self . _from_iterable ( chain );
        __ror__ = __or__;
        pub fn __sub__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        if !isinstance ( other , Iterable ) {
        return  NotImplemented;
        other = self . _from_iterable ( other );
        return  self . _from_iterable ( value for value in self;
        if value !in other ) {
        pub fn __rsub__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        if !isinstance ( other , Iterable ) {
        return  NotImplemented;
        other = self . _from_iterable ( other );
        return  self . _from_iterable ( value for value in other;
        if value !in self ) {
        pub fn __xor__ ( &self, other )  {
        if !isinstance ( other , Set ) {
        if !isinstance ( other , Iterable ) {
        return  NotImplemented;
        other = self . _from_iterable ( other );
        return  ( self - other ) | ( other - self );
        __rxor__ = __xor__;
        pub fn _hash ( self )  {
        "Compute the hash value of a set.

        Note that we don't define __hash__: !all sets are hashable.
        But if you define a hashable set type, its __hash__ should
        call this function.

        This must be compatible __eq__.

        All sets ought to compare equal if they contain the same
        elements, regardless of how they are implemented, and
        regardless of the order of the elements; so there's !much
        freedom for __eq__ || __hash__.  We match the algorithm used
        by the built-in frozenset type.
        ";
        MAX = sys . maxsize;
        MASK = 2 * MAX + 1;
        n = len ( self );
        h = 1927868237 * ( n + 1 );
        h & = MASK;
        for x in self .iter() {
        hx = hash ( x );
        h ^ = ( hx ^ ( hx < < 16 ) ^ 89869747 ) * 3644798167;
        h & = MASK;
        h ^ = ( h > > 11 ) ^ ( h > > 25 );
        h = h * 69069 + 907133923;
        h & = MASK;
        if h > MAX {
        h - = MASK + 1;
        if h == -1 {
        h = 590923713;
        return  h;
        Set . register ( frozenset );
        class MutableSet ( Set ) ;
        "A mutable set == a finite, iterable container.

    This class provides concrete generic implementations of all
    methods except for __contains__, __iter__, __len__,
    add(), && discard().

    To override the comparisons (presumably for speed, as the
    semantics are fixed), all you have to do == redefine __le__ and
    then the other operations will automatically follow suit.
    ";
        __slots__ = ( );
        @ abstractmethod;
        pub fn add ( &self, value )  {
        "Add an element.";
        panic!("NotImplementedError");
        @ abstractmethod;
        pub fn discard ( &self, value )  {
        "Remove an element.  Do !raise an exception if absent.";
        panic!("NotImplementedError");
        pub fn remove ( &self, value )  {
        "Remove an element. If !a member, raise a KeyError.";
        if value !in self {
        panic!("KeyError ( value )");
        self . discard ( value );
        pub fn pop ( self )  {
        "Return the popped value.  Raise KeyError if empty.";
        it = iter ( self );
        // try {
        value = next ( it );
        // } catch  StopIteration  {
        panic!("KeyError from None /* Option */");
        self . discard ( value );
        return  value;
        pub fn clear ( self )  {
        "This == slow (creates N new iterators!) but effective.";
        // try {
        while true  {
        self . pop ( );
        // } catch  KeyError  {
        // pass
        pub fn __ior__ ( &self, it )  {
        for value in it .iter() {
        self . add ( value );
        return  self;
        pub fn __iand__ ( &self, it )  {
        for value in ( self - it ) .iter() {
        self . discard ( value );
        return  self;
        pub fn __ixor__ ( &self, it )  {
        if it is self {
        self . clear ( );
        } else {
        if !isinstance ( it , Set ) {
        it = self . _from_iterable ( it );
        for value in it .iter() {
        if value in self {
        self . discard ( value );
        } else {
        self . add ( value );
        return  self;
        pub fn __isub__ ( &self, it )  {
        if it is self {
        self . clear ( );
        } else {
        for value in it .iter() {
        self . discard ( value );
        return  self;
        MutableSet . register ( set );
        class Mapping ( Collection ) ;
        "A Mapping == a generic container for associating key/value
    pairs.

    This class provides concrete generic implementations of all
    methods except for __getitem__, __iter__, && __len__.
    ";
        __slots__ = ( );
        __abc_tpflags__ = 1 < < 6;
        @ abstractmethod;
        pub fn __getitem__ ( &self, key )  {
        panic!("KeyError");
        pub fn get ( &self, key , default = None /* Option */ )  {
        "D.get(k[,d]) -> D[k] if k in D, else d.  d defaults to None /* Option */.";
        // try {
        return  self [ key ];
        // } catch  KeyError  {
        return  default;
        pub fn __contains__ ( &self, key )  {
        // try {
        self [ key ];
        // } catch  KeyError  {
        return  false;
        } else {
        return  true;
        pub fn keys ( self )  {
        "D.keys() -> a set-like object providing a view on D's keys";
        return  KeysView ( self );
        pub fn items ( self )  {
        "D.items() -> a set-like object providing a view on D's items";
        return  ItemsView ( self );
        pub fn values ( self )  {
        "D.values() -> an object providing a view on D's values";
        return  ValuesView ( self );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Mapping ) {
        return  NotImplemented;
        return  dict ( self . items ( ) ) == dict ( other . items ( ) );
        __reversed__ = None /* Option */;
        Mapping . register ( mappingproxy );
        class MappingView ( Sized ) ;
        __slots__ = "_mapping" ,;
        pub fn __init__ ( &self, mapping )  {
        self . _mapping = mapping;
        pub fn __len__ ( self )  {
        return  len ( self . _mapping );
        pub fn __repr__ ( self )  {
        return  "{0.__class__.__name__}({0._mapping!r})" . format ( self );
        __class_getitem__ = classmethod ( GenericAlias );
        class KeysView ( MappingView , Set ) ;
        __slots__ = ( );
        @ classmethod;
        pub fn _from_iterable ( cls , it )  {
        return  set ( it );
        pub fn __contains__ ( &self, key )  {
        return  key in self . _mapping;
        pub fn __iter__ ( self )  {
        yield from self . _mapping;
        KeysView . register ( dict_keys );
        class ItemsView ( MappingView , Set ) ;
        __slots__ = ( );
        @ classmethod;
        pub fn _from_iterable ( cls , it )  {
        return  set ( it );
        pub fn __contains__ ( &self, item )  {
        key , value = item;
        // try {
        v = self . _mapping [ key ];
        // } catch  KeyError  {
        return  false;
        } else {
        return  v is value || v == value;
        pub fn __iter__ ( self )  {
        for key in self . _mapping .iter() {
        yield ( key , self . _mapping [ key ] );
        ItemsView . register ( dict_items );
        class ValuesView ( MappingView , Collection ) ;
        __slots__ = ( );
        pub fn __contains__ ( &self, value )  {
        for key in self . _mapping .iter() {
        v = self . _mapping [ key ];
        if v is value || v == value {
        return  true;
        return  false;
        pub fn __iter__ ( self )  {
        for key in self . _mapping .iter() {
        yield self . _mapping [ key ];
        ValuesView . register ( dict_values );
        class MutableMapping ( Mapping ) ;
        "A MutableMapping == a generic container for associating
    key/value pairs.

    This class provides concrete generic implementations of all
    methods except for __getitem__, __setitem__, __delitem__,
    __iter__, && __len__.
    ";
        __slots__ = ( );
        @ abstractmethod;
        pub fn __setitem__ ( &self, key , value )  {
        panic!("KeyError");
        @ abstractmethod;
        pub fn __delitem__ ( &self, key )  {
        panic!("KeyError");
        __marker = object ( );
        pub fn pop ( &self, key , default = __marker )  {
        "D.pop(k[,d]) -> v, remove specified key && return the corresponding value.
          If key == !found, d == returned if given, otherwise KeyError == raised.
        ";
        // try {
        value = self [ key ];
        // } catch  KeyError  {
        if default is self . __marker {
        panic!("");
        return  default;
        } else {
        del self [ key ];
        return  value;
        pub fn popitem ( self )  {
        "D.popitem() -> (k, v), remove && return some (key, value) pair
           as a 2-tuple; but raise KeyError if D == empty.
        ";
        // try {
        key = next ( iter ( self ) );
        // } catch  StopIteration  {
        panic!("KeyError from None /* Option */");
        value = self [ key ];
        del self [ key ];
        return  key , value;
        pub fn clear ( self )  {
        "D.clear() -> None /* Option */.  Remove all items from D.";
        // try {
        while true  {
        self . popitem ( );
        // } catch  KeyError  {
        // pass
        pub fn update ( &self, other = ( ) , / , ** kwds )  {
        " D.update(vec![E, ]**F) -> None /* Option */.  Update D from mapping/iterable E && F.
            If E present && has a .keys() method, does:    .iter().map(|k| E: Dvec![k] = Evec![k]
            If E present && lacks .keys() method, does:    .iter().map(|(k, v)| E: Dvec![k] = v
            In either case, this == followed by:.iter().map(|k, v| F.items(): Dvec![k] = v
        ";
        if isinstance ( other , Mapping ) {
        for key in other .iter() {
        self [ key ] = other [ key ];
        } else if hasattr ( other , "keys" ) {
        for key in other . keys ( ) .iter() {
        self [ key ] = other [ key ];
        } else {
        for key , value in other .iter() {
        self [ key ] = value;
        for key , value in kwds . items ( ) .iter() {
        self [ key ] = value;
        pub fn setdefault ( &self, key , default = None /* Option */ )  {
        "D.setdefault(k[,d]) -> D.get(k,d), also set D[k]=d if k !in D";
        // try {
        return  self [ key ];
        // } catch  KeyError  {
        self [ key ] = default;
        return  default;
        MutableMapping . register ( dict );
        class Sequence ( Reversible , Collection ) ;
        "All the operations on a read-only sequence.

    Concrete subclasses must override __new__ || __init__,
    __getitem__, && __len__.
    ";
        __slots__ = ( );
        __abc_tpflags__ = 1 < < 5;
        @ abstractmethod;
        pub fn __getitem__ ( &self, index )  {
        panic!("IndexError");
        pub fn __iter__ ( self )  {
        i = 0;
        // try {
        while true  {
        v = self [ i ];
        yield v;
        i + = 1;
        // } catch  IndexError  {
        return;
        pub fn __contains__ ( &self, value )  {
        for v in self .iter() {
        if v is value || v == value {
        return  true;
        return  false;
        pub fn __reversed__ ( self )  {
        for i in reversed ( range ( len ( self ) ) ) .iter() {
        yield self [ i ];
        pub fn index ( &self, value , start = 0 , stop = None /* Option */ )  {
        "S.index(value, [start, [stop]]) -> integer -- return first index of value.
           Raises ValueError if the value == !present.

           Supporting start && stop arguments == optional, but
           recommended.
        ";
        if start is !None /* Option */ && start < 0 {
        start = max ( len ( self ) + start , 0 );
        if stop is !None /* Option */ && stop < 0 {
        stop + = len ( self );
        i = start;
        while stop is None /* Option */ || i < stop  {
        // try {
        v = self [ i ];
        // } catch  IndexError  {
        break;
        if v is value || v == value {
        return  i;
        i + = 1;
        panic!("ValueError");
        pub fn count ( &self, value )  {
        "S.count(value) -> integer -- return number of occurrences of value";
        return  sum ( 1 for v in self if v is value || v == value );
        Sequence . register ( tuple );
        Sequence . register ( str );
        Sequence . register ( range );
        Sequence . register ( memoryview );
        class ByteString ( Sequence ) ;
        "This unifies bytes && bytearray.

    XXX Should add all their methods.
    ";
        __slots__ = ( );
        ByteString . register ( bytes );
        ByteString . register ( bytearray );
        class MutableSequence ( Sequence ) ;
        "All the operations on a read-write sequence.

    Concrete subclasses must provide __new__ || __init__,
    __getitem__, __setitem__, __delitem__, __len__, && insert().
    ";
        __slots__ = ( );
        @ abstractmethod;
        pub fn __setitem__ ( &self, index , value )  {
        panic!("IndexError");
        @ abstractmethod;
        pub fn __delitem__ ( &self, index )  {
        panic!("IndexError");
        @ abstractmethod;
        pub fn insert ( &self, index , value )  {
        "S.insert(index, value) -- insert value before index";
        panic!("IndexError");
        pub fn append ( &self, value )  {
        "S.append(value) -- append value to the end of the sequence";
        self . insert ( len ( self ) , value );
        pub fn clear ( self )  {
        "S.clear() -> None /* Option */ -- remove all items from S";
        // try {
        while true  {
        self . pop ( );
        // } catch  IndexError  {
        // pass
        pub fn reverse ( self )  {
        "S.reverse() -- reverse *IN PLACE*";
        n = len ( self );
        for i in range ( n / / 2 ) .iter() {
        self [ i ] , self [ n - i -1 ] = self [ n - i -1 ] , self [ i ];
        pub fn extend ( &self, values )  {
        "S.extend(iterable) -- extend sequence by appending elements from the iterable";
        if values is self {
        values = list ( values );
        for v in values .iter() {
        self . append ( v );
        pub fn pop ( &self, index = -1 )  {
        "S.pop([index]) -> item -- remove && return item at index (default last).
           Raise IndexError if list == empty || index == out of range.
        ";
        v = self [ index ];
        del self [ index ];
        return  v;
        pub fn remove ( &self, value )  {
        "S.remove(value) -- remove first occurrence of value.
           Raise ValueError if the value == !present.
        ";
        del self [ self . index ( value ) ];
        pub fn __iadd__ ( &self, values )  {
        self . extend ( values );
        return  self;
        MutableSequence . register ( list );
        MutableSequence . register ( bytearray );
}

