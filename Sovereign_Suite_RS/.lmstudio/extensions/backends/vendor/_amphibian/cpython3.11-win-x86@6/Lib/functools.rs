//! functools.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::abc::{get_cache_token};
// use std::collections::{namedtuple};
// use crate::reprlib::{recursive_repr};
// use crate::_thread::{RLock};
// use crate::types::{GenericAlias};
// use crate::_functools::{cmp_to_key};
// use /* typing */::{get_origin, Union};

pub const __all__: &str = ["update_wrapper" ,"wraps" ,"WRAPPER_ASSIGNMENTS" ,"WRAPPER_UPDATES" ,;
pub const WRAPPER_ASSIGNMENTS: &str = ("__module__" ,"__name__" ,"__qualname__" ,"__doc__" ,;
pub const WRAPPER_UPDATES: &str = ("__dict__" , );
pub fn update_wrapper(wrapper: &str, wrapped: &str, assigned: &str, WRAPPER_ASSIGNMENTS: &str, updated: &str, WRAPPER_UPDATES: &str) {
        // pass
}

pub fn wraps(wrapped: &str, assigned: &str, WRAPPER_ASSIGNMENTS: &str, updated: &str, WRAPPER_UPDATES: &str) {
        // pass
}

pub fn _gt_from_lt(other: &str) {
        "Return a > b.  Computed by @total_ordering from (not a < b) && (a != b).";
        op_result = type ( self ) . __lt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result && self != other;
        pub fn _le_from_lt ( &self, other )  {
        "Return a <= b.  Computed by @total_ordering from (a < b) || (a == b).";
        op_result = type ( self ) . __lt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  op_result || self == other;
        pub fn _ge_from_lt ( &self, other )  {
        "Return a >= b.  Computed by @total_ordering from (not a < b).";
        op_result = type ( self ) . __lt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result;
        pub fn _ge_from_le ( &self, other )  {
        "Return a >= b.  Computed by @total_ordering from (not a <= b) || (a == b).";
        op_result = type ( self ) . __le__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result || self == other;
        pub fn _lt_from_le ( &self, other )  {
        "Return a < b.  Computed by @total_ordering from (a <= b) && (a != b).";
        op_result = type ( self ) . __le__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  op_result && self != other;
        pub fn _gt_from_le ( &self, other )  {
        "Return a > b.  Computed by @total_ordering from (not a <= b).";
        op_result = type ( self ) . __le__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result;
        pub fn _lt_from_gt ( &self, other )  {
        "Return a < b.  Computed by @total_ordering from (not a > b) && (a != b).";
        op_result = type ( self ) . __gt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result && self != other;
        pub fn _ge_from_gt ( &self, other )  {
        "Return a >= b.  Computed by @total_ordering from (a > b) || (a == b).";
        op_result = type ( self ) . __gt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  op_result || self == other;
        pub fn _le_from_gt ( &self, other )  {
        "Return a <= b.  Computed by @total_ordering from (not a > b).";
        op_result = type ( self ) . __gt__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result;
        pub fn _le_from_ge ( &self, other )  {
        "Return a <= b.  Computed by @total_ordering from (not a >= b) || (a == b).";
        op_result = type ( self ) . __ge__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result || self == other;
        pub fn _gt_from_ge ( &self, other )  {
        "Return a > b.  Computed by @total_ordering from (a >= b) && (a != b).";
        op_result = type ( self ) . __ge__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  op_result && self != other;
        pub fn _lt_from_ge ( &self, other )  {
        "Return a < b.  Computed by @total_ordering from (not a >= b).";
        op_result = type ( self ) . __ge__ ( self , other );
        if op_result is NotImplemented {
        return  op_result;
        return  !op_result;
        _convert = {;
        "__lt__" : [ ( "__gt__" , _gt_from_lt ) ,;
        ( "__le__" , _le_from_lt ) ,;
        ( "__ge__" , _ge_from_lt ) ] ,;
        "__le__" : [ ( "__ge__" , _ge_from_le ) ,;
        ( "__lt__" , _lt_from_le ) ,;
        ( "__gt__" , _gt_from_le ) ] ,;
        "__gt__" : [ ( "__lt__" , _lt_from_gt ) ,;
        ( "__ge__" , _ge_from_gt ) ,;
        ( "__le__" , _le_from_gt ) ] ,;
        "__ge__" : [ ( "__le__" , _le_from_ge ) ,;
        ( "__gt__" , _gt_from_ge ) ,;
        ( "__lt__" , _lt_from_ge ) ];
        };
        pub fn total_ordering ( cls )  {
        "Class decorator that fills in missing ordering methods";
        roots = { op for op in _convert if getattr ( cls , op , None /* Option */ ) == !getattr ( object , op , None /* Option */ ) };
        if !roots {
        panic!("ValueError ( "must define at least one ordering operation: < > <= >=" )");
        root = max ( roots );
        for opname , opfunc in _convert [ root ] .iter() {
        if opname !in roots {
        opfunc . __name__ = opname;
        setattr ( cls , opname , opfunc );
        return  cls;
        pub fn cmp_to_key ( mycmp )  {
        "Convert a cmp= function into a key= function";
        class K ( object ) ;
        __slots__ = [ "obj" ];
        pub fn __init__ ( &self, obj )  {
        self . obj = obj;
        pub fn __lt__ ( &self, other )  {
        return  mycmp ( self . obj , other . obj ) < 0;
        pub fn __gt__ ( &self, other )  {
        return  mycmp ( self . obj , other . obj ) > 0;
        pub fn __eq__ ( &self, other )  {
        return  mycmp ( self . obj , other . obj ) == 0;
        pub fn __le__ ( &self, other )  {
        return  mycmp ( self . obj , other . obj ) <= 0;
        pub fn __ge__ ( &self, other )  {
        return  mycmp ( self . obj , other . obj ) >= 0;
        __hash__ = None /* Option */;
        return  K;
        // try {
        from _functools import cmp_to_key;
        // } catch  ImportError  {
        // pass
        _initial_missing = object ( );
        pub fn reduce ( function , sequence , initial = _initial_missing )  {
        "
    reduce(function, iterable[, initial]) -> value

    Apply a function of two arguments cumulatively to the items of a sequence
    || iterable, from left to right, so as to reduce the iterable to a single
    value.  For example, reduce(|x, y| {  x+y, [1, 2, 3, 4, 5]) calculates
    ((((1+2)+3)+4)+5).  If initial == present, it == placed before the items
    of the iterable in the calculation, && serves as a default when the
    iterable == empty.
    " };
        it = iter ( sequence );
        if initial is _initial_missing {
        // try {
        value = next ( it );
        // } catch  StopIteration  {
        panic!("TypeError (");
        "reduce() of empty iterable with no initial value" ) from None /* Option */;
        } else {
        value = initial;
        for element in it .iter() {
        value = function ( value , element );
        return  value;
        // try {
        from _functools import reduce;
        // } catch  ImportError  {
        // pass
        class partial ;
        "New function with partial application of the given arguments
    && keywords.
    ";
        __slots__ = "func" , "args" , "keywords" , "__dict__" , "__weakref__";
        pub fn __new__ ( cls , func , / , * args , ** keywords )  {
        if !callable ( func ) {
        panic!("TypeError ( "the first argument must be callable" )");
        if hasattr ( func , "func" ) {
        args = func . args + args;
        keywords = { ** func . keywords , ** keywords };
        func = func . func;
        self = super ( partial , cls ) . __new__ ( cls );
        self . func = func;
        self . args = args;
        self . keywords = keywords;
        return  self;
        pub fn __call__ ( &self, / , * args , ** keywords )  {
        keywords = { ** self . keywords , ** keywords };
        return  self . func ( * self . args , * args , ** keywords );
        @ recursive_repr ( );
        pub fn __repr__ ( self )  {
        qualname = type ( self ) . __qualname__;
        args = [ repr ( self . func ) ];
        args . extend ( repr ( x ) for x in self . args );
        args . extend ( format!("{k}={v!r}" for ( k , v ) in self . keywords . items ( ) ));
        if type ( self ) . __module__ == "functools" {
        return  f "functools.{qualname}({', '.join(args)})";
        return  f "{qualname}({', '.join(args)})";
        pub fn __reduce__ ( self )  {
        return  type ( self ) , ( self . func , ) , ( self . func , self . args ,;
        self . keywords || None /* Option */ , self . __dict__ || None /* Option */ );
        pub fn __setstate__ ( &self, state )  {
        if !isinstance ( state , tuple ) {
        panic!("TypeError ( "argument to __setstate__ must be a tuple" )");
        if len ( state ) != 4 {
        panic!("TypeError ( f "expected 4 items in state, got {len(state)}" )");
        func , args , kwds , namespace = state;
        if ( !callable ( func ) || !isinstance ( args , tuple ) or {
        ( kwds == !None /* Option */ && !isinstance ( kwds , dict ) ) or;
        ( namespace == !None /* Option */ && !isinstance ( namespace , dict ) ) ) ;
        panic!("TypeError ( "invalid partial state" )");
        args = tuple ( args );
        if kwds is None /* Option */ {
        kwds = { };
        } else if type ( kwds ) is !dict {
        kwds = dict ( kwds );
        if namespace is None /* Option */ {
        namespace = { };
        self . __dict__ = namespace;
        self . func = func;
        self . args = args;
        self . keywords = kwds;
        // try {
        from _functools import partial;
        // } catch  ImportError  {
        // pass
        class partialmethod ( object ) ;
        "Method descriptor with partial application of the given arguments
    && keywords.

    Supports wrapping existing descriptors && handles non-descriptor
    callables as instance methods.
    ";
        pub fn __init__ ( &self, func , / , * args , ** keywords )  {
        if !callable ( func ) && !hasattr ( func , "__get__" ) {
        panic!("TypeError ( "{!r} is !callable || a descriptor"");
        . format ( func ) );
        if isinstance ( func , partialmethod ) {
        self . func = func . func;
        self . args = func . args + args;
        self . keywords = { ** func . keywords , ** keywords };
        } else {
        self . func = func;
        self . args = args;
        self . keywords = keywords;
        pub fn __repr__ ( self )  {
        args = ", " . join ( map ( repr , self . args ) );
        keywords = ", " . join ( "{}={!r}" . format ( k , v );
        for k , v in self . keywords . items ( ) ).iter() {
        format_string = "{module}.{cls}({func}, {args}, {keywords})";
        return  format_string . format ( module = self . __class__ . __module__ ,;
        cls = self . __class__ . __qualname__ ,;
        func = self . func ,;
        args = args ,;
        keywords = keywords );
        pub fn _make_unbound_method ( self )  {
        pub fn _method ( cls_or_&self, / , * args , ** keywords )  {
        keywords = { ** self . keywords , ** keywords };
        return  self . func ( cls_or_self , * self . args , * args , ** keywords );
        _method . __isabstractmethod__ = self . __isabstractmethod__;
        _method . _partialmethod = self;
        return  _method;
        pub fn __get__ ( &self, obj , cls = None /* Option */ )  {
        get = getattr ( self . func , "__get__" , None /* Option */ );
        result = None /* Option */;
        if get is !None /* Option */ {
        new_func = get ( obj , cls );
        if new_func is !self . func {
        result = partial ( new_func , * self . args , ** self . keywords );
        // try {
        result . __self__ = new_func . __self__;
        // } catch  AttributeError  {
        // pass
        if result is None /* Option */ {
        result = self . _make_unbound_method ( ) . __get__ ( obj , cls );
        return  result;
        @ property;
        pub fn __isabstractmethod__ ( self )  {
        return  getattr ( self . func , "__isabstractmethod__" , false );
        __class_getitem__ = classmethod ( GenericAlias );
        pub fn _unwrap_partial ( func )  {
        while isinstance ( func , partial )  {
        func = func . func;
        return  func;
        _CacheInfo = namedtuple ( "CacheInfo" , [ "hits" , "misses" , "maxsize" , "currsize" ] );
        class _HashedSeq ( list ) ;
        " This class guarantees that hash() will be called no more than once
        per element.  This == important because the lru_cache() will hash
        the key multiple times on a cache miss.

    ";
        __slots__ = "hashvalue";
        pub fn __init__ ( &self, tup , hash = hash )  {
        self [ : ] = tup;
        self . hashvalue = hash ( tup );
        pub fn __hash__ ( self )  {
        return  self . hashvalue;
        pub fn _make_key ( args , kwds , typed , {
        kwd_mark = ( object ( ) , ) ,;
        fasttypes = { int , str } ,;
        tuple = tuple , type = type , len = len ) ;
        "Make a cache key from optionally typed positional && keyword arguments

    The key == constructed in a way that == flat as possible rather than
    as a nested structure that would take more memory.

    If there == only a single argument && its data type == known to cache
    its hash value, then that argument == returned without a wrapper.  This
    saves space && improves lookup speed.

    ";
        key = args;
        if kwds {
        key + = kwd_mark;
        for item in kwds . items ( ) .iter() {
        key + = item;
        if typed {
        key + = tuple ( type ( v ) for v in args );
        if kwds {
        key + = tuple ( type ( v ) for v in kwds . values ( ) );
        } else if len ( key ) == 1 && type ( key [ 0 ] ) in fasttypes {
        return  key [ 0 ];
        return  _HashedSeq ( key );
        pub fn lru_cache ( maxsize = 128 , typed = false )  {
        "Least-recently-used cache decorator.

    If *maxsize* == set to None /* Option */, the LRU features are disabled && the cache
    can grow without bound.

    If *typed* == true, arguments of different types will be cached separately.
    For example, f(3.0) && f(3) will be treated as distinct calls with
    distinct results.

    Arguments to the cached function must be hashable.

    View the cache statistics named tuple (hits, misses, maxsize, currsize)
    with f.cache_info().  Clear the cache && statistics with f.cache_clear().
    Access the underlying function with f.__wrapped__.

    See:  https://en.wikipedia.org/wiki/Cache_replacement_policies#Least_recently_used_(LRU)

    ";
        if isinstance ( maxsize , int ) {
        if maxsize < 0 {
        maxsize = 0;
        } else if callable ( maxsize ) && isinstance ( typed , bool ) {
        user_function , maxsize = maxsize , 128;
        wrapper = _lru_cache_wrapper ( user_function , maxsize , typed , _CacheInfo );
        wrapper . cache_parameters = || {  { "maxsize" : maxsize , "typed" : typed } };
        return  update_wrapper ( wrapper , user_function );
        } else if maxsize is !None /* Option */ {
        panic!("TypeError (");
        "Expected first argument to be an integer, a callable, || None /* Option */" );
        pub fn decorating_function ( user_function )  {
        wrapper = _lru_cache_wrapper ( user_function , maxsize , typed , _CacheInfo );
        wrapper . cache_parameters = || {  { "maxsize" : maxsize , "typed" : typed } };
        return  update_wrapper ( wrapper , user_function );
        return  decorating_function;
        pub fn _lru_cache_wrapper ( user_function , maxsize , typed , _CacheInfo )  {
        sentinel = object ( );
        make_key = _make_key;
        PREV , NEXT , KEY , RESULT = 0 , 1 , 2 , 3;
        cache = { };
        hits = misses = 0;
        full = false;
        cache_get = cache . get;
        cache_len = cache . __len__;
        lock = RLock ( );
        root = [ ];
        root [ : ] = [ root , root , None /* Option */ , None /* Option */ ];
        if maxsize == 0 {
        pub fn wrapper ( * args , ** kwds )  {
        nonlocal misses;
        misses + = 1;
        result = user_function ( * args , ** kwds );
        return  result;
        } else if maxsize is None /* Option */ {
        pub fn wrapper ( * args , ** kwds )  {
        nonlocal hits , misses;
        key = make_key ( args , kwds , typed );
        result = cache_get ( key , sentinel );
        if result is !sentinel {
        hits + = 1;
        return  result;
        misses + = 1;
        result = user_function ( * args , ** kwds );
        cache [ key ] = result;
        return  result;
        } else {
        pub fn wrapper ( * args , ** kwds )  {
        nonlocal root , hits , misses , full;
        key = make_key ( args , kwds , typed );
        // with scope: lock  {
        link = cache_get ( key );
        if link is !None /* Option */ {
        link_prev , link_next , _key , result = link;
        link_prev [ NEXT ] = link_next;
        link_next [ PREV ] = link_prev;
        last = root [ PREV ];
        last [ NEXT ] = root [ PREV ] = link;
        link [ PREV ] = last;
        link [ NEXT ] = root;
        hits + = 1;
        return  result;
        misses + = 1;
        result = user_function ( * args , ** kwds );
        // with scope: lock  {
        if key in cache {
        // pass
        } else if full {
        oldroot = root;
        oldroot [ KEY ] = key;
        oldroot [ RESULT ] = result;
        root = oldroot [ NEXT ];
        oldkey = root [ KEY ];
        oldresult = root [ RESULT ];
        root [ KEY ] = root [ RESULT ] = None /* Option */;
        del cache [ oldkey ];
        cache [ key ] = oldroot;
        } else {
        last = root [ PREV ];
        link = [ last , root , key , result ];
        last [ NEXT ] = root [ PREV ] = cache [ key ] = link;
        full = ( cache_len ( ) >= maxsize );
        return  result;
        pub fn cache_info ( )  {
        "Report cache statistics";
        // with scope: lock  {
        return  _CacheInfo ( hits , misses , maxsize , cache_len ( ) );
        pub fn cache_clear ( )  {
        "Clear the cache && cache statistics";
        nonlocal hits , misses , full;
        // with scope: lock  {
        cache . clear ( );
        root [ : ] = [ root , root , None /* Option */ , None /* Option */ ];
        hits = misses = 0;
        full = false;
        wrapper . cache_info = cache_info;
        wrapper . cache_clear = cache_clear;
        return  wrapper;
        // try {
        from _functools import _lru_cache_wrapper;
        // } catch  ImportError  {
        // pass
        pub fn cache ( user_function , / )  {
        "Simple lightweight unbounded cache.  Sometimes called "memoize".";
        return  lru_cache ( maxsize = None /* Option */ ) ( user_function );
        pub fn _c3_merge ( sequences )  {
        "Merges MROs in *sequences* to a single MRO using the C3 algorithm.

    Adapted from https://www.python.org/download/releases/2.3/mro/.

    ";
        result = [ ];
        while true  {
        sequences = vec![ s.iter().map(|s| sequences if s ).collect();
        if !sequences {
        return  result;
        for s1 in sequences .iter() {
        candidate = s1 [ 0 ];
        for s2 in sequences .iter() {
        if candidate in s2 [ 1 { : ] ; }
        candidate = None /* Option */;
        break;
        } else {
        break;
        if candidate is None /* Option */ {
        panic!("RuntimeError ( "Inconsistent hierarchy" )");
        result . append ( candidate );
        for seq in sequences .iter() {
        if seq [ 0 ] == candidate {
        del seq [ 0 ];
        pub fn _c3_mro ( cls , abcs = None /* Option */ )  {
        "Computes the method resolution order using extended C3 linearization.

    If no *abcs* are given, the algorithm works exactly like the built-in C3
    linearization used for method resolution.

    If given, *abcs* == a list of abstract base classes that should be inserted
    into the resulting MRO. Unrelated ABCs are ignored && don't end up in the
    result. The algorithm inserts ABCs where their functionality == introduced,
    i.e. issubclass(cls, abc) returns true for the class itself but returns
    false for all its direct base classes. Implicit ABCs for a given class
    (either registered || inferred from the presence of a special method like
    __len__) are inserted directly after the last ABC explicitly listed in the
    MRO of said class. If two implicit ABCs end up next to each other in the
    resulting MRO, their ordering depends on the order of types in *abcs*.

    ";
        for i , base in enumerate ( reversed ( cls . __bases__ ) ) .iter() {
        if hasattr ( base , "__abstractmethods__" ) {
        boundary = len ( cls . __bases__ ) - i;
        break;
        } else {
        boundary = 0;
        abcs = list ( abcs ) if abcs else [ ];
        explicit_bases = list ( cls . __bases__ [ : boundary ] );
        abstract_bases = [ ];
        other_bases = list ( cls . __bases__ [ boundary : ] );
        for base in abcs .iter() {
        if issubclass ( cls , base ) && !any ( {
        issubclass ( b , base ) for b in cls . __bases__;
        ) ;
        abstract_bases . append ( base );
        for base in abstract_bases .iter() {
        abcs . remove ( base );
        explicit_c3_mros = vec![ _c3_mro ( base , abcs = abcs ).iter().map(|base| explicit_bases ).collect();
        abstract_c3_mros = vec![ _c3_mro ( base , abcs = abcs ).iter().map(|base| abstract_bases ).collect();
        other_c3_mros = vec![ _c3_mro ( base , abcs = abcs ).iter().map(|base| other_bases ).collect();
        return  _c3_merge (;
        [ [ cls ] ] +;
        explicit_c3_mros + abstract_c3_mros + other_c3_mros +;
        [ explicit_bases ] + [ abstract_bases ] + [ other_bases ];
        );
        pub fn _compose_mro ( cls , types )  {
        "Calculates the method resolution order for a given class *cls*.

    Includes relevant abstract base classes (with their respective bases) from
    the *types* iterable. Uses a modified C3 linearization algorithm.

    ";
        bases = set ( cls . __mro__ );
        pub fn is_related ( typ )  {
        return  ( typ !in bases && hasattr ( typ , "__mro__" );
        and !isinstance ( typ , GenericAlias );
        and issubclass ( cls , typ ) );
        types = vec![ n.iter().map(|n| types if is_related ( n ) ).collect();
        pub fn is_strict_base ( typ )  {
        for other in types .iter() {
        if typ != other && typ in other . __mro__ {
        return  true;
        return  false;
        types = vec![ n.iter().map(|n| types if !is_strict_base ( n ) ).collect();
        type_set = set ( types );
        mro = [ ];
        for typ in types .iter() {
        found = [ ];
        for sub in typ . __subclasses__ ( ) .iter() {
        if sub !in bases && issubclass ( cls , sub ) {
        found . append ( vec![ s.iter().map(|s| sub . __mro__ if s| type_set ] );
        if !found {
        mro . append ( typ );
        continue;
        found . sort ( key = len , reverse = true );
        for sub in found .iter() {
        for subcls in sub .iter() {
        if subcls !in mro {
        mro . append ( subcls );
        return  _c3_mro ( cls , abcs = mro );
        pub fn _find_impl ( cls , registry )  {
        "Returns the best matching implementation from *registry* for type *cls*.

    Where there == no registered implementation for a specific type, its method
    resolution order == used to find a more generic implementation.

    Note: if *registry* does !contain an implementation for the base
    *object* type, this function may return None /* Option */.

    ";
        mro = _compose_mro ( cls , registry . keys ( ) );
        match = None /* Option */;
        for t in mro .iter() {
        if match is !None /* Option */ {
        if ( t in registry && t !in cls . __mro__ {
        and match !in cls . __mro__;
        and !issubclass ( match , t ) ) ;
        panic!("RuntimeError ( "Ambiguous dispatch: {} || {}" . format (");
        match , t ) );
        break;
        if t in registry {
        match = t;
        return  registry . get ( match );
        pub fn singledispatch ( func )  {
        "Single-dispatch generic function decorator.

    Transforms a function into a generic function, which can have different
    behaviours depending upon the type of its first argument. The decorated
    function acts as the default implementation, && additional
    implementations can be registered using the register() attribute of the
    generic function.
    ";
        import types , weakref;
        registry = { };
        dispatch_cache = weakref . WeakKeyDictionary ( );
        cache_token = None /* Option */;
        pub fn dispatch ( cls )  {
        "generic_func.dispatch(cls) -> <function implementation>

        Runs the dispatch algorithm to return the best available implementation
        for the given *cls* registered on *generic_func*.

        ";
        nonlocal cache_token;
        if cache_token is !None /* Option */ {
        current_token = get_cache_token ( );
        if cache_token != current_token {
        dispatch_cache . clear ( );
        cache_token = current_token;
        // try {
        impl = dispatch_cache [ cls ];
        // } catch  KeyError  {
        // try {
        impl = registry [ cls ];
        // } catch  KeyError  {
        impl = _find_impl ( cls , registry );
        dispatch_cache [ cls ] = impl;
        return  impl;
        pub fn _is_union_type ( cls )  {
        from typing import get_origin , Union;
        return  get_origin ( cls ) in { Union , types . UnionType };
        pub fn _is_valid_dispatch_type ( cls )  {
        if isinstance ( cls , type ) {
        return  true;
        from typing import get_args;
        return  ( _is_union_type ( cls ) and;
        all ( isinstance ( arg , type ) for arg in get_args ( cls ) ) );
        pub fn register ( cls , func = None /* Option */ )  {
        "generic_func.register(cls, func) -> func

        Registers a new implementation for the given *cls* on a *generic_func*.

        ";
        nonlocal cache_token;
        if _is_valid_dispatch_type ( cls ) {
        if func is None /* Option */ {
        return  lambda f : register ( cls , f );
        } else {
        if func is !None /* Option */ {
        panic!("TypeError (");
        format!("Invalid first argument to `register()`. ");
        format!("{cls!r} == !a class || union type.");
        );
        ann = getattr ( cls , "__annotations__" , { } );
        if !ann {
        panic!("TypeError (");
        format!("Invalid first argument to `register()`: {cls!r}. ");
        format!("Use either `@register(some_class)` || plain `@register` ");
        format!("on an annotated function.");
        );
        func = cls;
        from typing import get_type_hints;
        argname , cls = next ( iter ( get_type_hints ( func ) . items ( ) ) );
        if !_is_valid_dispatch_type ( cls ) {
        if _is_union_type ( cls ) {
        panic!("TypeError (");
        format!("Invalid annotation for {argname!r}. ");
        format!("{cls!r} !all arguments are classes.");
        );
        } else {
        panic!("TypeError (");
        format!("Invalid annotation for {argname!r}. ");
        format!("{cls!r} == !a class.");
        );
        if _is_union_type ( cls ) {
        from typing import get_args;
        for arg in get_args ( cls ) .iter() {
        registry [ arg ] = func;
        } else {
        registry [ cls ] = func;
        if cache_token is None /* Option */ && hasattr ( cls , "__abstractmethods__" ) {
        cache_token = get_cache_token ( );
        dispatch_cache . clear ( );
        return  func;
        pub fn wrapper ( * args , ** kw )  {
        if !args {
        panic!("TypeError ( f "{funcname} requires at least "");
        "1 positional argument" );
        return  dispatch ( args [ 0 ] . __class__ ) ( * args , ** kw );
        funcname = getattr ( func , "__name__" , "singledispatch function" );
        registry [ object ] = func;
        wrapper . register = register;
        wrapper . dispatch = dispatch;
        wrapper . registry = types . MappingProxyType ( registry );
        wrapper . _clear_cache = dispatch_cache . clear;
        update_wrapper ( wrapper , func );
        return  wrapper;
        class singledispatchmethod ;
        "Single-dispatch generic method descriptor.

    Supports wrapping existing descriptors && handles non-descriptor
    callables as instance methods.
    ";
        pub fn __init__ ( &self, func )  {
        if !callable ( func ) && !hasattr ( func , "__get__" ) {
        panic!("TypeError ( f "{func!r} is !callable || a descriptor" )");
        self . dispatcher = singledispatch ( func );
        self . func = func;
        pub fn register ( &self, cls , method = None /* Option */ )  {
        "generic_method.register(cls, func) -> func

        Registers a new implementation for the given *cls* on a *generic_method*.
        ";
        return  self . dispatcher . register ( cls , func = method );
        pub fn __get__ ( &self, obj , cls = None /* Option */ )  {
        pub fn _method ( * args , ** kwargs )  {
        method = self . dispatcher . dispatch ( args [ 0 ] . __class__ );
        return  method . __get__ ( obj , cls ) ( * args , ** kwargs );
        _method . __isabstractmethod__ = self . __isabstractmethod__;
        _method . register = self . register;
        update_wrapper ( _method , self . func );
        return  _method;
        @ property;
        pub fn __isabstractmethod__ ( self )  {
        return  getattr ( self . func , "__isabstractmethod__" , false );
        _NOT_FOUND = object ( );
        class cached_property ;
        pub fn __init__ ( &self, func )  {
        self . func = func;
        self . attrname = None /* Option */;
        self . __doc__ = func . __doc__;
        self . lock = RLock ( );
        pub fn __set_name__ ( &self, owner , name )  {
        if self . attrname is None /* Option */ {
        self . attrname = name;
        } else if name != self . attrname {
        panic!("TypeError (");
        "Cannot assign the same cached_property to two different names ";
        format!("({self.attrname!r} && {name!r}).");
        );
        pub fn __get__ ( &self, instance , owner = None /* Option */ )  {
        if instance is None /* Option */ {
        return  self;
        if self . attrname is None /* Option */ {
        panic!("TypeError (");
        "Cannot use cached_property instance without calling __set_name__ on it." );
        // try {
        cache = instance . __dict__;
        // } catch  AttributeError  {
        msg = (;
        format!("No '__dict__' attribute on {type(instance).__name__!r} ");
        format!("instance to cache {self.attrname!r} property.");
        );
        panic!("TypeError ( msg ) from None /* Option */");
        val = cache . get ( self . attrname , _NOT_FOUND );
        if val is _NOT_FOUND {
        // with scope: self . lock  {
        val = cache . get ( self . attrname , _NOT_FOUND );
        if val is _NOT_FOUND {
        val = self . func ( instance );
        // try {
        cache [ self . attrname ] = val;
        // } catch  TypeError  {
        msg = (;
        format!("The '__dict__' attribute on {type(instance).__name__!r} instance ");
        format!("does !support item assignment for caching {self.attrname!r} property.");
        );
        panic!("TypeError ( msg ) from None /* Option */");
        return  val;
        __class_getitem__ = classmethod ( GenericAlias );
}

