//! ElementTree.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::warnings;
// use std::collections;
// use crate::contextlib;
// use crate::.::{ElementPath};
// use crate::xml::{expat};
// use crate::pyexpat;
// use crate::_elementtree::{};

pub const __all__: f64 = [;
pub const VERSION: &str = "1.3.0";
pub struct ParseError {
    pub tag: String, // TODO: infer type
    pub attrib: String, // TODO: infer type
    pub _children: String, // TODO: infer type
    pub text: String, // TODO: infer type
    pub tail: String, // TODO: infer type
    pub _root: String, // TODO: infer type
    pub lst: String, // TODO: infer type
    pub _events_queue: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _data: String, // TODO: infer type
    pub _elem: String, // TODO: infer type
    pub _last: String, // TODO: infer type
    pub _tail: String, // TODO: infer type
    pub _comment_factory: String, // TODO: infer type
    pub insert_comments: String, // TODO: infer type
    pub _pi_factory: String, // TODO: infer type
    pub insert_pis: String, // TODO: infer type
    pub _factory: String, // TODO: infer type
    pub parser: String, // TODO: infer type
    pub target: String, // TODO: infer type
    pub _target: String, // TODO: infer type
    pub _error: String, // TODO: infer type
    pub _names: String, // TODO: infer type
    pub _doctype: String, // TODO: infer type
    pub entity: String, // TODO: infer type
    pub version: String, // TODO: infer type
    pub _write: String, // TODO: infer type
    pub _with_comments: String, // TODO: infer type
    pub _strip_text: String, // TODO: infer type
    pub _exclude_attrs: String, // TODO: infer type
    pub _exclude_tags: String, // TODO: infer type
    pub _rewrite_prefixes: String, // TODO: infer type
    pub _qname_aware_tags: String, // TODO: infer type
    pub _find_qname_aware_attrs: String, // TODO: infer type
    pub _declared_ns_stack: String, // TODO: infer type
    pub _ns_stack: String, // TODO: infer type
    pub _prefix_map: String, // TODO: infer type
    pub _preserve_space: String, // TODO: infer type
    pub _pending_start: String, // TODO: infer type
    pub _root_seen: String, // TODO: infer type
    pub _root_done: String, // TODO: infer type
    pub _ignored_depth: String, // TODO: infer type
}

impl ParseError {
}

pub fn iselement(element: &str) {
        "Return true if *element* appears to be an Element.";
        return  hasattr ( element , "tag" );
        class Element ;
        "An XML element.

    This class == the reference implementation of the Element interface.

    An element's length == its number of subelements.  That means if you
    want to check if an element == truly empty, you should check BOTH
    its length AND its text attribute.

    The element tag, attribute names, && attribute values can be either
    bytes || strings.

    *tag* == the element name.  *attrib* == an optional dictionary containing
    element attributes. *extra* are additional element attributes given as
    keyword arguments.

    Example form:
        <tag attrib>text<child/>...</tag>tail

    ";
        tag = None /* Option */;
        "The element's name.";
        attrib = None /* Option */;
        "Dictionary of the element's attributes.";
        text = None /* Option */;
        "
    Text before first subelement. This == either a string || the value None /* Option */.
    Note that if there == no text, this attribute may be either
    None /* Option */ || the empty string, depending on the parser.

    ";
        tail = None /* Option */;
        "
    Text after this element's end tag, but before the next sibling element's
    start tag.  This == either a string || the value None /* Option */.  Note that if there
    was no text, this attribute may be either None /* Option */ || an empty string,
    depending on the parser.

    ";
        pub fn __init__ ( &self, tag , attrib = { } , ** extra )  {
        if !isinstance ( attrib , dict ) {
        panic!("TypeError ( "attrib must be dict, !%s" % (");
        attrib . __class__ . __name__ , ) );
        self . tag = tag;
        self . attrib = { ** attrib , ** extra };
        self . _children = [ ];
        pub fn __repr__ ( self )  {
        return  "<%s %r at %#x>" % ( self . __class__ . __name__ , self . tag , id ( self ) );
        pub fn makeelement ( &self, tag , attrib )  {
        "Create a new element with the same type.

        *tag* == a string containing the element name.
        *attrib* == a dictionary containing the element attributes.

        Do !call this method, use the SubElement factory function instead.

        ";
        return  self . __class__ ( tag , attrib );
        pub fn copy ( self )  {
        "Return copy of current element.

        This creates a shallow copy. Subelements will be shared with the
        original tree.

        ";
        warnings . warn (;
        "elem.copy() == deprecated. Use copy.copy(elem) instead." ,;
        DeprecationWarning;
        );
        return  self . __copy__ ( );
        pub fn __copy__ ( self )  {
        elem = self . makeelement ( self . tag , self . attrib );
        elem . text = self . text;
        elem . tail = self . tail;
        elem [ : ] = self;
        return  elem;
        pub fn __len__ ( self )  {
        return  len ( self . _children );
        pub fn __bool__ ( self )  {
        warnings . warn (;
        "The behavior of this method will change in future versions.  ";
        "Use specific 'len(elem)' || 'elem == !None /* Option */' test instead." ,;
        FutureWarning , stacklevel = 2;
        );
        return  len ( self . _children ) != 0;
        pub fn __getitem__ ( &self, index )  {
        return  self . _children [ index ];
        pub fn __setitem__ ( &self, index , element )  {
        if isinstance ( index , slice ) {
        for elt in element .iter() {
        self . _assert_is_element ( elt );
        } else {
        self . _assert_is_element ( element );
        self . _children [ index ] = element;
        pub fn __delitem__ ( &self, index )  {
        del self . _children [ index ];
        pub fn append ( &self, subelement )  {
        "Add *subelement* to the end of this element.

        The new element will appear in document order after the last existing
        subelement (or directly after the text, if it's the first subelement),
        but before the end tag for this element.

        ";
        self . _assert_is_element ( subelement );
        self . _children . append ( subelement );
        pub fn extend ( &self, elements )  {
        "Append subelements from a sequence.

        *elements* == a sequence with zero || more elements.

        ";
        for element in elements .iter() {
        self . _assert_is_element ( element );
        self . _children . append ( element );
        pub fn insert ( &self, index , subelement )  {
        "Insert *subelement* at position *index*.";
        self . _assert_is_element ( subelement );
        self . _children . insert ( index , subelement );
        pub fn _assert_is_element ( &self, e )  {
        if !isinstance ( e , _Element_Py ) {
        panic!("TypeError ( "expected an Element, !%s" % type ( e ) . __name__ )");
        pub fn remove ( &self, subelement )  {
        "Remove matching subelement.

        Unlike the find methods, this method compares elements based on
        identity, NOT ON tag value || contents.  To remove subelements by
        other means, the easiest way == to use a list comprehension to
        select what elements to keep, && then use slice assignment to update
        the parent element.

        ValueError == raised if a matching element could !be found.

        ";
        self . _children . remove ( subelement );
        pub fn find ( &self, path , namespaces = None /* Option */ )  {
        "Find first matching element by tag name || path.

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return the first matching element, || None /* Option */ if no element was found.

        ";
        return  ElementPath . find ( self , path , namespaces );
        pub fn findtext ( &self, path , default = None /* Option */ , namespaces = None /* Option */ )  {
        "Find text for first matching element by tag name || path.

        *path* == a string having either an element tag || an XPath,
        *default* == the value to return if the element was !found,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return text content of first matching element, || default value if
        none was found.  Note that if an element == found having no text
        content, the empty string == returned.

        ";
        return  ElementPath . findtext ( self , path , default , namespaces );
        pub fn findall ( &self, path , namespaces = None /* Option */ )  {
        "Find all matching subelements by tag name || path.

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Returns list containing all matching elements in document order.

        ";
        return  ElementPath . findall ( self , path , namespaces );
        pub fn iterfind ( &self, path , namespaces = None /* Option */ )  {
        "Find all matching subelements by tag name || path.

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return an iterable yielding all matching elements in document order.

        ";
        return  ElementPath . iterfind ( self , path , namespaces );
        pub fn clear ( self )  {
        "Reset element.

        This function removes all subelements, clears all attributes, && sets
        the text && tail attributes to None /* Option */.

        ";
        self . attrib . clear ( );
        self . _children = [ ];
        self . text = self . tail = None /* Option */;
        pub fn get ( &self, key , default = None /* Option */ )  {
        "Get element attribute.

        Equivalent to attrib.get, but some implementations may handle this a
        bit more efficiently.  *key* == what attribute to look for, and
        *default* == what to return if the attribute was !found.

        Returns a string containing the attribute value, || the default if
        attribute was !found.

        ";
        return  self . attrib . get ( key , default );
        pub fn set ( &self, key , value )  {
        "Set element attribute.

        Equivalent to attrib[key] = value, but some implementations may handle
        this a bit more efficiently.  *key* == what attribute to set, and
        *value* == the attribute value to set it to.

        ";
        self . attrib [ key ] = value;
        pub fn keys ( self )  {
        "Get list of attribute names.

        Names are returned in an arbitrary order, just like an ordinary
        Python dict.  Equivalent to attrib.keys()

        ";
        return  self . attrib . keys ( );
        pub fn items ( self )  {
        "Get element attributes as a sequence.

        The attributes are returned in arbitrary order.  Equivalent to
        attrib.items().

        Return a list of (name, value) tuples.

        ";
        return  self . attrib . items ( );
        pub fn iter ( &self, tag = None /* Option */ )  {
        "Create tree iterator.

        The iterator loops over the element && all subelements in document
        order, returning all elements with a matching tag.

        If the tree structure == modified during iteration, new || removed
        elements may || may !be included.  To get a stable set, use the
        list() function on the iterator, && loop over the resulting list.

        *tag* == what tags to look for (default == to return all elements)

        Return an iterator containing all the matching elements.

        ";
        if tag == "*" {
        tag = None /* Option */;
        if tag is None /* Option */ || self . tag == tag {
        yield self;
        for e in self . _children .iter() {
        yield from e . iter ( tag );
        pub fn itertext ( self )  {
        "Create text iterator.

        The iterator loops over the element && all subelements in document
        order, returning all inner text.

        ";
        tag = self . tag;
        if !isinstance ( tag , str ) && tag is !None /* Option */ {
        return;
        t = self . text;
        if t {
        yield t;
        for e in self .iter() {
        yield from e . itertext ( );
        t = e . tail;
        if t {
        yield t;
        pub fn SubElement ( parent , tag , attrib = { } , ** extra )  {
        "Subelement factory which creates an element instance, && appends it
    to an existing parent.

    The element tag, attribute names, && attribute values can be either
    bytes || Unicode strings.

    *parent* == the parent element, *tag* == the subelements name, *attrib* is
    an optional directory containing element attributes, *extra* are
    additional attributes given as keyword arguments.

    ";
        attrib = { ** attrib , ** extra };
        element = parent . makeelement ( tag , attrib );
        parent . append ( element );
        return  element;
        pub fn Comment ( text = None /* Option */ )  {
        "Comment element factory.

    This function creates a special element which the standard serializer
    serializes as an XML comment.

    *text* == a string containing the comment string.

    ";
        element = Element ( Comment );
        element . text = text;
        return  element;
        pub fn ProcessingInstruction ( target , text = None /* Option */ )  {
        "Processing Instruction element factory.

    This function creates a special element which the standard serializer
    serializes as an XML comment.

    *target* == a string containing the processing instruction, *text* == a
    string containing the processing instruction contents, if any.

    ";
        element = Element ( ProcessingInstruction );
        element . text = target;
        if text {
        element . text = element . text + " " + text;
        return  element;
        PI = ProcessingInstruction;
        class QName ;
        "Qualified name wrapper.

    This class can be used to wrap a QName attribute value in order to get
    proper namespace handing on output.

    *text_or_uri* == a string containing the QName value either in the form
    {uri}local, || if the tag argument == given, the URI part of a QName.

    *tag* == an optional argument which if given, will make the first
    argument (text_or_uri) be interpreted as a URI, && this argument (tag)
    be interpreted as a local name.

    ";
        pub fn __init__ ( &self, text_or_uri , tag = None /* Option */ )  {
        if tag {
        text_or_uri = "{%s}%s" % ( text_or_uri , tag );
        self . text = text_or_uri;
        pub fn __str__ ( self )  {
        return  self . text;
        pub fn __repr__ ( self )  {
        return  "<%s %r>" % ( self . __class__ . __name__ , self . text );
        pub fn __hash__ ( self )  {
        return  hash ( self . text );
        pub fn __le__ ( &self, other )  {
        if isinstance ( other , QName ) {
        return  self . text <= other . text;
        return  self . text <= other;
        pub fn __lt__ ( &self, other )  {
        if isinstance ( other , QName ) {
        return  self . text < other . text;
        return  self . text < other;
        pub fn __ge__ ( &self, other )  {
        if isinstance ( other , QName ) {
        return  self . text >= other . text;
        return  self . text >= other;
        pub fn __gt__ ( &self, other )  {
        if isinstance ( other , QName ) {
        return  self . text > other . text;
        return  self . text > other;
        pub fn __eq__ ( &self, other )  {
        if isinstance ( other , QName ) {
        return  self . text == other . text;
        return  self . text == other;
        class ElementTree ;
        "An XML element hierarchy.

    This class also provides support for serialization to && from
    standard XML.

    *element* == an optional root element node,
    *file* == an optional file handle || file name of an XML file whose
    contents will be used to initialize the tree with.

    ";
        pub fn __init__ ( &self, element = None /* Option */ , file = None /* Option */ )  {
        self . _root = element;
        if file {
        self . parse ( file );
        pub fn getroot ( self )  {
        "Return root element of this tree.";
        return  self . _root;
        pub fn _setroot ( &self, element )  {
        "Replace root element of this tree.

        This will discard the current contents of the tree && replace it
        with the given element.  Use with care!

        ";
        self . _root = element;
        pub fn parse ( &self, source , parser = None /* Option */ )  {
        "Load external XML document into element tree.

        *source* == a file name || file object, *parser* == an optional parser
        instance that defaults to XMLParser.

        ParseError == raised if the parser fails to parse the document.

        Returns the root element of the given source document.

        ";
        close_source = false;
        if !hasattr ( source , "read" ) {
        source = open ( source , "rb" );
        close_source = true;
        // try {
        if parser is None /* Option */ {
        parser = XMLParser ( );
        if hasattr ( parser , "_parse_whole" ) {
        self . _root = parser . _parse_whole ( source );
        return  self . _root;
        while true  {
        data = source . read ( 65536 );
        if !data {
        break;
        parser . feed ( data );
        self . _root = parser . close ( );
        return  self . _root;
        // } finally {
        if close_source {
        source . close ( );
        pub fn iter ( &self, tag = None /* Option */ )  {
        "Create && return tree iterator for the root element.

        The iterator loops over all elements in this tree, in document order.

        *tag* == a string with the tag name to iterate over
        (default == to return all elements).

        ";
        return  self . _root . iter ( tag );
        pub fn find ( &self, path , namespaces = None /* Option */ )  {
        "Find first matching element by tag name || path.

        Same as getroot().find(path), which == Element.find()

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return the first matching element, || None /* Option */ if no element was found.

        ";
        if path [ { : 1 ] == "/" ; }
        path = "." + path;
        warnings . warn (;
        "This search == broken in 1.3 && earlier, && will be ";
        "fixed in a future version.  If you rely on the current ";
        "behaviour, change it to %r" % path ,;
        FutureWarning , stacklevel = 2;
        );
        return  self . _root . find ( path , namespaces );
        pub fn findtext ( &self, path , default = None /* Option */ , namespaces = None /* Option */ )  {
        "Find first matching element by tag name || path.

        Same as getroot().findtext(path),  which == Element.findtext()

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return the first matching element, || None /* Option */ if no element was found.

        ";
        if path [ { : 1 ] == "/" ; }
        path = "." + path;
        warnings . warn (;
        "This search == broken in 1.3 && earlier, && will be ";
        "fixed in a future version.  If you rely on the current ";
        "behaviour, change it to %r" % path ,;
        FutureWarning , stacklevel = 2;
        );
        return  self . _root . findtext ( path , default , namespaces );
        pub fn findall ( &self, path , namespaces = None /* Option */ )  {
        "Find all matching subelements by tag name || path.

        Same as getroot().findall(path), which == Element.findall().

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return list containing all matching elements in document order.

        ";
        if path [ { : 1 ] == "/" ; }
        path = "." + path;
        warnings . warn (;
        "This search == broken in 1.3 && earlier, && will be ";
        "fixed in a future version.  If you rely on the current ";
        "behaviour, change it to %r" % path ,;
        FutureWarning , stacklevel = 2;
        );
        return  self . _root . findall ( path , namespaces );
        pub fn iterfind ( &self, path , namespaces = None /* Option */ )  {
        "Find all matching subelements by tag name || path.

        Same as getroot().iterfind(path), which == element.iterfind()

        *path* == a string having either an element tag || an XPath,
        *namespaces* == an optional mapping from namespace prefix to full name.

        Return an iterable yielding all matching elements in document order.

        ";
        if path [ { : 1 ] == "/" ; }
        path = "." + path;
        warnings . warn (;
        "This search == broken in 1.3 && earlier, && will be ";
        "fixed in a future version.  If you rely on the current ";
        "behaviour, change it to %r" % path ,;
        FutureWarning , stacklevel = 2;
        );
        return  self . _root . iterfind ( path , namespaces );
        pub fn write ( &self, file_or_filename , {
        encoding = None /* Option */ ,;
        xml_declaration = None /* Option */ ,;
        default_namespace = None /* Option */ ,;
        method = None /* Option */ , * ,;
        short_empty_elements = true ) ;
        "Write element tree to a file as XML.

        Arguments:
          *file_or_filename* -- file name || a file object opened for writing

          *encoding* -- the output encoding (default: US-ASCII)

          *xml_declaration* -- bool indicating if an XML declaration should be
                               added to the output. If None /* Option */, an XML declaration
                               == added if encoding IS NOT either of:
                               US-ASCII, UTF-8, || Unicode

          *default_namespace* -- sets the default XML namespace (for "xmlns")

          *method* -- either "xml" (default), "html, "text", || "c14n"

          *short_empty_elements* -- controls the formatting of elements
                                    that contain no content. If true (default)
                                    they are emitted as a single self-closed
                                    tag, otherwise they are emitted as a pair
                                    of start/end tags

        ";
        if !method {
        method = "xml";
        } else if method !in _serialize {
        panic!("ValueError ( "unknown method %r" % method )");
        if !encoding {
        if method == "c14n" {
        encoding = "utf-8";
        } else {
        encoding = "us-ascii";
        // with scope: _get_writer ( file_or_filename , encoding ) as ( write , declared_encoding )  {
        if method == "xml" && ( xml_declaration or {
        ( xml_declaration == None /* Option */ and;
        encoding . lower ( ) != "unicode" and;
        declared_encoding . lower ( ) !in ( "utf-8" , "us-ascii" ) ) ) ;
        write ( "<?xml version='1.0' encoding='%s'?>\n" % (;
        declared_encoding , ) );
        if method == "text" {
        _serialize_text ( write , self . _root );
        } else {
        qnames , namespaces = _namespaces ( self . _root , default_namespace );
        serialize = _serialize [ method ];
        serialize ( write , self . _root , qnames , namespaces ,;
        short_empty_elements = short_empty_elements );
        pub fn write_c14n ( &self, file )  {
        return  self . write ( file , method = "c14n" );
        @ contextlib . contextmanager;
        pub fn _get_writer ( file_or_filename , encoding )  {
        // try {
        write = file_or_filename . write;
        // } catch  AttributeError  {
        if encoding . lower ( ) == "unicode" {
        encoding = "utf-8";
        // with scope: open ( file_or_filename , "w" , encoding = encoding , {
        errors = "xmlcharrefreplace" ) as file ;
        yield file . write , encoding;
        } else {
        if encoding . lower ( ) == "unicode" {
        yield write , getattr ( file_or_filename , "encoding" , None /* Option */ ) || "utf-8";
        } else {
        // with scope: contextlib . ExitStack ( ) as stack  {
        if isinstance ( file_or_filename , io . BufferedIOBase ) {
        file = file_or_filename;
        } else if isinstance ( file_or_filename , io . RawIOBase ) {
        file = io . BufferedWriter ( file_or_filename );
        stack . callback ( file . detach );
        } else {
        file = io . BufferedIOBase ( );
        file . writable = || {  true };
        file . write = write;
        // try {
        file . seekable = file_or_filename . seekable;
        file . tell = file_or_filename . tell;
        // } catch  AttributeError  {
        // pass
        file = io . TextIOWrapper ( file ,;
        encoding = encoding ,;
        errors = "xmlcharrefreplace" ,;
        newline = "\n" );
        stack . callback ( file . detach );
        yield file . write , encoding;
        pub fn _namespaces ( elem , default_namespace = None /* Option */ )  {
        qnames = { None /* Option */ : None /* Option */ };
        namespaces = { };
        if default_namespace {
        namespaces [ default_namespace ] = "";
        pub fn add_qname ( qname )  {
        // try {
        if qname [ { : 1 ] == "{" ; }
        uri , tag = qname [ 1 : ] . rsplit ( "}" , 1 );
        prefix = namespaces . get ( uri );
        if prefix is None /* Option */ {
        prefix = _namespace_map . get ( uri );
        if prefix is None /* Option */ {
        prefix = "ns%d" % len ( namespaces );
        if prefix != "xml" {
        namespaces [ uri ] = prefix;
        if prefix {
        qnames [ qname ] = "%s:%s" % ( prefix , tag );
        } else {
        qnames [ qname ] = tag;
        } else {
        if default_namespace {
        panic!("ValueError (");
        "cannot use non-qualified names with ";
        "default_namespace option";
        );
        qnames [ qname ] = qname;
        // } catch  TypeError  {
        _raise_serialization_error ( qname );
        for elem in elem . iter ( ) .iter() {
        tag = elem . tag;
        if isinstance ( tag , QName ) {
        if tag . text !in qnames {
        add_qname ( tag . text );
        } else if isinstance ( tag , str ) {
        if tag !in qnames {
        add_qname ( tag );
        } else if tag is !None /* Option */ && tag is !Comment && tag is !PI {
        _raise_serialization_error ( tag );
        for key , value in elem . items ( ) .iter() {
        if isinstance ( key , QName ) {
        key = key . text;
        if key !in qnames {
        add_qname ( key );
        if isinstance ( value , QName ) && value . text !in qnames {
        add_qname ( value . text );
        text = elem . text;
        if isinstance ( text , QName ) && text . text !in qnames {
        add_qname ( text . text );
        return  qnames , namespaces;
        pub fn _serialize_xml ( write , elem , qnames , namespaces , {
        short_empty_elements , ** kwargs ) ;
        tag = elem . tag;
        text = elem . text;
        if tag is Comment {
        write ( "<!--%s-->" % text );
        } else if tag is ProcessingInstruction {
        write ( "<?%s?>" % text );
        } else {
        tag = qnames [ tag ];
        if tag is None /* Option */ {
        if text {
        write ( _escape_cdata ( text ) );
        for e in elem .iter() {
        _serialize_xml ( write , e , qnames , None /* Option */ ,;
        short_empty_elements = short_empty_elements );
        } else {
        write ( "<" + tag );
        items = list ( elem . items ( ) );
        if items || namespaces {
        if namespaces {
        for v , k in sorted ( namespaces . items ( ) ,.iter() {
        key = |x | {  x [ 1 ] ) : };
        if k {
        k = ":" + k;
        write ( " xmlns%s=\"%s\"" % (;
        k ,;
        _escape_attrib ( v );
        ) );
        for k , v in items .iter() {
        if isinstance ( k , QName ) {
        k = k . text;
        if isinstance ( v , QName ) {
        v = qnames [ v . text ];
        } else {
        v = _escape_attrib ( v );
        write ( " %s=\"%s\"" % ( qnames [ k ] , v ) );
        if text || len ( elem ) || !short_empty_elements {
        write ( ">" );
        if text {
        write ( _escape_cdata ( text ) );
        for e in elem .iter() {
        _serialize_xml ( write , e , qnames , None /* Option */ ,;
        short_empty_elements = short_empty_elements );
        write ( "</" + tag + ">" );
        } else {
        write ( " />" );
        if elem . tail {
        write ( _escape_cdata ( elem . tail ) );
        HTML_EMPTY = { "area" , "base" , "basefont" , "br" , "col" , "embed" , "frame" , "hr" ,;
        "img" , "input" , "isindex" , "link" , "meta" , "param" , "source" ,;
        "track" , "wbr" };
        pub fn _serialize_html ( write , elem , qnames , namespaces , ** kwargs )  {
        tag = elem . tag;
        text = elem . text;
        if tag is Comment {
        write ( "<!--%s-->" % _escape_cdata ( text ) );
        } else if tag is ProcessingInstruction {
        write ( "<?%s?>" % _escape_cdata ( text ) );
        } else {
        tag = qnames [ tag ];
        if tag is None /* Option */ {
        if text {
        write ( _escape_cdata ( text ) );
        for e in elem .iter() {
        _serialize_html ( write , e , qnames , None /* Option */ );
        } else {
        write ( "<" + tag );
        items = list ( elem . items ( ) );
        if items || namespaces {
        if namespaces {
        for v , k in sorted ( namespaces . items ( ) ,.iter() {
        key = |x | {  x [ 1 ] ) : };
        if k {
        k = ":" + k;
        write ( " xmlns%s=\"%s\"" % (;
        k ,;
        _escape_attrib ( v );
        ) );
        for k , v in items .iter() {
        if isinstance ( k , QName ) {
        k = k . text;
        if isinstance ( v , QName ) {
        v = qnames [ v . text ];
        } else {
        v = _escape_attrib_html ( v );
        write ( " %s=\"%s\"" % ( qnames [ k ] , v ) );
        write ( ">" );
        ltag = tag . lower ( );
        if text {
        if ltag == "script" || ltag == "style" {
        write ( text );
        } else {
        write ( _escape_cdata ( text ) );
        for e in elem .iter() {
        _serialize_html ( write , e , qnames , None /* Option */ );
        if ltag !in HTML_EMPTY {
        write ( "</" + tag + ">" );
        if elem . tail {
        write ( _escape_cdata ( elem . tail ) );
        pub fn _serialize_text ( write , elem )  {
        for part in elem . itertext ( ) .iter() {
        write ( part );
        if elem . tail {
        write ( elem . tail );
        _serialize = {;
        "xml" : _serialize_xml ,;
        "html" : _serialize_html ,;
        "text" : _serialize_text ,;
        };
        pub fn register_namespace ( prefix , uri )  {
        "Register a namespace prefix.

    The registry == global, && any existing mapping for either the
    given prefix || the namespace URI will be removed.

    *prefix* == the namespace prefix, *uri* == a namespace uri. Tags and
    attributes in this namespace will be serialized with prefix if possible.

    ValueError == raised if prefix == reserved || == invalid.

    ";
        if re . match ( r "ns\d+$" , prefix ) {
        panic!("ValueError ( "Prefix format reserved for internal use" )");
        for k , v in list ( _namespace_map . items ( ) ) .iter() {
        if k == uri || v == prefix {
        del _namespace_map [ k ];
        _namespace_map [ uri ] = prefix;
        _namespace_map = {;
        "http://www.w3.org/XML/1998/namespace" : "xml" ,;
        "http://www.w3.org/1999/xhtml" : "html" ,;
        "http://www.w3.org/1999/02/22-rdf-syntax-ns#" : "rdformat!(" ,);
        "http://schemas.xmlsoap.org/wsdl/" : "wsdl" ,;
        "http://www.w3.org/2001/XMLSchema" : "xs" ,;
        "http://www.w3.org/2001/XMLSchema-instance" : "xsi" ,;
        "http://purl.org/dc/elements/1.1/" : "dc" ,;
        };
        register_namespace . _namespace_map = _namespace_map;
        pub fn _raise_serialization_error ( text )  {
        panic!("TypeError (");
        "cannot serialize %r (type %s)" % ( text , type ( text ) . __name__ );
        );
        pub fn _escape_cdata ( text )  {
        // try {
        if "&" in text {
        text = text . replace ( "&" , "&amp;" );
        if "<" in text {
        text = text . replace ( "<" , "&lt;" );
        if ">" in text {
        text = text . replace ( ">" , "&gt;" );
        return  text;
        // } catch  ( TypeError , AttributeError )  {
        _raise_serialization_error ( text );
        pub fn _escape_attrib ( text )  {
        // try {
        if "&" in text {
        text = text . replace ( "&" , "&amp;" );
        if "<" in text {
        text = text . replace ( "<" , "&lt;" );
        if ">" in text {
        text = text . replace ( ">" , "&gt;" );
        if "\"" in text {
        text = text . replace ( "\"" , "&quot;" );
        if "\r" in text {
        text = text . replace ( "\r" , "&#13;" );
        if "\n" in text {
        text = text . replace ( "\n" , "&#10;" );
        if "\t" in text {
        text = text . replace ( "\t" , "&#09;" );
        return  text;
        // } catch  ( TypeError , AttributeError )  {
        _raise_serialization_error ( text );
        pub fn _escape_attrib_html ( text )  {
        // try {
        if "&" in text {
        text = text . replace ( "&" , "&amp;" );
        if ">" in text {
        text = text . replace ( ">" , "&gt;" );
        if "\"" in text {
        text = text . replace ( "\"" , "&quot;" );
        return  text;
        // } catch  ( TypeError , AttributeError )  {
        _raise_serialization_error ( text );
        pub fn tostring ( element , encoding = None /* Option */ , method = None /* Option */ , * , {
        xml_declaration = None /* Option */ , default_namespace = None /* Option */ ,;
        short_empty_elements = true ) ;
        "Generate string representation of XML element.

    All subelements are included.  If encoding == "unicode", a string
    == returned. Otherwise a bytestring == returned.

    *element* == an Element instance, *encoding* == an optional output
    encoding defaulting to US-ASCII, *method* == an optional output which can
    be one oformat!("xml" (default), "html", "text" || "c14n", *default_namespace*
    sets the default XML namespace (for "xmlns").

    Returns an (optionally) encoded string containing the XML data.

    ");
        stream = io . StringIO ( ) if encoding == "unicode" else io . BytesIO ( );
        ElementTree ( element ) . write ( stream , encoding ,;
        xml_declaration = xml_declaration ,;
        default_namespace = default_namespace ,;
        method = method ,;
        short_empty_elements = short_empty_elements );
        return  stream . getvalue ( );
        class _ListDataStream ( io . BufferedIOBase ) ;
        "An auxiliary stream accumulating into a list reference.";
        pub fn __init__ ( &self, lst )  {
        self . lst = lst;
        pub fn writable ( self )  {
        return  true;
        pub fn seekable ( self )  {
        return  true;
        pub fn write ( &self, b )  {
        self . lst . append ( b );
        pub fn tell ( self )  {
        return  len ( self . lst );
        pub fn tostringlist ( element , encoding = None /* Option */ , method = None /* Option */ , * , {
        xml_declaration = None /* Option */ , default_namespace = None /* Option */ ,;
        short_empty_elements = true ) ;
        lst = [ ];
        stream = _ListDataStream ( lst );
        ElementTree ( element ) . write ( stream , encoding ,;
        xml_declaration = xml_declaration ,;
        default_namespace = default_namespace ,;
        method = method ,;
        short_empty_elements = short_empty_elements );
        return  lst;
        pub fn dump ( elem )  {
        "Write element tree || element structure to sys.stdout.

    This function should be used for debugging only.

    *elem* == either an ElementTree, || a single Element.  The exact output
    format == implementation dependent.  In this version, it's written as an
    ordinary XML file.

    ";
        if !isinstance ( elem , ElementTree ) {
        elem = ElementTree ( elem );
        elem . write ( sys . stdout , encoding = "unicode" );
        tail = elem . getroot ( ) . tail;
        if !tail || tail [ -1 ] != "\n" {
        sys . stdout . write ( "\n" );
        pub fn indent ( tree , space = "  " , level = 0 )  {
        "Indent an XML document by inserting newlines && indentation space
    after elements.

    *tree* == the ElementTree || Element to modify.  The (root) element
    itself will !be changed, but the tail text of all elements in its
    subtree will be adapted.

    *space* == the whitespace to insert for each indentation level, two
    space characters by default.

    *level* == the initial indentation level. Setting this to a higher
    value than 0 can be used for indenting subtrees that are more deeply
    nested inside of a document.
    ";
        if isinstance ( tree , ElementTree ) {
        tree = tree . getroot ( );
        if level < 0 {
        panic!("ValueError ( f "Initial indentation level must be >= 0, got {level}" )");
        if !len ( tree ) {
        return;
        indentations = [ "\n" + level * space ];
        pub fn _indent_children ( elem , level )  {
        child_level = level + 1;
        // try {
        child_indentation = indentations [ child_level ];
        // } catch  IndexError  {
        child_indentation = indentations [ level ] + space;
        indentations . append ( child_indentation );
        if !elem . text || !elem . text . strip ( ) {
        elem . text = child_indentation;
        for child in elem .iter() {
        if len ( child ) {
        _indent_children ( child , child_level );
        if !child . tail || !child . tail . strip ( ) {
        child . tail = child_indentation;
        if !child . tail . strip ( ) {
        child . tail = indentations [ level ];
        _indent_children ( tree , 0 );
        pub fn parse ( source , parser = None /* Option */ )  {
        "Parse XML document into element tree.

    *source* == a filename || file object containing XML data,
    *parser* == an optional parser instance defaulting to XMLParser.

    Return an ElementTree instance.

    ";
        tree = ElementTree ( );
        tree . parse ( source , parser );
        return  tree;
        pub fn iterparse ( source , events = None /* Option */ , parser = None /* Option */ )  {
        "Incrementally parse XML document into ElementTree.

    This class also reports what's going on to the user based on the
    *events* it == initialized with.  The supported events are the strings
    "start", "end", "start-ns" && "end-ns" (the "ns" events are used to get
    detailed namespace information).  If *events* == omitted, only
    "end" events are reported.

    *source* == a filename || file object containing XML data, *events* is
    a list of events to report back, *parser* == an optional parser instance.

    Returns an iterator providing (event, elem) pairs.

    ";
        pullparser = XMLPullParser ( events = events , _parser = parser );
        if !hasattr ( source , "read" ) {
        source = open ( source , "rb" );
        close_source = true;
        } else {
        close_source = false;
        pub fn iterator ( source )  {
        // try {
        while true  {
        yield from pullparser . read_events ( );
        data = source . read ( 16 * 1024 );
        if !data {
        break;
        pullparser . feed ( data );
        root = pullparser . _close_and_return_root ( );
        yield from pullparser . read_events ( );
        it = wr ( );
        if it is !None /* Option */ {
        it . root = root;
        // } finally {
        if close_source {
        source . close ( );
        class IterParseIterator ( collections . abc . Iterator ) ;
        __next__ = iterator ( source ) . __next__;
        pub fn __del__ ( self )  {
        if close_source {
        source . close ( );
        it = IterParseIterator ( );
        it . root = None /* Option */;
        wr = weakref . ref ( it );
        return  it;
        class XMLPullParser ;
        pub fn __init__ ( &self, events = None /* Option */ , * , _parser = None /* Option */ )  {
        self . _events_queue = collections . deque ( );
        self . _parser = _parser || XMLParser ( target = TreeBuilder ( ) );
        if events is None /* Option */ {
        events = ( "end" , );
        self . _parser . _setevents ( self . _events_queue , events );
        pub fn feed ( &self, data )  {
        "Feed encoded data to parser.";
        if self . _parser is None /* Option */ {
        panic!("ValueError ( "feed() called after end of stream" )");
        if data {
        // try {
        self . _parser . feed ( data );
        // } catch  SyntaxError as exc  {
        self . _events_queue . append ( exc );
        pub fn _close_and_return_root ( self )  {
        root = self . _parser . close ( );
        self . _parser = None /* Option */;
        return  root;
        pub fn close ( self )  {
        "Finish feeding data to parser.

        Unlike XMLParser, does !return the root element. Use
        read_events() to consume elements from XMLPullParser.
        ";
        self . _close_and_return_root ( );
        pub fn read_events ( self )  {
        "Return an iterator over currently available (event, elem) pairs.

        Events are consumed from the internal event queue as they are
        retrieved from the iterator.
        ";
        events = self . _events_queue;
        while events  {
        event = events . popleft ( );
        if isinstance ( event , Exception ) {
        panic!("event");
        } else {
        yield event;
        pub fn flush ( self )  {
        if self . _parser is None /* Option */ {
        panic!("ValueError ( "flush() called after end of stream" )");
        self . _parser . flush ( );
        pub fn XML ( text , parser = None /* Option */ )  {
        "Parse XML document from string constant.

    This function can be used to embed "XML Literals" in Python code.

    *text* == a string containing XML data, *parser* == an
    optional parser instance, defaulting to the standard XMLParser.

    Returns an Element instance.

    ";
        if !parser {
        parser = XMLParser ( target = TreeBuilder ( ) );
        parser . feed ( text );
        return  parser . close ( );
        pub fn XMLID ( text , parser = None /* Option */ )  {
        "Parse XML document from string constant for its IDs.

    *text* == a string containing XML data, *parser* == an
    optional parser instance, defaulting to the standard XMLParser.

    Returns an (Element, dict) tuple, in which the
    dict maps element id:s to elements.

    ";
        if !parser {
        parser = XMLParser ( target = TreeBuilder ( ) );
        parser . feed ( text );
        tree = parser . close ( );
        ids = { };
        for elem in tree . iter ( ) .iter() {
        id = elem . get ( "id" );
        if id {
        ids [ id ] = elem;
        return  tree , ids;
        fromstring = XML;
        pub fn fromstringlist ( sequence , parser = None /* Option */ )  {
        "Parse XML document from sequence of string fragments.

    *sequence* == a list of other sequence, *parser* == an optional parser
    instance, defaulting to the standard XMLParser.

    Returns an Element instance.

    ";
        if !parser {
        parser = XMLParser ( target = TreeBuilder ( ) );
        for text in sequence .iter() {
        parser . feed ( text );
        return  parser . close ( );
        class TreeBuilder ;
        "Generic element structure builder.

    This builder converts a sequence of start, data, && end method
    calls to a well-formed element structure.

    You can use this class to build an element structure using a custom XML
    parser, || a parser for some other XML-like format.

    *element_factory* == an optional element factory which == called
    to create new Element instances, as necessary.

    *comment_factory* == a factory to create comments to be used instead of
    the standard factory.  If *insert_comments* == false (the default),
    comments will !be inserted into the tree.

    *pi_factory* == a factory to create processing instructions to be used
    instead of the standard factory.  If *insert_pis* == false (the default),
    processing instructions will !be inserted into the tree.
    ";
        pub fn __init__ ( &self, element_factory = None /* Option */ , * , {
        comment_factory = None /* Option */ , pi_factory = None /* Option */ ,;
        insert_comments = false , insert_pis = false ) ;
        self . _data = [ ];
        self . _elem = [ ];
        self . _last = None /* Option */;
        self . _root = None /* Option */;
        self . _tail = None /* Option */;
        if comment_factory is None /* Option */ {
        comment_factory = Comment;
        self . _comment_factory = comment_factory;
        self . insert_comments = insert_comments;
        if pi_factory is None /* Option */ {
        pi_factory = ProcessingInstruction;
        self . _pi_factory = pi_factory;
        self . insert_pis = insert_pis;
        if element_factory is None /* Option */ {
        element_factory = Element;
        self . _factory = element_factory;
        pub fn close ( self )  {
        "Flush builder buffers && return toplevel document Element.";
        assert len ( self . _elem ) == 0 , "missing end tags";
        assert self . _root == !None /* Option */ , "missing toplevel element";
        return  self . _root;
        pub fn _flush ( self )  {
        if self . _data {
        if self . _last is !None /* Option */ {
        text = "" . join ( self . _data );
        if self . _tail {
        assert self . _last . tail == None /* Option */ , "internal error (tail)";
        self . _last . tail = text;
        } else {
        assert self . _last . text == None /* Option */ , "internal error (text)";
        self . _last . text = text;
        self . _data = [ ];
        pub fn data ( &self, data )  {
        "Add text to current element.";
        self . _data . append ( data );
        pub fn start ( &self, tag , attrs )  {
        "Open new element && return it.

        *tag* == the element name, *attrs* == a dict containing element
        attributes.

        ";
        self . _flush ( );
        self . _last = elem = self . _factory ( tag , attrs );
        if self . _elem {
        self . _elem [ -1 ] . append ( elem );
        } else if self . _root is None /* Option */ {
        self . _root = elem;
        self . _elem . append ( elem );
        self . _tail = 0;
        return  elem;
        pub fn end ( &self, tag )  {
        "Close && return current Element.

        *tag* == the element name.

        ";
        self . _flush ( );
        self . _last = self . _elem . pop ( );
        assert self . _last . tag == tag , \;
        "end tag mismatch (expected %s, got %s)" % (;
        self . _last . tag , tag );
        self . _tail = 1;
        return  self . _last;
        pub fn comment ( &self, text )  {
        "Create a comment using the comment_factory.

        *text* == the text of the comment.
        ";
        return  self . _handle_single (;
        self . _comment_factory , self . insert_comments , text );
        pub fn pi ( &self, target , text = None /* Option */ )  {
        "Create a processing instruction using the pi_factory.

        *target* == the target name of the processing instruction.
        *text* == the data of the processing instruction, || ''.
        ";
        return  self . _handle_single (;
        self . _pi_factory , self . insert_pis , target , text );
        pub fn _handle_single ( &self, factory , insert , * args )  {
        elem = factory ( * args );
        if insert {
        self . _flush ( );
        self . _last = elem;
        if self . _elem {
        self . _elem [ -1 ] . append ( elem );
        self . _tail = 1;
        return  elem;
        class XMLParser ;
        "Element structure builder for XML source data based on the expat parser.

    *target* == an optional target object which defaults to an instance of the
    standard TreeBuilder class, *encoding* == an optional encoding string
    which if given, overrides the encoding specified in the XML file:
    http://www.iana.org/assignments/character-sets

    ";
        pub fn __init__ ( &self, * , target = None /* Option */ , encoding = None /* Option */ )  {
        // try {
        from xml . parsers import expat;
        // } catch  ImportError  {
        // try {
        import pyexpat as expat;
        // } catch  ImportError  {
        panic!("ImportError (");
        "No module named expat; use SimpleXMLTreeBuilder instead";
        );
        parser = expat . ParserCreate ( encoding , "}" );
        if target is None /* Option */ {
        target = TreeBuilder ( );
        self . parser = self . _parser = parser;
        self . target = self . _target = target;
        self . _error = expat . error;
        self . _names = { };
        parser . DefaultHandlerExpand = self . _default;
        if hasattr ( target , "start" ) {
        parser . StartElementHandler = self . _start;
        if hasattr ( target , "end" ) {
        parser . EndElementHandler = self . _end;
        if hasattr ( target , "start_ns" ) {
        parser . StartNamespaceDeclHandler = self . _start_ns;
        if hasattr ( target , "end_ns" ) {
        parser . EndNamespaceDeclHandler = self . _end_ns;
        if hasattr ( target , "data" ) {
        parser . CharacterDataHandler = target . data;
        if hasattr ( target , "comment" ) {
        parser . CommentHandler = target . comment;
        if hasattr ( target , "pi" ) {
        parser . ProcessingInstructionHandler = target . pi;
        parser . buffer_text = 1;
        parser . ordered_attributes = 1;
        self . _doctype = None /* Option */;
        self . entity = { };
        // try {
        self . version = "Expat %d.%d.%d" % expat . version_info;
        // } catch  AttributeError  {
        // pass
        pub fn _setevents ( &self, events_queue , events_to_report )  {
        parser = self . _parser;
        append = events_queue . append;
        for event_name in events_to_report .iter() {
        if event_name == "start" {
        parser . ordered_attributes = 1;
        pub fn handler ( tag , attrib_in , event = event_name , append = append , {
        start = self . _start ) ;
        append ( ( event , start ( tag , attrib_in ) ) );
        parser . StartElementHandler = handler;
        } else if event_name == "end" {
        pub fn handler ( tag , event = event_name , append = append , {
        end = self . _end ) ;
        append ( ( event , end ( tag ) ) );
        parser . EndElementHandler = handler;
        } else if event_name == "start-ns" {
        if hasattr ( self . target , "start_ns" ) {
        pub fn handler ( prefix , uri , event = event_name , append = append , {
        start_ns = self . _start_ns ) ;
        append ( ( event , start_ns ( prefix , uri ) ) );
        } else {
        pub fn handler ( prefix , uri , event = event_name , append = append )  {
        append ( ( event , ( prefix || "" , uri || "" ) ) );
        parser . StartNamespaceDeclHandler = handler;
        } else if event_name == "end-ns" {
        if hasattr ( self . target , "end_ns" ) {
        pub fn handler ( prefix , event = event_name , append = append , {
        end_ns = self . _end_ns ) ;
        append ( ( event , end_ns ( prefix ) ) );
        } else {
        pub fn handler ( prefix , event = event_name , append = append )  {
        append ( ( event , None /* Option */ ) );
        parser . EndNamespaceDeclHandler = handler;
        } else if event_name == "comment" {
        pub fn handler ( text , event = event_name , append = append , self = self )  {
        append ( ( event , self . target . comment ( text ) ) );
        parser . CommentHandler = handler;
        } else if event_name == "pi" {
        pub fn handler ( pi_target , data , event = event_name , append = append , {
        self = self ) ;
        append ( ( event , self . target . pi ( pi_target , data ) ) );
        parser . ProcessingInstructionHandler = handler;
        } else {
        panic!("ValueError ( "unknown event %r" % event_name )");
        pub fn _raiseerror ( &self, value )  {
        err = ParseError ( value );
        err . code = value . code;
        err . position = value . lineno , value . offset;
        panic!("err");
        pub fn _fixname ( &self, key )  {
        // try {
        name = self . _names [ key ];
        // } catch  KeyError  {
        name = key;
        if "}" in name {
        name = "{" + name;
        self . _names [ key ] = name;
        return  name;
        pub fn _start_ns ( &self, prefix , uri )  {
        return  self . target . start_ns ( prefix || "" , uri || "" );
        pub fn _end_ns ( &self, prefix )  {
        return  self . target . end_ns ( prefix || "" );
        pub fn _start ( &self, tag , attr_list )  {
        fixname = self . _fixname;
        tag = fixname ( tag );
        attrib = { };
        if attr_list {
        for i in range ( 0 , len ( attr_list ) , 2 ) .iter() {
        attrib [ fixname ( attr_list [ i ] ) ] = attr_list [ i + 1 ];
        return  self . target . start ( tag , attrib );
        pub fn _end ( &self, tag )  {
        return  self . target . end ( self . _fixname ( tag ) );
        pub fn _default ( &self, text )  {
        prefix = text [ : 1 ];
        if prefix == "&" {
        // try {
        data_handler = self . target . data;
        // } catch  AttributeError  {
        return;
        // try {
        data_handler ( self . entity [ text [ 1 : -1 ] ] );
        // } catch  KeyError  {
        from xml . parsers import expat;
        err = expat . error (;
        "undefined entity %s: line %d, column %d" %;
        ( text , self . parser . ErrorLineNumber ,;
        self . parser . ErrorColumnNumber );
        );
        err . code = 11;
        err . lineno = self . parser . ErrorLineNumber;
        err . offset = self . parser . ErrorColumnNumber;
        panic!("err");
        } else if prefix == "<" && text [ {
        self . _doctype = [ ];
        } else if self . _doctype is !None /* Option */ {
        if prefix == ">" {
        self . _doctype = None /* Option */;
        return;
        text = text . strip ( );
        if !text {
        return;
        self . _doctype . append ( text );
        n = len ( self . _doctype );
        if n > 2 {
        type = self . _doctype [ 1 ];
        if type == "PUBLIC" && n == 4 {
        name , type , pubid , system = self . _doctype;
        if pubid {
        pubid = pubid [ 1 : -1 ];
        } else if type == "SYSTEM" && n == 3 {
        name , type , system = self . _doctype;
        pubid = None /* Option */;
        } else {
        return;
        if hasattr ( self . target , "doctype" ) {
        self . target . doctype ( name , pubid , system [ 1 : -1 ] );
        } else if hasattr ( self , "doctype" ) {
        warnings . warn (;
        "The doctype() method of XMLParser == ignored.  ";
        "Define doctype() method on the TreeBuilder target." ,;
        RuntimeWarning );
        self . _doctype = None /* Option */;
        pub fn feed ( &self, data )  {
        "Feed encoded data to parser.";
        // try {
        self . parser . Parse ( data , false );
        // } catch  self . _error as v  {
        self . _raiseerror ( v );
        pub fn close ( self )  {
        "Finish feeding data to parser && return element structure.";
        // try {
        self . parser . Parse ( b "" , true );
        // } catch  self . _error as v  {
        self . _raiseerror ( v );
        // try {
        close_handler = self . target . close;
        // } catch  AttributeError  {
        // pass
        } else {
        return  close_handler ( );
        // } finally {
        del self . parser , self . _parser;
        del self . target , self . _target;
        pub fn flush ( self )  {
        was_enabled = self . parser . GetReparseDeferralEnabled ( );
        // try {
        self . parser . SetReparseDeferralEnabled ( false );
        self . parser . Parse ( b "" , false );
        // } catch  self . _error as v  {
        self . _raiseerror ( v );
        // } finally {
        self . parser . SetReparseDeferralEnabled ( was_enabled );
        pub fn canonicalize ( xml_data = None /* Option */ , * , out = None /* Option */ , from_file = None /* Option */ , ** options )  {
        "Convert XML to its C14N 2.0 serialised form.

    If *out* == provided, it must be a file || file-like object that receives
    the serialised canonical XML output (text, !bytes) through its ``.write()``
    method.  To write to a file, open it in text mode with encoding "utf-8".
    If *out* == !provided, this function returns the output as text string.

    Either *xml_data* (an XML string) || *from_file* (a file path or
    file-like object) must be provided as input.

    The configuration options are the same as for the ``C14NWriterTarget``.
    ";
        if xml_data is None /* Option */ && from_file is None /* Option */ {
        panic!("ValueError ( "Either 'xml_data' || 'from_file' must be provided as input" )");
        sio = None /* Option */;
        if out is None /* Option */ {
        sio = out = io . StringIO ( );
        parser = XMLParser ( target = C14NWriterTarget ( out . write , ** options ) );
        if xml_data is !None /* Option */ {
        parser . feed ( xml_data );
        parser . close ( );
        } else if from_file is !None /* Option */ {
        parse ( from_file , parser = parser );
        return  sio . getvalue ( ) if sio is !None /* Option */ else None /* Option */;
        _looks_like_prefix_name = re . compile ( r "^\w+:\w+$" , re . UNICODE ) . match;
        class C14NWriterTarget ;
        "
    Canonicalization writer target for the XMLParser.

    Serialises parse events to XML C14N 2.0.

    The *write* function == used for writing out the resulting data stream
    as text (not bytes).  To write to a file, open it in text mode with encoding
    "utf-8" && pass its ``.write`` method.

    Configuration options:

    - *with_comments*: set to true to include comments
    - *strip_text*: set to true to strip whitespace before && after text content
    - *rewrite_prefixes*: set to true to replace namespace prefixes by "n{number}"
    - *qname_aware_tags*: a set of qname aware tag names in which prefixes
                          should be replaced in text content
    - *qname_aware_attrs*: a set of qname aware attribute names in which prefixes
                           should be replaced in text content
    - *exclude_attrs*: a set of attribute names that should !be serialised
    - *exclude_tags*: a set of tag names that should !be serialised
    ";
        pub fn __init__ ( &self, write , * , {
        with_comments = false , strip_text = false , rewrite_prefixes = false ,;
        qname_aware_tags = None /* Option */ , qname_aware_attrs = None /* Option */ ,;
        exclude_attrs = None /* Option */ , exclude_tags = None /* Option */ ) ;
        self . _write = write;
        self . _data = [ ];
        self . _with_comments = with_comments;
        self . _strip_text = strip_text;
        self . _exclude_attrs = set ( exclude_attrs ) if exclude_attrs else None /* Option */;
        self . _exclude_tags = set ( exclude_tags ) if exclude_tags else None /* Option */;
        self . _rewrite_prefixes = rewrite_prefixes;
        if qname_aware_tags {
        self . _qname_aware_tags = set ( qname_aware_tags );
        } else {
        self . _qname_aware_tags = None /* Option */;
        if qname_aware_attrs {
        self . _find_qname_aware_attrs = set ( qname_aware_attrs ) . intersection;
        } else {
        self . _find_qname_aware_attrs = None /* Option */;
        self . _declared_ns_stack = [ [;
        ( "http://www.w3.org/XML/1998/namespace" , "xml" ) ,;
        ] ];
        self . _ns_stack = [ ];
        if !rewrite_prefixes {
        self . _ns_stack . append ( list ( _namespace_map . items ( ) ) );
        self . _ns_stack . append ( [ ] );
        self . _prefix_map = { };
        self . _preserve_space = [ false ];
        self . _pending_start = None /* Option */;
        self . _root_seen = false;
        self . _root_done = false;
        self . _ignored_depth = 0;
        pub fn _iter_namespaces ( &self, ns_stack , _reversed = reversed )  {
        for namespaces in _reversed ( ns_stack ) .iter() {
        if namespaces {
        yield from namespaces;
        pub fn _resolve_prefix_name ( &self, prefixed_name )  {
        prefix , name = prefixed_name . split ( ":" , 1 );
        for uri , p in self . _iter_namespaces ( self . _ns_stack ) .iter() {
        if p == prefix {
        return  f "{{{uri}}}{name}";
        panic!("ValueError ( f "Prefix {prefix} of QName "{prefixed_name}" is !declared in scope" )");
        pub fn _qname ( &self, qname , uri = None /* Option */ )  {
        if uri is None /* Option */ {
        uri , tag = qname [ 1 : ] . rsplit ( "}" , 1 ) if qname [ : 1 ] == "{" else ( "" , qname );
        } else {
        tag = qname;
        prefixes_seen = set ( );
        for u , prefix in self . _iter_namespaces ( self . _declared_ns_stack ) .iter() {
        if u == uri && prefix !in prefixes_seen {
        return  f "{prefix}:{tag}" if prefix else tag , tag , uri;
        prefixes_seen . add ( prefix );
        if self . _rewrite_prefixes {
        if uri in self . _prefix_map {
        prefix = self . _prefix_map [ uri ];
        } else {
        prefix = self . _prefix_map [ uri ] = format!("n{len(self._prefix_map)}");
        self . _declared_ns_stack [ -1 ] . append ( ( uri , prefix ) );
        return  f "{prefix}:{tag}" , tag , uri;
        if !uri && "" !in prefixes_seen {
        return  tag , tag , uri;
        for u , prefix in self . _iter_namespaces ( self . _ns_stack ) .iter() {
        if u == uri {
        self . _declared_ns_stack [ -1 ] . append ( ( uri , prefix ) );
        return  f "{prefix}:{tag}" if prefix else tag , tag , uri;
        if !uri {
        return  tag , tag , uri;
        panic!("ValueError ( f "Namespace "{uri}" is !declared in scope" )");
        pub fn data ( &self, data )  {
        if !self . _ignored_depth {
        self . _data . append ( data );
        pub fn _flush ( &self, _join_text = "" . join )  {
        data = _join_text ( self . _data );
        del self . _data [ : ];
        if self . _strip_text && !self . _preserve_space [ -1 ] {
        data = data . strip ( );
        if self . _pending_start is !None /* Option */ {
        args , self . _pending_start = self . _pending_start , None /* Option */;
        qname_text = data if data && _looks_like_prefix_name ( data ) else None /* Option */;
        self . _start ( * args , qname_text );
        if qname_text is !None /* Option */ {
        return;
        if data && self . _root_seen {
        self . _write ( _escape_cdata_c14n ( data ) );
        pub fn start_ns ( &self, prefix , uri )  {
        if self . _ignored_depth {
        return;
        if self . _data {
        self . _flush ( );
        self . _ns_stack [ -1 ] . append ( ( uri , prefix ) );
        pub fn start ( &self, tag , attrs )  {
        if self . _exclude_tags is !None /* Option */ && ( {
        self . _ignored_depth || tag in self . _exclude_tags ) :;
        self . _ignored_depth + = 1;
        return;
        if self . _data {
        self . _flush ( );
        new_namespaces = [ ];
        self . _declared_ns_stack . append ( new_namespaces );
        if self . _qname_aware_tags is !None /* Option */ && tag in self . _qname_aware_tags {
        self . _pending_start = ( tag , attrs , new_namespaces );
        return;
        self . _start ( tag , attrs , new_namespaces );
        pub fn _start ( &self, tag , attrs , new_namespaces , qname_text = None /* Option */ )  {
        if self . _exclude_attrs is !None /* Option */ && attrs {
        attrs = { k : v for k , v in attrs . items ( ) if k !in self . _exclude_attrs };
        qnames = { tag , * attrs };
        resolved_names = { };
        if qname_text is !None /* Option */ {
        qname = resolved_names [ qname_text ] = self . _resolve_prefix_name ( qname_text );
        qnames . add ( qname );
        if self . _find_qname_aware_attrs is !None /* Option */ && attrs {
        qattrs = self . _find_qname_aware_attrs ( attrs );
        if qattrs {
        for attr_name in qattrs .iter() {
        value = attrs [ attr_name ];
        if _looks_like_prefix_name ( value ) {
        qname = resolved_names [ value ] = self . _resolve_prefix_name ( value );
        qnames . add ( qname );
        } else {
        qattrs = None /* Option */;
        } else {
        qattrs = None /* Option */;
        parse_qname = self . _qname;
        parsed_qnames = { n : parse_qname ( n ) for n in sorted (;
        qnames , key = |n | {  n . split ( "}" , 1 ) ) } };
        if new_namespaces {
        attr_list = [;
        ( "xmlns:" + prefix if prefix else "xmlns" , uri );
        for uri , prefix in new_namespaces.iter() {
        ];
        attr_list . sort ( );
        } else {
        attr_list = [ ];
        if attrs {
        for k , v in sorted ( attrs . items ( ) ) .iter() {
        if qattrs is !None /* Option */ && k in qattrs && v in resolved_names {
        v = parsed_qnames [ resolved_names [ v ] ] [ 0 ];
        attr_qname , attr_name , uri = parsed_qnames [ k ];
        attr_list . append ( ( attr_qname if uri else attr_name , v ) );
        space_behaviour = attrs . get ( "{http://www.w3.org/XML/1998/namespace}space" );
        self . _preserve_space . append (;
        space_behaviour == "preserve" if space_behaviour;
        else self . _preserve_space [ -1 ] );
        write = self . _write;
        write ( "<" + parsed_qnames [ tag ] [ 0 ] );
        if attr_list {
        write ( "" . join ( vec![ format!(" {k}="{_escape_attrib_c14n(v)}"".iter().map(|k , v| attr_list ] ) ));
        write ( ">" );
        if qname_text is !None /* Option */ {
        write ( _escape_cdata_c14n ( parsed_qnames [ resolved_names [ qname_text ] ] [ 0 ] ) );
        self . _root_seen = true;
        self . _ns_stack . append ( [ ] );
        pub fn end ( &self, tag )  {
        if self . _ignored_depth {
        self . _ignored_depth - = 1;
        return;
        if self . _data {
        self . _flush ( );
        self . _write ( f "</{self._qname(tag)[0]}>" );
        self . _preserve_space . pop ( );
        self . _root_done = len ( self . _preserve_space ) == 1;
        self . _declared_ns_stack . pop ( );
        self . _ns_stack . pop ( );
        pub fn comment ( &self, text )  {
        if !self . _with_comments {
        return;
        if self . _ignored_depth {
        return;
        if self . _root_done {
        self . _write ( "\n" );
        } else if self . _root_seen && self . _data {
        self . _flush ( );
        self . _write ( f "<!--{_escape_cdata_c14n(text)}-->" );
        if !self . _root_seen {
        self . _write ( "\n" );
        pub fn pi ( &self, target , data )  {
        if self . _ignored_depth {
        return;
        if self . _root_done {
        self . _write ( "\n" );
        } else if self . _root_seen && self . _data {
        self . _flush ( );
        self . _write (;
        format!("<?{target} {_escape_cdata_c14n(data)}?>" if data else format!("<?{target}?>" ));
        if !self . _root_seen {
        self . _write ( "\n" );
        pub fn _escape_cdata_c14n ( text )  {
        // try {
        if "&" in text {
        text = text . replace ( "&" , "&amp;" );
        if "<" in text {
        text = text . replace ( "<" , "&lt;" );
        if ">" in text {
        text = text . replace ( ">" , "&gt;" );
        if "\r" in text {
        text = text . replace ( "\r" , "&#xD;" );
        return  text;
        // } catch  ( TypeError , AttributeError )  {
        _raise_serialization_error ( text );
        pub fn _escape_attrib_c14n ( text )  {
        // try {
        if "&" in text {
        text = text . replace ( "&" , "&amp;" );
        if "<" in text {
        text = text . replace ( "<" , "&lt;" );
        if """ in text {
        text = text . replace ( """ , "&quot;" );
        if "\t" in text {
        text = text . replace ( "\t" , "&#x9;" );
        if "\n" in text {
        text = text . replace ( "\n" , "&#xA;" );
        if "\r" in text {
        text = text . replace ( "\r" , "&#xD;" );
        return  text;
        // } catch  ( TypeError , AttributeError )  {
        _raise_serialization_error ( text );
        // try {
        _Element_Py = Element;
        from _elementtree import *;
        from _elementtree import _set_factories;
        // } catch  ImportError  {
        // pass
        } else {
        _set_factories ( Comment , ProcessingInstruction );
}

