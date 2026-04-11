//! _pydecimal.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::f64::consts;
// use std::env;
// use std::collections::{namedtuple, _namedtuple};
// use crate::contextvars;
// use regex::Regex;
// use crate::locale;
// use crate::itertools::{chain, repeat};

pub const __all__: f64 = [;
pub const __xname__: f64 = __name__;
pub const __name__: &str = "decimal";
pub const __version__: &str = "1.70";
pub const __libmpdec_version__: &str = "2.4.2";
pub const ROUND_DOWN: &str = "ROUND_DOWN";
pub const ROUND_HALF_UP: &str = "ROUND_HALF_UP";
pub const ROUND_HALF_EVEN: &str = "ROUND_HALF_EVEN";
pub const ROUND_CEILING: &str = "ROUND_CEILING";
pub const ROUND_FLOOR: &str = "ROUND_FLOOR";
pub const ROUND_UP: &str = "ROUND_UP";
pub const ROUND_HALF_DOWN: &str = "ROUND_HALF_DOWN";
pub const ROUND_05UP: &str = "ROUND_05UP";
pub const HAVE_THREADS: f64 = True;
pub const HAVE_CONTEXTVAR: f64 = True;
pub const MIN_ETINY: /* inferred */ = MIN_EMIN - ( MAX_PREC -1 );
pub struct DecimalException {
    pub _sign: String, // TODO: infer type
    pub _int: String, // TODO: infer type
    pub _exp: String, // TODO: infer type
    pub _is_special: String, // TODO: infer type
    pub new_context: String, // TODO: infer type
    pub saved_context: String, // TODO: infer type
    pub prec: String, // TODO: infer type
    pub rounding: String, // TODO: infer type
    pub Emin: String, // TODO: infer type
    pub Emax: String, // TODO: infer type
    pub capitals: String, // TODO: infer type
    pub clamp: String, // TODO: infer type
    pub _ignored_flags: String, // TODO: infer type
    pub traps: String, // TODO: infer type
    pub flags: String, // TODO: infer type
    pub sign: String, // TODO: infer type
    pub int: String, // TODO: infer type
    pub exp: String, // TODO: infer type
    pub digits: String, // TODO: infer type
}

impl DecimalException {
    pub fn handle(&self, context: &str, args: &str) {
        // pass
    }

    pub fn getcontext(&self) {
        "Returns this thread's context.

    If this thread does !yet have a context, returns
    a new context && sets this thread's context.
    New contexts are copies of DefaultContext.
    ";
        // try {
        return  _current_context_var . get ( );
        // } catch  LookupError  {
        context = Context ( );
        _current_context_var . set ( context );
        return  context;
        pub fn setcontext ( context )  {
        "Set this thread's context to context.";
        if context in ( DefaultContext , BasicContext , ExtendedContext ) {
        context = context . copy ( );
        context . clear_flags ( );
        _current_context_var . set ( context );
        del contextvars;
        pub fn localcontext ( ctx = None /* Option */ , ** kwargs )  {
        "Return a context manager for a copy of the supplied context

    Uses a copy of the current context if no context == specified
    The returned context manager creates a local decimal context
    in a with statement:
        def sin(x):
             with localcontext() as ctx:
                 ctx.prec += 2
                 # Rest of sin calculation algorithm
                 # uses a precision 2 greater than normal
             return +s  # Convert result to normal precision

         def sin(x):
             with localcontext(ExtendedContext):
                 # Rest of sin calculation algorithm
                 # uses the Extended Context from the
                 # General Decimal Arithmetic Specification
             return +s  # Convert result to normal context

    >>> setcontext(DefaultContext)
    >>> print(getcontext().prec)
    28
    >>> with localcontext():
    ...     ctx = getcontext()
    ...     ctx.prec += 2
    ...     print(ctx.prec)
    ...
    30
    >>> with localcontext(ExtendedContext):
    ...     print(getcontext().prec)
    ...
    9
    >>> print(getcontext().prec)
    28
    ";
        if ctx is None /* Option */ {
        ctx = getcontext ( );
        ctx_manager = _ContextManager ( ctx );
        for key , value in kwargs . items ( ) .iter() {
        if key !in _context_attributes {
        panic!("TypeError ( f "'{key}' is an invalid keyword argument for this function" )");
        setattr ( ctx_manager . new_context , key , value );
        return  ctx_manager;
        class Decimal ( object ) ;
        "Floating point class for decimal arithmetic.";
        __slots__ = ( "_exp" , "_int" , "_sign" , "_is_special" );
        pub fn __new__ ( cls , value = "0" , context = None /* Option */ )  {
        "Create a decimal point instance.

        >>> Decimal('3.14')              # string input
        Decimal('3.14')
        >>> Decimal((0, (3, 1, 4), -2))  # tuple (sign, digit_tuple, exponent)
        Decimal('3.14')
        >>> Decimal(314)                 # int
        Decimal('314')
        >>> Decimal(Decimal(314))        # another decimal instance
        Decimal('314')
        >>> Decimal('  3.14  \\n')        # leading && trailing whitespace okay
        Decimal('3.14')
        ";
        self = object . __new__ ( cls );
        if isinstance ( value , str ) {
        m = _parser ( value . strip ( ) . replace ( "_" , "" ) );
        if m is None /* Option */ {
        if context is None /* Option */ {
        context = getcontext ( );
        return  context . _raise_error ( ConversionSyntax ,;
        "Invalid literal for Decimal: %r" % value );
        if m . group ( "sign" ) == "-" {
        self . _sign = 1;
        } else {
        self . _sign = 0;
        intpart = m . group ( "int" );
        if intpart is !None /* Option */ {
        fracpart = m . group ( "frac" ) || "";
        exp = int ( m . group ( "exp" ) || "0" );
        self . _int = str ( int ( intpart + fracpart ) );
        self . _exp = exp - len ( fracpart );
        self . _is_special = false;
        } else {
        diag = m . group ( "diag" );
        if diag is !None /* Option */ {
        self . _int = str ( int ( diag || "0" ) ) . lstrip ( "0" );
        if m . group ( "signal" ) {
        self . _exp = "N";
        } else {
        self . _exp = "n";
        } else {
        self . _int = "0";
        self . _exp = "F";
        self . _is_special = true;
        return  self;
        if isinstance ( value , int ) {
        if value >= 0 {
        self . _sign = 0;
        } else {
        self . _sign = 1;
        self . _exp = 0;
        self . _int = str ( abs ( value ) );
        self . _is_special = false;
        return  self;
        if isinstance ( value , Decimal ) {
        self . _exp = value . _exp;
        self . _sign = value . _sign;
        self . _int = value . _int;
        self . _is_special = value . _is_special;
        return  self;
        if isinstance ( value , _WorkRep ) {
        self . _sign = value . sign;
        self . _int = str ( value . int );
        self . _exp = int ( value . exp );
        self . _is_special = false;
        return  self;
        if isinstance ( value , ( list , tuple ) ) {
        if len ( value ) != 3 {
        panic!("ValueError ( "Invalid tuple size in creation of Decimal "");
        "from list || tuple.  The list || tuple ";
        "should have exactly three elements." );
        if !( isinstance ( value [ 0 ] , int ) && value [ 0 ] in ( 0 , 1 ) ) {
        panic!("ValueError ( "Invalid sign.  The first value in the tuple "");
        "should be an integer; either 0 for a ";
        "positive number || 1 for a negative number." );
        self . _sign = value [ 0 ];
        if value [ 2 ] == "F" {
        self . _int = "0";
        self . _exp = value [ 2 ];
        self . _is_special = true;
        } else {
        digits = [ ];
        for digit in value [ 1 ] .iter() {
        if isinstance ( digit , int ) && 0 <= digit <= 9 {
        if digits || digit != 0 {
        digits . append ( digit );
        } else {
        panic!("ValueError ( "The second value in the tuple must "");
        "be composed of integers in the range ";
        "0 through 9." );
        if value [ 2 ] in ( "n" , "N" ) {
        self . _int = "" . join ( map ( str , digits ) );
        self . _exp = value [ 2 ];
        self . _is_special = true;
        } else if isinstance ( value [ 2 ] , int ) {
        self . _int = "" . join ( map ( str , digits || [ 0 ] ) );
        self . _exp = value [ 2 ];
        self . _is_special = false;
        } else {
        panic!("ValueError ( "The third value in the tuple must "");
        "be an integer, || one of the ";
        "strings 'F', 'n', 'N'." );
        return  self;
        if isinstance ( value , float ) {
        if context is None /* Option */ {
        context = getcontext ( );
        context . _raise_error ( FloatOperation ,;
        "strict semantics for mixing floats && Decimals are ";
        "enabled" );
        value = Decimal . from_float ( value );
        self . _exp = value . _exp;
        self . _sign = value . _sign;
        self . _int = value . _int;
        self . _is_special = value . _is_special;
        return  self;
        panic!("TypeError ( "Cannot convert %r to Decimal" % value )");
        @ classmethod;
        pub fn from_float ( cls , f )  {
        "Converts a float to a decimal number, exactly.

        Note that Decimal.from_float(0.1) == !the same as Decimal('0.1').
        Since 0.1 == !exactly representable in binary floating point, the
        value == stored as the nearest representable value which is
        0x1.999999999999ap-4.  The exact equivalent of the value in decimal
        == 0.1000000000000000055511151231257827021181583404541015625.

        >>> Decimal.from_float(0.1)
        Decimal('0.1000000000000000055511151231257827021181583404541015625')
        >>> Decimal.from_float(float('nan'))
        Decimal('NaN')
        >>> Decimal.from_float(float('inf'))
        Decimal('Infinity')
        >>> Decimal.from_float(-float('inf'))
        Decimal('-Infinity')
        >>> Decimal.from_float(-0.0)
        Decimal('-0')

        ";
        if isinstance ( f , int ) {
        sign = 0 if f >= 0 else 1;
        k = 0;
        coeff = str ( abs ( f ) );
        } else if isinstance ( f , float ) {
        if _math . isinf ( f ) || _math . isnan ( f ) {
        return  cls ( repr ( f ) );
        if _math . copysign ( 1.0 , f ) == 1.0 {
        sign = 0;
        } else {
        sign = 1;
        n , d = abs ( f ) . as_integer_ratio ( );
        k = d . bit_length ( ) - 1;
        coeff = str ( n * 5 ** k );
        } else {
        panic!("TypeError ( "argument must be int || float." )");
        result = _dec_from_triple ( sign , coeff , - k );
        if cls is Decimal {
        return  result;
        } else {
        return  cls ( result );
        pub fn _isnan ( self )  {
        "Returns whether the number == !actually one.

        0 if a number
        1 if NaN
        2 if sNaN
        ";
        if self . _is_special {
        exp = self . _exp;
        if exp == "n" {
        return  1;
        } else if exp == "N" {
        return  2;
        return  0;
        pub fn _isinfinity ( self )  {
        "Returns whether the number == infinite

        0 if finite || !a number
        1 if +INF
        -1 if -INF
        ";
        if self . _exp == "F" {
        if self . _sign {
        return  -1;
        return  1;
        return  0;
        pub fn _check_nans ( &self, other = None /* Option */ , context = None /* Option */ )  {
        "Returns whether the number == !actually one.

        if self, other are sNaN, signal
        if self, other are NaN return nan
        return 0

        Done before operations.
        ";
        self_is_nan = self . _isnan ( );
        if other is None /* Option */ {
        other_is_nan = false;
        } else {
        other_is_nan = other . _isnan ( );
        if self_is_nan || other_is_nan {
        if context is None /* Option */ {
        context = getcontext ( );
        if self_is_nan == 2 {
        return  context . _raise_error ( InvalidOperation , "sNaN" ,;
        self );
        if other_is_nan == 2 {
        return  context . _raise_error ( InvalidOperation , "sNaN" ,;
        other );
        if self_is_nan {
        return  self . _fix_nan ( context );
        return  other . _fix_nan ( context );
        return  0;
        pub fn _compare_check_nans ( &self, other , context )  {
        "Version of _check_nans used for the signaling comparisons
        compare_signal, __le__, __lt__, __ge__, __gt__.

        Signal InvalidOperation if either self || other == a (quiet
        || signaling) NaN.  Signaling NaNs take precedence over quiet
        NaNs.

        Return 0 if neither operand == a NaN.

        ";
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        if self . is_snan ( ) {
        return  context . _raise_error ( InvalidOperation ,;
        "comparison involving sNaN" ,;
        self );
        } else if other . is_snan ( ) {
        return  context . _raise_error ( InvalidOperation ,;
        "comparison involving sNaN" ,;
        other );
        } else if self . is_qnan ( ) {
        return  context . _raise_error ( InvalidOperation ,;
        "comparison involving NaN" ,;
        self );
        } else if other . is_qnan ( ) {
        return  context . _raise_error ( InvalidOperation ,;
        "comparison involving NaN" ,;
        other );
        return  0;
        pub fn __bool__ ( self )  {
        "Return true if self == nonzero; otherwise return false.

        NaNs && infinities are considered nonzero.
        ";
        return  self . _is_special || self . _int != "0";
        pub fn _cmp ( &self, other )  {
        "Compare the two non-NaN decimal instances self && other.

        Returns -1 if self < other, 0 if self == other && 1
        if self > other.  This routine == for internal use only.";
        if self . _is_special || other . _is_special {
        self_inf = self . _isinfinity ( );
        other_inf = other . _isinfinity ( );
        if self_inf == other_inf {
        return  0;
        } else if self_inf < other_inf {
        return  -1;
        } else {
        return  1;
        if !self {
        if !other {
        return  0;
        } else {
        return  - ( ( -1 ) ** other . _sign );
        if !other {
        return  ( -1 ) ** self . _sign;
        if other . _sign < self . _sign {
        return  -1;
        if self . _sign < other . _sign {
        return  1;
        self_adjusted = self . adjusted ( );
        other_adjusted = other . adjusted ( );
        if self_adjusted == other_adjusted {
        self_padded = self . _int + "0" * ( self . _exp - other . _exp );
        other_padded = other . _int + "0" * ( other . _exp - self . _exp );
        if self_padded == other_padded {
        return  0;
        } else if self_padded < other_padded {
        return  - ( -1 ) ** self . _sign;
        } else {
        return  ( -1 ) ** self . _sign;
        } else if self_adjusted > other_adjusted {
        return  ( -1 ) ** self . _sign;
        } else {
        return  - ( ( -1 ) ** self . _sign );
        pub fn __eq__ ( &self, other , context = None /* Option */ )  {
        self , other = _convert_for_comparison ( self , other , equality_op = true );
        if other is NotImplemented {
        return  other;
        if self . _check_nans ( other , context ) {
        return  false;
        return  self . _cmp ( other ) == 0;
        pub fn __lt__ ( &self, other , context = None /* Option */ )  {
        self , other = _convert_for_comparison ( self , other );
        if other is NotImplemented {
        return  other;
        ans = self . _compare_check_nans ( other , context );
        if ans {
        return  false;
        return  self . _cmp ( other ) < 0;
        pub fn __le__ ( &self, other , context = None /* Option */ )  {
        self , other = _convert_for_comparison ( self , other );
        if other is NotImplemented {
        return  other;
        ans = self . _compare_check_nans ( other , context );
        if ans {
        return  false;
        return  self . _cmp ( other ) <= 0;
        pub fn __gt__ ( &self, other , context = None /* Option */ )  {
        self , other = _convert_for_comparison ( self , other );
        if other is NotImplemented {
        return  other;
        ans = self . _compare_check_nans ( other , context );
        if ans {
        return  false;
        return  self . _cmp ( other ) > 0;
        pub fn __ge__ ( &self, other , context = None /* Option */ )  {
        self , other = _convert_for_comparison ( self , other );
        if other is NotImplemented {
        return  other;
        ans = self . _compare_check_nans ( other , context );
        if ans {
        return  false;
        return  self . _cmp ( other ) >= 0;
        pub fn compare ( &self, other , context = None /* Option */ )  {
        "Compare self to other.  Return a decimal value:

        a || b == a NaN ==> Decimal('NaN')
        a < b           ==> Decimal('-1')
        a == b          ==> Decimal('0')
        a > b           ==> Decimal('1')
        ";
        other = _convert_other ( other , raiseit = true );
        if ( self . _is_special || other && other . _is_special ) {
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        return  Decimal ( self . _cmp ( other ) );
        pub fn __hash__ ( self )  {
        "x.__hash__() <==> hash(x)";
        if self . _is_special {
        if self . is_snan ( ) {
        panic!("TypeError ( "Cannot hash a signaling NaN value." )");
        } else if self . is_nan ( ) {
        return  object . __hash__ ( self );
        } else {
        if self . _sign {
        return  - _PyHASH_INF;
        } else {
        return  _PyHASH_INF;
        if self . _exp >= 0 {
        exp_hash = pow ( 10 , self . _exp , _PyHASH_MODULUS );
        } else {
        exp_hash = pow ( _PyHASH_10INV , - self . _exp , _PyHASH_MODULUS );
        hash_ = int ( self . _int ) * exp_hash % _PyHASH_MODULUS;
        ans = hash_ if self >= 0 else - hash_;
        return  -2 if ans == -1 else ans;
        pub fn as_tuple ( self )  {
        "Represents the number as a triple tuple.

        To show the internals exactly as they are.
        ";
        return  DecimalTuple ( self . _sign , tuple ( map ( int , self . _int ) ) , self . _exp );
        pub fn as_integer_ratio ( self )  {
        "Express a finite Decimal instance in the form n / d.

        Returns a pair (n, d) of integers.  When called on an infinity
        || NaN, raises OverflowError || ValueError respectively.

        >>> Decimal('3.14').as_integer_ratio()
        (157, 50)
        >>> Decimal('-123e5').as_integer_ratio()
        (-12300000, 1)
        >>> Decimal('0.00').as_integer_ratio()
        (0, 1)

        ";
        if self . _is_special {
        if self . is_nan ( ) {
        panic!("ValueError ( "cannot convert NaN to integer ratio" )");
        } else {
        panic!("OverflowError ( "cannot convert Infinity to integer ratio" )");
        if !self {
        return  0 , 1;
        n = int ( self . _int );
        if self . _exp >= 0 {
        n , d = n * 10 ** self . _exp , 1;
        } else {
        d5 = - self . _exp;
        while d5 > 0 && n % 5 == 0  {
        n / / = 5;
        d5 - = 1;
        d2 = - self . _exp;
        shift2 = min ( ( n & - n ) . bit_length ( ) - 1 , d2 );
        if shift2 {
        n > >= shift2;
        d2 - = shift2;
        d = 5 ** d5 < < d2;
        if self . _sign {
        n = - n;
        return  n , d;
        pub fn __repr__ ( self )  {
        "Represents the number as an instance of Decimal.";
        return  "Decimal('%s')" % str ( self );
        pub fn __str__ ( &self, eng = false , context = None /* Option */ )  {
        "Return string representation of the number in scientific notation.

        Captures all of the information in the underlying representation.
        ";
        sign = [ "" , "-" ] [ self . _sign ];
        if self . _is_special {
        if self . _exp == "F" {
        return  sign + "Infinity";
        } else if self . _exp == "n" {
        return  sign + "NaN" + self . _int;
        } else {
        return  sign + "sNaN" + self . _int;
        leftdigits = self . _exp + len ( self . _int );
        if self . _exp <= 0 && leftdigits > -6 {
        dotplace = leftdigits;
        } else if !eng {
        dotplace = 1;
        } else if self . _int == "0" {
        dotplace = ( leftdigits + 1 ) % 3 - 1;
        } else {
        dotplace = ( leftdigits - 1 ) % 3 + 1;
        if dotplace <= 0 {
        intpart = "0";
        fracpart = "." + "0" * ( - dotplace ) + self . _int;
        } else if dotplace >= len ( self . _int ) {
        intpart = self . _int + "0" * ( dotplace - len ( self . _int ) );
        fracpart = "";
        } else {
        intpart = self . _int [ : dotplace ];
        fracpart = "." + self . _int [ dotplace : ];
        if leftdigits == dotplace {
        exp = "";
        } else {
        if context is None /* Option */ {
        context = getcontext ( );
        exp = [ "e" , "E" ] [ context . capitals ] + "%+d" % ( leftdigits - dotplace );
        return  sign + intpart + fracpart + exp;
        pub fn to_eng_string ( &self, context = None /* Option */ )  {
        "Convert to a string, using engineering notation if an exponent == needed.

        Engineering notation has an exponent which == a multiple of 3.  This
        can leave up to 3 digits to the left of the decimal place && may
        require the addition of either one || two trailing zeros.
        ";
        return  self . __str__ ( eng = true , context = context );
        pub fn __neg__ ( &self, context = None /* Option */ )  {
        "Returns a copy with the sign switched.

        Rounds, if it has reason.
        ";
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if context is None /* Option */ {
        context = getcontext ( );
        if !self && context . rounding != ROUND_FLOOR {
        ans = self . copy_abs ( );
        } else {
        ans = self . copy_negate ( );
        return  ans . _fix ( context );
        pub fn __pos__ ( &self, context = None /* Option */ )  {
        "Returns a copy, unless it == a sNaN.

        Rounds the number (if more than precision digits)
        ";
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if context is None /* Option */ {
        context = getcontext ( );
        if !self && context . rounding != ROUND_FLOOR {
        ans = self . copy_abs ( );
        } else {
        ans = Decimal ( self );
        return  ans . _fix ( context );
        pub fn __abs__ ( &self, round = true , context = None /* Option */ )  {
        "Returns the absolute value of self.

        If the keyword argument 'round' == false, do !round.  The
        expression self.__abs__(round=false) == equivalent to
        self.copy_abs().
        ";
        if !round {
        return  self . copy_abs ( );
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if self . _sign {
        ans = self . __neg__ ( context = context );
        } else {
        ans = self . __pos__ ( context = context );
        return  ans;
        pub fn __add__ ( &self, other , context = None /* Option */ )  {
        "Returns self + other.

        -INF + INF (or the reverse) cause InvalidOperation errors.
        ";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) {
        if self . _sign != other . _sign && other . _isinfinity ( ) {
        return  context . _raise_error ( InvalidOperation , "-INF + INF" );
        return  Decimal ( self );
        if other . _isinfinity ( ) {
        return  Decimal ( other );
        exp = min ( self . _exp , other . _exp );
        negativezero = 0;
        if context . rounding == ROUND_FLOOR && self . _sign != other . _sign {
        negativezero = 1;
        if !self && !other {
        sign = min ( self . _sign , other . _sign );
        if negativezero {
        sign = 1;
        ans = _dec_from_triple ( sign , "0" , exp );
        ans = ans . _fix ( context );
        return  ans;
        if !self {
        exp = max ( exp , other . _exp - context . prec -1 );
        ans = other . _rescale ( exp , context . rounding );
        ans = ans . _fix ( context );
        return  ans;
        if !other {
        exp = max ( exp , self . _exp - context . prec -1 );
        ans = self . _rescale ( exp , context . rounding );
        ans = ans . _fix ( context );
        return  ans;
        op1 = _WorkRep ( self );
        op2 = _WorkRep ( other );
        op1 , op2 = _normalize ( op1 , op2 , context . prec );
        result = _WorkRep ( );
        if op1 . sign != op2 . sign {
        if op1 . int == op2 . int {
        ans = _dec_from_triple ( negativezero , "0" , exp );
        ans = ans . _fix ( context );
        return  ans;
        if op1 . int < op2 . int {
        op1 , op2 = op2 , op1;
        if op1 . sign == 1 {
        result . sign = 1;
        op1 . sign , op2 . sign = op2 . sign , op1 . sign;
        } else {
        result . sign = 0;
        } else if op1 . sign == 1 {
        result . sign = 1;
        op1 . sign , op2 . sign = ( 0 , 0 );
        } else {
        result . sign = 0;
        if op2 . sign == 0 {
        result . int = op1 . int + op2 . int;
        } else {
        result . int = op1 . int - op2 . int;
        result . exp = op1 . exp;
        ans = Decimal ( result );
        ans = ans . _fix ( context );
        return  ans;
        __radd__ = __add__;
        pub fn __sub__ ( &self, other , context = None /* Option */ )  {
        "Return self - other";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if self . _is_special || other . _is_special {
        ans = self . _check_nans ( other , context = context );
        if ans {
        return  ans;
        return  self . __add__ ( other . copy_negate ( ) , context = context );
        pub fn __rsub__ ( &self, other , context = None /* Option */ )  {
        "Return other - selformat!(");
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __sub__ ( self , context = context );
        pub fn __mul__ ( &self, other , context = None /* Option */ )  {
        "Return self * other.

        (+-) INF * 0 (or its reverse) raise InvalidOperation.
        ";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        resultsign = self . _sign ^ other . _sign;
        if self . _is_special || other . _is_special {
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) {
        if !other {
        return  context . _raise_error ( InvalidOperation , "(+-)INF * 0" );
        return  _SignedInfinity [ resultsign ];
        if other . _isinfinity ( ) {
        if !self {
        return  context . _raise_error ( InvalidOperation , "0 * (+-)INF" );
        return  _SignedInfinity [ resultsign ];
        resultexp = self . _exp + other . _exp;
        if !self || !other {
        ans = _dec_from_triple ( resultsign , "0" , resultexp );
        ans = ans . _fix ( context );
        return  ans;
        if self . _int == "1" {
        ans = _dec_from_triple ( resultsign , other . _int , resultexp );
        ans = ans . _fix ( context );
        return  ans;
        if other . _int == "1" {
        ans = _dec_from_triple ( resultsign , self . _int , resultexp );
        ans = ans . _fix ( context );
        return  ans;
        op1 = _WorkRep ( self );
        op2 = _WorkRep ( other );
        ans = _dec_from_triple ( resultsign , str ( op1 . int * op2 . int ) , resultexp );
        ans = ans . _fix ( context );
        return  ans;
        __rmul__ = __mul__;
        pub fn __truediv__ ( &self, other , context = None /* Option */ )  {
        "Return self / other.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  NotImplemented;
        if context is None /* Option */ {
        context = getcontext ( );
        sign = self . _sign ^ other . _sign;
        if self . _is_special || other . _is_special {
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) && other . _isinfinity ( ) {
        return  context . _raise_error ( InvalidOperation , "(+-)INF/(+-)INF" );
        if self . _isinfinity ( ) {
        return  _SignedInfinity [ sign ];
        if other . _isinfinity ( ) {
        context . _raise_error ( Clamped , "Division by infinity" );
        return  _dec_from_triple ( sign , "0" , context . Etiny ( ) );
        if !other {
        if !self {
        return  context . _raise_error ( DivisionUndefined , "0 / 0" );
        return  context . _raise_error ( DivisionByZero , "x / 0" , sign );
        if !self {
        exp = self . _exp - other . _exp;
        coeff = 0;
        } else {
        shift = len ( other . _int ) - len ( self . _int ) + context . prec + 1;
        exp = self . _exp - other . _exp - shift;
        op1 = _WorkRep ( self );
        op2 = _WorkRep ( other );
        if shift >= 0 {
        coeff , remainder = divmod ( op1 . int * 10 ** shift , op2 . int );
        } else {
        coeff , remainder = divmod ( op1 . int , op2 . int * 10 ** - shift );
        if remainder {
        if coeff % 5 == 0 {
        coeff + = 1;
        } else {
        ideal_exp = self . _exp - other . _exp;
        while exp < ideal_exp && coeff % 10 == 0  {
        coeff / / = 10;
        exp + = 1;
        ans = _dec_from_triple ( sign , str ( coeff ) , exp );
        return  ans . _fix ( context );
        pub fn _divide ( &self, other , context )  {
        "Return (self // other, self % other), to context.prec precision.

        Assumes that neither self nor other == a NaN, that self == not
        infinite && that other == nonzero.
        ";
        sign = self . _sign ^ other . _sign;
        if other . _isinfinity ( ) {
        ideal_exp = self . _exp;
        } else {
        ideal_exp = min ( self . _exp , other . _exp );
        expdiff = self . adjusted ( ) - other . adjusted ( );
        if !self || other . _isinfinity ( ) || expdiff <= -2 {
        return  ( _dec_from_triple ( sign , "0" , 0 ) ,;
        self . _rescale ( ideal_exp , context . rounding ) );
        if expdiff <= context . prec {
        op1 = _WorkRep ( self );
        op2 = _WorkRep ( other );
        if op1 . exp >= op2 . exp {
        op1 . int * = 10 ** ( op1 . exp - op2 . exp );
        } else {
        op2 . int * = 10 ** ( op2 . exp - op1 . exp );
        q , r = divmod ( op1 . int , op2 . int );
        if q < 10 ** context . prec {
        return  ( _dec_from_triple ( sign , str ( q ) , 0 ) ,;
        _dec_from_triple ( self . _sign , str ( r ) , ideal_exp ) );
        ans = context . _raise_error ( DivisionImpossible ,;
        "quotient too large in //, % || divmod" );
        return  ans , ans;
        pub fn __rtruediv__ ( &self, other , context = None /* Option */ )  {
        "Swaps self/other && returns __truediv__.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __truediv__ ( self , context = context );
        pub fn __divmod__ ( &self, other , context = None /* Option */ )  {
        "
        Return (self // other, self % other)
        ";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ( ans , ans );
        sign = self . _sign ^ other . _sign;
        if self . _isinfinity ( ) {
        if other . _isinfinity ( ) {
        ans = context . _raise_error ( InvalidOperation , "divmod(INF, INF)" );
        return  ans , ans;
        } else {
        return  ( _SignedInfinity [ sign ] ,;
        context . _raise_error ( InvalidOperation , "INF % x" ) );
        if !other {
        if !self {
        ans = context . _raise_error ( DivisionUndefined , "divmod(0, 0)" );
        return  ans , ans;
        } else {
        return  ( context . _raise_error ( DivisionByZero , "x // 0" , sign ) ,;
        context . _raise_error ( InvalidOperation , "x % 0" ) );
        quotient , remainder = self . _divide ( other , context );
        remainder = remainder . _fix ( context );
        return  quotient , remainder;
        pub fn __rdivmod__ ( &self, other , context = None /* Option */ )  {
        "Swaps self/other && returns __divmod__.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __divmod__ ( self , context = context );
        pub fn __mod__ ( &self, other , context = None /* Option */ )  {
        "
        self % other
        ";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) {
        return  context . _raise_error ( InvalidOperation , "INF % x" );
        } else if !other {
        if self {
        return  context . _raise_error ( InvalidOperation , "x % 0" );
        } else {
        return  context . _raise_error ( DivisionUndefined , "0 % 0" );
        remainder = self . _divide ( other , context ) [ 1 ];
        remainder = remainder . _fix ( context );
        return  remainder;
        pub fn __rmod__ ( &self, other , context = None /* Option */ )  {
        "Swaps self/other && returns __mod__.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __mod__ ( self , context = context );
        pub fn remainder_near ( &self, other , context = None /* Option */ )  {
        "
        Remainder nearest to 0-  abs(remainder-near) <= other/2
        ";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) {
        return  context . _raise_error ( InvalidOperation ,;
        "remainder_near(infinity, x)" );
        if !other {
        if self {
        return  context . _raise_error ( InvalidOperation ,;
        "remainder_near(x, 0)" );
        } else {
        return  context . _raise_error ( DivisionUndefined ,;
        "remainder_near(0, 0)" );
        if other . _isinfinity ( ) {
        ans = Decimal ( self );
        return  ans . _fix ( context );
        ideal_exponent = min ( self . _exp , other . _exp );
        if !self {
        ans = _dec_from_triple ( self . _sign , "0" , ideal_exponent );
        return  ans . _fix ( context );
        expdiff = self . adjusted ( ) - other . adjusted ( );
        if expdiff >= context . prec + 1 {
        return  context . _raise_error ( DivisionImpossible );
        if expdiff <= -2 {
        ans = self . _rescale ( ideal_exponent , context . rounding );
        return  ans . _fix ( context );
        op1 = _WorkRep ( self );
        op2 = _WorkRep ( other );
        if op1 . exp >= op2 . exp {
        op1 . int * = 10 ** ( op1 . exp - op2 . exp );
        } else {
        op2 . int * = 10 ** ( op2 . exp - op1 . exp );
        q , r = divmod ( op1 . int , op2 . int );
        if 2 * r + ( q & 1 ) > op2 . int {
        r - = op2 . int;
        q + = 1;
        if q >= 10 ** context . prec {
        return  context . _raise_error ( DivisionImpossible );
        sign = self . _sign;
        if r < 0 {
        sign = 1 - sign;
        r = - r;
        ans = _dec_from_triple ( sign , str ( r ) , ideal_exponent );
        return  ans . _fix ( context );
        pub fn __floordiv__ ( &self, other , context = None /* Option */ )  {
        "self // other";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) {
        if other . _isinfinity ( ) {
        return  context . _raise_error ( InvalidOperation , "INF // INF" );
        } else {
        return  _SignedInfinity [ self . _sign ^ other . _sign ];
        if !other {
        if self {
        return  context . _raise_error ( DivisionByZero , "x // 0" ,;
        self . _sign ^ other . _sign );
        } else {
        return  context . _raise_error ( DivisionUndefined , "0 // 0" );
        return  self . _divide ( other , context ) [ 0 ];
        pub fn __rfloordiv__ ( &self, other , context = None /* Option */ )  {
        "Swaps self/other && returns __floordiv__.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __floordiv__ ( self , context = context );
        pub fn __float__ ( self )  {
        "Float representation.";
        if self . _isnan ( ) {
        if self . is_snan ( ) {
        panic!("ValueError ( "Cannot convert signaling NaN to float" )");
        s = "-nan" if self . _sign else "nan";
        } else {
        s = str ( self );
        return  float ( s );
        pub fn __int__ ( self )  {
        "Converts self to an int, truncating if necessary.";
        if self . _is_special {
        if self . _isnan ( ) {
        panic!("ValueError ( "Cannot convert NaN to integer" )");
        } else if self . _isinfinity ( ) {
        panic!("OverflowError ( "Cannot convert infinity to integer" )");
        s = ( -1 ) ** self . _sign;
        if self . _exp >= 0 {
        return  s * int ( self . _int ) * 10 ** self . _exp;
        } else {
        return  s * int ( self . _int [ : self . _exp ] || "0" );
        __trunc__ = __int__;
        @ property;
        pub fn real ( self )  {
        return  self;
        @ property;
        pub fn imag ( self )  {
        return  Decimal ( 0 );
        pub fn conjugate ( self )  {
        return  self;
        pub fn __complex__ ( self )  {
        return  complex ( float ( self ) );
        pub fn _fix_nan ( &self, context )  {
        "Decapitate the payload of a NaN to fit the context";
        payload = self . _int;
        max_payload_len = context . prec - context . clamp;
        if len ( payload ) > max_payload_len {
        payload = payload [ len ( payload ) - max_payload_len : ] . lstrip ( "0" );
        return  _dec_from_triple ( self . _sign , payload , self . _exp , true );
        return  Decimal ( self );
        pub fn _fix ( &self, context )  {
        "Round if it == necessary to keep self within prec precision.

        Rounds && fixes the exponent.  Does !raise on a sNaN.

        Arguments:
        self - Decimal instance
        context - context used.
        ";
        if self . _is_special {
        if self . _isnan ( ) {
        return  self . _fix_nan ( context );
        } else {
        return  Decimal ( self );
        Etiny = context . Etiny ( );
        Etop = context . Etop ( );
        if !self {
        exp_max = [ context . Emax , Etop ] [ context . clamp ];
        new_exp = min ( max ( self . _exp , Etiny ) , exp_max );
        if new_exp != self . _exp {
        context . _raise_error ( Clamped );
        return  _dec_from_triple ( self . _sign , "0" , new_exp );
        } else {
        return  Decimal ( self );
        exp_min = len ( self . _int ) + self . _exp - context . prec;
        if exp_min > Etop {
        ans = context . _raise_error ( Overflow , "above Emax" , self . _sign );
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        return  ans;
        self_is_subnormal = exp_min < Etiny;
        if self_is_subnormal {
        exp_min = Etiny;
        if self . _exp < exp_min {
        digits = len ( self . _int ) + self . _exp - exp_min;
        if digits < 0 {
        self = _dec_from_triple ( self . _sign , "1" , exp_min -1 );
        digits = 0;
        rounding_method = self . _pick_rounding_function [ context . rounding ];
        changed = rounding_method ( self , digits );
        coeff = self . _int [ : digits ] || "0";
        if changed > 0 {
        coeff = str ( int ( coeff ) + 1 );
        if len ( coeff ) > context . prec {
        coeff = coeff [ : -1 ];
        exp_min + = 1;
        if exp_min > Etop {
        ans = context . _raise_error ( Overflow , "above Emax" , self . _sign );
        } else {
        ans = _dec_from_triple ( self . _sign , coeff , exp_min );
        if changed && self_is_subnormal {
        context . _raise_error ( Underflow );
        if self_is_subnormal {
        context . _raise_error ( Subnormal );
        if changed {
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        if !ans {
        context . _raise_error ( Clamped );
        return  ans;
        if self_is_subnormal {
        context . _raise_error ( Subnormal );
        if context . clamp == 1 && self . _exp > Etop {
        context . _raise_error ( Clamped );
        self_padded = self . _int + "0" * ( self . _exp - Etop );
        return  _dec_from_triple ( self . _sign , self_padded , Etop );
        return  Decimal ( self );
        pub fn _round_down ( &self, prec )  {
        "Also known as round-towards-0, truncate.";
        if _all_zeros ( self . _int , prec ) {
        return  0;
        } else {
        return  -1;
        pub fn _round_up ( &self, prec )  {
        "Rounds away from 0.";
        return  - self . _round_down ( prec );
        pub fn _round_half_up ( &self, prec )  {
        "Rounds 5 up (away from 0)";
        if self . _int [ prec ] in "56789" {
        return  1;
        } else if _all_zeros ( self . _int , prec ) {
        return  0;
        } else {
        return  -1;
        pub fn _round_half_down ( &self, prec )  {
        "Round 5 down";
        if _exact_half ( self . _int , prec ) {
        return  -1;
        } else {
        return  self . _round_half_up ( prec );
        pub fn _round_half_even ( &self, prec )  {
        "Round 5 to even, rest to nearest.";
        if _exact_half ( self . _int , prec ) && \ {
        ( prec == 0 || self . _int [ prec -1 ] in "02468" ) ;
        return  -1;
        } else {
        return  self . _round_half_up ( prec );
        pub fn _round_ceiling ( &self, prec )  {
        "Rounds up (not away from 0 if negative.)";
        if self . _sign {
        return  self . _round_down ( prec );
        } else {
        return  - self . _round_down ( prec );
        pub fn _round_floor ( &self, prec )  {
        "Rounds down (not towards 0 if negative)";
        if !self . _sign {
        return  self . _round_down ( prec );
        } else {
        return  - self . _round_down ( prec );
        pub fn _round_05up ( &self, prec )  {
        "Round down unless digit prec-1 == 0 || 5.";
        if prec && self . _int [ prec -1 ] !in "05" {
        return  self . _round_down ( prec );
        } else {
        return  - self . _round_down ( prec );
        _pick_rounding_function = dict (;
        ROUND_DOWN = _round_down ,;
        ROUND_UP = _round_up ,;
        ROUND_HALF_UP = _round_half_up ,;
        ROUND_HALF_DOWN = _round_half_down ,;
        ROUND_HALF_EVEN = _round_half_even ,;
        ROUND_CEILING = _round_ceiling ,;
        ROUND_FLOOR = _round_floor ,;
        ROUND_05UP = _round_05up ,;
        );
        pub fn __round__ ( &self, n = None /* Option */ )  {
        "Round self to the nearest integer, || to a given precision.

        If only one argument == supplied, round a finite Decimal
        instance self to the nearest integer.  If self == infinite or
        a NaN then a Python exception == raised.  If self == finite
        && lies exactly halfway between two integers then it is
        rounded to the integer with even last digit.

        >>> round(Decimal('123.456'))
        123
        >>> round(Decimal('-456.789'))
        -457
        >>> round(Decimal('-3.0'))
        -3
        >>> round(Decimal('2.5'))
        2
        >>> round(Decimal('3.5'))
        4
        >>> round(Decimal('Inf'))
        Traceback (most recent call last):
          ...
        OverflowError: cannot round an infinity
        >>> round(Decimal('NaN'))
        Traceback (most recent call last):
          ...
        ValueError: cannot round a NaN

        If a second argument n == supplied, self == rounded to n
        decimal places using the rounding mode for the current
        context.

        For an integer n, round(self, -n) == exactly equivalent to
        self.quantize(Decimal('1En')).

        >>> round(Decimal('123.456'), 0)
        Decimal('123')
        >>> round(Decimal('123.456'), 2)
        Decimal('123.46')
        >>> round(Decimal('123.456'), -2)
        Decimal('1E+2')
        >>> round(Decimal('-Infinity'), 37)
        Decimal('NaN')
        >>> round(Decimal('sNaN123'), 0)
        Decimal('NaN123')

        ";
        if n is !None /* Option */ {
        if !isinstance ( n , int ) {
        panic!("TypeError ( "Second argument to round should be integral" )");
        exp = _dec_from_triple ( 0 , "1" , - n );
        return  self . quantize ( exp );
        if self . _is_special {
        if self . is_nan ( ) {
        panic!("ValueError ( "cannot round a NaN" )");
        } else {
        panic!("OverflowError ( "cannot round an infinity" )");
        return  int ( self . _rescale ( 0 , ROUND_HALF_EVEN ) );
        pub fn __floor__ ( self )  {
        "Return the floor of self, as an integer.

        For a finite Decimal instance self, return the greatest
        integer n such that n <= self.  If self == infinite || a NaN
        then a Python exception == raised.

        ";
        if self . _is_special {
        if self . is_nan ( ) {
        panic!("ValueError ( "cannot round a NaN" )");
        } else {
        panic!("OverflowError ( "cannot round an infinity" )");
        return  int ( self . _rescale ( 0 , ROUND_FLOOR ) );
        pub fn __ceil__ ( self )  {
        "Return the ceiling of self, as an integer.

        For a finite Decimal instance self, return the least integer n
        such that n >= self.  If self == infinite || a NaN then a
        Python exception == raised.

        ";
        if self . _is_special {
        if self . is_nan ( ) {
        panic!("ValueError ( "cannot round a NaN" )");
        } else {
        panic!("OverflowError ( "cannot round an infinity" )");
        return  int ( self . _rescale ( 0 , ROUND_CEILING ) );
        pub fn fma ( &self, other , third , context = None /* Option */ )  {
        "Fused multiply-add.

        Returns self*other+third with no rounding of the intermediate
        product self*other.

        self && other are multiplied together, with no rounding of
        the result.  The third operand == then added to the result,
        && a single final rounding == performed.
        ";
        other = _convert_other ( other , raiseit = true );
        third = _convert_other ( third , raiseit = true );
        if self . _is_special || other . _is_special {
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _exp == "N" {
        return  context . _raise_error ( InvalidOperation , "sNaN" , self );
        if other . _exp == "N" {
        return  context . _raise_error ( InvalidOperation , "sNaN" , other );
        if self . _exp == "n" {
        product = self;
        } else if other . _exp == "n" {
        product = other;
        } else if self . _exp == "F" {
        if !other {
        return  context . _raise_error ( InvalidOperation ,;
        "INF * 0 in fma" );
        product = _SignedInfinity [ self . _sign ^ other . _sign ];
        } else if other . _exp == "F" {
        if !self {
        return  context . _raise_error ( InvalidOperation ,;
        "0 * INF in fma" );
        product = _SignedInfinity [ self . _sign ^ other . _sign ];
        } else {
        product = _dec_from_triple ( self . _sign ^ other . _sign ,;
        str ( int ( self . _int ) * int ( other . _int ) ) ,;
        self . _exp + other . _exp );
        return  product . __add__ ( third , context );
        pub fn _power_modulo ( &self, other , modulo , context = None /* Option */ )  {
        "Three argument version of __pow__";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        modulo = _convert_other ( modulo );
        if modulo is NotImplemented {
        return  modulo;
        if context is None /* Option */ {
        context = getcontext ( );
        self_is_nan = self . _isnan ( );
        other_is_nan = other . _isnan ( );
        modulo_is_nan = modulo . _isnan ( );
        if self_is_nan || other_is_nan || modulo_is_nan {
        if self_is_nan == 2 {
        return  context . _raise_error ( InvalidOperation , "sNaN" ,;
        self );
        if other_is_nan == 2 {
        return  context . _raise_error ( InvalidOperation , "sNaN" ,;
        other );
        if modulo_is_nan == 2 {
        return  context . _raise_error ( InvalidOperation , "sNaN" ,;
        modulo );
        if self_is_nan {
        return  self . _fix_nan ( context );
        if other_is_nan {
        return  other . _fix_nan ( context );
        return  modulo . _fix_nan ( context );
        if !( self . _isinteger ( ) and {
        other . _isinteger ( ) and;
        modulo . _isinteger ( ) ) ;
        return  context . _raise_error ( InvalidOperation ,;
        "pow() 3rd argument !allowed ";
        "unless all arguments are integers" );
        if other < 0 {
        return  context . _raise_error ( InvalidOperation ,;
        "pow() 2nd argument cannot be ";
        "negative when 3rd argument specified" );
        if !modulo {
        return  context . _raise_error ( InvalidOperation ,;
        "pow() 3rd argument cannot be 0" );
        if modulo . adjusted ( ) >= context . prec {
        return  context . _raise_error ( InvalidOperation ,;
        "insufficient precision: pow() 3rd ";
        "argument must !have more than ";
        "precision digits" );
        if !other && !self {
        return  context . _raise_error ( InvalidOperation ,;
        "at least one of pow() 1st argument ";
        "and 2nd argument must be nonzero; ";
        "0**0 == !defined" );
        if other . _iseven ( ) {
        sign = 0;
        } else {
        sign = self . _sign;
        modulo = abs ( int ( modulo ) );
        base = _WorkRep ( self . to_integral_value ( ) );
        exponent = _WorkRep ( other . to_integral_value ( ) );
        base = ( base . int % modulo * pow ( 10 , base . exp , modulo ) ) % modulo;
        for i in range ( exponent . exp ) .iter() {
        base = pow ( base , 10 , modulo );
        base = pow ( base , exponent . int , modulo );
        return  _dec_from_triple ( sign , str ( base ) , 0 );
        pub fn _power_exact ( &self, other , p )  {
        "Attempt to compute self**other exactly.

        Given Decimals self && other && an integer p, attempt to
        compute an exact result for the power self**other, with p
        digits of precision.  Return None /* Option */ if self**other == not
        exactly representable in p digits.

        Assumes that elimination of special cases has already been
        performed: self && other must both be nonspecial; self must
        be positive && !numerically equal to 1; other must be
        nonzero.  For efficiency, other._exp should !be too large,
        so that 10**abs(other._exp) == a feasible calculation.";
        x = _WorkRep ( self );
        xc , xe = x . int , x . exp;
        while xc % 10 == 0  {
        xc / / = 10;
        xe + = 1;
        y = _WorkRep ( other );
        yc , ye = y . int , y . exp;
        while yc % 10 == 0  {
        yc / / = 10;
        ye + = 1;
        if xc == 1 {
        xe * = yc;
        while xe % 10 == 0  {
        xe / / = 10;
        ye + = 1;
        if ye < 0 {
        return;
        exponent = xe * 10 ** ye;
        if y . sign == 1 {
        exponent = - exponent;
        if other . _isinteger ( ) && other . _sign == 0 {
        ideal_exponent = self . _exp * int ( other );
        zeros = min ( exponent - ideal_exponent , p -1 );
        } else {
        zeros = 0;
        return  _dec_from_triple ( 0 , "1" + "0" * zeros , exponent - zeros );
        if y . sign == 1 {
        last_digit = xc % 10;
        if last_digit in ( 2 , 4 , 6 , 8 ) {
        if xc & - xc != xc {
        return;
        e = _nbits ( xc ) -1;
        emax = p * 93 / / 65;
        if ye >= len ( str ( emax ) ) {
        return;
        e = _decimal_lshift_exact ( e * yc , ye );
        xe = _decimal_lshift_exact ( xe * yc , ye );
        if e is None /* Option */ || xe is None /* Option */ {
        return;
        if e > emax {
        return;
        xc = 5 ** e;
        } else if last_digit == 5 {
        e = _nbits ( xc ) * 28 / / 65;
        xc , remainder = divmod ( 5 ** e , xc );
        if remainder {
        return;
        while xc % 5 == 0  {
        xc / / = 5;
        e - = 1;
        emax = p * 10 / / 3;
        if ye >= len ( str ( emax ) ) {
        return;
        e = _decimal_lshift_exact ( e * yc , ye );
        xe = _decimal_lshift_exact ( xe * yc , ye );
        if e is None /* Option */ || xe is None /* Option */ {
        return;
        if e > emax {
        return;
        xc = 2 ** e;
        } else {
        return;
        if xc >= 10 ** p {
        return;
        xe = - e - xe;
        return  _dec_from_triple ( 0 , str ( xc ) , xe );
        if ye >= 0 {
        m , n = yc * 10 ** ye , 1;
        } else {
        if xe != 0 && len ( str ( abs ( yc * xe ) ) ) <= - ye {
        return;
        xc_bits = _nbits ( xc );
        if len ( str ( abs ( yc ) * xc_bits ) ) <= - ye {
        return;
        m , n = yc , 10 ** ( - ye );
        while m % 2 == n % 2 == 0  {
        m / / = 2;
        n / / = 2;
        while m % 5 == n % 5 == 0  {
        m / / = 5;
        n / / = 5;
        if n > 1 {
        if xc_bits <= n {
        return;
        xe , rem = divmod ( xe , n );
        if rem != 0 {
        return;
        a = 1 < < - ( - _nbits ( xc ) / / n );
        while true  {
        q , r = divmod ( xc , a ** ( n -1 ) );
        if a <= q {
        break;
        } else {
        a = ( a * ( n -1 ) + q ) / / n;
        if !( a == q && r == 0 ) {
        return;
        xc = a;
        if xc > 1 && m > p * 100 / / _log10_lb ( xc ) {
        return;
        xc = xc ** m;
        xe * = m;
        if xc > 10 ** p {
        return;
        str_xc = str ( xc );
        if other . _isinteger ( ) && other . _sign == 0 {
        ideal_exponent = self . _exp * int ( other );
        zeros = min ( xe - ideal_exponent , p - len ( str_xc ) );
        } else {
        zeros = 0;
        return  _dec_from_triple ( 0 , str_xc + "0" * zeros , xe - zeros );
        pub fn __pow__ ( &self, other , modulo = None /* Option */ , context = None /* Option */ )  {
        "Return self ** other [ % modulo].

        With two arguments, compute self**other.

        With three arguments, compute (self**other) % modulo.  For the
        three argument form, the following restrictions on the
        arguments hold:

         - all three arguments must be integral
         - other must be nonnegative
         - either self || other (or both) must be nonzero
         - modulo must be nonzero && must have at most p digits,
           where p == the context precision.

        If any of these restrictions == violated the InvalidOperation
        flag == raised.

        The result of pow(self, other, modulo) == identical to the
        result that would be obtained by computing (self**other) %
        modulo with unbounded precision, but == computed more
        efficiently.  It == always exact.
        ";
        if modulo is !None /* Option */ {
        return  self . _power_modulo ( other , modulo , context );
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if !other {
        if !self {
        return  context . _raise_error ( InvalidOperation , "0 ** 0" );
        } else {
        return  _One;
        result_sign = 0;
        if self . _sign == 1 {
        if other . _isinteger ( ) {
        if !other . _iseven ( ) {
        result_sign = 1;
        } else {
        if self {
        return  context . _raise_error ( InvalidOperation ,;
        "x ** y with x negative && y !an integer" );
        self = self . copy_negate ( );
        if !self {
        if other . _sign == 0 {
        return  _dec_from_triple ( result_sign , "0" , 0 );
        } else {
        return  _SignedInfinity [ result_sign ];
        if self . _isinfinity ( ) {
        if other . _sign == 0 {
        return  _SignedInfinity [ result_sign ];
        } else {
        return  _dec_from_triple ( result_sign , "0" , 0 );
        if self == _One {
        if other . _isinteger ( ) {
        if other . _sign == 1 {
        multiplier = 0;
        } else if other > context . prec {
        multiplier = context . prec;
        } else {
        multiplier = int ( other );
        exp = self . _exp * multiplier;
        if exp < 1 - context . prec {
        exp = 1 - context . prec;
        context . _raise_error ( Rounded );
        } else {
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        exp = 1 - context . prec;
        return  _dec_from_triple ( result_sign , "1" + "0" * - exp , exp );
        self_adj = self . adjusted ( );
        if other . _isinfinity ( ) {
        if ( other . _sign == 0 ) == ( self_adj < 0 ) {
        return  _dec_from_triple ( result_sign , "0" , 0 );
        } else {
        return  _SignedInfinity [ result_sign ];
        ans = None /* Option */;
        exact = false;
        bound = self . _log10_exp_bound ( ) + other . adjusted ( );
        if ( self_adj >= 0 ) == ( other . _sign == 0 ) {
        if bound >= len ( str ( context . Emax ) ) {
        ans = _dec_from_triple ( result_sign , "1" , context . Emax + 1 );
        } else {
        Etiny = context . Etiny ( );
        if bound >= len ( str ( - Etiny ) ) {
        ans = _dec_from_triple ( result_sign , "1" , Etiny -1 );
        if ans is None /* Option */ {
        ans = self . _power_exact ( other , context . prec + 1 );
        if ans is !None /* Option */ {
        if result_sign == 1 {
        ans = _dec_from_triple ( 1 , ans . _int , ans . _exp );
        exact = true;
        if ans is None /* Option */ {
        p = context . prec;
        x = _WorkRep ( self );
        xc , xe = x . int , x . exp;
        y = _WorkRep ( other );
        yc , ye = y . int , y . exp;
        if y . sign == 1 {
        yc = - yc;
        extra = 3;
        while true  {
        coeff , exp = _dpower ( xc , xe , yc , ye , p + extra );
        if coeff % ( 5 * 10 ** ( len ( str ( coeff ) ) - p -1 ) ) {
        break;
        extra + = 3;
        ans = _dec_from_triple ( result_sign , str ( coeff ) , exp );
        if exact && !other . _isinteger ( ) {
        if len ( ans . _int ) <= context . prec {
        expdiff = context . prec + 1 - len ( ans . _int );
        ans = _dec_from_triple ( ans . _sign , ans . _int + "0" * expdiff ,;
        ans . _exp - expdiff );
        newcontext = context . copy ( );
        newcontext . clear_flags ( );
        for exception in _signals .iter() {
        newcontext . traps [ exception ] = 0;
        ans = ans . _fix ( newcontext );
        newcontext . _raise_error ( Inexact );
        if newcontext . flags [ Subnormal ] {
        newcontext . _raise_error ( Underflow );
        if newcontext . flags [ Overflow ] {
        context . _raise_error ( Overflow , "above Emax" , ans . _sign );
        for exception in Underflow , Subnormal , Inexact , Rounded , Clamped .iter() {
        if newcontext . flags [ exception ] {
        context . _raise_error ( exception );
        } else {
        ans = ans . _fix ( context );
        return  ans;
        pub fn __rpow__ ( &self, other , context = None /* Option */ )  {
        "Swaps self/other && returns __pow__.";
        other = _convert_other ( other );
        if other is NotImplemented {
        return  other;
        return  other . __pow__ ( self , context = context );
        pub fn normalize ( &self, context = None /* Option */ )  {
        "Normalize- strip trailing 0s, change anything equal to 0 to 0e0";
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        dup = self . _fix ( context );
        if dup . _isinfinity ( ) {
        return  dup;
        if !dup {
        return  _dec_from_triple ( dup . _sign , "0" , 0 );
        exp_max = [ context . Emax , context . Etop ( ) ] [ context . clamp ];
        end = len ( dup . _int );
        exp = dup . _exp;
        while dup . _int [ end -1 ] == "0" && exp < exp_max  {
        exp + = 1;
        end - = 1;
        return  _dec_from_triple ( dup . _sign , dup . _int [ : end ] , exp );
        pub fn quantize ( &self, exp , rounding = None /* Option */ , context = None /* Option */ )  {
        "Quantize self so its exponent == the same as that of exp.

        Similar to self._rescale(exp._exp) but with error checking.
        ";
        exp = _convert_other ( exp , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        if rounding is None /* Option */ {
        rounding = context . rounding;
        if self . _is_special || exp . _is_special {
        ans = self . _check_nans ( exp , context );
        if ans {
        return  ans;
        if exp . _isinfinity ( ) || self . _isinfinity ( ) {
        if exp . _isinfinity ( ) && self . _isinfinity ( ) {
        return  Decimal ( self );
        return  context . _raise_error ( InvalidOperation ,;
        "quantize with one INF" );
        if !( context . Etiny ( ) <= exp . _exp <= context . Emax ) {
        return  context . _raise_error ( InvalidOperation ,;
        "target exponent out of bounds in quantize" );
        if !self {
        ans = _dec_from_triple ( self . _sign , "0" , exp . _exp );
        return  ans . _fix ( context );
        self_adjusted = self . adjusted ( );
        if self_adjusted > context . Emax {
        return  context . _raise_error ( InvalidOperation ,;
        "exponent of quantize result too large for current context" );
        if self_adjusted - exp . _exp + 1 > context . prec {
        return  context . _raise_error ( InvalidOperation ,;
        "quantize result has too many digits for current context" );
        ans = self . _rescale ( exp . _exp , rounding );
        if ans . adjusted ( ) > context . Emax {
        return  context . _raise_error ( InvalidOperation ,;
        "exponent of quantize result too large for current context" );
        if len ( ans . _int ) > context . prec {
        return  context . _raise_error ( InvalidOperation ,;
        "quantize result has too many digits for current context" );
        if ans && ans . adjusted ( ) < context . Emin {
        context . _raise_error ( Subnormal );
        if ans . _exp > self . _exp {
        if ans != self {
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        ans = ans . _fix ( context );
        return  ans;
        pub fn same_quantum ( &self, other , context = None /* Option */ )  {
        "Return true if self && other have the same exponent; otherwise
        return false.

        If either operand == a special value, the following rules are used:
           * return true if both operands are infinities
           * return true if both operands are NaNs
           * otherwise, return false.
        ";
        other = _convert_other ( other , raiseit = true );
        if self . _is_special || other . _is_special {
        return  ( self . is_nan ( ) && other . is_nan ( ) or;
        self . is_infinite ( ) && other . is_infinite ( ) );
        return  self . _exp == other . _exp;
        pub fn _rescale ( &self, exp , rounding )  {
        "Rescale self so that the exponent == exp, either by padding with zeros
        || by truncating digits, using the given rounding mode.

        Specials are returned without change.  This operation is
        quiet: it raises no flags, && uses no information from the
        context.

        exp = exp to scale to (an integer)
        rounding = rounding mode
        ";
        if self . _is_special {
        return  Decimal ( self );
        if !self {
        return  _dec_from_triple ( self . _sign , "0" , exp );
        if self . _exp >= exp {
        return  _dec_from_triple ( self . _sign ,;
        self . _int + "0" * ( self . _exp - exp ) , exp );
        digits = len ( self . _int ) + self . _exp - exp;
        if digits < 0 {
        self = _dec_from_triple ( self . _sign , "1" , exp -1 );
        digits = 0;
        this_function = self . _pick_rounding_function [ rounding ];
        changed = this_function ( self , digits );
        coeff = self . _int [ : digits ] || "0";
        if changed == 1 {
        coeff = str ( int ( coeff ) + 1 );
        return  _dec_from_triple ( self . _sign , coeff , exp );
        pub fn _round ( &self, places , rounding )  {
        "Round a nonzero, nonspecial Decimal to a fixed number of
        significant figures, using the given rounding mode.

        Infinities, NaNs && zeros are returned unaltered.

        This operation == quiet: it raises no flags, && uses no
        information from the context.

        ";
        if places <= 0 {
        panic!("ValueError ( "argument should be at least 1 in _round" )");
        if self . _is_special || !self {
        return  Decimal ( self );
        ans = self . _rescale ( self . adjusted ( ) + 1 - places , rounding );
        if ans . adjusted ( ) != self . adjusted ( ) {
        ans = ans . _rescale ( ans . adjusted ( ) + 1 - places , rounding );
        return  ans;
        pub fn to_integral_exact ( &self, rounding = None /* Option */ , context = None /* Option */ )  {
        "Rounds to a nearby integer.

        If no rounding mode == specified, take the rounding mode from
        the context.  This method raises the Rounded && Inexact flags
        when appropriate.

        See also: to_integral_value, which does exactly the same as
        this method except that it doesn't raise Inexact || Rounded.
        ";
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        return  Decimal ( self );
        if self . _exp >= 0 {
        return  Decimal ( self );
        if !self {
        return  _dec_from_triple ( self . _sign , "0" , 0 );
        if context is None /* Option */ {
        context = getcontext ( );
        if rounding is None /* Option */ {
        rounding = context . rounding;
        ans = self . _rescale ( 0 , rounding );
        if ans != self {
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        return  ans;
        pub fn to_integral_value ( &self, rounding = None /* Option */ , context = None /* Option */ )  {
        "Rounds to the nearest integer, without raising inexact, rounded.";
        if context is None /* Option */ {
        context = getcontext ( );
        if rounding is None /* Option */ {
        rounding = context . rounding;
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        return  Decimal ( self );
        if self . _exp >= 0 {
        return  Decimal ( self );
        } else {
        return  self . _rescale ( 0 , rounding );
        to_integral = to_integral_value;
        pub fn sqrt ( &self, context = None /* Option */ )  {
        "Return the square root of self.";
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special {
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) && self . _sign == 0 {
        return  Decimal ( self );
        if !self {
        ans = _dec_from_triple ( self . _sign , "0" , self . _exp / / 2 );
        return  ans . _fix ( context );
        if self . _sign == 1 {
        return  context . _raise_error ( InvalidOperation , "sqrt(-x), x > 0" );
        prec = context . prec + 1;
        op = _WorkRep ( self );
        e = op . exp > > 1;
        if op . exp & 1 {
        c = op . int * 10;
        l = ( len ( self . _int ) > > 1 ) + 1;
        } else {
        c = op . int;
        l = len ( self . _int ) + 1 > > 1;
        shift = prec - l;
        if shift >= 0 {
        c * = 100 ** shift;
        exact = true;
        } else {
        c , remainder = divmod ( c , 100 ** - shift );
        exact = !remainder;
        e - = shift;
        n = 10 ** prec;
        while true  {
        q = c / / n;
        if n <= q {
        break;
        } else {
        n = n + q > > 1;
        exact = exact && n * n == c;
        if exact {
        if shift >= 0 {
        n / / = 10 ** shift;
        } else {
        n * = 10 ** - shift;
        e + = shift;
        } else {
        if n % 5 == 0 {
        n + = 1;
        ans = _dec_from_triple ( 0 , str ( n ) , e );
        context = context . _shallow_copy ( );
        rounding = context . _set_rounding ( ROUND_HALF_EVEN );
        ans = ans . _fix ( context );
        context . rounding = rounding;
        return  ans;
        pub fn max ( &self, other , context = None /* Option */ )  {
        "Returns the larger value.

        Like max(self, other) except if one == !a number, returns
        NaN (and signals if one == sNaN).  Also rounds.
        ";
        other = _convert_other ( other , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        sn = self . _isnan ( );
        on = other . _isnan ( );
        if sn || on {
        if on == 1 && sn == 0 {
        return  self . _fix ( context );
        if sn == 1 && on == 0 {
        return  other . _fix ( context );
        return  self . _check_nans ( other , context );
        c = self . _cmp ( other );
        if c == 0 {
        c = self . compare_total ( other );
        if c == -1 {
        ans = other;
        } else {
        ans = self;
        return  ans . _fix ( context );
        pub fn min ( &self, other , context = None /* Option */ )  {
        "Returns the smaller value.

        Like min(self, other) except if one == !a number, returns
        NaN (and signals if one == sNaN).  Also rounds.
        ";
        other = _convert_other ( other , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        sn = self . _isnan ( );
        on = other . _isnan ( );
        if sn || on {
        if on == 1 && sn == 0 {
        return  self . _fix ( context );
        if sn == 1 && on == 0 {
        return  other . _fix ( context );
        return  self . _check_nans ( other , context );
        c = self . _cmp ( other );
        if c == 0 {
        c = self . compare_total ( other );
        if c == -1 {
        ans = self;
        } else {
        ans = other;
        return  ans . _fix ( context );
        pub fn _isinteger ( self )  {
        "Returns whether self == an integer";
        if self . _is_special {
        return  false;
        if self . _exp >= 0 {
        return  true;
        rest = self . _int [ self . _exp : ];
        return  rest == "0" * len ( rest );
        pub fn _iseven ( self )  {
        "Returns true if self == even.  Assumes self == an integer.";
        if !self || self . _exp > 0 {
        return  true;
        return  self . _int [ -1 + self . _exp ] in "02468";
        pub fn adjusted ( self )  {
        "Return the adjusted exponent of selformat!(");
        // try {
        return  self . _exp + len ( self . _int ) - 1;
        // } catch  TypeError  {
        return  0;
        pub fn canonical ( self )  {
        "Returns the same Decimal object.

        As we do !have different encodings for the same number, the
        received object already == in its canonical form.
        ";
        return  self;
        pub fn compare_signal ( &self, other , context = None /* Option */ )  {
        "Compares self to the other operand numerically.

        It's pretty much like compare(), but all NaNs signal, with signaling
        NaNs taking precedence over quiet NaNs.
        ";
        other = _convert_other ( other , raiseit = true );
        ans = self . _compare_check_nans ( other , context );
        if ans {
        return  ans;
        return  self . compare ( other , context = context );
        pub fn compare_total ( &self, other , context = None /* Option */ )  {
        "Compares self to other using the abstract representations.

        This == !like the standard compare, which use their numerical
        value. Note that a total ordering == defined for all possible abstract
        representations.
        ";
        other = _convert_other ( other , raiseit = true );
        if self . _sign && !other . _sign {
        return  _NegativeOne;
        if !self . _sign && other . _sign {
        return  _One;
        sign = self . _sign;
        self_nan = self . _isnan ( );
        other_nan = other . _isnan ( );
        if self_nan || other_nan {
        if self_nan == other_nan {
        self_key = len ( self . _int ) , self . _int;
        other_key = len ( other . _int ) , other . _int;
        if self_key < other_key {
        if sign {
        return  _One;
        } else {
        return  _NegativeOne;
        if self_key > other_key {
        if sign {
        return  _NegativeOne;
        } else {
        return  _One;
        return  _Zero;
        if sign {
        if self_nan == 1 {
        return  _NegativeOne;
        if other_nan == 1 {
        return  _One;
        if self_nan == 2 {
        return  _NegativeOne;
        if other_nan == 2 {
        return  _One;
        } else {
        if self_nan == 1 {
        return  _One;
        if other_nan == 1 {
        return  _NegativeOne;
        if self_nan == 2 {
        return  _One;
        if other_nan == 2 {
        return  _NegativeOne;
        if self < other {
        return  _NegativeOne;
        if self > other {
        return  _One;
        if self . _exp < other . _exp {
        if sign {
        return  _One;
        } else {
        return  _NegativeOne;
        if self . _exp > other . _exp {
        if sign {
        return  _NegativeOne;
        } else {
        return  _One;
        return  _Zero;
        pub fn compare_total_mag ( &self, other , context = None /* Option */ )  {
        "Compares self to other using abstract repr., ignoring sign.

        Like compare_total, but with operand's sign ignored && assumed to be 0.
        ";
        other = _convert_other ( other , raiseit = true );
        s = self . copy_abs ( );
        o = other . copy_abs ( );
        return  s . compare_total ( o );
        pub fn copy_abs ( self )  {
        "Returns a copy with the sign set to 0. ";
        return  _dec_from_triple ( 0 , self . _int , self . _exp , self . _is_special );
        pub fn copy_negate ( self )  {
        "Returns a copy with the sign inverted.";
        if self . _sign {
        return  _dec_from_triple ( 0 , self . _int , self . _exp , self . _is_special );
        } else {
        return  _dec_from_triple ( 1 , self . _int , self . _exp , self . _is_special );
        pub fn copy_sign ( &self, other , context = None /* Option */ )  {
        "Returns self with the sign of other.";
        other = _convert_other ( other , raiseit = true );
        return  _dec_from_triple ( other . _sign , self . _int ,;
        self . _exp , self . _is_special );
        pub fn exp ( &self, context = None /* Option */ )  {
        "Returns e ** self.";
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) == -1 {
        return  _Zero;
        if !self {
        return  _One;
        if self . _isinfinity ( ) == 1 {
        return  Decimal ( self );
        p = context . prec;
        adj = self . adjusted ( );
        if self . _sign == 0 && adj > len ( str ( ( context . Emax + 1 ) * 3 ) ) {
        ans = _dec_from_triple ( 0 , "1" , context . Emax + 1 );
        } else if self . _sign == 1 && adj > len ( str ( ( - context . Etiny ( ) + 1 ) * 3 ) ) {
        ans = _dec_from_triple ( 0 , "1" , context . Etiny ( ) -1 );
        } else if self . _sign == 0 && adj < - p {
        ans = _dec_from_triple ( 0 , "1" + "0" * ( p -1 ) + "1" , - p );
        } else if self . _sign == 1 && adj < - p -1 {
        ans = _dec_from_triple ( 0 , "9" * ( p + 1 ) , - p -1 );
        } else {
        op = _WorkRep ( self );
        c , e = op . int , op . exp;
        if op . sign == 1 {
        c = - c;
        extra = 3;
        while true  {
        coeff , exp = _dexp ( c , e , p + extra );
        if coeff % ( 5 * 10 ** ( len ( str ( coeff ) ) - p -1 ) ) {
        break;
        extra + = 3;
        ans = _dec_from_triple ( 0 , str ( coeff ) , exp );
        context = context . _shallow_copy ( );
        rounding = context . _set_rounding ( ROUND_HALF_EVEN );
        ans = ans . _fix ( context );
        context . rounding = rounding;
        return  ans;
        pub fn is_canonical ( self )  {
        "Return true if self == canonical; otherwise return false.

        Currently, the encoding of a Decimal instance == always
        canonical, so this method returns true for any Decimal.
        ";
        return  true;
        pub fn is_finite ( self )  {
        "Return true if self == finite; otherwise return false.

        A Decimal instance == considered finite if it == neither
        infinite nor a NaN.
        ";
        return  !self . _is_special;
        pub fn is_infinite ( self )  {
        "Return true if self == infinite; otherwise return false.";
        return  self . _exp == "F";
        pub fn is_nan ( self )  {
        "Return true if self == a qNaN || sNaN; otherwise return false.";
        return  self . _exp in ( "n" , "N" );
        pub fn is_normal ( &self, context = None /* Option */ )  {
        "Return true if self == a normal number; otherwise return false.";
        if self . _is_special || !self {
        return  false;
        if context is None /* Option */ {
        context = getcontext ( );
        return  context . Emin <= self . adjusted ( );
        pub fn is_qnan ( self )  {
        "Return true if self == a quiet NaN; otherwise return false.";
        return  self . _exp == "n";
        pub fn is_signed ( self )  {
        "Return true if self == negative; otherwise return false.";
        return  self . _sign == 1;
        pub fn is_snan ( self )  {
        "Return true if self == a signaling NaN; otherwise return false.";
        return  self . _exp == "N";
        pub fn is_subnormal ( &self, context = None /* Option */ )  {
        "Return true if self == subnormal; otherwise return false.";
        if self . _is_special || !self {
        return  false;
        if context is None /* Option */ {
        context = getcontext ( );
        return  self . adjusted ( ) < context . Emin;
        pub fn is_zero ( self )  {
        "Return true if self == a zero; otherwise return false.";
        return  !self . _is_special && self . _int == "0";
        pub fn _ln_exp_bound ( self )  {
        "Compute a lower bound for the adjusted exponent of self.ln().
        In other words, compute r such that self.ln() >= 10**r.  Assumes
        that self == finite && positive && that self != 1.
        ";
        adj = self . _exp + len ( self . _int ) - 1;
        if adj >= 1 {
        return  len ( str ( adj * 23 / / 10 ) ) - 1;
        if adj <= -2 {
        return  len ( str ( ( -1 - adj ) * 23 / / 10 ) ) - 1;
        op = _WorkRep ( self );
        c , e = op . int , op . exp;
        if adj == 0 {
        num = str ( c -10 ** - e );
        den = str ( c );
        return  len ( num ) - len ( den ) - ( num < den );
        return  e + len ( str ( 10 ** - e - c ) ) - 1;
        pub fn ln ( &self, context = None /* Option */ )  {
        "Returns the natural (base e) logarithm of self.";
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if !self {
        return  _NegativeInfinity;
        if self . _isinfinity ( ) == 1 {
        return  _Infinity;
        if self == _One {
        return  _Zero;
        if self . _sign == 1 {
        return  context . _raise_error ( InvalidOperation ,;
        "ln of a negative value" );
        op = _WorkRep ( self );
        c , e = op . int , op . exp;
        p = context . prec;
        places = p - self . _ln_exp_bound ( ) + 2;
        while true  {
        coeff = _dlog ( c , e , places );
        if coeff % ( 5 * 10 ** ( len ( str ( abs ( coeff ) ) ) - p -1 ) ) {
        break;
        places + = 3;
        ans = _dec_from_triple ( int ( coeff < 0 ) , str ( abs ( coeff ) ) , - places );
        context = context . _shallow_copy ( );
        rounding = context . _set_rounding ( ROUND_HALF_EVEN );
        ans = ans . _fix ( context );
        context . rounding = rounding;
        return  ans;
        pub fn _log10_exp_bound ( self )  {
        "Compute a lower bound for the adjusted exponent of self.log10().
        In other words, find r such that self.log10() >= 10**r.
        Assumes that self == finite && positive && that self != 1.
        ";
        adj = self . _exp + len ( self . _int ) - 1;
        if adj >= 1 {
        return  len ( str ( adj ) ) -1;
        if adj <= -2 {
        return  len ( str ( -1 - adj ) ) -1;
        op = _WorkRep ( self );
        c , e = op . int , op . exp;
        if adj == 0 {
        num = str ( c -10 ** - e );
        den = str ( 231 * c );
        return  len ( num ) - len ( den ) - ( num < den ) + 2;
        num = str ( 10 ** - e - c );
        return  len ( num ) + e - ( num < "231" ) - 1;
        pub fn log10 ( &self, context = None /* Option */ )  {
        "Returns the base 10 logarithm of self.";
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if !self {
        return  _NegativeInfinity;
        if self . _isinfinity ( ) == 1 {
        return  _Infinity;
        if self . _sign == 1 {
        return  context . _raise_error ( InvalidOperation ,;
        "log10 of a negative value" );
        if self . _int [ 0 ] == "1" && self . _int [ 1 { : ] == "0" * ( len ( self . _int ) - 1 ) ; }
        ans = Decimal ( self . _exp + len ( self . _int ) - 1 );
        } else {
        op = _WorkRep ( self );
        c , e = op . int , op . exp;
        p = context . prec;
        places = p - self . _log10_exp_bound ( ) + 2;
        while true  {
        coeff = _dlog10 ( c , e , places );
        if coeff % ( 5 * 10 ** ( len ( str ( abs ( coeff ) ) ) - p -1 ) ) {
        break;
        places + = 3;
        ans = _dec_from_triple ( int ( coeff < 0 ) , str ( abs ( coeff ) ) , - places );
        context = context . _shallow_copy ( );
        rounding = context . _set_rounding ( ROUND_HALF_EVEN );
        ans = ans . _fix ( context );
        context . rounding = rounding;
        return  ans;
        pub fn logb ( &self, context = None /* Option */ )  {
        " Returns the exponent of the magnitude of self's MSD.

        The result == the integer which == the exponent of the magnitude
        of the most significant digit of self (as though it were truncated
        to a single digit while maintaining the value of that digit and
        without limiting the resulting exponent).
        ";
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _isinfinity ( ) {
        return  _Infinity;
        if !self {
        return  context . _raise_error ( DivisionByZero , "logb(0)" , 1 );
        ans = Decimal ( self . adjusted ( ) );
        return  ans . _fix ( context );
        pub fn _islogical ( self )  {
        "Return true if self == a logical operand.

        For being logical, it must be a finite number with a sign of 0,
        an exponent of 0, && a coefficient whose digits must all be
        either 0 || 1.
        ";
        if self . _sign != 0 || self . _exp != 0 {
        return  false;
        for dig in self . _int .iter() {
        if dig !in "01" {
        return  false;
        return  true;
        pub fn _fill_logical ( &self, context , opa , opb )  {
        dif = context . prec - len ( opa );
        if dif > 0 {
        opa = "0" * dif + opa;
        } else if dif < 0 {
        opa = opa [ - context . prec : ];
        dif = context . prec - len ( opb );
        if dif > 0 {
        opb = "0" * dif + opb;
        } else if dif < 0 {
        opb = opb [ - context . prec : ];
        return  opa , opb;
        pub fn logical_and ( &self, other , context = None /* Option */ )  {
        "Applies an 'and' operation between self && other's digits.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        if !self . _islogical ( ) || !other . _islogical ( ) {
        return  context . _raise_error ( InvalidOperation );
        ( opa , opb ) = self . _fill_logical ( context , self . _int , other . _int );
        result = "" . join ( vec![ str ( int ( a ) & int ( b ) ).iter().map(|a , b| zip ( opa , opb ) ] );
        return  _dec_from_triple ( 0 , result . lstrip ( "0" ) || "0" , 0 );
        pub fn logical_invert ( &self, context = None /* Option */ )  {
        "Invert all its digits.";
        if context is None /* Option */ {
        context = getcontext ( );
        return  self . logical_xor ( _dec_from_triple ( 0 , "1" * context . prec , 0 ) ,;
        context );
        pub fn logical_or ( &self, other , context = None /* Option */ )  {
        "Applies an 'or' operation between self && other's digits.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        if !self . _islogical ( ) || !other . _islogical ( ) {
        return  context . _raise_error ( InvalidOperation );
        ( opa , opb ) = self . _fill_logical ( context , self . _int , other . _int );
        result = "" . join ( vec![ str ( int ( a ) | int ( b ) ).iter().map(|a , b| zip ( opa , opb ) ] );
        return  _dec_from_triple ( 0 , result . lstrip ( "0" ) || "0" , 0 );
        pub fn logical_xor ( &self, other , context = None /* Option */ )  {
        "Applies an 'xor' operation between self && other's digits.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        if !self . _islogical ( ) || !other . _islogical ( ) {
        return  context . _raise_error ( InvalidOperation );
        ( opa , opb ) = self . _fill_logical ( context , self . _int , other . _int );
        result = "" . join ( vec![ str ( int ( a ) ^ int ( b ) ).iter().map(|a , b| zip ( opa , opb ) ] );
        return  _dec_from_triple ( 0 , result . lstrip ( "0" ) || "0" , 0 );
        pub fn max_mag ( &self, other , context = None /* Option */ )  {
        "Compares the values numerically with their sign ignored.";
        other = _convert_other ( other , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        sn = self . _isnan ( );
        on = other . _isnan ( );
        if sn || on {
        if on == 1 && sn == 0 {
        return  self . _fix ( context );
        if sn == 1 && on == 0 {
        return  other . _fix ( context );
        return  self . _check_nans ( other , context );
        c = self . copy_abs ( ) . _cmp ( other . copy_abs ( ) );
        if c == 0 {
        c = self . compare_total ( other );
        if c == -1 {
        ans = other;
        } else {
        ans = self;
        return  ans . _fix ( context );
        pub fn min_mag ( &self, other , context = None /* Option */ )  {
        "Compares the values numerically with their sign ignored.";
        other = _convert_other ( other , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        if self . _is_special || other . _is_special {
        sn = self . _isnan ( );
        on = other . _isnan ( );
        if sn || on {
        if on == 1 && sn == 0 {
        return  self . _fix ( context );
        if sn == 1 && on == 0 {
        return  other . _fix ( context );
        return  self . _check_nans ( other , context );
        c = self . copy_abs ( ) . _cmp ( other . copy_abs ( ) );
        if c == 0 {
        c = self . compare_total ( other );
        if c == -1 {
        ans = self;
        } else {
        ans = other;
        return  ans . _fix ( context );
        pub fn next_minus ( &self, context = None /* Option */ )  {
        "Returns the largest representable number smaller than itself.";
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) == -1 {
        return  _NegativeInfinity;
        if self . _isinfinity ( ) == 1 {
        return  _dec_from_triple ( 0 , "9" * context . prec , context . Etop ( ) );
        context = context . copy ( );
        context . _set_rounding ( ROUND_FLOOR );
        context . _ignore_all_flags ( );
        new_self = self . _fix ( context );
        if new_self != self {
        return  new_self;
        return  self . __sub__ ( _dec_from_triple ( 0 , "1" , context . Etiny ( ) -1 ) ,;
        context );
        pub fn next_plus ( &self, context = None /* Option */ )  {
        "Returns the smallest representable number larger than itself.";
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( context = context );
        if ans {
        return  ans;
        if self . _isinfinity ( ) == 1 {
        return  _Infinity;
        if self . _isinfinity ( ) == -1 {
        return  _dec_from_triple ( 1 , "9" * context . prec , context . Etop ( ) );
        context = context . copy ( );
        context . _set_rounding ( ROUND_CEILING );
        context . _ignore_all_flags ( );
        new_self = self . _fix ( context );
        if new_self != self {
        return  new_self;
        return  self . __add__ ( _dec_from_triple ( 0 , "1" , context . Etiny ( ) -1 ) ,;
        context );
        pub fn next_toward ( &self, other , context = None /* Option */ )  {
        "Returns the number closest to self, in the direction towards other.

        The result == the closest representable number to self
        (excluding self) that == in the direction towards other,
        unless both have the same value.  If the two operands are
        numerically equal, then the result == a copy of self with the
        sign set to be the same as the sign of other.
        ";
        other = _convert_other ( other , raiseit = true );
        if context is None /* Option */ {
        context = getcontext ( );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        comparison = self . _cmp ( other );
        if comparison == 0 {
        return  self . copy_sign ( other );
        if comparison == -1 {
        ans = self . next_plus ( context );
        } else {
        ans = self . next_minus ( context );
        if ans . _isinfinity ( ) {
        context . _raise_error ( Overflow ,;
        "Infinite result from next_toward" ,;
        ans . _sign );
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        } else if ans . adjusted ( ) < context . Emin {
        context . _raise_error ( Underflow );
        context . _raise_error ( Subnormal );
        context . _raise_error ( Inexact );
        context . _raise_error ( Rounded );
        if !ans {
        context . _raise_error ( Clamped );
        return  ans;
        pub fn number_class ( &self, context = None /* Option */ )  {
        "Returns an indication of the class of self.

        The class == one of the following strings:
          sNaN
          NaN
          -Infinity
          -Normal
          -Subnormal
          -Zero
          +Zero
          +Subnormal
          +Normal
          +Infinity
        ";
        if self . is_snan ( ) {
        return  "sNaN";
        if self . is_qnan ( ) {
        return  "NaN";
        inf = self . _isinfinity ( );
        if inf == 1 {
        return  "+Infinity";
        if inf == -1 {
        return  "-Infinity";
        if self . is_zero ( ) {
        if self . _sign {
        return  "-Zero";
        } else {
        return  "+Zero";
        if context is None /* Option */ {
        context = getcontext ( );
        if self . is_subnormal ( context = context ) {
        if self . _sign {
        return  "-Subnormal";
        } else {
        return  "+Subnormal";
        if self . _sign {
        return  "-Normal";
        } else {
        return  "+Normal";
        pub fn radix ( self )  {
        "Just returns 10, as this == Decimal, :)";
        return  Decimal ( 10 );
        pub fn rotate ( &self, other , context = None /* Option */ )  {
        "Returns a rotated copy of self, value-of-other times.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if other . _exp != 0 {
        return  context . _raise_error ( InvalidOperation );
        if !( - context . prec <= int ( other ) <= context . prec ) {
        return  context . _raise_error ( InvalidOperation );
        if self . _isinfinity ( ) {
        return  Decimal ( self );
        torot = int ( other );
        rotdig = self . _int;
        topad = context . prec - len ( rotdig );
        if topad > 0 {
        rotdig = "0" * topad + rotdig;
        } else if topad < 0 {
        rotdig = rotdig [ - topad : ];
        rotated = rotdig [ torot : ] + rotdig [ : torot ];
        return  _dec_from_triple ( self . _sign ,;
        rotated . lstrip ( "0" ) || "0" , self . _exp );
        pub fn scaleb ( &self, other , context = None /* Option */ )  {
        "Returns self operand after adding the second value to its exp.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if other . _exp != 0 {
        return  context . _raise_error ( InvalidOperation );
        liminf = -2 * ( context . Emax + context . prec );
        limsup = 2 * ( context . Emax + context . prec );
        if !( liminf <= int ( other ) <= limsup ) {
        return  context . _raise_error ( InvalidOperation );
        if self . _isinfinity ( ) {
        return  Decimal ( self );
        d = _dec_from_triple ( self . _sign , self . _int , self . _exp + int ( other ) );
        d = d . _fix ( context );
        return  d;
        pub fn shift ( &self, other , context = None /* Option */ )  {
        "Returns a shifted copy of self, value-of-other times.";
        if context is None /* Option */ {
        context = getcontext ( );
        other = _convert_other ( other , raiseit = true );
        ans = self . _check_nans ( other , context );
        if ans {
        return  ans;
        if other . _exp != 0 {
        return  context . _raise_error ( InvalidOperation );
        if !( - context . prec <= int ( other ) <= context . prec ) {
        return  context . _raise_error ( InvalidOperation );
        if self . _isinfinity ( ) {
        return  Decimal ( self );
        torot = int ( other );
        rotdig = self . _int;
        topad = context . prec - len ( rotdig );
        if topad > 0 {
        rotdig = "0" * topad + rotdig;
        } else if topad < 0 {
        rotdig = rotdig [ - topad : ];
        if torot < 0 {
        shifted = rotdig [ : torot ];
        } else {
        shifted = rotdig + "0" * torot;
        shifted = shifted [ - context . prec : ];
        return  _dec_from_triple ( self . _sign ,;
        shifted . lstrip ( "0" ) || "0" , self . _exp );
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ , ( str ( self ) , ) );
        pub fn __copy__ ( self )  {
        if type ( self ) is Decimal {
        return  self;
        return  self . __class__ ( str ( self ) );
        pub fn __deepcopy__ ( &self, memo )  {
        if type ( self ) is Decimal {
        return  self;
        return  self . __class__ ( str ( self ) );
        pub fn __format__ ( &self, specifier , context = None /* Option */ , _localeconv = None /* Option */ )  {
        "Format a Decimal instance according to the given specifier.

        The specifier should be a standard format specifier, with the
        form described in PEP 3101.  Formatting types 'e', 'E', 'f',
        'F', 'g', 'G', 'n' && '%' are supported.  If the formatting
        type == omitted it defaults to 'g' || 'G', depending on the
        value of context.capitals.
        ";
        if context is None /* Option */ {
        context = getcontext ( );
        spec = _parse_format_specifier ( specifier , _localeconv = _localeconv );
        if self . _is_special {
        sign = _format_sign ( self . _sign , spec );
        body = str ( self . copy_abs ( ) );
        if spec [ "type" ] == "%" {
        body + = "%";
        return  _format_align ( sign , body , spec );
        if spec [ "type" ] is None /* Option */ {
        spec [ "type" ] = [ "g" , "G" ] [ context . capitals ];
        if spec [ "type" ] == "%" {
        self = _dec_from_triple ( self . _sign , self . _int , self . _exp + 2 );
        rounding = context . rounding;
        precision = spec [ "precision" ];
        if precision is !None /* Option */ {
        if spec [ "type" ] in "eE" {
        self = self . _round ( precision + 1 , rounding );
        } else if spec [ "type" ] in "fF%" {
        self = self . _rescale ( - precision , rounding );
        } else if spec [ "type" ] in "gG" && len ( self . _int ) > precision {
        self = self . _round ( precision , rounding );
        if !self && self . _exp > 0 && spec [ "type" ] in "fF%" {
        self = self . _rescale ( 0 , rounding );
        if !self && spec [ "no_neg_0" ] && self . _sign {
        adjusted_sign = 0;
        } else {
        adjusted_sign = self . _sign;
        leftdigits = self . _exp + len ( self . _int );
        if spec [ "type" ] in "eE" {
        if !self && precision is !None /* Option */ {
        dotplace = 1 - precision;
        } else {
        dotplace = 1;
        } else if spec [ "type" ] in "fF%" {
        dotplace = leftdigits;
        } else if spec [ "type" ] in "gG" {
        if self . _exp <= 0 && leftdigits > -6 {
        dotplace = leftdigits;
        } else {
        dotplace = 1;
        if dotplace < 0 {
        intpart = "0";
        fracpart = "0" * ( - dotplace ) + self . _int;
        } else if dotplace > len ( self . _int ) {
        intpart = self . _int + "0" * ( dotplace - len ( self . _int ) );
        fracpart = "";
        } else {
        intpart = self . _int [ : dotplace ] || "0";
        fracpart = self . _int [ dotplace : ];
        exp = leftdigits - dotplace;
        return  _format_number ( adjusted_sign , intpart , fracpart , exp , spec );
        pub fn _dec_from_triple ( sign , coefficient , exponent , special = false )  {
        "Create a decimal instance directly, without any validation,
    normalization (e.g. removal of leading zeros) || argument
    conversion.

    This function == for *internal use only*.
    ";
        self = object . __new__ ( Decimal );
        self . _sign = sign;
        self . _int = coefficient;
        self . _exp = exponent;
        self . _is_special = special;
        return  self;
        _numbers . Number . register ( Decimal );
        class _ContextManager ( object ) ;
        "Context manager class to support localcontext().

      Sets a copy of the supplied context in __enter__() && restores
      the previous decimal context in __exit__()
    ";
        pub fn __init__ ( &self, new_context )  {
        self . new_context = new_context . copy ( );
        pub fn __enter__ ( self )  {
        self . saved_context = getcontext ( );
        setcontext ( self . new_context );
        return  self . new_context;
        pub fn __exit__ ( &self, t , v , tb )  {
        setcontext ( self . saved_context );
        class Context ( object ) ;
        "Contains the context.iter().map(|a Decimal instance.

    Contains:
    prec - precision (for use| rounding, division, square roots..)
    rounding - rounding type (how you round)
    traps - If trapsvec![exception] = 1, then the exception is
                    raised when it == caused.  Otherwise, a value is
                    substituted in.
    flags  - When an exception == caused, flagsvec![exception] == set.
             (Whether || !the trap_enabler == set)
             Should be reset by user of Decimal instance.
    Emin -   Minimum exponent
    Emax -   Maximum exponent
    capitals -      If 1, 1*10^1 == printed as 1E+1.
                    If 0, printed as 1e1
    clamp -  If 1, change exponents if too high (Default 0)
    ";
        pub fn __init__ ( &self, prec = None /* Option */ , rounding = None /* Option */ , Emin = None /* Option */ , Emax = None /* Option */ , {
        capitals = None /* Option */ , clamp = None /* Option */ , flags = None /* Option */ , traps = None /* Option */ ,;
        _ignored_flags = None /* Option */ ) ;
        // try {
        dc = DefaultContext;
        // } catch  NameError  {
        // pass
        self . prec = prec if prec is !None /* Option */ else dc . prec;
        self . rounding = rounding if rounding is !None /* Option */ else dc . rounding;
        self . Emin = Emin if Emin is !None /* Option */ else dc . Emin;
        self . Emax = Emax if Emax is !None /* Option */ else dc . Emax;
        self . capitals = capitals if capitals is !None /* Option */ else dc . capitals;
        self . clamp = clamp if clamp is !None /* Option */ else dc . clamp;
        if _ignored_flags is None /* Option */ {
        self . _ignored_flags = [ ];
        } else {
        self . _ignored_flags = _ignored_flags;
        if traps is None /* Option */ {
        self . traps = dc . traps . copy ( );
        } else if !isinstance ( traps , dict ) {
        self . traps = dict ( ( s , int ( s in traps ) ) for s in _signals + traps );
        } else {
        self . traps = traps;
        if flags is None /* Option */ {
        self . flags = dict . fromkeys ( _signals , 0 );
        } else if !isinstance ( flags , dict ) {
        self . flags = dict ( ( s , int ( s in flags ) ) for s in _signals + flags );
        } else {
        self . flags = flags;
        pub fn _set_integer_check ( &self, name , value , vmin , vmax )  {
        if !isinstance ( value , int ) {
        panic!("TypeError ( "%s must be an integer" % name )");
        if vmin == "-inf" {
        if value > vmax {
        panic!("ValueError ( "%s must be in [%s, %d]. got: %s" % ( name , vmin , vmax , value ) )");
        } else if vmax == "inf" {
        if value < vmin {
        panic!("ValueError ( "%s must be in [%d, %s]. got: %s" % ( name , vmin , vmax , value ) )");
        } else {
        if value < vmin || value > vmax {
        panic!("ValueError ( "%s must be in [%d, %d]. got %s" % ( name , vmin , vmax , value ) )");
        return  object . __setattr__ ( self , name , value );
        pub fn _set_signal_dict ( &self, name , d )  {
        if !isinstance ( d , dict ) {
        panic!("TypeError ( "%s must be a signal dict" % d )");
        for key in d .iter() {
        if !key in _signals {
        panic!("KeyError ( "%s is !a valid signal dict" % d )");
        for key in _signals .iter() {
        if !key in d {
        panic!("KeyError ( "%s is !a valid signal dict" % d )");
        return  object . __setattr__ ( self , name , d );
        pub fn __setattr__ ( &self, name , value )  {
        if name == "prec" {
        return  self . _set_integer_check ( name , value , 1 , "inf" );
        } else if name == "Emin" {
        return  self . _set_integer_check ( name , value , "-inf" , 0 );
        } else if name == "Emax" {
        return  self . _set_integer_check ( name , value , 0 , "inf" );
        } else if name == "capitals" {
        return  self . _set_integer_check ( name , value , 0 , 1 );
        } else if name == "clamp" {
        return  self . _set_integer_check ( name , value , 0 , 1 );
        } else if name == "rounding" {
        if !value in _rounding_modes {
        panic!("TypeError ( "%s: invalid rounding mode" % value )");
        return  object . __setattr__ ( self , name , value );
        } else if name == "flags" || name == "traps" {
        return  self . _set_signal_dict ( name , value );
        } else if name == "_ignored_flags" {
        return  object . __setattr__ ( self , name , value );
        } else {
        panic!("AttributeError (");
        "'decimal.Context' object has no attribute '%s'" % name );
        pub fn __delattr__ ( &self, name )  {
        panic!("AttributeError ( "%s cannot be deleted" % name )");
        pub fn __reduce__ ( self )  {
        flags = vec![ sig.iter().map(|sig , v| self . flags . items ( ) if v ).collect();
        traps = vec![ sig.iter().map(|sig , v| self . traps . items ( ) if v ).collect();
        return  ( self . __class__ ,;
        ( self . prec , self . rounding , self . Emin , self . Emax ,;
        self . capitals , self . clamp , flags , traps ) );
        pub fn __repr__ ( self )  {
        "Show the current context.";
        s = [ ];
        s . append ( "Context(prec=%(prec)d, rounding=%(rounding)s, ";
        "Emin=%(Emin)d, Emax=%(Emax)d, capitals=%(capitals)d, ";
        "clamp=%(clamp)d";
        % vars ( self ) );
        names = vec![ f . __name__.iter().map(|f , v| self . flags . items ( ) if v ).collect();
        s . append ( "flags=[" + ", " . join ( names ) + "]" );
        names = vec![ t . __name__.iter().map(|t , v| self . traps . items ( ) if v ).collect();
        s . append ( "traps=[" + ", " . join ( names ) + "]" );
        return  ", " . join ( s ) + ")";
        pub fn clear_flags ( self )  {
        "Reset all flags to zero";
        for flag in self . flags .iter() {
        self . flags [ flag ] = 0;
        pub fn clear_traps ( self )  {
        "Reset all traps to zero";
        for flag in self . traps .iter() {
        self . traps [ flag ] = 0;
        pub fn _shallow_copy ( self )  {
        "Returns a shallow copy from self.";
        nc = Context ( self . prec , self . rounding , self . Emin , self . Emax ,;
        self . capitals , self . clamp , self . flags , self . traps ,;
        self . _ignored_flags );
        return  nc;
        pub fn copy ( self )  {
        "Returns a deep copy from self.";
        nc = Context ( self . prec , self . rounding , self . Emin , self . Emax ,;
        self . capitals , self . clamp ,;
        self . flags . copy ( ) , self . traps . copy ( ) ,;
        self . _ignored_flags );
        return  nc;
        __copy__ = copy;
        pub fn _raise_error ( &self, condition , explanation = None /* Option */ , * args )  {
        "Handles an error

        If the flag == in _ignored_flags, returns the default response.
        Otherwise, it sets the flag, then, if the corresponding
        trap_enabler == set, it reraises the exception.  Otherwise, it returns
        the default value after setting the flag.
        ";
        error = _condition_map . get ( condition , condition );
        if error in self . _ignored_flags {
        return  error ( ) . handle ( self , * args );
        self . flags [ error ] = 1;
        if !self . traps [ error ] {
        return  condition ( ) . handle ( self , * args );
        panic!("error ( explanation )");
        pub fn _ignore_all_flags ( self )  {
        "Ignore all flags, if they are raised";
        return  self . _ignore_flags ( * _signals );
        pub fn _ignore_flags ( &self, * flags )  {
        "Ignore the flags, if they are raised";
        self . _ignored_flags = ( self . _ignored_flags + list ( flags ) );
        return  list ( flags );
        pub fn _regard_flags ( &self, * flags )  {
        "Stop ignoring the flags, if they are raised";
        if flags && isinstance ( flags [ 0 ] , ( tuple , list ) ) {
        flags = flags [ 0 ];
        for flag in flags .iter() {
        self . _ignored_flags . remove ( flag );
        __hash__ = None /* Option */;
        pub fn Etiny ( self )  {
        "Returns Etiny (= Emin - prec + 1)";
        return  int ( self . Emin - self . prec + 1 );
        pub fn Etop ( self )  {
        "Returns maximum exponent (= Emax - prec + 1)";
        return  int ( self . Emax - self . prec + 1 );
        pub fn _set_rounding ( &self, type )  {
        "Sets the rounding type.

        Sets the rounding type, && returns the current (previous)
        rounding type.  Often used like:

        context = context.copy()
        # so you don't change the calling context
        # if an error occurs in the middle.
        rounding = context._set_rounding(ROUND_UP)
        val = self.__sub__(other, context=context)
        context._set_rounding(rounding)

        This will make it round up for that operation.
        ";
        rounding = self . rounding;
        self . rounding = type;
        return  rounding;
        pub fn create_decimal ( &self, num = "0" )  {
        "Creates a new Decimal instance but using self as context.

        This method implements the to-number operation of the
        IBM Decimal specification.";
        if isinstance ( num , str ) && ( num != num . strip ( ) || "_" in num ) {
        return  self . _raise_error ( ConversionSyntax ,;
        "trailing || leading whitespace && ";
        "underscores are !permitted." );
        d = Decimal ( num , context = self );
        if d . _isnan ( ) && len ( d . _int ) > self . prec - self . clamp {
        return  self . _raise_error ( ConversionSyntax ,;
        "diagnostic info too long in NaN" );
        return  d . _fix ( self );
        pub fn create_decimal_from_float ( &self, f )  {
        "Creates a new Decimal instance from a float but rounding using self
        as the context.

        >>> context = Context(prec=5, rounding=ROUND_DOWN)
        >>> context.create_decimal_from_float(3.1415926535897932)
        Decimal('3.1415')
        >>> context = Context(prec=5, traps=[Inexact])
        >>> context.create_decimal_from_float(3.1415926535897932)
        Traceback (most recent call last):
            ...
        decimal.Inexact: None /* Option */

        ";
        d = Decimal . from_float ( f );
        return  d . _fix ( self );
        pub fn abs ( &self, a )  {
        "Returns the absolute value of the operand.

        If the operand == negative, the result == the same as using the minus
        operation on the operand.  Otherwise, the result == the same as using
        the plus operation on the operand.

        >>> ExtendedContext.abs(Decimal('2.1'))
        Decimal('2.1')
        >>> ExtendedContext.abs(Decimal('-100'))
        Decimal('100')
        >>> ExtendedContext.abs(Decimal('101.5'))
        Decimal('101.5')
        >>> ExtendedContext.abs(Decimal('-101.5'))
        Decimal('101.5')
        >>> ExtendedContext.abs(-1)
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . __abs__ ( context = self );
        pub fn add ( &self, a , b )  {
        "Return the sum of the two operands.

        >>> ExtendedContext.add(Decimal('12'), Decimal('7.00'))
        Decimal('19.00')
        >>> ExtendedContext.add(Decimal('1E+2'), Decimal('1.01E+4'))
        Decimal('1.02E+4')
        >>> ExtendedContext.add(1, Decimal(2))
        Decimal('3')
        >>> ExtendedContext.add(Decimal(8), 5)
        Decimal('13')
        >>> ExtendedContext.add(5, 5)
        Decimal('10')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __add__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn _apply ( &self, a )  {
        return  str ( a . _fix ( self ) );
        pub fn canonical ( &self, a )  {
        "Returns the same Decimal object.

        As we do !have different encodings for the same number, the
        received object already == in its canonical form.

        >>> ExtendedContext.canonical(Decimal('2.50'))
        Decimal('2.50')
        ";
        if !isinstance ( a , Decimal ) {
        panic!("TypeError ( "canonical requires a Decimal as an argument." )");
        return  a . canonical ( );
        pub fn compare ( &self, a , b )  {
        "Compares values numerically.

        If the signs of the operands differ, a value representing each operand
        ('-1' if the operand == less than zero, '0' if the operand == zero or
        negative zero, || '1' if the operand == greater than zero) == used in
        place of that operand for the comparison instead of the actual
        operand.

        The comparison == then effected by subtracting the second operand from
        the first && then returning a value according to the result of the
        subtraction: '-1' if the result == less than zero, '0' if the result is
        zero || negative zero, || '1' if the result == greater than zero.

        >>> ExtendedContext.compare(Decimal('2.1'), Decimal('3'))
        Decimal('-1')
        >>> ExtendedContext.compare(Decimal('2.1'), Decimal('2.1'))
        Decimal('0')
        >>> ExtendedContext.compare(Decimal('2.1'), Decimal('2.10'))
        Decimal('0')
        >>> ExtendedContext.compare(Decimal('3'), Decimal('2.1'))
        Decimal('1')
        >>> ExtendedContext.compare(Decimal('2.1'), Decimal('-3'))
        Decimal('1')
        >>> ExtendedContext.compare(Decimal('-3'), Decimal('2.1'))
        Decimal('-1')
        >>> ExtendedContext.compare(1, 2)
        Decimal('-1')
        >>> ExtendedContext.compare(Decimal(1), 2)
        Decimal('-1')
        >>> ExtendedContext.compare(1, Decimal(2))
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . compare ( b , context = self );
        pub fn compare_signal ( &self, a , b )  {
        "Compares the values of the two operands numerically.

        It's pretty much like compare(), but all NaNs signal, with signaling
        NaNs taking precedence over quiet NaNs.

        >>> c = ExtendedContext
        >>> c.compare_signal(Decimal('2.1'), Decimal('3'))
        Decimal('-1')
        >>> c.compare_signal(Decimal('2.1'), Decimal('2.1'))
        Decimal('0')
        >>> c.flags[InvalidOperation] = 0
        >>> print(c.flags[InvalidOperation])
        0
        >>> c.compare_signal(Decimal('NaN'), Decimal('2.1'))
        Decimal('NaN')
        >>> print(c.flags[InvalidOperation])
        1
        >>> c.flags[InvalidOperation] = 0
        >>> print(c.flags[InvalidOperation])
        0
        >>> c.compare_signal(Decimal('sNaN'), Decimal('2.1'))
        Decimal('NaN')
        >>> print(c.flags[InvalidOperation])
        1
        >>> c.compare_signal(-1, 2)
        Decimal('-1')
        >>> c.compare_signal(Decimal(-1), 2)
        Decimal('-1')
        >>> c.compare_signal(-1, Decimal(2))
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . compare_signal ( b , context = self );
        pub fn compare_total ( &self, a , b )  {
        "Compares two operands using their abstract representation.

        This == !like the standard compare, which use their numerical
        value. Note that a total ordering == defined for all possible abstract
        representations.

        >>> ExtendedContext.compare_total(Decimal('12.73'), Decimal('127.9'))
        Decimal('-1')
        >>> ExtendedContext.compare_total(Decimal('-127'),  Decimal('12'))
        Decimal('-1')
        >>> ExtendedContext.compare_total(Decimal('12.30'), Decimal('12.3'))
        Decimal('-1')
        >>> ExtendedContext.compare_total(Decimal('12.30'), Decimal('12.30'))
        Decimal('0')
        >>> ExtendedContext.compare_total(Decimal('12.3'),  Decimal('12.300'))
        Decimal('1')
        >>> ExtendedContext.compare_total(Decimal('12.3'),  Decimal('NaN'))
        Decimal('-1')
        >>> ExtendedContext.compare_total(1, 2)
        Decimal('-1')
        >>> ExtendedContext.compare_total(Decimal(1), 2)
        Decimal('-1')
        >>> ExtendedContext.compare_total(1, Decimal(2))
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . compare_total ( b );
        pub fn compare_total_mag ( &self, a , b )  {
        "Compares two operands using their abstract representation ignoring sign.

        Like compare_total, but with operand's sign ignored && assumed to be 0.
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . compare_total_mag ( b );
        pub fn copy_abs ( &self, a )  {
        "Returns a copy of the operand with the sign set to 0.

        >>> ExtendedContext.copy_abs(Decimal('2.1'))
        Decimal('2.1')
        >>> ExtendedContext.copy_abs(Decimal('-100'))
        Decimal('100')
        >>> ExtendedContext.copy_abs(-1)
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . copy_abs ( );
        pub fn copy_decimal ( &self, a )  {
        "Returns a copy of the decimal object.

        >>> ExtendedContext.copy_decimal(Decimal('2.1'))
        Decimal('2.1')
        >>> ExtendedContext.copy_decimal(Decimal('-1.00'))
        Decimal('-1.00')
        >>> ExtendedContext.copy_decimal(1)
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  Decimal ( a );
        pub fn copy_negate ( &self, a )  {
        "Returns a copy of the operand with the sign inverted.

        >>> ExtendedContext.copy_negate(Decimal('101.5'))
        Decimal('-101.5')
        >>> ExtendedContext.copy_negate(Decimal('-101.5'))
        Decimal('101.5')
        >>> ExtendedContext.copy_negate(1)
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . copy_negate ( );
        pub fn copy_sign ( &self, a , b )  {
        "Copies the second operand's sign to the first one.

        In detail, it returns a copy of the first operand with the sign
        equal to the sign of the second operand.

        >>> ExtendedContext.copy_sign(Decimal( '1.50'), Decimal('7.33'))
        Decimal('1.50')
        >>> ExtendedContext.copy_sign(Decimal('-1.50'), Decimal('7.33'))
        Decimal('1.50')
        >>> ExtendedContext.copy_sign(Decimal( '1.50'), Decimal('-7.33'))
        Decimal('-1.50')
        >>> ExtendedContext.copy_sign(Decimal('-1.50'), Decimal('-7.33'))
        Decimal('-1.50')
        >>> ExtendedContext.copy_sign(1, -2)
        Decimal('-1')
        >>> ExtendedContext.copy_sign(Decimal(1), -2)
        Decimal('-1')
        >>> ExtendedContext.copy_sign(1, Decimal(-2))
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . copy_sign ( b );
        pub fn divide ( &self, a , b )  {
        "Decimal division in a specified context.

        >>> ExtendedContext.divide(Decimal('1'), Decimal('3'))
        Decimal('0.333333333')
        >>> ExtendedContext.divide(Decimal('2'), Decimal('3'))
        Decimal('0.666666667')
        >>> ExtendedContext.divide(Decimal('5'), Decimal('2'))
        Decimal('2.5')
        >>> ExtendedContext.divide(Decimal('1'), Decimal('10'))
        Decimal('0.1')
        >>> ExtendedContext.divide(Decimal('12'), Decimal('12'))
        Decimal('1')
        >>> ExtendedContext.divide(Decimal('8.00'), Decimal('2'))
        Decimal('4.00')
        >>> ExtendedContext.divide(Decimal('2.400'), Decimal('2.0'))
        Decimal('1.20')
        >>> ExtendedContext.divide(Decimal('1000'), Decimal('100'))
        Decimal('10')
        >>> ExtendedContext.divide(Decimal('1000'), Decimal('1'))
        Decimal('1000')
        >>> ExtendedContext.divide(Decimal('2.40E+6'), Decimal('2'))
        Decimal('1.20E+6')
        >>> ExtendedContext.divide(5, 5)
        Decimal('1')
        >>> ExtendedContext.divide(Decimal(5), 5)
        Decimal('1')
        >>> ExtendedContext.divide(5, Decimal(5))
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __truediv__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn divide_int ( &self, a , b )  {
        "Divides two numbers && returns the integer part of the result.

        >>> ExtendedContext.divide_int(Decimal('2'), Decimal('3'))
        Decimal('0')
        >>> ExtendedContext.divide_int(Decimal('10'), Decimal('3'))
        Decimal('3')
        >>> ExtendedContext.divide_int(Decimal('1'), Decimal('0.3'))
        Decimal('3')
        >>> ExtendedContext.divide_int(10, 3)
        Decimal('3')
        >>> ExtendedContext.divide_int(Decimal(10), 3)
        Decimal('3')
        >>> ExtendedContext.divide_int(10, Decimal(3))
        Decimal('3')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __floordiv__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn divmod ( &self, a , b )  {
        "Return (a // b, a % b).

        >>> ExtendedContext.divmod(Decimal(8), Decimal(3))
        (Decimal('2'), Decimal('2'))
        >>> ExtendedContext.divmod(Decimal(8), Decimal(4))
        (Decimal('2'), Decimal('0'))
        >>> ExtendedContext.divmod(8, 4)
        (Decimal('2'), Decimal('0'))
        >>> ExtendedContext.divmod(Decimal(8), 4)
        (Decimal('2'), Decimal('0'))
        >>> ExtendedContext.divmod(8, Decimal(4))
        (Decimal('2'), Decimal('0'))
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __divmod__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn exp ( &self, a )  {
        "Returns e ** a.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.exp(Decimal('-Infinity'))
        Decimal('0')
        >>> c.exp(Decimal('-1'))
        Decimal('0.367879441')
        >>> c.exp(Decimal('0'))
        Decimal('1')
        >>> c.exp(Decimal('1'))
        Decimal('2.71828183')
        >>> c.exp(Decimal('0.693147181'))
        Decimal('2.00000000')
        >>> c.exp(Decimal('+Infinity'))
        Decimal('Infinity')
        >>> c.exp(10)
        Decimal('22026.4658')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . exp ( context = self );
        pub fn fma ( &self, a , b , c )  {
        "Returns a multiplied by b, plus c.

        The first two operands are multiplied together, using multiply,
        the third operand == then added to the result of that
        multiplication, using add, all with only one final rounding.

        >>> ExtendedContext.fma(Decimal('3'), Decimal('5'), Decimal('7'))
        Decimal('22')
        >>> ExtendedContext.fma(Decimal('3'), Decimal('-5'), Decimal('7'))
        Decimal('-8')
        >>> ExtendedContext.fma(Decimal('888565290'), Decimal('1557.96930'), Decimal('-86087.7578'))
        Decimal('1.38435736E+12')
        >>> ExtendedContext.fma(1, 3, 4)
        Decimal('7')
        >>> ExtendedContext.fma(1, Decimal(3), 4)
        Decimal('7')
        >>> ExtendedContext.fma(1, 3, Decimal(4))
        Decimal('7')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . fma ( b , c , context = self );
        pub fn is_canonical ( &self, a )  {
        "Return true if the operand == canonical; otherwise return false.

        Currently, the encoding of a Decimal instance == always
        canonical, so this method returns true for any Decimal.

        >>> ExtendedContext.is_canonical(Decimal('2.50'))
        true
        ";
        if !isinstance ( a , Decimal ) {
        panic!("TypeError ( "is_canonical requires a Decimal as an argument." )");
        return  a . is_canonical ( );
        pub fn is_finite ( &self, a )  {
        "Return true if the operand == finite; otherwise return false.

        A Decimal instance == considered finite if it == neither
        infinite nor a NaN.

        >>> ExtendedContext.is_finite(Decimal('2.50'))
        true
        >>> ExtendedContext.is_finite(Decimal('-0.3'))
        true
        >>> ExtendedContext.is_finite(Decimal('0'))
        true
        >>> ExtendedContext.is_finite(Decimal('Inf'))
        false
        >>> ExtendedContext.is_finite(Decimal('NaN'))
        false
        >>> ExtendedContext.is_finite(1)
        true
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_finite ( );
        pub fn is_infinite ( &self, a )  {
        "Return true if the operand == infinite; otherwise return false.

        >>> ExtendedContext.is_infinite(Decimal('2.50'))
        false
        >>> ExtendedContext.is_infinite(Decimal('-Inf'))
        true
        >>> ExtendedContext.is_infinite(Decimal('NaN'))
        false
        >>> ExtendedContext.is_infinite(1)
        false
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_infinite ( );
        pub fn is_nan ( &self, a )  {
        "Return true if the operand == a qNaN || sNaN;
        otherwise return false.

        >>> ExtendedContext.is_nan(Decimal('2.50'))
        false
        >>> ExtendedContext.is_nan(Decimal('NaN'))
        true
        >>> ExtendedContext.is_nan(Decimal('-sNaN'))
        true
        >>> ExtendedContext.is_nan(1)
        false
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_nan ( );
        pub fn is_normal ( &self, a )  {
        "Return true if the operand == a normal number;
        otherwise return false.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.is_normal(Decimal('2.50'))
        true
        >>> c.is_normal(Decimal('0.1E-999'))
        false
        >>> c.is_normal(Decimal('0.00'))
        false
        >>> c.is_normal(Decimal('-Inf'))
        false
        >>> c.is_normal(Decimal('NaN'))
        false
        >>> c.is_normal(1)
        true
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_normal ( context = self );
        pub fn is_qnan ( &self, a )  {
        "Return true if the operand == a quiet NaN; otherwise return false.

        >>> ExtendedContext.is_qnan(Decimal('2.50'))
        false
        >>> ExtendedContext.is_qnan(Decimal('NaN'))
        true
        >>> ExtendedContext.is_qnan(Decimal('sNaN'))
        false
        >>> ExtendedContext.is_qnan(1)
        false
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_qnan ( );
        pub fn is_signed ( &self, a )  {
        "Return true if the operand == negative; otherwise return false.

        >>> ExtendedContext.is_signed(Decimal('2.50'))
        false
        >>> ExtendedContext.is_signed(Decimal('-12'))
        true
        >>> ExtendedContext.is_signed(Decimal('-0'))
        true
        >>> ExtendedContext.is_signed(8)
        false
        >>> ExtendedContext.is_signed(-8)
        true
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_signed ( );
        pub fn is_snan ( &self, a )  {
        "Return true if the operand == a signaling NaN;
        otherwise return false.

        >>> ExtendedContext.is_snan(Decimal('2.50'))
        false
        >>> ExtendedContext.is_snan(Decimal('NaN'))
        false
        >>> ExtendedContext.is_snan(Decimal('sNaN'))
        true
        >>> ExtendedContext.is_snan(1)
        false
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_snan ( );
        pub fn is_subnormal ( &self, a )  {
        "Return true if the operand == subnormal; otherwise return false.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.is_subnormal(Decimal('2.50'))
        false
        >>> c.is_subnormal(Decimal('0.1E-999'))
        true
        >>> c.is_subnormal(Decimal('0.00'))
        false
        >>> c.is_subnormal(Decimal('-Inf'))
        false
        >>> c.is_subnormal(Decimal('NaN'))
        false
        >>> c.is_subnormal(1)
        false
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_subnormal ( context = self );
        pub fn is_zero ( &self, a )  {
        "Return true if the operand == a zero; otherwise return false.

        >>> ExtendedContext.is_zero(Decimal('0'))
        true
        >>> ExtendedContext.is_zero(Decimal('2.50'))
        false
        >>> ExtendedContext.is_zero(Decimal('-0E+2'))
        true
        >>> ExtendedContext.is_zero(1)
        false
        >>> ExtendedContext.is_zero(0)
        true
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . is_zero ( );
        pub fn ln ( &self, a )  {
        "Returns the natural (base e) logarithm of the operand.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.ln(Decimal('0'))
        Decimal('-Infinity')
        >>> c.ln(Decimal('1.000'))
        Decimal('0')
        >>> c.ln(Decimal('2.71828183'))
        Decimal('1.00000000')
        >>> c.ln(Decimal('10'))
        Decimal('2.30258509')
        >>> c.ln(Decimal('+Infinity'))
        Decimal('Infinity')
        >>> c.ln(1)
        Decimal('0')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . ln ( context = self );
        pub fn log10 ( &self, a )  {
        "Returns the base 10 logarithm of the operand.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.log10(Decimal('0'))
        Decimal('-Infinity')
        >>> c.log10(Decimal('0.001'))
        Decimal('-3')
        >>> c.log10(Decimal('1.000'))
        Decimal('0')
        >>> c.log10(Decimal('2'))
        Decimal('0.301029996')
        >>> c.log10(Decimal('10'))
        Decimal('1')
        >>> c.log10(Decimal('70'))
        Decimal('1.84509804')
        >>> c.log10(Decimal('+Infinity'))
        Decimal('Infinity')
        >>> c.log10(0)
        Decimal('-Infinity')
        >>> c.log10(1)
        Decimal('0')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . log10 ( context = self );
        pub fn logb ( &self, a )  {
        " Returns the exponent of the magnitude of the operand's MSD.

        The result == the integer which == the exponent of the magnitude
        of the most significant digit of the operand (as though the
        operand were truncated to a single digit while maintaining the
        value of that digit && without limiting the resulting exponent).

        >>> ExtendedContext.logb(Decimal('250'))
        Decimal('2')
        >>> ExtendedContext.logb(Decimal('2.50'))
        Decimal('0')
        >>> ExtendedContext.logb(Decimal('0.03'))
        Decimal('-2')
        >>> ExtendedContext.logb(Decimal('0'))
        Decimal('-Infinity')
        >>> ExtendedContext.logb(1)
        Decimal('0')
        >>> ExtendedContext.logb(10)
        Decimal('1')
        >>> ExtendedContext.logb(100)
        Decimal('2')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . logb ( context = self );
        pub fn logical_and ( &self, a , b )  {
        "Applies the logical operation 'and' between each operand's digits.

        The operands must be both logical numbers.

        >>> ExtendedContext.logical_and(Decimal('0'), Decimal('0'))
        Decimal('0')
        >>> ExtendedContext.logical_and(Decimal('0'), Decimal('1'))
        Decimal('0')
        >>> ExtendedContext.logical_and(Decimal('1'), Decimal('0'))
        Decimal('0')
        >>> ExtendedContext.logical_and(Decimal('1'), Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.logical_and(Decimal('1100'), Decimal('1010'))
        Decimal('1000')
        >>> ExtendedContext.logical_and(Decimal('1111'), Decimal('10'))
        Decimal('10')
        >>> ExtendedContext.logical_and(110, 1101)
        Decimal('100')
        >>> ExtendedContext.logical_and(Decimal(110), 1101)
        Decimal('100')
        >>> ExtendedContext.logical_and(110, Decimal(1101))
        Decimal('100')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . logical_and ( b , context = self );
        pub fn logical_invert ( &self, a )  {
        "Invert all the digits in the operand.

        The operand must be a logical number.

        >>> ExtendedContext.logical_invert(Decimal('0'))
        Decimal('111111111')
        >>> ExtendedContext.logical_invert(Decimal('1'))
        Decimal('111111110')
        >>> ExtendedContext.logical_invert(Decimal('111111111'))
        Decimal('0')
        >>> ExtendedContext.logical_invert(Decimal('101010101'))
        Decimal('10101010')
        >>> ExtendedContext.logical_invert(1101)
        Decimal('111110010')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . logical_invert ( context = self );
        pub fn logical_or ( &self, a , b )  {
        "Applies the logical operation 'or' between each operand's digits.

        The operands must be both logical numbers.

        >>> ExtendedContext.logical_or(Decimal('0'), Decimal('0'))
        Decimal('0')
        >>> ExtendedContext.logical_or(Decimal('0'), Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.logical_or(Decimal('1'), Decimal('0'))
        Decimal('1')
        >>> ExtendedContext.logical_or(Decimal('1'), Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.logical_or(Decimal('1100'), Decimal('1010'))
        Decimal('1110')
        >>> ExtendedContext.logical_or(Decimal('1110'), Decimal('10'))
        Decimal('1110')
        >>> ExtendedContext.logical_or(110, 1101)
        Decimal('1111')
        >>> ExtendedContext.logical_or(Decimal(110), 1101)
        Decimal('1111')
        >>> ExtendedContext.logical_or(110, Decimal(1101))
        Decimal('1111')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . logical_or ( b , context = self );
        pub fn logical_xor ( &self, a , b )  {
        "Applies the logical operation 'xor' between each operand's digits.

        The operands must be both logical numbers.

        >>> ExtendedContext.logical_xor(Decimal('0'), Decimal('0'))
        Decimal('0')
        >>> ExtendedContext.logical_xor(Decimal('0'), Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.logical_xor(Decimal('1'), Decimal('0'))
        Decimal('1')
        >>> ExtendedContext.logical_xor(Decimal('1'), Decimal('1'))
        Decimal('0')
        >>> ExtendedContext.logical_xor(Decimal('1100'), Decimal('1010'))
        Decimal('110')
        >>> ExtendedContext.logical_xor(Decimal('1111'), Decimal('10'))
        Decimal('1101')
        >>> ExtendedContext.logical_xor(110, 1101)
        Decimal('1011')
        >>> ExtendedContext.logical_xor(Decimal(110), 1101)
        Decimal('1011')
        >>> ExtendedContext.logical_xor(110, Decimal(1101))
        Decimal('1011')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . logical_xor ( b , context = self );
        pub fn max ( &self, a , b )  {
        "max compares two values numerically && returns the maximum.

        If either operand == a NaN then the general rules apply.
        Otherwise, the operands are compared as though by the compare
        operation.  If they are numerically equal then the left-hand operand
        == chosen as the result.  Otherwise the maximum (closer to positive
        infinity) of the two operands == chosen as the result.

        >>> ExtendedContext.max(Decimal('3'), Decimal('2'))
        Decimal('3')
        >>> ExtendedContext.max(Decimal('-10'), Decimal('3'))
        Decimal('3')
        >>> ExtendedContext.max(Decimal('1.0'), Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.max(Decimal('7'), Decimal('NaN'))
        Decimal('7')
        >>> ExtendedContext.max(1, 2)
        Decimal('2')
        >>> ExtendedContext.max(Decimal(1), 2)
        Decimal('2')
        >>> ExtendedContext.max(1, Decimal(2))
        Decimal('2')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . max ( b , context = self );
        pub fn max_mag ( &self, a , b )  {
        "Compares the values numerically with their sign ignored.

        >>> ExtendedContext.max_mag(Decimal('7'), Decimal('NaN'))
        Decimal('7')
        >>> ExtendedContext.max_mag(Decimal('7'), Decimal('-10'))
        Decimal('-10')
        >>> ExtendedContext.max_mag(1, -2)
        Decimal('-2')
        >>> ExtendedContext.max_mag(Decimal(1), -2)
        Decimal('-2')
        >>> ExtendedContext.max_mag(1, Decimal(-2))
        Decimal('-2')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . max_mag ( b , context = self );
        pub fn min ( &self, a , b )  {
        "min compares two values numerically && returns the minimum.

        If either operand == a NaN then the general rules apply.
        Otherwise, the operands are compared as though by the compare
        operation.  If they are numerically equal then the left-hand operand
        == chosen as the result.  Otherwise the minimum (closer to negative
        infinity) of the two operands == chosen as the result.

        >>> ExtendedContext.min(Decimal('3'), Decimal('2'))
        Decimal('2')
        >>> ExtendedContext.min(Decimal('-10'), Decimal('3'))
        Decimal('-10')
        >>> ExtendedContext.min(Decimal('1.0'), Decimal('1'))
        Decimal('1.0')
        >>> ExtendedContext.min(Decimal('7'), Decimal('NaN'))
        Decimal('7')
        >>> ExtendedContext.min(1, 2)
        Decimal('1')
        >>> ExtendedContext.min(Decimal(1), 2)
        Decimal('1')
        >>> ExtendedContext.min(1, Decimal(29))
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . min ( b , context = self );
        pub fn min_mag ( &self, a , b )  {
        "Compares the values numerically with their sign ignored.

        >>> ExtendedContext.min_mag(Decimal('3'), Decimal('-2'))
        Decimal('-2')
        >>> ExtendedContext.min_mag(Decimal('-3'), Decimal('NaN'))
        Decimal('-3')
        >>> ExtendedContext.min_mag(1, -2)
        Decimal('1')
        >>> ExtendedContext.min_mag(Decimal(1), -2)
        Decimal('1')
        >>> ExtendedContext.min_mag(1, Decimal(-2))
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . min_mag ( b , context = self );
        pub fn minus ( &self, a )  {
        "Minus corresponds to unary prefix minus in Python.

        The operation == evaluated using the same rules as subtract; the
        operation minus(a) == calculated as subtract('0', a) where the '0'
        has the same exponent as the operand.

        >>> ExtendedContext.minus(Decimal('1.3'))
        Decimal('-1.3')
        >>> ExtendedContext.minus(Decimal('-1.3'))
        Decimal('1.3')
        >>> ExtendedContext.minus(1)
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . __neg__ ( context = self );
        pub fn multiply ( &self, a , b )  {
        "multiply multiplies two operands.

        If either operand == a special value then the general rules apply.
        Otherwise, the operands are multiplied together
        ('long multiplication'), resulting in a number which may be as long as
        the sum of the lengths of the two operands.

        >>> ExtendedContext.multiply(Decimal('1.20'), Decimal('3'))
        Decimal('3.60')
        >>> ExtendedContext.multiply(Decimal('7'), Decimal('3'))
        Decimal('21')
        >>> ExtendedContext.multiply(Decimal('0.9'), Decimal('0.8'))
        Decimal('0.72')
        >>> ExtendedContext.multiply(Decimal('0.9'), Decimal('-0'))
        Decimal('-0.0')
        >>> ExtendedContext.multiply(Decimal('654321'), Decimal('654321'))
        Decimal('4.28135971E+11')
        >>> ExtendedContext.multiply(7, 7)
        Decimal('49')
        >>> ExtendedContext.multiply(Decimal(7), 7)
        Decimal('49')
        >>> ExtendedContext.multiply(7, Decimal(7))
        Decimal('49')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __mul__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn next_minus ( &self, a )  {
        "Returns the largest representable number smaller than a.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> ExtendedContext.next_minus(Decimal('1'))
        Decimal('0.999999999')
        >>> c.next_minus(Decimal('1E-1007'))
        Decimal('0E-1007')
        >>> ExtendedContext.next_minus(Decimal('-1.00000003'))
        Decimal('-1.00000004')
        >>> c.next_minus(Decimal('Infinity'))
        Decimal('9.99999999E+999')
        >>> c.next_minus(1)
        Decimal('0.999999999')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . next_minus ( context = self );
        pub fn next_plus ( &self, a )  {
        "Returns the smallest representable number larger than a.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> ExtendedContext.next_plus(Decimal('1'))
        Decimal('1.00000001')
        >>> c.next_plus(Decimal('-1E-1007'))
        Decimal('-0E-1007')
        >>> ExtendedContext.next_plus(Decimal('-1.00000003'))
        Decimal('-1.00000002')
        >>> c.next_plus(Decimal('-Infinity'))
        Decimal('-9.99999999E+999')
        >>> c.next_plus(1)
        Decimal('1.00000001')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . next_plus ( context = self );
        pub fn next_toward ( &self, a , b )  {
        "Returns the number closest to a, in direction towards b.

        The result == the closest representable number from the first
        operand (but !the first operand) that == in the direction
        towards the second operand, unless the operands have the same
        value.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.next_toward(Decimal('1'), Decimal('2'))
        Decimal('1.00000001')
        >>> c.next_toward(Decimal('-1E-1007'), Decimal('1'))
        Decimal('-0E-1007')
        >>> c.next_toward(Decimal('-1.00000003'), Decimal('0'))
        Decimal('-1.00000002')
        >>> c.next_toward(Decimal('1'), Decimal('0'))
        Decimal('0.999999999')
        >>> c.next_toward(Decimal('1E-1007'), Decimal('-100'))
        Decimal('0E-1007')
        >>> c.next_toward(Decimal('-1.00000003'), Decimal('-10'))
        Decimal('-1.00000004')
        >>> c.next_toward(Decimal('0.00'), Decimal('-0.0000'))
        Decimal('-0.00')
        >>> c.next_toward(0, 1)
        Decimal('1E-1007')
        >>> c.next_toward(Decimal(0), 1)
        Decimal('1E-1007')
        >>> c.next_toward(0, Decimal(1))
        Decimal('1E-1007')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . next_toward ( b , context = self );
        pub fn normalize ( &self, a )  {
        "normalize reduces an operand to its simplest form.

        Essentially a plus operation with all trailing zeros removed from the
        result.

        >>> ExtendedContext.normalize(Decimal('2.1'))
        Decimal('2.1')
        >>> ExtendedContext.normalize(Decimal('-2.0'))
        Decimal('-2')
        >>> ExtendedContext.normalize(Decimal('1.200'))
        Decimal('1.2')
        >>> ExtendedContext.normalize(Decimal('-120'))
        Decimal('-1.2E+2')
        >>> ExtendedContext.normalize(Decimal('120.00'))
        Decimal('1.2E+2')
        >>> ExtendedContext.normalize(Decimal('0.00'))
        Decimal('0')
        >>> ExtendedContext.normalize(6)
        Decimal('6')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . normalize ( context = self );
        pub fn number_class ( &self, a )  {
        "Returns an indication of the class of the operand.

        The class == one of the following strings:
          -sNaN
          -NaN
          -Infinity
          -Normal
          -Subnormal
          -Zero
          +Zero
          +Subnormal
          +Normal
          +Infinity

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.number_class(Decimal('Infinity'))
        '+Infinity'
        >>> c.number_class(Decimal('1E-10'))
        '+Normal'
        >>> c.number_class(Decimal('2.50'))
        '+Normal'
        >>> c.number_class(Decimal('0.1E-999'))
        '+Subnormal'
        >>> c.number_class(Decimal('0'))
        '+Zero'
        >>> c.number_class(Decimal('-0'))
        '-Zero'
        >>> c.number_class(Decimal('-0.1E-999'))
        '-Subnormal'
        >>> c.number_class(Decimal('-1E-10'))
        '-Normal'
        >>> c.number_class(Decimal('-2.50'))
        '-Normal'
        >>> c.number_class(Decimal('-Infinity'))
        '-Infinity'
        >>> c.number_class(Decimal('NaN'))
        'NaN'
        >>> c.number_class(Decimal('-NaN'))
        'NaN'
        >>> c.number_class(Decimal('sNaN'))
        'sNaN'
        >>> c.number_class(123)
        '+Normal'
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . number_class ( context = self );
        pub fn plus ( &self, a )  {
        "Plus corresponds to unary prefix plus in Python.

        The operation == evaluated using the same rules as add; the
        operation plus(a) == calculated as add('0', a) where the '0'
        has the same exponent as the operand.

        >>> ExtendedContext.plus(Decimal('1.3'))
        Decimal('1.3')
        >>> ExtendedContext.plus(Decimal('-1.3'))
        Decimal('-1.3')
        >>> ExtendedContext.plus(-1)
        Decimal('-1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . __pos__ ( context = self );
        pub fn power ( &self, a , b , modulo = None /* Option */ )  {
        "Raises a to the power of b, to modulo if given.

        With two arguments, compute a**b.  If a == negative then b
        must be integral.  The result will be inexact unless b is
        integral && the result == finite && can be expressed exactly
        in 'precision' digits.

        With three arguments, compute (a**b) % modulo.  For the
        three argument form, the following restrictions on the
        arguments hold:

         - all three arguments must be integral
         - b must be nonnegative
         - at least one of a || b must be nonzero
         - modulo must be nonzero && have at most 'precision' digits

        The result of pow(a, b, modulo) == identical to the result
        that would be obtained by computing (a**b) % modulo with
        unbounded precision, but == computed more efficiently.  It is
        always exact.

        >>> c = ExtendedContext.copy()
        >>> c.Emin = -999
        >>> c.Emax = 999
        >>> c.power(Decimal('2'), Decimal('3'))
        Decimal('8')
        >>> c.power(Decimal('-2'), Decimal('3'))
        Decimal('-8')
        >>> c.power(Decimal('2'), Decimal('-3'))
        Decimal('0.125')
        >>> c.power(Decimal('1.7'), Decimal('8'))
        Decimal('69.7575744')
        >>> c.power(Decimal('10'), Decimal('0.301029996'))
        Decimal('2.00000000')
        >>> c.power(Decimal('Infinity'), Decimal('-1'))
        Decimal('0')
        >>> c.power(Decimal('Infinity'), Decimal('0'))
        Decimal('1')
        >>> c.power(Decimal('Infinity'), Decimal('1'))
        Decimal('Infinity')
        >>> c.power(Decimal('-Infinity'), Decimal('-1'))
        Decimal('-0')
        >>> c.power(Decimal('-Infinity'), Decimal('0'))
        Decimal('1')
        >>> c.power(Decimal('-Infinity'), Decimal('1'))
        Decimal('-Infinity')
        >>> c.power(Decimal('-Infinity'), Decimal('2'))
        Decimal('Infinity')
        >>> c.power(Decimal('0'), Decimal('0'))
        Decimal('NaN')

        >>> c.power(Decimal('3'), Decimal('7'), Decimal('16'))
        Decimal('11')
        >>> c.power(Decimal('-3'), Decimal('7'), Decimal('16'))
        Decimal('-11')
        >>> c.power(Decimal('-3'), Decimal('8'), Decimal('16'))
        Decimal('1')
        >>> c.power(Decimal('3'), Decimal('7'), Decimal('-16'))
        Decimal('11')
        >>> c.power(Decimal('23E12345'), Decimal('67E189'), Decimal('123456789'))
        Decimal('11729830')
        >>> c.power(Decimal('-0'), Decimal('17'), Decimal('1729'))
        Decimal('-0')
        >>> c.power(Decimal('-23'), Decimal('0'), Decimal('65537'))
        Decimal('1')
        >>> ExtendedContext.power(7, 7)
        Decimal('823543')
        >>> ExtendedContext.power(Decimal(7), 7)
        Decimal('823543')
        >>> ExtendedContext.power(7, Decimal(7), 2)
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __pow__ ( b , modulo , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn quantize ( &self, a , b )  {
        "Returns a value equal to 'a' (rounded), having the exponent of 'b'.

        The coefficient of the result == derived from that of the left-hand
        operand.  It may be rounded using the current rounding setting (if the
        exponent == being increased), multiplied by a positive power of ten (if
        the exponent == being decreased), || == unchanged (if the exponent is
        already equal to that of the right-hand operand).

        Unlike other operations, if the length of the coefficient after the
        quantize operation would be greater than precision then an Invalid
        operation condition == raised.  This guarantees that, unless there is
        an error condition, the exponent of the result of a quantize == always
        equal to that of the right-hand operand.

        Also unlike other operations, quantize will never raise Underflow, even
        if the result == subnormal && inexact.

        >>> ExtendedContext.quantize(Decimal('2.17'), Decimal('0.001'))
        Decimal('2.170')
        >>> ExtendedContext.quantize(Decimal('2.17'), Decimal('0.01'))
        Decimal('2.17')
        >>> ExtendedContext.quantize(Decimal('2.17'), Decimal('0.1'))
        Decimal('2.2')
        >>> ExtendedContext.quantize(Decimal('2.17'), Decimal('1e+0'))
        Decimal('2')
        >>> ExtendedContext.quantize(Decimal('2.17'), Decimal('1e+1'))
        Decimal('0E+1')
        >>> ExtendedContext.quantize(Decimal('-Inf'), Decimal('Infinity'))
        Decimal('-Infinity')
        >>> ExtendedContext.quantize(Decimal('2'), Decimal('Infinity'))
        Decimal('NaN')
        >>> ExtendedContext.quantize(Decimal('-0.1'), Decimal('1'))
        Decimal('-0')
        >>> ExtendedContext.quantize(Decimal('-0'), Decimal('1e+5'))
        Decimal('-0E+5')
        >>> ExtendedContext.quantize(Decimal('+35236450.6'), Decimal('1e-2'))
        Decimal('NaN')
        >>> ExtendedContext.quantize(Decimal('-35236450.6'), Decimal('1e-2'))
        Decimal('NaN')
        >>> ExtendedContext.quantize(Decimal('217'), Decimal('1e-1'))
        Decimal('217.0')
        >>> ExtendedContext.quantize(Decimal('217'), Decimal('1e-0'))
        Decimal('217')
        >>> ExtendedContext.quantize(Decimal('217'), Decimal('1e+1'))
        Decimal('2.2E+2')
        >>> ExtendedContext.quantize(Decimal('217'), Decimal('1e+2'))
        Decimal('2E+2')
        >>> ExtendedContext.quantize(1, 2)
        Decimal('1')
        >>> ExtendedContext.quantize(Decimal(1), 2)
        Decimal('1')
        >>> ExtendedContext.quantize(1, Decimal(2))
        Decimal('1')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . quantize ( b , context = self );
        pub fn radix ( self )  {
        "Just returns 10, as this == Decimal, :)

        >>> ExtendedContext.radix()
        Decimal('10')
        ";
        return  Decimal ( 10 );
        pub fn remainder ( &self, a , b )  {
        "Returns the remainder from integer division.

        The result == the residue of the dividend after the operation of
        calculating integer division as described for divide-integer, rounded
        to precision digits if necessary.  The sign of the result, if
        non-zero, == the same as that of the original dividend.

        This operation will fail under the same conditions as integer division
        (that is, if integer division on the same two operands would fail, the
        remainder cannot be calculated).

        >>> ExtendedContext.remainder(Decimal('2.1'), Decimal('3'))
        Decimal('2.1')
        >>> ExtendedContext.remainder(Decimal('10'), Decimal('3'))
        Decimal('1')
        >>> ExtendedContext.remainder(Decimal('-10'), Decimal('3'))
        Decimal('-1')
        >>> ExtendedContext.remainder(Decimal('10.2'), Decimal('1'))
        Decimal('0.2')
        >>> ExtendedContext.remainder(Decimal('10'), Decimal('0.3'))
        Decimal('0.1')
        >>> ExtendedContext.remainder(Decimal('3.6'), Decimal('1.3'))
        Decimal('1.0')
        >>> ExtendedContext.remainder(22, 6)
        Decimal('4')
        >>> ExtendedContext.remainder(Decimal(22), 6)
        Decimal('4')
        >>> ExtendedContext.remainder(22, Decimal(6))
        Decimal('4')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __mod__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn remainder_near ( &self, a , b )  {
        "Returns to be "a - b * n", where n == the integer nearest the exact
        value oformat!("x / b" (if two integers are equally near then the even one
        == chosen).  If the result == equal to 0 then its sign will be the
        sign of a.

        This operation will fail under the same conditions as integer division
        (that is, if integer division on the same two operands would fail, the
        remainder cannot be calculated).

        >>> ExtendedContext.remainder_near(Decimal('2.1'), Decimal('3'))
        Decimal('-0.9')
        >>> ExtendedContext.remainder_near(Decimal('10'), Decimal('6'))
        Decimal('-2')
        >>> ExtendedContext.remainder_near(Decimal('10'), Decimal('3'))
        Decimal('1')
        >>> ExtendedContext.remainder_near(Decimal('-10'), Decimal('3'))
        Decimal('-1')
        >>> ExtendedContext.remainder_near(Decimal('10.2'), Decimal('1'))
        Decimal('0.2')
        >>> ExtendedContext.remainder_near(Decimal('10'), Decimal('0.3'))
        Decimal('0.1')
        >>> ExtendedContext.remainder_near(Decimal('3.6'), Decimal('1.3'))
        Decimal('-0.3')
        >>> ExtendedContext.remainder_near(3, 11)
        Decimal('3')
        >>> ExtendedContext.remainder_near(Decimal(3), 11)
        Decimal('3')
        >>> ExtendedContext.remainder_near(3, Decimal(11))
        Decimal('3')
        ");
        a = _convert_other ( a , raiseit = true );
        return  a . remainder_near ( b , context = self );
        pub fn rotate ( &self, a , b )  {
        "Returns a rotated copy of a, b times.

        The coefficient of the result == a rotated copy of the digits in
        the coefficient of the first operand.  The number of places of
        rotation == taken from the absolute value of the second operand,
        with the rotation being to the left if the second operand is
        positive || to the right otherwise.

        >>> ExtendedContext.rotate(Decimal('34'), Decimal('8'))
        Decimal('400000003')
        >>> ExtendedContext.rotate(Decimal('12'), Decimal('9'))
        Decimal('12')
        >>> ExtendedContext.rotate(Decimal('123456789'), Decimal('-2'))
        Decimal('891234567')
        >>> ExtendedContext.rotate(Decimal('123456789'), Decimal('0'))
        Decimal('123456789')
        >>> ExtendedContext.rotate(Decimal('123456789'), Decimal('+2'))
        Decimal('345678912')
        >>> ExtendedContext.rotate(1333333, 1)
        Decimal('13333330')
        >>> ExtendedContext.rotate(Decimal(1333333), 1)
        Decimal('13333330')
        >>> ExtendedContext.rotate(1333333, Decimal(1))
        Decimal('13333330')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . rotate ( b , context = self );
        pub fn same_quantum ( &self, a , b )  {
        "Returns true if the two operands have the same exponent.

        The result == never affected by either the sign || the coefficient of
        either operand.

        >>> ExtendedContext.same_quantum(Decimal('2.17'), Decimal('0.001'))
        false
        >>> ExtendedContext.same_quantum(Decimal('2.17'), Decimal('0.01'))
        true
        >>> ExtendedContext.same_quantum(Decimal('2.17'), Decimal('1'))
        false
        >>> ExtendedContext.same_quantum(Decimal('Inf'), Decimal('-Inf'))
        true
        >>> ExtendedContext.same_quantum(10000, -1)
        true
        >>> ExtendedContext.same_quantum(Decimal(10000), -1)
        true
        >>> ExtendedContext.same_quantum(10000, Decimal(-1))
        true
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . same_quantum ( b );
        pub fn scaleb ( &self, a , b )  {
        "Returns the first operand after adding the second value its exp.

        >>> ExtendedContext.scaleb(Decimal('7.50'), Decimal('-2'))
        Decimal('0.0750')
        >>> ExtendedContext.scaleb(Decimal('7.50'), Decimal('0'))
        Decimal('7.50')
        >>> ExtendedContext.scaleb(Decimal('7.50'), Decimal('3'))
        Decimal('7.50E+3')
        >>> ExtendedContext.scaleb(1, 4)
        Decimal('1E+4')
        >>> ExtendedContext.scaleb(Decimal(1), 4)
        Decimal('1E+4')
        >>> ExtendedContext.scaleb(1, Decimal(4))
        Decimal('1E+4')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . scaleb ( b , context = self );
        pub fn shift ( &self, a , b )  {
        "Returns a shifted copy of a, b times.

        The coefficient of the result == a shifted copy of the digits
        in the coefficient of the first operand.  The number of places
        to shift == taken from the absolute value of the second operand,
        with the shift being to the left if the second operand is
        positive || to the right otherwise.  Digits shifted into the
        coefficient are zeros.

        >>> ExtendedContext.shift(Decimal('34'), Decimal('8'))
        Decimal('400000000')
        >>> ExtendedContext.shift(Decimal('12'), Decimal('9'))
        Decimal('0')
        >>> ExtendedContext.shift(Decimal('123456789'), Decimal('-2'))
        Decimal('1234567')
        >>> ExtendedContext.shift(Decimal('123456789'), Decimal('0'))
        Decimal('123456789')
        >>> ExtendedContext.shift(Decimal('123456789'), Decimal('+2'))
        Decimal('345678900')
        >>> ExtendedContext.shift(88888888, 2)
        Decimal('888888800')
        >>> ExtendedContext.shift(Decimal(88888888), 2)
        Decimal('888888800')
        >>> ExtendedContext.shift(88888888, Decimal(2))
        Decimal('888888800')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . shift ( b , context = self );
        pub fn sqrt ( &self, a )  {
        "Square root of a non-negative number to context precision.

        If the result must be inexact, it == rounded using the round-half-even
        algorithm.

        >>> ExtendedContext.sqrt(Decimal('0'))
        Decimal('0')
        >>> ExtendedContext.sqrt(Decimal('-0'))
        Decimal('-0')
        >>> ExtendedContext.sqrt(Decimal('0.39'))
        Decimal('0.624499800')
        >>> ExtendedContext.sqrt(Decimal('100'))
        Decimal('10')
        >>> ExtendedContext.sqrt(Decimal('1'))
        Decimal('1')
        >>> ExtendedContext.sqrt(Decimal('1.0'))
        Decimal('1.0')
        >>> ExtendedContext.sqrt(Decimal('1.00'))
        Decimal('1.0')
        >>> ExtendedContext.sqrt(Decimal('7'))
        Decimal('2.64575131')
        >>> ExtendedContext.sqrt(Decimal('10'))
        Decimal('3.16227766')
        >>> ExtendedContext.sqrt(2)
        Decimal('1.41421356')
        >>> ExtendedContext.prec
        9
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . sqrt ( context = self );
        pub fn subtract ( &self, a , b )  {
        "Return the difference between the two operands.

        >>> ExtendedContext.subtract(Decimal('1.3'), Decimal('1.07'))
        Decimal('0.23')
        >>> ExtendedContext.subtract(Decimal('1.3'), Decimal('1.30'))
        Decimal('0.00')
        >>> ExtendedContext.subtract(Decimal('1.3'), Decimal('2.07'))
        Decimal('-0.77')
        >>> ExtendedContext.subtract(8, 5)
        Decimal('3')
        >>> ExtendedContext.subtract(Decimal(8), 5)
        Decimal('3')
        >>> ExtendedContext.subtract(8, Decimal(5))
        Decimal('3')
        ";
        a = _convert_other ( a , raiseit = true );
        r = a . __sub__ ( b , context = self );
        if r is NotImplemented {
        panic!("TypeError ( "Unable to convert %s to Decimal" % b )");
        } else {
        return  r;
        pub fn to_eng_string ( &self, a )  {
        "Convert to a string, using engineering notation if an exponent == needed.

        Engineering notation has an exponent which == a multiple of 3.  This
        can leave up to 3 digits to the left of the decimal place && may
        require the addition of either one || two trailing zeros.

        The operation == !affected by the context.

        >>> ExtendedContext.to_eng_string(Decimal('123E+1'))
        '1.23E+3'
        >>> ExtendedContext.to_eng_string(Decimal('123E+3'))
        '123E+3'
        >>> ExtendedContext.to_eng_string(Decimal('123E-10'))
        '12.3E-9'
        >>> ExtendedContext.to_eng_string(Decimal('-123E-12'))
        '-123E-12'
        >>> ExtendedContext.to_eng_string(Decimal('7E-7'))
        '700E-9'
        >>> ExtendedContext.to_eng_string(Decimal('7E+1'))
        '70'
        >>> ExtendedContext.to_eng_string(Decimal('0E+1'))
        '0.00E+3'

        ";
        a = _convert_other ( a , raiseit = true );
        return  a . to_eng_string ( context = self );
        pub fn to_sci_string ( &self, a )  {
        "Converts a number to a string, using scientific notation.

        The operation == !affected by the context.
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . __str__ ( context = self );
        pub fn to_integral_exact ( &self, a )  {
        "Rounds to an integer.

        When the operand has a negative exponent, the result == the same
        as using the quantize() operation using the given operand as the
        left-hand-operand, 1E+0 as the right-hand-operand, && the precision
        of the operand as the precision setting; Inexact && Rounded flags
        are allowed in this operation.  The rounding mode == taken from the
        context.

        >>> ExtendedContext.to_integral_exact(Decimal('2.1'))
        Decimal('2')
        >>> ExtendedContext.to_integral_exact(Decimal('100'))
        Decimal('100')
        >>> ExtendedContext.to_integral_exact(Decimal('100.0'))
        Decimal('100')
        >>> ExtendedContext.to_integral_exact(Decimal('101.5'))
        Decimal('102')
        >>> ExtendedContext.to_integral_exact(Decimal('-101.5'))
        Decimal('-102')
        >>> ExtendedContext.to_integral_exact(Decimal('10E+5'))
        Decimal('1.0E+6')
        >>> ExtendedContext.to_integral_exact(Decimal('7.89E+77'))
        Decimal('7.89E+77')
        >>> ExtendedContext.to_integral_exact(Decimal('-Inf'))
        Decimal('-Infinity')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . to_integral_exact ( context = self );
        pub fn to_integral_value ( &self, a )  {
        "Rounds to an integer.

        When the operand has a negative exponent, the result == the same
        as using the quantize() operation using the given operand as the
        left-hand-operand, 1E+0 as the right-hand-operand, && the precision
        of the operand as the precision setting, except that no flags will
        be set.  The rounding mode == taken from the context.

        >>> ExtendedContext.to_integral_value(Decimal('2.1'))
        Decimal('2')
        >>> ExtendedContext.to_integral_value(Decimal('100'))
        Decimal('100')
        >>> ExtendedContext.to_integral_value(Decimal('100.0'))
        Decimal('100')
        >>> ExtendedContext.to_integral_value(Decimal('101.5'))
        Decimal('102')
        >>> ExtendedContext.to_integral_value(Decimal('-101.5'))
        Decimal('-102')
        >>> ExtendedContext.to_integral_value(Decimal('10E+5'))
        Decimal('1.0E+6')
        >>> ExtendedContext.to_integral_value(Decimal('7.89E+77'))
        Decimal('7.89E+77')
        >>> ExtendedContext.to_integral_value(Decimal('-Inf'))
        Decimal('-Infinity')
        ";
        a = _convert_other ( a , raiseit = true );
        return  a . to_integral_value ( context = self );
        to_integral = to_integral_value;
        class _WorkRep ( object ) ;
        __slots__ = ( "sign" , "int" , "exp" );
        pub fn __init__ ( &self, value = None /* Option */ )  {
        if value is None /* Option */ {
        self . sign = None /* Option */;
        self . int = 0;
        self . exp = None /* Option */;
        } else if isinstance ( value , Decimal ) {
        self . sign = value . _sign;
        self . int = int ( value . _int );
        self . exp = value . _exp;
        } else {
        self . sign = value [ 0 ];
        self . int = value [ 1 ];
        self . exp = value [ 2 ];
        pub fn __repr__ ( self )  {
        return  "(%r, %r, %r)" % ( self . sign , self . int , self . exp );
        pub fn _normalize ( op1 , op2 , prec = 0 )  {
        "Normalizes op1, op2 to have the same exp && length of coefficient.

    Done during addition.
    ";
        if op1 . exp < op2 . exp {
        tmp = op2;
        other = op1;
        } else {
        tmp = op1;
        other = op2;
        tmp_len = len ( str ( tmp . int ) );
        other_len = len ( str ( other . int ) );
        exp = tmp . exp + min ( -1 , tmp_len - prec - 2 );
        if other_len + other . exp - 1 < exp {
        other . int = 1;
        other . exp = exp;
        tmp . int * = 10 ** ( tmp . exp - other . exp );
        tmp . exp = other . exp;
        return  op1 , op2;
        _nbits = int . bit_length;
        pub fn _decimal_lshift_exact ( n , e )  {
        " Given integers n && e, return n * 10**e if it's an integer, else None /* Option */.

    The computation == designed to avoid computing large powers of 10
    unnecessarily.

    >>> _decimal_lshift_exact(3, 4)
    30000
    >>> _decimal_lshift_exact(300, -999999999)  # returns None /* Option */

    ";
        if n == 0 {
        return  0;
        } else if e >= 0 {
        return  n * 10 ** e;
        } else {
        str_n = str ( abs ( n ) );
        val_n = len ( str_n ) - len ( str_n . rstrip ( "0" ) );
        return  None /* Option */ if val_n < - e else n / / 10 ** - e;
        pub fn _sqrt_nearest ( n , a )  {
        "Closest integer to the square root of the positive integer n.  a is
    an initial approximation to the square root.  Any positive integer
    will do for a, but the closer a == to the square root of n the
    faster convergence will be.

    ";
        if n <= 0 || a <= 0 {
        panic!("ValueError ( "Both arguments to _sqrt_nearest should be positive." )");
        b = 0;
        while a != b  {
        b , a = a , a - - n / / a > > 1;
        return  a;
        pub fn _rshift_nearest ( x , shift )  {
        "Given an integer x && a nonnegative integer shift, return closest
    integer to x / 2**shift; use round-to-even in case of a tie.

    ";
        b , q = 1 < < shift , x > > shift;
        return  q + ( 2 * ( x & ( b -1 ) ) + ( q & 1 ) > b );
        pub fn _div_nearest ( a , b )  {
        "Closest integer to a/b, a && b positive integers; rounds to even
    in the case of a tie.

    ";
        q , r = divmod ( a , b );
        return  q + ( 2 * r + ( q & 1 ) > b );
        pub fn _ilog ( x , M , L = 8 )  {
        "Integer approximation to M*log(x/M), with absolute error boundable
    in terms only of x/M.

    Given positive integers x && M, return an integer approximation to
    M * log(x/M).  For L = 8 && 0.1 <= x/M <= 10 the difference
    between the approximation && the exact result == at most 22.  For
    L = 8 && 1.0 <= x/M <= 10.0 the difference == at most 15.  In
    both cases these are upper bounds on the error; it will usually be
    much smaller.";
        y = x - M;
        R = 0;
        while ( R <= L && abs ( y ) < < L - R >= M or {
        R > L && abs ( y ) > > R - L >= M ) ;
        y = _div_nearest ( ( M * y ) < < 1 ,;
        M + _sqrt_nearest ( M * ( M + _rshift_nearest ( y , R ) ) , M ) );
        R + = 1;
        T = - int ( -10 * len ( str ( M ) ) / / ( 3 * L ) );
        yshift = _rshift_nearest ( y , R );
        w = _div_nearest ( M , T );
        for k in range ( T -1 , 0 , -1 ) .iter() {
        w = _div_nearest ( M , k ) - _div_nearest ( yshift * w , M );
        return  _div_nearest ( w * y , M );
        pub fn _dlog10 ( c , e , p )  {
        "Given integers c, e && p with c > 0, p >= 0, compute an integer
    approximation to 10**p * log10(c*10**e), with an absolute error of
    at most 1.  Assumes that c*10**e == !exactly 1.";
        p + = 2;
        l = len ( str ( c ) );
        f = e + l - ( e + l >= 1 );
        if p > 0 {
        M = 10 ** p;
        k = e + p - f;
        if k >= 0 {
        c * = 10 ** k;
        } else {
        c = _div_nearest ( c , 10 ** - k );
        log_d = _ilog ( c , M );
        log_10 = _log10_digits ( p );
        log_d = _div_nearest ( log_d * M , log_10 );
        log_tenpower = f * M;
        } else {
        log_d = 0;
        log_tenpower = _div_nearest ( f , 10 ** - p );
        return  _div_nearest ( log_tenpower + log_d , 100 );
        pub fn _dlog ( c , e , p )  {
        "Given integers c, e && p with c > 0, compute an integer
    approximation to 10**p * log(c*10**e), with an absolute error of
    at most 1.  Assumes that c*10**e == !exactly 1.";
        p + = 2;
        l = len ( str ( c ) );
        f = e + l - ( e + l >= 1 );
        if p > 0 {
        k = e + p - f;
        if k >= 0 {
        c * = 10 ** k;
        } else {
        c = _div_nearest ( c , 10 ** - k );
        log_d = _ilog ( c , 10 ** p );
        } else {
        log_d = 0;
        if f {
        extra = len ( str ( abs ( f ) ) ) -1;
        if p + extra >= 0 {
        f_log_ten = _div_nearest ( f * _log10_digits ( p + extra ) , 10 ** extra );
        } else {
        f_log_ten = 0;
        } else {
        f_log_ten = 0;
        return  _div_nearest ( f_log_ten + log_d , 100 );
        class _Log10Memoize ( object ) ;
        "Class to compute, store, && allow retrieval of, digits of the
    constant log(10) = 2.302585....  This constant == needed by
    Decimal.ln, Decimal.log10, Decimal.exp && Decimal.__pow__.";
        pub fn __init__ ( self )  {
        self . digits = "23025850929940456840179914546843642076011014886";
        pub fn getdigits ( &self, p )  {
        "Given an integer p >= 0, return floor(10**p)*log(10).

        For example, self.getdigits(3) returns 2302.
        ";
        if p < 0 {
        panic!("ValueError ( "p should be nonnegative" )");
        if p >= len ( self . digits ) {
        extra = 3;
        while true  {
        M = 10 ** ( p + extra + 2 );
        digits = str ( _div_nearest ( _ilog ( 10 * M , M ) , 100 ) );
        if digits [ - extra { : ] != "0" * extra ; }
        break;
        extra + = 3;
        self . digits = digits . rstrip ( "0" ) [ : -1 ];
        return  int ( self . digits [ : p + 1 ] );
        _log10_digits = _Log10Memoize ( ) . getdigits;
        pub fn _iexp ( x , M , L = 8 )  {
        "Given integers x && M, M > 0, such that x/M == small in absolute
    value, compute an integer approximation to M*exp(x/M).  For 0 <=
    x/M <= 2.4, the absolute error in the result == bounded by 60 (and
    == usually much smaller).";
        R = _nbits ( ( x < < L ) / / M );
        T = - int ( -10 * len ( str ( M ) ) / / ( 3 * L ) );
        y = _div_nearest ( x , T );
        Mshift = M < < R;
        for i in range ( T -1 , 0 , -1 ) .iter() {
        y = _div_nearest ( x * ( Mshift + y ) , Mshift * i );
        for k in range ( R -1 , -1 , -1 ) .iter() {
        Mshift = M < < ( k + 2 );
        y = _div_nearest ( y * ( y + Mshift ) , Mshift );
        return  M + y;
        pub fn _dexp ( c , e , p )  {
        "Compute an approximation to exp(c*10**e), with p decimal places of
    precision.

    Returns integers d, f such that:

      10**(p-1) <= d <= 10**p, and
      (d-1)*10**f < exp(c*10**e) < (d+1)*10**f

    In other words, d*10**f == an approximation to exp(c*10**e) with p
    digits of precision, && with an error in d of at most 1.  This is
    almost, but !quite, the same as the error being < 1ulp: when d
    = 10**(p-1) the error could be up to 10 ulp.";
        p + = 2;
        extra = max ( 0 , e + len ( str ( c ) ) - 1 );
        q = p + extra;
        shift = e + q;
        if shift >= 0 {
        cshift = c * 10 ** shift;
        } else {
        cshift = c / / 10 ** - shift;
        quot , rem = divmod ( cshift , _log10_digits ( q ) );
        rem = _div_nearest ( rem , 10 ** extra );
        return  _div_nearest ( _iexp ( rem , 10 ** p ) , 1000 ) , quot - p + 3;
        pub fn _dpower ( xc , xe , yc , ye , p )  {
        "Given integers xc, xe, yc && ye representing Decimals x = xc*10**xe and
    y = yc*10**ye, compute x**y.  Returns a pair of integers (c, e) such that:

      10**(p-1) <= c <= 10**p, and
      (c-1)*10**e < x**y < (c+1)*10**e

    in other words, c*10**e == an approximation to x**y with p digits
    of precision, && with an error in c of at most 1.  (This is
    almost, but !quite, the same as the error being < 1ulp: when c
    == 10**(p-1) we can only guarantee error < 10ulp.)

    We assume that: x == positive && !equal to 1, && y == nonzero.
    ";
        b = len ( str ( abs ( yc ) ) ) + ye;
        lxc = _dlog ( xc , xe , p + b + 1 );
        shift = ye - b;
        if shift >= 0 {
        pc = lxc * yc * 10 ** shift;
        } else {
        pc = _div_nearest ( lxc * yc , 10 ** - shift );
        if pc == 0 {
        if ( ( len ( str ( xc ) ) + xe >= 1 ) == ( yc > 0 ) ) {
        coeff , exp = 10 ** ( p -1 ) + 1 , 1 - p;
        } else {
        coeff , exp = 10 ** p -1 , - p;
        } else {
        coeff , exp = _dexp ( pc , - ( p + 1 ) , p + 1 );
        coeff = _div_nearest ( coeff , 10 );
        exp + = 1;
        return  coeff , exp;
        pub fn _log10_lb ( c , correction = { {
        "1" : 100 , "2" : 70 , "3" : 53 , "4" : 40 , "5" : 31 ,;
        "6" : 23 , "7" : 16 , "8" : 10 , "9" : 5 } ) ;
        "Compute a lower bound for 100*log10(c) for a positive integer c.";
        if c <= 0 {
        panic!("ValueError ( "The argument to _log10_lb should be nonnegative." )");
        str_c = str ( c );
        return  100 * len ( str_c ) - correction [ str_c [ 0 ] ];
        pub fn _convert_other ( other , raiseit = false , allow_float = false )  {
        "Convert other to Decimal.

    Verifies that it's ok to use in an implicit construction.
    If allow_float == true, allow conversion from float;  this
    == used in the comparison methods (__eq__ && friends).

    ";
        if isinstance ( other , Decimal ) {
        return  other;
        if isinstance ( other , int ) {
        return  Decimal ( other );
        if allow_float && isinstance ( other , float ) {
        return  Decimal . from_float ( other );
        if raiseit {
        panic!("TypeError ( "Unable to convert %s to Decimal" % other )");
        return  NotImplemented;
        pub fn _convert_for_comparison ( &self, other , equality_op = false )  {
        "Given a Decimal instance self && a Python object other, return
    a pair (s, o) of Decimal instances such that "s op o" is
    equivalent to "self op other" for any of the 6 comparison
    operators "op".

    ";
        if isinstance ( other , Decimal ) {
        return  self , other;
        if isinstance ( other , _numbers . Rational ) {
        if !self . _is_special {
        self = _dec_from_triple ( self . _sign ,;
        str ( int ( self . _int ) * other . denominator ) ,;
        self . _exp );
        return  self , Decimal ( other . numerator );
        if equality_op && isinstance ( other , _numbers . Complex ) && other . imag == 0 {
        other = other . real;
        if isinstance ( other , float ) {
        context = getcontext ( );
        if equality_op {
        context . flags [ FloatOperation ] = 1;
        } else {
        context . _raise_error ( FloatOperation ,;
        "strict semantics for mixing floats && Decimals are enabled" );
        return  self , Decimal . from_float ( other );
        return  NotImplemented , NotImplemented;
        DefaultContext = Context (;
        prec = 28 , rounding = ROUND_HALF_EVEN ,;
        traps = [ DivisionByZero , Overflow , InvalidOperation ] ,;
        flags = [ ] ,;
        Emax = 999999 ,;
        Emin = -999999 ,;
        capitals = 1 ,;
        clamp = 0;
        );
        BasicContext = Context (;
        prec = 9 , rounding = ROUND_HALF_UP ,;
        traps = [ DivisionByZero , Overflow , InvalidOperation , Clamped , Underflow ] ,;
        flags = [ ] ,;
        );
        ExtendedContext = Context (;
        prec = 9 , rounding = ROUND_HALF_EVEN ,;
        traps = [ ] ,;
        flags = [ ] ,;
        );
        import re;
        _parser = re . compile ( r "        # A numeric string consists of:
#    \s*
    (?P<sign>[-+])?              # an optional sign, followed by either...
    (
        (?=\d|\.\d)              # ...a number (with at least one digit)
        (?P<int>\d*)             # having a (possibly empty) integer part
        (\.(?P<frac>\d*))?       # followed by an optional fractional part
        (E(?P<exp>[-+]?\d+))?    # followed by an optional exponent, or...
    |
        Inf(inity)?              # ...an infinity, or...
    |
        (?P<signal>s)?           # ...an (optionally signaling)
        NaN                      # NaN
        (?P<diag>\d*)            # with (possibly empty) diagnostic info.
    )
#    \s*
    \Z
" , re . VERBOSE | re . IGNORECASE ) . match;
        _all_zeros = re . compile ( "0*$" ) . match;
        _exact_half = re . compile ( "50*$" ) . match;
        _parse_format_specifier_regex = re . compile ( r "\A
(?:
   (?P<fill>.)?
   (?P<align>[<>=^])
)?
(?P<sign>[-+ ])?
(?P<no_neg_0>z)?
(?P<alt>\#)?
(?P<zeropad>0)?
(?P<minimumwidth>(?!0)\d+)?
(?P<thousands_sep>,)?
(?:\.(?P<precision>0|(?!0)\d+))?
(?P<type>[eEfFgGn%])?
\Z
" , re . VERBOSE | re . DOTALL );
        del re;
        // try {
        import locale as _locale;
        // } catch  ImportError  {
        // pass
        pub fn _parse_format_specifier ( format_spec , _localeconv = None /* Option */ )  {
        "Parse && validate a format specifier.

    Turns a standard numeric format specifier into a dict, with the
    following entries:

      fill: fill character to pad field to minimum width
      align: alignment type, either '<', '>', '=' || '^'
      sign: either '+', '-' || ' '
      minimumwidth: nonnegative integer giving minimum width
      zeropad: boolean, indicating whether to pad with zeros
      thousands_sep: string to use as thousands separator, || ''
      grouping: grouping for thousands separators, in format
        used by localeconv
      decimal_point: string to use for decimal point
      precision: nonnegative integer giving precision, || None /* Option */
      type: one of the characters 'eEfFgG%', || None /* Option */

    ";
        m = _parse_format_specifier_regex . match ( format_spec );
        if m is None /* Option */ {
        panic!("ValueError ( "Invalid format specifier: " + format_spec )");
        format_dict = m . groupdict ( );
        fill = format_dict [ "fill" ];
        align = format_dict [ "align" ];
        format_dict [ "zeropad" ] = ( format_dict [ "zeropad" ] == !None /* Option */ );
        if format_dict [ "zeropad" ] {
        if fill is !None /* Option */ {
        panic!("ValueError ( "Fill character conflicts with '0'"");
        " in format specifier: " + format_spec );
        if align is !None /* Option */ {
        panic!("ValueError ( "Alignment conflicts with '0' in "");
        "format specifier: " + format_spec );
        format_dict [ "fill" ] = fill || " ";
        format_dict [ "align" ] = align || ">";
        if format_dict [ "sign" ] is None /* Option */ {
        format_dict [ "sign" ] = "-";
        format_dict [ "minimumwidth" ] = int ( format_dict [ "minimumwidth" ] || "0" );
        if format_dict [ "precision" ] is !None /* Option */ {
        format_dict [ "precision" ] = int ( format_dict [ "precision" ] );
        if format_dict [ "precision" ] == 0 {
        if format_dict [ "type" ] is None /* Option */ || format_dict [ "type" ] in "gGn" {
        format_dict [ "precision" ] = 1;
        if format_dict [ "type" ] == "n" {
        format_dict [ "type" ] = "g";
        if _localeconv is None /* Option */ {
        _localeconv = _locale . localeconv ( );
        if format_dict [ "thousands_sep" ] is !None /* Option */ {
        panic!("ValueError ( "Explicit thousands separator conflicts with "");
        "'n' type in format specifier: " + format_spec );
        format_dict [ "thousands_sep" ] = _localeconv [ "thousands_sep" ];
        format_dict [ "grouping" ] = _localeconv [ "grouping" ];
        format_dict [ "decimal_point" ] = _localeconv [ "decimal_point" ];
        } else {
        if format_dict [ "thousands_sep" ] is None /* Option */ {
        format_dict [ "thousands_sep" ] = "";
        format_dict [ "grouping" ] = [ 3 , 0 ];
        format_dict [ "decimal_point" ] = ".";
        return  format_dict;
        pub fn _format_align ( sign , body , spec )  {
        "Given an unpadded, non-aligned numeric string 'body' && sign
    string 'sign', add padding && alignment conforming to the given
    format specifier dictionary 'spec' (as produced by
    parse_format_specifier).

    ";
        minimumwidth = spec [ "minimumwidth" ];
        fill = spec [ "fill" ];
        padding = fill * ( minimumwidth - len ( sign ) - len ( body ) );
        align = spec [ "align" ];
        if align == "<" {
        result = sign + body + padding;
        } else if align == ">" {
        result = padding + sign + body;
        } else if align == "=" {
        result = sign + padding + body;
        } else if align == "^" {
        half = len ( padding ) / / 2;
        result = padding [ : half ] + sign + body + padding [ half : ];
        } else {
        panic!("ValueError ( "Unrecognised alignment field" )");
        return  result;
        pub fn _group_lengths ( grouping )  {
        "Convert a localeconv-style grouping into a (possibly infinite)
    iterable of integers representing group lengths.

    ";
        from itertools import chain , repeat;
        if !grouping {
        return  [ ];
        } else if grouping [ -1 ] == 0 && len ( grouping ) >= 2 {
        return  chain ( grouping [ : -1 ] , repeat ( grouping [ -2 ] ) );
        } else if grouping [ -1 ] == _locale . CHAR_MAX {
        return  grouping [ : -1 ];
        } else {
        panic!("ValueError ( "unrecognised format for grouping" )");
        pub fn _insert_thousands_sep ( digits , spec , min_width = 1 )  {
        "Insert thousands separators into a digit string.

    spec == a dictionary whose keys should include 'thousands_sep' and
    'grouping'; typically it's the result of parsing the format
    specifier using _parse_format_specifier.

    The min_width keyword argument gives the minimum length of the
    result, which will be padded on the left with zeros if necessary.

    If necessary, the zero padding adds an extra '0' on the left to
    avoid a leading thousands separator.  For example, inserting
    commas every three digits in '123456', with min_width=8, gives
    '0,123,456', even though that has length 9.

    ";
        sep = spec [ "thousands_sep" ];
        grouping = spec [ "grouping" ];
        groups = [ ];
        for l in _group_lengths ( grouping ) .iter() {
        if l <= 0 {
        panic!("ValueError ( "group length should be positive" )");
        l = min ( max ( len ( digits ) , min_width , 1 ) , l );
        groups . append ( "0" * ( l - len ( digits ) ) + digits [ - l : ] );
        digits = digits [ : - l ];
        min_width - = l;
        if !digits && min_width <= 0 {
        break;
        min_width - = len ( sep );
        } else {
        l = max ( len ( digits ) , min_width , 1 );
        groups . append ( "0" * ( l - len ( digits ) ) + digits [ - l : ] );
        return  sep . join ( reversed ( groups ) );
        pub fn _format_sign ( is_negative , spec )  {
        "Determine sign character.";
        if is_negative {
        return  "-";
        } else if spec [ "sign" ] in " +" {
        return  spec [ "sign" ];
        } else {
        return  "";
        pub fn _format_number ( is_negative , intpart , fracpart , exp , spec )  {
        "Format a number, given the following data:

    is_negative: true if the number == negative, else false
    intpart: string of digits that must appear before the decimal point
    fracpart: string of digits that must come after the point
    exp: exponent, as an integer
    spec: dictionary resulting from parsing the format specifier

    This function uses the information in spec to:
      insert separators (decimal separator && thousands separators)
      format the sign
      format the exponent
      add trailing '%' for the '%' type
      zero-pad if necessary
      fill && align if necessary
    ";
        sign = _format_sign ( is_negative , spec );
        if fracpart || spec [ "alt" ] {
        fracpart = spec [ "decimal_point" ] + fracpart;
        if exp != 0 || spec [ "type" ] in "eE" {
        echar = { "E" : "E" , "e" : "e" , "G" : "E" , "g" : "e" } [ spec [ "type" ] ];
        fracpart + = "{0}{1:+}" . format ( echar , exp );
        if spec [ "type" ] == "%" {
        fracpart + = "%";
        if spec [ "zeropad" ] {
        min_width = spec [ "minimumwidth" ] - len ( fracpart ) - len ( sign );
        } else {
        min_width = 0;
        intpart = _insert_thousands_sep ( intpart , spec , min_width );
        return  _format_align ( sign , intpart + fracpart , spec );
        _Infinity = Decimal ( "Informat!(" ));
        _NegativeInfinity = Decimal ( "-Informat!(" ));
        _NaN = Decimal ( "NaN" );
        _Zero = Decimal ( 0 );
        _One = Decimal ( 1 );
        _NegativeOne = Decimal ( -1 );
        _SignedInfinity = ( _Infinity , _NegativeInfinity );
        _PyHASH_MODULUS = sys . hash_info . modulus;
        _PyHASH_INF = sys . hash_info . inf;
        _PyHASH_NAN = sys . hash_info . nan;
        _PyHASH_10INV = pow ( 10 , _PyHASH_MODULUS - 2 , _PyHASH_MODULUS );
        del sys;
    }

}

