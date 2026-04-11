//! saxutils.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::codecs;
// use crate::handler;
// use crate::xmlreader;
// use std::env;

pub fn __dict_replace(s: &str, d: &str) {
        "Replace substrings of a string using a dictionary.";
        for key , value in d . items ( ) .iter() {
        s = s . replace ( key , value );
        return  s;
        pub fn escape ( data , entities = { } )  {
        "Escape &, <, && > in a string of data.

    You can escape other strings of data by passing a dictionary as
    the optional entities parameter.  The keys && values must all be
    strings; each key will be replaced with its corresponding value.
    ";
        data = data . replace ( "&" , "&amp;" );
        data = data . replace ( ">" , "&gt;" );
        data = data . replace ( "<" , "&lt;" );
        if entities {
        data = __dict_replace ( data , entities );
        return  data;
        pub fn unescape ( data , entities = { } )  {
        "Unescape &amp;, &lt;, && &gt; in a string of data.

    You can unescape other strings of data by passing a dictionary as
    the optional entities parameter.  The keys && values must all be
    strings; each key will be replaced with its corresponding value.
    ";
        data = data . replace ( "&lt;" , "<" );
        data = data . replace ( "&gt;" , ">" );
        if entities {
        data = __dict_replace ( data , entities );
        return  data . replace ( "&amp;" , "&" );
        pub fn quoteattr ( data , entities = { } )  {
        "Escape && quote an attribute value.

    Escape &, <, && > in a string of data, then quote it for use as
    an attribute value.  The \" character will be escaped as well, if
    necessary.

    You can escape other strings of data by passing a dictionary as
    the optional entities parameter.  The keys && values must all be
    strings; each key will be replaced with its corresponding value.
    ";
        entities = { ** entities , "\n" : "&#10;" , "\r" : "&#13;" , "\t" : "&#9;" };
        data = escape ( data , entities );
        if """ in data {
        if "'" in data {
        data = ""%s"" % data . replace ( """ , "&quot;" );
        } else {
        data = "'%s'" % data;
        } else {
        data = ""%s"" % data;
        return  data;
        pub fn _gettextwriter ( out , encoding )  {
        if out is None /* Option */ {
        import sys;
        return  sys . stdout;
        if isinstance ( out , io . TextIOBase ) {
        return  out;
        if isinstance ( out , ( codecs . StreamWriter , codecs . StreamReaderWriter ) ) {
        return  out;
        if isinstance ( out , io . RawIOBase ) {
        class _wrapper ;
        __class__ = out . __class__;
        pub fn __getattr__ ( &self, name )  {
        return  getattr ( out , name );
        buffer = _wrapper ( );
        buffer . close = || {  None /* Option */ };
        } else {
        buffer = io . BufferedIOBase ( );
        buffer . writable = || {  true };
        buffer . write = out . write;
        // try {
        buffer . seekable = out . seekable;
        buffer . tell = out . tell;
        // } catch  AttributeError  {
        // pass
        return  io . TextIOWrapper ( buffer , encoding = encoding ,;
        errors = "xmlcharrefreplace" ,;
        newline = "\n" ,;
        write_through = true );
        class XMLGenerator ( handler . ContentHandler ) ;
        pub fn __init__ ( &self, out = None /* Option */ , encoding = "iso-8859-1" , short_empty_elements = false )  {
        handler . ContentHandler . __init__ ( self );
        out = _gettextwriter ( out , encoding );
        self . _write = out . write;
        self . _flush = out . flush;
        self . _ns_contexts = [ { } ];
        self . _current_context = self . _ns_contexts [ -1 ];
        self . _undeclared_ns_maps = [ ];
        self . _encoding = encoding;
        self . _short_empty_elements = short_empty_elements;
        self . _pending_start_element = false;
        pub fn _qname ( &self, name )  {
        "Builds a qualified name from a (ns_url, localname) pair";
        if name [ 0 ] {
        if "http://www.w3.org/XML/1998/namespace" == name [ 0 ] {
        return  "xml:" + name [ 1 ];
        prefix = self . _current_context [ name [ 0 ] ];
        if prefix {
        return  prefix + ":" + name [ 1 ];
        return  name [ 1 ];
        pub fn _finish_pending_start_element ( &self, endElement = false )  {
        if self . _pending_start_element {
        self . _write ( ">" );
        self . _pending_start_element = false;
        pub fn startDocument ( self )  {
        self . _write ( "<?xml version="1.0" encoding="%s"?>\n" %;
        self . _encoding );
        pub fn endDocument ( self )  {
        self . _flush ( );
        pub fn startPrefixMapping ( &self, prefix , uri )  {
        self . _ns_contexts . append ( self . _current_context . copy ( ) );
        self . _current_context [ uri ] = prefix;
        self . _undeclared_ns_maps . append ( ( prefix , uri ) );
        pub fn endPrefixMapping ( &self, prefix )  {
        self . _current_context = self . _ns_contexts [ -1 ];
        del self . _ns_contexts [ -1 ];
        pub fn startElement ( &self, name , attrs )  {
        self . _finish_pending_start_element ( );
        self . _write ( "<" + name );
        for ( name , value ) in attrs . items ( ) .iter() {
        self . _write ( " %s=%s" % ( name , quoteattr ( value ) ) );
        if self . _short_empty_elements {
        self . _pending_start_element = true;
        } else {
        self . _write ( ">" );
        pub fn endElement ( &self, name )  {
        if self . _pending_start_element {
        self . _write ( "/>" );
        self . _pending_start_element = false;
        } else {
        self . _write ( "</%s>" % name );
        pub fn startElementNS ( &self, name , qname , attrs )  {
        self . _finish_pending_start_element ( );
        self . _write ( "<" + self . _qname ( name ) );
        for prefix , uri in self . _undeclared_ns_maps .iter() {
        if prefix {
        self . _write ( " xmlns:%s="%s"" % ( prefix , uri ) );
        } else {
        self . _write ( " xmlns="%s"" % uri );
        self . _undeclared_ns_maps = [ ];
        for ( name , value ) in attrs . items ( ) .iter() {
        self . _write ( " %s=%s" % ( self . _qname ( name ) , quoteattr ( value ) ) );
        if self . _short_empty_elements {
        self . _pending_start_element = true;
        } else {
        self . _write ( ">" );
        pub fn endElementNS ( &self, name , qname )  {
        if self . _pending_start_element {
        self . _write ( "/>" );
        self . _pending_start_element = false;
        } else {
        self . _write ( "</%s>" % self . _qname ( name ) );
        pub fn characters ( &self, content )  {
        if content {
        self . _finish_pending_start_element ( );
        if !isinstance ( content , str ) {
        content = str ( content , self . _encoding );
        self . _write ( escape ( content ) );
        pub fn ignorableWhitespace ( &self, content )  {
        if content {
        self . _finish_pending_start_element ( );
        if !isinstance ( content , str ) {
        content = str ( content , self . _encoding );
        self . _write ( content );
        pub fn processingInstruction ( &self, target , data )  {
        self . _finish_pending_start_element ( );
        self . _write ( "<?%s %s?>" % ( target , data ) );
        class XMLFilterBase ( xmlreader . XMLReader ) ;
        "This class == designed to sit between an XMLReader && the
    client application's event handlers.  By default, it does nothing
    but pass requests up to the reader && events on to the handlers
    unmodified, but subclasses can override specific methods to modify
    the event stream || the configuration requests as they pass
    through.";
        pub fn __init__ ( &self, parent = None /* Option */ )  {
        xmlreader . XMLReader . __init__ ( self );
        self . _parent = parent;
        pub fn error ( &self, exception )  {
        self . _err_handler . error ( exception );
        pub fn fatalError ( &self, exception )  {
        self . _err_handler . fatalError ( exception );
        pub fn warning ( &self, exception )  {
        self . _err_handler . warning ( exception );
        pub fn setDocumentLocator ( &self, locator )  {
        self . _cont_handler . setDocumentLocator ( locator );
        pub fn startDocument ( self )  {
        self . _cont_handler . startDocument ( );
        pub fn endDocument ( self )  {
        self . _cont_handler . endDocument ( );
        pub fn startPrefixMapping ( &self, prefix , uri )  {
        self . _cont_handler . startPrefixMapping ( prefix , uri );
        pub fn endPrefixMapping ( &self, prefix )  {
        self . _cont_handler . endPrefixMapping ( prefix );
        pub fn startElement ( &self, name , attrs )  {
        self . _cont_handler . startElement ( name , attrs );
        pub fn endElement ( &self, name )  {
        self . _cont_handler . endElement ( name );
        pub fn startElementNS ( &self, name , qname , attrs )  {
        self . _cont_handler . startElementNS ( name , qname , attrs );
        pub fn endElementNS ( &self, name , qname )  {
        self . _cont_handler . endElementNS ( name , qname );
        pub fn characters ( &self, content )  {
        self . _cont_handler . characters ( content );
        pub fn ignorableWhitespace ( &self, chars )  {
        self . _cont_handler . ignorableWhitespace ( chars );
        pub fn processingInstruction ( &self, target , data )  {
        self . _cont_handler . processingInstruction ( target , data );
        pub fn skippedEntity ( &self, name )  {
        self . _cont_handler . skippedEntity ( name );
        pub fn notationDecl ( &self, name , publicId , systemId )  {
        self . _dtd_handler . notationDecl ( name , publicId , systemId );
        pub fn unparsedEntityDecl ( &self, name , publicId , systemId , ndata )  {
        self . _dtd_handler . unparsedEntityDecl ( name , publicId , systemId , ndata );
        pub fn resolveEntity ( &self, publicId , systemId )  {
        return  self . _ent_handler . resolveEntity ( publicId , systemId );
        pub fn parse ( &self, source )  {
        self . _parent . setContentHandler ( self );
        self . _parent . setErrorHandler ( self );
        self . _parent . setEntityResolver ( self );
        self . _parent . setDTDHandler ( self );
        self . _parent . parse ( source );
        pub fn setLocale ( &self, locale )  {
        self . _parent . setLocale ( locale );
        pub fn getFeature ( &self, name )  {
        return  self . _parent . getFeature ( name );
        pub fn setFeature ( &self, name , state )  {
        self . _parent . setFeature ( name , state );
        pub fn getProperty ( &self, name )  {
        return  self . _parent . getProperty ( name );
        pub fn setProperty ( &self, name , value )  {
        self . _parent . setProperty ( name , value );
        pub fn getParent ( self )  {
        return  self . _parent;
        pub fn setParent ( &self, parent )  {
        self . _parent = parent;
        pub fn prepare_input_source ( source , base = "" )  {
        "This function takes an InputSource && an optional base URL and
    returns a fully resolved InputSource object ready for reading.";
        if isinstance ( source , os . PathLike ) {
        source = os . fspath ( source );
        if isinstance ( source , str ) {
        source = xmlreader . InputSource ( source );
        } else if hasattr ( source , "read" ) {
        f = source;
        source = xmlreader . InputSource ( );
        if isinstance ( f . read ( 0 ) , str ) {
        source . setCharacterStream ( f );
        } else {
        source . setByteStream ( f );
        if hasattr ( f , "name" ) && isinstance ( f . name , str ) {
        source . setSystemId ( f . name );
        if source . getCharacterStream ( ) is None /* Option */ && source . getByteStream ( ) is None /* Option */ {
        sysid = source . getSystemId ( );
        basehead = os . path . dirname ( os . path . normpath ( base ) );
        sysidfilename = os . path . join ( basehead , sysid );
        if os . path . isfile ( sysidfilename ) {
        source . setSystemId ( sysidfilename );
        f = open ( sysidfilename , "rb" );
        } else {
        source . setSystemId ( urllib . parse . urljoin ( base , sysid ) );
        f = urllib . request . urlopen ( source . getSystemId ( ) );
        source . setByteStream ( f );
        return  source;
}

