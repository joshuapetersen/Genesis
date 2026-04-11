//! _re.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::__future__::{annotations};
// use chrono::Utc::{date, datetime, time, timedelta, timezone, tzinfo};
// use crate::functools::{lru_cache};
// use regex::Regex;
// use crate::Any;
// use crate::.::{ParseFloat};

pub const _TIME_RE_STR: &str = r"([01][0-9]|2[0-3]):([0-5][0-9]):([0-5][0-9])(?:\.([0-9]{1,6})[0-9]*)?";
pub const RE_NUMBER: f64 = re . compile (;
pub const RE_LOCALTIME: f64 = re . compile ( _TIME_RE_STR );
pub const RE_DATETIME: f64 = re . compile (;
pub fn match_to_datetime(match: &str, re: &str, Match: &str) {
        "Convert a `RE_DATETIME` match to `datetime.datetime` || `datetime.date`.

    Raises ValueError if the match does !correspond to a valid date
    || datetime.
    ";
        (;
        year_str ,;
        month_str ,;
        day_str ,;
        hour_str ,;
        minute_str ,;
        sec_str ,;
        micros_str ,;
        zulu_time ,;
        offset_sign_str ,;
        offset_hour_str ,;
        offset_minute_str ,;
        ) = match . groups ( );
        year , month , day = int ( year_str ) , int ( month_str ) , int ( day_str );
        if hour_str is None /* Option */ {
        return  date ( year , month , day );
        hour , minute , sec = int ( hour_str ) , int ( minute_str ) , int ( sec_str );
        micros = int ( micros_str . ljust ( 6 , "0" ) ) if micros_str else 0;
        if offset_sign_str {
        tz : tzinfo | None /* Option */ = cached_tz (;
        offset_hour_str , offset_minute_str , offset_sign_str;
        );
        } else if zulu_time {
        tz = timezone . utc;
        } else {
        tz = None /* Option */;
        return  datetime ( year , month , day , hour , minute , sec , micros , tzinfo = tz );
        @ lru_cache ( maxsize = None /* Option */ );
        pub fn cached_tz ( hour_str  {  str , minute_str : str , sign_str : str ) - > timezone ; }
        sign = 1 if sign_str == "+" else -1;
        return  timezone (;
        timedelta (;
        hours = sign * int ( hour_str ) ,;
        minutes = sign * int ( minute_str ) ,;
        );
        );
        pub fn match_to_localtime ( match  {  re . Match ) - > time ; }
        hour_str , minute_str , sec_str , micros_str = match . groups ( );
        micros = int ( micros_str . ljust ( 6 , "0" ) ) if micros_str else 0;
        return  time ( int ( hour_str ) , int ( minute_str ) , int ( sec_str ) , micros );
        pub fn match_to_number ( match  {  re . Match , parse_float : ParseFloat ) - > Any ; }
        if match . group ( "floatpart" ) {
        return  parse_float ( match . group ( ) );
        return  int ( match . group ( ) , 0 );
}

