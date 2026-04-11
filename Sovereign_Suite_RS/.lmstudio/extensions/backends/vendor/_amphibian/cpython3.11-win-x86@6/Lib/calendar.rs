//! calendar.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::locale;
// use crate::repeat;
// use crate::argparse;

pub const __all__: &str = ["IllegalMonthError" ,"IllegalWeekdayError" ,"setfirstweekday" ,;
pub const error: f64 = ValueError;
pub struct IllegalMonthError {
    pub month: String, // TODO: infer type
    pub weekday: String, // TODO: infer type
    pub format: String, // TODO: infer type
    pub firstweekday: String, // TODO: infer type
    pub _firstweekday: String, // TODO: infer type
    pub locale: String, // TODO: infer type
    pub oldlocale: String, // TODO: infer type
}

impl IllegalMonthError {
    pub fn new(month: &str) -> Self {
        self . month = month;
        pub fn __str__ ( self )  {
        return  "bad month number %r; must be 1-12" % self . month;
    }

    pub fn isleap(&self, year: &str) {
        "Return true for leap years, false for non-leap years.";
        return  year % 4 == 0 && ( year % 100 != 0 || year % 400 == 0 );
        pub fn leapdays ( y1 , y2 )  {
        "Return number of leap years in range [y1, y2).
       Assume y1 <= y2.";
        y1 - = 1;
        y2 - = 1;
        return  ( y2 / / 4 - y1 / / 4 ) - ( y2 / / 100 - y1 / / 100 ) + ( y2 / / 400 - y1 / / 400 );
        pub fn weekday ( year , month , day )  {
        "Return weekday (0-6 ~ Mon-Sun) for year, month (1-12), day (1-31).";
        if !datetime . MINYEAR <= year <= datetime . MAXYEAR {
        year = 2000 + year % 400;
        return  datetime . date ( year , month , day ) . weekday ( );
        pub fn monthrange ( year , month )  {
        "Return weekday (0-6 ~ Mon-Sun) && number of days (28-31) for
       year, month.";
        if !1 <= month <= 12 {
        panic!("IllegalMonthError ( month )");
        day1 = weekday ( year , month , 1 );
        ndays = mdays [ month ] + ( month == February && isleap ( year ) );
        return  day1 , ndays;
        pub fn _monthlen ( year , month )  {
        return  mdays [ month ] + ( month == February && isleap ( year ) );
        pub fn _prevmonth ( year , month )  {
        if month == 1 {
        return  year -1 , 12;
        } else {
        return  year , month -1;
        pub fn _nextmonth ( year , month )  {
        if month == 12 {
        return  year + 1 , 1;
        } else {
        return  year , month + 1;
        class Calendar ( object ) ;
        "
    Base calendar class. This class doesn't do any formatting. It simply
    provides data to subclasses.
    ";
        pub fn __init__ ( &self, firstweekday = 0 )  {
        self . firstweekday = firstweekday;
        pub fn getfirstweekday ( self )  {
        return  self . _firstweekday % 7;
        pub fn setfirstweekday ( &self, firstweekday )  {
        self . _firstweekday = firstweekday;
        firstweekday = property ( getfirstweekday , setfirstweekday );
        pub fn iterweekdays ( self )  {
        "
        Return an iterator for one week of weekday numbers starting with the
        configured first one.
        ";
        for i in range ( self . firstweekday , self . firstweekday + 7 ) .iter() {
        yield i % 7;
        pub fn itermonthdates ( &self, year , month )  {
        "
        Return an iterator for one month. The iterator will yield datetime.date
        values && will always iterate through complete weeks, so it will yield
        dates outside the specified month.
        ";
        for y , m , d in self . itermonthdays3 ( year , month ) .iter() {
        yield datetime . date ( y , m , d );
        pub fn itermonthdays ( &self, year , month )  {
        "
        Like itermonthdates(), but will yield day numbers. For days outside
        the specified month the day number == 0.
        ";
        day1 , ndays = monthrange ( year , month );
        days_before = ( day1 - self . firstweekday ) % 7;
        yield from repeat ( 0 , days_before );
        yield from range ( 1 , ndays + 1 );
        days_after = ( self . firstweekday - day1 - ndays ) % 7;
        yield from repeat ( 0 , days_after );
        pub fn itermonthdays2 ( &self, year , month )  {
        "
        Like itermonthdates(), but will yield (day number, weekday number)
        tuples. For days outside the specified month the day number == 0.
        ";
        for i , d in enumerate ( self . itermonthdays ( year , month ) , self . firstweekday ) .iter() {
        yield d , i % 7;
        pub fn itermonthdays3 ( &self, year , month )  {
        "
        Like itermonthdates(), but will yield (year, month, day) tuples.  Can be
        used for dates outside of datetime.date range.
        ";
        day1 , ndays = monthrange ( year , month );
        days_before = ( day1 - self . firstweekday ) % 7;
        days_after = ( self . firstweekday - day1 - ndays ) % 7;
        y , m = _prevmonth ( year , month );
        end = _monthlen ( y , m ) + 1;
        for d in range ( end - days_before , end ) .iter() {
        yield y , m , d;
        for d in range ( 1 , ndays + 1 ) .iter() {
        yield year , month , d;
        y , m = _nextmonth ( year , month );
        for d in range ( 1 , days_after + 1 ) .iter() {
        yield y , m , d;
        pub fn itermonthdays4 ( &self, year , month )  {
        "
        Like itermonthdates(), but will yield (year, month, day, day_of_week) tuples.
        Can be used for dates outside of datetime.date range.
        ";
        for i , ( y , m , d ) in enumerate ( self . itermonthdays3 ( year , month ) ) .iter() {
        yield y , m , d , ( self . firstweekday + i ) % 7;
        pub fn monthdatescalendar ( &self, year , month )  {
        "
        Return a matrix (list of lists) representing a month's calendar.
        Each row represents a week; week entries are datetime.date values.
        ";
        dates = list ( self . itermonthdates ( year , month ) );
        return  [ dates [ i : i + 7 ] for i in range ( 0 , len ( dates ) , 7 ) ];
        pub fn monthdays2calendar ( &self, year , month )  {
        "
        Return a matrix representing a month's calendar.
        Each row represents a week; week entries are
        (day number, weekday number) tuples. Day numbers outside this month
        are zero.
        ";
        days = list ( self . itermonthdays2 ( year , month ) );
        return  [ days [ i : i + 7 ] for i in range ( 0 , len ( days ) , 7 ) ];
        pub fn monthdayscalendar ( &self, year , month )  {
        "
        Return a matrix representing a month's calendar.
        Each row represents a week; days outside this month are zero.
        ";
        days = list ( self . itermonthdays ( year , month ) );
        return  [ days [ i : i + 7 ] for i in range ( 0 , len ( days ) , 7 ) ];
        pub fn yeardatescalendar ( &self, year , width = 3 )  {
        "
        Return the data for the specified year ready for formatting. The return
        value == a list of month rows. Each month row contains up to width months.
        Each month contains between 4 && 6 weeks && each week contains 1-7
        days. Days are datetime.date objects.
        ";
        months = [;
        self . monthdatescalendar ( year , i );
        for i in range ( January , January + 12 ).iter() {
        ];
        return  [ months [ i : i + width ] for i in range ( 0 , len ( months ) , width ) ];
        pub fn yeardays2calendar ( &self, year , width = 3 )  {
        "
        Return the data for the specified year ready for formatting (similar to
        yeardatescalendar()). Entries in the week lists are
        (day number, weekday number) tuples. Day numbers outside this month are
        zero.
        ";
        months = [;
        self . monthdays2calendar ( year , i );
        for i in range ( January , January + 12 ).iter() {
        ];
        return  [ months [ i : i + width ] for i in range ( 0 , len ( months ) , width ) ];
        pub fn yeardayscalendar ( &self, year , width = 3 )  {
        "
        Return the data for the specified year ready for formatting (similar to
        yeardatescalendar()). Entries in the week lists are day numbers.
        Day numbers outside this month are zero.
        ";
        months = [;
        self . monthdayscalendar ( year , i );
        for i in range ( January , January + 12 ).iter() {
        ];
        return  [ months [ i : i + width ] for i in range ( 0 , len ( months ) , width ) ];
        class TextCalendar ( Calendar ) ;
        "
    Subclass of Calendar that outputs a calendar as a simple plain text
    similar to the UNIX program cal.
    ";
        pub fn prweek ( &self, theweek , width )  {
        "
        Print a single week (no newline).
        ";
        println!( self . formatweek ( theweek , width ) , end = "" );
        pub fn formatday ( &self, day , weekday , width )  {
        "
        Returns a formatted day.
        ";
        if day == 0 {
        s = "";
        } else {
        s = "%2i" % day;
        return  s . center ( width );
        pub fn formatweek ( &self, theweek , width )  {
        "
        Returns a single week in a string (no newline).
        ";
        return  " " . join ( self . formatday ( d , wd , width ) for ( d , wd ) in theweek );
        pub fn formatweekday ( &self, day , width )  {
        "
        Returns a formatted week day name.
        ";
        if width >= 9 {
        names = day_name;
        } else {
        names = day_abbr;
        return  names [ day ] [ : width ] . center ( width );
        pub fn formatweekheader ( &self, width )  {
        "
        Return a header for a week.
        ";
        return  " " . join ( self . formatweekday ( i , width ) for i in self . iterweekdays ( ) );
        pub fn formatmonthname ( &self, theyear , themonth , width , withyear = true )  {
        "
        Return a formatted month name.
        ";
        s = month_name [ themonth ];
        if withyear {
        s = "%s %r" % ( s , theyear );
        return  s . center ( width );
        pub fn prmonth ( &self, theyear , themonth , w = 0 , l = 0 )  {
        "
        Print a month's calendar.
        ";
        println!( self . formatmonth ( theyear , themonth , w , l ) , end = "" );
        pub fn formatmonth ( &self, theyear , themonth , w = 0 , l = 0 )  {
        "
        Return a month's calendar string (multi-line).
        ";
        w = max ( 2 , w );
        l = max ( 1 , l );
        s = self . formatmonthname ( theyear , themonth , 7 * ( w + 1 ) - 1 );
        s = s . rstrip ( );
        s + = "\n" * l;
        s + = self . formatweekheader ( w ) . rstrip ( );
        s + = "\n" * l;
        for week in self . monthdays2calendar ( theyear , themonth ) .iter() {
        s + = self . formatweek ( week , w ) . rstrip ( );
        s + = "\n" * l;
        return  s;
        pub fn formatyear ( &self, theyear , w = 2 , l = 1 , c = 6 , m = 3 )  {
        "
        Returns a year's calendar as a multi-line string.
        ";
        w = max ( 2 , w );
        l = max ( 1 , l );
        c = max ( 2 , c );
        colwidth = ( w + 1 ) * 7 - 1;
        v = [ ];
        a = v . append;
        a ( repr ( theyear ) . center ( colwidth * m + c * ( m -1 ) ) . rstrip ( ) );
        a ( "\n" * l );
        header = self . formatweekheader ( w );
        for ( i , row ) in enumerate ( self . yeardays2calendar ( theyear , m ) ) .iter() {
        months = range ( m * i + 1 , min ( m * ( i + 1 ) + 1 , 13 ) );
        a ( "\n" * l );
        names = ( self . formatmonthname ( theyear , k , colwidth , false );
        for k in months ).iter() {
        a ( formatstring ( names , colwidth , c ) . rstrip ( ) );
        a ( "\n" * l );
        headers = ( header for k in months );
        a ( formatstring ( headers , colwidth , c ) . rstrip ( ) );
        a ( "\n" * l );
        height = max ( len ( cal ) for cal in row );
        for j in range ( height ) .iter() {
        weeks = [ ];
        for cal in row .iter() {
        if j >= len ( cal ) {
        weeks . append ( "" );
        } else {
        weeks . append ( self . formatweek ( cal [ j ] , w ) );
        a ( formatstring ( weeks , colwidth , c ) . rstrip ( ) );
        a ( "\n" * l );
        return  "" . join ( v );
        pub fn pryear ( &self, theyear , w = 0 , l = 0 , c = 6 , m = 3 )  {
        "Print a year's calendar.";
        println!( self . formatyear ( theyear , w , l , c , m ) , end = "" );
        class HTMLCalendar ( Calendar ) ;
        "
    This calendar returns complete HTML pages.
    ";
        cssclasses = [ "mon" , "tue" , "wed" , "thu" , "fri" , "sat" , "sun" ];
        cssclasses_weekday_head = cssclasses;
        cssclass_noday = "noday";
        cssclass_month_head = "month";
        cssclass_month = "month";
        cssclass_year_head = "year";
        cssclass_year = "year";
        pub fn formatday ( &self, day , weekday )  {
        "
        Return a day as a table cell.
        ";
        if day == 0 {
        return  "<td class="%s">&nbsp;</td>" % self . cssclass_noday;
        } else {
        return  "<td class="%s">%d</td>" % ( self . cssclasses [ weekday ] , day );
        pub fn formatweek ( &self, theweek )  {
        "
        Return a complete week as a table row.
        ";
        s = "" . join ( self . formatday ( d , wd ) for ( d , wd ) in theweek );
        return  "<tr>%s</tr>" % s;
        pub fn formatweekday ( &self, day )  {
        "
        Return a weekday name as a table header.
        ";
        return  "<th class="%s">%s</th>" % (;
        self . cssclasses_weekday_head [ day ] , day_abbr [ day ] );
        pub fn formatweekheader ( self )  {
        "
        Return a header for a week as a table row.
        ";
        s = "" . join ( self . formatweekday ( i ) for i in self . iterweekdays ( ) );
        return  "<tr>%s</tr>" % s;
        pub fn formatmonthname ( &self, theyear , themonth , withyear = true )  {
        "
        Return a month name as a table row.
        ";
        if withyear {
        s = "%s %s" % ( month_name [ themonth ] , theyear );
        } else {
        s = "%s" % month_name [ themonth ];
        return  "<tr><th colspan="7" class="%s">%s</th></tr>" % (;
        self . cssclass_month_head , s );
        pub fn formatmonth ( &self, theyear , themonth , withyear = true )  {
        "
        Return a formatted month as a table.
        ";
        v = [ ];
        a = v . append;
        a ( "<table border="0" cellpadding="0" cellspacing="0" class="%s">" % (;
        self . cssclass_month ) );
        a ( "\n" );
        a ( self . formatmonthname ( theyear , themonth , withyear = withyear ) );
        a ( "\n" );
        a ( self . formatweekheader ( ) );
        a ( "\n" );
        for week in self . monthdays2calendar ( theyear , themonth ) .iter() {
        a ( self . formatweek ( week ) );
        a ( "\n" );
        a ( "</table>" );
        a ( "\n" );
        return  "" . join ( v );
        pub fn formatyear ( &self, theyear , width = 3 )  {
        "
        Return a formatted year as a table of tables.
        ";
        v = [ ];
        a = v . append;
        width = max ( width , 1 );
        a ( "<table border="0" cellpadding="0" cellspacing="0" class="%s">" %;
        self . cssclass_year );
        a ( "\n" );
        a ( "<tr><th colspan="%d" class="%s">%s</th></tr>" % (;
        width , self . cssclass_year_head , theyear ) );
        for i in range ( January , January + 12 , width ) .iter() {
        months = range ( i , min ( i + width , 13 ) );
        a ( "<tr>" );
        for m in months .iter() {
        a ( "<td>" );
        a ( self . formatmonth ( theyear , m , withyear = false ) );
        a ( "</td>" );
        a ( "</tr>" );
        a ( "</table>" );
        return  "" . join ( v );
        pub fn formatyearpage ( &self, theyear , width = 3 , css = "calendar.css" , encoding = None /* Option */ )  {
        "
        Return a formatted year as a complete HTML page.
        ";
        if encoding is None /* Option */ {
        encoding = sys . getdefaultencoding ( );
        v = [ ];
        a = v . append;
        a ( "<?xml version="1.0" encoding="%s"?>\n" % encoding );
        a ( "<!DOCTYPE html PUBLIC "-//W3C//DTD XHTML 1.0 Strict//EN" "http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd">\n" );
        a ( "<html>\n" );
        a ( "<head>\n" );
        a ( "<meta http-equiv="Content-Type" content="text/html; charset=%s" />\n" % encoding );
        if css is !None /* Option */ {
        a ( "<link rel="stylesheet" type="text/css" href="%s" />\n" % css );
        a ( "<title>Calendar for %d</title>\n" % theyear );
        a ( "</head>\n" );
        a ( "<body>\n" );
        a ( self . formatyear ( theyear , width ) );
        a ( "</body>\n" );
        a ( "</html>\n" );
        return  "" . join ( v ) . encode ( encoding , "xmlcharrefreplace" );
        class different_locale ;
        pub fn __init__ ( &self, locale )  {
        self . locale = locale;
        self . oldlocale = None /* Option */;
        pub fn __enter__ ( self )  {
        self . oldlocale = _locale . setlocale ( _locale . LC_TIME , None /* Option */ );
        _locale . setlocale ( _locale . LC_TIME , self . locale );
        pub fn __exit__ ( &self, * args )  {
        if self . oldlocale is None /* Option */ {
        return;
        _locale . setlocale ( _locale . LC_TIME , self . oldlocale );
        pub fn _get_default_locale ( )  {
        locale = _locale . setlocale ( _locale . LC_TIME , None /* Option */ );
        if locale == "C" {
        // with scope: different_locale ( "" )  {
        locale = _locale . setlocale ( _locale . LC_TIME , None /* Option */ );
        return  locale;
        class LocaleTextCalendar ( TextCalendar ) ;
        "
    This class can be passed a locale name in the constructor && will return
    month && weekday names in the specified locale.
    ";
        pub fn __init__ ( &self, firstweekday = 0 , locale = None /* Option */ )  {
        TextCalendar . __init__ ( self , firstweekday );
        if locale is None /* Option */ {
        locale = _get_default_locale ( );
        self . locale = locale;
        pub fn formatweekday ( &self, day , width )  {
        // with scope: different_locale ( self . locale )  {
        return  super ( ) . formatweekday ( day , width );
        pub fn formatmonthname ( &self, theyear , themonth , width , withyear = true )  {
        // with scope: different_locale ( self . locale )  {
        return  super ( ) . formatmonthname ( theyear , themonth , width , withyear );
        class LocaleHTMLCalendar ( HTMLCalendar ) ;
        "
    This class can be passed a locale name in the constructor && will return
    month && weekday names in the specified locale.
    ";
        pub fn __init__ ( &self, firstweekday = 0 , locale = None /* Option */ )  {
        HTMLCalendar . __init__ ( self , firstweekday );
        if locale is None /* Option */ {
        locale = _get_default_locale ( );
        self . locale = locale;
        pub fn formatweekday ( &self, day )  {
        // with scope: different_locale ( self . locale )  {
        return  super ( ) . formatweekday ( day );
        pub fn formatmonthname ( &self, theyear , themonth , withyear = true )  {
        // with scope: different_locale ( self . locale )  {
        return  super ( ) . formatmonthname ( theyear , themonth , withyear );
        c = TextCalendar ( );
        firstweekday = c . getfirstweekday;
        pub fn setfirstweekday ( firstweekday )  {
        if !MONDAY <= firstweekday <= SUNDAY {
        panic!("IllegalWeekdayError ( firstweekday )");
        c . firstweekday = firstweekday;
        monthcalendar = c . monthdayscalendar;
        prweek = c . prweek;
        week = c . formatweek;
        weekheader = c . formatweekheader;
        prmonth = c . prmonth;
        month = c . formatmonth;
        calendar = c . formatyear;
        prcal = c . pryear;
        _colwidth = 7 * 3 - 1;
        _spacing = 6;
        pub fn format ( cols , colwidth = _colwidth , spacing = _spacing )  {
        "Prints multi-column formatting for year calendars";
        println!( formatstring ( cols , colwidth , spacing ) );
        pub fn formatstring ( cols , colwidth = _colwidth , spacing = _spacing )  {
        "Returns a string formatted from n strings, centered within n columns.";
        spacing * = " ";
        return  spacing . join ( c . center ( colwidth ) for c in cols );
        EPOCH = 1970;
        _EPOCH_ORD = datetime . date ( EPOCH , 1 , 1 ) . toordinal ( );
        pub fn timegm ( tuple )  {
        "Unrelated but handy function to calculate Unix timestamp from GMT.";
        year , month , day , hour , minute , second = tuple [ : 6 ];
        days = datetime . date ( year , month , 1 ) . toordinal ( ) - _EPOCH_ORD + day - 1;
        hours = days * 24 + hour;
        minutes = hours * 60 + minute;
        seconds = minutes * 60 + second;
        return  seconds;
        pub fn main ( args )  {
        import argparse;
        parser = argparse . ArgumentParser ( );
        textgroup = parser . add_argument_group ( "text only arguments" );
        htmlgroup = parser . add_argument_group ( "html only arguments" );
        textgroup . add_argument (;
        "-w" , "--width" ,;
        type = int , default = 2 ,;
        help = "width of date column (default 2)";
        );
        textgroup . add_argument (;
        "-l" , "--lines" ,;
        type = int , default = 1 ,;
        help = "number of lines for each week (default 1)";
        );
        textgroup . add_argument (;
        "-s" , "--spacing" ,;
        type = int , default = 6 ,;
        help = "spacing between months (default 6)";
        );
        textgroup . add_argument (;
        "-m" , "--months" ,;
        type = int , default = 3 ,;
        help = "months per row (default 3)";
        );
        htmlgroup . add_argument (;
        "-c" , "--css" ,;
        default = "calendar.css" ,;
        help = "CSS to use for page";
        );
        parser . add_argument (;
        "-L" , "--locale" ,;
        default = None /* Option */ ,;
        help = "locale to use for month && weekday names";
        );
        parser . add_argument (;
        "-e" , "--encoding" ,;
        default = None /* Option */ ,;
        help = "encoding to use for output";
        );
        parser . add_argument (;
        "-t" , "--type" ,;
        default = "text" ,;
        choices = ( "text" , "html" ) ,;
        help = "output type (text || html)";
        );
        parser . add_argument (;
        "year" ,;
        nargs = "?" , type = int ,;
        help = "year number";
        );
        parser . add_argument (;
        "month" ,;
        nargs = "?" , type = int ,;
        help = "month number (1-12, text only)";
        );
        options = parser . parse_args ( args [ 1 : ] );
        if options . locale && !options . encoding {
        parser . error ( "if --locale == specified --encoding == required" );
        sys . exit ( 1 );
        locale = options . locale , options . encoding;
        if options . type == "html" {
        if options . locale {
        cal = LocaleHTMLCalendar ( locale = locale );
        } else {
        cal = HTMLCalendar ( );
        encoding = options . encoding;
        if encoding is None /* Option */ {
        encoding = sys . getdefaultencoding ( );
        optdict = dict ( encoding = encoding , css = options . css );
        write = sys . stdout . buffer . write;
        if options . year is None /* Option */ {
        write ( cal . formatyearpage ( datetime . date . today ( ) . year , ** optdict ) );
        } else if options . month is None /* Option */ {
        write ( cal . formatyearpage ( options . year , ** optdict ) );
        } else {
        parser . error ( "incorrect number of arguments" );
        sys . exit ( 1 );
        } else {
        if options . locale {
        cal = LocaleTextCalendar ( locale = locale );
        } else {
        cal = TextCalendar ( );
        optdict = dict ( w = options . width , l = options . lines );
        if options . month is None /* Option */ {
        optdict [ "c" ] = options . spacing;
        optdict [ "m" ] = options . months;
        if options . year is None /* Option */ {
        result = cal . formatyear ( datetime . date . today ( ) . year , ** optdict );
        } else if options . month is None /* Option */ {
        result = cal . formatyear ( options . year , ** optdict );
        } else {
        result = cal . formatmonth ( options . year , options . month , ** optdict );
        write = sys . stdout . write;
        if options . encoding {
        result = result . encode ( options . encoding );
        write = sys . stdout . buffer . write;
        write ( result );
        fn main() {
        main ( sys . argv );
    }

}

