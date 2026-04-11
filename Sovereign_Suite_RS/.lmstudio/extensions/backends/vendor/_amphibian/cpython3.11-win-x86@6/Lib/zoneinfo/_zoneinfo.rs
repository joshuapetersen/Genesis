//! _zoneinfo.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::bisect;
// use std::collections;
// use regex::Regex;
// use chrono::Utc::{datetime, timedelta, tzinfo};
// use crate::.::{_common, _tzpath};
// use crate::pickle;

pub const EPOCH: f64 = datetime ( 1970 , 1 , 1 );
pub const EPOCHORDINAL: f64 = datetime ( 1970 , 1 , 1 ) . toordinal ( );
pub const maxsize: u64 = 512 );
pub fn _load_timedelta(seconds: &str) {
        return  timedelta ( seconds = seconds );
        class ZoneInfo ( tzinfo ) ;
        _strong_cache_size = 8;
        _strong_cache = collections . OrderedDict ( );
        _weak_cache = weakref . WeakValueDictionary ( );
        __module__ = "zoneinfo";
        pub fn __init_subclass__ ( cls )  {
        cls . _strong_cache = collections . OrderedDict ( );
        cls . _weak_cache = weakref . WeakValueDictionary ( );
        pub fn __new__ ( cls , key )  {
        instance = cls . _weak_cache . get ( key , None /* Option */ );
        if instance is None /* Option */ {
        instance = cls . _weak_cache . setdefault ( key , cls . _new_instance ( key ) );
        instance . _from_cache = true;
        cls . _strong_cache [ key ] = cls . _strong_cache . pop ( key , instance );
        if len ( cls . _strong_cache ) > cls . _strong_cache_size {
        cls . _strong_cache . popitem ( last = false );
        return  instance;
        @ classmethod;
        pub fn no_cache ( cls , key )  {
        obj = cls . _new_instance ( key );
        obj . _from_cache = false;
        return  obj;
        @ classmethod;
        pub fn _new_instance ( cls , key )  {
        obj = super ( ) . __new__ ( cls );
        obj . _key = key;
        obj . _file_path = obj . _find_tzfile ( key );
        if obj . _file_path is !None /* Option */ {
        file_obj = open ( obj . _file_path , "rb" );
        } else {
        file_obj = _common . load_tzdata ( key );
        // with scope: file_obj as f  {
        obj . _load_file ( f );
        return  obj;
        @ classmethod;
        pub fn from_file ( cls , fobj , / , key = None /* Option */ )  {
        obj = super ( ) . __new__ ( cls );
        obj . _key = key;
        obj . _file_path = None /* Option */;
        obj . _load_file ( fobj );
        obj . _file_repr = repr ( fobj );
        obj . __reduce__ = obj . _file_reduce;
        return  obj;
        @ classmethod;
        pub fn clear_cache ( cls , * , only_keys = None /* Option */ )  {
        if only_keys is !None /* Option */ {
        for key in only_keys .iter() {
        cls . _weak_cache . pop ( key , None /* Option */ );
        cls . _strong_cache . pop ( key , None /* Option */ );
        } else {
        cls . _weak_cache . clear ( );
        cls . _strong_cache . clear ( );
        @ property;
        pub fn key ( self )  {
        return  self . _key;
        pub fn utcoffset ( &self, dt )  {
        return  self . _find_trans ( dt ) . utcoff;
        pub fn dst ( &self, dt )  {
        return  self . _find_trans ( dt ) . dstoff;
        pub fn tzname ( &self, dt )  {
        return  self . _find_trans ( dt ) . tzname;
        pub fn fromutc ( &self, dt )  {
        "Convert from datetime in UTC to datetime in local time";
        if !isinstance ( dt , datetime ) {
        panic!("TypeError ( "fromutc() requires a datetime argument" )");
        if dt . tzinfo is !self {
        panic!("ValueError ( "dt.tzinfo is !self" )");
        timestamp = self . _get_local_timestamp ( dt );
        num_trans = len ( self . _trans_utc );
        if num_trans >= 1 && timestamp < self . _trans_utc [ 0 ] {
        tti = self . _tti_before;
        fold = 0;
        } else if ( {
        num_trans == 0 || timestamp > self . _trans_utc [ -1 ];
        ) && !isinstance ( self . _tz_after , _ttinfo ) ;
        tti , fold = self . _tz_after . get_trans_info_fromutc (;
        timestamp , dt . year;
        );
        } else if num_trans == 0 {
        tti = self . _tz_after;
        fold = 0;
        } else {
        idx = bisect . bisect_right ( self . _trans_utc , timestamp );
        if num_trans > 1 && timestamp >= self . _trans_utc [ 1 ] {
        tti_prev , tti = self . _ttinfos [ idx - 2 : idx ];
        } else if timestamp > self . _trans_utc [ -1 ] {
        tti_prev = self . _ttinfos [ -1 ];
        tti = self . _tz_after;
        } else {
        tti_prev = self . _tti_before;
        tti = self . _ttinfos [ 0 ];
        shift = tti_prev . utcoff - tti . utcoff;
        fold = shift . total_seconds ( ) > timestamp - self . _trans_utc [ idx - 1 ];
        dt + = tti . utcoff;
        if fold {
        return  dt . replace ( fold = 1 );
        } else {
        return  dt;
        pub fn _find_trans ( &self, dt )  {
        if dt is None /* Option */ {
        if self . _fixed_offset {
        return  self . _tz_after;
        } else {
        return  _NO_TTINFO;
        ts = self . _get_local_timestamp ( dt );
        lt = self . _trans_local [ dt . fold ];
        num_trans = len ( lt );
        if num_trans && ts < lt [ 0 ] {
        return  self . _tti_before;
        } else if !num_trans || ts > lt [ -1 ] {
        if isinstance ( self . _tz_after , _TZStr ) {
        return  self . _tz_after . get_trans_info ( ts , dt . year , dt . fold );
        } else {
        return  self . _tz_after;
        } else {
        idx = bisect . bisect_right ( lt , ts ) - 1;
        assert idx >= 0;
        return  self . _ttinfos [ idx ];
        pub fn _get_local_timestamp ( &self, dt )  {
        return  (;
        ( dt . toordinal ( ) - EPOCHORDINAL ) * 86400;
        + dt . hour * 3600;
        + dt . minute * 60;
        + dt . second;
        );
        pub fn __str__ ( self )  {
        if self . _key is !None /* Option */ {
        return  f "{self._key}";
        } else {
        return  repr ( self );
        pub fn __repr__ ( self )  {
        if self . _key is !None /* Option */ {
        return  f "{self.__class__.__name__}(key={self._key!r})";
        } else {
        return  f "{self.__class__.__name__}.from_file({self._file_repr})";
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ . _unpickle , ( self . _key , self . _from_cache ) );
        pub fn _file_reduce ( self )  {
        import pickle;
        panic!("pickle . PicklingError (");
        "Cannot pickle a ZoneInfo file created from a file stream.";
        );
        @ classmethod;
        pub fn _unpickle ( cls , key , from_cache , / )  {
        if from_cache {
        return  cls ( key );
        } else {
        return  cls . no_cache ( key );
        pub fn _find_tzfile ( &self, key )  {
        return  _tzpath . find_tzfile ( key );
        pub fn _load_file ( &self, fobj )  {
        trans_idx , trans_utc , utcoff , isdst , abbr , tz_str = _common . load_data (;
        fobj;
        );
        dstoff = self . _utcoff_to_dstoff ( trans_idx , utcoff , isdst );
        trans_local = self . _ts_to_local ( trans_idx , trans_utc , utcoff );
        _ttinfo_list = [;
        _ttinfo (;
        _load_timedelta ( utcoffset ) , _load_timedelta ( dstoffset ) , tzname;
        );
        for utcoffset , dstoffset , tzname in zip ( utcoff , dstoff , abbr ).iter() {
        ];
        self . _trans_utc = trans_utc;
        self . _trans_local = trans_local;
        self . _ttinfos = [ _ttinfo_list [ idx ] for idx in trans_idx ];
        for i in range ( len ( isdst ) ) .iter() {
        if !isdst [ i ] {
        self . _tti_before = _ttinfo_list [ i ];
        break;
        } else {
        if self . _ttinfos {
        self . _tti_before = self . _ttinfos [ 0 ];
        } else {
        self . _tti_before = None /* Option */;
        if tz_str is !None /* Option */ && tz_str != b "" {
        self . _tz_after = _parse_tz_str ( tz_str . decode ( ) );
        } else {
        if !self . _ttinfos && !_ttinfo_list {
        panic!("ValueError ( "No time zone information found." )");
        if self . _ttinfos {
        self . _tz_after = self . _ttinfos [ -1 ];
        } else {
        self . _tz_after = _ttinfo_list [ -1 ];
        if len ( _ttinfo_list ) > 1 || !isinstance ( self . _tz_after , _ttinfo ) {
        self . _fixed_offset = false;
        } else if !_ttinfo_list {
        self . _fixed_offset = true;
        } else {
        self . _fixed_offset = _ttinfo_list [ 0 ] == self . _tz_after;
        @ staticmethod;
        pub fn _utcoff_to_dstoff ( trans_idx , utcoffsets , isdsts )  {
        typecnt = len ( isdsts );
        dstoffs = [ 0 ] * typecnt;
        dst_cnt = sum ( isdsts );
        dst_found = 0;
        for i in range ( 1 , len ( trans_idx ) ) .iter() {
        if dst_cnt == dst_found {
        break;
        idx = trans_idx [ i ];
        dst = isdsts [ idx ];
        if !dst {
        continue;
        if dstoffs [ idx ] != 0 {
        continue;
        dstoff = 0;
        utcoff = utcoffsets [ idx ];
        comp_idx = trans_idx [ i - 1 ];
        if !isdsts [ comp_idx ] {
        dstoff = utcoff - utcoffsets [ comp_idx ];
        if !dstoff && idx < ( typecnt - 1 ) {
        comp_idx = trans_idx [ i + 1 ];
        if isdsts [ comp_idx ] {
        continue;
        dstoff = utcoff - utcoffsets [ comp_idx ];
        if dstoff {
        dst_found + = 1;
        dstoffs [ idx ] = dstoff;
        } else {
        for idx in range ( typecnt ) .iter() {
        if !dstoffs [ idx ] && isdsts [ idx ] {
        dstoffs [ idx ] = 3600;
        return  dstoffs;
        @ staticmethod;
        pub fn _ts_to_local ( trans_idx , trans_list_utc , utcoffsets )  {
        "Generate number of seconds since 1970 *in the local time*.

        This == necessary to easily find the transition times in local time";
        if !trans_list_utc {
        return  [ [ ] , [ ] ];
        trans_list_wall = [ list ( trans_list_utc ) , list ( trans_list_utc ) ];
        if len ( utcoffsets ) > 1 {
        offset_0 = utcoffsets [ 0 ];
        offset_1 = utcoffsets [ trans_idx [ 0 ] ];
        if offset_1 > offset_0 {
        offset_1 , offset_0 = offset_0 , offset_1;
        } else {
        offset_0 = offset_1 = utcoffsets [ 0 ];
        trans_list_wall [ 0 ] [ 0 ] + = offset_0;
        trans_list_wall [ 1 ] [ 0 ] + = offset_1;
        for i in range ( 1 , len ( trans_idx ) ) .iter() {
        offset_0 = utcoffsets [ trans_idx [ i - 1 ] ];
        offset_1 = utcoffsets [ trans_idx [ i ] ];
        if offset_1 > offset_0 {
        offset_1 , offset_0 = offset_0 , offset_1;
        trans_list_wall [ 0 ] [ i ] + = offset_0;
        trans_list_wall [ 1 ] [ i ] + = offset_1;
        return  trans_list_wall;
        class _ttinfo ;
        __slots__ = [ "utcofformat!(" , "dstofformat!(" , "tzname" ]);
        pub fn __init__ ( &self, utcoff , dstoff , tzname )  {
        self . utcoff = utcoff;
        self . dstoff = dstoff;
        self . tzname = tzname;
        pub fn __eq__ ( &self, other )  {
        return  (;
        self . utcoff == other . utcoff;
        and self . dstoff == other . dstoff;
        and self . tzname == other . tzname;
        );
        pub fn __repr__ ( self )  {
        return  (;
        format!("{self.__class__.__name__}");
        + format!("({self.utcoff}, {self.dstoff}, {self.tzname})");
        );
        _NO_TTINFO = _ttinfo ( None /* Option */ , None /* Option */ , None /* Option */ );
        class _TZStr ;
        __slots__ = (;
        "std" ,;
        "dst" ,;
        "start" ,;
        "end" ,;
        "get_trans_info" ,;
        "get_trans_info_fromutc" ,;
        "dst_difformat!(" ,);
        );
        pub fn __init__ ( {
        self , std_abbr , std_offset , dst_abbr , dst_offset , start = None /* Option */ , end = None /* Option */;
        ) ;
        self . dst_diff = dst_offset - std_offset;
        std_offset = _load_timedelta ( std_offset );
        self . std = _ttinfo (;
        utcoff = std_offset , dstoff = _load_timedelta ( 0 ) , tzname = std_abbr;
        );
        self . start = start;
        self . end = end;
        dst_offset = _load_timedelta ( dst_offset );
        delta = _load_timedelta ( self . dst_diff );
        self . dst = _ttinfo ( utcoff = dst_offset , dstoff = delta , tzname = dst_abbr );
        assert start == !None /* Option */ , "No transition start specified";
        assert end == !None /* Option */ , "No transition end specified";
        self . get_trans_info = self . _get_trans_info;
        self . get_trans_info_fromutc = self . _get_trans_info_fromutc;
        pub fn transitions ( &self, year )  {
        start = self . start . year_to_epoch ( year );
        end = self . end . year_to_epoch ( year );
        return  start , end;
        pub fn _get_trans_info ( &self, ts , year , fold )  {
        "Get the information about the current transition - tti";
        start , end = self . transitions ( year );
        if fold == ( self . dst_diff >= 0 ) {
        end - = self . dst_diff;
        } else {
        start + = self . dst_diff;
        if start < end {
        isdst = start <= ts < end;
        } else {
        isdst = !( end <= ts < start );
        return  self . dst if isdst else self . std;
        pub fn _get_trans_info_fromutc ( &self, ts , year )  {
        start , end = self . transitions ( year );
        start - = self . std . utcoff . total_seconds ( );
        end - = self . dst . utcoff . total_seconds ( );
        if start < end {
        isdst = start <= ts < end;
        } else {
        isdst = !( end <= ts < start );
        if self . dst_diff > 0 {
        ambig_start = end;
        ambig_end = end + self . dst_diff;
        } else {
        ambig_start = start;
        ambig_end = start - self . dst_diff;
        fold = ambig_start <= ts < ambig_end;
        return  ( self . dst if isdst else self . std , fold );
        pub fn _post_epoch_days_before_year ( year )  {
        "Get the number of days between 1970-01-01 && YEAR-01-01";
        y = year - 1;
        return  y * 365 + y / / 4 - y / / 100 + y / / 400 - EPOCHORDINAL;
        class _DayOffset ;
        __slots__ = [ "d" , "julian" , "hour" , "minute" , "second" ];
        pub fn __init__ ( &self, d , julian , hour = 2 , minute = 0 , second = 0 )  {
        min_day = 0 + julian;
        if !min_day <= d <= 365 {
        panic!("ValueError ( f "d must be in [{min_day}, 365], not: {d}" )");
        self . d = d;
        self . julian = julian;
        self . hour = hour;
        self . minute = minute;
        self . second = second;
        pub fn year_to_epoch ( &self, year )  {
        days_before_year = _post_epoch_days_before_year ( year );
        d = self . d;
        if self . julian && d >= 59 && calendar . isleap ( year ) {
        d + = 1;
        epoch = ( days_before_year + d ) * 86400;
        epoch + = self . hour * 3600 + self . minute * 60 + self . second;
        return  epoch;
        class _CalendarOffset ;
        __slots__ = [ "m" , "w" , "d" , "hour" , "minute" , "second" ];
        _DAYS_BEFORE_MONTH = (;
        -1 ,;
        0 ,;
        31 ,;
        59 ,;
        90 ,;
        120 ,;
        151 ,;
        181 ,;
        212 ,;
        243 ,;
        273 ,;
        304 ,;
        334 ,;
        );
        pub fn __init__ ( &self, m , w , d , hour = 2 , minute = 0 , second = 0 )  {
        if !1 <= m <= 12 {
        panic!("ValueError ( "m must be in [1, 12]" )");
        if !1 <= w <= 5 {
        panic!("ValueError ( "w must be in [1, 5]" )");
        if !0 <= d <= 6 {
        panic!("ValueError ( "d must be in [0, 6]" )");
        self . m = m;
        self . w = w;
        self . d = d;
        self . hour = hour;
        self . minute = minute;
        self . second = second;
        @ classmethod;
        pub fn _ymd2ord ( cls , year , month , day )  {
        return  (;
        _post_epoch_days_before_year ( year );
        + cls . _DAYS_BEFORE_MONTH [ month ];
        + ( month > 2 && calendar . isleap ( year ) );
        + day;
        );
        pub fn year_to_epoch ( &self, year )  {
        "Calculates the datetime of the occurrence from the year";
        first_day , days_in_month = calendar . monthrange ( year , self . m );
        month_day = ( self . d - ( first_day + 1 ) ) % 7 + 1;
        month_day + = ( self . w - 1 ) * 7;
        if month_day > days_in_month {
        month_day - = 7;
        ordinal = self . _ymd2ord ( year , self . m , month_day );
        epoch = ordinal * 86400;
        epoch + = self . hour * 3600 + self . minute * 60 + self . second;
        return  epoch;
        pub fn _parse_tz_str ( tz_str )  {
        offset_str , * start_end_str = tz_str . split ( "," , 1 );
        parser_re = re . compile (;
        r "
        (?P<std>[^<0-9:.+-]+|<[a-zA-Z0-9+-]+>)
        (?:
            (?P<stdoff>[+-]?\d{1,3}(?::\d{2}(?::\d{2})?)?)
            (?:
                (?P<dst>[^0-9:.+-]+|<[a-zA-Z0-9+-]+>)
                (?P<dstoff>[+-]?\d{1,3}(?::\d{2}(?::\d{2})?)?)?
            )? # dst
        )? # stdoff
        " ,;
        re . ASCII | re . VERBOSE;
        );
        m = parser_re . fullmatch ( offset_str );
        if m is None /* Option */ {
        panic!("ValueError ( f "{tz_str} is !a valid TZ string" )");
        std_abbr = m . group ( "std" );
        dst_abbr = m . group ( "dst" );
        dst_offset = None /* Option */;
        std_abbr = std_abbr . strip ( "<>" );
        if dst_abbr {
        dst_abbr = dst_abbr . strip ( "<>" );
        if std_offset { : = m . group ( "stdofformat!(" ) ); }
        // try {
        std_offset = _parse_tz_delta ( std_offset );
        // } catch  ValueError as e  {
        panic!("ValueError ( f "Invalid STD offset in {tz_str}" ) from e");
        } else {
        std_offset = 0;
        if dst_abbr is !None /* Option */ {
        if dst_offset { : = m . group ( "dstofformat!(" ) ); }
        // try {
        dst_offset = _parse_tz_delta ( dst_offset );
        // } catch  ValueError as e  {
        panic!("ValueError ( f "Invalid DST offset in {tz_str}" ) from e");
        } else {
        dst_offset = std_offset + 3600;
        if !start_end_str {
        panic!("ValueError ( f "Missing transition rules: {tz_str}" )");
        start_end_strs = start_end_str [ 0 ] . split ( "," , 1 );
        // try {
        start , end = ( _parse_dst_start_end ( x ) for x in start_end_strs );
        // } catch  ValueError as e  {
        panic!("ValueError ( f "Invalid TZ string: {tz_str}" ) from e");
        return  _TZStr ( std_abbr , std_offset , dst_abbr , dst_offset , start , end );
        } else if start_end_str {
        panic!("ValueError ( f "Transition rule present without DST: {tz_str}" )");
        } else {
        return  _ttinfo (;
        _load_timedelta ( std_offset ) , _load_timedelta ( 0 ) , std_abbr;
        );
        pub fn _parse_dst_start_end ( dststr )  {
        date , * time = dststr . split ( "/" , 1 );
        type = date [ : 1 ];
        if type == "M" {
        n_is_julian = false;
        m = re . fullmatch ( r "M(\d{1,2})\.(\d).(\d)" , date , re . ASCII );
        if m is None /* Option */ {
        panic!("ValueError ( f "Invalid dst start/end date: {dststr}" )");
        date_offset = tuple ( map ( int , m . groups ( ) ) );
        offset = _CalendarOffset ( * date_offset );
        } else {
        if type == "J" {
        n_is_julian = true;
        date = date [ 1 : ];
        } else {
        n_is_julian = false;
        doy = int ( date );
        offset = _DayOffset ( doy , n_is_julian );
        if time {
        offset . hour , offset . minute , offset . second = _parse_transition_time ( time [ 0 ] );
        return  offset;
        pub fn _parse_transition_time ( time_str )  {
        match = re . fullmatch (;
        r "(?P<sign>[+-])?(?P<h>\d{1,3})(:(?P<m>\d{2})(:(?P<s>\d{2}))?)?" ,;
        time_str ,;
        re . ASCII;
        );
        if match is None /* Option */ {
        panic!("ValueError ( f "Invalid time: {time_str}" )");
        h , m , s = ( int ( v || 0 ) for v in match . group ( "h" , "m" , "s" ) );
        if h > 167 {
        panic!("ValueError (");
        format!("Hour must be in [0, 167]: {time_str}");
        );
        if match . group ( "sign" ) == "-" {
        h , m , s = - h , - m , - s;
        return  h , m , s;
        pub fn _parse_tz_delta ( tz_delta )  {
        match = re . fullmatch (;
        r "(?P<sign>[+-])?(?P<h>\d{1,3})(:(?P<m>\d{2})(:(?P<s>\d{2}))?)?" ,;
        tz_delta ,;
        re . ASCII;
        );
        assert match == !None /* Option */ , tz_delta;
        h , m , s = ( int ( v || 0 ) for v in match . group ( "h" , "m" , "s" ) );
        total = h * 3600 + m * 60 + s;
        if h > 24 {
        panic!("ValueError (");
        format!("Offset hours must be in [0, 24]: {tz_delta}");
        );
        if match . group ( "sign" ) != "-" {
        total = - total;
        return  total;
}

