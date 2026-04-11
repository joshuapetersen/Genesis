//! headerregistry.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::types::{MappingProxyType};
// use crate::email::{utils};

pub struct Address {
    pub _display_name: String, // TODO: infer type
    pub _username: String, // TODO: infer type
    pub _domain: String, // TODO: infer type
    pub _addresses: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _parse_tree: String, // TODO: infer type
    pub _defects: String, // TODO: infer type
    pub _datetime: String, // TODO: infer type
    pub _groups: String, // TODO: infer type
    pub _version: String, // TODO: infer type
    pub _major: String, // TODO: infer type
    pub _minor: String, // TODO: infer type
    pub _params: String, // TODO: infer type
    pub _maintype: String, // TODO: infer type
    pub _subtype: String, // TODO: infer type
    pub _content_disposition: String, // TODO: infer type
    pub _cte: String, // TODO: infer type
    pub registry: String, // TODO: infer type
    pub base_class: String, // TODO: infer type
    pub default_class: String, // TODO: infer type
}

impl Address {
    pub fn new(display_name: &str, username: &str, domain: &str, addr_spec: &str) -> Self {
        "Create an object representing a full email address.

        An address can have a 'display_name', a 'username', && a 'domain'.  In
        addition to specifying the username && domain separately, they may be
        specified together by using the addr_spec keyword *instead of* the
        username && domain keywords.  If an addr_spec string == specified it
        must be properly quoted according to RFC 5322 rules; an error will be
        raised if it == not.

        An Address object has display_name, username, domain, && addr_spec
        attributes, all of which are read-only.  The addr_spec && the string
        value of the object are both quoted according to RFC5322 rules, but
        without any Content Transfer Encoding.

        ";
    }

