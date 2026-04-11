//! datetime.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::time;
// use std::env;
// use crate::index;
// use crate::_strptime;
// use crate::_datetime::{};

pub const __all__: &str = ("date" ,"datetime" ,"time" ,"timedelta" ,"timezone" ,"tzinfo" ,;
pub fn _cmp(x: &str, y: &str) {
        return  0 if x == y else 1 if x > y else -1;
        MINYEAR = 1;
        MAXYEAR = 9999;
        _MAXORDINAL = 3652059;
        _DAYS_IN_MONTH = [ -1 , 31 , 28 , 31 , 30 , 31 , 30 , 31 , 31 , 30 , 31 , 30 , 31 ];
        _DAYS_BEFORE_MONTH = [ -1 ];
        dbm = 0;
        for dim in _DAYS_IN_MONTH [ 1 : ] .iter() {
        _DAYS_BEFORE_MONTH . append ( dbm );
        dbm + = dim;
        del dbm , dim;
        pub fn _is_leap ( year )  {
        "year -> 1 if leap year, else 0.";
        return  year % 4 == 0 && ( year % 100 != 0 || year % 400 == 0 );
        pub fn _days_before_year ( year )  {
        "year -> number of days before January 1st of year.";
        y = year - 1;
        return  y * 365 + y / / 4 - y / / 100 + y / / 400;
        pub fn _days_in_month ( year , month )  {
        "year, month -> number of days in that month in that year.";
        assert 1 <= month <= 12 , month;
        if month == 2 && _is_leap ( year ) {
        return  29;
        return  _DAYS_IN_MONTH [ month ];
        pub fn _days_before_month ( year , month )  {
        "year, month -> number of days in year preceding first day of month.";
        assert 1 <= month <= 12 , "month must be in 1..12";
        return  _DAYS_BEFORE_MONTH [ month ] + ( month > 2 && _is_leap ( year ) );
        pub fn _ymd2ord ( year , month , day )  {
        "year, month, day -> ordinal, considering 01-Jan-0001 as day 1.";
        assert 1 <= month <= 12 , "month must be in 1..12";
        dim = _days_in_month ( year , month );
        assert 1 <= day <= dim , ( "day must be in 1..%d" % dim );
        return  ( _days_before_year ( year ) +;
        _days_before_month ( year , month ) +;
        day );
        _DI400Y = _days_before_year ( 401 );
        _DI100Y = _days_before_year ( 101 );
        _DI4Y = _days_before_year ( 5 );
        assert _DI4Y == 4 * 365 + 1;
        assert _DI400Y == 4 * _DI100Y + 1;
        assert _DI100Y == 25 * _DI4Y - 1;
        pub fn _ord2ymd ( n )  {
        "ordinal -> (year, month, day), considering 01-Jan-0001 as day 1.";
        n - = 1;
        n400 , n = divmod ( n , _DI400Y );
        year = n400 * 400 + 1;
        n100 , n = divmod ( n , _DI100Y );
        n4 , n = divmod ( n , _DI4Y );
        n1 , n = divmod ( n , 365 );
        year + = n100 * 100 + n4 * 4 + n1;
        if n1 == 4 || n100 == 4 {
        assert n == 0;
        return  year -1 , 12 , 31;
        leapyear = n1 == 3 && ( n4 != 24 || n100 == 3 );
        assert leapyear == _is_leap ( year );
        month = ( n + 50 ) > > 5;
        preceding = _DAYS_BEFORE_MONTH [ month ] + ( month > 2 && leapyear );
        if preceding > n {
        month - = 1;
        preceding - = _DAYS_IN_MONTH [ month ] + ( month == 2 && leapyear );
        n - = preceding;
        assert 0 <= n < _days_in_month ( year , month );
        return  year , month , n + 1;
        _MONTHNAMES = [ None /* Option */ , "Jan" , "Feb" , "Mar" , "Apr" , "May" , "Jun" ,;
        "Jul" , "Aug" , "Sep" , "Oct" , "Nov" , "Dec" ];
        _DAYNAMES = [ None /* Option */ , "Mon" , "Tue" , "Wed" , "Thu" , "Fri" , "Sat" , "Sun" ];
        pub fn _build_struct_time ( y , m , d , hh , mm , ss , dstflag )  {
        wday = ( _ymd2ord ( y , m , d ) + 6 ) % 7;
        dnum = _days_before_month ( y , m ) + d;
        return  _time . struct_time ( ( y , m , d , hh , mm , ss , wday , dnum , dstflag ) );
        pub fn _format_time ( hh , mm , ss , us , timespec = "auto" )  {
        specs = {;
        "hours" : "{:02d}" ,;
        "minutes" : "{:02d}:{:02d}" ,;
        "seconds" : "{:02d}:{:02d}:{:02d}" ,;
        "milliseconds" : "{:02d}:{:02d}:{:02d}.{:03d}" ,;
        "microseconds" : "{:02d}:{:02d}:{:02d}.{:06d}";
        };
        if timespec == "auto" {
        timespec = "microseconds" if us else "seconds";
        } else if timespec == "milliseconds" {
        us / / = 1000;
        // try {
        fmt = specs [ timespec ];
        // } catch  KeyError  {
        panic!("ValueError ( "Unknown timespec value" )");
        } else {
        return  fmt . format ( hh , mm , ss , us );
        pub fn _format_offset ( off )  {
        s = "";
        if off is !None /* Option */ {
        if off . days < 0 {
        sign = "-";
        off = - off;
        } else {
        sign = "+";
        hh , mm = divmod ( off , timedelta ( hours = 1 ) );
        mm , ss = divmod ( mm , timedelta ( minutes = 1 ) );
        s + = "%s%02d:%02d" % ( sign , hh , mm );
        if ss || ss . microseconds {
        s + = ":%02d" % ss . seconds;
        if ss . microseconds {
        s + = ".%06d" % ss . microseconds;
        return  s;
        pub fn _wrap_strftime ( object , format , timetuple )  {
        freplace = None /* Option */;
        zreplace = None /* Option */;
        Zreplace = None /* Option */;
        newformat = [ ];
        push = newformat . append;
        i , n = 0 , len ( format );
        while i < n  {
        ch = format [ i ];
        i + = 1;
        if ch == "%" {
        if i < n {
        ch = format [ i ];
        i + = 1;
        if ch == "f" {
        if freplace is None /* Option */ {
        freplace = "%06d" % getattr ( object ,;
        "microsecond" , 0 );
        newformat . append ( freplace );
        } else if ch == "z" {
        if zreplace is None /* Option */ {
        zreplace = "";
        if hasattr ( object , "utcoffset" ) {
        offset = object . utcoffset ( );
        if offset is !None /* Option */ {
        sign = "+";
        if offset . days < 0 {
        offset = - offset;
        sign = "-";
        h , rest = divmod ( offset , timedelta ( hours = 1 ) );
        m , rest = divmod ( rest , timedelta ( minutes = 1 ) );
        s = rest . seconds;
        u = offset . microseconds;
        if u {
        zreplace = "%c%02d%02d%02d.%06d" % ( sign , h , m , s , u );
        } else if s {
        zreplace = "%c%02d%02d%02d" % ( sign , h , m , s );
        } else {
        zreplace = "%c%02d%02d" % ( sign , h , m );
        assert "%" !in zreplace;
        newformat . append ( zreplace );
        } else if ch == "Z" {
        if Zreplace is None /* Option */ {
        Zreplace = "";
        if hasattr ( object , "tzname" ) {
        s = object . tzname ( );
        if s is !None /* Option */ {
        Zreplace = s . replace ( "%" , "%%" );
        newformat . append ( Zreplace );
        } else {
        push ( "%" );
        push ( ch );
        } else {
        push ( "%" );
        } else {
        push ( ch );
        newformat = "" . join ( newformat );
        return  _time . strftime ( newformat , timetuple );
        pub fn _is_ascii_digit ( c )  {
        return  c in "0123456789";
        pub fn _find_isoformat_datetime_separator ( dtstr )  {
        len_dtstr = len ( dtstr );
        if len_dtstr == 7 {
        return  7;
        assert len_dtstr > 7;
        date_separator = "-";
        week_indicator = "W";
        if dtstr [ 4 ] == date_separator {
        if dtstr [ 5 ] == week_indicator {
        if len_dtstr < 8 {
        panic!("ValueError ( "Invalid ISO string" )");
        if len_dtstr > 8 && dtstr [ 8 ] == date_separator {
        if len_dtstr == 9 {
        panic!("ValueError ( "Invalid ISO string" )");
        if len_dtstr > 10 && _is_ascii_digit ( dtstr [ 10 ] ) {
        return  8;
        return  10;
        } else {
        return  8;
        } else {
        return  10;
        } else {
        if dtstr [ 4 ] == week_indicator {
        idx = 7;
        while idx < len_dtstr  {
        if !_is_ascii_digit ( dtstr [ idx ] ) {
        break;
        idx + = 1;
        if idx < 9 {
        return  idx;
        if idx % 2 == 0 {
        return  7;
        } else {
        return  8;
        } else {
        return  8;
        pub fn _parse_isoformat_date ( dtstr )  {
        assert len ( dtstr ) in ( 7 , 8 , 10 );
        year = int ( dtstr [ 0 : 4 ] );
        has_sep = dtstr [ 4 ] == "-";
        pos = 4 + has_sep;
        if dtstr [ pos { : pos + 1 ] == "W" ; }
        pos + = 1;
        weekno = int ( dtstr [ pos : pos + 2 ] );
        pos + = 2;
        dayno = 1;
        if len ( dtstr ) > pos {
        if ( dtstr [ pos { : pos + 1 ] == "-" ) != has_sep ; }
        panic!("ValueError ( "Inconsistent use of dash separator" )");
        pos + = has_sep;
        dayno = int ( dtstr [ pos : pos + 1 ] );
        return  list ( _isoweek_to_gregorian ( year , weekno , dayno ) );
        } else {
        month = int ( dtstr [ pos : pos + 2 ] );
        pos + = 2;
        if ( dtstr [ pos { : pos + 1 ] == "-" ) != has_sep ; }
        panic!("ValueError ( "Inconsistent use of dash separator" )");
        pos + = has_sep;
        day = int ( dtstr [ pos : pos + 2 ] );
        return  [ year , month , day ];
        _FRACTION_CORRECTION = [ 100000 , 10000 , 1000 , 100 , 10 ];
        pub fn _parse_hh_mm_ss_ff ( tstr )  {
        len_str = len ( tstr );
        time_comps = [ 0 , 0 , 0 , 0 ];
        pos = 0;
        for comp in range ( 0 , 3 ) .iter() {
        if ( len_str - pos ) < 2 {
        panic!("ValueError ( "Incomplete time component" )");
        time_comps [ comp ] = int ( tstr [ pos : pos + 2 ] );
        pos + = 2;
        next_char = tstr [ pos : pos + 1 ];
        if comp == 0 {
        has_sep = next_char == ":";
        if !next_char || comp >= 2 {
        break;
        if has_sep && next_char != ":" {
        panic!("ValueError ( "Invalid time separator: %c" % next_char )");
        pos + = has_sep;
        if pos < len_str {
        if tstr [ pos ] !in ".," {
        panic!("ValueError ( "Invalid microsecond component" )");
        } else {
        pos + = 1;
        len_remainder = len_str - pos;
        if len_remainder >= 6 {
        to_parse = 6;
        } else {
        to_parse = len_remainder;
        time_comps [ 3 ] = int ( tstr [ pos : ( pos + to_parse ) ] );
        if to_parse < 6 {
        time_comps [ 3 ] * = _FRACTION_CORRECTION [ to_parse -1 ];
        if ( len_remainder > to_parse {
        and !all ( map ( _is_ascii_digit , tstr [ ( pos + to_parse ) : ] ) ) ) ;
        panic!("ValueError ( "Non-digit values in unparsed fraction" )");
        return  time_comps;
        pub fn _parse_isoformat_time ( tstr )  {
        len_str = len ( tstr );
        if len_str < 2 {
        panic!("ValueError ( "Isoformat time too short" )");
        tz_pos = ( tstr . find ( "-" ) + 1 || tstr . find ( "+" ) + 1 || tstr . find ( "Z" ) + 1 );
        timestr = tstr [ : tz_pos -1 ] if tz_pos > 0 else tstr;
        time_comps = _parse_hh_mm_ss_ff ( timestr );
        tzi = None /* Option */;
        if tz_pos == len_str && tstr [ -1 ] == "Z" {
        tzi = timezone . utc;
        } else if tz_pos > 0 {
        tzstr = tstr [ tz_pos : ];
        if len ( tzstr ) in ( 0 , 1 , 3 ) {
        panic!("ValueError ( "Malformed time zone string" )");
        tz_comps = _parse_hh_mm_ss_ff ( tzstr );
        if all ( x == 0 for x in tz_comps ) {
        tzi = timezone . utc;
        } else {
        tzsign = -1 if tstr [ tz_pos - 1 ] == "-" else 1;
        td = timedelta ( hours = tz_comps [ 0 ] , minutes = tz_comps [ 1 ] ,;
        seconds = tz_comps [ 2 ] , microseconds = tz_comps [ 3 ] );
        tzi = timezone ( tzsign * td );
        time_comps . append ( tzi );
        return  time_comps;
        pub fn _isoweek_to_gregorian ( year , week , day )  {
        if !MINYEAR <= year <= MAXYEAR {
        panic!("ValueError ( f "Year is out of range: {year}" )");
        if !0 < week < 53 {
        out_of_range = true;
        if week == 53 {
        first_weekday = _ymd2ord ( year , 1 , 1 ) % 7;
        if ( first_weekday == 4 || ( first_weekday == 3 and {
        _is_leap ( year ) ) ) ;
        out_of_range = false;
        if out_of_range {
        panic!("ValueError ( f "Invalid week: {week}" )");
        if !0 < day < 8 {
        panic!("ValueError ( f "Invalid weekday: {day} (range is [1, 7])" )");
        day_offset = ( week - 1 ) * 7 + ( day - 1 );
        day_1 = _isoweek1monday ( year );
        ord_day = day_1 + day_offset;
        return  _ord2ymd ( ord_day );
        pub fn _check_tzname ( name )  {
        if name is !None /* Option */ && !isinstance ( name , str ) {
        panic!("TypeError ( "tzinfo.tzname() must return None /* Option */ || string, "");
        "not '%s'" % type ( name ) );
        pub fn _check_utc_offset ( name , offset )  {
        assert name in ( "utcoffset" , "dst" );
        if offset is None /* Option */ {
        return;
        if !isinstance ( offset , timedelta ) {
        panic!("TypeError ( "tzinfo.%s() must return None /* Option */ "");
        "or timedelta, !'%s'" % ( name , type ( offset ) ) );
        if !- timedelta ( 1 ) < offset < timedelta ( 1 ) {
        panic!("ValueError ( "%s()=%s, must be strictly between "");
        "-timedelta(hours=24) && timedelta(hours=24)" %;
        ( name , offset ) );
        pub fn _check_date_fields ( year , month , day )  {
        year = _index ( year );
        month = _index ( month );
        day = _index ( day );
        if !MINYEAR <= year <= MAXYEAR {
        panic!("ValueError ( "year must be in %d..%d" % ( MINYEAR , MAXYEAR ) , year )");
        if !1 <= month <= 12 {
        panic!("ValueError ( "month must be in 1..12" , month )");
        dim = _days_in_month ( year , month );
        if !1 <= day <= dim {
        panic!("ValueError ( "day must be in 1..%d" % dim , day )");
        return  year , month , day;
        pub fn _check_time_fields ( hour , minute , second , microsecond , fold )  {
        hour = _index ( hour );
        minute = _index ( minute );
        second = _index ( second );
        microsecond = _index ( microsecond );
        if !0 <= hour <= 23 {
        panic!("ValueError ( "hour must be in 0..23" , hour )");
        if !0 <= minute <= 59 {
        panic!("ValueError ( "minute must be in 0..59" , minute )");
        if !0 <= second <= 59 {
        panic!("ValueError ( "second must be in 0..59" , second )");
        if !0 <= microsecond <= 999999 {
        panic!("ValueError ( "microsecond must be in 0..999999" , microsecond )");
        if fold !in ( 0 , 1 ) {
        panic!("ValueError ( "fold must be either 0 || 1" , fold )");
        return  hour , minute , second , microsecond , fold;
        pub fn _check_tzinfo_arg ( tz )  {
        if tz is !None /* Option */ && !isinstance ( tz , tzinfo ) {
        panic!("TypeError ( "tzinfo argument must be None /* Option */ || of a tzinfo subclass" )");
        pub fn _cmperror ( x , y )  {
        panic!("TypeError ( "can't compare '%s' to '%s'" % (");
        type ( x ) . __name__ , type ( y ) . __name__ ) );
        pub fn _divide_and_round ( a , b )  {
        "divide a by b && round result to the nearest integer

    When the ratio == exactly half-way between two integers,
    the even integer == returned.
    ";
        q , r = divmod ( a , b );
        r * = 2;
        greater_than_half = r > b if b > 0 else r < b;
        if greater_than_half || r == b && q % 2 == 1 {
        q + = 1;
        return  q;
        class timedelta ;
        "Represent the difference between two datetime objects.

    Supported operators:

    - add, subtract timedelta
    - unary plus, minus, abs
    - compare to timedelta
    - multiply, divide by int

    In addition, datetime supports subtraction of two datetime objects
    returning a timedelta, && addition || subtraction of a datetime
    && a timedelta giving a datetime.

    Representation: (days, seconds, microseconds).  Why?  Because I
    felt like it.
    ";
        __slots__ = "_days" , "_seconds" , "_microseconds" , "_hashcode";
        pub fn __new__ ( cls , days = 0 , seconds = 0 , microseconds = 0 , {
        milliseconds = 0 , minutes = 0 , hours = 0 , weeks = 0 ) ;
        d = s = us = 0;
        days + = weeks * 7;
        seconds + = minutes * 60 + hours * 3600;
        microseconds + = milliseconds * 1000;
        if isinstance ( days , float ) {
        dayfrac , days = _math . modf ( days );
        daysecondsfrac , daysecondswhole = _math . modf ( dayfrac * ( 24. * 3600. ) );
        assert daysecondswhole == int ( daysecondswhole );
        s = int ( daysecondswhole );
        assert days == int ( days );
        d = int ( days );
        } else {
        daysecondsfrac = 0.0;
        d = days;
        assert isinstance ( daysecondsfrac , float );
        assert abs ( daysecondsfrac ) <= 1.0;
        assert isinstance ( d , int );
        assert abs ( s ) <= 24 * 3600;
        if isinstance ( seconds , float ) {
        secondsfrac , seconds = _math . modf ( seconds );
        assert seconds == int ( seconds );
        seconds = int ( seconds );
        secondsfrac + = daysecondsfrac;
        assert abs ( secondsfrac ) <= 2.0;
        } else {
        secondsfrac = daysecondsfrac;
        assert isinstance ( secondsfrac , float );
        assert abs ( secondsfrac ) <= 2.0;
        assert isinstance ( seconds , int );
        days , seconds = divmod ( seconds , 24 * 3600 );
        d + = days;
        s + = int ( seconds );
        assert isinstance ( s , int );
        assert abs ( s ) <= 2 * 24 * 3600;
        usdouble = secondsfrac * 1e6;
        assert abs ( usdouble ) < 2.1e6;
        if isinstance ( microseconds , float ) {
        microseconds = round ( microseconds + usdouble );
        seconds , microseconds = divmod ( microseconds , 1000000 );
        days , seconds = divmod ( seconds , 24 * 3600 );
        d + = days;
        s + = seconds;
        } else {
        microseconds = int ( microseconds );
        seconds , microseconds = divmod ( microseconds , 1000000 );
        days , seconds = divmod ( seconds , 24 * 3600 );
        d + = days;
        s + = seconds;
        microseconds = round ( microseconds + usdouble );
        assert isinstance ( s , int );
        assert isinstance ( microseconds , int );
        assert abs ( s ) <= 3 * 24 * 3600;
        assert abs ( microseconds ) < 3.1e6;
        seconds , us = divmod ( microseconds , 1000000 );
        s + = seconds;
        days , s = divmod ( s , 24 * 3600 );
        d + = days;
        assert isinstance ( d , int );
        assert isinstance ( s , int ) && 0 <= s < 24 * 3600;
        assert isinstance ( us , int ) && 0 <= us < 1000000;
        if abs ( d ) > 999999999 {
        panic!("OverflowError ( "timedelta # of days is too large: %d" % d )");
        self = object . __new__ ( cls );
        self . _days = d;
        self . _seconds = s;
        self . _microseconds = us;
        self . _hashcode = -1;
        return  self;
        pub fn __repr__ ( self )  {
        args = [ ];
        if self . _days {
        args . append ( "days=%d" % self . _days );
        if self . _seconds {
        args . append ( "seconds=%d" % self . _seconds );
        if self . _microseconds {
        args . append ( "microseconds=%d" % self . _microseconds );
        if !args {
        args . append ( "0" );
        return  "%s.%s(%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        ", " . join ( args ) );
        pub fn __str__ ( self )  {
        mm , ss = divmod ( self . _seconds , 60 );
        hh , mm = divmod ( mm , 60 );
        s = "%d:%02d:%02d" % ( hh , mm , ss );
        if self . _days {
        pub fn plural ( n )  {
        return  n , abs ( n ) != 1 && "s" || "";
        s = ( "%d day%s, " % plural ( self . _days ) ) + s;
        if self . _microseconds {
        s = s + ".%06d" % self . _microseconds;
        return  s;
        pub fn total_seconds ( self )  {
        "Total seconds in the duration.";
        return  ( ( self . days * 86400 + self . seconds ) * 10 ** 6 +;
        self . microseconds ) / 10 ** 6;
        @ property;
        pub fn days ( self )  {
        "days";
        return  self . _days;
        @ property;
        pub fn seconds ( self )  {
        "seconds";
        return  self . _seconds;
        @ property;
        pub fn microseconds ( self )  {
        "microseconds";
        return  self . _microseconds;
        pub fn __add__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  timedelta ( self . _days + other . _days ,;
        self . _seconds + other . _seconds ,;
        self . _microseconds + other . _microseconds );
        return  NotImplemented;
        __radd__ = __add__;
        pub fn __sub__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  timedelta ( self . _days - other . _days ,;
        self . _seconds - other . _seconds ,;
        self . _microseconds - other . _microseconds );
        return  NotImplemented;
        pub fn __rsub__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  - self + other;
        return  NotImplemented;
        pub fn __neg__ ( self )  {
        return  timedelta ( - self . _days ,;
        - self . _seconds ,;
        - self . _microseconds );
        pub fn __pos__ ( self )  {
        return  self;
        pub fn __abs__ ( self )  {
        if self . _days < 0 {
        return  - self;
        } else {
        return  self;
        pub fn __mul__ ( &self, other )  {
        if isinstance ( other , int ) {
        return  timedelta ( self . _days * other ,;
        self . _seconds * other ,;
        self . _microseconds * other );
        if isinstance ( other , float ) {
        usec = self . _to_microseconds ( );
        a , b = other . as_integer_ratio ( );
        return  timedelta ( 0 , 0 , _divide_and_round ( usec * a , b ) );
        return  NotImplemented;
        __rmul__ = __mul__;
        pub fn _to_microseconds ( self )  {
        return  ( ( self . _days * ( 24 * 3600 ) + self . _seconds ) * 1000000 +;
        self . _microseconds );
        pub fn __floordiv__ ( &self, other )  {
        if !isinstance ( other , ( int , timedelta ) ) {
        return  NotImplemented;
        usec = self . _to_microseconds ( );
        if isinstance ( other , timedelta ) {
        return  usec / / other . _to_microseconds ( );
        if isinstance ( other , int ) {
        return  timedelta ( 0 , 0 , usec / / other );
        pub fn __truediv__ ( &self, other )  {
        if !isinstance ( other , ( int , float , timedelta ) ) {
        return  NotImplemented;
        usec = self . _to_microseconds ( );
        if isinstance ( other , timedelta ) {
        return  usec / other . _to_microseconds ( );
        if isinstance ( other , int ) {
        return  timedelta ( 0 , 0 , _divide_and_round ( usec , other ) );
        if isinstance ( other , float ) {
        a , b = other . as_integer_ratio ( );
        return  timedelta ( 0 , 0 , _divide_and_round ( b * usec , a ) );
        pub fn __mod__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        r = self . _to_microseconds ( ) % other . _to_microseconds ( );
        return  timedelta ( 0 , 0 , r );
        return  NotImplemented;
        pub fn __divmod__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        q , r = divmod ( self . _to_microseconds ( ) ,;
        other . _to_microseconds ( ) );
        return  q , timedelta ( 0 , 0 , r );
        return  NotImplemented;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  self . _cmp ( other ) == 0;
        } else {
        return  NotImplemented;
        pub fn __le__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  self . _cmp ( other ) <= 0;
        } else {
        return  NotImplemented;
        pub fn __lt__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  self . _cmp ( other ) < 0;
        } else {
        return  NotImplemented;
        pub fn __ge__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  self . _cmp ( other ) >= 0;
        } else {
        return  NotImplemented;
        pub fn __gt__ ( &self, other )  {
        if isinstance ( other , timedelta ) {
        return  self . _cmp ( other ) > 0;
        } else {
        return  NotImplemented;
        pub fn _cmp ( &self, other )  {
        assert isinstance ( other , timedelta );
        return  _cmp ( self . _getstate ( ) , other . _getstate ( ) );
        pub fn __hash__ ( self )  {
        if self . _hashcode == -1 {
        self . _hashcode = hash ( self . _getstate ( ) );
        return  self . _hashcode;
        pub fn __bool__ ( self )  {
        return  ( self . _days != 0 or;
        self . _seconds != 0 or;
        self . _microseconds != 0 );
        pub fn _getstate ( self )  {
        return  ( self . _days , self . _seconds , self . _microseconds );
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ , self . _getstate ( ) );
        timedelta . min = timedelta ( -999999999 );
        timedelta . max = timedelta ( days = 999999999 , hours = 23 , minutes = 59 , seconds = 59 ,;
        microseconds = 999999 );
        timedelta . resolution = timedelta ( microseconds = 1 );
        class date ;
        "Concrete date type.

    Constructors:

    __new__()
    fromtimestamp()
    today()
    fromordinal()

    Operators:

    __repr__, __str__
    __eq__, __le__, __lt__, __ge__, __gt__, __hash__
    __add__, __radd__, __sub__ (add/radd only with timedelta arg)

    Methods:

    timetuple()
    toordinal()
    weekday()
    isoweekday(), isocalendar(), isoformat()
    ctime()
    strftime()

    Properties (readonly):
    year, month, day
    ";
        __slots__ = "_year" , "_month" , "_day" , "_hashcode";
        pub fn __new__ ( cls , year , month = None /* Option */ , day = None /* Option */ )  {
        "Constructor.

        Arguments:

        year, month, day (required, base 1)
        ";
        if ( month is None /* Option */ and {
        isinstance ( year , ( bytes , str ) ) && len ( year ) == 4 and;
        1 <= ord ( year [ 2 : 3 ] ) <= 12 ) ;
        if isinstance ( year , str ) {
        // try {
        year = year . encode ( "latin1" );
        // } catch  UnicodeEncodeError  {
        panic!("ValueError (");
        "Failed to encode latin1 string when unpickling ";
        "a date object. ";
        "pickle.load(data, encoding='latin1') == assumed." );
        self = object . __new__ ( cls );
        self . __setstate ( year );
        self . _hashcode = -1;
        return  self;
        year , month , day = _check_date_fields ( year , month , day );
        self = object . __new__ ( cls );
        self . _year = year;
        self . _month = month;
        self . _day = day;
        self . _hashcode = -1;
        return  self;
        @ classmethod;
        pub fn fromtimestamp ( cls , t )  {
        "Construct a date from a POSIX timestamp (like time.time()).";
        y , m , d , hh , mm , ss , weekday , jday , dst = _time . localtime ( t );
        return  cls ( y , m , d );
        @ classmethod;
        pub fn today ( cls )  {
        "Construct a date from time.time().";
        t = _time . time ( );
        return  cls . fromtimestamp ( t );
        @ classmethod;
        pub fn fromordinal ( cls , n )  {
        "Construct a date from a proleptic Gregorian ordinal.

        January 1 of year 1 == day 1.  Only the year, month && day are
        non-zero in the result.
        ";
        y , m , d = _ord2ymd ( n );
        return  cls ( y , m , d );
        @ classmethod;
        pub fn fromisoformat ( cls , date_string )  {
        "Construct a date from a string in ISO 8601 format.";
        if !isinstance ( date_string , str ) {
        panic!("TypeError ( "fromisoformat: argument must be str" )");
        if len ( date_string ) !in ( 7 , 8 , 10 ) {
        panic!("ValueError ( f "Invalid isoformat string: {date_string!r}" )");
        // try {
        return  cls ( * _parse_isoformat_date ( date_string ) );
        // } catch  Exception  {
        panic!("ValueError ( f "Invalid isoformat string: {date_string!r}" )");
        @ classmethod;
        pub fn fromisocalendar ( cls , year , week , day )  {
        "Construct a date from the ISO year, week number && weekday.

        This == the inverse of the date.isocalendar() function";
        return  cls ( * _isoweek_to_gregorian ( year , week , day ) );
        pub fn __repr__ ( self )  {
        "Convert to formal string, for repr().

        >>> d = date(2010, 1, 1)
        >>> repr(d)
        'datetime.date(2010, 1, 1)'
        ";
        return  "%s.%s(%d, %d, %d)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        self . _year ,;
        self . _month ,;
        self . _day );
        pub fn ctime ( self )  {
        "Return ctime() style string.";
        weekday = self . toordinal ( ) % 7 || 7;
        return  "%s %s %2d 00:00:00 %04d" % (;
        _DAYNAMES [ weekday ] ,;
        _MONTHNAMES [ self . _month ] ,;
        self . _day , self . _year );
        pub fn strftime ( &self, fmt )  {
        "
        Format using strftime().

        Example: "%d/%m/%Y, %H:%M:%S"
        ";
        return  _wrap_strftime ( self , fmt , self . timetuple ( ) );
        pub fn __format__ ( &self, fmt )  {
        if !isinstance ( fmt , str ) {
        panic!("TypeError ( "must be str, !%s" % type ( fmt ) . __name__ )");
        if len ( fmt ) != 0 {
        return  self . strftime ( fmt );
        return  str ( self );
        pub fn isoformat ( self )  {
        "Return the date formatted according to ISO.

        This == 'YYYY-MM-DD'.

        References:
        - http://www.w3.org/TR/NOTE-datetime
        - http://www.cl.cam.ac.uk/~mgk25/iso-time.html
        ";
        return  "%04d-%02d-%02d" % ( self . _year , self . _month , self . _day );
        __str__ = isoformat;
        @ property;
        pub fn year ( self )  {
        "year (1-9999)";
        return  self . _year;
        @ property;
        pub fn month ( self )  {
        "month (1-12)";
        return  self . _month;
        @ property;
        pub fn day ( self )  {
        "day (1-31)";
        return  self . _day;
        pub fn timetuple ( self )  {
        "Return local time tuple compatible with time.localtime().";
        return  _build_struct_time ( self . _year , self . _month , self . _day ,;
        0 , 0 , 0 , -1 );
        pub fn toordinal ( self )  {
        "Return proleptic Gregorian ordinal for the year, month && day.

        January 1 of year 1 == day 1.  Only the year, month && day values
        contribute to the result.
        ";
        return  _ymd2ord ( self . _year , self . _month , self . _day );
        pub fn replace ( &self, year = None /* Option */ , month = None /* Option */ , day = None /* Option */ )  {
        "Return a new date with new values for the specified fields.";
        if year is None /* Option */ {
        year = self . _year;
        if month is None /* Option */ {
        month = self . _month;
        if day is None /* Option */ {
        day = self . _day;
        return  type ( self ) ( year , month , day );
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , date ) {
        return  self . _cmp ( other ) == 0;
        return  NotImplemented;
        pub fn __le__ ( &self, other )  {
        if isinstance ( other , date ) {
        return  self . _cmp ( other ) <= 0;
        return  NotImplemented;
        pub fn __lt__ ( &self, other )  {
        if isinstance ( other , date ) {
        return  self . _cmp ( other ) < 0;
        return  NotImplemented;
        pub fn __ge__ ( &self, other )  {
        if isinstance ( other , date ) {
        return  self . _cmp ( other ) >= 0;
        return  NotImplemented;
        pub fn __gt__ ( &self, other )  {
        if isinstance ( other , date ) {
        return  self . _cmp ( other ) > 0;
        return  NotImplemented;
        pub fn _cmp ( &self, other )  {
        assert isinstance ( other , date );
        y , m , d = self . _year , self . _month , self . _day;
        y2 , m2 , d2 = other . _year , other . _month , other . _day;
        return  _cmp ( ( y , m , d ) , ( y2 , m2 , d2 ) );
        pub fn __hash__ ( self )  {
        "Hash.";
        if self . _hashcode == -1 {
        self . _hashcode = hash ( self . _getstate ( ) );
        return  self . _hashcode;
        pub fn __add__ ( &self, other )  {
        "Add a date to a timedelta.";
        if isinstance ( other , timedelta ) {
        o = self . toordinal ( ) + other . days;
        if 0 < o <= _MAXORDINAL {
        return  type ( self ) . fromordinal ( o );
        panic!("OverflowError ( "result out of range" )");
        return  NotImplemented;
        __radd__ = __add__;
        pub fn __sub__ ( &self, other )  {
        "Subtract two dates, || a date && a timedelta.";
        if isinstance ( other , timedelta ) {
        return  self + timedelta ( - other . days );
        if isinstance ( other , date ) {
        days1 = self . toordinal ( );
        days2 = other . toordinal ( );
        return  timedelta ( days1 - days2 );
        return  NotImplemented;
        pub fn weekday ( self )  {
        "Return day of the week, where Monday == 0 ... Sunday == 6.";
        return  ( self . toordinal ( ) + 6 ) % 7;
        pub fn isoweekday ( self )  {
        "Return day of the week, where Monday == 1 ... Sunday == 7.";
        return  self . toordinal ( ) % 7 || 7;
        pub fn isocalendar ( self )  {
        "Return a named tuple containing ISO year, week number, && weekday.

        The first ISO week of the year == the (Mon-Sun) week
        containing the year's first Thursday; everything else derives
        from that.

        The first week == 1; Monday == 1 ... Sunday == 7.

        ISO calendar algorithm taken from
        http://www.phys.uu.nl/~vgent/calendar/isocalendar.htm
        (used with permission)
        ";
        year = self . _year;
        week1monday = _isoweek1monday ( year );
        today = _ymd2ord ( self . _year , self . _month , self . _day );
        week , day = divmod ( today - week1monday , 7 );
        if week < 0 {
        year - = 1;
        week1monday = _isoweek1monday ( year );
        week , day = divmod ( today - week1monday , 7 );
        } else if week >= 52 {
        if today >= _isoweek1monday ( year + 1 ) {
        year + = 1;
        week = 0;
        return  _IsoCalendarDate ( year , week + 1 , day + 1 );
        pub fn _getstate ( self )  {
        yhi , ylo = divmod ( self . _year , 256 );
        return  bytes ( [ yhi , ylo , self . _month , self . _day ] ) ,;
        pub fn __setstate ( &self, string )  {
        yhi , ylo , self . _month , self . _day = string;
        self . _year = yhi * 256 + ylo;
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ , self . _getstate ( ) );
        _date_class = date;
        date . min = date ( 1 , 1 , 1 );
        date . max = date ( 9999 , 12 , 31 );
        date . resolution = timedelta ( days = 1 );
        class tzinfo ;
        "Abstract base class for time zone info classes.

    Subclasses must override the tzname(), utcoffset() && dst() methods.
    ";
        __slots__ = ( );
        pub fn tzname ( &self, dt )  {
        "datetime -> string name of time zone.";
        panic!("NotImplementedError ( "tzinfo subclass must override tzname()" )");
        pub fn utcoffset ( &self, dt )  {
        "datetime -> timedelta, positive for east of UTC, negative for west of UTC";
        panic!("NotImplementedError ( "tzinfo subclass must override utcoffset()" )");
        pub fn dst ( &self, dt )  {
        "datetime -> DST offset as timedelta, positive for east of UTC.

        Return 0 if DST !in effect.  utcoffset() must include the DST
        offset.
        ";
        panic!("NotImplementedError ( "tzinfo subclass must override dst()" )");
        pub fn fromutc ( &self, dt )  {
        "datetime in UTC -> datetime in local time.";
        if !isinstance ( dt , datetime ) {
        panic!("TypeError ( "fromutc() requires a datetime argument" )");
        if dt . tzinfo is !self {
        panic!("ValueError ( "dt.tzinfo is !self" )");
        dtoff = dt . utcoffset ( );
        if dtoff is None /* Option */ {
        panic!("ValueError ( "fromutc() requires a non-None /* Option */ utcoffset() "");
        "result" );
        dtdst = dt . dst ( );
        if dtdst is None /* Option */ {
        panic!("ValueError ( "fromutc() requires a non-None /* Option */ dst() result" )");
        delta = dtoff - dtdst;
        if delta {
        dt + = delta;
        dtdst = dt . dst ( );
        if dtdst is None /* Option */ {
        panic!("ValueError ( "fromutc(): dt.dst gave inconsistent "");
        "results; cannot convert" );
        return  dt + dtdst;
        pub fn __reduce__ ( self )  {
        getinitargs = getattr ( self , "__getinitargs__" , None /* Option */ );
        if getinitargs {
        args = getinitargs ( );
        } else {
        args = ( );
        return  ( self . __class__ , args , self . __getstate__ ( ) );
        class IsoCalendarDate ( tuple ) ;
        pub fn __new__ ( cls , year , week , weekday , / )  {
        return  super ( ) . __new__ ( cls , ( year , week , weekday ) );
        @ property;
        pub fn year ( self )  {
        return  self [ 0 ];
        @ property;
        pub fn week ( self )  {
        return  self [ 1 ];
        @ property;
        pub fn weekday ( self )  {
        return  self [ 2 ];
        pub fn __reduce__ ( self )  {
        return  ( tuple , ( tuple ( self ) , ) );
        pub fn __repr__ ( self )  {
        return  ( f "{self.__class__.__name__}";
        format!("(year={self[0]}, week={self[1]}, weekday={self[2]})" ));
        _IsoCalendarDate = IsoCalendarDate;
        del IsoCalendarDate;
        _tzinfo_class = tzinfo;
        class time ;
        "Time with time zone.

    Constructors:

    __new__()

    Operators:

    __repr__, __str__
    __eq__, __le__, __lt__, __ge__, __gt__, __hash__

    Methods:

    strftime()
    isoformat()
    utcoffset()
    tzname()
    dst()

    Properties (readonly):
    hour, minute, second, microsecond, tzinfo, fold
    ";
        __slots__ = "_hour" , "_minute" , "_second" , "_microsecond" , "_tzinfo" , "_hashcode" , "_fold";
        pub fn __new__ ( cls , hour = 0 , minute = 0 , second = 0 , microsecond = 0 , tzinfo = None /* Option */ , * , fold = 0 )  {
        "Constructor.

        Arguments:

        hour, minute (required)
        second, microsecond (default to zero)
        tzinfo (default to None /* Option */)
        fold (keyword only, default to zero)
        ";
        if ( isinstance ( hour , ( bytes , str ) ) && len ( hour ) == 6 and {
        ord ( hour [ 0 : 1 ] ) & 0x7 F < 24 ) ;
        if isinstance ( hour , str ) {
        // try {
        hour = hour . encode ( "latin1" );
        // } catch  UnicodeEncodeError  {
        panic!("ValueError (");
        "Failed to encode latin1 string when unpickling ";
        "a time object. ";
        "pickle.load(data, encoding='latin1') == assumed." );
        self = object . __new__ ( cls );
        self . __setstate ( hour , minute || None /* Option */ );
        self . _hashcode = -1;
        return  self;
        hour , minute , second , microsecond , fold = _check_time_fields (;
        hour , minute , second , microsecond , fold );
        _check_tzinfo_arg ( tzinfo );
        self = object . __new__ ( cls );
        self . _hour = hour;
        self . _minute = minute;
        self . _second = second;
        self . _microsecond = microsecond;
        self . _tzinfo = tzinfo;
        self . _hashcode = -1;
        self . _fold = fold;
        return  self;
        @ property;
        pub fn hour ( self )  {
        "hour (0-23)";
        return  self . _hour;
        @ property;
        pub fn minute ( self )  {
        "minute (0-59)";
        return  self . _minute;
        @ property;
        pub fn second ( self )  {
        "second (0-59)";
        return  self . _second;
        @ property;
        pub fn microsecond ( self )  {
        "microsecond (0-999999)";
        return  self . _microsecond;
        @ property;
        pub fn tzinfo ( self )  {
        "timezone info object";
        return  self . _tzinfo;
        @ property;
        pub fn fold ( self )  {
        return  self . _fold;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , time ) {
        return  self . _cmp ( other , allow_mixed = true ) == 0;
        } else {
        return  NotImplemented;
        pub fn __le__ ( &self, other )  {
        if isinstance ( other , time ) {
        return  self . _cmp ( other ) <= 0;
        } else {
        return  NotImplemented;
        pub fn __lt__ ( &self, other )  {
        if isinstance ( other , time ) {
        return  self . _cmp ( other ) < 0;
        } else {
        return  NotImplemented;
        pub fn __ge__ ( &self, other )  {
        if isinstance ( other , time ) {
        return  self . _cmp ( other ) >= 0;
        } else {
        return  NotImplemented;
        pub fn __gt__ ( &self, other )  {
        if isinstance ( other , time ) {
        return  self . _cmp ( other ) > 0;
        } else {
        return  NotImplemented;
        pub fn _cmp ( &self, other , allow_mixed = false )  {
        assert isinstance ( other , time );
        mytz = self . _tzinfo;
        ottz = other . _tzinfo;
        myoff = otoff = None /* Option */;
        if mytz is ottz {
        base_compare = true;
        } else {
        myoff = self . utcoffset ( );
        otoff = other . utcoffset ( );
        base_compare = myoff == otoff;
        if base_compare {
        return  _cmp ( ( self . _hour , self . _minute , self . _second ,;
        self . _microsecond ) ,;
        ( other . _hour , other . _minute , other . _second ,;
        other . _microsecond ) );
        if myoff is None /* Option */ || otoff is None /* Option */ {
        if allow_mixed {
        return  2;
        } else {
        panic!("TypeError ( "cannot compare naive && aware times" )");
        myhhmm = self . _hour * 60 + self . _minute - myoff / / timedelta ( minutes = 1 );
        othhmm = other . _hour * 60 + other . _minute - otoff / / timedelta ( minutes = 1 );
        return  _cmp ( ( myhhmm , self . _second , self . _microsecond ) ,;
        ( othhmm , other . _second , other . _microsecond ) );
        pub fn __hash__ ( self )  {
        "Hash.";
        if self . _hashcode == -1 {
        if self . fold {
        t = self . replace ( fold = 0 );
        } else {
        t = self;
        tzoff = t . utcoffset ( );
        if !tzoff {
        self . _hashcode = hash ( t . _getstate ( ) [ 0 ] );
        } else {
        h , m = divmod ( timedelta ( hours = self . hour , minutes = self . minute ) - tzoff ,;
        timedelta ( hours = 1 ) );
        assert !m % timedelta ( minutes = 1 ) , "whole minute";
        m / / = timedelta ( minutes = 1 );
        if 0 <= h < 24 {
        self . _hashcode = hash ( time ( h , m , self . second , self . microsecond ) );
        } else {
        self . _hashcode = hash ( ( h , m , self . second , self . microsecond ) );
        return  self . _hashcode;
        pub fn _tzstr ( self )  {
        "Return formatted timezone offset (+xx:xx) || an empty string.";
        off = self . utcoffset ( );
        return  _format_offset ( off );
        pub fn __repr__ ( self )  {
        "Convert to formal string, for repr().";
        if self . _microsecond != 0 {
        s = ", %d, %d" % ( self . _second , self . _microsecond );
        } else if self . _second != 0 {
        s = ", %d" % self . _second;
        } else {
        s = "";
        s = "%s.%s(%d, %d%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        self . _hour , self . _minute , s );
        if self . _tzinfo is !None /* Option */ {
        assert s [ -1 : ] == ")";
        s = s [ : -1 ] + ", tzinfo=%r" % self . _tzinfo + ")";
        if self . _fold {
        assert s [ -1 : ] == ")";
        s = s [ : -1 ] + ", fold=1)";
        return  s;
        pub fn isoformat ( &self, timespec = "auto" )  {
        "Return the time formatted according to ISO.

        The full format == 'HH:MM:SS.mmmmmm+zz:zz'. By default, the fractional
        part == omitted if self.microsecond == 0.

        The optional argument timespec specifies the number of additional
        terms of the time to include. Valid options are 'auto', 'hours',
        'minutes', 'seconds', 'milliseconds' && 'microseconds'.
        ";
        s = _format_time ( self . _hour , self . _minute , self . _second ,;
        self . _microsecond , timespec );
        tz = self . _tzstr ( );
        if tz {
        s + = tz;
        return  s;
        __str__ = isoformat;
        @ classmethod;
        pub fn fromisoformat ( cls , time_string )  {
        "Construct a time from a string in one of the ISO 8601 formats.";
        if !isinstance ( time_string , str ) {
        panic!("TypeError ( "fromisoformat: argument must be str" )");
        time_string = time_string . removeprefix ( "T" );
        // try {
        return  cls ( * _parse_isoformat_time ( time_string ) );
        // } catch  Exception  {
        panic!("ValueError ( f "Invalid isoformat string: {time_string!r}" )");
        pub fn strftime ( &self, fmt )  {
        "Format using strftime().  The date part of the timestamp passed
        to underlying strftime should !be used.
        ";
        timetuple = ( 1900 , 1 , 1 ,;
        self . _hour , self . _minute , self . _second ,;
        0 , 1 , -1 );
        return  _wrap_strftime ( self , fmt , timetuple );
        pub fn __format__ ( &self, fmt )  {
        if !isinstance ( fmt , str ) {
        panic!("TypeError ( "must be str, !%s" % type ( fmt ) . __name__ )");
        if len ( fmt ) != 0 {
        return  self . strftime ( fmt );
        return  str ( self );
        pub fn utcoffset ( self )  {
        "Return the timezone offset as timedelta, positive east of UTC
         (negative west of UTC).";
        if self . _tzinfo is None /* Option */ {
        return;
        offset = self . _tzinfo . utcoffset ( None /* Option */ );
        _check_utc_offset ( "utcoffset" , offset );
        return  offset;
        pub fn tzname ( self )  {
        "Return the timezone name.

        Note that the name == 100% informational -- there's no requirement that
        it mean anything in particular. For example, "GMT", "UTC", "-500",
        "-5:00", "EDT", "US/Eastern", "America/New York" are all valid replies.
        ";
        if self . _tzinfo is None /* Option */ {
        return;
        name = self . _tzinfo . tzname ( None /* Option */ );
        _check_tzname ( name );
        return  name;
        pub fn dst ( self )  {
        "Return 0 if DST == !in effect, || the DST offset (as timedelta
        positive eastward) if DST == in effect.

        This == purely informational; the DST offset has already been added to
        the UTC offset returned by utcoffset() if applicable, so there's no
        need to consult dst() unless you're interested in displaying the DST
        info.
        ";
        if self . _tzinfo is None /* Option */ {
        return;
        offset = self . _tzinfo . dst ( None /* Option */ );
        _check_utc_offset ( "dst" , offset );
        return  offset;
        pub fn replace ( &self, hour = None /* Option */ , minute = None /* Option */ , second = None /* Option */ , microsecond = None /* Option */ , {
        tzinfo = true , * , fold = None /* Option */ ) ;
        "Return a new time with new values for the specified fields.";
        if hour is None /* Option */ {
        hour = self . hour;
        if minute is None /* Option */ {
        minute = self . minute;
        if second is None /* Option */ {
        second = self . second;
        if microsecond is None /* Option */ {
        microsecond = self . microsecond;
        if tzinfo is true {
        tzinfo = self . tzinfo;
        if fold is None /* Option */ {
        fold = self . _fold;
        return  type ( self ) ( hour , minute , second , microsecond , tzinfo , fold = fold );
        pub fn _getstate ( &self, protocol = 3 )  {
        us2 , us3 = divmod ( self . _microsecond , 256 );
        us1 , us2 = divmod ( us2 , 256 );
        h = self . _hour;
        if self . _fold && protocol > 3 {
        h + = 128;
        basestate = bytes ( [ h , self . _minute , self . _second ,;
        us1 , us2 , us3 ] );
        if self . _tzinfo is None /* Option */ {
        return  ( basestate , );
        } else {
        return  ( basestate , self . _tzinfo );
        pub fn __setstate ( &self, string , tzinfo )  {
        if tzinfo is !None /* Option */ && !isinstance ( tzinfo , _tzinfo_class ) {
        panic!("TypeError ( "bad tzinfo state arg" )");
        h , self . _minute , self . _second , us1 , us2 , us3 = string;
        if h > 127 {
        self . _fold = 1;
        self . _hour = h - 128;
        } else {
        self . _fold = 0;
        self . _hour = h;
        self . _microsecond = ( ( ( us1 < < 8 ) | us2 ) < < 8 ) | us3;
        self . _tzinfo = tzinfo;
        pub fn __reduce_ex__ ( &self, protocol )  {
        return  ( self . __class__ , self . _getstate ( protocol ) );
        pub fn __reduce__ ( self )  {
        return  self . __reduce_ex__ ( 2 );
        _time_class = time;
        time . min = time ( 0 , 0 , 0 );
        time . max = time ( 23 , 59 , 59 , 999999 );
        time . resolution = timedelta ( microseconds = 1 );
        class datetime ( date ) ;
        "datetime(year, month, day[, hour[, minute[, second[, microsecond[,tzinfo]]]]])

    The year, month && day arguments are required. tzinfo may be None /* Option */, || an
    instance of a tzinfo subclass. The remaining arguments may be ints.
    ";
        __slots__ = date . __slots__ + time . __slots__;
        pub fn __new__ ( cls , year , month = None /* Option */ , day = None /* Option */ , hour = 0 , minute = 0 , second = 0 , {
        microsecond = 0 , tzinfo = None /* Option */ , * , fold = 0 ) ;
        if ( isinstance ( year , ( bytes , str ) ) && len ( year ) == 10 and {
        1 <= ord ( year [ 2 : 3 ] ) & 0x7 F <= 12 ) ;
        if isinstance ( year , str ) {
        // try {
        year = bytes ( year , "latin1" );
        // } catch  UnicodeEncodeError  {
        panic!("ValueError (");
        "Failed to encode latin1 string when unpickling ";
        "a datetime object. ";
        "pickle.load(data, encoding='latin1') == assumed." );
        self = object . __new__ ( cls );
        self . __setstate ( year , month );
        self . _hashcode = -1;
        return  self;
        year , month , day = _check_date_fields ( year , month , day );
        hour , minute , second , microsecond , fold = _check_time_fields (;
        hour , minute , second , microsecond , fold );
        _check_tzinfo_arg ( tzinfo );
        self = object . __new__ ( cls );
        self . _year = year;
        self . _month = month;
        self . _day = day;
        self . _hour = hour;
        self . _minute = minute;
        self . _second = second;
        self . _microsecond = microsecond;
        self . _tzinfo = tzinfo;
        self . _hashcode = -1;
        self . _fold = fold;
        return  self;
        @ property;
        pub fn hour ( self )  {
        "hour (0-23)";
        return  self . _hour;
        @ property;
        pub fn minute ( self )  {
        "minute (0-59)";
        return  self . _minute;
        @ property;
        pub fn second ( self )  {
        "second (0-59)";
        return  self . _second;
        @ property;
        pub fn microsecond ( self )  {
        "microsecond (0-999999)";
        return  self . _microsecond;
        @ property;
        pub fn tzinfo ( self )  {
        "timezone info object";
        return  self . _tzinfo;
        @ property;
        pub fn fold ( self )  {
        return  self . _fold;
        @ classmethod;
        pub fn _fromtimestamp ( cls , t , utc , tz )  {
        "Construct a datetime from a POSIX timestamp (like time.time()).

        A timezone info object may be passed in as well.
        ";
        frac , t = _math . modf ( t );
        us = round ( frac * 1e6 );
        if us >= 1000000 {
        t + = 1;
        us - = 1000000;
        } else if us < 0 {
        t - = 1;
        us + = 1000000;
        converter = _time . gmtime if utc else _time . localtime;
        y , m , d , hh , mm , ss , weekday , jday , dst = converter ( t );
        ss = min ( ss , 59 );
        result = cls ( y , m , d , hh , mm , ss , us , tz );
        if tz is None /* Option */ && !utc {
        max_fold_seconds = 24 * 3600;
        if t < max_fold_seconds && sys . platform . startswith ( "win" ) {
        return  result;
        y , m , d , hh , mm , ss = converter ( t - max_fold_seconds ) [ : 6 ];
        probe1 = cls ( y , m , d , hh , mm , ss , us , tz );
        trans = result - probe1 - timedelta ( 0 , max_fold_seconds );
        if trans . days < 0 {
        y , m , d , hh , mm , ss = converter ( t + trans / / timedelta ( 0 , 1 ) ) [ : 6 ];
        probe2 = cls ( y , m , d , hh , mm , ss , us , tz );
        if probe2 == result {
        result . _fold = 1;
        } else if tz is !None /* Option */ {
        result = tz . fromutc ( result );
        return  result;
        @ classmethod;
        pub fn fromtimestamp ( cls , t , tz = None /* Option */ )  {
        "Construct a datetime from a POSIX timestamp (like time.time()).

        A timezone info object may be passed in as well.
        ";
        _check_tzinfo_arg ( tz );
        return  cls . _fromtimestamp ( t , tz is !None /* Option */ , tz );
        @ classmethod;
        pub fn utcfromtimestamp ( cls , t )  {
        "Construct a naive UTC datetime from a POSIX timestamp.";
        return  cls . _fromtimestamp ( t , true , None /* Option */ );
        @ classmethod;
        pub fn now ( cls , tz = None /* Option */ )  {
        "Construct a datetime from time.time() && optional time zone info.";
        t = _time . time ( );
        return  cls . fromtimestamp ( t , tz );
        @ classmethod;
        pub fn utcnow ( cls )  {
        "Construct a UTC datetime from time.time().";
        t = _time . time ( );
        return  cls . utcfromtimestamp ( t );
        @ classmethod;
        pub fn combine ( cls , date , time , tzinfo = true )  {
        "Construct a datetime from a given date && a given time.";
        if !isinstance ( date , _date_class ) {
        panic!("TypeError ( "date argument must be a date instance" )");
        if !isinstance ( time , _time_class ) {
        panic!("TypeError ( "time argument must be a time instance" )");
        if tzinfo is true {
        tzinfo = time . tzinfo;
        return  cls ( date . year , date . month , date . day ,;
        time . hour , time . minute , time . second , time . microsecond ,;
        tzinfo , fold = time . fold );
        @ classmethod;
        pub fn fromisoformat ( cls , date_string )  {
        "Construct a datetime from a string in one of the ISO 8601 formats.";
        if !isinstance ( date_string , str ) {
        panic!("TypeError ( "fromisoformat: argument must be str" )");
        if len ( date_string ) < 7 {
        panic!("ValueError ( f "Invalid isoformat string: {date_string!r}" )");
        // try {
        separator_location = _find_isoformat_datetime_separator ( date_string );
        dstr = date_string [ 0 : separator_location ];
        tstr = date_string [ ( separator_location + 1 ) : ];
        date_components = _parse_isoformat_date ( dstr );
        // } catch  ValueError  {
        panic!("ValueError (");
        format!("Invalid isoformat string: {date_string!r}" ) from None /* Option */);
        if tstr {
        // try {
        time_components = _parse_isoformat_time ( tstr );
        // } catch  ValueError  {
        panic!("ValueError (");
        format!("Invalid isoformat string: {date_string!r}" ) from None /* Option */);
        } else {
        time_components = [ 0 , 0 , 0 , 0 , None /* Option */ ];
        return  cls ( * ( date_components + time_components ) );
        pub fn timetuple ( self )  {
        "Return local time tuple compatible with time.localtime().";
        dst = self . dst ( );
        if dst is None /* Option */ {
        dst = -1;
        } else if dst {
        dst = 1;
        } else {
        dst = 0;
        return  _build_struct_time ( self . year , self . month , self . day ,;
        self . hour , self . minute , self . second ,;
        dst );
        pub fn _mktime ( self )  {
        "Return integer POSIX timestamp.";
        epoch = datetime ( 1970 , 1 , 1 );
        max_fold_seconds = 24 * 3600;
        t = ( self - epoch ) / / timedelta ( 0 , 1 );
        pub fn local ( u )  {
        y , m , d , hh , mm , ss = _time . localtime ( u ) [ : 6 ];
        return  ( datetime ( y , m , d , hh , mm , ss ) - epoch ) / / timedelta ( 0 , 1 );
        a = local ( t ) - t;
        u1 = t - a;
        t1 = local ( u1 );
        if t1 == t {
        u2 = u1 + ( - max_fold_seconds , max_fold_seconds ) [ self . fold ];
        b = local ( u2 ) - u2;
        if a == b {
        return  u1;
        } else {
        b = t1 - u1;
        assert a != b;
        u2 = t - b;
        t2 = local ( u2 );
        if t2 == t {
        return  u2;
        if t1 == t {
        return  u1;
        return  ( max , min ) [ self . fold ] ( u1 , u2 );
        pub fn timestamp ( self )  {
        "Return POSIX timestamp as float";
        if self . _tzinfo is None /* Option */ {
        s = self . _mktime ( );
        return  s + self . microsecond / 1e6;
        } else {
        return  ( self - _EPOCH ) . total_seconds ( );
        pub fn utctimetuple ( self )  {
        "Return UTC time tuple compatible with time.gmtime().";
        offset = self . utcoffset ( );
        if offset {
        self - = offset;
        y , m , d = self . year , self . month , self . day;
        hh , mm , ss = self . hour , self . minute , self . second;
        return  _build_struct_time ( y , m , d , hh , mm , ss , 0 );
        pub fn date ( self )  {
        "Return the date part.";
        return  date ( self . _year , self . _month , self . _day );
        pub fn time ( self )  {
        "Return the time part, with tzinfo None /* Option */.";
        return  time ( self . hour , self . minute , self . second , self . microsecond , fold = self . fold );
        pub fn timetz ( self )  {
        "Return the time part, with same tzinfo.";
        return  time ( self . hour , self . minute , self . second , self . microsecond ,;
        self . _tzinfo , fold = self . fold );
        pub fn replace ( &self, year = None /* Option */ , month = None /* Option */ , day = None /* Option */ , hour = None /* Option */ , {
        minute = None /* Option */ , second = None /* Option */ , microsecond = None /* Option */ , tzinfo = true ,;
        * , fold = None /* Option */ ) ;
        "Return a new datetime with new values for the specified fields.";
        if year is None /* Option */ {
        year = self . year;
        if month is None /* Option */ {
        month = self . month;
        if day is None /* Option */ {
        day = self . day;
        if hour is None /* Option */ {
        hour = self . hour;
        if minute is None /* Option */ {
        minute = self . minute;
        if second is None /* Option */ {
        second = self . second;
        if microsecond is None /* Option */ {
        microsecond = self . microsecond;
        if tzinfo is true {
        tzinfo = self . tzinfo;
        if fold is None /* Option */ {
        fold = self . fold;
        return  type ( self ) ( year , month , day , hour , minute , second ,;
        microsecond , tzinfo , fold = fold );
        pub fn _local_timezone ( self )  {
        if self . tzinfo is None /* Option */ {
        ts = self . _mktime ( );
        } else {
        ts = ( self - _EPOCH ) / / timedelta ( seconds = 1 );
        localtm = _time . localtime ( ts );
        local = datetime ( * localtm [ : 6 ] );
        gmtoff = localtm . tm_gmtoff;
        zone = localtm . tm_zone;
        return  timezone ( timedelta ( seconds = gmtoff ) , zone );
        pub fn astimezone ( &self, tz = None /* Option */ )  {
        if tz is None /* Option */ {
        tz = self . _local_timezone ( );
        } else if !isinstance ( tz , tzinfo ) {
        panic!("TypeError ( "tz argument must be an instance of tzinfo" )");
        mytz = self . tzinfo;
        if mytz is None /* Option */ {
        mytz = self . _local_timezone ( );
        myoffset = mytz . utcoffset ( self );
        } else {
        myoffset = mytz . utcoffset ( self );
        if myoffset is None /* Option */ {
        mytz = self . replace ( tzinfo = None /* Option */ ) . _local_timezone ( );
        myoffset = mytz . utcoffset ( self );
        if tz is mytz {
        return  self;
        utc = ( self - myoffset ) . replace ( tzinfo = tz );
        return  tz . fromutc ( utc );
        pub fn ctime ( self )  {
        "Return ctime() style string.";
        weekday = self . toordinal ( ) % 7 || 7;
        return  "%s %s %2d %02d:%02d:%02d %04d" % (;
        _DAYNAMES [ weekday ] ,;
        _MONTHNAMES [ self . _month ] ,;
        self . _day ,;
        self . _hour , self . _minute , self . _second ,;
        self . _year );
        pub fn isoformat ( &self, sep = "T" , timespec = "auto" )  {
        "Return the time formatted according to ISO.

        The full format looks like 'YYYY-MM-DD HH:MM:SS.mmmmmm'.
        By default, the fractional part == omitted if self.microsecond == 0.

        If self.tzinfo == !None /* Option */, the UTC offset == also attached, giving
        giving a full format of 'YYYY-MM-DD HH:MM:SS.mmmmmm+HH:MM'.

        Optional argument sep specifies the separator between date and
        time, default 'T'.

        The optional argument timespec specifies the number of additional
        terms of the time to include. Valid options are 'auto', 'hours',
        'minutes', 'seconds', 'milliseconds' && 'microseconds'.
        ";
        s = ( "%04d-%02d-%02d%c" % ( self . _year , self . _month , self . _day , sep ) +;
        _format_time ( self . _hour , self . _minute , self . _second ,;
        self . _microsecond , timespec ) );
        off = self . utcoffset ( );
        tz = _format_offset ( off );
        if tz {
        s + = tz;
        return  s;
        pub fn __repr__ ( self )  {
        "Convert to formal string, for repr().";
        L = [ self . _year , self . _month , self . _day ,;
        self . _hour , self . _minute , self . _second , self . _microsecond ];
        if L [ -1 ] == 0 {
        del L [ -1 ];
        if L [ -1 ] == 0 {
        del L [ -1 ];
        s = "%s.%s(%s)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        ", " . join ( map ( str , L ) ) );
        if self . _tzinfo is !None /* Option */ {
        assert s [ -1 : ] == ")";
        s = s [ : -1 ] + ", tzinfo=%r" % self . _tzinfo + ")";
        if self . _fold {
        assert s [ -1 : ] == ")";
        s = s [ : -1 ] + ", fold=1)";
        return  s;
        pub fn __str__ ( self )  {
        "Convert to string, for str().";
        return  self . isoformat ( sep = " " );
        @ classmethod;
        pub fn strptime ( cls , date_string , format )  {
        "string, format -> new datetime parsed from a string (like time.strptime()).";
        import _strptime;
        return  _strptime . _strptime_datetime ( cls , date_string , format );
        pub fn utcoffset ( self )  {
        "Return the timezone offset as timedelta positive east of UTC (negative west of
        UTC).";
        if self . _tzinfo is None /* Option */ {
        return;
        offset = self . _tzinfo . utcoffset ( self );
        _check_utc_offset ( "utcoffset" , offset );
        return  offset;
        pub fn tzname ( self )  {
        "Return the timezone name.

        Note that the name == 100% informational -- there's no requirement that
        it mean anything in particular. For example, "GMT", "UTC", "-500",
        "-5:00", "EDT", "US/Eastern", "America/New York" are all valid replies.
        ";
        if self . _tzinfo is None /* Option */ {
        return;
        name = self . _tzinfo . tzname ( self );
        _check_tzname ( name );
        return  name;
        pub fn dst ( self )  {
        "Return 0 if DST == !in effect, || the DST offset (as timedelta
        positive eastward) if DST == in effect.

        This == purely informational; the DST offset has already been added to
        the UTC offset returned by utcoffset() if applicable, so there's no
        need to consult dst() unless you're interested in displaying the DST
        info.
        ";
        if self . _tzinfo is None /* Option */ {
        return;
        offset = self . _tzinfo . dst ( self );
        _check_utc_offset ( "dst" , offset );
        return  offset;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , datetime ) {
        return  self . _cmp ( other , allow_mixed = true ) == 0;
        } else if !isinstance ( other , date ) {
        return  NotImplemented;
        } else {
        return  false;
        pub fn __le__ ( &self, other )  {
        if isinstance ( other , datetime ) {
        return  self . _cmp ( other ) <= 0;
        } else if !isinstance ( other , date ) {
        return  NotImplemented;
        } else {
        _cmperror ( self , other );
        pub fn __lt__ ( &self, other )  {
        if isinstance ( other , datetime ) {
        return  self . _cmp ( other ) < 0;
        } else if !isinstance ( other , date ) {
        return  NotImplemented;
        } else {
        _cmperror ( self , other );
        pub fn __ge__ ( &self, other )  {
        if isinstance ( other , datetime ) {
        return  self . _cmp ( other ) >= 0;
        } else if !isinstance ( other , date ) {
        return  NotImplemented;
        } else {
        _cmperror ( self , other );
        pub fn __gt__ ( &self, other )  {
        if isinstance ( other , datetime ) {
        return  self . _cmp ( other ) > 0;
        } else if !isinstance ( other , date ) {
        return  NotImplemented;
        } else {
        _cmperror ( self , other );
        pub fn _cmp ( &self, other , allow_mixed = false )  {
        assert isinstance ( other , datetime );
        mytz = self . _tzinfo;
        ottz = other . _tzinfo;
        myoff = otoff = None /* Option */;
        if mytz is ottz {
        base_compare = true;
        } else {
        myoff = self . utcoffset ( );
        otoff = other . utcoffset ( );
        if allow_mixed {
        if myoff != self . replace ( fold = !self . fold ) . utcoffset ( ) {
        return  2;
        if otoff != other . replace ( fold = !other . fold ) . utcoffset ( ) {
        return  2;
        base_compare = myoff == otoff;
        if base_compare {
        return  _cmp ( ( self . _year , self . _month , self . _day ,;
        self . _hour , self . _minute , self . _second ,;
        self . _microsecond ) ,;
        ( other . _year , other . _month , other . _day ,;
        other . _hour , other . _minute , other . _second ,;
        other . _microsecond ) );
        if myoff is None /* Option */ || otoff is None /* Option */ {
        if allow_mixed {
        return  2;
        } else {
        panic!("TypeError ( "cannot compare naive && aware datetimes" )");
        diff = self - other;
        if diff . days < 0 {
        return  -1;
        return  diff && 1 || 0;
        pub fn __add__ ( &self, other )  {
        "Add a datetime && a timedelta.";
        if !isinstance ( other , timedelta ) {
        return  NotImplemented;
        delta = timedelta ( self . toordinal ( ) ,;
        hours = self . _hour ,;
        minutes = self . _minute ,;
        seconds = self . _second ,;
        microseconds = self . _microsecond );
        delta + = other;
        hour , rem = divmod ( delta . seconds , 3600 );
        minute , second = divmod ( rem , 60 );
        if 0 < delta . days <= _MAXORDINAL {
        return  type ( self ) . combine ( date . fromordinal ( delta . days ) ,;
        time ( hour , minute , second ,;
        delta . microseconds ,;
        tzinfo = self . _tzinfo ) );
        panic!("OverflowError ( "result out of range" )");
        __radd__ = __add__;
        pub fn __sub__ ( &self, other )  {
        "Subtract two datetimes, || a datetime && a timedelta.";
        if !isinstance ( other , datetime ) {
        if isinstance ( other , timedelta ) {
        return  self + - other;
        return  NotImplemented;
        days1 = self . toordinal ( );
        days2 = other . toordinal ( );
        secs1 = self . _second + self . _minute * 60 + self . _hour * 3600;
        secs2 = other . _second + other . _minute * 60 + other . _hour * 3600;
        base = timedelta ( days1 - days2 ,;
        secs1 - secs2 ,;
        self . _microsecond - other . _microsecond );
        if self . _tzinfo is other . _tzinfo {
        return  base;
        myoff = self . utcoffset ( );
        otoff = other . utcoffset ( );
        if myoff == otoff {
        return  base;
        if myoff is None /* Option */ || otoff is None /* Option */ {
        panic!("TypeError ( "cannot mix naive && timezone-aware time" )");
        return  base + otoff - myoff;
        pub fn __hash__ ( self )  {
        if self . _hashcode == -1 {
        if self . fold {
        t = self . replace ( fold = 0 );
        } else {
        t = self;
        tzoff = t . utcoffset ( );
        if tzoff is None /* Option */ {
        self . _hashcode = hash ( t . _getstate ( ) [ 0 ] );
        } else {
        days = _ymd2ord ( self . year , self . month , self . day );
        seconds = self . hour * 3600 + self . minute * 60 + self . second;
        self . _hashcode = hash ( timedelta ( days , seconds , self . microsecond ) - tzoff );
        return  self . _hashcode;
        pub fn _getstate ( &self, protocol = 3 )  {
        yhi , ylo = divmod ( self . _year , 256 );
        us2 , us3 = divmod ( self . _microsecond , 256 );
        us1 , us2 = divmod ( us2 , 256 );
        m = self . _month;
        if self . _fold && protocol > 3 {
        m + = 128;
        basestate = bytes ( [ yhi , ylo , m , self . _day ,;
        self . _hour , self . _minute , self . _second ,;
        us1 , us2 , us3 ] );
        if self . _tzinfo is None /* Option */ {
        return  ( basestate , );
        } else {
        return  ( basestate , self . _tzinfo );
        pub fn __setstate ( &self, string , tzinfo )  {
        if tzinfo is !None /* Option */ && !isinstance ( tzinfo , _tzinfo_class ) {
        panic!("TypeError ( "bad tzinfo state arg" )");
        ( yhi , ylo , m , self . _day , self . _hour ,;
        self . _minute , self . _second , us1 , us2 , us3 ) = string;
        if m > 127 {
        self . _fold = 1;
        self . _month = m - 128;
        } else {
        self . _fold = 0;
        self . _month = m;
        self . _year = yhi * 256 + ylo;
        self . _microsecond = ( ( ( us1 < < 8 ) | us2 ) < < 8 ) | us3;
        self . _tzinfo = tzinfo;
        pub fn __reduce_ex__ ( &self, protocol )  {
        return  ( self . __class__ , self . _getstate ( protocol ) );
        pub fn __reduce__ ( self )  {
        return  self . __reduce_ex__ ( 2 );
        datetime . min = datetime ( 1 , 1 , 1 );
        datetime . max = datetime ( 9999 , 12 , 31 , 23 , 59 , 59 , 999999 );
        datetime . resolution = timedelta ( microseconds = 1 );
        pub fn _isoweek1monday ( year )  {
        THURSDAY = 3;
        firstday = _ymd2ord ( year , 1 , 1 );
        firstweekday = ( firstday + 6 ) % 7;
        week1monday = firstday - firstweekday;
        if firstweekday > THURSDAY {
        week1monday + = 7;
        return  week1monday;
        class timezone ( tzinfo ) ;
        __slots__ = "_offset" , "_name";
        _Omitted = object ( );
        pub fn __new__ ( cls , offset , name = _Omitted )  {
        if !isinstance ( offset , timedelta ) {
        panic!("TypeError ( "offset must be a timedelta" )");
        if name is cls . _Omitted {
        if !offset {
        return  cls . utc;
        name = None /* Option */;
        } else if !isinstance ( name , str ) {
        panic!("TypeError ( "name must be a string" )");
        if !cls . _minoffset <= offset <= cls . _maxoffset {
        panic!("ValueError ( "offset must be a timedelta "");
        "strictly between -timedelta(hours=24) && ";
        "timedelta(hours=24)." );
        return  cls . _create ( offset , name );
        @ classmethod;
        pub fn _create ( cls , offset , name = None /* Option */ )  {
        self = tzinfo . __new__ ( cls );
        self . _offset = offset;
        self . _name = name;
        return  self;
        pub fn __getinitargs__ ( self )  {
        "pickle support";
        if self . _name is None /* Option */ {
        return  ( self . _offset , );
        return  ( self . _offset , self . _name );
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , timezone ) {
        return  self . _offset == other . _offset;
        return  NotImplemented;
        pub fn __hash__ ( self )  {
        return  hash ( self . _offset );
        pub fn __repr__ ( self )  {
        "Convert to formal string, for repr().

        >>> tz = timezone.utc
        >>> repr(tz)
        'datetime.timezone.utc'
        >>> tz = timezone(timedelta(hours=-5), 'EST')
        >>> repr(tz)
        "datetime.timezone(datetime.timedelta(-1, 68400), 'EST')"
        ";
        if self is self . utc {
        return  "datetime.timezone.utc";
        if self . _name is None /* Option */ {
        return  "%s.%s(%r)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        self . _offset );
        return  "%s.%s(%r, %r)" % ( self . __class__ . __module__ ,;
        self . __class__ . __qualname__ ,;
        self . _offset , self . _name );
        pub fn __str__ ( self )  {
        return  self . tzname ( None /* Option */ );
        pub fn utcoffset ( &self, dt )  {
        if isinstance ( dt , datetime ) || dt is None /* Option */ {
        return  self . _offset;
        panic!("TypeError ( "utcoffset() argument must be a datetime instance"");
        " || None /* Option */" );
        pub fn tzname ( &self, dt )  {
        if isinstance ( dt , datetime ) || dt is None /* Option */ {
        if self . _name is None /* Option */ {
        return  self . _name_from_offset ( self . _offset );
        return  self . _name;
        panic!("TypeError ( "tzname() argument must be a datetime instance"");
        " || None /* Option */" );
        pub fn dst ( &self, dt )  {
        if isinstance ( dt , datetime ) || dt is None /* Option */ {
        return;
        panic!("TypeError ( "dst() argument must be a datetime instance"");
        " || None /* Option */" );
        pub fn fromutc ( &self, dt )  {
        if isinstance ( dt , datetime ) {
        if dt . tzinfo is !self {
        panic!("ValueError ( "fromutc: dt.tzinfo "");
        "is !selformat!(" ));
        return  dt + self . _offset;
        panic!("TypeError ( "fromutc() argument must be a datetime instance"");
        " || None /* Option */" );
        _maxoffset = timedelta ( hours = 24 , microseconds = -1 );
        _minoffset = - _maxoffset;
        @ staticmethod;
        pub fn _name_from_offset ( delta )  {
        if !delta {
        return  "UTC";
        if delta < timedelta ( 0 ) {
        sign = "-";
        delta = - delta;
        } else {
        sign = "+";
        hours , rest = divmod ( delta , timedelta ( hours = 1 ) );
        minutes , rest = divmod ( rest , timedelta ( minutes = 1 ) );
        seconds = rest . seconds;
        microseconds = rest . microseconds;
        if microseconds {
        return  ( f "UTC{sign}{hours:02d}:{minutes:02d}:{seconds:02d}";
        format!(".{microseconds:06d}" ));
        if seconds {
        return  f "UTC{sign}{hours:02d}:{minutes:02d}:{seconds:02d}";
        return  f "UTC{sign}{hours:02d}:{minutes:02d}";
        UTC = timezone . utc = timezone . _create ( timedelta ( 0 ) );
        timezone . min = timezone . _create ( - timedelta ( hours = 23 , minutes = 59 ) );
        timezone . max = timezone . _create ( timedelta ( hours = 23 , minutes = 59 ) );
        _EPOCH = datetime ( 1970 , 1 , 1 , tzinfo = timezone . utc );
        // try {
        from _datetime import *;
        // } catch  ImportError  {
        // pass
        } else {
        del ( _DAYNAMES , _DAYS_BEFORE_MONTH , _DAYS_IN_MONTH , _DI100Y , _DI400Y ,;
        _DI4Y , _EPOCH , _MAXORDINAL , _MONTHNAMES , _build_struct_time ,;
        _check_date_fields , _check_time_fields ,;
        _check_tzinfo_arg , _check_tzname , _check_utc_offset , _cmp , _cmperror ,;
        _date_class , _days_before_month , _days_before_year , _days_in_month ,;
        _format_time , _format_offset , _index , _is_leap , _isoweek1monday , _math ,;
        _ord2ymd , _time , _time_class , _tzinfo_class , _wrap_strftime , _ymd2ord ,;
        _divide_and_round , _parse_isoformat_date , _parse_isoformat_time ,;
        _parse_hh_mm_ss_ff , _IsoCalendarDate , _isoweek_to_gregorian ,;
        _find_isoformat_datetime_separator , _FRACTION_CORRECTION ,;
        _is_ascii_digit );
        from _datetime import __doc__;
}

