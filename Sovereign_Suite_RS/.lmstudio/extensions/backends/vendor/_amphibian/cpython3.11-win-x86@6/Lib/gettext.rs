//! gettext.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::operator;
// use regex::Regex;
// use crate::warnings;
// use crate::locale;
// use crate::builtins;
// use crate::struct::{unpack};
// use crate::errno::{ENOENT};
// use crate::copy;

pub const __all__: &str = ["NullTranslations" ,"GNUTranslations" ,"Catalog" ,;
pub const _default_localedir: &str = os . path . join ( sys . base_prefix ,"share" ,"locale" );
pub const _token_pattern: &str = re . compile ( r"
        (?P<WHITESPACES>[ \t]+)                    | # spaces and horizontal tabs
        (?P<NUMBER>[0-9]+\b)                       | # decimal integer
        (?P<NAME>n\b)                              | # only n is allowed
        (?P<PARENTHESIS>[()])                      |
        (?P<OPERATOR>[-*/%+?:]|[><!]=?|==|&&|\|\|) | # !, *, /, %, +, -, <, >,
                                                     # <=, >=, ==, !=, &&, ||,
                                                     # ? :
                                                     # unary and bitwise ops
                                                     # not allowed
        (?P<INVALID>\w+|.)                           # invalid token
    " , re . VERBOSE | re . DOTALL );
pub fn _tokenize(plural: &str) {
        for mo in re . finditer ( _token_pattern , plural ) .iter() {
        kind = mo . lastgroup;
        if kind == "WHITESPACES" {
        continue;
        value = mo . group ( kind );
        if kind == "INVALID" {
        panic!("ValueError ( "invalid token in plural form: %s" % value )");
        yield value;
        yield "";
        pub fn _error ( value )  {
        if value {
        return  ValueError ( "unexpected token in plural form: %s" % value );
        } else {
        return  ValueError ( "unexpected end of plural form" );
        _binary_ops = (;
        ( "||" , ) ,;
        ( "&&" , ) ,;
        ( "==" , "!=" ) ,;
        ( "<" , ">" , "<=" , ">=" ) ,;
        ( "+" , "-" ) ,;
        ( "*" , "/" , "%" ) ,;
        );
        _binary_ops = { op : i for i , ops in enumerate ( _binary_ops , 1 ) for op in ops };
        _c2py_ops = { "||" : "or" , "&&" : "and" , "/" : "//" };
        pub fn _parse ( tokens , priority = -1 )  {
        result = "";
        nexttok = next ( tokens );
        while nexttok == "!"  {
        result + = "not ";
        nexttok = next ( tokens );
        if nexttok == "(" {
        sub , nexttok = _parse ( tokens );
        result = "%s(%s)" % ( result , sub );
        if nexttok != ")" {
        panic!("ValueError ( "unbalanced parenthesis in plural form" )");
        } else if nexttok == "n" {
        result = "%s%s" % ( result , nexttok );
        } else {
        // try {
        value = int ( nexttok , 10 );
        // } catch  ValueError  {
        panic!("_error ( nexttok ) from None /* Option */");
        result = "%s%d" % ( result , value );
        nexttok = next ( tokens );
        j = 100;
        while nexttok in _binary_ops  {
        i = _binary_ops [ nexttok ];
        if i < priority {
        break;
        if i in ( 3 , 4 ) && j in ( 3 , 4 ) {
        result = "(%s)" % result;
        op = _c2py_ops . get ( nexttok , nexttok );
        right , nexttok = _parse ( tokens , i + 1 );
        result = "%s %s %s" % ( result , op , right );
        j = i;
        if j == priority == 4 {
        result = "(%s)" % result;
        if nexttok == "?" && priority <= 0 {
        if_true , nexttok = _parse ( tokens , 0 );
        if nexttok != ":" {
        panic!("_error ( nexttok )");
        if_false , nexttok = _parse ( tokens );
        result = "%s if %s else %s" % ( if_true , result , if_false );
        if priority == 0 {
        result = "(%s)" % result;
        return  result , nexttok;
        pub fn _as_int ( n )  {
        // try {
        round ( n );
        // } catch  TypeError  {
        panic!("TypeError ( "Plural value must be an integer, got %s" %");
        ( n . __class__ . __name__ , ) ) from None /* Option */;
        import warnings;
        frame = sys . _getframe ( 1 );
        stacklevel = 2;
        while frame . f_back is !None /* Option */ && frame . f_globals . get ( "__name__" ) == __name__  {
        stacklevel + = 1;
        frame = frame . f_back;
        warnings . warn ( "Plural value must be an integer, got %s" %;
        ( n . __class__ . __name__ , ) ,;
        DeprecationWarning ,;
        stacklevel );
        return  n;
        pub fn c2py ( plural )  {
        "Gets a C expression as used in PO files for plural forms && returns a
    Python function that implements an equivalent expression.
    ";
        if len ( plural ) > 1000 {
        panic!("ValueError ( "plural form expression is too long" )");
        // try {
        result , nexttok = _parse ( _tokenize ( plural ) );
        if nexttok {
        panic!("_error ( nexttok )");
        depth = 0;
        for c in result .iter() {
        if c == "(" {
        depth + = 1;
        if depth > 20 {
        panic!("ValueError ( "plural form expression is too complex" )");
        } else if c == ")" {
        depth - = 1;
        ns = { "_as_int" : _as_int , "__name__" : __name__ };
        exec ( "if true:
            def func(n):
                if !isinstance(n, int):
                    n = _as_int(n)
                return int(%s)
            " % result , ns );
        return  ns [ "func" ];
        // } catch  RecursionError  {
        panic!("ValueError ( "plural form expression is too complex" )");
        pub fn _expand_lang ( loc )  {
        import locale;
        loc = locale . normalize ( loc );
        COMPONENT_CODESET = 1 < < 0;
        COMPONENT_TERRITORY = 1 < < 1;
        COMPONENT_MODIFIER = 1 < < 2;
        mask = 0;
        pos = loc . find ( "@" );
        if pos >= 0 {
        modifier = loc [ pos : ];
        loc = loc [ : pos ];
        mask | = COMPONENT_MODIFIER;
        } else {
        modifier = "";
        pos = loc . find ( "." );
        if pos >= 0 {
        codeset = loc [ pos : ];
        loc = loc [ : pos ];
        mask | = COMPONENT_CODESET;
        } else {
        codeset = "";
        pos = loc . find ( "_" );
        if pos >= 0 {
        territory = loc [ pos : ];
        loc = loc [ : pos ];
        mask | = COMPONENT_TERRITORY;
        } else {
        territory = "";
        language = loc;
        ret = [ ];
        for i in range ( mask + 1 ) .iter() {
        if !( i & ~ mask ) {
        val = language;
        if i & COMPONENT_TERRITORY { : val + = territory; }
        if i & COMPONENT_CODESET { : val + = codeset; }
        if i & COMPONENT_MODIFIER { : val + = modifier; }
        ret . append ( val );
        ret . reverse ( );
        return  ret;
        class NullTranslations ;
        pub fn __init__ ( &self, fp = None /* Option */ )  {
        self . _info = { };
        self . _charset = None /* Option */;
        self . _fallback = None /* Option */;
        if fp is !None /* Option */ {
        self . _parse ( fp );
        pub fn _parse ( &self, fp )  {
        // pass
        pub fn add_fallback ( &self, fallback )  {
        if self . _fallback {
        self . _fallback . add_fallback ( fallback );
        } else {
        self . _fallback = fallback;
        pub fn gettext ( &self, message )  {
        if self . _fallback {
        return  self . _fallback . gettext ( message );
        return  message;
        pub fn ngettext ( &self, msgid1 , msgid2 , n )  {
        if self . _fallback {
        return  self . _fallback . ngettext ( msgid1 , msgid2 , n );
        if n == 1 {
        return  msgid1;
        } else {
        return  msgid2;
        pub fn pgettext ( &self, context , message )  {
        if self . _fallback {
        return  self . _fallback . pgettext ( context , message );
        return  message;
        pub fn npgettext ( &self, context , msgid1 , msgid2 , n )  {
        if self . _fallback {
        return  self . _fallback . npgettext ( context , msgid1 , msgid2 , n );
        if n == 1 {
        return  msgid1;
        } else {
        return  msgid2;
        pub fn info ( self )  {
        return  self . _info;
        pub fn charset ( self )  {
        return  self . _charset;
        pub fn install ( &self, names = None /* Option */ )  {
        import builtins;
        builtins . __dict__ [ "_" ] = self . gettext;
        if names is !None /* Option */ {
        allowed = { "gettext" , "ngettext" , "npgettext" , "pgettext" };
        for name in allowed & set ( names ) .iter() {
        builtins . __dict__ [ name ] = getattr ( self , name );
        class GNUTranslations ( NullTranslations ) ;
        LE_MAGIC = 0x950412 de;
        BE_MAGIC = 0x de120495;
        CONTEXT = "%s\x04%s";
        VERSIONS = ( 0 , 1 );
        pub fn _get_versions ( &self, version )  {
        "Returns a tuple of major version, minor version";
        return  ( version > > 16 , version & 0x ffff );
        pub fn _parse ( &self, fp )  {
        "Override this method to support alternative .mo formats.";
        from struct import unpack;
        filename = getattr ( fp , "name" , "" );
        self . _catalog = catalog = { };
        self . plural = lambda n : int ( n != 1 );
        buf = fp . read ( );
        buflen = len ( buf );
        magic = unpack ( "<I" , buf [ : 4 ] ) [ 0 ];
        if magic == self . LE_MAGIC {
        version , msgcount , masteridx , transidx = unpack ( "<4I" , buf [ 4 : 20 ] );
        ii = "<II";
        } else if magic == self . BE_MAGIC {
        version , msgcount , masteridx , transidx = unpack ( ">4I" , buf [ 4 : 20 ] );
        ii = ">II";
        } else {
        panic!("OSError ( 0 , "Bad magic number" , filename )");
        major_version , minor_version = self . _get_versions ( version );
        if major_version !in self . VERSIONS {
        panic!("OSError ( 0 , "Bad version number " + str ( major_version ) , filename )");
        for i in range ( 0 , msgcount ) .iter() {
        mlen , moff = unpack ( ii , buf [ masteridx : masteridx + 8 ] );
        mend = moff + mlen;
        tlen , toff = unpack ( ii , buf [ transidx : transidx + 8 ] );
        tend = toff + tlen;
        if mend < buflen && tend < buflen {
        msg = buf [ moff : mend ];
        tmsg = buf [ toff : tend ];
        } else {
        panic!("OSError ( 0 , "File is corrupt" , filename )");
        if mlen == 0 {
        lastk = None /* Option */;
        for b_item in tmsg . split ( b "\n" ) .iter() {
        item = b_item . decode ( ) . strip ( );
        if !item {
        continue;
        if item . startswith ( "#-#-#-#-#" ) && item . endswith ( "#-#-#-#-#" ) {
        continue;
        k = v = None /* Option */;
        if ":" in item {
        k , v = item . split ( ":" , 1 );
        k = k . strip ( ) . lower ( );
        v = v . strip ( );
        self . _info [ k ] = v;
        lastk = k;
        } else if lastk {
        self . _info [ lastk ] + = "\n" + item;
        if k == "content-type" {
        self . _charset = v . split ( "charset=" ) [ 1 ];
        } else if k == "plural-forms" {
        v = v . split ( ";" );
        plural = v [ 1 ] . split ( "plural=" ) [ 1 ];
        self . plural = c2py ( plural );
        charset = self . _charset || "ascii";
        if b "\x00" in msg {
        msgid1 , msgid2 = msg . split ( b "\x00" );
        tmsg = tmsg . split ( b "\x00" );
        msgid1 = str ( msgid1 , charset );
        for i , x in enumerate ( tmsg ) .iter() {
        catalog [ ( msgid1 , i ) ] = str ( x , charset );
        } else {
        catalog [ str ( msg , charset ) ] = str ( tmsg , charset );
        masteridx + = 8;
        transidx + = 8;
        pub fn gettext ( &self, message )  {
        missing = object ( );
        tmsg = self . _catalog . get ( message , missing );
        if tmsg is missing {
        tmsg = self . _catalog . get ( ( message , self . plural ( 1 ) ) , missing );
        if tmsg is !missing {
        return  tmsg;
        if self . _fallback {
        return  self . _fallback . gettext ( message );
        return  message;
        pub fn ngettext ( &self, msgid1 , msgid2 , n )  {
        // try {
        tmsg = self . _catalog [ ( msgid1 , self . plural ( n ) ) ];
        // } catch  KeyError  {
        if self . _fallback {
        return  self . _fallback . ngettext ( msgid1 , msgid2 , n );
        if n == 1 {
        tmsg = msgid1;
        } else {
        tmsg = msgid2;
        return  tmsg;
        pub fn pgettext ( &self, context , message )  {
        ctxt_msg_id = self . CONTEXT % ( context , message );
        missing = object ( );
        tmsg = self . _catalog . get ( ctxt_msg_id , missing );
        if tmsg is missing {
        tmsg = self . _catalog . get ( ( ctxt_msg_id , self . plural ( 1 ) ) , missing );
        if tmsg is !missing {
        return  tmsg;
        if self . _fallback {
        return  self . _fallback . pgettext ( context , message );
        return  message;
        pub fn npgettext ( &self, context , msgid1 , msgid2 , n )  {
        ctxt_msg_id = self . CONTEXT % ( context , msgid1 );
        // try {
        tmsg = self . _catalog [ ctxt_msg_id , self . plural ( n ) ];
        // } catch  KeyError  {
        if self . _fallback {
        return  self . _fallback . npgettext ( context , msgid1 , msgid2 , n );
        if n == 1 {
        tmsg = msgid1;
        } else {
        tmsg = msgid2;
        return  tmsg;
        pub fn find ( domain , localedir = None /* Option */ , languages = None /* Option */ , all = false )  {
        if localedir is None /* Option */ {
        localedir = _default_localedir;
        if languages is None /* Option */ {
        languages = [ ];
        for envar in ( "LANGUAGE" , "LC_ALL" , "LC_MESSAGES" , "LANG" ) .iter() {
        val = os . environ . get ( envar );
        if val {
        languages = val . split ( ":" );
        break;
        if "C" !in languages {
        languages . append ( "C" );
        nelangs = [ ];
        for lang in languages .iter() {
        for nelang in _expand_lang ( lang ) .iter() {
        if nelang !in nelangs {
        nelangs . append ( nelang );
        if all {
        result = [ ];
        } else {
        result = None /* Option */;
        for lang in nelangs .iter() {
        if lang == "C" {
        break;
        mofile = os . path . join ( localedir , lang , "LC_MESSAGES" , "%s.mo" % domain );
        if os . path . exists ( mofile ) {
        if all {
        result . append ( mofile );
        } else {
        return  mofile;
        return  result;
        _translations = { };
        pub fn translation ( domain , localedir = None /* Option */ , languages = None /* Option */ , {
        class_ = None /* Option */ , fallback = false ) ;
        if class_ is None /* Option */ {
        class_ = GNUTranslations;
        mofiles = find ( domain , localedir , languages , all = true );
        if !mofiles {
        if fallback {
        return  NullTranslations ( );
        from errno import ENOENT;
        panic!("FileNotFoundError ( ENOENT ,");
        "No translation file found for domain" , domain );
        result = None /* Option */;
        for mofile in mofiles .iter() {
        key = ( class_ , os . path . abspath ( mofile ) );
        t = _translations . get ( key );
        if t is None /* Option */ {
        // with scope: open ( mofile , "rb" ) as fp  {
        t = _translations . setdefault ( key , class_ ( fp ) );
        import copy;
        t = copy . copy ( t );
        if result is None /* Option */ {
        result = t;
        } else {
        result . add_fallback ( t );
        return  result;
        pub fn install ( domain , localedir = None /* Option */ , * , names = None /* Option */ )  {
        t = translation ( domain , localedir , fallback = true );
        t . install ( names );
        _localedirs = { };
        _current_domain = "messages";
        pub fn textdomain ( domain = None /* Option */ )  {
        global _current_domain;
        if domain is !None /* Option */ {
        _current_domain = domain;
        return  _current_domain;
        pub fn bindtextdomain ( domain , localedir = None /* Option */ )  {
        global _localedirs;
        if localedir is !None /* Option */ {
        _localedirs [ domain ] = localedir;
        return  _localedirs . get ( domain , _default_localedir );
        pub fn dgettext ( domain , message )  {
        // try {
        t = translation ( domain , _localedirs . get ( domain , None /* Option */ ) );
        // } catch  OSError  {
        return  message;
        return  t . gettext ( message );
        pub fn dngettext ( domain , msgid1 , msgid2 , n )  {
        // try {
        t = translation ( domain , _localedirs . get ( domain , None /* Option */ ) );
        // } catch  OSError  {
        if n == 1 {
        return  msgid1;
        } else {
        return  msgid2;
        return  t . ngettext ( msgid1 , msgid2 , n );
        pub fn dpgettext ( domain , context , message )  {
        // try {
        t = translation ( domain , _localedirs . get ( domain , None /* Option */ ) );
        // } catch  OSError  {
        return  message;
        return  t . pgettext ( context , message );
        pub fn dnpgettext ( domain , context , msgid1 , msgid2 , n )  {
        // try {
        t = translation ( domain , _localedirs . get ( domain , None /* Option */ ) );
        // } catch  OSError  {
        if n == 1 {
        return  msgid1;
        } else {
        return  msgid2;
        return  t . npgettext ( context , msgid1 , msgid2 , n );
        pub fn gettext ( message )  {
        return  dgettext ( _current_domain , message );
        pub fn ngettext ( msgid1 , msgid2 , n )  {
        return  dngettext ( _current_domain , msgid1 , msgid2 , n );
        pub fn pgettext ( context , message )  {
        return  dpgettext ( _current_domain , context , message );
        pub fn npgettext ( context , msgid1 , msgid2 , n )  {
        return  dnpgettext ( _current_domain , context , msgid1 , msgid2 , n );
        Catalog = translation;
}

