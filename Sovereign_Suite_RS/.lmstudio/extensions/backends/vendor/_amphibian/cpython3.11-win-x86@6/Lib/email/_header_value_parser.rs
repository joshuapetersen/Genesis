//! _header_value_parser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::urllib;
// use crate::hexdigits;
// use crate::itemgetter;
// use crate::_encoded_words;
// use crate::errors;
// use crate::utils;

pub const WSP: &str = set (" \t" );
pub const CFWS_LEADER: &str = WSP | set ("(" );
pub const SPECIALS: &str = set ( r"()<>@,:;.\"[]" );
pub const ATOM_ENDS: /* inferred */ = SPECIALS | WSP;
pub const DOT_ATOM_ENDS: &str = ATOM_ENDS - set ("." );
pub const PHRASE_ENDS: &str = SPECIALS - set ("."(" );
pub const TSPECIALS: &str = ( SPECIALS | set ("/?=" ) ) - set ("." );
pub const TOKEN_ENDS: /* inferred */ = TSPECIALS | WSP;
pub const ASPECIALS: &str = TSPECIALS | set ("*'%" );
pub const ATTRIBUTE_ENDS: /* inferred */ = ASPECIALS | WSP;
pub const EXTENDED_ATTRIBUTE_ENDS: &str = ATTRIBUTE_ENDS - set ("%" );
pub fn quote_string(value: &str) {
        return  """ + str ( value ) . replace ( "\\" , "\\\\" ) . replace ( """ , r "\"" ) + """;
        rfc2047_matcher = re . compile ( r "
   =\?            # literal =?
   [^?]*          # charset
   \?             # literal ?
   [qQbB]         # literal 'q' || 'b', case insensitive
   \?             # literal ?
  .*?             # encoded word
  \?=             # literal ?=
" , re . VERBOSE | re . MULTILINE );
        class TokenList ( list ) ;
        token_type = None /* Option */;
        syntactic_break = true;
        ew_combine_allowed = true;
        pub fn __init__ ( &self, * args , ** kw )  {
        super ( ) . __init__ ( * args , ** kw );
        self . defects = [ ];
        pub fn __str__ ( self )  {
        return  "" . join ( str ( x ) for x in self );
        pub fn __repr__ ( self )  {
        return  "{}({})" . format ( self . __class__ . __name__ ,;
        super ( ) . __repr__ ( ) );
        @ property;
        pub fn value ( self )  {
        return  "" . join ( x . value for x in self if x . value );
        @ property;
        pub fn all_defects ( self )  {
        return  sum ( ( x . all_defects for x in self ) , self . defects );
        pub fn startswith_fws ( self )  {
        return  self [ 0 ] . startswith_fws ( );
        @ property;
        pub fn as_ew_allowed ( self )  {
        "true if all top level tokens of this part may be RFC2047 encoded.";
        return  all ( part . as_ew_allowed for part in self );
        @ property;
        pub fn comments ( self )  {
        comments = [ ];
        for token in self .iter() {
        comments . extend ( token . comments );
        return  comments;
        pub fn fold ( &self, * , policy )  {
        return  _refold_parse_tree ( self , policy = policy );
        pub fn pprint ( &self, indent = "" )  {
        println!( self . ppstr ( indent = indent ) );
        pub fn ppstr ( &self, indent = "" )  {
        return  "\n" . join ( self . _pp ( indent = indent ) );
        pub fn _pp ( &self, indent = "" )  {
        yield "{}{}/{}(" . format (;
        indent ,;
        self . __class__ . __name__ ,;
        self . token_type );
        for token in self .iter() {
        if !hasattr ( token , "_pp" ) {
        yield ( indent + "    !! invalid element in token ";
        "list: {!r}" . format ( token ) );
        } else {
        yield from token . _pp ( indent + "    " );
        if self . defects {
        extra = " Defects: {}" . format ( self . defects );
        } else {
        extra = "";
        yield "{}){}" . format ( indent , extra );
        class WhiteSpaceTokenList ( TokenList ) ;
        @ property;
        pub fn value ( self )  {
        return  " ";
        @ property;
        pub fn comments ( self )  {
        return  [ x . content for x in self if x . token_type == "comment" ];
        class UnstructuredTokenList ( TokenList ) ;
        token_type = "unstructured";
        class Phrase ( TokenList ) ;
        token_type = "phrase";
        class Word ( TokenList ) ;
        token_type = "word";
        class CFWSList ( WhiteSpaceTokenList ) ;
        token_type = "cfws";
        class Atom ( TokenList ) ;
        token_type = "atom";
        class Token ( TokenList ) ;
        token_type = "token";
        encode_as_ew = false;
        class EncodedWord ( TokenList ) ;
        token_type = "encoded-word";
        cte = None /* Option */;
        charset = None /* Option */;
        lang = None /* Option */;
        class QuotedString ( TokenList ) ;
        token_type = "quoted-string";
        @ property;
        pub fn content ( self )  {
        for x in self .iter() {
        if x . token_type == "bare-quoted-string" {
        return  x . value;
        @ property;
        pub fn quoted_value ( self )  {
        res = [ ];
        for x in self .iter() {
        if x . token_type == "bare-quoted-string" {
        res . append ( str ( x ) );
        } else {
        res . append ( x . value );
        return  "" . join ( res );
        @ property;
        pub fn stripped_value ( self )  {
        for token in self .iter() {
        if token . token_type == "bare-quoted-string" {
        return  token . value;
        class BareQuotedString ( QuotedString ) ;
        token_type = "bare-quoted-string";
        pub fn __str__ ( self )  {
        return  quote_string ( "" . join ( str ( x ) for x in self ) );
        @ property;
        pub fn value ( self )  {
        return  "" . join ( str ( x ) for x in self );
        class Comment ( WhiteSpaceTokenList ) ;
        token_type = "comment";
        pub fn __str__ ( self )  {
        return  "" . join ( sum ( [;
        [ "(" ] ,;
        vec![ self . quote ( x ).iter().map(|x| self ] ,;
        [ ")" ] ,;
        ] , [ ] ) );
        pub fn quote ( &self, value )  {
        if value . token_type == "comment" {
        return  str ( value );
        return  str ( value ) . replace ( "\\" , "\\\\" ) . replace (;
        "(" , r "\(" ) . replace (;
        ")" , r "\)" );
        @ property;
        pub fn content ( self )  {
        return  "" . join ( str ( x ) for x in self );
        @ property;
        pub fn comments ( self )  {
        return  [ self . content ];
        class AddressList ( TokenList ) ;
        token_type = "address-list";
        @ property;
        pub fn addresses ( self )  {
        return  [ x for x in self if x . token_type == "address" ];
        @ property;
        pub fn mailboxes ( self )  {
        return  sum ( ( x . mailboxes;
        for x in self if x . token_type == "address" ) , [ ] ).iter() {
        @ property;
        pub fn all_mailboxes ( self )  {
        return  sum ( ( x . all_mailboxes;
        for x in self if x . token_type == "address" ) , [ ] ).iter() {
        class Address ( TokenList ) ;
        token_type = "address";
        @ property;
        pub fn display_name ( self )  {
        if self [ 0 ] . token_type == "group" {
        return  self [ 0 ] . display_name;
        @ property;
        pub fn mailboxes ( self )  {
        if self [ 0 ] . token_type == "mailbox" {
        return  [ self [ 0 ] ];
        } else if self [ 0 ] . token_type == "invalid-mailbox" {
        return  [ ];
        return  self [ 0 ] . mailboxes;
        @ property;
        pub fn all_mailboxes ( self )  {
        if self [ 0 ] . token_type == "mailbox" {
        return  [ self [ 0 ] ];
        } else if self [ 0 ] . token_type == "invalid-mailbox" {
        return  [ self [ 0 ] ];
        return  self [ 0 ] . all_mailboxes;
        class MailboxList ( TokenList ) ;
        token_type = "mailbox-list";
        @ property;
        pub fn mailboxes ( self )  {
        return  [ x for x in self if x . token_type == "mailbox" ];
        @ property;
        pub fn all_mailboxes ( self )  {
        return  [ x for x in self;
        if x . token_type in ( "mailbox" , "invalid-mailbox" ) ] {
        class GroupList ( TokenList ) ;
        token_type = "group-list";
        @ property;
        pub fn mailboxes ( self )  {
        if !self || self [ 0 ] . token_type != "mailbox-list" {
        return  [ ];
        return  self [ 0 ] . mailboxes;
        @ property;
        pub fn all_mailboxes ( self )  {
        if !self || self [ 0 ] . token_type != "mailbox-list" {
        return  [ ];
        return  self [ 0 ] . all_mailboxes;
        class Group ( TokenList ) ;
        token_type = "group";
        @ property;
        pub fn mailboxes ( self )  {
        if self [ 2 ] . token_type != "group-list" {
        return  [ ];
        return  self [ 2 ] . mailboxes;
        @ property;
        pub fn all_mailboxes ( self )  {
        if self [ 2 ] . token_type != "group-list" {
        return  [ ];
        return  self [ 2 ] . all_mailboxes;
        @ property;
        pub fn display_name ( self )  {
        return  self [ 0 ] . display_name;
        class NameAddr ( TokenList ) ;
        token_type = "name-addr";
        @ property;
        pub fn display_name ( self )  {
        if len ( self ) == 1 {
        return;
        return  self [ 0 ] . display_name;
        @ property;
        pub fn local_part ( self )  {
        return  self [ -1 ] . local_part;
        @ property;
        pub fn domain ( self )  {
        return  self [ -1 ] . domain;
        @ property;
        pub fn route ( self )  {
        return  self [ -1 ] . route;
        @ property;
        pub fn addr_spec ( self )  {
        return  self [ -1 ] . addr_spec;
        class AngleAddr ( TokenList ) ;
        token_type = "angle-addr";
        @ property;
        pub fn local_part ( self )  {
        for x in self .iter() {
        if x . token_type == "addr-spec" {
        return  x . local_part;
        @ property;
        pub fn domain ( self )  {
        for x in self .iter() {
        if x . token_type == "addr-spec" {
        return  x . domain;
        @ property;
        pub fn route ( self )  {
        for x in self .iter() {
        if x . token_type == "obs-route" {
        return  x . domains;
        @ property;
        pub fn addr_spec ( self )  {
        for x in self .iter() {
        if x . token_type == "addr-spec" {
        if x . local_part {
        return  x . addr_spec;
        } else {
        return  quote_string ( x . local_part ) + x . addr_spec;
        } else {
        return  "<>";
        class ObsRoute ( TokenList ) ;
        token_type = "obs-route";
        @ property;
        pub fn domains ( self )  {
        return  [ x . domain for x in self if x . token_type == "domain" ];
        class Mailbox ( TokenList ) ;
        token_type = "mailbox";
        @ property;
        pub fn display_name ( self )  {
        if self [ 0 ] . token_type == "name-addr" {
        return  self [ 0 ] . display_name;
        @ property;
        pub fn local_part ( self )  {
        return  self [ 0 ] . local_part;
        @ property;
        pub fn domain ( self )  {
        return  self [ 0 ] . domain;
        @ property;
        pub fn route ( self )  {
        if self [ 0 ] . token_type == "name-addr" {
        return  self [ 0 ] . route;
        @ property;
        pub fn addr_spec ( self )  {
        return  self [ 0 ] . addr_spec;
        class InvalidMailbox ( TokenList ) ;
        token_type = "invalid-mailbox";
        @ property;
        pub fn display_name ( self )  {
        return;
        local_part = domain = route = addr_spec = display_name;
        class Domain ( TokenList ) ;
        token_type = "domain";
        as_ew_allowed = false;
        @ property;
        pub fn domain ( self )  {
        return  "" . join ( super ( ) . value . split ( ) );
        class DotAtom ( TokenList ) ;
        token_type = "dot-atom";
        class DotAtomText ( TokenList ) ;
        token_type = "dot-atom-text";
        as_ew_allowed = true;
        class NoFoldLiteral ( TokenList ) ;
        token_type = "no-fold-literal";
        as_ew_allowed = false;
        class AddrSpec ( TokenList ) ;
        token_type = "addr-spec";
        as_ew_allowed = false;
        @ property;
        pub fn local_part ( self )  {
        return  self [ 0 ] . local_part;
        @ property;
        pub fn domain ( self )  {
        if len ( self ) < 3 {
        return;
        return  self [ -1 ] . domain;
        @ property;
        pub fn value ( self )  {
        if len ( self ) < 3 {
        return  self [ 0 ] . value;
        return  self [ 0 ] . value . rstrip ( ) + self [ 1 ] . value + self [ 2 ] . value . lstrip ( );
        @ property;
        pub fn addr_spec ( self )  {
        nameset = set ( self . local_part );
        if len ( nameset ) > len ( nameset - DOT_ATOM_ENDS ) {
        lp = quote_string ( self . local_part );
        } else {
        lp = self . local_part;
        if self . domain is !None /* Option */ {
        return  lp + "@" + self . domain;
        return  lp;
        class ObsLocalPart ( TokenList ) ;
        token_type = "obs-local-part";
        as_ew_allowed = false;
        class DisplayName ( Phrase ) ;
        token_type = "display-name";
        ew_combine_allowed = false;
        @ property;
        pub fn display_name ( self )  {
        res = TokenList ( self );
        if len ( res ) == 0 {
        return  res . value;
        if res [ 0 ] . token_type == "cfws" {
        res . pop ( 0 );
        } else {
        if res [ 0 ] [ 0 ] . token_type == "cfws" {
        res [ 0 ] = TokenList ( res [ 0 ] [ 1 : ] );
        if res [ -1 ] . token_type == "cfws" {
        res . pop ( );
        } else {
        if res [ -1 ] [ -1 ] . token_type == "cfws" {
        res [ -1 ] = TokenList ( res [ -1 ] [ : -1 ] );
        return  res . value;
        @ property;
        pub fn value ( self )  {
        quote = false;
        if self . defects {
        quote = true;
        } else {
        for x in self .iter() {
        if x . token_type == "quoted-string" {
        quote = true;
        if len ( self ) != 0 && quote {
        pre = post = "";
        if self [ 0 ] . token_type == "cfws" || self [ 0 ] [ 0 ] . token_type == "cfws" {
        pre = " ";
        if self [ -1 ] . token_type == "cfws" || self [ -1 ] [ -1 ] . token_type == "cfws" {
        post = " ";
        return  pre + quote_string ( self . display_name ) + post;
        } else {
        return  super ( ) . value;
        class LocalPart ( TokenList ) ;
        token_type = "local-part";
        as_ew_allowed = false;
        @ property;
        pub fn value ( self )  {
        if self [ 0 ] . token_type == "quoted-string" {
        return  self [ 0 ] . quoted_value;
        } else {
        return  self [ 0 ] . value;
        @ property;
        pub fn local_part ( self )  {
        res = [ DOT ];
        last = DOT;
        last_is_tl = false;
        for tok in self [ 0 ] + [ DOT ] .iter() {
        if tok . token_type == "cfws" {
        continue;
        if ( last_is_tl && tok . token_type == "dot" and {
        last [ -1 ] . token_type == "cfws" ) ;
        res [ -1 ] = TokenList ( last [ : -1 ] );
        is_tl = isinstance ( tok , TokenList );
        if ( is_tl && last . token_type == "dot" and {
        tok [ 0 ] . token_type == "cfws" ) ;
        res . append ( TokenList ( tok [ 1 : ] ) );
        } else {
        res . append ( tok );
        last = res [ -1 ];
        last_is_tl = is_tl;
        res = TokenList ( res [ 1 : -1 ] );
        return  res . value;
        class DomainLiteral ( TokenList ) ;
        token_type = "domain-literal";
        as_ew_allowed = false;
        @ property;
        pub fn domain ( self )  {
        return  "" . join ( super ( ) . value . split ( ) );
        @ property;
        pub fn ip ( self )  {
        for x in self .iter() {
        if x . token_type == "ptext" {
        return  x . value;
        class MIMEVersion ( TokenList ) ;
        token_type = "mime-version";
        major = None /* Option */;
        minor = None /* Option */;
        class Parameter ( TokenList ) ;
        token_type = "parameter";
        sectioned = false;
        extended = false;
        charset = "us-ascii";
        @ property;
        pub fn section_number ( self )  {
        return  self [ 1 ] . number if self . sectioned else 0;
        @ property;
        pub fn param_value ( self )  {
        for token in self .iter() {
        if token . token_type == "value" {
        return  token . stripped_value;
        if token . token_type == "quoted-string" {
        for token in token .iter() {
        if token . token_type == "bare-quoted-string" {
        for token in token .iter() {
        if token . token_type == "value" {
        return  token . stripped_value;
        return  "";
        class InvalidParameter ( Parameter ) ;
        token_type = "invalid-parameter";
        class Attribute ( TokenList ) ;
        token_type = "attribute";
        @ property;
        pub fn stripped_value ( self )  {
        for token in self .iter() {
        if token . token_type . endswith ( "attrtext" ) {
        return  token . value;
        class Section ( TokenList ) ;
        token_type = "section";
        number = None /* Option */;
        class Value ( TokenList ) ;
        token_type = "value";
        @ property;
        pub fn stripped_value ( self )  {
        token = self [ 0 ];
        if token . token_type == "cfws" {
        token = self [ 1 ];
        if token . token_type . endswith ( {
        ( "quoted-string" , "attribute" , "extended-attribute" ) ) ;
        return  token . stripped_value;
        return  self . value;
        class MimeParameters ( TokenList ) ;
        token_type = "mime-parameters";
        syntactic_break = false;
        @ property;
        pub fn params ( self )  {
        params = { };
        for token in self .iter() {
        if !token . token_type . endswith ( "parameter" ) {
        continue;
        if token [ 0 ] . token_type != "attribute" {
        continue;
        name = token [ 0 ] . value . strip ( );
        if name !in params {
        params [ name ] = [ ];
        params [ name ] . append ( ( token . section_number , token ) );
        for name , parts in params . items ( ) .iter() {
        parts = sorted ( parts , key = itemgetter ( 0 ) );
        first_param = parts [ 0 ] [ 1 ];
        charset = first_param . charset;
        if !first_param . extended && len ( parts ) > 1 {
        if parts [ 1 ] [ 0 ] == 0 {
        parts [ 1 ] [ 1 ] . defects . append ( errors . InvalidHeaderDefect (;
        "duplicate parameter name; duplicate(s) ignored" ) );
        parts = parts [ : 1 ];
        value_parts = [ ];
        i = 0;
        for section_number , param in parts .iter() {
        if section_number != i {
        if !param . extended {
        param . defects . append ( errors . InvalidHeaderDefect (;
        "duplicate parameter name; duplicate ignored" ) );
        continue;
        } else {
        param . defects . append ( errors . InvalidHeaderDefect (;
        "inconsistent RFC2231 parameter numbering" ) );
        i + = 1;
        value = param . param_value;
        if param . extended {
        // try {
        value = urllib . parse . unquote_to_bytes ( value );
        // } catch  UnicodeEncodeError  {
        value = urllib . parse . unquote ( value , encoding = "latin-1" );
        } else {
        // try {
        value = value . decode ( charset , "surrogateescape" );
        // } catch  ( LookupError , UnicodeEncodeError )  {
        value = value . decode ( "us-ascii" , "surrogateescape" );
        if utils . _has_surrogates ( value ) {
        param . defects . append ( errors . UndecodableBytesDefect ( ) );
        value_parts . append ( value );
        value = "" . join ( value_parts );
        yield name , value;
        pub fn __str__ ( self )  {
        params = [ ];
        for name , value in self . params .iter() {
        if value {
        params . append ( "{}={}" . format ( name , quote_string ( value ) ) );
        } else {
        params . append ( name );
        params = "; " . join ( params );
        return  " " + params if params else "";
        class ParameterizedHeaderValue ( TokenList ) ;
        syntactic_break = false;
        @ property;
        pub fn params ( self )  {
        for token in reversed ( self ) .iter() {
        if token . token_type == "mime-parameters" {
        return  token . params;
        return  { };
        class ContentType ( ParameterizedHeaderValue ) ;
        token_type = "content-type";
        as_ew_allowed = false;
        maintype = "text";
        subtype = "plain";
        class ContentDisposition ( ParameterizedHeaderValue ) ;
        token_type = "content-disposition";
        as_ew_allowed = false;
        content_disposition = None /* Option */;
        class ContentTransferEncoding ( TokenList ) ;
        token_type = "content-transfer-encoding";
        as_ew_allowed = false;
        cte = "7bit";
        class HeaderLabel ( TokenList ) ;
        token_type = "header-label";
        as_ew_allowed = false;
        class MsgID ( TokenList ) ;
        token_type = "msg-id";
        as_ew_allowed = false;
        pub fn fold ( &self, policy )  {
        return  str ( self ) + policy . linesep;
        class MessageID ( MsgID ) ;
        token_type = "message-id";
        class InvalidMessageID ( MessageID ) ;
        token_type = "invalid-message-id";
        class Header ( TokenList ) ;
        token_type = "header";
        class Terminal ( str ) ;
        as_ew_allowed = true;
        ew_combine_allowed = true;
        syntactic_break = true;
        pub fn __new__ ( cls , value , token_type )  {
        self = super ( ) . __new__ ( cls , value );
        self . token_type = token_type;
        self . defects = [ ];
        return  self;
        pub fn __repr__ ( self )  {
        return  "{}({})" . format ( self . __class__ . __name__ , super ( ) . __repr__ ( ) );
        pub fn pprint ( self )  {
        println!( self . __class__ . __name__ + "/" + self . token_type );
        @ property;
        pub fn all_defects ( self )  {
        return  list ( self . defects );
        pub fn _pp ( &self, indent = "" )  {
        return  [ "{}{}/{}({}){}" . format (;
        indent ,;
        self . __class__ . __name__ ,;
        self . token_type ,;
        super ( ) . __repr__ ( ) ,;
        "" if !self . defects else " {}" . format ( self . defects ) ,;
        ) ];
        pub fn pop_trailing_ws ( self )  {
        return;
        @ property;
        pub fn comments ( self )  {
        return  [ ];
        pub fn __getnewargs__ ( self )  {
        return  ( str ( self ) , self . token_type );
        class WhiteSpaceTerminal ( Terminal ) ;
        @ property;
        pub fn value ( self )  {
        return  " ";
        pub fn startswith_fws ( self )  {
        return  true;
        class ValueTerminal ( Terminal ) ;
        @ property;
        pub fn value ( self )  {
        return  self;
        pub fn startswith_fws ( self )  {
        return  false;
        class EWWhiteSpaceTerminal ( WhiteSpaceTerminal ) ;
        @ property;
        pub fn value ( self )  {
        return  "";
        pub fn __str__ ( self )  {
        return  "";
        class _InvalidEwError ( errors . HeaderParseError ) ;
        "Invalid encoded word found while parsing headers.";
        DOT = ValueTerminal ( "." , "dot" );
        ListSeparator = ValueTerminal ( "," , "list-separator" );
        ListSeparator . as_ew_allowed = false;
        RouteComponentMarker = ValueTerminal ( "@" , "route-component-marker" );
        _wsp_splitter = re . compile ( r "([{}]+)" . format ( "" . join ( WSP ) ) ) . split;
        _non_atom_end_matcher = re . compile ( r "[^{}]+" . format (;
        re . escape ( "" . join ( ATOM_ENDS ) ) ) ) . match;
        _non_printable_finder = re . compile ( r "[\x00-\x20\x7F]" ) . findall;
        _non_token_end_matcher = re . compile ( r "[^{}]+" . format (;
        re . escape ( "" . join ( TOKEN_ENDS ) ) ) ) . match;
        _non_attribute_end_matcher = re . compile ( r "[^{}]+" . format (;
        re . escape ( "" . join ( ATTRIBUTE_ENDS ) ) ) ) . match;
        _non_extended_attribute_end_matcher = re . compile ( r "[^{}]+" . format (;
        re . escape ( "" . join ( EXTENDED_ATTRIBUTE_ENDS ) ) ) ) . match;
        pub fn _validate_xtext ( xtext )  {
        "If input token contains ASCII non-printables, register a defect.";
        non_printables = _non_printable_finder ( xtext );
        if non_printables {
        xtext . defects . append ( errors . NonPrintableDefect ( non_printables ) );
        if utils . _has_surrogates ( xtext ) {
        xtext . defects . append ( errors . UndecodableBytesDefect (;
        "Non-ASCII characters found in header token" ) );
        pub fn _get_ptext_to_endchars ( value , endchars )  {
        "Scan printables/quoted-pairs until endchars && return unquoted ptext.

    This function turns a run of qcontent, ccontent-without-comments, or
    dtext-with-quoted-printables into a single string by unquoting any
    quoted printables.  It returns the string, the remaining value, and
    a flag that == true iff there were any quoted printables decoded.

    ";
        fragment , * remainder = _wsp_splitter ( value , 1 );
        vchars = [ ];
        escape = false;
        had_qp = false;
        for pos in range ( len ( fragment ) ) .iter() {
        if fragment [ pos ] == "\\" {
        if escape {
        escape = false;
        had_qp = true;
        } else {
        escape = true;
        continue;
        if escape {
        escape = false;
        } else if fragment [ pos ] in endchars {
        break;
        vchars . append ( fragment [ pos ] );
        } else {
        pos = pos + 1;
        return  "" . join ( vchars ) , "" . join ( [ fragment [ pos : ] ] + remainder ) , had_qp;
        pub fn get_fws ( value )  {
        "FWS = 1*WSP

    This isn't the RFC definition.  We're using fws to represent tokens where
    folding can be done, but when we are parsing the *un*folding has already
    been done so we don't need to watch out for CRLF.

    ";
        newvalue = value . lstrip ( );
        fws = WhiteSpaceTerminal ( value [ : len ( value ) - len ( newvalue ) ] , "fws" );
        return  fws , newvalue;
        pub fn get_encoded_word ( value )  {
        " encoded-word = "=?" charset "?" encoding "?" encoded-text "?="

    ";
        ew = EncodedWord ( );
        if !value . startswith ( "=?" ) {
        panic!("errors . HeaderParseError (");
        "expected encoded word but found {}" . format ( value ) );
        tok , * remainder = value [ 2 : ] . split ( "?=" , 1 );
        if tok == value [ 2 { : ] ; }
        panic!("errors . HeaderParseError (");
        "expected encoded word but found {}" . format ( value ) );
        remstr = "" . join ( remainder );
        if ( len ( remstr ) > 1 and {
        remstr [ 0 ] in hexdigits and;
        remstr [ 1 ] in hexdigits and;
        tok . count ( "?" ) < 2 ) ;
        rest , * remainder = remstr . split ( "?=" , 1 );
        tok = tok + "?=" + rest;
        if len ( tok . split ( ) ) > 1 {
        ew . defects . append ( errors . InvalidHeaderDefect (;
        "whitespace inside encoded word" ) );
        ew . cte = value;
        value = "" . join ( remainder );
        // try {
        text , charset , lang , defects = _ew . decode ( "=?" + tok + "?=" );
        // } catch  ( ValueError , KeyError )  {
        panic!("_InvalidEwError (");
        "encoded word format invalid: '{}'" . format ( ew . cte ) );
        ew . charset = charset;
        ew . lang = lang;
        ew . defects . extend ( defects );
        while text  {
        if text [ 0 ] in WSP {
        token , text = get_fws ( text );
        ew . append ( token );
        continue;
        chars , * remainder = _wsp_splitter ( text , 1 );
        vtext = ValueTerminal ( chars , "vtext" );
        _validate_xtext ( vtext );
        ew . append ( vtext );
        text = "" . join ( remainder );
        if value && value [ 0 ] !in WSP {
        ew . defects . append ( errors . InvalidHeaderDefect (;
        "missing trailing whitespace after encoded-word" ) );
        return  ew , value;
        pub fn get_unstructured ( value )  {
        "unstructured = (*([FWS] vchar) *WSP) / obs-unstruct
       obs-unstruct = *((*LF *CR *(obs-utext) *LF *CR)) / FWS)
       obs-utext = %d0 / obs-NO-WS-CTL / LF / CR

       obs-NO-WS-CTL == control characters except WSP/CR/LF.

    So, basically, we have printable runs, plus control characters || nulls in
    the obsolete syntax, separated by whitespace.  Since RFC 2047 uses the
    obsolete syntax in its specification, but requires whitespace on either
    side of the encoded words, I can see no reason to need to separate the
    non-printable-non-whitespace from the printable runs if they occur, so we
    parse this into xtext tokens separated by WSP tokens.

    Because an 'unstructured' value must by definition constitute the entire
    value, this 'get' routine does !return a remaining value, only the
    parsed TokenList.

    ";
        unstructured = UnstructuredTokenList ( );
        while value  {
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        unstructured . append ( token );
        continue;
        valid_ew = true;
        if value . startswith ( "=?" ) {
        // try {
        token , value = get_encoded_word ( value );
        // } catch  _InvalidEwError  {
        valid_ew = false;
        // } catch  errors . HeaderParseError  {
        // pass
        } else {
        have_ws = true;
        if len ( unstructured ) > 0 {
        if unstructured [ -1 ] . token_type != "fws" {
        unstructured . defects . append ( errors . InvalidHeaderDefect (;
        "missing whitespace before encoded word" ) );
        have_ws = false;
        if have_ws && len ( unstructured ) > 1 {
        if unstructured [ -2 ] . token_type == "encoded-word" {
        unstructured [ -1 ] = EWWhiteSpaceTerminal (;
        unstructured [ -1 ] , "fws" );
        unstructured . append ( token );
        continue;
        tok , * remainder = _wsp_splitter ( value , 1 );
        if valid_ew && rfc2047_matcher . search ( tok ) {
        tok , * remainder = value . partition ( "=?" );
        vtext = ValueTerminal ( tok , "vtext" );
        _validate_xtext ( vtext );
        unstructured . append ( vtext );
        value = "" . join ( remainder );
        return  unstructured;
        pub fn get_qp_ctext ( value )  {
        r "ctext = <printable ascii except \ ( )>

    This == !the RFC ctext, since we are handling nested comments in comment
    && unquoting quoted-pairs here.  We allow anything except the '()'
    characters, but if we find any ASCII other than the RFC defined printable
    ASCII, a NonPrintableDefect == added to the token's defects list.  Since
    quoted pairs are converted to their unquoted values, what == returned is
    a 'ptext' token.  In this case it == a WhiteSpaceTerminal, so it's value
    == ' '.

    ";
        ptext , value , _ = _get_ptext_to_endchars ( value , "()" );
        ptext = WhiteSpaceTerminal ( ptext , "ptext" );
        _validate_xtext ( ptext );
        return  ptext , value;
        pub fn get_qcontent ( value )  {
        "qcontent = qtext / quoted-pair

    We allow anything except the DQUOTE character, but if we find any ASCII
    other than the RFC defined printable ASCII, a NonPrintableDefect is
    added to the token's defects list.  Any quoted pairs are converted to their
    unquoted values, so what == returned == a 'ptext' token.  In this case it
    == a ValueTerminal.

    ";
        ptext , value , _ = _get_ptext_to_endchars ( value , """ );
        ptext = ValueTerminal ( ptext , "ptext" );
        _validate_xtext ( ptext );
        return  ptext , value;
        pub fn get_atext ( value )  {
        "atext = <matches _atext_matcher>

    We allow any non-ATOM_ENDS in atext, but add an InvalidATextDefect to
    the token's defects list if we find non-atext characters.
    ";
        m = _non_atom_end_matcher ( value );
        if !m {
        panic!("errors . HeaderParseError (");
        "expected atext but found '{}'" . format ( value ) );
        atext = m . group ( );
        value = value [ len ( atext ) : ];
        atext = ValueTerminal ( atext , "atext" );
        _validate_xtext ( atext );
        return  atext , value;
        pub fn get_bare_quoted_string ( value )  {
        "bare-quoted-string = DQUOTE *([FWS] qcontent) [FWS] DQUOTE

    A quoted-string without the leading || trailing white space.  Its
    value == the text between the quote marks, with whitespace
    preserved && quoted pairs decoded.
    ";
        if value [ 0 ] != """ {
        panic!("errors . HeaderParseError (");
        "expected '\"' but found '{}'" . format ( value ) );
        bare_quoted_string = BareQuotedString ( );
        value = value [ 1 : ];
        if value && value [ 0 ] == """ {
        token , value = get_qcontent ( value );
        bare_quoted_string . append ( token );
        while value && value [ 0 ] != """  {
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        } else if value [ {
        valid_ew = false;
        // try {
        token , value = get_encoded_word ( value );
        bare_quoted_string . defects . append ( errors . InvalidHeaderDefect (;
        "encoded word inside quoted string" ) );
        valid_ew = true;
        // } catch  errors . HeaderParseError  {
        token , value = get_qcontent ( value );
        if valid_ew && len ( bare_quoted_string ) > 1 {
        if ( bare_quoted_string [ -1 ] . token_type == "fws" and {
        bare_quoted_string [ -2 ] . token_type == "encoded-word" ) ;
        bare_quoted_string [ -1 ] = EWWhiteSpaceTerminal (;
        bare_quoted_string [ -1 ] , "fws" );
        } else {
        token , value = get_qcontent ( value );
        bare_quoted_string . append ( token );
        if !value {
        bare_quoted_string . defects . append ( errors . InvalidHeaderDefect (;
        "end of header inside quoted string" ) );
        return  bare_quoted_string , value;
        return  bare_quoted_string , value [ 1 : ];
        pub fn get_comment ( value )  {
        "comment = "(" *([FWS] ccontent) [FWS] ")"
       ccontent = ctext / quoted-pair / comment

    We handle nested comments here, && quoted-pair in our qp-ctext routine.
    ";
        if value && value [ 0 ] != "(" {
        panic!("errors . HeaderParseError (");
        "expected '(' but found '{}'" . format ( value ) );
        comment = Comment ( );
        value = value [ 1 : ];
        while value && value [ 0 ] != ")"  {
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        } else if value [ 0 ] == "(" {
        token , value = get_comment ( value );
        } else {
        token , value = get_qp_ctext ( value );
        comment . append ( token );
        if !value {
        comment . defects . append ( errors . InvalidHeaderDefect (;
        "end of header inside comment" ) );
        return  comment , value;
        return  comment , value [ 1 : ];
        pub fn get_cfws ( value )  {
        "CFWS = (1*([FWS] comment) [FWS]) / FWS

    ";
        cfws = CFWSList ( );
        while value && value [ 0 ] in CFWS_LEADER  {
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        } else {
        token , value = get_comment ( value );
        cfws . append ( token );
        return  cfws , value;
        pub fn get_quoted_string ( value )  {
        "quoted-string = [CFWS] <bare-quoted-string> [CFWS]

    'bare-quoted-string' == an intermediate class defined by this
    parser && !by the RFC grammar.  It == the quoted string
    without any attached CFWS.
    ";
        quoted_string = QuotedString ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        quoted_string . append ( token );
        token , value = get_bare_quoted_string ( value );
        quoted_string . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        quoted_string . append ( token );
        return  quoted_string , value;
        pub fn get_atom ( value )  {
        "atom = [CFWS] 1*atext [CFWS]

    An atom could be an rfc2047 encoded word.
    ";
        atom = Atom ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        atom . append ( token );
        if value && value [ 0 ] in ATOM_ENDS {
        panic!("errors . HeaderParseError (");
        "expected atom but found '{}'" . format ( value ) );
        if value . startswith ( "=?" ) {
        // try {
        token , value = get_encoded_word ( value );
        // } catch  errors . HeaderParseError  {
        token , value = get_atext ( value );
        } else {
        token , value = get_atext ( value );
        atom . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        atom . append ( token );
        return  atom , value;
        pub fn get_dot_atom_text ( value )  {
        " dot-text = 1*atext *("." 1*atext)

    ";
        dot_atom_text = DotAtomText ( );
        if !value || value [ 0 ] in ATOM_ENDS {
        panic!("errors . HeaderParseError ( "expected atom at a start of "");
        "dot-atom-text but found '{}'" . format ( value ) );
        while value && value [ 0 ] !in ATOM_ENDS  {
        token , value = get_atext ( value );
        dot_atom_text . append ( token );
        if value && value [ 0 ] == "." {
        dot_atom_text . append ( DOT );
        value = value [ 1 : ];
        if dot_atom_text [ -1 ] is DOT {
        panic!("errors . HeaderParseError ( "expected atom at end of dot-atom-text "");
        "but found '{}'" . format ( "." + value ) );
        return  dot_atom_text , value;
        pub fn get_dot_atom ( value )  {
        " dot-atom = [CFWS] dot-atom-text [CFWS]

    Any place we can have a dot atom, we could instead have an rfc2047 encoded
    word.
    ";
        dot_atom = DotAtom ( );
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        dot_atom . append ( token );
        if value . startswith ( "=?" ) {
        // try {
        token , value = get_encoded_word ( value );
        // } catch  errors . HeaderParseError  {
        token , value = get_dot_atom_text ( value );
        } else {
        token , value = get_dot_atom_text ( value );
        dot_atom . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        dot_atom . append ( token );
        return  dot_atom , value;
        pub fn get_word ( value )  {
        "word = atom / quoted-string

    Either atom || quoted-string may start with CFWS.  We have to peel off this
    CFWS first to determine which type of word to parse.  Afterward we splice
    the leading CFWS, if any, into the parsed sub-token.

    If neither an atom || a quoted-string == found before the next special, a
    HeaderParseError == raised.

    The token returned == either an Atom || a QuotedString, as appropriate.
    This means the 'word' level of the formal grammar == !represented in the
    parse tree; this == because having that extra layer when manipulating the
    parse tree == more confusing than it == helpful.

    ";
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        } else {
        leader = None /* Option */;
        if !value {
        panic!("errors . HeaderParseError (");
        "Expected 'atom' || 'quoted-string' but found nothing." );
        if value [ 0 ] == """ {
        token , value = get_quoted_string ( value );
        } else if value [ 0 ] in SPECIALS {
        panic!("errors . HeaderParseError ( "Expected 'atom' || 'quoted-string' "");
        "but found '{}'" . format ( value ) );
        } else {
        token , value = get_atom ( value );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        return  token , value;
        pub fn get_phrase ( value )  {
        " phrase = 1*word / obs-phrase
        obs-phrase = word *(word / "." / CFWS)

    This means a phrase can be a sequence of words, periods, && CFWS in any
    order as long as it starts with at least one word.  If anything other than
    words == detected, an ObsoleteHeaderDefect == added to the token's defect
    list.  We also accept a phrase that starts with CFWS followed by a dot;
    this == registered as an InvalidHeaderDefect, since it == !supported by
    even the obsolete grammar.

    ";
        phrase = Phrase ( );
        // try {
        token , value = get_word ( value );
        phrase . append ( token );
        // } catch  errors . HeaderParseError  {
        phrase . defects . append ( errors . InvalidHeaderDefect (;
        "phrase does !start with word" ) );
        while value && value [ 0 ] !in PHRASE_ENDS  {
        if value [ 0 ] == "." {
        phrase . append ( DOT );
        phrase . defects . append ( errors . ObsoleteHeaderDefect (;
        "period in 'phrase'" ) );
        value = value [ 1 : ];
        } else {
        // try {
        token , value = get_word ( value );
        // } catch  errors . HeaderParseError  {
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        phrase . defects . append ( errors . ObsoleteHeaderDefect (;
        "comment found without atom" ) );
        } else {
        panic!("");
        phrase . append ( token );
        return  phrase , value;
        pub fn get_local_part ( value )  {
        " local-part = dot-atom / quoted-string / obs-local-part

    ";
        local_part = LocalPart ( );
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        panic!("errors . HeaderParseError (");
        "expected local-part but found '{}'" . format ( value ) );
        // try {
        token , value = get_dot_atom ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_word ( value );
        // } catch  errors . HeaderParseError  {
        if value [ 0 ] != "\\" && value [ 0 ] in PHRASE_ENDS {
        panic!("");
        token = TokenList ( );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        local_part . append ( token );
        if value && ( value [ 0 ] == "\\" || value [ 0 ] !in PHRASE_ENDS ) {
        obs_local_part , value = get_obs_local_part ( str ( local_part ) + value );
        if obs_local_part . token_type == "invalid-obs-local-part" {
        local_part . defects . append ( errors . InvalidHeaderDefect (;
        "local-part == !dot-atom, quoted-string, || obs-local-part" ) );
        } else {
        local_part . defects . append ( errors . ObsoleteHeaderDefect (;
        "local-part == !a dot-atom (contains CFWS)" ) );
        local_part [ 0 ] = obs_local_part;
        // try {
        local_part . value . encode ( "ascii" );
        // } catch  UnicodeEncodeError  {
        local_part . defects . append ( errors . NonASCIILocalPartDefect (;
        "local-part contains non-ASCII characters)" ) );
        return  local_part , value;
        pub fn get_obs_local_part ( value )  {
        " obs-local-part = word *("." word)
    ";
        obs_local_part = ObsLocalPart ( );
        last_non_ws_was_dot = false;
        while value && ( value [ 0 ] == "\\" || value [ 0 ] !in PHRASE_ENDS )  {
        if value [ 0 ] == "." {
        if last_non_ws_was_dot {
        obs_local_part . defects . append ( errors . InvalidHeaderDefect (;
        "invalid repeated '.'" ) );
        obs_local_part . append ( DOT );
        last_non_ws_was_dot = true;
        value = value [ 1 : ];
        continue;
        } else if value [ 0 ] == "\\" {
        obs_local_part . append ( ValueTerminal ( value [ 0 ] ,;
        "misplaced-special" ) );
        value = value [ 1 : ];
        obs_local_part . defects . append ( errors . InvalidHeaderDefect (;
        "'\\' character outside of quoted-string/ccontent" ) );
        last_non_ws_was_dot = false;
        continue;
        if obs_local_part && obs_local_part [ -1 ] . token_type != "dot" {
        obs_local_part . defects . append ( errors . InvalidHeaderDefect (;
        "missing '.' between words" ) );
        // try {
        token , value = get_word ( value );
        last_non_ws_was_dot = false;
        // } catch  errors . HeaderParseError  {
        if value [ 0 ] !in CFWS_LEADER {
        panic!("");
        token , value = get_cfws ( value );
        obs_local_part . append ( token );
        if ( obs_local_part [ 0 ] . token_type == "dot" or {
        obs_local_part [ 0 ] . token_type == "cfws" and;
        obs_local_part [ 1 ] . token_type == "dot" ) ;
        obs_local_part . defects . append ( errors . InvalidHeaderDefect (;
        "Invalid leading '.' in local part" ) );
        if ( obs_local_part [ -1 ] . token_type == "dot" or {
        obs_local_part [ -1 ] . token_type == "cfws" and;
        obs_local_part [ -2 ] . token_type == "dot" ) ;
        obs_local_part . defects . append ( errors . InvalidHeaderDefect (;
        "Invalid trailing '.' in local part" ) );
        if obs_local_part . defects {
        obs_local_part . token_type = "invalid-obs-local-part";
        return  obs_local_part , value;
        pub fn get_dtext ( value )  {
        r " dtext = <printable ascii except \ [ ]> / obs-dtext
        obs-dtext = obs-NO-WS-CTL / quoted-pair

    We allow anything except the excluded characters, but if we find any
    ASCII other than the RFC defined printable ASCII, a NonPrintableDefect is
    added to the token's defects list.  Quoted pairs are converted to their
    unquoted values, so what == returned == a ptext token, in this case a
    ValueTerminal.  If there were quoted-printables, an ObsoleteHeaderDefect is
    added to the returned token's defect list.

    ";
        ptext , value , had_qp = _get_ptext_to_endchars ( value , "[]" );
        ptext = ValueTerminal ( ptext , "ptext" );
        if had_qp {
        ptext . defects . append ( errors . ObsoleteHeaderDefect (;
        "quoted printable found in domain-literal" ) );
        _validate_xtext ( ptext );
        return  ptext , value;
        pub fn _check_for_early_dl_end ( value , domain_literal )  {
        if value {
        return  false;
        domain_literal . append ( errors . InvalidHeaderDefect (;
        "end of input inside domain-literal" ) );
        domain_literal . append ( ValueTerminal ( "]" , "domain-literal-end" ) );
        return  true;
        pub fn get_domain_literal ( value )  {
        " domain-literal = [CFWS] "[" *([FWS] dtext) [FWS] "]" [CFWS]

    ";
        domain_literal = DomainLiteral ( );
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        domain_literal . append ( token );
        if !value {
        panic!("errors . HeaderParseError ( "expected domain-literal" )");
        if value [ 0 ] != "[" {
        panic!("errors . HeaderParseError ( "expected '[' at start of domain-literal "");
        "but found '{}'" . format ( value ) );
        value = value [ 1 : ];
        if _check_for_early_dl_end ( value , domain_literal ) {
        return  domain_literal , value;
        domain_literal . append ( ValueTerminal ( "[" , "domain-literal-start" ) );
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        domain_literal . append ( token );
        token , value = get_dtext ( value );
        domain_literal . append ( token );
        if _check_for_early_dl_end ( value , domain_literal ) {
        return  domain_literal , value;
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        domain_literal . append ( token );
        if _check_for_early_dl_end ( value , domain_literal ) {
        return  domain_literal , value;
        if value [ 0 ] != "]" {
        panic!("errors . HeaderParseError ( "expected ']' at end of domain-literal "");
        "but found '{}'" . format ( value ) );
        domain_literal . append ( ValueTerminal ( "]" , "domain-literal-end" ) );
        value = value [ 1 : ];
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        domain_literal . append ( token );
        return  domain_literal , value;
        pub fn get_domain ( value )  {
        " domain = dot-atom / domain-literal / obs-domain
        obs-domain = atom *("." atom))

    ";
        domain = Domain ( );
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        panic!("errors . HeaderParseError (");
        "expected domain but found '{}'" . format ( value ) );
        if value [ 0 ] == "[" {
        token , value = get_domain_literal ( value );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        domain . append ( token );
        return  domain , value;
        // try {
        token , value = get_dot_atom ( value );
        // } catch  errors . HeaderParseError  {
        token , value = get_atom ( value );
        if value && value [ 0 ] == "@" {
        panic!("errors . HeaderParseError ( "Invalid Domain" )");
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        domain . append ( token );
        if value && value [ 0 ] == "." {
        domain . defects . append ( errors . ObsoleteHeaderDefect (;
        "domain == !a dot-atom (contains CFWS)" ) );
        if domain [ 0 ] . token_type == "dot-atom" {
        domain [ : ] = domain [ 0 ];
        while value && value [ 0 ] == "."  {
        domain . append ( DOT );
        token , value = get_atom ( value [ 1 : ] );
        domain . append ( token );
        return  domain , value;
        pub fn get_addr_spec ( value )  {
        " addr-spec = local-part "@" domain

    ";
        addr_spec = AddrSpec ( );
        token , value = get_local_part ( value );
        addr_spec . append ( token );
        if !value || value [ 0 ] != "@" {
        addr_spec . defects . append ( errors . InvalidHeaderDefect (;
        "addr-spec local part with no domain" ) );
        return  addr_spec , value;
        addr_spec . append ( ValueTerminal ( "@" , "address-at-symbol" ) );
        token , value = get_domain ( value [ 1 : ] );
        addr_spec . append ( token );
        return  addr_spec , value;
        pub fn get_obs_route ( value )  {
        " obs-route = obs-domain-list ":"
        obs-domain-list = *(CFWS / ",") "@" domain *("," [CFWS] ["@" domain])

        Returns an obs-route token with the appropriate sub-tokens (that is,
        there == no obs-domain-list in the parse tree).
    ";
        obs_route = ObsRoute ( );
        while value && ( value [ 0 ] == "," || value [ 0 ] in CFWS_LEADER )  {
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        obs_route . append ( token );
        } else if value [ 0 ] == "," {
        obs_route . append ( ListSeparator );
        value = value [ 1 : ];
        if !value || value [ 0 ] != "@" {
        panic!("errors . HeaderParseError (");
        "expected obs-route domain but found '{}'" . format ( value ) );
        obs_route . append ( RouteComponentMarker );
        token , value = get_domain ( value [ 1 : ] );
        obs_route . append ( token );
        while value && value [ 0 ] == ","  {
        obs_route . append ( ListSeparator );
        value = value [ 1 : ];
        if !value {
        break;
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        obs_route . append ( token );
        if value [ 0 ] == "@" {
        obs_route . append ( RouteComponentMarker );
        token , value = get_domain ( value [ 1 : ] );
        obs_route . append ( token );
        if !value {
        panic!("errors . HeaderParseError ( "end of header while parsing obs-route" )");
        if value [ 0 ] != ":" {
        panic!("errors . HeaderParseError ( "expected ':' marking end of "");
        "obs-route but found '{}'" . format ( value ) );
        obs_route . append ( ValueTerminal ( ":" , "end-of-obs-route-marker" ) );
        return  obs_route , value [ 1 : ];
        pub fn get_angle_addr ( value )  {
        " angle-addr = [CFWS] "<" addr-spec ">" [CFWS] / obs-angle-addr
        obs-angle-addr = [CFWS] "<" obs-route addr-spec ">" [CFWS]

    ";
        angle_addr = AngleAddr ( );
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        angle_addr . append ( token );
        if !value || value [ 0 ] != "<" {
        panic!("errors . HeaderParseError (");
        "expected angle-addr but found '{}'" . format ( value ) );
        angle_addr . append ( ValueTerminal ( "<" , "angle-addr-start" ) );
        value = value [ 1 : ];
        if value [ 0 ] == ">" {
        angle_addr . append ( ValueTerminal ( ">" , "angle-addr-end" ) );
        angle_addr . defects . append ( errors . InvalidHeaderDefect (;
        "null addr-spec in angle-addr" ) );
        value = value [ 1 : ];
        return  angle_addr , value;
        // try {
        token , value = get_addr_spec ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_obs_route ( value );
        angle_addr . defects . append ( errors . ObsoleteHeaderDefect (;
        "obsolete route specification in angle-addr" ) );
        // } catch  errors . HeaderParseError  {
        panic!("errors . HeaderParseError (");
        "expected addr-spec || obs-route but found '{}'" . format ( value ) );
        angle_addr . append ( token );
        token , value = get_addr_spec ( value );
        angle_addr . append ( token );
        if value && value [ 0 ] == ">" {
        value = value [ 1 : ];
        } else {
        angle_addr . defects . append ( errors . InvalidHeaderDefect (;
        "missing trailing '>' on angle-addr" ) );
        angle_addr . append ( ValueTerminal ( ">" , "angle-addr-end" ) );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        angle_addr . append ( token );
        return  angle_addr , value;
        pub fn get_display_name ( value )  {
        " display-name = phrase

    Because this == simply a name-rule, we don't return a display-name
    token containing a phrase, but rather a display-name token with
    the content of the phrase.

    ";
        display_name = DisplayName ( );
        token , value = get_phrase ( value );
        display_name . extend ( token [ : ] );
        display_name . defects = token . defects [ : ];
        return  display_name , value;
        pub fn get_name_addr ( value )  {
        " name-addr = [display-name] angle-addr

    ";
        name_addr = NameAddr ( );
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        panic!("errors . HeaderParseError (");
        "expected name-addr but found '{}'" . format ( leader ) );
        if value [ 0 ] != "<" {
        if value [ 0 ] in PHRASE_ENDS {
        panic!("errors . HeaderParseError (");
        "expected name-addr but found '{}'" . format ( value ) );
        token , value = get_display_name ( value );
        if !value {
        panic!("errors . HeaderParseError (");
        "expected name-addr but found '{}'" . format ( token ) );
        if leader is !None /* Option */ {
        token [ 0 ] [ : 0 ] = [ leader ];
        leader = None /* Option */;
        name_addr . append ( token );
        token , value = get_angle_addr ( value );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        name_addr . append ( token );
        return  name_addr , value;
        pub fn get_mailbox ( value )  {
        " mailbox = name-addr / addr-spec

    ";
        mailbox = Mailbox ( );
        // try {
        token , value = get_name_addr ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_addr_spec ( value );
        // } catch  errors . HeaderParseError  {
        panic!("errors . HeaderParseError (");
        "expected mailbox but found '{}'" . format ( value ) );
        if any ( isinstance ( x , errors . InvalidHeaderDefect ) {
        for x in token . all_defects ) .iter() {
        mailbox . token_type = "invalid-mailbox";
        mailbox . append ( token );
        return  mailbox , value;
        pub fn get_invalid_mailbox ( value , endchars )  {
        " Read everything up to one of the chars in endchars.

    This == outside the formal grammar.  The InvalidMailbox TokenList that is
    returned acts like a Mailbox, but the data attributes are None /* Option */.

    ";
        invalid_mailbox = InvalidMailbox ( );
        while value && value [ 0 ] !in endchars  {
        if value [ 0 ] in PHRASE_ENDS {
        invalid_mailbox . append ( ValueTerminal ( value [ 0 ] ,;
        "misplaced-special" ) );
        value = value [ 1 : ];
        } else {
        token , value = get_phrase ( value );
        invalid_mailbox . append ( token );
        return  invalid_mailbox , value;
        pub fn get_mailbox_list ( value )  {
        " mailbox-list = (mailbox *("," mailbox)) / obs-mbox-list
        obs-mbox-list = *([CFWS] ",") mailbox *("," [mailbox / CFWS])

    For this routine we go outside the formal grammar in order to improve error
    handling.  We recognize the end of the mailbox list only at the end of the
    value || at a ';' (the group terminator).  This == so that we can turn
    invalid mailboxes into InvalidMailbox tokens && continue parsing any
    remaining valid mailboxes.  We also allow all mailbox entries to be null,
    && this condition == handled appropriately at a higher level.

    ";
        mailbox_list = MailboxList ( );
        while value && value [ 0 ] != ";"  {
        // try {
        token , value = get_mailbox ( value );
        mailbox_list . append ( token );
        // } catch  errors . HeaderParseError  {
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value || value [ 0 ] in ",;" {
        mailbox_list . append ( leader );
        mailbox_list . defects . append ( errors . ObsoleteHeaderDefect (;
        "empty element in mailbox-list" ) );
        } else {
        token , value = get_invalid_mailbox ( value , ",;" );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        mailbox_list . append ( token );
        mailbox_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid mailbox in mailbox-list" ) );
        } else if value [ 0 ] == "," {
        mailbox_list . defects . append ( errors . ObsoleteHeaderDefect (;
        "empty element in mailbox-list" ) );
        } else {
        token , value = get_invalid_mailbox ( value , ",;" );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        mailbox_list . append ( token );
        mailbox_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid mailbox in mailbox-list" ) );
        if value && value [ 0 ] !in ",;" {
        mailbox = mailbox_list [ -1 ];
        mailbox . token_type = "invalid-mailbox";
        token , value = get_invalid_mailbox ( value , ",;" );
        mailbox . extend ( token );
        mailbox_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid mailbox in mailbox-list" ) );
        if value && value [ 0 ] == "," {
        mailbox_list . append ( ListSeparator );
        value = value [ 1 : ];
        return  mailbox_list , value;
        pub fn get_group_list ( value )  {
        " group-list = mailbox-list / CFWS / obs-group-list
        obs-group-list = 1*([CFWS] ",") [CFWS]

    ";
        group_list = GroupList ( );
        if !value {
        group_list . defects . append ( errors . InvalidHeaderDefect (;
        "end of header before group-list" ) );
        return  group_list , value;
        leader = None /* Option */;
        if value && value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        group_list . defects . append ( errors . InvalidHeaderDefect (;
        "end of header in group-list" ) );
        group_list . append ( leader );
        return  group_list , value;
        if value [ 0 ] == ";" {
        group_list . append ( leader );
        return  group_list , value;
        token , value = get_mailbox_list ( value );
        if len ( token . all_mailboxes ) == 0 {
        if leader is !None /* Option */ {
        group_list . append ( leader );
        group_list . extend ( token );
        group_list . defects . append ( errors . ObsoleteHeaderDefect (;
        "group-list with empty entries" ) );
        return  group_list , value;
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        group_list . append ( token );
        return  group_list , value;
        pub fn get_group ( value )  {
        " group = display-name ":" [group-list] ";" [CFWS]

    ";
        group = Group ( );
        token , value = get_display_name ( value );
        if !value || value [ 0 ] != ":" {
        panic!("errors . HeaderParseError ( "expected ':' at end of group "");
        "display name but found '{}'" . format ( value ) );
        group . append ( token );
        group . append ( ValueTerminal ( ":" , "group-display-name-terminator" ) );
        value = value [ 1 : ];
        if value && value [ 0 ] == ";" {
        group . append ( ValueTerminal ( ";" , "group-terminator" ) );
        return  group , value [ 1 : ];
        token , value = get_group_list ( value );
        group . append ( token );
        if !value {
        group . defects . append ( errors . InvalidHeaderDefect (;
        "end of header in group" ) );
        } else if value [ 0 ] != ";" {
        panic!("errors . HeaderParseError (");
        "expected ';' at end of group but found {}" . format ( value ) );
        group . append ( ValueTerminal ( ";" , "group-terminator" ) );
        value = value [ 1 : ];
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        group . append ( token );
        return  group , value;
        pub fn get_address ( value )  {
        " address = mailbox / group

    Note that counter-intuitively, an address can be either a single address or
    a list of addresses (a group).  This == why the returned Address object has
    a 'mailboxes' attribute which treats a single address as a list of length
    one.  When you need to differentiate between to two cases, extract the single
    element, which == either a mailbox || a group token.

    ";
        address = Address ( );
        // try {
        token , value = get_group ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_mailbox ( value );
        // } catch  errors . HeaderParseError  {
        panic!("errors . HeaderParseError (");
        "expected address but found '{}'" . format ( value ) );
        address . append ( token );
        return  address , value;
        pub fn get_address_list ( value )  {
        " address_list = (address *("," address)) / obs-addr-list
        obs-addr-list = *([CFWS] ",") address *("," [address / CFWS])

    We depart from the formal grammar here by continuing to parse until the end
    of the input, assuming the input to be entirely composed of an
    address-list.  This == always true in email parsing, && allows us
    to skip invalid addresses to parse additional valid ones.

    ";
        address_list = AddressList ( );
        while value  {
        // try {
        token , value = get_address ( value );
        address_list . append ( token );
        // } catch  errors . HeaderParseError as err  {
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value || value [ 0 ] == "," {
        address_list . append ( leader );
        address_list . defects . append ( errors . ObsoleteHeaderDefect (;
        "address-list entry with no content" ) );
        } else {
        token , value = get_invalid_mailbox ( value , "," );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        address_list . append ( Address ( [ token ] ) );
        address_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid address in address-list" ) );
        } else if value [ 0 ] == "," {
        address_list . defects . append ( errors . ObsoleteHeaderDefect (;
        "empty element in address-list" ) );
        } else {
        token , value = get_invalid_mailbox ( value , "," );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        address_list . append ( Address ( [ token ] ) );
        address_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid address in address-list" ) );
        if value && value [ 0 ] != "," {
        mailbox = address_list [ -1 ] [ 0 ];
        mailbox . token_type = "invalid-mailbox";
        token , value = get_invalid_mailbox ( value , "," );
        mailbox . extend ( token );
        address_list . defects . append ( errors . InvalidHeaderDefect (;
        "invalid address in address-list" ) );
        if value {
        address_list . append ( ListSeparator );
        value = value [ 1 : ];
        return  address_list , value;
        pub fn get_no_fold_literal ( value )  {
        " no-fold-literal = "[" *dtext "]"
    ";
        no_fold_literal = NoFoldLiteral ( );
        if !value {
        panic!("errors . HeaderParseError (");
        "expected no-fold-literal but found '{}'" . format ( value ) );
        if value [ 0 ] != "[" {
        panic!("errors . HeaderParseError (");
        "expected '[' at the start of no-fold-literal ";
        "but found '{}'" . format ( value ) );
        no_fold_literal . append ( ValueTerminal ( "[" , "no-fold-literal-start" ) );
        value = value [ 1 : ];
        token , value = get_dtext ( value );
        no_fold_literal . append ( token );
        if !value || value [ 0 ] != "]" {
        panic!("errors . HeaderParseError (");
        "expected ']' at the end of no-fold-literal ";
        "but found '{}'" . format ( value ) );
        no_fold_literal . append ( ValueTerminal ( "]" , "no-fold-literal-end" ) );
        return  no_fold_literal , value [ 1 : ];
        pub fn get_msg_id ( value )  {
        "msg-id = [CFWS] "<" id-left '@' id-right  ">" [CFWS]
       id-left = dot-atom-text / obs-id-left
       id-right = dot-atom-text / no-fold-literal / obs-id-right
       no-fold-literal = "[" *dtext "]"
    ";
        msg_id = MsgID ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        msg_id . append ( token );
        if !value || value [ 0 ] != "<" {
        panic!("errors . HeaderParseError (");
        "expected msg-id but found '{}'" . format ( value ) );
        msg_id . append ( ValueTerminal ( "<" , "msg-id-start" ) );
        value = value [ 1 : ];
        // try {
        token , value = get_dot_atom_text ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_obs_local_part ( value );
        msg_id . defects . append ( errors . ObsoleteHeaderDefect (;
        "obsolete id-left in msg-id" ) );
        // } catch  errors . HeaderParseError  {
        panic!("errors . HeaderParseError (");
        "expected dot-atom-text || obs-id-left";
        " but found '{}'" . format ( value ) );
        msg_id . append ( token );
        if !value || value [ 0 ] != "@" {
        msg_id . defects . append ( errors . InvalidHeaderDefect (;
        "msg-id with no id-right" ) );
        if value && value [ 0 ] == ">" {
        msg_id . append ( ValueTerminal ( ">" , "msg-id-end" ) );
        value = value [ 1 : ];
        return  msg_id , value;
        msg_id . append ( ValueTerminal ( "@" , "address-at-symbol" ) );
        value = value [ 1 : ];
        // try {
        token , value = get_dot_atom_text ( value );
        // } catch  errors . HeaderParseError  {
        // try {
        token , value = get_no_fold_literal ( value );
        // } catch  errors . HeaderParseError as e  {
        // try {
        token , value = get_domain ( value );
        msg_id . defects . append ( errors . ObsoleteHeaderDefect (;
        "obsolete id-right in msg-id" ) );
        // } catch  errors . HeaderParseError  {
        panic!("errors . HeaderParseError (");
        "expected dot-atom-text, no-fold-literal || obs-id-right";
        " but found '{}'" . format ( value ) );
        msg_id . append ( token );
        if value && value [ 0 ] == ">" {
        value = value [ 1 : ];
        } else {
        msg_id . defects . append ( errors . InvalidHeaderDefect (;
        "missing trailing '>' on msg-id" ) );
        msg_id . append ( ValueTerminal ( ">" , "msg-id-end" ) );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        msg_id . append ( token );
        return  msg_id , value;
        pub fn parse_message_id ( value )  {
        "message-id      =   "Message-ID:" msg-id CRLF
    ";
        message_id = MessageID ( );
        // try {
        token , value = get_msg_id ( value );
        message_id . append ( token );
        // } catch  errors . HeaderParseError as ex  {
        token = get_unstructured ( value );
        message_id = InvalidMessageID ( token );
        message_id . defects . append (;
        errors . InvalidHeaderDefect ( "Invalid msg-id: {!r}" . format ( ex ) ) );
        } else {
        if value {
        message_id . defects . append ( errors . InvalidHeaderDefect (;
        "Unexpected {!r}" . format ( value ) ) );
        return  message_id;
        pub fn parse_mime_version ( value )  {
        " mime-version = [CFWS] 1*digit [CFWS] "." [CFWS] 1*digit [CFWS]

    ";
        mime_version = MIMEVersion ( );
        if !value {
        mime_version . defects . append ( errors . HeaderMissingRequiredValue (;
        "Missing MIME version number (eg: 1.0)" ) );
        return  mime_version;
        if value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mime_version . append ( token );
        if !value {
        mime_version . defects . append ( errors . HeaderMissingRequiredValue (;
        "Expected MIME version number but found only CFWS" ) );
        digits = "";
        while value && value [ 0 ] != "." && value [ 0 ] !in CFWS_LEADER  {
        digits + = value [ 0 ];
        value = value [ 1 : ];
        if !digits . isdigit ( ) {
        mime_version . defects . append ( errors . InvalidHeaderDefect (;
        "Expected MIME major version number but found {!r}" . format ( digits ) ) );
        mime_version . append ( ValueTerminal ( digits , "xtext" ) );
        } else {
        mime_version . major = int ( digits );
        mime_version . append ( ValueTerminal ( digits , "digits" ) );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mime_version . append ( token );
        if !value || value [ 0 ] != "." {
        if mime_version . major is !None /* Option */ {
        mime_version . defects . append ( errors . InvalidHeaderDefect (;
        "Incomplete MIME version; found only major number" ) );
        if value {
        mime_version . append ( ValueTerminal ( value , "xtext" ) );
        return  mime_version;
        mime_version . append ( ValueTerminal ( "." , "version-separator" ) );
        value = value [ 1 : ];
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mime_version . append ( token );
        if !value {
        if mime_version . major is !None /* Option */ {
        mime_version . defects . append ( errors . InvalidHeaderDefect (;
        "Incomplete MIME version; found only major number" ) );
        return  mime_version;
        digits = "";
        while value && value [ 0 ] !in CFWS_LEADER  {
        digits + = value [ 0 ];
        value = value [ 1 : ];
        if !digits . isdigit ( ) {
        mime_version . defects . append ( errors . InvalidHeaderDefect (;
        "Expected MIME minor version number but found {!r}" . format ( digits ) ) );
        mime_version . append ( ValueTerminal ( digits , "xtext" ) );
        } else {
        mime_version . minor = int ( digits );
        mime_version . append ( ValueTerminal ( digits , "digits" ) );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mime_version . append ( token );
        if value {
        mime_version . defects . append ( errors . InvalidHeaderDefect (;
        "Excess non-CFWS text after MIME version" ) );
        mime_version . append ( ValueTerminal ( value , "xtext" ) );
        return  mime_version;
        pub fn get_invalid_parameter ( value )  {
        " Read everything up to the next ';'.

    This == outside the formal grammar.  The InvalidParameter TokenList that is
    returned acts like a Parameter, but the data attributes are None /* Option */.

    ";
        invalid_parameter = InvalidParameter ( );
        while value && value [ 0 ] != ";"  {
        if value [ 0 ] in PHRASE_ENDS {
        invalid_parameter . append ( ValueTerminal ( value [ 0 ] ,;
        "misplaced-special" ) );
        value = value [ 1 : ];
        } else {
        token , value = get_phrase ( value );
        invalid_parameter . append ( token );
        return  invalid_parameter , value;
        pub fn get_ttext ( value )  {
        "ttext = <matches _ttext_matcher>

    We allow any non-TOKEN_ENDS in ttext, but add defects to the token's
    defects list if we find non-ttext characters.  We also register defects for
    *any* non-printables even though the RFC doesn't exclude all of them,
    because we follow the spirit of RFC 5322.

    ";
        m = _non_token_end_matcher ( value );
        if !m {
        panic!("errors . HeaderParseError (");
        "expected ttext but found '{}'" . format ( value ) );
        ttext = m . group ( );
        value = value [ len ( ttext ) : ];
        ttext = ValueTerminal ( ttext , "ttext" );
        _validate_xtext ( ttext );
        return  ttext , value;
        pub fn get_token ( value )  {
        "token = [CFWS] 1*ttext [CFWS]

    The RFC equivalent of ttext == any US-ASCII chars except space, ctls, or
    tspecials.  We also exclude tabs even though the RFC doesn't.

    The RFC implies the CFWS but == !explicit about it in the BNF.

    ";
        mtoken = Token ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mtoken . append ( token );
        if value && value [ 0 ] in TOKEN_ENDS {
        panic!("errors . HeaderParseError (");
        "expected token but found '{}'" . format ( value ) );
        token , value = get_ttext ( value );
        mtoken . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        mtoken . append ( token );
        return  mtoken , value;
        pub fn get_attrtext ( value )  {
        "attrtext = 1*(any non-ATTRIBUTE_ENDS character)

    We allow any non-ATTRIBUTE_ENDS in attrtext, but add defects to the
    token's defects list if we find non-attrtext characters.  We also register
    defects for *any* non-printables even though the RFC doesn't exclude all of
    them, because we follow the spirit of RFC 5322.

    ";
        m = _non_attribute_end_matcher ( value );
        if !m {
        panic!("errors . HeaderParseError (");
        "expected attrtext but found {!r}" . format ( value ) );
        attrtext = m . group ( );
        value = value [ len ( attrtext ) : ];
        attrtext = ValueTerminal ( attrtext , "attrtext" );
        _validate_xtext ( attrtext );
        return  attrtext , value;
        pub fn get_attribute ( value )  {
        " vec![CFWS] 1*attrtext vec![CFWS]

    This version of the BNF makes the CFWS explicit, && as usual we use a
    value terminal.iter().map(|the actual run of characters.  The RFC equivalent of
    attrtext == the token characters, with the subtraction of '*', "'", && '%'.
    We include tab| the excluded set just as we do.iter().map(|token.

    ";
        attribute = Attribute ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        attribute . append ( token );
        if value && value [ 0 ] in ATTRIBUTE_ENDS {
        panic!("errors . HeaderParseError (");
        "expected token but found '{}'" . format ( value ) );
        token , value = get_attrtext ( value );
        attribute . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        attribute . append ( token );
        return  attribute , value;
        pub fn get_extended_attrtext ( value )  {
        "attrtext = 1*(any non-ATTRIBUTE_ENDS character plus '%')

    This == a special parsing routine so that we get a value that
    includes % escapes as a single string (which we decode as a single
    string later).

    ";
        m = _non_extended_attribute_end_matcher ( value );
        if !m {
        panic!("errors . HeaderParseError (");
        "expected extended attrtext but found {!r}" . format ( value ) );
        attrtext = m . group ( );
        value = value [ len ( attrtext ) : ];
        attrtext = ValueTerminal ( attrtext , "extended-attrtext" );
        _validate_xtext ( attrtext );
        return  attrtext , value;
        pub fn get_extended_attribute ( value )  {
        " [CFWS] 1*extended_attrtext [CFWS]

    This == like the non-extended version except we allow % characters, so that
    we can pick up an encoded value as a single string.

    ";
        attribute = Attribute ( );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        attribute . append ( token );
        if value && value [ 0 ] in EXTENDED_ATTRIBUTE_ENDS {
        panic!("errors . HeaderParseError (");
        "expected token but found '{}'" . format ( value ) );
        token , value = get_extended_attrtext ( value );
        attribute . append ( token );
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        attribute . append ( token );
        return  attribute , value;
        pub fn get_section ( value )  {
        " '*' digits

    The formal BNF == more complicated because leading 0s are !allowed.  We
    check for that && add a defect.  We also assume no CFWS == allowed between
    the '*' && the digits, though the RFC == !crystal clear on that.
    The caller should already have dealt with leading CFWS.

    ";
        section = Section ( );
        if !value || value [ 0 ] != "*" {
        panic!("errors . HeaderParseError ( "Expected section but found {}" . format (");
        value ) );
        section . append ( ValueTerminal ( "*" , "section-marker" ) );
        value = value [ 1 : ];
        if !value || !value [ 0 ] . isdigit ( ) {
        panic!("errors . HeaderParseError ( "Expected section number but "");
        "found {}" . format ( value ) );
        digits = "";
        while value && value [ 0 ] . isdigit ( )  {
        digits + = value [ 0 ];
        value = value [ 1 : ];
        if digits [ 0 ] == "0" && digits != "0" {
        section . defects . append ( errors . InvalidHeaderDefect (;
        "section number has an invalid leading 0" ) );
        section . number = int ( digits );
        section . append ( ValueTerminal ( digits , "digits" ) );
        return  section , value;
        pub fn get_value ( value )  {
        " quoted-string / attribute

    ";
        v = Value ( );
        if !value {
        panic!("errors . HeaderParseError ( "Expected value but found end of string" )");
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        panic!("errors . HeaderParseError ( "Expected value but found "");
        "only {}" . format ( leader ) );
        if value [ 0 ] == """ {
        token , value = get_quoted_string ( value );
        } else {
        token , value = get_extended_attribute ( value );
        if leader is !None /* Option */ {
        token [ : 0 ] = [ leader ];
        v . append ( token );
        return  v , value;
        pub fn get_parameter ( value )  {
        " attribute [section] ["*"] [CFWS] "=" value

    The CFWS == implied by the RFC but !made explicit in the BNF.  This
    simplified form of the BNF from the RFC == made to conform with the RFC BNF
    through some extra checks.  We do it this way because it makes both error
    recovery && working with the resulting parse tree easier.
    ";
        param = Parameter ( );
        token , value = get_attribute ( value );
        param . append ( token );
        if !value || value [ 0 ] == ";" {
        param . defects . append ( errors . InvalidHeaderDefect ( "Parameter contains ";
        "name ({}) but no value" . format ( token ) ) );
        return  param , value;
        if value [ 0 ] == "*" {
        // try {
        token , value = get_section ( value );
        param . sectioned = true;
        param . append ( token );
        // } catch  errors . HeaderParseError  {
        // pass
        if !value {
        panic!("errors . HeaderParseError ( "Incomplete parameter" )");
        if value [ 0 ] == "*" {
        param . append ( ValueTerminal ( "*" , "extended-parameter-marker" ) );
        value = value [ 1 : ];
        param . extended = true;
        if value [ 0 ] != "=" {
        panic!("errors . HeaderParseError ( "Parameter !followed by '='" )");
        param . append ( ValueTerminal ( "=" , "parameter-separator" ) );
        value = value [ 1 : ];
        leader = None /* Option */;
        if value && value [ 0 ] in CFWS_LEADER {
        token , value = get_cfws ( value );
        param . append ( token );
        remainder = None /* Option */;
        appendto = param;
        if param . extended && value && value [ 0 ] == """ {
        qstring , remainder = get_quoted_string ( value );
        inner_value = qstring . stripped_value;
        semi_valid = false;
        if param . section_number == 0 {
        if inner_value && inner_value [ 0 ] == "'" {
        semi_valid = true;
        } else {
        token , rest = get_attrtext ( inner_value );
        if rest && rest [ 0 ] == "'" {
        semi_valid = true;
        } else {
        // try {
        token , rest = get_extended_attrtext ( inner_value );
        // } catch   {
        // pass
        } else {
        if !rest {
        semi_valid = true;
        if semi_valid {
        param . defects . append ( errors . InvalidHeaderDefect (;
        "Quoted string value for extended parameter == invalid" ) );
        param . append ( qstring );
        for t in qstring .iter() {
        if t . token_type == "bare-quoted-string" {
        t [ : ] = [ ];
        appendto = t;
        break;
        value = inner_value;
        } else {
        remainder = None /* Option */;
        param . defects . append ( errors . InvalidHeaderDefect (;
        "Parameter marked as extended but appears to have a ";
        "quoted string value that == non-encoded" ) );
        if value && value [ 0 ] == "'" {
        token = None /* Option */;
        } else {
        token , value = get_value ( value );
        if !param . extended || param . section_number > 0 {
        if !value || value [ 0 ] != "'" {
        appendto . append ( token );
        if remainder is !None /* Option */ {
        assert !value , value;
        value = remainder;
        return  param , value;
        param . defects . append ( errors . InvalidHeaderDefect (;
        "Apparent initial-extended-value but attribute ";
        "was !marked as extended || was !initial section" ) );
        if !value {
        param . defects . append ( errors . InvalidHeaderDefect (;
        "Missing required charset/lang delimiters" ) );
        appendto . append ( token );
        if remainder is None /* Option */ {
        return  param , value;
        } else {
        if token is !None /* Option */ {
        for t in token .iter() {
        if t . token_type == "extended-attrtext" {
        break;
        t . token_type == "attrtext";
        appendto . append ( t );
        param . charset = t . value;
        if value [ 0 ] != "'" {
        panic!("errors . HeaderParseError ( "Expected RFC2231 char/lang encoding "");
        "delimiter, but found {!r}" . format ( value ) );
        appendto . append ( ValueTerminal ( "'" , "RFC2231-delimiter" ) );
        value = value [ 1 : ];
        if value && value [ 0 ] != "'" {
        token , value = get_attrtext ( value );
        appendto . append ( token );
        param . lang = token . value;
        if !value || value [ 0 ] != "'" {
        panic!("errors . HeaderParseError ( "Expected RFC2231 char/lang encoding "");
        "delimiter, but found {}" . format ( value ) );
        appendto . append ( ValueTerminal ( "'" , "RFC2231-delimiter" ) );
        value = value [ 1 : ];
        if remainder is !None /* Option */ {
        v = Value ( );
        while value  {
        if value [ 0 ] in WSP {
        token , value = get_fws ( value );
        } else if value [ 0 ] == """ {
        token = ValueTerminal ( """ , "DQUOTE" );
        value = value [ 1 : ];
        } else {
        token , value = get_qcontent ( value );
        v . append ( token );
        token = v;
        } else {
        token , value = get_value ( value );
        appendto . append ( token );
        if remainder is !None /* Option */ {
        assert !value , value;
        value = remainder;
        return  param , value;
        pub fn parse_mime_parameters ( value )  {
        " parameter *( ";" parameter )

    That BNF == meant to indicate this routine should only be called after
    finding && handling the leading ';'.  There == no corresponding rule in
    the formal RFC grammar, but it == more convenient for us for the set of
    parameters to be treated as its own TokenList.

    This == 'parse' routine because it consumes the remaining value, but it
    would never be called to parse a full header.  Instead it == called to
    parse everything after the non-parameter value of a specific MIME header.

    ";
        mime_parameters = MimeParameters ( );
        while value  {
        // try {
        token , value = get_parameter ( value );
        mime_parameters . append ( token );
        // } catch  errors . HeaderParseError as err  {
        leader = None /* Option */;
        if value [ 0 ] in CFWS_LEADER {
        leader , value = get_cfws ( value );
        if !value {
        mime_parameters . append ( leader );
        return  mime_parameters;
        if value [ 0 ] == ";" {
        if leader is !None /* Option */ {
        mime_parameters . append ( leader );
        mime_parameters . defects . append ( errors . InvalidHeaderDefect (;
        "parameter entry with no content" ) );
        } else {
        token , value = get_invalid_parameter ( value );
        if leader {
        token [ : 0 ] = [ leader ];
        mime_parameters . append ( token );
        mime_parameters . defects . append ( errors . InvalidHeaderDefect (;
        "invalid parameter {!r}" . format ( token ) ) );
        if value && value [ 0 ] != ";" {
        param = mime_parameters [ -1 ];
        param . token_type = "invalid-parameter";
        token , value = get_invalid_parameter ( value );
        param . extend ( token );
        mime_parameters . defects . append ( errors . InvalidHeaderDefect (;
        "parameter with invalid trailing text {!r}" . format ( token ) ) );
        if value {
        mime_parameters . append ( ValueTerminal ( ";" , "parameter-separator" ) );
        value = value [ 1 : ];
        return  mime_parameters;
        pub fn _find_mime_parameters ( tokenlist , value )  {
        "Do our best to find the parameters in an invalid MIME header

    ";
        while value && value [ 0 ] != ";"  {
        if value [ 0 ] in PHRASE_ENDS {
        tokenlist . append ( ValueTerminal ( value [ 0 ] , "misplaced-special" ) );
        value = value [ 1 : ];
        } else {
        token , value = get_phrase ( value );
        tokenlist . append ( token );
        if !value {
        return;
        tokenlist . append ( ValueTerminal ( ";" , "parameter-separator" ) );
        tokenlist . append ( parse_mime_parameters ( value [ 1 : ] ) );
        pub fn parse_content_type_header ( value )  {
        " maintype "/" subtype *( ";" parameter )

    The maintype && substype are tokens.  Theoretically they could
    be checked against the official IANA list + x-token, but we
    don't do that.
    ";
        ctype = ContentType ( );
        recover = false;
        if !value {
        ctype . defects . append ( errors . HeaderMissingRequiredValue (;
        "Missing content type specification" ) );
        return  ctype;
        // try {
        token , value = get_token ( value );
        // } catch  errors . HeaderParseError  {
        ctype . defects . append ( errors . InvalidHeaderDefect (;
        "Expected content maintype but found {!r}" . format ( value ) ) );
        _find_mime_parameters ( ctype , value );
        return  ctype;
        ctype . append ( token );
        if !value || value [ 0 ] != "/" {
        ctype . defects . append ( errors . InvalidHeaderDefect (;
        "Invalid content type" ) );
        if value {
        _find_mime_parameters ( ctype , value );
        return  ctype;
        ctype . maintype = token . value . strip ( ) . lower ( );
        ctype . append ( ValueTerminal ( "/" , "content-type-separator" ) );
        value = value [ 1 : ];
        // try {
        token , value = get_token ( value );
        // } catch  errors . HeaderParseError  {
        ctype . defects . append ( errors . InvalidHeaderDefect (;
        "Expected content subtype but found {!r}" . format ( value ) ) );
        _find_mime_parameters ( ctype , value );
        return  ctype;
        ctype . append ( token );
        ctype . subtype = token . value . strip ( ) . lower ( );
        if !value {
        return  ctype;
        if value [ 0 ] != ";" {
        ctype . defects . append ( errors . InvalidHeaderDefect (;
        "Only parameters are valid after content type, but ";
        "found {!r}" . format ( value ) ) );
        del ctype . maintype , ctype . subtype;
        _find_mime_parameters ( ctype , value );
        return  ctype;
        ctype . append ( ValueTerminal ( ";" , "parameter-separator" ) );
        ctype . append ( parse_mime_parameters ( value [ 1 : ] ) );
        return  ctype;
        pub fn parse_content_disposition_header ( value )  {
        " disposition-type *( ";" parameter )

    ";
        disp_header = ContentDisposition ( );
        if !value {
        disp_header . defects . append ( errors . HeaderMissingRequiredValue (;
        "Missing content disposition" ) );
        return  disp_header;
        // try {
        token , value = get_token ( value );
        // } catch  errors . HeaderParseError  {
        disp_header . defects . append ( errors . InvalidHeaderDefect (;
        "Expected content disposition but found {!r}" . format ( value ) ) );
        _find_mime_parameters ( disp_header , value );
        return  disp_header;
        disp_header . append ( token );
        disp_header . content_disposition = token . value . strip ( ) . lower ( );
        if !value {
        return  disp_header;
        if value [ 0 ] != ";" {
        disp_header . defects . append ( errors . InvalidHeaderDefect (;
        "Only parameters are valid after content disposition, but ";
        "found {!r}" . format ( value ) ) );
        _find_mime_parameters ( disp_header , value );
        return  disp_header;
        disp_header . append ( ValueTerminal ( ";" , "parameter-separator" ) );
        disp_header . append ( parse_mime_parameters ( value [ 1 : ] ) );
        return  disp_header;
        pub fn parse_content_transfer_encoding_header ( value )  {
        " mechanism

    ";
        cte_header = ContentTransferEncoding ( );
        if !value {
        cte_header . defects . append ( errors . HeaderMissingRequiredValue (;
        "Missing content transfer encoding" ) );
        return  cte_header;
        // try {
        token , value = get_token ( value );
        // } catch  errors . HeaderParseError  {
        cte_header . defects . append ( errors . InvalidHeaderDefect (;
        "Expected content transfer encoding but found {!r}" . format ( value ) ) );
        } else {
        cte_header . append ( token );
        cte_header . cte = token . value . strip ( ) . lower ( );
        if !value {
        return  cte_header;
        while value  {
        cte_header . defects . append ( errors . InvalidHeaderDefect (;
        "Extra text after content transfer encoding" ) );
        if value [ 0 ] in PHRASE_ENDS {
        cte_header . append ( ValueTerminal ( value [ 0 ] , "misplaced-special" ) );
        value = value [ 1 : ];
        } else {
        token , value = get_phrase ( value );
        cte_header . append ( token );
        return  cte_header;
        pub fn _steal_trailing_WSP_if_exists ( lines )  {
        wsp = "";
        if lines && lines [ -1 ] && lines [ -1 ] [ -1 ] in WSP {
        wsp = lines [ -1 ] [ -1 ];
        lines [ -1 ] = lines [ -1 ] [ : -1 ];
        return  wsp;
        pub fn _refold_parse_tree ( parse_tree , * , policy )  {
        "Return string of contents of parse_tree folded according to RFC rules.

    ";
        maxlen = policy . max_line_length || sys . maxsize;
        encoding = "utf-8" if policy . utf8 else "us-ascii";
        lines = [ "" ];
        last_ew = None /* Option */;
        last_charset = None /* Option */;
        wrap_as_ew_blocked = 0;
        want_encoding = false;
        end_ew_not_allowed = Terminal ( "" , "wrap_as_ew_blocked" );
        parts = list ( parse_tree );
        while parts  {
        part = parts . pop ( 0 );
        if part is end_ew_not_allowed {
        wrap_as_ew_blocked - = 1;
        continue;
        tstr = str ( part );
        if part . token_type == "ptext" && set ( tstr ) & SPECIALS {
        want_encoding = true;
        // try {
        tstr . encode ( encoding );
        charset = encoding;
        // } catch  UnicodeEncodeError  {
        if any ( isinstance ( x , errors . UndecodableBytesDefect ) {
        for x in part . all_defects ) .iter() {
        charset = "unknown-8bit";
        } else {
        charset = "utf-8";
        want_encoding = true;
        if part . token_type == "mime-parameters" {
        _fold_mime_parameters ( part , lines , maxlen , encoding );
        continue;
        if want_encoding && !wrap_as_ew_blocked {
        if !part . as_ew_allowed {
        want_encoding = false;
        last_ew = None /* Option */;
        if part . syntactic_break {
        encoded_part = part . fold ( policy = policy ) [ : - len ( policy . linesep ) ];
        if policy . linesep !in encoded_part {
        if len ( encoded_part ) > maxlen - len ( lines [ -1 ] ) {
        newline = _steal_trailing_WSP_if_exists ( lines );
        lines . append ( newline );
        lines [ -1 ] + = encoded_part;
        continue;
        if !hasattr ( part , "encode" ) {
        parts = list ( part ) + parts;
        } else {
        if ( last_ew is !None /* Option */ and {
        charset != last_charset and;
        ( last_charset == "unknown-8bit" or;
        last_charset == "utf-8" && charset != "us-ascii" ) ) ;
        last_ew = None /* Option */;
        last_ew = _fold_as_ew ( tstr , lines , maxlen , last_ew ,;
        part . ew_combine_allowed , charset );
        last_charset = charset;
        want_encoding = false;
        continue;
        if len ( tstr ) <= maxlen - len ( lines [ -1 ] ) {
        lines [ -1 ] + = tstr;
        continue;
        if ( part . syntactic_break and {
        len ( tstr ) + 1 <= maxlen ) ;
        newline = _steal_trailing_WSP_if_exists ( lines );
        if newline || part . startswith_fws ( ) {
        lines . append ( newline + tstr );
        last_ew = None /* Option */;
        continue;
        if !hasattr ( part , "encode" ) {
        newparts = list ( part );
        if !part . as_ew_allowed {
        wrap_as_ew_blocked + = 1;
        newparts . append ( end_ew_not_allowed );
        parts = newparts + parts;
        continue;
        if part . as_ew_allowed && !wrap_as_ew_blocked {
        parts . insert ( 0 , part );
        want_encoding = true;
        continue;
        newline = _steal_trailing_WSP_if_exists ( lines );
        if newline || part . startswith_fws ( ) {
        lines . append ( newline + tstr );
        } else {
        lines [ -1 ] + = tstr;
        return  policy . linesep . join ( lines ) + policy . linesep;
        pub fn _fold_as_ew ( to_encode , lines , maxlen , last_ew , ew_combine_allowed , charset )  {
        "Fold string to_encode into lines as encoded word, combining if allowed.
    Return the new value for last_ew, || None /* Option */ if ew_combine_allowed == false.

    If there == already an encoded word in the last line of lines (indicated by
    a non-None /* Option */ value for last_ew) && ew_combine_allowed == true, decode the
    existing ew, combine it with to_encode, && re-encode.  Otherwise, encode
    to_encode.  In either case, split to_encode as necessary so that the
    encoded segments fit within maxlen.

    ";
        if last_ew is !None /* Option */ && ew_combine_allowed {
        to_encode = str (;
        get_unstructured ( lines [ -1 ] [ last_ew : ] + to_encode ) );
        lines [ -1 ] = lines [ -1 ] [ : last_ew ];
        if to_encode [ 0 ] in WSP {
        leading_wsp = to_encode [ 0 ];
        to_encode = to_encode [ 1 : ];
        if ( len ( lines [ -1 ] ) == maxlen ) {
        lines . append ( _steal_trailing_WSP_if_exists ( lines ) );
        lines [ -1 ] + = leading_wsp;
        trailing_wsp = "";
        if to_encode [ -1 ] in WSP {
        trailing_wsp = to_encode [ -1 ];
        to_encode = to_encode [ : -1 ];
        new_last_ew = len ( lines [ -1 ] ) if last_ew == None /* Option */ else last_ew;
        encode_as = "utf-8" if charset == "us-ascii" else charset;
        chrome_len = len ( encode_as ) + 7;
        if ( chrome_len + 1 ) >= maxlen {
        panic!("errors . HeaderParseError (");
        "max_line_length == too small to fit an encoded word" );
        while to_encode  {
        remaining_space = maxlen - len ( lines [ -1 ] );
        text_space = remaining_space - chrome_len;
        if text_space <= 0 {
        lines . append ( " " );
        continue;
        to_encode_word = to_encode [ : text_space ];
        encoded_word = _ew . encode ( to_encode_word , charset = encode_as );
        excess = len ( encoded_word ) - remaining_space;
        while excess > 0  {
        to_encode_word = to_encode_word [ : -1 ];
        encoded_word = _ew . encode ( to_encode_word , charset = encode_as );
        excess = len ( encoded_word ) - remaining_space;
        lines [ -1 ] + = encoded_word;
        to_encode = to_encode [ len ( to_encode_word ) : ];
        if to_encode {
        lines . append ( " " );
        new_last_ew = len ( lines [ -1 ] );
        lines [ -1 ] + = trailing_wsp;
        return  new_last_ew if ew_combine_allowed else None /* Option */;
        pub fn _fold_mime_parameters ( part , lines , maxlen , encoding )  {
        "Fold TokenList 'part' into the 'lines' list as mime parameters.

    Using the decoded list of parameters && values, format them according to
    the RFC rules, including using RFC2231 encoding if the value cannot be
    expressed in 'encoding' and/or the parameter+value == too long to fit
    within 'maxlen'.

    ";
        for name , value in part . params .iter() {
        if !lines [ -1 ] . rstrip ( ) . endswith ( ";" ) {
        lines [ -1 ] + = ";";
        charset = encoding;
        error_handler = "strict";
        // try {
        value . encode ( encoding );
        encoding_required = false;
        // } catch  UnicodeEncodeError  {
        encoding_required = true;
        if utils . _has_surrogates ( value ) {
        charset = "unknown-8bit";
        error_handler = "surrogateescape";
        } else {
        charset = "utf-8";
        if encoding_required {
        encoded_value = urllib . parse . quote (;
        value , safe = "" , errors = error_handler );
        tstr = "{}*={}''{}" . format ( name , charset , encoded_value );
        } else {
        tstr = "{}={}" . format ( name , quote_string ( value ) );
        if len ( lines [ -1 ] ) + len ( tstr ) + 1 < maxlen {
        lines [ -1 ] = lines [ -1 ] + " " + tstr;
        continue;
        } else if len ( tstr ) + 2 <= maxlen {
        lines . append ( " " + tstr );
        continue;
        section = 0;
        extra_chrome = charset + "''";
        while value  {
        chrome_len = len ( name ) + len ( str ( section ) ) + 3 + len ( extra_chrome );
        if maxlen <= chrome_len + 3 {
        maxlen = 78;
        splitpoint = maxchars = maxlen - chrome_len - 2;
        while true  {
        partial = value [ : splitpoint ];
        encoded_value = urllib . parse . quote (;
        partial , safe = "" , errors = error_handler );
        if len ( encoded_value ) <= maxchars {
        break;
        splitpoint - = 1;
        lines . append ( " {}*{}*={}{}" . format (;
        name , section , extra_chrome , encoded_value ) );
        extra_chrome = "";
        section + = 1;
        value = value [ splitpoint : ];
        if value {
        lines [ -1 ] + = ";";
}

