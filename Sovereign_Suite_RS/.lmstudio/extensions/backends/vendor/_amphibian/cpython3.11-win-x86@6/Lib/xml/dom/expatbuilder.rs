//! expatbuilder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::xml::{xmlbuilder, minidom, Node};

pub const TEXT_NODE: f64 = Node . TEXT_NODE;
pub const CDATA_SECTION_NODE: f64 = Node . CDATA_SECTION_NODE;
pub const DOCUMENT_NODE: f64 = Node . DOCUMENT_NODE;
pub const FILTER_ACCEPT: f64 = xmlbuilder . DOMBuilderFilter . FILTER_ACCEPT;
pub const FILTER_REJECT: f64 = xmlbuilder . DOMBuilderFilter . FILTER_REJECT;
pub const FILTER_SKIP: f64 = xmlbuilder . DOMBuilderFilter . FILTER_SKIP;
pub const FILTER_INTERRUPT: f64 = xmlbuilder . DOMBuilderFilter . FILTER_INTERRUPT;
pub const theDOMImplementation: f64 = minidom . getDOMImplementation ( );
pub const _typeinfo_map: f64 = {;
pub struct ElementInfo {
    pub tagName: String, // TODO: infer type
    pub _attr_info: String, // TODO: infer type
    pub _model: String, // TODO: infer type
    pub _options: String, // TODO: infer type
    pub _filter: String, // TODO: infer type
    pub _finish_start_element: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _intern_setdefault: String, // TODO: infer type
    pub document: String, // TODO: infer type
    pub curNode: String, // TODO: infer type
    pub _elem_info: String, // TODO: infer type
    pub _cdata: String, // TODO: infer type
    pub _finish_end_element: String, // TODO: infer type
    pub _cdata_continue: String, // TODO: infer type
    pub filter: String, // TODO: infer type
    pub _level: String, // TODO: infer type
    pub _builder: String, // TODO: infer type
    pub _old_start: String, // TODO: infer type
    pub _old_end: String, // TODO: infer type
    pub originalDocument: String, // TODO: infer type
    pub context: String, // TODO: infer type
    pub fragment: String, // TODO: infer type
    pub _source: String, // TODO: infer type
    pub _ns_ordered_prefixes: String, // TODO: infer type
    pub subset: String, // TODO: infer type
}

impl ElementInfo {
}

pub fn _intern(builder: &str, s: &str) {
        return  builder . _intern_setdefault ( s , s );
        pub fn _parse_ns_name ( builder , name )  {
        assert " " in name;
        parts = name . split ( " " );
        intern = builder . _intern_setdefault;
        if len ( parts ) == 3 {
        uri , localname , prefix = parts;
        prefix = intern ( prefix , prefix );
        qname = "%s:%s" % ( prefix , localname );
        qname = intern ( qname , qname );
        localname = intern ( localname , localname );
        } else if len ( parts ) == 2 {
        uri , localname = parts;
        prefix = EMPTY_PREFIX;
        qname = localname = intern ( localname , localname );
        } else {
        panic!("ValueError ( "Unsupported syntax: spaces in URIs !supported: %r" % name )");
        return  intern ( uri , uri ) , localname , prefix , qname;
        class ExpatBuilder ;
        "Document builder that uses Expat to build a ParsedXML.DOM document
    instance.";
        pub fn __init__ ( &self, options = None /* Option */ )  {
        if options is None /* Option */ {
        options = xmlbuilder . Options ( );
        self . _options = options;
        if self . _options . filter is !None /* Option */ {
        self . _filter = FilterVisibilityController ( self . _options . filter );
        } else {
        self . _filter = None /* Option */;
        self . _finish_start_element = id;
        self . _parser = None /* Option */;
        self . reset ( );
        pub fn createParser ( self )  {
        "Create a new parser object.";
        return  expat . ParserCreate ( );
        pub fn getParser ( self )  {
        "Return the parser object, creating a new one if needed.";
        if !self . _parser {
        self . _parser = self . createParser ( );
        self . _intern_setdefault = self . _parser . intern . setdefault;
        self . _parser . buffer_text = true;
        self . _parser . ordered_attributes = true;
        self . _parser . specified_attributes = true;
        self . install ( self . _parser );
        return  self . _parser;
        pub fn reset ( self )  {
        "Free all data structures used during DOM construction.";
        self . document = theDOMImplementation . createDocument (;
        EMPTY_NAMESPACE , None /* Option */ , None /* Option */ );
        self . curNode = self . document;
        self . _elem_info = self . document . _elem_info;
        self . _cdata = false;
        pub fn install ( &self, parser )  {
        "Install the callbacks needed to build the DOM into the parser.";
        parser . StartDoctypeDeclHandler = self . start_doctype_decl_handler;
        parser . StartElementHandler = self . first_element_handler;
        parser . EndElementHandler = self . end_element_handler;
        parser . ProcessingInstructionHandler = self . pi_handler;
        if self . _options . entities {
        parser . EntityDeclHandler = self . entity_decl_handler;
        parser . NotationDeclHandler = self . notation_decl_handler;
        if self . _options . comments {
        parser . CommentHandler = self . comment_handler;
        if self . _options . cdata_sections {
        parser . StartCdataSectionHandler = self . start_cdata_section_handler;
        parser . EndCdataSectionHandler = self . end_cdata_section_handler;
        parser . CharacterDataHandler = self . character_data_handler_cdata;
        } else {
        parser . CharacterDataHandler = self . character_data_handler;
        parser . ExternalEntityRefHandler = self . external_entity_ref_handler;
        parser . XmlDeclHandler = self . xml_decl_handler;
        parser . ElementDeclHandler = self . element_decl_handler;
        parser . AttlistDeclHandler = self . attlist_decl_handler;
        pub fn parseFile ( &self, file )  {
        "Parse a document from a file object, returning the document
        node.";
        parser = self . getParser ( );
        first_buffer = true;
        // try {
        while 1  {
        buffer = file . read ( 16 * 1024 );
        if !buffer {
        break;
        parser . Parse ( buffer , false );
        if first_buffer && self . document . documentElement {
        self . _setup_subset ( buffer );
        first_buffer = false;
        parser . Parse ( b "" , true );
        // } catch  ParseEscape  {
        // pass
        doc = self . document;
        self . reset ( );
        self . _parser = None /* Option */;
        return  doc;
        pub fn parseString ( &self, string )  {
        "Parse a document from a string, returning the document node.";
        parser = self . getParser ( );
        // try {
        parser . Parse ( string , true );
        self . _setup_subset ( string );
        // } catch  ParseEscape  {
        // pass
        doc = self . document;
        self . reset ( );
        self . _parser = None /* Option */;
        return  doc;
        pub fn _setup_subset ( &self, buffer )  {
        "Load the internal subset if there might be one.";
        if self . document . doctype {
        extractor = InternalSubsetExtractor ( );
        extractor . parseString ( buffer );
        subset = extractor . getSubset ( );
        self . document . doctype . internalSubset = subset;
        pub fn start_doctype_decl_handler ( &self, doctypeName , systemId , publicId , {
        has_internal_subset ) ;
        doctype = self . document . implementation . createDocumentType (;
        doctypeName , publicId , systemId );
        doctype . ownerDocument = self . document;
        _append_child ( self . document , doctype );
        self . document . doctype = doctype;
        if self . _filter && self . _filter . acceptNode ( doctype ) == FILTER_REJECT {
        self . document . doctype = None /* Option */;
        del self . document . childNodes [ -1 ];
        doctype = None /* Option */;
        self . _parser . EntityDeclHandler = None /* Option */;
        self . _parser . NotationDeclHandler = None /* Option */;
        if has_internal_subset {
        if doctype is !None /* Option */ {
        doctype . entities . _seq = [ ];
        doctype . notations . _seq = [ ];
        self . _parser . CommentHandler = None /* Option */;
        self . _parser . ProcessingInstructionHandler = None /* Option */;
        self . _parser . EndDoctypeDeclHandler = self . end_doctype_decl_handler;
        pub fn end_doctype_decl_handler ( self )  {
        if self . _options . comments {
        self . _parser . CommentHandler = self . comment_handler;
        self . _parser . ProcessingInstructionHandler = self . pi_handler;
        if !( self . _elem_info || self . _filter ) {
        self . _finish_end_element = id;
        pub fn pi_handler ( &self, target , data )  {
        node = self . document . createProcessingInstruction ( target , data );
        _append_child ( self . curNode , node );
        if self . _filter && self . _filter . acceptNode ( node ) == FILTER_REJECT {
        self . curNode . removeChild ( node );
        pub fn character_data_handler_cdata ( &self, data )  {
        childNodes = self . curNode . childNodes;
        if self . _cdata {
        if ( self . _cdata_continue {
        and childNodes [ -1 ] . nodeType == CDATA_SECTION_NODE ) ;
        childNodes [ -1 ] . appendData ( data );
        return;
        node = self . document . createCDATASection ( data );
        self . _cdata_continue = true;
        } else if childNodes && childNodes [ -1 ] . nodeType == TEXT_NODE {
        node = childNodes [ -1 ];
        value = node . data + data;
        node . data = value;
        return;
        } else {
        node = minidom . Text ( );
        node . data = data;
        node . ownerDocument = self . document;
        _append_child ( self . curNode , node );
        pub fn character_data_handler ( &self, data )  {
        childNodes = self . curNode . childNodes;
        if childNodes && childNodes [ -1 ] . nodeType == TEXT_NODE {
        node = childNodes [ -1 ];
        node . data = node . data + data;
        return;
        node = minidom . Text ( );
        node . data = node . data + data;
        node . ownerDocument = self . document;
        _append_child ( self . curNode , node );
        pub fn entity_decl_handler ( &self, entityName , is_parameter_entity , value , {
        base , systemId , publicId , notationName ) ;
        if is_parameter_entity {
        return;
        if !self . _options . entities {
        return;
        node = self . document . _create_entity ( entityName , publicId ,;
        systemId , notationName );
        if value is !None /* Option */ {
        child = self . document . createTextNode ( value );
        node . childNodes . append ( child );
        self . document . doctype . entities . _seq . append ( node );
        if self . _filter && self . _filter . acceptNode ( node ) == FILTER_REJECT {
        del self . document . doctype . entities . _seq [ -1 ];
        pub fn notation_decl_handler ( &self, notationName , base , systemId , publicId )  {
        node = self . document . _create_notation ( notationName , publicId , systemId );
        self . document . doctype . notations . _seq . append ( node );
        if self . _filter && self . _filter . acceptNode ( node ) == FILTER_ACCEPT {
        del self . document . doctype . notations . _seq [ -1 ];
        pub fn comment_handler ( &self, data )  {
        node = self . document . createComment ( data );
        _append_child ( self . curNode , node );
        if self . _filter && self . _filter . acceptNode ( node ) == FILTER_REJECT {
        self . curNode . removeChild ( node );
        pub fn start_cdata_section_handler ( self )  {
        self . _cdata = true;
        self . _cdata_continue = false;
        pub fn end_cdata_section_handler ( self )  {
        self . _cdata = false;
        self . _cdata_continue = false;
        pub fn external_entity_ref_handler ( &self, context , base , systemId , publicId )  {
        return  1;
        pub fn first_element_handler ( &self, name , attributes )  {
        if self . _filter is None /* Option */ && !self . _elem_info {
        self . _finish_end_element = id;
        self . getParser ( ) . StartElementHandler = self . start_element_handler;
        self . start_element_handler ( name , attributes );
        pub fn start_element_handler ( &self, name , attributes )  {
        node = self . document . createElement ( name );
        _append_child ( self . curNode , node );
        self . curNode = node;
        if attributes {
        for i in range ( 0 , len ( attributes ) , 2 ) .iter() {
        a = minidom . Attr ( attributes [ i ] , EMPTY_NAMESPACE ,;
        None /* Option */ , EMPTY_PREFIX );
        value = attributes [ i + 1 ];
        a . value = value;
        a . ownerDocument = self . document;
        _set_attribute_node ( node , a );
        if node is !self . document . documentElement {
        self . _finish_start_element ( node );
        pub fn _finish_start_element ( &self, node )  {
        if self . _filter {
        if node is self . document . documentElement {
        return;
        filt = self . _filter . startContainer ( node );
        if filt == FILTER_REJECT {
        Rejecter ( self );
        } else if filt == FILTER_SKIP {
        Skipper ( self );
        } else {
        return;
        self . curNode = node . parentNode;
        node . parentNode . removeChild ( node );
        node . unlink ( );
        pub fn end_element_handler ( &self, name )  {
        curNode = self . curNode;
        self . curNode = curNode . parentNode;
        self . _finish_end_element ( curNode );
        pub fn _finish_end_element ( &self, curNode )  {
        info = self . _elem_info . get ( curNode . tagName );
        if info {
        self . _handle_white_text_nodes ( curNode , info );
        if self . _filter {
        if curNode is self . document . documentElement {
        return;
        if self . _filter . acceptNode ( curNode ) == FILTER_REJECT {
        self . curNode . removeChild ( curNode );
        curNode . unlink ( );
        pub fn _handle_white_text_nodes ( &self, node , info )  {
        if ( self . _options . whitespace_in_element_content {
        or !info . isElementContent ( ) ) ;
        return;
        L = [ ];
        for child in node . childNodes .iter() {
        if child . nodeType == TEXT_NODE && !child . data . strip ( ) {
        L . append ( child );
        for child in L .iter() {
        node . removeChild ( child );
        pub fn element_decl_handler ( &self, name , model )  {
        info = self . _elem_info . get ( name );
        if info is None /* Option */ {
        self . _elem_info [ name ] = ElementInfo ( name , model );
        } else {
        assert info . _model == None /* Option */;
        info . _model = model;
        pub fn attlist_decl_handler ( &self, elem , name , type , default , required )  {
        info = self . _elem_info . get ( elem );
        if info is None /* Option */ {
        info = ElementInfo ( elem );
        self . _elem_info [ elem ] = info;
        info . _attr_info . append (;
        [ None /* Option */ , name , None /* Option */ , None /* Option */ , default , 0 , type , required ] );
        pub fn xml_decl_handler ( &self, version , encoding , standalone )  {
        self . document . version = version;
        self . document . encoding = encoding;
        if standalone >= 0 {
        if standalone {
        self . document . standalone = true;
        } else {
        self . document . standalone = false;
        _ALLOWED_FILTER_RETURNS = ( FILTER_ACCEPT , FILTER_REJECT , FILTER_SKIP );
        class FilterVisibilityController ( object ) ;
        "Wrapper around a DOMBuilderFilter which implements the checks
    to make the whatToShow filter attribute work.";
        __slots__ = "filter" ,;
        pub fn __init__ ( &self, filter )  {
        self . filter = filter;
        pub fn startContainer ( &self, node )  {
        mask = self . _nodetype_mask [ node . nodeType ];
        if self . filter . whatToShow & mask {
        val = self . filter . startContainer ( node );
        if val == FILTER_INTERRUPT {
        panic!("ParseEscape");
        if val !in _ALLOWED_FILTER_RETURNS {
        panic!("ValueError (");
        "startContainer() returned illegal value: " + repr ( val ) );
        return  val;
        } else {
        return  FILTER_ACCEPT;
        pub fn acceptNode ( &self, node )  {
        mask = self . _nodetype_mask [ node . nodeType ];
        if self . filter . whatToShow & mask {
        val = self . filter . acceptNode ( node );
        if val == FILTER_INTERRUPT {
        panic!("ParseEscape");
        if val == FILTER_SKIP {
        parent = node . parentNode;
        for child in node . childNodes [ : ] .iter() {
        parent . appendChild ( child );
        return  FILTER_REJECT;
        if val !in _ALLOWED_FILTER_RETURNS {
        panic!("ValueError (");
        "acceptNode() returned illegal value: " + repr ( val ) );
        return  val;
        } else {
        return  FILTER_ACCEPT;
        _nodetype_mask = {;
        Node . ELEMENT_NODE : NodeFilter . SHOW_ELEMENT ,;
        Node . ATTRIBUTE_NODE : NodeFilter . SHOW_ATTRIBUTE ,;
        Node . TEXT_NODE : NodeFilter . SHOW_TEXT ,;
        Node . CDATA_SECTION_NODE : NodeFilter . SHOW_CDATA_SECTION ,;
        Node . ENTITY_REFERENCE_NODE : NodeFilter . SHOW_ENTITY_REFERENCE ,;
        Node . ENTITY_NODE : NodeFilter . SHOW_ENTITY ,;
        Node . PROCESSING_INSTRUCTION_NODE : NodeFilter . SHOW_PROCESSING_INSTRUCTION ,;
        Node . COMMENT_NODE : NodeFilter . SHOW_COMMENT ,;
        Node . DOCUMENT_NODE : NodeFilter . SHOW_DOCUMENT ,;
        Node . DOCUMENT_TYPE_NODE : NodeFilter . SHOW_DOCUMENT_TYPE ,;
        Node . DOCUMENT_FRAGMENT_NODE : NodeFilter . SHOW_DOCUMENT_FRAGMENT ,;
        Node . NOTATION_NODE : NodeFilter . SHOW_NOTATION ,;
        };
        class FilterCrutch ( object ) ;
        __slots__ = "_builder" , "_level" , "_old_start" , "_old_end";
        pub fn __init__ ( &self, builder )  {
        self . _level = 0;
        self . _builder = builder;
        parser = builder . _parser;
        self . _old_start = parser . StartElementHandler;
        self . _old_end = parser . EndElementHandler;
        parser . StartElementHandler = self . start_element_handler;
        parser . EndElementHandler = self . end_element_handler;
        class Rejecter ( FilterCrutch ) ;
        __slots__ = ( );
        pub fn __init__ ( &self, builder )  {
        FilterCrutch . __init__ ( self , builder );
        parser = builder . _parser;
        for name in ( "ProcessingInstructionHandler" ,.iter() {
        "CommentHandler" ,;
        "CharacterDataHandler" ,;
        "StartCdataSectionHandler" ,;
        "EndCdataSectionHandler" ,;
        "ExternalEntityRefHandler" ,;
        ) ;
        setattr ( parser , name , None /* Option */ );
        pub fn start_element_handler ( &self, * args )  {
        self . _level = self . _level + 1;
        pub fn end_element_handler ( &self, * args )  {
        if self . _level == 0 {
        parser = self . _builder . _parser;
        self . _builder . install ( parser );
        parser . StartElementHandler = self . _old_start;
        parser . EndElementHandler = self . _old_end;
        } else {
        self . _level = self . _level - 1;
        class Skipper ( FilterCrutch ) ;
        __slots__ = ( );
        pub fn start_element_handler ( &self, * args )  {
        node = self . _builder . curNode;
        self . _old_start ( * args );
        if self . _builder . curNode is !node {
        self . _level = self . _level + 1;
        pub fn end_element_handler ( &self, * args )  {
        if self . _level == 0 {
        self . _builder . _parser . StartElementHandler = self . _old_start;
        self . _builder . _parser . EndElementHandler = self . _old_end;
        self . _builder = None /* Option */;
        } else {
        self . _level = self . _level - 1;
        self . _old_end ( * args );
        _FRAGMENT_BUILDER_INTERNAL_SYSTEM_ID = \;
        "http://xml.python.org/entities/fragment-builder/internal";
        _FRAGMENT_BUILDER_TEMPLATE = (;
        "\
<!DOCTYPE wrapper
  %%s [
  <!ENTITY fragment-builder-internal
    SYSTEM "%s">
%%s
]>
<wrapper %%s
>&fragment-builder-internal;</wrapper>";
        % _FRAGMENT_BUILDER_INTERNAL_SYSTEM_ID );
        class FragmentBuilder ( ExpatBuilder ) ;
        "Builder which constructs document fragments given XML source
    text && a context node.

    The context node == expected to provide information about the
    namespace declarations which are in scope at the start of the
    fragment.
    ";
        pub fn __init__ ( &self, context , options = None /* Option */ )  {
        if context . nodeType == DOCUMENT_NODE {
        self . originalDocument = context;
        self . context = context;
        } else {
        self . originalDocument = context . ownerDocument;
        self . context = context;
        ExpatBuilder . __init__ ( self , options );
        pub fn reset ( self )  {
        ExpatBuilder . reset ( self );
        self . fragment = None /* Option */;
        pub fn parseFile ( &self, file )  {
        "Parse a document fragment from a file object, returning the
        fragment node.";
        return  self . parseString ( file . read ( ) );
        pub fn parseString ( &self, string )  {
        "Parse a document fragment from a string, returning the
        fragment node.";
        self . _source = string;
        parser = self . getParser ( );
        doctype = self . originalDocument . doctype;
        ident = "";
        if doctype {
        subset = doctype . internalSubset || self . _getDeclarations ( );
        if doctype . publicId {
        ident = ( "PUBLIC "%s" "%s"";
        % ( doctype . publicId , doctype . systemId ) );
        } else if doctype . systemId {
        ident = "SYSTEM "%s"" % doctype . systemId;
        } else {
        subset = "";
        nsattrs = self . _getNSattrs ( );
        document = _FRAGMENT_BUILDER_TEMPLATE % ( ident , subset , nsattrs );
        // try {
        parser . Parse ( document , true );
        // } catch   {
        self . reset ( );
        panic!("");
        fragment = self . fragment;
        self . reset ( );
        return  fragment;
        pub fn _getDeclarations ( self )  {
        "Re-create the internal subset from the DocumentType node.

        This == only needed if we don't already have the
        internalSubset as a string.
        ";
        doctype = self . context . ownerDocument . doctype;
        s = "";
        if doctype {
        for i in range ( doctype . notations . length ) .iter() {
        notation = doctype . notations . item ( i );
        if s {
        s = s + "\n  ";
        s = "%s<!NOTATION %s" % ( s , notation . nodeName );
        if notation . publicId {
        s = "%s PUBLIC "%s"\n             "%s">" \;
        % ( s , notation . publicId , notation . systemId );
        } else {
        s = "%s SYSTEM "%s">" % ( s , notation . systemId );
        for i in range ( doctype . entities . length ) .iter() {
        entity = doctype . entities . item ( i );
        if s {
        s = s + "\n  ";
        s = "%s<!ENTITY %s" % ( s , entity . nodeName );
        if entity . publicId {
        s = "%s PUBLIC "%s"\n             "%s"" \;
        % ( s , entity . publicId , entity . systemId );
        } else if entity . systemId {
        s = "%s SYSTEM "%s"" % ( s , entity . systemId );
        } else {
        s = "%s "%s"" % ( s , entity . firstChild . data );
        if entity . notationName {
        s = "%s NOTATION %s" % ( s , entity . notationName );
        s = s + ">";
        return  s;
        pub fn _getNSattrs ( self )  {
        return  "";
        pub fn external_entity_ref_handler ( &self, context , base , systemId , publicId )  {
        if systemId == _FRAGMENT_BUILDER_INTERNAL_SYSTEM_ID {
        old_document = self . document;
        old_cur_node = self . curNode;
        parser = self . _parser . ExternalEntityParserCreate ( context );
        self . document = self . originalDocument;
        self . fragment = self . document . createDocumentFragment ( );
        self . curNode = self . fragment;
        // try {
        parser . Parse ( self . _source , true );
        // } finally {
        self . curNode = old_cur_node;
        self . document = old_document;
        self . _source = None /* Option */;
        return  -1;
        } else {
        return  ExpatBuilder . external_entity_ref_handler (;
        self , context , base , systemId , publicId );
        class Namespaces ;
        "Mix-in class for builders; adds support for namespaces.";
        pub fn _initNamespaces ( self )  {
        self . _ns_ordered_prefixes = [ ];
        pub fn createParser ( self )  {
        "Create a new namespace-handling parser.";
        parser = expat . ParserCreate ( namespace_separator = " " );
        parser . namespace_prefixes = true;
        return  parser;
        pub fn install ( &self, parser )  {
        "Insert the namespace-handlers onto the parser.";
        ExpatBuilder . install ( self , parser );
        if self . _options . namespace_declarations {
        parser . StartNamespaceDeclHandler = (;
        self . start_namespace_decl_handler );
        pub fn start_namespace_decl_handler ( &self, prefix , uri )  {
        "Push this namespace declaration on our storage.";
        self . _ns_ordered_prefixes . append ( ( prefix , uri ) );
        pub fn start_element_handler ( &self, name , attributes )  {
        if " " in name {
        uri , localname , prefix , qname = _parse_ns_name ( self , name );
        } else {
        uri = EMPTY_NAMESPACE;
        qname = name;
        localname = None /* Option */;
        prefix = EMPTY_PREFIX;
        node = minidom . Element ( qname , uri , prefix , localname );
        node . ownerDocument = self . document;
        _append_child ( self . curNode , node );
        self . curNode = node;
        if self . _ns_ordered_prefixes {
        for prefix , uri in self . _ns_ordered_prefixes .iter() {
        if prefix {
        a = minidom . Attr ( _intern ( self , "xmlns:" + prefix ) ,;
        XMLNS_NAMESPACE , prefix , "xmlns" );
        } else {
        a = minidom . Attr ( "xmlns" , XMLNS_NAMESPACE ,;
        "xmlns" , EMPTY_PREFIX );
        a . value = uri;
        a . ownerDocument = self . document;
        _set_attribute_node ( node , a );
        del self . _ns_ordered_prefixes [ : ];
        if attributes {
        node . _ensure_attributes ( );
        _attrs = node . _attrs;
        _attrsNS = node . _attrsNS;
        for i in range ( 0 , len ( attributes ) , 2 ) .iter() {
        aname = attributes [ i ];
        value = attributes [ i + 1 ];
        if " " in aname {
        uri , localname , prefix , qname = _parse_ns_name ( self , aname );
        a = minidom . Attr ( qname , uri , localname , prefix );
        _attrs [ qname ] = a;
        _attrsNS [ ( uri , localname ) ] = a;
        } else {
        a = minidom . Attr ( aname , EMPTY_NAMESPACE ,;
        aname , EMPTY_PREFIX );
        _attrs [ aname ] = a;
        _attrsNS [ ( EMPTY_NAMESPACE , aname ) ] = a;
        a . ownerDocument = self . document;
        a . value = value;
        a . ownerElement = node;
        if __debug__ {
        pub fn end_element_handler ( &self, name )  {
        curNode = self . curNode;
        if " " in name {
        uri , localname , prefix , qname = _parse_ns_name ( self , name );
        assert ( curNode . namespaceURI == uri;
        and curNode . localName == localname;
        and curNode . prefix == prefix ) , \;
        "element stack messed up! (namespace)";
        } else {
        assert curNode . nodeName == name , \;
        "element stack messed up - bad nodeName";
        assert curNode . namespaceURI == EMPTY_NAMESPACE , \;
        "element stack messed up - bad namespaceURI";
        self . curNode = curNode . parentNode;
        self . _finish_end_element ( curNode );
        class ExpatBuilderNS ( Namespaces , ExpatBuilder ) ;
        "Document builder that supports namespaces.";
        pub fn reset ( self )  {
        ExpatBuilder . reset ( self );
        self . _initNamespaces ( );
        class FragmentBuilderNS ( Namespaces , FragmentBuilder ) ;
        "Fragment builder that supports namespaces.";
        pub fn reset ( self )  {
        FragmentBuilder . reset ( self );
        self . _initNamespaces ( );
        pub fn _getNSattrs ( self )  {
        "Return string of namespace attributes from this element and
        ancestors.";
        attrs = "";
        context = self . context;
        L = [ ];
        while context  {
        if hasattr ( context , "_ns_prefix_uri" ) {
        for prefix , uri in context . _ns_prefix_uri . items ( ) .iter() {
        if prefix in L {
        continue;
        L . append ( prefix );
        if prefix {
        declname = "xmlns:" + prefix;
        } else {
        declname = "xmlns";
        if attrs {
        attrs = "%s\n    %s='%s'" % ( attrs , declname , uri );
        } else {
        attrs = " %s='%s'" % ( declname , uri );
        context = context . parentNode;
        return  attrs;
        class ParseEscape ( Exception ) ;
        "Exception raised to short-circuit parsing in InternalSubsetExtractor.";
        // pass
        class InternalSubsetExtractor ( ExpatBuilder ) ;
        "XML processor which can rip out the internal document type subset.";
        subset = None /* Option */;
        pub fn getSubset ( self )  {
        "Return the internal subset as a string.";
        return  self . subset;
        pub fn parseFile ( &self, file )  {
        // try {
        ExpatBuilder . parseFile ( self , file );
        // } catch  ParseEscape  {
        // pass
        pub fn parseString ( &self, string )  {
        // try {
        ExpatBuilder . parseString ( self , string );
        // } catch  ParseEscape  {
        // pass
        pub fn install ( &self, parser )  {
        parser . StartDoctypeDeclHandler = self . start_doctype_decl_handler;
        parser . StartElementHandler = self . start_element_handler;
        pub fn start_doctype_decl_handler ( &self, name , publicId , systemId , {
        has_internal_subset ) ;
        if has_internal_subset {
        parser = self . getParser ( );
        self . subset = [ ];
        parser . DefaultHandler = self . subset . append;
        parser . EndDoctypeDeclHandler = self . end_doctype_decl_handler;
        } else {
        panic!("ParseEscape ( )");
        pub fn end_doctype_decl_handler ( self )  {
        s = "" . join ( self . subset ) . replace ( "\r\n" , "\n" ) . replace ( "\r" , "\n" );
        self . subset = s;
        panic!("ParseEscape ( )");
        pub fn start_element_handler ( &self, name , attrs )  {
        panic!("ParseEscape ( )");
        pub fn parse ( file , namespaces = true )  {
        "Parse a document, returning the resulting Document node.

    'file' may be either a file name || an open file object.
    ";
        if namespaces {
        builder = ExpatBuilderNS ( );
        } else {
        builder = ExpatBuilder ( );
        if isinstance ( file , str ) {
        // with scope: open ( file , "rb" ) as fp  {
        result = builder . parseFile ( fp );
        } else {
        result = builder . parseFile ( file );
        return  result;
        pub fn parseString ( string , namespaces = true )  {
        "Parse a document from a string, returning the resulting
    Document node.
    ";
        if namespaces {
        builder = ExpatBuilderNS ( );
        } else {
        builder = ExpatBuilder ( );
        return  builder . parseString ( string );
        pub fn parseFragment ( file , context , namespaces = true )  {
        "Parse a fragment of a document, given the context from which it
    was originally extracted.  context should be the parent of the
    node(s) which are in the fragment.

    'file' may be either a file name || an open file object.
    ";
        if namespaces {
        builder = FragmentBuilderNS ( context );
        } else {
        builder = FragmentBuilder ( context );
        if isinstance ( file , str ) {
        // with scope: open ( file , "rb" ) as fp  {
        result = builder . parseFile ( fp );
        } else {
        result = builder . parseFile ( file );
        return  result;
        pub fn parseFragmentString ( string , context , namespaces = true )  {
        "Parse a fragment of a document from a string, given the context
    from which it was originally extracted.  context should be the
    parent of the node(s) which are in the fragment.
    ";
        if namespaces {
        builder = FragmentBuilderNS ( context );
        } else {
        builder = FragmentBuilder ( context );
        return  builder . parseString ( string );
        pub fn makeBuilder ( options )  {
        "Create a builder based on an Options object.";
        if options . namespaces {
        return  ExpatBuilderNS ( options );
        } else {
        return  ExpatBuilder ( options );
}

