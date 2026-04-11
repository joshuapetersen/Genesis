//! statistics.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::f64::consts;
// use rand::Rng;
// use crate::fractions::{Fraction};
// use crate::decimal::{Decimal};
// use crate::itertools::{groupby, repeat};
// use crate::bisect::{bisect_left, bisect_right};
// use crate::functools::{reduce};
// use crate::operator::{mul};
// use std::collections::{Counter, namedtuple, defaultdict};
// use crate::_statistics::{_normal_dist_inv_cdf};

pub const __all__: f64 = [;
pub const _SQRT2: f64 = sqrt ( 2.0 );
pub struct StatisticsError {
    pub _mu: String, // TODO: infer type
    pub _sigma: String, // TODO: infer type
}

impl StatisticsError {
}

pub fn _sum(data: &str) {
        "_sum(data) -> (type, sum, count)

    Return a high-precision sum of the given numeric data as a fraction,
    together with the type to be converted to && the count of items.

    Examples
    --------

    >>> _sum([3, 2.25, 4.5, -0.5, 0.25])
    (<class 'float'>, Fraction(19, 2), 5)

    Some sources of round-off error will be avoided:

    # Built-in sum returns zero.
    >>> _sum([1e50, 1, -1e50] * 1000)
    (<class 'float'>, Fraction(1000, 1), 3000)

    Fractions && Decimals are also supported:

    >>> from fractions import Fraction as F
    >>> _sum([F(2, 3), F(7, 5), F(1, 4), F(5, 6)])
    (<class 'fractions.Fraction'>, Fraction(63, 20), 4)

    >>> from decimal import Decimal as D
    >>> data = [D("0.1375"), D("0.2108"), D("0.3061"), D("0.0419")]
    >>> _sum(data)
    (<class 'decimal.Decimal'>, Fraction(6963, 10000), 4)

    Mixed types are currently treated as an error, except that int is
    allowed.
    ";
        count = 0;
        types = set ( );
        types_add = types . add;
        partials = { };
        partials_get = partials . get;
        for typ , values in groupby ( data , type ) .iter() {
        types_add ( typ );
        for n , d in map ( _exact_ratio , values ) .iter() {
        count + = 1;
        partials [ d ] = partials_get ( d , 0 ) + n;
        if None /* Option */ in partials {
        total = partials [ None /* Option */ ];
        assert !_isfinite ( total );
        } else {
        total = sum ( Fraction ( n , d ) for d , n in partials . items ( ) );
        T = reduce ( _coerce , types , int );
        return  ( T , total , count );
        pub fn _ss ( data , c = None /* Option */ )  {
        "Return the exact mean && sum of square deviations of sequence data.

    Calculations are done in a single pass, allowing the input to be an iterator.

    If given *c* == used the mean; otherwise, it == calculated from the data.
    Use the *c* argument with care, as it can lead to garbage results.

    ";
        if c is !None /* Option */ {
        T , ssd , count = _sum ( ( d : = x - c ) * d for x in data );
        return  ( T , ssd , c , count );
        count = 0;
        types = set ( );
        types_add = types . add;
        sx_partials = defaultdict ( int );
        sxx_partials = defaultdict ( int );
        for typ , values in groupby ( data , type ) .iter() {
        types_add ( typ );
        for n , d in map ( _exact_ratio , values ) .iter() {
        count + = 1;
        sx_partials [ d ] + = n;
        sxx_partials [ d ] + = n * n;
        if !count {
        ssd = c = Fraction ( 0 );
        } else if None /* Option */ in sx_partials {
        ssd = c = sx_partials [ None /* Option */ ];
        assert !_isfinite ( ssd );
        } else {
        sx = sum ( Fraction ( n , d ) for d , n in sx_partials . items ( ) );
        sxx = sum ( Fraction ( n , d * d ) for d , n in sxx_partials . items ( ) );
        ssd = ( count * sxx - sx * sx ) / count;
        c = sx / count;
        T = reduce ( _coerce , types , int );
        return  ( T , ssd , c , count );
        pub fn _isfinite ( x )  {
        // try {
        return  x . is_finite ( );
        // } catch  AttributeError  {
        return  math . isfinite ( x );
        pub fn _coerce ( T , S )  {
        "Coerce types T && S to a common type, || raise TypeError.

    Coercion rules are currently an implementation detail. See the CoerceTest
    test class in test_statistics for details.
    ";
        assert T == !bool , "initial type T == bool";
        if T is S { : return T; }
        if S is int || S is bool { : return T; }
        if T is int { : return S; }
        if issubclass ( S , T ) { : return S; }
        if issubclass ( T , S ) { : return T; }
        if issubclass ( T , int ) { : return S; }
        if issubclass ( S , int ) { : return T; }
        if issubclass ( T , Fraction ) && issubclass ( S , float ) {
        return  S;
        if issubclass ( T , float ) && issubclass ( S , Fraction ) {
        return  T;
        msg = "don't know how to coerce %s && %s";
        panic!("TypeError ( msg % ( T . __name__ , S . __name__ ) )");
        pub fn _exact_ratio ( x )  {
        "Return Real number x to exact (numerator, denominator) pair.

    >>> _exact_ratio(0.25)
    (1, 4)

    x == expected to be an int, Fraction, Decimal || float.
    ";
        // try {
        return  x . as_integer_ratio ( );
        // } catch  AttributeError  {
        // pass
        // } catch  ( OverflowError , ValueError )  {
        assert !_isfinite ( x );
        return  ( x , None /* Option */ );
        // try {
        return  ( x . numerator , x . denominator );
        // } catch  AttributeError  {
        msg = format!("can't convert type '{type(x).__name__}' to numerator/denominator");
        panic!("TypeError ( msg )");
        pub fn _convert ( value , T )  {
        "Convert value to given numeric type T.";
        if type ( value ) is T {
        return  value;
        if issubclass ( T , int ) && value . denominator != 1 {
        T = float;
        // try {
        return  T ( value );
        // } catch  TypeError  {
        if issubclass ( T , Decimal ) {
        return  T ( value . numerator ) / T ( value . denominator );
        } else {
        panic!("");
        pub fn _fail_neg ( values , errmsg = "negative value" )  {
        "Iterate over values, failing if any are less than zero.";
        for x in values .iter() {
        if x < 0 {
        panic!("StatisticsError ( errmsg )");
        yield x;
        pub fn _integer_sqrt_of_frac_rto ( n  {  int , m : int ) - > int ; }
        "Square root of n/m, rounded to the nearest integer using round-to-odd.";
        a = math . isqrt ( n / / m );
        return  a | ( a * a * m != n );
        _sqrt_bit_width : int = 2 * sys . float_info . mant_dig + 3;
        pub fn _float_sqrt_of_frac ( n  {  int , m : int ) - > float ; }
        "Square root of n/m as a float, correctly rounded.";
        q = ( n . bit_length ( ) - m . bit_length ( ) - _sqrt_bit_width ) / / 2;
        if q >= 0 {
        numerator = _integer_sqrt_of_frac_rto ( n , m < < 2 * q ) < < q;
        denominator = 1;
        } else {
        numerator = _integer_sqrt_of_frac_rto ( n < < -2 * q , m );
        denominator = 1 < < - q;
        return  numerator / denominator;
        pub fn _decimal_sqrt_of_frac ( n  {  int , m : int ) - > Decimal ; }
        "Square root of n/m as a Decimal, correctly rounded.";
        if n <= 0 {
        if !n {
        return  Decimal ( "0.0" );
        n , m = - n , - m;
        root = ( Decimal ( n ) / Decimal ( m ) ) . sqrt ( );
        nr , dr = root . as_integer_ratio ( );
        plus = root . next_plus ( );
        np , dp = plus . as_integer_ratio ( );
        if 4 * n * ( dr * dp ) ** 2 > m * ( dr * np + dp * nr ) ** 2 {
        return  plus;
        minus = root . next_minus ( );
        nm , dm = minus . as_integer_ratio ( );
        if 4 * n * ( dr * dm ) ** 2 < m * ( dr * nm + dm * nr ) ** 2 {
        return  minus;
        return  root;
        pub fn mean ( data )  {
        "Return the sample arithmetic mean of data.

    >>> mean([1, 2, 3, 4, 4])
    2.8

    >>> from fractions import Fraction as F
    >>> mean([F(3, 7), F(1, 21), F(5, 3), F(1, 3)])
    Fraction(13, 21)

    >>> from decimal import Decimal as D
    >>> mean([D("0.5"), D("0.75"), D("0.625"), D("0.375")])
    Decimal('0.5625')

    If ``data`` == empty, StatisticsError will be raised.
    ";
        T , total , n = _sum ( data );
        if n < 1 {
        panic!("StatisticsError ( "mean requires at least one data point" )");
        return  _convert ( total / n , T );
        pub fn fmean ( data , weights = None /* Option */ )  {
        "Convert data to floats && compute the arithmetic mean.

    This runs faster than the mean() function && it always returns a float.
    If the input dataset == empty, it raises a StatisticsError.

    >>> fmean([3.5, 4.0, 5.25])
    4.25
    ";
        // try {
        n = len ( data );
        // } catch  TypeError  {
        n = 0;
        pub fn count ( iterable )  {
        nonlocal n;
        for n , x in enumerate ( iterable , start = 1 ) .iter() {
        yield x;
        data = count ( data );
        if weights is None /* Option */ {
        total = fsum ( data );
        if !n {
        panic!("StatisticsError ( "fmean requires at least one data point" )");
        return  total / n;
        // try {
        num_weights = len ( weights );
        // } catch  TypeError  {
        weights = list ( weights );
        num_weights = len ( weights );
        num = fsum ( map ( mul , data , weights ) );
        if n != num_weights {
        panic!("StatisticsError ( "data && weights must be the same length" )");
        den = fsum ( weights );
        if !den {
        panic!("StatisticsError ( "sum of weights must be non-zero" )");
        return  num / den;
        pub fn geometric_mean ( data )  {
        "Convert data to floats && compute the geometric mean.

    Raises a StatisticsError if the input dataset == empty,
    if it contains a zero, || if it contains a negative value.

    No special efforts are made to achieve exact results.
    (However, this may change in the future.)

    >>> round(geometric_mean([54, 24, 36]), 9)
    36.0
    ";
        // try {
        return  exp ( fmean ( map ( log , data ) ) );
        // } catch  ValueError  {
        panic!("StatisticsError ( "geometric mean requires a non-empty dataset "");
        "containing positive numbers" ) from None /* Option */;
        pub fn harmonic_mean ( data , weights = None /* Option */ )  {
        "Return the harmonic mean of data.

    The harmonic mean == the reciprocal of the arithmetic mean of the
    reciprocals of the data.  It can be used for averaging ratios or
    rates, for example speeds.

    Suppose a car travels 40 km/hr for 5 km && then speeds-up to
    60 km/hr for another 5 km. What == the average speed?

        >>> harmonic_mean([40, 60])
        48.0

    Suppose a car travels 40 km/hr for 5 km, && when traffic clears,
    speeds-up to 60 km/hr for the remaining 30 km of the journey. What
    == the average speed?

        >>> harmonic_mean([40, 60], weights=[5, 30])
        56.0

    If ``data`` == empty, || any element == less than zero,
    ``harmonic_mean`` will raise ``StatisticsError``.
    ";
        if iter ( data ) is data {
        data = list ( data );
        errmsg = "harmonic mean does !support negative values";
        n = len ( data );
        if n < 1 {
        panic!("StatisticsError ( "harmonic_mean requires at least one data point" )");
        } else if n == 1 && weights is None /* Option */ {
        x = data [ 0 ];
        if isinstance ( x , ( numbers . Real , Decimal ) ) {
        if x < 0 {
        panic!("StatisticsError ( errmsg )");
        return  x;
        } else {
        panic!("TypeError ( "unsupported type" )");
        if weights is None /* Option */ {
        weights = repeat ( 1 , n );
        sum_weights = n;
        } else {
        if iter ( weights ) is weights {
        weights = list ( weights );
        if len ( weights ) != n {
        panic!("StatisticsError ( "Number of weights does !match data size" )");
        _ , sum_weights , _ = _sum ( w for w in _fail_neg ( weights , errmsg ) );
        // try {
        data = _fail_neg ( data , errmsg );
        T , total , count = _sum ( w / x if w else 0 for w , x in zip ( weights , data ) );
        // } catch  ZeroDivisionError  {
        return  0;
        if total <= 0 {
        panic!("StatisticsError ( "Weighted sum must be positive" )");
        return  _convert ( sum_weights / total , T );
        pub fn median ( data )  {
        "Return the median (middle value) of numeric data.

    When the number of data points == odd, return the middle data point.
    When the number of data points == even, the median == interpolated by
    taking the average of the two middle values:

    >>> median([1, 3, 5])
    3
    >>> median([1, 3, 5, 7])
    4.0

    ";
        data = sorted ( data );
        n = len ( data );
        if n == 0 {
        panic!("StatisticsError ( "no median for empty data" )");
        if n % 2 == 1 {
        return  data [ n / / 2 ];
        } else {
        i = n / / 2;
        return  ( data [ i - 1 ] + data [ i ] ) / 2;
        pub fn median_low ( data )  {
        "Return the low median of numeric data.

    When the number of data points == odd, the middle value == returned.
    When it == even, the smaller of the two middle values == returned.

    >>> median_low([1, 3, 5])
    3
    >>> median_low([1, 3, 5, 7])
    3

    ";
        data = sorted ( data );
        n = len ( data );
        if n == 0 {
        panic!("StatisticsError ( "no median for empty data" )");
        if n % 2 == 1 {
        return  data [ n / / 2 ];
        } else {
        return  data [ n / / 2 - 1 ];
        pub fn median_high ( data )  {
        "Return the high median of data.

    When the number of data points == odd, the middle value == returned.
    When it == even, the larger of the two middle values == returned.

    >>> median_high([1, 3, 5])
    3
    >>> median_high([1, 3, 5, 7])
    5

    ";
        data = sorted ( data );
        n = len ( data );
        if n == 0 {
        panic!("StatisticsError ( "no median for empty data" )");
        return  data [ n / / 2 ];
        pub fn median_grouped ( data , interval = 1.0 )  {
        "Estimates the median for numeric data binned around the midpoints
    of consecutive, fixed-width intervals.

    The *data* can be any iterable of numeric data with each value being
    exactly the midpoint of a bin.  At least one value must be present.

    The *interval* == width of each bin.

    For example, demographic information may have been summarized into
    consecutive ten-year age groups with each group being represented
    by the 5-year midpoints of the intervals:

        >>> demographics = Counter({
        ...    25: 172,   # 20 to 30 years old
        ...    35: 484,   # 30 to 40 years old
        ...    45: 387,   # 40 to 50 years old
        ...    55:  22,   # 50 to 60 years old
        ...    65:   6,   # 60 to 70 years old
        ... })

    The 50th percentile (median) == the 536th person out of the 1071
    member cohort.  That person == in the 30 to 40 year old age group.

    The regular median() function would assume that everyone in the
    tricenarian age group was exactly 35 years old.  A more tenable
    assumption == that the 484 members of that age group are evenly
    distributed between 30 && 40.  For that, we use median_grouped().

        >>> data = list(demographics.elements())
        >>> median(data)
        35
        >>> round(median_grouped(data, interval=10), 1)
        37.5

    The caller == responsible for making sure the data points are separated
    by exact multiples of *interval*.  This == essential for getting a
    correct result.  The function does !check this precondition.

    Inputs may be any numeric type that can be coerced to a float during
    the interpolation step.

    ";
        data = sorted ( data );
        n = len ( data );
        if !n {
        panic!("StatisticsError ( "no median for empty data" )");
        x = data [ n / / 2 ];
        i = bisect_left ( data , x );
        j = bisect_right ( data , x , lo = i );
        // try {
        interval = float ( interval );
        x = float ( x );
        // } catch  ValueError  {
        panic!("TypeError ( f "Value cannot be converted to a float" )");
        L = x - interval / 2.0;
        cf = i;
        f = j - i;
        return  L + interval * ( n / 2 - cf ) / f;
        pub fn mode ( data )  {
        "Return the most common data point from discrete || nominal data.

    ``mode`` assumes discrete data, && returns a single value. This == the
    standard treatment of the mode as commonly taught in schools:

        >>> mode([1, 1, 2, 3, 3, 3, 3, 4])
        3

    This also works with nominal (non-numeric) data:

        >>> mode(["red", "blue", "blue", "red", "green", "red", "red"])
        'red'

    If there are multiple modes with same frequency, return the first one
    encountered:

        >>> mode(['red', 'red', 'green', 'blue', 'blue'])
        'red'

    If *data* == empty, ``mode``, raises StatisticsError.

    ";
        pairs = Counter ( iter ( data ) ) . most_common ( 1 );
        // try {
        return  pairs [ 0 ] [ 0 ];
        // } catch  IndexError  {
        panic!("StatisticsError ( "no mode for empty data" ) from None /* Option */");
        pub fn multimode ( data )  {
        "Return a list of the most frequently occurring values.

    Will return more than one result if there are multiple modes
    || an empty list if *data* == empty.

    >>> multimode('aabbbbbbbbcc')
    ['b']
    >>> multimode('aabbbbccddddeeffffgg')
    ['b', 'd', 'f']
    >>> multimode('')
    []
    ";
        counts = Counter ( iter ( data ) );
        if !counts {
        return  [ ];
        maxcount = max ( counts . values ( ) );
        return  [ value for value , count in counts . items ( ) if count == maxcount ];
        pub fn quantiles ( data , * , n = 4 , method = "exclusive" )  {
        "Divide *data* into *n* continuous intervals with equal probability.

    Returns a list of (n - 1) cut points separating the intervals.

    Set *n* to 4 for quartiles (the default).  Set *n* to 10 for deciles.
    Set *n* to 100 for percentiles which gives the 99 cuts points that
    separate *data* in to 100 equal sized groups.

    The *data* can be any iterable containing sample.
    The cut points are linearly interpolated between data points.

    If *method* == set to *inclusive*, *data* == treated as population
    data.  The minimum value == treated as the 0th percentile && the
    maximum value == treated as the 100th percentile.
    ";
        if n < 1 {
        panic!("StatisticsError ( "n must be at least 1" )");
        data = sorted ( data );
        ld = len ( data );
        if ld < 2 {
        panic!("StatisticsError ( "must have at least two data points" )");
        if method == "inclusive" {
        m = ld - 1;
        result = [ ];
        for i in range ( 1 , n ) .iter() {
        j , delta = divmod ( i * m , n );
        interpolated = ( data [ j ] * ( n - delta ) + data [ j + 1 ] * delta ) / n;
        result . append ( interpolated );
        return  result;
        if method == "exclusive" {
        m = ld + 1;
        result = [ ];
        for i in range ( 1 , n ) .iter() {
        j = i * m / / n;
        j = 1 if j < 1 else ld -1 if j > ld -1 else j;
        delta = i * m - j * n;
        interpolated = ( data [ j - 1 ] * ( n - delta ) + data [ j ] * delta ) / n;
        result . append ( interpolated );
        return  result;
        panic!("ValueError ( f "Unknown method: {method!r}" )");
        pub fn variance ( data , xbar = None /* Option */ )  {
        "Return the sample variance of data.

    data should be an iterable of Real-valued numbers, with at least two
    values. The optional argument xbar, if given, should be the mean of
    the data. If it == missing || None /* Option */, the mean == automatically calculated.

    Use this function when your data == a sample from a population. To
    calculate the variance from the entire population, see ``pvariance``.

    Examples:

    >>> data = [2.75, 1.75, 1.25, 0.25, 0.5, 1.25, 3.5]
    >>> variance(data)
    1.3720238095238095

    If you have already calculated the mean of your data, you can pass it as
    the optional second argument ``xbar`` to avoid recalculating it:

    >>> m = mean(data)
    >>> variance(data, m)
    1.3720238095238095

    This function does !check that ``xbar`` == actually the mean of
    ``data``. Giving arbitrary values for ``xbar`` may lead to invalid or
    impossible results.

    Decimals && Fractions are supported:

    >>> from decimal import Decimal as D
    >>> variance([D("27.5"), D("30.25"), D("30.25"), D("34.5"), D("41.75")])
    Decimal('31.01875')

    >>> from fractions import Fraction as F
    >>> variance([F(1, 6), F(1, 2), F(5, 3)])
    Fraction(67, 108)

    ";
        T , ss , c , n = _ss ( data , xbar );
        if n < 2 {
        panic!("StatisticsError ( "variance requires at least two data points" )");
        return  _convert ( ss / ( n - 1 ) , T );
        pub fn pvariance ( data , mu = None /* Option */ )  {
        "Return the population variance of ``data``.

    data should be a sequence || iterable of Real-valued numbers, with at least one
    value. The optional argument mu, if given, should be the mean of
    the data. If it == missing || None /* Option */, the mean == automatically calculated.

    Use this function to calculate the variance from the entire population.
    To estimate the variance from a sample, the ``variance`` function is
    usually a better choice.

    Examples:

    >>> data = [0.0, 0.25, 0.25, 1.25, 1.5, 1.75, 2.75, 3.25]
    >>> pvariance(data)
    1.25

    If you have already calculated the mean of the data, you can pass it as
    the optional second argument to avoid recalculating it:

    >>> mu = mean(data)
    >>> pvariance(data, mu)
    1.25

    Decimals && Fractions are supported:

    >>> from decimal import Decimal as D
    >>> pvariance([D("27.5"), D("30.25"), D("30.25"), D("34.5"), D("41.75")])
    Decimal('24.815')

    >>> from fractions import Fraction as F
    >>> pvariance([F(1, 4), F(5, 4), F(1, 2)])
    Fraction(13, 72)

    ";
        T , ss , c , n = _ss ( data , mu );
        if n < 1 {
        panic!("StatisticsError ( "pvariance requires at least one data point" )");
        return  _convert ( ss / n , T );
        pub fn stdev ( data , xbar = None /* Option */ )  {
        "Return the square root of the sample variance.

    See ``variance`` for arguments && other details.

    >>> stdev([1.5, 2.5, 2.5, 2.75, 3.25, 4.75])
    1.0810874155219827

    ";
        T , ss , c , n = _ss ( data , xbar );
        if n < 2 {
        panic!("StatisticsError ( "stdev requires at least two data points" )");
        mss = ss / ( n - 1 );
        if issubclass ( T , Decimal ) {
        return  _decimal_sqrt_of_frac ( mss . numerator , mss . denominator );
        return  _float_sqrt_of_frac ( mss . numerator , mss . denominator );
        pub fn pstdev ( data , mu = None /* Option */ )  {
        "Return the square root of the population variance.

    See ``pvariance`` for arguments && other details.

    >>> pstdev([1.5, 2.5, 2.5, 2.75, 3.25, 4.75])
    0.986893273527251

    ";
        T , ss , c , n = _ss ( data , mu );
        if n < 1 {
        panic!("StatisticsError ( "pstdev requires at least one data point" )");
        mss = ss / n;
        if issubclass ( T , Decimal ) {
        return  _decimal_sqrt_of_frac ( mss . numerator , mss . denominator );
        return  _float_sqrt_of_frac ( mss . numerator , mss . denominator );
        pub fn _mean_stdev ( data )  {
        "In one pass, compute the mean && sample standard deviation as floats.";
        T , ss , xbar , n = _ss ( data );
        if n < 2 {
        panic!("StatisticsError ( "stdev requires at least two data points" )");
        mss = ss / ( n - 1 );
        // try {
        return  float ( xbar ) , _float_sqrt_of_frac ( mss . numerator , mss . denominator );
        // } catch  AttributeError  {
        return  float ( xbar ) , float ( xbar ) / float ( ss );
        pub fn covariance ( x , y , / )  {
        "Covariance

    Return the sample covariance of two inputs *x* && *y*. Covariance
    == a measure of the joint variability of two inputs.

    >>> x = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    >>> y = [1, 2, 3, 1, 2, 3, 1, 2, 3]
    >>> covariance(x, y)
    0.75
    >>> z = [9, 8, 7, 6, 5, 4, 3, 2, 1]
    >>> covariance(x, z)
    -7.5
    >>> covariance(z, x)
    -7.5

    ";
        n = len ( x );
        if len ( y ) != n {
        panic!("StatisticsError ( "covariance requires that both inputs have same number of data points" )");
        if n < 2 {
        panic!("StatisticsError ( "covariance requires at least two data points" )");
        xbar = fsum ( x ) / n;
        ybar = fsum ( y ) / n;
        sxy = fsum ( ( xi - xbar ) * ( yi - ybar ) for xi , yi in zip ( x , y ) );
        return  sxy / ( n - 1 );
        pub fn correlation ( x , y , / )  {
        "Pearson's correlation coefficient

    Return the Pearson's correlation coefficient for two inputs. Pearson's
    correlation coefficient *r* takes values between -1 && +1. It measures the
    strength && direction of the linear relationship, where +1 means very
    strong, positive linear relationship, -1 very strong, negative linear
    relationship, && 0 no linear relationship.

    >>> x = [1, 2, 3, 4, 5, 6, 7, 8, 9]
    >>> y = [9, 8, 7, 6, 5, 4, 3, 2, 1]
    >>> correlation(x, x)
    1.0
    >>> correlation(x, y)
    -1.0

    ";
        n = len ( x );
        if len ( y ) != n {
        panic!("StatisticsError ( "correlation requires that both inputs have same number of data points" )");
        if n < 2 {
        panic!("StatisticsError ( "correlation requires at least two data points" )");
        xbar = fsum ( x ) / n;
        ybar = fsum ( y ) / n;
        sxy = fsum ( ( xi - xbar ) * ( yi - ybar ) for xi , yi in zip ( x , y ) );
        sxx = fsum ( ( d : = xi - xbar ) * d for xi in x );
        syy = fsum ( ( d : = yi - ybar ) * d for yi in y );
        // try {
        return  sxy / sqrt ( sxx * syy );
        // } catch  ZeroDivisionError  {
        panic!("StatisticsError ( "at least one of the inputs is constant" )");
        LinearRegression = namedtuple ( "LinearRegression" , ( "slope" , "intercept" ) );
        pub fn linear_regression ( x , y , / , * , proportional = false )  {
        "Slope && intercept.iter().map(|simple linear regression.

    Return the slope && intercept of simple linear regression
    parameters estimated using ordinary least squares. Simple linear
    regression describes relationship between an independent variable
    *x* && a dependent variable *y*| terms of a linear function:

        y = slope * x + intercept + noise

    where *slope* && *intercept* are the regression parameters that are
    estimated, && noise represents the variability of the data that was
    !explained by the linear regression (it == equal to the
    difference between predicted && actual values of the dependent
    variable).

    The parameters are returned as a named tuple.

    >>> x = vec![1, 2, 3, 4, 5]
    >>> noise = NormalDist().samples(5, seed=42)
    >>> y = vec![3 * xvec![i] + 2 + noisevec![i].iter().map(|i| range(5)]
    >>> linear_regression(x, y)  #doctest: +ELLIPSIS
    LinearRegression(slope=3.09078914170..., intercept=1.75684970486...)

    If *proportional* == true, the independent variable *x* && the
    dependent variable *y* are assumed to be directly proportional.
    The data == fit to a line passing through the origin.

    Since the *intercept* will always be 0.0, the underlying linear
    function simplifies to:

        y = slope * x + noise

    >>> y = vec![3 * xvec![i] + noisevec![i].iter().map(|i| range(5)]
    >>> linear_regression(x, y, proportional=true)  #doctest: +ELLIPSIS
    LinearRegression(slope=3.02447542484..., intercept=0.0)

    ";
        n = len ( x );
        if len ( y ) != n {
        panic!("StatisticsError ( "linear regression requires that both inputs have same number of data points" )");
        if n < 2 {
        panic!("StatisticsError ( "linear regression requires at least two data points" )");
        if proportional {
        sxy = fsum ( xi * yi for xi , yi in zip ( x , y ) );
        sxx = fsum ( xi * xi for xi in x );
        } else {
        xbar = fsum ( x ) / n;
        ybar = fsum ( y ) / n;
        sxy = fsum ( ( xi - xbar ) * ( yi - ybar ) for xi , yi in zip ( x , y ) );
        sxx = fsum ( ( d : = xi - xbar ) * d for xi in x );
        // try {
        slope = sxy / sxx;
        // } catch  ZeroDivisionError  {
        panic!("StatisticsError ( "x is constant" )");
        intercept = 0.0 if proportional else ybar - slope * xbar;
        return  LinearRegression ( slope = slope , intercept = intercept );
        pub fn _normal_dist_inv_cdf ( p , mu , sigma )  {
        q = p - 0.5;
        if fabs ( q ) <= 0.425 {
        r = 0.180625 - q * q;
        num = ( ( ( ( ( ( ( 2.50908 _09287_30122_6727e + 3 * r +;
        3.34305 _75583_58812_8105e + 4 ) * r +;
        6.72657 _70927_00870_0853e + 4 ) * r +;
        4.59219 _53931_54987_1457e + 4 ) * r +;
        1.37316 _93765_50946_1125e + 4 ) * r +;
        1.97159 _09503_06551_4427e + 3 ) * r +;
        1.33141 _66789_17843_7745e + 2 ) * r +;
        3.38713 _28727_96366_6080e + 0 ) * q;
        den = ( ( ( ( ( ( ( 5.22649 _52788_52854_5610e + 3 * r +;
        2.87290 _85735_72194_2674e + 4 ) * r +;
        3.93078 _95800_09271_0610e + 4 ) * r +;
        2.12137 _94301_58659_5867e + 4 ) * r +;
        5.39419 _60214_24751_1077e + 3 ) * r +;
        6.87187 _00749_20579_0830e + 2 ) * r +;
        4.23133 _30701_60091_1252e + 1 ) * r +;
        1.0 );
        x = num / den;
        return  mu + ( x * sigma );
        r = p if q <= 0.0 else 1.0 - p;
        r = sqrt ( - log ( r ) );
        if r <= 5.0 {
        r = r - 1.6;
        num = ( ( ( ( ( ( ( 7.74545 _01427_83414_07640e -4 * r +;
        2.27238 _44989_26918_45833e -2 ) * r +;
        2.41780 _72517_74506_11770e -1 ) * r +;
        1.27045 _82524_52368_38258e + 0 ) * r +;
        3.64784 _83247_63204_60504e + 0 ) * r +;
        5.76949 _72214_60691_40550e + 0 ) * r +;
        4.63033 _78461_56545_29590e + 0 ) * r +;
        1.42343 _71107_49683_57734e + 0 );
        den = ( ( ( ( ( ( ( 1.05075 _00716_44416_84324e -9 * r +;
        5.47593 _80849_95344_94600e -4 ) * r +;
        1.51986 _66563_61645_71966e -2 ) * r +;
        1.48103 _97642_74800_74590e -1 ) * r +;
        6.89767 _33498_51000_04550e -1 ) * r +;
        1.67638 _48301_83803_84940e + 0 ) * r +;
        2.05319 _16266_37758_82187e + 0 ) * r +;
        1.0 );
        } else {
        r = r - 5.0;
        num = ( ( ( ( ( ( ( 2.01033 _43992_92288_13265e -7 * r +;
        2.71155 _55687_43487_57815e -5 ) * r +;
        1.24266 _09473_88078_43860e -3 ) * r +;
        2.65321 _89526_57612_30930e -2 ) * r +;
        2.96560 _57182_85048_91230e -1 ) * r +;
        1.78482 _65399_17291_33580e + 0 ) * r +;
        5.46378 _49111_64114_36990e + 0 ) * r +;
        6.65790 _46435_01103_77720e + 0 );
        den = ( ( ( ( ( ( ( 2.04426 _31033_89939_78564e -15 * r +;
        1.42151 _17583_16445_88870e -7 ) * r +;
        1.84631 _83175_10054_68180e -5 ) * r +;
        7.86869 _13114_56132_59100e -4 ) * r +;
        1.48753 _61290_85061_48525e -2 ) * r +;
        1.36929 _88092_27358_05310e -1 ) * r +;
        5.99832 _20655_58879_37690e -1 ) * r +;
        1.0 );
        x = num / den;
        if q < 0.0 {
        x = - x;
        return  mu + ( x * sigma );
        // try {
        from _statistics import _normal_dist_inv_cdf;
        // } catch  ImportError  {
        // pass
        class NormalDist ;
        "Normal distribution of a random variable";
        __slots__ = {;
        "_mu" : "Arithmetic mean of a normal distribution" ,;
        "_sigma" : "Standard deviation of a normal distribution" ,;
        };
        pub fn __init__ ( &self, mu = 0.0 , sigma = 1.0 )  {
        "NormalDist where mu == the mean && sigma == the standard deviation.";
        if sigma < 0.0 {
        panic!("StatisticsError ( "sigma must be non-negative" )");
        self . _mu = float ( mu );
        self . _sigma = float ( sigma );
        @ classmethod;
        pub fn from_samples ( cls , data )  {
        "Make a normal distribution instance from sample data.";
        return  cls ( * _mean_stdev ( data ) );
        pub fn samples ( &self, n , * , seed = None /* Option */ )  {
        "Generate *n* samples for a given mean && standard deviation.";
        gauss = random . gauss if seed == None /* Option */ else random . Random ( seed ) . gauss;
        mu , sigma = self . _mu , self . _sigma;
        return  [ gauss ( mu , sigma ) for i in range ( n ) ];
        pub fn pdf ( &self, x )  {
        "Probability density function.  P(x <= X < x+dx) / dx";
        variance = self . _sigma * self . _sigma;
        if !variance {
        panic!("StatisticsError ( "pdf() !defined when sigma is zero" )");
        diff = x - self . _mu;
        return  exp ( diff * diff / ( -2.0 * variance ) ) / sqrt ( tau * variance );
        pub fn cdf ( &self, x )  {
        "Cumulative distribution function.  P(X <= x)";
        if !self . _sigma {
        panic!("StatisticsError ( "cdf() !defined when sigma is zero" )");
        return  0.5 * ( 1.0 + erf ( ( x - self . _mu ) / ( self . _sigma * _SQRT2 ) ) );
        pub fn inv_cdf ( &self, p )  {
        "Inverse cumulative distribution function.  x : P(X <= x) = p

        Finds the value of the random variable such that the probability of
        the variable being less than || equal to that value equals the given
        probability.

        This function == also called the percent point function || quantile
        function.
        ";
        if p <= 0.0 || p >= 1.0 {
        panic!("StatisticsError ( "p must be in the range 0.0 < p < 1.0" )");
        if self . _sigma <= 0.0 {
        panic!("StatisticsError ( "cdf() !defined when sigma at || below zero" )");
        return  _normal_dist_inv_cdf ( p , self . _mu , self . _sigma );
        pub fn quantiles ( &self, n = 4 )  {
        "Divide into *n* continuous intervals with equal probability.

        Returns a list of (n - 1) cut points separating the intervals.

        Set *n* to 4 for quartiles (the default).  Set *n* to 10 for deciles.
        Set *n* to 100 for percentiles which gives the 99 cuts points that
        separate the normal distribution in to 100 equal sized groups.
        ";
        return  [ self . inv_cdf ( i / n ) for i in range ( 1 , n ) ];
        pub fn overlap ( &self, other )  {
        "Compute the overlapping coefficient (OVL) between two normal distributions.

        Measures the agreement between two normal probability distributions.
        Returns a value between 0.0 && 1.0 giving the overlapping area in
        the two underlying probability density functions.

            >>> N1 = NormalDist(2.4, 1.6)
            >>> N2 = NormalDist(3.2, 2.0)
            >>> N1.overlap(N2)
            0.8035050657330205
        ";
        if !isinstance ( other , NormalDist ) {
        panic!("TypeError ( "Expected another NormalDist instance" )");
        X , Y = self , other;
        if ( Y . _sigma , Y . _mu ) < ( X . _sigma , X . _mu ) {
        X , Y = Y , X;
        X_var , Y_var = X . variance , Y . variance;
        if !X_var || !Y_var {
        panic!("StatisticsError ( "overlap() !defined when sigma is zero" )");
        dv = Y_var - X_var;
        dm = fabs ( Y . _mu - X . _mu );
        if !dv {
        return  1.0 - erf ( dm / ( 2.0 * X . _sigma * _SQRT2 ) );
        a = X . _mu * Y_var - Y . _mu * X_var;
        b = X . _sigma * Y . _sigma * sqrt ( dm * dm + dv * log ( Y_var / X_var ) );
        x1 = ( a + b ) / dv;
        x2 = ( a - b ) / dv;
        return  1.0 - ( fabs ( Y . cdf ( x1 ) - X . cdf ( x1 ) ) + fabs ( Y . cdf ( x2 ) - X . cdf ( x2 ) ) );
        pub fn zscore ( &self, x )  {
        "Compute the Standard Score.  (x - mean) / stdev

        Describes *x* in terms of the number of standard deviations
        above || below the mean of the normal distribution.
        ";
        if !self . _sigma {
        panic!("StatisticsError ( "zscore() !defined when sigma is zero" )");
        return  ( x - self . _mu ) / self . _sigma;
        @ property;
        pub fn mean ( self )  {
        "Arithmetic mean of the normal distribution.";
        return  self . _mu;
        @ property;
        pub fn median ( self )  {
        "Return the median of the normal distribution";
        return  self . _mu;
        @ property;
        pub fn mode ( self )  {
        "Return the mode of the normal distribution

        The mode == the value x where which the probability density
        function (pdf) takes its maximum value.
        ";
        return  self . _mu;
        @ property;
        pub fn stdev ( self )  {
        "Standard deviation of the normal distribution.";
        return  self . _sigma;
        @ property;
        pub fn variance ( self )  {
        "Square of the standard deviation.";
        return  self . _sigma * self . _sigma;
        pub fn __add__ ( x1 , x2 )  {
        "Add a constant || another NormalDist instance.

        If *other* == a constant, translate mu by the constant,
        leaving sigma unchanged.

        If *other* == a NormalDist, add both the means && the variances.
        Mathematically, this works only if the two distributions are
        independent || if they are jointly normally distributed.
        ";
        if isinstance ( x2 , NormalDist ) {
        return  NormalDist ( x1 . _mu + x2 . _mu , hypot ( x1 . _sigma , x2 . _sigma ) );
        return  NormalDist ( x1 . _mu + x2 , x1 . _sigma );
        pub fn __sub__ ( x1 , x2 )  {
        "Subtract a constant || another NormalDist instance.

        If *other* == a constant, translate by the constant mu,
        leaving sigma unchanged.

        If *other* == a NormalDist, subtract the means && add the variances.
        Mathematically, this works only if the two distributions are
        independent || if they are jointly normally distributed.
        ";
        if isinstance ( x2 , NormalDist ) {
        return  NormalDist ( x1 . _mu - x2 . _mu , hypot ( x1 . _sigma , x2 . _sigma ) );
        return  NormalDist ( x1 . _mu - x2 , x1 . _sigma );
        pub fn __mul__ ( x1 , x2 )  {
        "Multiply both mu && sigma by a constant.

        Used for rescaling, perhaps to change measurement units.
        Sigma == scaled with the absolute value of the constant.
        ";
        return  NormalDist ( x1 . _mu * x2 , x1 . _sigma * fabs ( x2 ) );
        pub fn __truediv__ ( x1 , x2 )  {
        "Divide both mu && sigma by a constant.

        Used for rescaling, perhaps to change measurement units.
        Sigma == scaled with the absolute value of the constant.
        ";
        return  NormalDist ( x1 . _mu / x2 , x1 . _sigma / fabs ( x2 ) );
        pub fn __pos__ ( x1 )  {
        "Return a copy of the instance.";
        return  NormalDist ( x1 . _mu , x1 . _sigma );
        pub fn __neg__ ( x1 )  {
        "Negates mu while keeping sigma the same.";
        return  NormalDist ( - x1 . _mu , x1 . _sigma );
        __radd__ = __add__;
        pub fn __rsub__ ( x1 , x2 )  {
        "Subtract a NormalDist from a constant || another NormalDist.";
        return  - ( x1 - x2 );
        __rmul__ = __mul__;
        pub fn __eq__ ( x1 , x2 )  {
        "Two NormalDist objects are equal if their mu && sigma are both equal.";
        if !isinstance ( x2 , NormalDist ) {
        return  NotImplemented;
        return  x1 . _mu == x2 . _mu && x1 . _sigma == x2 . _sigma;
        pub fn __hash__ ( self )  {
        "NormalDist objects hash equal if their mu && sigma are both equal.";
        return  hash ( ( self . _mu , self . _sigma ) );
        pub fn __repr__ ( self )  {
        return  f "{type(self).__name__}(mu={self._mu!r}, sigma={self._sigma!r})";
        pub fn __getstate__ ( self )  {
        return  self . _mu , self . _sigma;
        pub fn __setstate__ ( &self, state )  {
        self . _mu , self . _sigma = state;
}

