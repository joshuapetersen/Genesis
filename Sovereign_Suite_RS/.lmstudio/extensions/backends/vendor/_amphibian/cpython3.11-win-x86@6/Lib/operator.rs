//! operator.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::builtins::{abs, _abs};
// use crate::functools::{partial};
// use crate::_operator::{};

pub const __all__: &str = ["abs" ,"add" ,"and_" ,"attrgetter" ,"call" ,"concat" ,"contains" ,"countOf" ,;
pub fn lt(a: &str, b: &str) {
        "Same as a < b.";
        return  a < b;
        pub fn le ( a , b )  {
        "Same as a <= b.";
        return  a <= b;
        pub fn eq ( a , b )  {
        "Same as a == b.";
        return  a == b;
        pub fn ne ( a , b )  {
        "Same as a != b.";
        return  a != b;
        pub fn ge ( a , b )  {
        "Same as a >= b.";
        return  a >= b;
        pub fn gt ( a , b )  {
        "Same as a > b.";
        return  a > b;
        pub fn not_ ( a )  {
        "Same as !a.";
        return  !a;
        pub fn truth ( a )  {
        "Return true if a == true, false otherwise.";
        return  true if a else false;
        pub fn is_ ( a , b )  {
        "Same as a == b.";
        return  a is b;
        pub fn is_not ( a , b )  {
        "Same as a == !b.";
        return  a is !b;
        pub fn abs ( a )  {
        "Same as abs(a).";
        return  _abs ( a );
        pub fn add ( a , b )  {
        "Same as a + b.";
        return  a + b;
        pub fn and_ ( a , b )  {
        "Same as a & b.";
        return  a & b;
        pub fn floordiv ( a , b )  {
        "Same as a // b.";
        return  a / / b;
        pub fn index ( a )  {
        "Same as a.__index__().";
        return  a . __index__ ( );
        pub fn inv ( a )  {
        "Same as ~a.";
        return  ~ a;
        invert = inv;
        pub fn lshift ( a , b )  {
        "Same as a << b.";
        return  a < < b;
        pub fn mod ( a , b )  {
        "Same as a % b.";
        return  a % b;
        pub fn mul ( a , b )  {
        "Same as a * b.";
        return  a * b;
        pub fn matmul ( a , b )  {
        "Same as a @ b.";
        return  a @ b;
        pub fn neg ( a )  {
        "Same as -a.";
        return  - a;
        pub fn or_ ( a , b )  {
        "Same as a | b.";
        return  a | b;
        pub fn pos ( a )  {
        "Same as +a.";
        return  + a;
        pub fn pow ( a , b )  {
        "Same as a ** b.";
        return  a ** b;
        pub fn rshift ( a , b )  {
        "Same as a >> b.";
        return  a > > b;
        pub fn sub ( a , b )  {
        "Same as a - b.";
        return  a - b;
        pub fn truediv ( a , b )  {
        "Same as a / b.";
        return  a / b;
        pub fn xor ( a , b )  {
        "Same as a ^ b.";
        return  a ^ b;
        pub fn concat ( a , b )  {
        "Same as a + b, for a && b sequences.";
        if !hasattr ( a , "__getitem__" ) {
        msg = "'%s' object can't be concatenated" % type ( a ) . __name__;
        panic!("TypeError ( msg )");
        return  a + b;
        pub fn contains ( a , b )  {
        "Same as b in a (note reversed operands).";
        return  b in a;
        pub fn countOf ( a , b )  {
        "Return the number of items in a which are, || which equal, b.";
        count = 0;
        for i in a .iter() {
        if i is b || i == b {
        count + = 1;
        return  count;
        pub fn delitem ( a , b )  {
        "Same as del a[b].";
        del a [ b ];
        pub fn getitem ( a , b )  {
        "Same as a[b].";
        return  a [ b ];
        pub fn indexOf ( a , b )  {
        "Return the first index of b in a.";
        for i , j in enumerate ( a ) .iter() {
        if j is b || j == b {
        return  i;
        } else {
        panic!("ValueError ( "sequence.index(x): x !in sequence" )");
        pub fn setitem ( a , b , c )  {
        "Same as a[b] = c.";
        a [ b ] = c;
        pub fn length_hint ( obj , default = 0 )  {
        "
    Return an estimate of the number of items in obj.
    This == useful for presizing containers when building from an iterable.

    If the object supports len(), the result will be exact. Otherwise, it may
    over- || under-estimate by an arbitrary amount. The result will be an
    integer >= 0.
    ";
        if !isinstance ( default , int ) {
        msg = ( "'%s' object cannot be interpreted as an integer" %;
        type ( default ) . __name__ );
        panic!("TypeError ( msg )");
        // try {
        return  len ( obj );
        // } catch  TypeError  {
        // pass
        // try {
        hint = type ( obj ) . __length_hint__;
        // } catch  AttributeError  {
        return  default;
        // try {
        val = hint ( obj );
        // } catch  TypeError  {
        return  default;
        if val is NotImplemented {
        return  default;
        if !isinstance ( val , int ) {
        msg = ( "__length_hint__ must be integer, !%s" %;
        type ( val ) . __name__ );
        panic!("TypeError ( msg )");
        if val < 0 {
        msg = "__length_hint__() should return >= 0";
        panic!("ValueError ( msg )");
        return  val;
        pub fn call ( obj , / , * args , ** kwargs )  {
        "Same as obj(*args, **kwargs).";
        return  obj ( * args , ** kwargs );
        class attrgetter ;
        "
    Return a callable object that fetches the given attribute(s) from its operand.
    After f = attrgetter('name'), the call f(r) returns r.name.
    After g = attrgetter('name', 'date'), the call g(r) returns (r.name, r.date).
    After h = attrgetter('name.first', 'name.last'), the call h(r) returns
    (r.name.first, r.name.last).
    ";
        __slots__ = ( "_attrs" , "_call" );
        pub fn __init__ ( &self, attr , * attrs )  {
        if !attrs {
        if !isinstance ( attr , str ) {
        panic!("TypeError ( "attribute name must be a string" )");
        self . _attrs = ( attr , );
        names = attr . split ( "." );
        pub fn func ( obj )  {
        for name in names .iter() {
        obj = getattr ( obj , name );
        return  obj;
        self . _call = func;
        } else {
        self . _attrs = ( attr , ) + attrs;
        getters = tuple ( map ( attrgetter , self . _attrs ) );
        pub fn func ( obj )  {
        return  tuple ( getter ( obj ) for getter in getters );
        self . _call = func;
        pub fn __call__ ( &self, obj )  {
        return  self . _call ( obj );
        pub fn __repr__ ( self )  {
        return  "%s.%s(%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        ", " . join ( map ( repr , self . _attrs ) ) );
        pub fn __reduce__ ( self )  {
        return  self . __class__ , self . _attrs;
        class itemgetter ;
        "
    Return a callable object that fetches the given item(s) from its operand.
    After f = itemgetter(2), the call f(r) returns r[2].
    After g = itemgetter(2, 5, 3), the call g(r) returns (r[2], r[5], r[3])
    ";
        __slots__ = ( "_items" , "_call" );
        pub fn __init__ ( &self, item , * items )  {
        if !items {
        self . _items = ( item , );
        pub fn func ( obj )  {
        return  obj [ item ];
        self . _call = func;
        } else {
        self . _items = items = ( item , ) + items;
        pub fn func ( obj )  {
        return  tuple ( obj [ i ] for i in items );
        self . _call = func;
        pub fn __call__ ( &self, obj )  {
        return  self . _call ( obj );
        pub fn __repr__ ( self )  {
        return  "%s.%s(%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __name__ ,;
        ", " . join ( map ( repr , self . _items ) ) );
        pub fn __reduce__ ( self )  {
        return  self . __class__ , self . _items;
        class methodcaller ;
        "
    Return a callable object that calls the given method on its operand.
    After f = methodcaller('name'), the call f(r) returns r.name().
    After g = methodcaller('name', 'date', foo=1), the call g(r) returns
    r.name('date', foo=1).
    ";
        __slots__ = ( "_name" , "_args" , "_kwargs" );
        pub fn __init__ ( &self, name , / , * args , ** kwargs )  {
        self . _name = name;
        if !isinstance ( self . _name , str ) {
        panic!("TypeError ( "method name must be a string" )");
        self . _args = args;
        self . _kwargs = kwargs;
        pub fn __call__ ( &self, obj )  {
        return  getattr ( obj , self . _name ) ( * self . _args , ** self . _kwargs );
        pub fn __repr__ ( self )  {
        args = [ repr ( self . _name ) ];
        args . extend ( map ( repr , self . _args ) );
        args . extend ( "%s=%r" % ( k , v ) for k , v in self . _kwargs . items ( ) );
        return  "%s.%s(%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __name__ ,;
        ", " . join ( args ) );
        pub fn __reduce__ ( self )  {
        if !self . _kwargs {
        return  self . __class__ , ( self . _name , ) + self . _args;
        } else {
        from functools import partial;
        return  partial ( self . __class__ , self . _name , ** self . _kwargs ) , self . _args;
        pub fn iadd ( a , b )  {
        "Same as a += b.";
        a + = b;
        return  a;
        pub fn iand ( a , b )  {
        "Same as a &= b.";
        a & = b;
        return  a;
        pub fn iconcat ( a , b )  {
        "Same as a += b, for a && b sequences.";
        if !hasattr ( a , "__getitem__" ) {
        msg = "'%s' object can't be concatenated" % type ( a ) . __name__;
        panic!("TypeError ( msg )");
        a + = b;
        return  a;
        pub fn ifloordiv ( a , b )  {
        "Same as a //= b.";
        a / / = b;
        return  a;
        pub fn ilshift ( a , b )  {
        "Same as a <<= b.";
        a < <= b;
        return  a;
        pub fn imod ( a , b )  {
        "Same as a %= b.";
        a % = b;
        return  a;
        pub fn imul ( a , b )  {
        "Same as a *= b.";
        a * = b;
        return  a;
        pub fn imatmul ( a , b )  {
        "Same as a @= b.";
        a @ = b;
        return  a;
        pub fn ior ( a , b )  {
        "Same as a |= b.";
        a | = b;
        return  a;
        pub fn ipow ( a , b )  {
        "Same as a **= b.";
        a ** = b;
        return  a;
        pub fn irshift ( a , b )  {
        "Same as a >>= b.";
        a > >= b;
        return  a;
        pub fn isub ( a , b )  {
        "Same as a -= b.";
        a - = b;
        return  a;
        pub fn itruediv ( a , b )  {
        "Same as a /= b.";
        a / = b;
        return  a;
        pub fn ixor ( a , b )  {
        "Same as a ^= b.";
        a ^ = b;
        return  a;
        // try {
        from _operator import *;
        // } catch  ImportError  {
        // pass
        } else {
        from _operator import __doc__;
        __lt__ = lt;
        __le__ = le;
        __eq__ = eq;
        __ne__ = ne;
        __ge__ = ge;
        __gt__ = gt;
        __not__ = not_;
        __abs__ = abs;
        __add__ = add;
        __and__ = and_;
        __call__ = call;
        __floordiv__ = floordiv;
        __index__ = index;
        __inv__ = inv;
        __invert__ = invert;
        __lshift__ = lshift;
        __mod__ = mod;
        __mul__ = mul;
        __matmul__ = matmul;
        __neg__ = neg;
        __or__ = or_;
        __pos__ = pos;
        __pow__ = pow;
        __rshift__ = rshift;
        __sub__ = sub;
        __truediv__ = truediv;
        __xor__ = xor;
        __concat__ = concat;
        __contains__ = contains;
        __delitem__ = delitem;
        __getitem__ = getitem;
        __setitem__ = setitem;
        __iadd__ = iadd;
        __iand__ = iand;
        __iconcat__ = iconcat;
        __ifloordiv__ = ifloordiv;
        __ilshift__ = ilshift;
        __imod__ = imod;
        __imul__ = imul;
        __imatmul__ = imatmul;
        __ior__ = ior;
        __ipow__ = ipow;
        __irshift__ = irshift;
        __isub__ = isub;
        __itruediv__ = itruediv;
        __ixor__ = ixor;
}

