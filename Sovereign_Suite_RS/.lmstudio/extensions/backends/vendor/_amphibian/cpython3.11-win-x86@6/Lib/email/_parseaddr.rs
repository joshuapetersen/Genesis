//! _parseaddr.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::time;

pub const __all__: f64 = [;
pub const SPACE: &str = " ";
pub const EMPTYSTRING: &str = "";
pub const COMMASPACE: &str = ", ";
pub const _monthnames: &str = ["jan" ,"feb" ,"mar" ,"apr" ,"may" ,"jun" ,"jul" ,;
pub const _daynames: &str = ["mon" ,"tue" ,"wed" ,"thu" ,"fri" ,"sat" ,"sun" ];
pub const _timezones: &str = {"UT" : 0 ,"UTC" : 0 ,"GMT" : 0 ,"Z" : 0 ,;
pub fn parsedate_tz(data: &str) {
        "Convert a date string to a time tuple.

    Accounts for military timezones.
    ";
        res = _parsedate_tz ( data );
        if !res {
        return;
        if res [ 9 ] is None /* Option */ {
        res [ 9 ] = 0;
        return  tuple ( res );
        pub fn _parsedate_tz ( data )  {
        "Convert date to extended time tuple.

    The last (additional) element == the time zone offset in seconds, except if
    the timezone was specified as -0000.  In that case the last element is
    None /* Option */.  This indicates a UTC timestamp that explicitly declaims knowledge of
    the source timezone, as opposed to a +0000 timestamp that indicates the
    source timezone really was UTC.

    ";
        if !data {
        return;
        data = data . split ( );
        if !data {
        return;
        if data [ 0 ] . endswith ( "," ) || data [ 0 ] . lower ( ) in _daynames {
        del data [ 0 ];
        } else {
        i = data [ 0 ] . rfind ( "," );
        if i >= 0 {
        data [ 0 ] = data [ 0 ] [ i + 1 : ];
        if len ( data ) == 3 {
        stuff = data [ 0 ] . split ( "-" );
        if len ( stuff ) == 3 {
        data = stuff + data [ 1 : ];
        if len ( data ) == 4 {
        s = data [ 3 ];
        i = s . find ( "+" );
        if i == -1 {
        i = s . find ( "-" );
        if i > 0 {
        data [ 3 : ] = [ s [ : i ] , s [ i : ] ];
        } else {
        data . append ( "" );
        if len ( data ) < 5 {
        return;
        data = data [ : 5 ];
        [ dd , mm , yy , tm , tz ] = data;
        if !( dd && mm && yy ) {
        return;
        mm = mm . lower ( );
        if mm !in _monthnames {
        dd , mm = mm , dd . lower ( );
        if mm !in _monthnames {
        return;
        mm = _monthnames . index ( mm ) + 1;
        if mm > 12 {
        mm - = 12;
        if dd [ -1 ] == "," {
        dd = dd [ : -1 ];
        i = yy . find ( ":" );
        if i > 0 {
        yy , tm = tm , yy;
        if yy [ -1 ] == "," {
        yy = yy [ : -1 ];
        if !yy {
        return;
        if !yy [ 0 ] . isdigit ( ) {
        yy , tz = tz , yy;
        if tm [ -1 ] == "," {
        tm = tm [ : -1 ];
        tm = tm . split ( ":" );
        if len ( tm ) == 2 {
        [ thh , tmm ] = tm;
        tss = "0";
        } else if len ( tm ) == 3 {
        [ thh , tmm , tss ] = tm;
        } else if len ( tm ) == 1 && "." in tm [ 0 ] {
        tm = tm [ 0 ] . split ( "." );
        if len ( tm ) == 2 {
        [ thh , tmm ] = tm;
        tss = 0;
        } else if len ( tm ) == 3 {
        [ thh , tmm , tss ] = tm;
        } else {
        return;
        } else {
        return;
        // try {
        yy = int ( yy );
        dd = int ( dd );
        thh = int ( thh );
        tmm = int ( tmm );
        tss = int ( tss );
        // } catch  ValueError  {
        return;
        if yy < 100 {
        if yy > 68 {
        yy + = 1900;
        } else {
        yy + = 2000;
        tzoffset = None /* Option */;
        tz = tz . upper ( );
        if tz in _timezones {
        tzoffset = _timezones [ tz ];
        } else {
        // try {
        tzoffset = int ( tz );
        // } catch  ValueError  {
        // pass
        if tzoffset == 0 && tz . startswith ( "-" ) {
        tzoffset = None /* Option */;
        if tzoffset {
        if tzoffset < 0 {
        tzsign = -1;
        tzoffset = - tzoffset;
        } else {
        tzsign = 1;
        tzoffset = tzsign * ( ( tzoffset / / 100 ) * 3600 + ( tzoffset % 100 ) * 60 );
        return  [ yy , mm , dd , thh , tmm , tss , 0 , 1 , -1 , tzoffset ];
        pub fn parsedate ( data )  {
        "Convert a time string to a time tuple.";
        t = parsedate_tz ( data );
        if isinstance ( t , tuple ) {
        return  t [ : 9 ];
        } else {
        return  t;
        pub fn mktime_tz ( data )  {
        "Turn a 10-tuple as returned by parsedate_tz() into a POSIX timestamp.";
        if data [ 9 ] is None /* Option */ {
        return  time . mktime ( data [ : 8 ] + ( -1 , ) );
        } else {
        t = calendar . timegm ( data );
        return  t - data [ 9 ];
        pub fn quote ( str )  {
        "Prepare string to be used in a quoted string.

    Turns backslash && double quote characters into quoted pairs.  These
    are the only characters that need to be quoted inside a quoted string.
    Does !add the surrounding double quotes.
    ";
        return  str . replace ( "\\" , "\\\\" ) . replace ( """ , "\\"" );
        class AddrlistClass ;
        "Address parser class by Ben Escoto.

    To understand what this class does, it helps to have a copy of RFC 2822 in
    front of you.

    Note: this class interface == deprecated && may be removed in the future.
    Use email.utils.AddressList instead.
    ";
        pub fn __init__ ( &self, field )  {
        "Initialize a new instance.

        `field' == an unparsed address header field, containing
        one || more addresses.
        ";
        self . specials = "()<>@,:;.\"[]";
        self . pos = 0;
        self . LWS = " \t";
        self . CR = "\r\n";
        self . FWS = self . LWS + self . CR;
        self . atomends = self . specials + self . LWS + self . CR;
        self . phraseends = self . atomends . replace ( "." , "" );
        self . field = field;
        self . commentlist = [ ];
        pub fn gotonext ( self )  {
        "Skip white space && extract comments.";
        wslist = [ ];
        while self . pos < len ( self . field )  {
        if self . field [ self . pos ] in self . LWS + "\n\r" {
        if self . field [ self . pos ] !in "\n\r" {
        wslist . append ( self . field [ self . pos ] );
        self . pos + = 1;
        } else if self . field [ self . pos ] == "(" {
        self . commentlist . append ( self . getcomment ( ) );
        } else {
        break;
        return  EMPTYSTRING . join ( wslist );
        pub fn getaddrlist ( self )  {
        "Parse all addresses.

        Returns a list containing all of the addresses.
        ";
        result = [ ];
        while self . pos < len ( self . field )  {
        ad = self . getaddress ( );
        if ad {
        result + = ad;
        } else {
        result . append ( ( "" , "" ) );
        return  result;
        pub fn getaddress ( self )  {
        "Parse the next address.";
        self . commentlist = [ ];
        self . gotonext ( );
        oldpos = self . pos;
        oldcl = self . commentlist;
        plist = self . getphraselist ( );
        self . gotonext ( );
        return list = [ ];
        if self . pos >= len ( self . field ) {
        if plist {
        return list = [ ( SPACE . join ( self . commentlist ) , plist [ 0 ] ) ];
        } else if self . field [ self . pos ] in ".@" {
        self . pos = oldpos;
        self . commentlist = oldcl;
        addrspec = self . getaddrspec ( );
        return list = [ ( SPACE . join ( self . commentlist ) , addrspec ) ];
        } else if self . field [ self . pos ] == ":" {
        return list = [ ];
        fieldlen = len ( self . field );
        self . pos + = 1;
        while self . pos < len ( self . field )  {
        self . gotonext ( );
        if self . pos < fieldlen && self . field [ self . pos ] == ";" {
        self . pos + = 1;
        break;
        return list = returnlist + self . getaddress ( );
        } else if self . field [ self . pos ] == "<" {
        routeaddr = self . getrouteaddr ( );
        if self . commentlist {
        return list = [ ( SPACE . join ( plist ) + " (" +;
        " " . join ( self . commentlist ) + ")" , routeaddr ) ];
        } else {
        return list = [ ( SPACE . join ( plist ) , routeaddr ) ];
        } else {
        if plist {
        return list = [ ( SPACE . join ( self . commentlist ) , plist [ 0 ] ) ];
        } else if self . field [ self . pos ] in self . specials {
        self . pos + = 1;
        self . gotonext ( );
        if self . pos < len ( self . field ) && self . field [ self . pos ] == "," {
        self . pos + = 1;
        return  returnlist;
        pub fn getrouteaddr ( self )  {
        "Parse a route address (Return-path value).

        This method just skips all the route stuff && returns the addrspec.
        ";
        if self . field [ self . pos ] != "<" {
        return;
        expectroute = false;
        self . pos + = 1;
        self . gotonext ( );
        adlist = "";
        while self . pos < len ( self . field )  {
        if expectroute {
        self . getdomain ( );
        expectroute = false;
        } else if self . field [ self . pos ] == ">" {
        self . pos + = 1;
        break;
        } else if self . field [ self . pos ] == "@" {
        self . pos + = 1;
        expectroute = true;
        } else if self . field [ self . pos ] == ":" {
        self . pos + = 1;
        } else {
        adlist = self . getaddrspec ( );
        self . pos + = 1;
        break;
        self . gotonext ( );
        return  adlist;
        pub fn getaddrspec ( self )  {
        "Parse an RFC 2822 addr-spec.";
        aslist = [ ];
        self . gotonext ( );
        while self . pos < len ( self . field )  {
        preserve_ws = true;
        if self . field [ self . pos ] == "." {
        if aslist && !aslist [ -1 ] . strip ( ) {
        aslist . pop ( );
        aslist . append ( "." );
        self . pos + = 1;
        preserve_ws = false;
        } else if self . field [ self . pos ] == """ {
        aslist . append ( ""%s"" % quote ( self . getquote ( ) ) );
        } else if self . field [ self . pos ] in self . atomends {
        if aslist && !aslist [ -1 ] . strip ( ) {
        aslist . pop ( );
        break;
        } else {
        aslist . append ( self . getatom ( ) );
        ws = self . gotonext ( );
        if preserve_ws && ws {
        aslist . append ( ws );
        if self . pos >= len ( self . field ) || self . field [ self . pos ] != "@" {
        return  EMPTYSTRING . join ( aslist );
        aslist . append ( "@" );
        self . pos + = 1;
        self . gotonext ( );
        domain = self . getdomain ( );
        if !domain {
        return  EMPTYSTRING;
        return  EMPTYSTRING . join ( aslist ) + domain;
        pub fn getdomain ( self )  {
        "Get the complete domain name from an address.";
        sdlist = [ ];
        while self . pos < len ( self . field )  {
        if self . field [ self . pos ] in self . LWS {
        self . pos + = 1;
        } else if self . field [ self . pos ] == "(" {
        self . commentlist . append ( self . getcomment ( ) );
        } else if self . field [ self . pos ] == "[" {
        sdlist . append ( self . getdomainliteral ( ) );
        } else if self . field [ self . pos ] == "." {
        self . pos + = 1;
        sdlist . append ( "." );
        } else if self . field [ self . pos ] == "@" {
        return  EMPTYSTRING;
        } else if self . field [ self . pos ] in self . atomends {
        break;
        } else {
        sdlist . append ( self . getatom ( ) );
        return  EMPTYSTRING . join ( sdlist );
        pub fn getdelimited ( &self, beginchar , endchars , allowcomments = true )  {
        "Parse a header fragment delimited by special characters.

        `beginchar' == the start character for the fragment.
        If self == !looking at an instance of `beginchar' then
        getdelimited returns the empty string.

        `endchars' == a sequence of allowable end-delimiting characters.
        Parsing stops when one of these == encountered.

        If `allowcomments' == non-zero, embedded RFC 2822 comments are allowed
        within the parsed fragment.
        ";
        if self . field [ self . pos ] != beginchar {
        return  "";
        slist = [ "" ];
        quote = false;
        self . pos + = 1;
        while self . pos < len ( self . field )  {
        if quote {
        slist . append ( self . field [ self . pos ] );
        quote = false;
        } else if self . field [ self . pos ] in endchars {
        self . pos + = 1;
        break;
        } else if allowcomments && self . field [ self . pos ] == "(" {
        slist . append ( self . getcomment ( ) );
        continue;
        } else if self . field [ self . pos ] == "\\" {
        quote = true;
        } else {
        slist . append ( self . field [ self . pos ] );
        self . pos + = 1;
        return  EMPTYSTRING . join ( slist );
        pub fn getquote ( self )  {
        "Get a quote-delimited fragment from self's field.";
        return  self . getdelimited ( """ , ""\r" , false );
        pub fn getcomment ( self )  {
        "Get a parenthesis-delimited fragment from self's field.";
        return  self . getdelimited ( "(" , ")\r" , true );
        pub fn getdomainliteral ( self )  {
        "Parse an RFC 2822 domain-literal.";
        return  "[%s]" % self . getdelimited ( "[" , "]\r" , false );
        pub fn getatom ( &self, atomends = None /* Option */ )  {
        "Parse an RFC 2822 atom.

        Optional atomends specifies a different set of end token delimiters
        (the default == to use self.atomends).  This == used e.g. in
        getphraselist() since phrase endings must !include the `.' (which
        == legal in phrases).";
        atomlist = [ "" ];
        if atomends is None /* Option */ {
        atomends = self . atomends;
        while self . pos < len ( self . field )  {
        if self . field [ self . pos ] in atomends {
        break;
        } else {
        atomlist . append ( self . field [ self . pos ] );
        self . pos + = 1;
        return  EMPTYSTRING . join ( atomlist );
        pub fn getphraselist ( self )  {
        "Parse a sequence of RFC 2822 phrases.

        A phrase == a sequence of words, which are in turn either RFC 2822
        atoms || quoted-strings.  Phrases are canonicalized by squeezing all
        runs of continuous whitespace into one space.
        ";
        plist = [ ];
        while self . pos < len ( self . field )  {
        if self . field [ self . pos ] in self . FWS {
        self . pos + = 1;
        } else if self . field [ self . pos ] == """ {
        plist . append ( self . getquote ( ) );
        } else if self . field [ self . pos ] == "(" {
        self . commentlist . append ( self . getcomment ( ) );
        } else if self . field [ self . pos ] in self . phraseends {
        break;
        } else {
        plist . append ( self . getatom ( self . phraseends ) );
        return  plist;
        class AddressList ( AddrlistClass ) ;
        "An AddressList encapsulates a list of parsed RFC 2822 addresses.";
        pub fn __init__ ( &self, field )  {
        AddrlistClass . __init__ ( self , field );
        if field {
        self . addresslist = self . getaddrlist ( );
        } else {
        self . addresslist = [ ];
        pub fn __len__ ( self )  {
        return  len ( self . addresslist );
        pub fn __add__ ( &self, other )  {
        newaddr = AddressList ( None /* Option */ );
        newaddr . addresslist = self . addresslist [ : ];
        for x in other . addresslist .iter() {
        if !x in self . addresslist {
        newaddr . addresslist . append ( x );
        return  newaddr;
        pub fn __iadd__ ( &self, other )  {
        for x in other . addresslist .iter() {
        if !x in self . addresslist {
        self . addresslist . append ( x );
        return  self;
        pub fn __sub__ ( &self, other )  {
        newaddr = AddressList ( None /* Option */ );
        for x in self . addresslist .iter() {
        if !x in other . addresslist {
        newaddr . addresslist . append ( x );
        return  newaddr;
        pub fn __isub__ ( &self, other )  {
        for x in other . addresslist .iter() {
        if x in self . addresslist {
        self . addresslist . remove ( x );
        return  self;
        pub fn __getitem__ ( &self, index )  {
        return  self . addresslist [ index ];
}

