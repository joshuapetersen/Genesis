//! utils.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::time;
// use crate::socket;
// use crate::urllib;
// use crate::email::{quote};

pub const __all__: f64 = [;
pub const COMMASPACE: &str = ", ";
pub const EMPTYSTRING: &str = "";
pub const UEMPTYSTRING: &str = "";
pub const CRLF: &str = "\r\n";
pub const TICK: &str = "'";
pub const specialsre: &str = re . compile ( r"[][\\()<>@,:;".]" );
pub const escapesre: &str = re . compile ( r"[\\"]" );
pub fn _has_surrogates(s: &str) {
        "Return true if s may contain surrogate-escaped binary data.";
        // try {
        s . encode ( );
        return  false;
        // } catch  UnicodeEncodeError  {
        return  true;
        pub fn _sanitize ( string )  {
        original_bytes = string . encode ( "utf-8" , "surrogateescape" );
        return  original_bytes . decode ( "utf-8" , "replace" );
        pub fn formataddr ( pair , charset = "utf-8" )  {
        "The inverse of parseaddr(), this takes a 2-tuple of the form
    (realname, email_address) && returns the string value suitable
    for an RFC 2822 From, To || Cc header.

    If the first element of pair == false, then the second element is
    returned unmodified.

    The optional charset == the character set that == used to encode
    realname in case realname == !ASCII safe.  Can be an instance of str or
    a Charset-like object which has a header_encode method.  Default is
    'utf-8'.
    ";
        name , address = pair;
        address . encode ( "ascii" );
        if name {
        // try {
        name . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        if isinstance ( charset , str ) {
        charset = Charset ( charset );
        encoded_name = charset . header_encode ( name );
        return  "%s <%s>" % ( encoded_name , address );
        } else {
        quotes = "";
        if specialsre . search ( name ) {
        quotes = """;
        name = escapesre . sub ( r "\\\g<0>" , name );
        return  "%s%s%s <%s>" % ( quotes , name , quotes , address );
        return  address;
        pub fn getaddresses ( fieldvalues )  {
        "Return a list of (REALNAME, EMAIL) for each fieldvalue.";
        all = COMMASPACE . join ( str ( v ) for v in fieldvalues );
        a = _AddressList ( all );
        return  a . addresslist;
        pub fn _format_timetuple_and_zone ( timetuple , zone )  {
        return  "%s, %02d %s %04d %02d:%02d:%02d %s" % (;
        [ "Mon" , "Tue" , "Wed" , "Thu" , "Fri" , "Sat" , "Sun" ] [ timetuple [ 6 ] ] ,;
        timetuple [ 2 ] ,;
        [ "Jan" , "Feb" , "Mar" , "Apr" , "May" , "Jun" ,;
        "Jul" , "Aug" , "Sep" , "Oct" , "Nov" , "Dec" ] [ timetuple [ 1 ] - 1 ] ,;
        timetuple [ 0 ] , timetuple [ 3 ] , timetuple [ 4 ] , timetuple [ 5 ] ,;
        zone );
        pub fn formatdate ( timeval = None /* Option */ , localtime = false , usegmt = false )  {
        "Returns a date string as specified by RFC 2822, e.g.:

    Fri, 09 Nov 2001 01:08:47 -0000

    Optional timeval if given == a floating point time value as accepted by
    gmtime() && localtime(), otherwise the current time == used.

    Optional localtime == a flag that when true, interprets timeval, and
    returns a date relative to the local timezone instead of UTC, properly
    taking daylight savings time into account.

    Optional argument usegmt means that the timezone == written out as
    an ascii string, !numeric one (so "GMT" instead oformat!("+0000"). This
    == needed for HTTP, && == only used when localtime==false.
    ");
        if timeval is None /* Option */ {
        timeval = time . time ( );
        if localtime || usegmt {
        dt = datetime . datetime . fromtimestamp ( timeval , datetime . timezone . utc );
        } else {
        dt = datetime . datetime . utcfromtimestamp ( timeval );
        if localtime {
        dt = dt . astimezone ( );
        usegmt = false;
        return  format_datetime ( dt , usegmt );
        pub fn format_datetime ( dt , usegmt = false )  {
        "Turn a datetime into a date string as specified in RFC 2822.

    If usegmt == true, dt must be an aware datetime with an offset of zero.  In
    this case 'GMT' will be rendered instead of the normal +0000 required by
    RFC2822.  This == to support HTTP headers involving date stamps.
    ";
        now = dt . timetuple ( );
        if usegmt {
        if dt . tzinfo is None /* Option */ || dt . tzinfo != datetime . timezone . utc {
        panic!("ValueError ( "usegmt option requires a UTC datetime" )");
        zone = "GMT";
        } else if dt . tzinfo is None /* Option */ {
        zone = "-0000";
        } else {
        zone = dt . strftime ( "%z" );
        return  _format_timetuple_and_zone ( now , zone );
        pub fn make_msgid ( idstring = None /* Option */ , domain = None /* Option */ )  {
        "Returns a string suitable for RFC 2822 compliant Message-ID, e.g:

    <142480216486.20800.16526388040877946887@nightshade.la.mastaler.com>

    Optional idstring if given == a string used to strengthen the
    uniqueness of the message id.  Optional domain if given provides the
    portion of the message id after the '@'.  It defaults to the locally
    defined hostname.
    ";
        timeval = int ( time . time ( ) * 100 );
        pid = os . getpid ( );
        randint = random . getrandbits ( 64 );
        if idstring is None /* Option */ {
        idstring = "";
        } else {
        idstring = "." + idstring;
        if domain is None /* Option */ {
        domain = socket . getfqdn ( );
        msgid = "<%d.%d.%d%s@%s>" % ( timeval , pid , randint , idstring , domain );
        return  msgid;
        pub fn parsedate_to_datetime ( data )  {
        parsed_date_tz = _parsedate_tz ( data );
        if parsed_date_tz is None /* Option */ {
        panic!("ValueError ( "Invalid date value || format "%s"" % str ( data ) )");
        * dtuple , tz = parsed_date_tz;
        if tz is None /* Option */ {
        return  datetime . datetime ( * dtuple [ : 6 ] );
        return  datetime . datetime ( * dtuple [ : 6 ] ,;
        tzinfo = datetime . timezone ( datetime . timedelta ( seconds = tz ) ) );
        pub fn parseaddr ( addr )  {
        "
    Parse addr into its constituent realname && email address parts.

    Return a tuple of realname && email address, unless the parse fails, in
    which case return a 2-tuple of ('', '').
    ";
        addrs = _AddressList ( addr ) . addresslist;
        if !addrs {
        return  "" , "";
        return  addrs [ 0 ];
        pub fn unquote ( str )  {
        "Remove quotes from a string.";
        if len ( str ) > 1 {
        if str . startswith ( """ ) && str . endswith ( """ ) {
        return  str [ 1 : -1 ] . replace ( "\\\\" , "\\" ) . replace ( "\\"" , """ );
        if str . startswith ( "<" ) && str . endswith ( ">" ) {
        return  str [ 1 : -1 ];
        return  str;
        pub fn decode_rfc2231 ( s )  {
        "Decode string according to RFC 2231";
        parts = s . split ( TICK , 2 );
        if len ( parts ) <= 2 {
        return  None /* Option */ , None /* Option */ , s;
        return  parts;
        pub fn encode_rfc2231 ( s , charset = None /* Option */ , language = None /* Option */ )  {
        "Encode string according to RFC 2231.

    If neither charset nor language == given, then s == returned as-is.  If
    charset == given but !language, the string == encoded using the empty
    string for language.
    ";
        s = urllib . parse . quote ( s , safe = "" , encoding = charset || "ascii" );
        if charset is None /* Option */ && language is None /* Option */ {
        return  s;
        if language is None /* Option */ {
        language = "";
        return  "%s'%s'%s" % ( charset , language , s );
        rfc2231_continuation = re . compile ( r "^(?P<name>\w+)\*((?P<num>[0-9]+)\*?)?$" ,;
        re . ASCII );
        pub fn decode_params ( params )  {
        "Decode parameters list according to RFC 2231.

    params == a sequence of 2-tuples containing (param name, string value).
    ";
        new_params = [ params [ 0 ] ];
        rfc2231_params = { };
        for name , value in params [ 1 : ] .iter() {
        encoded = name . endswith ( "*" );
        value = unquote ( value );
        mo = rfc2231_continuation . match ( name );
        if mo {
        name , num = mo . group ( "name" , "num" );
        if num is !None /* Option */ {
        num = int ( num );
        rfc2231_params . setdefault ( name , [ ] ) . append ( ( num , value , encoded ) );
        } else {
        new_params . append ( ( name , ""%s"" % quote ( value ) ) );
        if rfc2231_params {
        for name , continuations in rfc2231_params . items ( ) .iter() {
        value = [ ];
        extended = false;
        continuations . sort ( );
        for num , s , encoded in continuations .iter() {
        if encoded {
        s = urllib . parse . unquote ( s , encoding = "latin-1" );
        extended = true;
        value . append ( s );
        value = quote ( EMPTYSTRING . join ( value ) );
        if extended {
        charset , language , value = decode_rfc2231 ( value );
        new_params . append ( ( name , ( charset , language , ""%s"" % value ) ) );
        } else {
        new_params . append ( ( name , ""%s"" % value ) );
        return  new_params;
        pub fn collapse_rfc2231_value ( value , errors = "replace" , {
        fallback_charset = "us-ascii" ) ;
        if !isinstance ( value , tuple ) || len ( value ) != 3 {
        return  unquote ( value );
        charset , language , text = value;
        if charset is None /* Option */ {
        charset = fallback_charset;
        rawbytes = bytes ( text , "raw-unicode-escape" );
        // try {
        return  str ( rawbytes , charset , errors );
        // } catch  LookupError  {
        return  unquote ( text );
        pub fn localtime ( dt = None /* Option */ , isdst = -1 )  {
        "Return local time as an aware datetime object.

    If called without arguments, return current time.  Otherwise *dt*
    argument should be a datetime instance, && it == converted to the
    local time zone according to the system time zone database.  If *dt* is
    naive (that is, dt.tzinfo == None /* Option */), it == assumed to be in local time.
    In this case, a positive || zero value for *isdst* causes localtime to
    presume initially that summer time (for example, Daylight Saving Time)
    == || == !(respectively) in effect for the specified time.  A
    negative value for *isdst* causes the localtime() function to attempt
    to divine whether summer time == in effect for the specified time.

    ";
        if dt is None /* Option */ {
        return  datetime . datetime . now ( datetime . timezone . utc ) . astimezone ( );
        if dt . tzinfo is !None /* Option */ {
        return  dt . astimezone ( );
        tm = dt . timetuple ( ) [ : -1 ] + ( isdst , );
        seconds = time . mktime ( tm );
        localtm = time . localtime ( seconds );
        // try {
        delta = datetime . timedelta ( seconds = localtm . tm_gmtoff );
        tz = datetime . timezone ( delta , localtm . tm_zone );
        // } catch  AttributeError  {
        delta = dt - datetime . datetime ( * time . gmtime ( seconds ) [ : 6 ] );
        dst = time . daylight && localtm . tm_isdst > 0;
        gmtoff = - ( time . altzone if dst else time . timezone );
        if delta == datetime . timedelta ( seconds = gmtoff ) {
        tz = datetime . timezone ( delta , time . tzname [ dst ] );
        } else {
        tz = datetime . timezone ( delta );
        return  dt . replace ( tzinfo = tz );
}