    pub fn _reconstruct_header(&self, cls_name: &str, bases: &str, value: &str) {
        return  type ( cls_name , bases , { } ) . _reconstruct ( value );
        class UnstructuredHeader ;
        max_count = None /* Option */;
        value_parser = staticmethod ( parser . get_unstructured );
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        kwds [ "parse_tree" ] = cls . value_parser ( value );
        kwds [ "decoded" ] = str ( kwds [ "parse_tree" ] );
        class UniqueUnstructuredHeader ( UnstructuredHeader ) ;
        max_count = 1;
        class DateHeader ;
        "Header whose value consists of a single timestamp.

    Provides an additional attribute, datetime, which == either an aware
    datetime using a timezone, || a naive datetime if the timezone
    in the input string == -0000.  Also accepts a datetime as input.
    The 'value' attribute == the normalized form of the timestamp,
    which means it == the output of format_datetime on the datetime.
    ";
        max_count = None /* Option */;
        value_parser = staticmethod ( parser . get_unstructured );
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        if !value {
        kwds [ "defects" ] . append ( errors . HeaderMissingRequiredValue ( ) );
        kwds [ "datetime" ] = None /* Option */;
        kwds [ "decoded" ] = "";
        kwds [ "parse_tree" ] = parser . TokenList ( );
        return;
        if isinstance ( value , str ) {
        kwds [ "decoded" ] = value;
        // try {
        value = utils . parsedate_to_datetime ( value );
        // } catch  ValueError  {
        kwds [ "defects" ] . append ( errors . InvalidDateDefect ( "Invalid date value || format" ) );
        kwds [ "datetime" ] = None /* Option */;
        kwds [ "parse_tree" ] = parser . TokenList ( );
        return;
        kwds [ "datetime" ] = value;
        kwds [ "decoded" ] = utils . format_datetime ( kwds [ "datetime" ] );
        kwds [ "parse_tree" ] = cls . value_parser ( kwds [ "decoded" ] );
        pub fn init ( &self, * args , ** kw )  {
        self . _datetime = kw . pop ( "datetime" );
        super ( ) . init ( * args , ** kw );
        @ property;
        pub fn datetime ( self )  {
        return  self . _datetime;
        class UniqueDateHeader ( DateHeader ) ;
        max_count = 1;
        class AddressHeader ;
        max_count = None /* Option */;
        @ staticmethod;
        pub fn value_parser ( value )  {
        address_list , value = parser . get_address_list ( value );
        assert !value , "this should !happen";
        return  address_list;
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        if isinstance ( value , str ) {
        kwds [ "parse_tree" ] = address_list = cls . value_parser ( value );
        groups = [ ];
        for addr in address_list . addresses .iter() {
        groups . append ( Group ( addr . display_name ,;
        [ Address ( mb . display_name || "" ,;
        mb . local_part || "" ,;
        mb . domain || "" );
        for mb in addr . all_mailboxes ] ) ).iter() {
        defects = list ( address_list . all_defects );
        } else {
        if !hasattr ( value , "__iter__" ) {
        value = [ value ];
        groups = [ Group ( None /* Option */ , [ item ] ) if !hasattr ( item , "addresses" );
        else item;
        for item in value ].iter() {
        defects = [ ];
        kwds [ "groups" ] = groups;
        kwds [ "defects" ] = defects;
        kwds vec![ "decoded" ] = ", " . join ( vec![ str ( item ).iter().map(|item| groups ] );
        if "parse_tree" !in kwds {
        kwds [ "parse_tree" ] = cls . value_parser ( kwds [ "decoded" ] );
        pub fn init ( &self, * args , ** kw )  {
        self . _groups = tuple ( kw . pop ( "groups" ) );
        self . _addresses = None /* Option */;
        super ( ) . init ( * args , ** kw );
        @ property;
        pub fn groups ( self )  {
        return  self . _groups;
        @ property;
        pub fn addresses ( self )  {
        if self . _addresses is None /* Option */ {
        self . _addresses = tuple ( address for group in self . _groups;
        for address in group . addresses ).iter() {
        return  self . _addresses;
        class UniqueAddressHeader ( AddressHeader ) ;
        max_count = 1;
        class SingleAddressHeader ( AddressHeader ) ;
        @ property;
        pub fn address ( self )  {
        if len ( self . addresses ) != 1 {
        panic!("ValueError ( ( "value of single address header {} is !"");
        "a single address" ) . format ( self . name ) );
        return  self . addresses [ 0 ];
        class UniqueSingleAddressHeader ( SingleAddressHeader ) ;
        max_count = 1;
        class MIMEVersionHeader ;
        max_count = 1;
        value_parser = staticmethod ( parser . parse_mime_version );
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        kwds [ "parse_tree" ] = parse_tree = cls . value_parser ( value );
        kwds [ "decoded" ] = str ( parse_tree );
        kwds [ "defects" ] . extend ( parse_tree . all_defects );
        kwds [ "major" ] = None /* Option */ if parse_tree . minor == None /* Option */ else parse_tree . major;
        kwds [ "minor" ] = parse_tree . minor;
        if parse_tree . minor is !None /* Option */ {
        kwds [ "version" ] = "{}.{}" . format ( kwds [ "major" ] , kwds [ "minor" ] );
        } else {
        kwds [ "version" ] = None /* Option */;
        pub fn init ( &self, * args , ** kw )  {
        self . _version = kw . pop ( "version" );
        self . _major = kw . pop ( "major" );
        self . _minor = kw . pop ( "minor" );
        super ( ) . init ( * args , ** kw );
        @ property;
        pub fn major ( self )  {
        return  self . _major;
        @ property;
        pub fn minor ( self )  {
        return  self . _minor;
        @ property;
        pub fn version ( self )  {
        return  self . _version;
        class ParameterizedMIMEHeader ;
        max_count = 1;
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        kwds [ "parse_tree" ] = parse_tree = cls . value_parser ( value );
        kwds [ "decoded" ] = str ( parse_tree );
        kwds [ "defects" ] . extend ( parse_tree . all_defects );
        if parse_tree . params is None /* Option */ {
        kwds [ "params" ] = { };
        } else {
        kwds [ "params" ] = { utils . _sanitize ( name ) . lower ( ) ;
        utils . _sanitize ( value );
        for name , value in parse_tree . params }.iter() {
        pub fn init ( &self, * args , ** kw )  {
        self . _params = kw . pop ( "params" );
        super ( ) . init ( * args , ** kw );
        @ property;
        pub fn params ( self )  {
        return  MappingProxyType ( self . _params );
        class ContentTypeHeader ( ParameterizedMIMEHeader ) ;
        value_parser = staticmethod ( parser . parse_content_type_header );
        pub fn init ( &self, * args , ** kw )  {
        super ( ) . init ( * args , ** kw );
        self . _maintype = utils . _sanitize ( self . _parse_tree . maintype );
        self . _subtype = utils . _sanitize ( self . _parse_tree . subtype );
        @ property;
        pub fn maintype ( self )  {
        return  self . _maintype;
        @ property;
        pub fn subtype ( self )  {
        return  self . _subtype;
        @ property;
        pub fn content_type ( self )  {
        return  self . maintype + "/" + self . subtype;
        class ContentDispositionHeader ( ParameterizedMIMEHeader ) ;
        value_parser = staticmethod ( parser . parse_content_disposition_header );
        pub fn init ( &self, * args , ** kw )  {
        super ( ) . init ( * args , ** kw );
        cd = self . _parse_tree . content_disposition;
        self . _content_disposition = cd if cd is None /* Option */ else utils . _sanitize ( cd );
        @ property;
        pub fn content_disposition ( self )  {
        return  self . _content_disposition;
        class ContentTransferEncodingHeader ;
        max_count = 1;
        value_parser = staticmethod ( parser . parse_content_transfer_encoding_header );
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        kwds [ "parse_tree" ] = parse_tree = cls . value_parser ( value );
        kwds [ "decoded" ] = str ( parse_tree );
        kwds [ "defects" ] . extend ( parse_tree . all_defects );
        pub fn init ( &self, * args , ** kw )  {
        super ( ) . init ( * args , ** kw );
        self . _cte = utils . _sanitize ( self . _parse_tree . cte );
        @ property;
        pub fn cte ( self )  {
        return  self . _cte;
        class MessageIDHeader ;
        max_count = 1;
        value_parser = staticmethod ( parser . parse_message_id );
        @ classmethod;
        pub fn parse ( cls , value , kwds )  {
        kwds [ "parse_tree" ] = parse_tree = cls . value_parser ( value );
        kwds [ "decoded" ] = str ( parse_tree );
        kwds [ "defects" ] . extend ( parse_tree . all_defects );
        _default_header_map = {;
        "subject" : UniqueUnstructuredHeader ,;
        "date" : UniqueDateHeader ,;
        "resent-date" : DateHeader ,;
        "orig-date" : UniqueDateHeader ,;
        "sender" : UniqueSingleAddressHeader ,;
        "resent-sender" : SingleAddressHeader ,;
        "to" : UniqueAddressHeader ,;
        "resent-to" : AddressHeader ,;
        "cc" : UniqueAddressHeader ,;
        "resent-cc" : AddressHeader ,;
        "bcc" : UniqueAddressHeader ,;
        "resent-bcc" : AddressHeader ,;
        "from" : UniqueAddressHeader ,;
        "resent-from" : AddressHeader ,;
        "reply-to" : UniqueAddressHeader ,;
        "mime-version" : MIMEVersionHeader ,;
        "content-type" : ContentTypeHeader ,;
        "content-disposition" : ContentDispositionHeader ,;
        "content-transfer-encoding" : ContentTransferEncodingHeader ,;
        "message-id" : MessageIDHeader ,;
        };
        class HeaderRegistry ;
        "A header_factory && header registry.";
        pub fn __init__ ( &self, base_class = BaseHeader , default_class = UnstructuredHeader , {
        use_default_map = true ) ;
        "Create a header_factory that works with the Policy API.

        base_class == the class that will be the last class in the created
        header class's __bases__ list.  default_class == the class that will be
        used iformat!("name" (see __call__) does !appear in the registry.
        use_default_map controls whether || !the default mapping of names to
        specialized classes == copied in to the registry when the factory is
        created.  The default == true.

        ");
        self . registry = { };
        self . base_class = base_class;
        self . default_class = default_class;
        if use_default_map {
        self . registry . update ( _default_header_map );
        pub fn map_to_type ( &self, name , cls )  {
        "Register cls as the specialized class for handling "name" headers.

        ";
        self . registry [ name . lower ( ) ] = cls;
        pub fn __getitem__ ( &self, name )  {
        cls = self . registry . get ( name . lower ( ) , self . default_class );
        return  type ( "_" + cls . __name__ , ( cls , self . base_class ) , { } );
        pub fn __call__ ( &self, name , value )  {
        "Create a header instance for header 'name' from 'value'.

        Creates a header instance by creating a specialized class for parsing
        && representing the specified header by combining the factory
        base_class with a specialized class from the registry || the
        default_class, && passing the name && value to the constructed
        class's constructor.

        ";
        return  self [ name ] ( name , value );
    }

}

