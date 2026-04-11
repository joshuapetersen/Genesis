//! minidom.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::io;
// use crate::xml::{EMPTY_NAMESPACE, EMPTY_PREFIX, XMLNS_NAMESPACE, domreg};

pub const _nodeTypes_with_children: f64 = ( xml . dom . Node . ELEMENT_NODE ,;
pub struct Node {
    pub _user_data: String, // TODO: infer type
    pub parentNode: String, // TODO: infer type
    pub ownerDocument: String, // TODO: infer type
    pub childNodes: String, // TODO: infer type
    pub previousSibling: String, // TODO: infer type
    pub nextSibling: String, // TODO: infer type
    pub ownerElement: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub namespaceURI: String, // TODO: infer type
    pub _prefix: String, // TODO: infer type
    pub _localName: String, // TODO: infer type
    pub _value: String, // TODO: infer type
    pub name: String, // TODO: infer type
    pub _is_id: String, // TODO: infer type
    pub _attrs: String, // TODO: infer type
    pub _attrsNS: String, // TODO: infer type
    pub _ownerElement: String, // TODO: infer type
    pub namespace: String, // TODO: infer type
    pub tagName: String, // TODO: infer type
    pub nodeName: String, // TODO: infer type
    pub prefix: String, // TODO: infer type
    pub target: String, // TODO: infer type
    pub data: String, // TODO: infer type
    pub _data: String, // TODO: infer type
    pub _seq: String, // TODO: infer type
    pub publicId: String, // TODO: infer type
    pub systemId: String, // TODO: infer type
    pub entities: String, // TODO: infer type
    pub notations: String, // TODO: infer type
    pub notationName: String, // TODO: infer type
    pub doctype: String, // TODO: infer type
    pub _elem_info: String, // TODO: infer type
    pub _id_cache: String, // TODO: infer type
    pub _id_search_stack: String, // TODO: infer type
    pub documentElement: String, // TODO: infer type
}

impl Node {
}

pub const doc: &str = "First child node, or None." );
pub const doc: &str = "Last child node, or None." );
pub const doc: &str = "Namespace-local name of this node." );
pub fn _append_child(node: &str) {
        childNodes = self . childNodes;
        if childNodes {
        last = childNodes [ -1 ];
        node . previousSibling = last;
        last . nextSibling = node;
        childNodes . append ( node );
        node . parentNode = self;
        pub fn _in_document ( node )  {
        while node is !None /* Option */  {
        if node . nodeType == Node . DOCUMENT_NODE {
        return  true;
        node = node . parentNode;
        return  false;
        pub fn _write_data ( writer , data )  {
        "Writes datachars to writer.";
        if data {
        data = data . replace ( "&" , "&amp;" ) . replace ( "<" , "&lt;" ) . \;
        replace ( "\"" , "&quot;" ) . replace ( ">" , "&gt;" );
        writer . write ( data );
        pub fn _get_elements_by_tagName_helper ( parent , name , rc )  {
        for node in parent . childNodes .iter() {
        if node . nodeType == Node . ELEMENT_NODE && \ {
        ( name == "*" || node . tagName == name ) ;
        rc . append ( node );
        _get_elements_by_tagName_helper ( node , name , rc );
        return  rc;
        pub fn _get_elements_by_tagName_ns_helper ( parent , nsURI , localName , rc )  {
        for node in parent . childNodes .iter() {
        if node . nodeType == Node . ELEMENT_NODE {
        if ( ( localName == "*" || node . localName == localName ) and {
        ( nsURI == "*" || node . namespaceURI == nsURI ) ) ;
        rc . append ( node );
        _get_elements_by_tagName_ns_helper ( node , nsURI , localName , rc );
        return  rc;
        class DocumentFragment ( Node ) ;
        nodeType = Node . DOCUMENT_FRAGMENT_NODE;
        nodeName = "#document-fragment";
        nodeValue = None /* Option */;
        attributes = None /* Option */;
        parentNode = None /* Option */;
        _child_node_types = ( Node . ELEMENT_NODE ,;
        Node . TEXT_NODE ,;
        Node . CDATA_SECTION_NODE ,;
        Node . ENTITY_REFERENCE_NODE ,;
        Node . PROCESSING_INSTRUCTION_NODE ,;
        Node . COMMENT_NODE ,;
        Node . NOTATION_NODE );
        pub fn __init__ ( self )  {
        self . childNodes = NodeList ( );
        class Attr ( Node ) ;
        __slots__ = ( "_name" , "_value" , "namespaceURI" ,;
        "_prefix" , "childNodes" , "_localName" , "ownerDocument" , "ownerElement" );
        nodeType = Node . ATTRIBUTE_NODE;
        attributes = None /* Option */;
        specified = false;
        _is_id = false;
        _child_node_types = ( Node . TEXT_NODE , Node . ENTITY_REFERENCE_NODE );
        pub fn __init__ ( &self, qName , namespaceURI = EMPTY_NAMESPACE , localName = None /* Option */ , {
        prefix = None /* Option */ ) ;
        self . ownerElement = None /* Option */;
        self . _name = qName;
        self . namespaceURI = namespaceURI;
        self . _prefix = prefix;
        if localName is !None /* Option */ {
        self . _localName = localName;
        self . childNodes = NodeList ( );
        self . childNodes . append ( Text ( ) );
        pub fn _get_localName ( self )  {
        // try {
        return  self . _localName;
        // } catch  AttributeError  {
        return  self . nodeName . split ( ":" , 1 ) [ -1 ];
        pub fn _get_specified ( self )  {
        return  self . specified;
        pub fn _get_name ( self )  {
        return  self . _name;
        pub fn _set_name ( &self, value )  {
        self . _name = value;
        if self . ownerElement is !None /* Option */ {
        _clear_id_cache ( self . ownerElement );
        nodeName = name = property ( _get_name , _set_name );
        pub fn _get_value ( self )  {
        return  self . _value;
        pub fn _set_value ( &self, value )  {
        self . _value = value;
        self . childNodes [ 0 ] . data = value;
        if self . ownerElement is !None /* Option */ {
        _clear_id_cache ( self . ownerElement );
        self . childNodes [ 0 ] . data = value;
        nodeValue = value = property ( _get_value , _set_value );
        pub fn _get_prefix ( self )  {
        return  self . _prefix;
        pub fn _set_prefix ( &self, prefix )  {
        nsuri = self . namespaceURI;
        if prefix == "xmlns" {
        if nsuri && nsuri != XMLNS_NAMESPACE {
        panic!("xml . dom . NamespaceErr (");
        "illegal use of 'xmlns' prefix for the wrong namespace" );
        self . _prefix = prefix;
        if prefix is None /* Option */ {
        newName = self . localName;
        } else {
        newName = "%s:%s" % ( prefix , self . localName );
        if self . ownerElement {
        _clear_id_cache ( self . ownerElement );
        self . name = newName;
        prefix = property ( _get_prefix , _set_prefix );
        pub fn unlink ( self )  {
        elem = self . ownerElement;
        if elem is !None /* Option */ {
        del elem . _attrs [ self . nodeName ];
        del elem . _attrsNS [ ( self . namespaceURI , self . localName ) ];
        if self . _is_id {
        self . _is_id = false;
        elem . _magic_id_nodes - = 1;
        self . ownerDocument . _magic_id_count - = 1;
        for child in self . childNodes .iter() {
        child . unlink ( );
        del self . childNodes [ : ];
        pub fn _get_isId ( self )  {
        if self . _is_id {
        return  true;
        doc = self . ownerDocument;
        elem = self . ownerElement;
        if doc is None /* Option */ || elem is None /* Option */ {
        return  false;
        info = doc . _get_elem_info ( elem );
        if info is None /* Option */ {
        return  false;
        if self . namespaceURI {
        return  info . isIdNS ( self . namespaceURI , self . localName );
        } else {
        return  info . isId ( self . nodeName );
        pub fn _get_schemaType ( self )  {
        doc = self . ownerDocument;
        elem = self . ownerElement;
        if doc is None /* Option */ || elem is None /* Option */ {
        return  _no_type;
        info = doc . _get_elem_info ( elem );
        if info is None /* Option */ {
        return  _no_type;
        if self . namespaceURI {
        return  info . getAttributeTypeNS ( self . namespaceURI , self . localName );
        } else {
        return  info . getAttributeType ( self . nodeName );
        defproperty ( Attr , "isId" , doc = "true if this attribute == an ID." );
        defproperty ( Attr , "localName" , doc = "Namespace-local name of this attribute." );
        defproperty ( Attr , "schemaType" , doc = "Schema type for this attribute." );
        class NamedNodeMap ( object ) ;
        "The attribute list == a transient interface to the underlying
    dictionaries.  Mutations here will change the underlying element's
    dictionary.

    Ordering == imposed artificially && does !reflect the order of
    attributes as found in an input document.
    ";
        __slots__ = ( "_attrs" , "_attrsNS" , "_ownerElement" );
        pub fn __init__ ( &self, attrs , attrsNS , ownerElement )  {
        self . _attrs = attrs;
        self . _attrsNS = attrsNS;
        self . _ownerElement = ownerElement;
        pub fn _get_length ( self )  {
        return  len ( self . _attrs );
        pub fn item ( &self, index )  {
        // try {
        return  self [ list ( self . _attrs . keys ( ) ) [ index ] ];
        // } catch  IndexError  {
        return;
        pub fn items ( self )  {
        L = [ ];
        for node in self . _attrs . values ( ) .iter() {
        L . append ( ( node . nodeName , node . value ) );
        return  L;
        pub fn itemsNS ( self )  {
        L = [ ];
        for node in self . _attrs . values ( ) .iter() {
        L . append ( ( ( node . namespaceURI , node . localName ) , node . value ) );
        return  L;
        pub fn __contains__ ( &self, key )  {
        if isinstance ( key , str ) {
        return  key in self . _attrs;
        } else {
        return  key in self . _attrsNS;
        pub fn keys ( self )  {
        return  self . _attrs . keys ( );
        pub fn keysNS ( self )  {
        return  self . _attrsNS . keys ( );
        pub fn values ( self )  {
        return  self . _attrs . values ( );
        pub fn get ( &self, name , value = None /* Option */ )  {
        return  self . _attrs . get ( name , value );
        __len__ = _get_length;
        pub fn _cmp ( &self, other )  {
        if self . _attrs is getattr ( other , "_attrs" , None /* Option */ ) {
        return  0;
        } else {
        return  ( id ( self ) > id ( other ) ) - ( id ( self ) < id ( other ) );
        pub fn __eq__ ( &self, other )  {
        return  self . _cmp ( other ) == 0;
        pub fn __ge__ ( &self, other )  {
        return  self . _cmp ( other ) >= 0;
        pub fn __gt__ ( &self, other )  {
        return  self . _cmp ( other ) > 0;
        pub fn __le__ ( &self, other )  {
        return  self . _cmp ( other ) <= 0;
        pub fn __lt__ ( &self, other )  {
        return  self . _cmp ( other ) < 0;
        pub fn __getitem__ ( &self, attname_or_tuple )  {
        if isinstance ( attname_or_tuple , tuple ) {
        return  self . _attrsNS [ attname_or_tuple ];
        } else {
        return  self . _attrs [ attname_or_tuple ];
        pub fn __setitem__ ( &self, attname , value )  {
        if isinstance ( value , str ) {
        // try {
        node = self . _attrs [ attname ];
        // } catch  KeyError  {
        node = Attr ( attname );
        node . ownerDocument = self . _ownerElement . ownerDocument;
        self . setNamedItem ( node );
        node . value = value;
        } else {
        if !isinstance ( value , Attr ) {
        panic!("TypeError ( "value must be a string || Attr object" )");
        node = value;
        self . setNamedItem ( node );
        pub fn getNamedItem ( &self, name )  {
        // try {
        return  self . _attrs [ name ];
        // } catch  KeyError  {
        return;
        pub fn getNamedItemNS ( &self, namespaceURI , localName )  {
        // try {
        return  self . _attrsNS [ ( namespaceURI , localName ) ];
        // } catch  KeyError  {
        return;
        pub fn removeNamedItem ( &self, name )  {
        n = self . getNamedItem ( name );
        if n is !None /* Option */ {
        _clear_id_cache ( self . _ownerElement );
        del self . _attrs [ n . nodeName ];
        del self . _attrsNS [ ( n . namespaceURI , n . localName ) ];
        if hasattr ( n , "ownerElement" ) {
        n . ownerElement = None /* Option */;
        return  n;
        } else {
        panic!("xml . dom . NotFoundErr ( )");
        pub fn removeNamedItemNS ( &self, namespaceURI , localName )  {
        n = self . getNamedItemNS ( namespaceURI , localName );
        if n is !None /* Option */ {
        _clear_id_cache ( self . _ownerElement );
        del self . _attrsNS [ ( n . namespaceURI , n . localName ) ];
        del self . _attrs [ n . nodeName ];
        if hasattr ( n , "ownerElement" ) {
        n . ownerElement = None /* Option */;
        return  n;
        } else {
        panic!("xml . dom . NotFoundErr ( )");
        pub fn setNamedItem ( &self, node )  {
        if !isinstance ( node , Attr ) {
        panic!("xml . dom . HierarchyRequestErr (");
        "%s cannot be child of %s" % ( repr ( node ) , repr ( self ) ) );
        old = self . _attrs . get ( node . name );
        if old {
        old . unlink ( );
        self . _attrs [ node . name ] = node;
        self . _attrsNS [ ( node . namespaceURI , node . localName ) ] = node;
        node . ownerElement = self . _ownerElement;
        _clear_id_cache ( node . ownerElement );
        return  old;
        pub fn setNamedItemNS ( &self, node )  {
        return  self . setNamedItem ( node );
        pub fn __delitem__ ( &self, attname_or_tuple )  {
        node = self [ attname_or_tuple ];
        _clear_id_cache ( node . ownerElement );
        node . unlink ( );
        pub fn __getstate__ ( self )  {
        return  self . _attrs , self . _attrsNS , self . _ownerElement;
        pub fn __setstate__ ( &self, state )  {
        self . _attrs , self . _attrsNS , self . _ownerElement = state;
        defproperty ( NamedNodeMap , "length" ,;
        doc = "Number of nodes in the NamedNodeMap." );
        AttributeList = NamedNodeMap;
        class TypeInfo ( object ) ;
        __slots__ = "namespace" , "name";
        pub fn __init__ ( &self, namespace , name )  {
        self . namespace = namespace;
        self . name = name;
        pub fn __repr__ ( self )  {
        if self . namespace {
        return  "<%s %r (from %r)>" % ( self . __class__ . __name__ , self . name ,;
        self . namespace );
        } else {
        return  "<%s %r>" % ( self . __class__ . __name__ , self . name );
        pub fn _get_name ( self )  {
        return  self . name;
        pub fn _get_namespace ( self )  {
        return  self . namespace;
        _no_type = TypeInfo ( None /* Option */ , None /* Option */ );
        class Element ( Node ) ;
        __slots__ = ( "ownerDocument" , "parentNode" , "tagName" , "nodeName" , "prefix" ,;
        "namespaceURI" , "_localName" , "childNodes" , "_attrs" , "_attrsNS" ,;
        "nextSibling" , "previousSibling" );
        nodeType = Node . ELEMENT_NODE;
        nodeValue = None /* Option */;
        schemaType = _no_type;
        _magic_id_nodes = 0;
        _child_node_types = ( Node . ELEMENT_NODE ,;
        Node . PROCESSING_INSTRUCTION_NODE ,;
        Node . COMMENT_NODE ,;
        Node . TEXT_NODE ,;
        Node . CDATA_SECTION_NODE ,;
        Node . ENTITY_REFERENCE_NODE );
        pub fn __init__ ( &self, tagName , namespaceURI = EMPTY_NAMESPACE , prefix = None /* Option */ , {
        localName = None /* Option */ ) ;
        self . parentNode = None /* Option */;
        self . tagName = self . nodeName = tagName;
        self . prefix = prefix;
        self . namespaceURI = namespaceURI;
        self . childNodes = NodeList ( );
        self . nextSibling = self . previousSibling = None /* Option */;
        self . _attrs = None /* Option */;
        self . _attrsNS = None /* Option */;
        pub fn _ensure_attributes ( self )  {
        if self . _attrs is None /* Option */ {
        self . _attrs = { };
        self . _attrsNS = { };
        pub fn _get_localName ( self )  {
        // try {
        return  self . _localName;
        // } catch  AttributeError  {
        return  self . tagName . split ( ":" , 1 ) [ -1 ];
        pub fn _get_tagName ( self )  {
        return  self . tagName;
        pub fn unlink ( self )  {
        if self . _attrs is !None /* Option */ {
        for attr in list ( self . _attrs . values ( ) ) .iter() {
        attr . unlink ( );
        self . _attrs = None /* Option */;
        self . _attrsNS = None /* Option */;
        Node . unlink ( self );
        pub fn getAttribute ( &self, attname )  {
        "Returns the value of the specified attribute.

        Returns the value of the element's attribute named attname as
        a string. An empty string == returned if the element does not
        have such an attribute. Note that an empty string may also be
        returned as an explicitly given attribute value, use the
        hasAttribute method to distinguish these two cases.
        ";
        if self . _attrs is None /* Option */ {
        return  "";
        // try {
        return  self . _attrs [ attname ] . value;
        // } catch  KeyError  {
        return  "";
        pub fn getAttributeNS ( &self, namespaceURI , localName )  {
        if self . _attrsNS is None /* Option */ {
        return  "";
        // try {
        return  self . _attrsNS [ ( namespaceURI , localName ) ] . value;
        // } catch  KeyError  {
        return  "";
        pub fn setAttribute ( &self, attname , value )  {
        attr = self . getAttributeNode ( attname );
        if attr is None /* Option */ {
        attr = Attr ( attname );
        attr . value = value;
        attr . ownerDocument = self . ownerDocument;
        self . setAttributeNode ( attr );
        } else if value != attr . value {
        attr . value = value;
        if attr . isId {
        _clear_id_cache ( self );
        pub fn setAttributeNS ( &self, namespaceURI , qualifiedName , value )  {
        prefix , localname = _nssplit ( qualifiedName );
        attr = self . getAttributeNodeNS ( namespaceURI , localname );
        if attr is None /* Option */ {
        attr = Attr ( qualifiedName , namespaceURI , localname , prefix );
        attr . value = value;
        attr . ownerDocument = self . ownerDocument;
        self . setAttributeNode ( attr );
        } else {
        if value != attr . value {
        attr . value = value;
        if attr . isId {
        _clear_id_cache ( self );
        if attr . prefix != prefix {
        attr . prefix = prefix;
        attr . nodeName = qualifiedName;
        pub fn getAttributeNode ( &self, attrname )  {
        if self . _attrs is None /* Option */ {
        return;
        return  self . _attrs . get ( attrname );
        pub fn getAttributeNodeNS ( &self, namespaceURI , localName )  {
        if self . _attrsNS is None /* Option */ {
        return;
        return  self . _attrsNS . get ( ( namespaceURI , localName ) );
        pub fn setAttributeNode ( &self, attr )  {
        if attr . ownerElement !in ( None /* Option */ , self ) {
        panic!("xml . dom . InuseAttributeErr ( "attribute node already owned" )");
        self . _ensure_attributes ( );
        old1 = self . _attrs . get ( attr . name , None /* Option */ );
        if old1 is !None /* Option */ {
        self . removeAttributeNode ( old1 );
        old2 = self . _attrsNS . get ( ( attr . namespaceURI , attr . localName ) , None /* Option */ );
        if old2 is !None /* Option */ && old2 is !old1 {
        self . removeAttributeNode ( old2 );
        _set_attribute_node ( self , attr );
        if old1 is !attr {
        return  old1;
        if old2 is !attr {
        return  old2;
        setAttributeNodeNS = setAttributeNode;
        pub fn removeAttribute ( &self, name )  {
        if self . _attrsNS is None /* Option */ {
        panic!("xml . dom . NotFoundErr ( )");
        // try {
        attr = self . _attrs [ name ];
        // } catch  KeyError  {
        panic!("xml . dom . NotFoundErr ( )");
        self . removeAttributeNode ( attr );
        pub fn removeAttributeNS ( &self, namespaceURI , localName )  {
        if self . _attrsNS is None /* Option */ {
        panic!("xml . dom . NotFoundErr ( )");
        // try {
        attr = self . _attrsNS [ ( namespaceURI , localName ) ];
        // } catch  KeyError  {
        panic!("xml . dom . NotFoundErr ( )");
        self . removeAttributeNode ( attr );
        pub fn removeAttributeNode ( &self, node )  {
        if node is None /* Option */ {
        panic!("xml . dom . NotFoundErr ( )");
        // try {
        self . _attrs [ node . name ];
        // } catch  KeyError  {
        panic!("xml . dom . NotFoundErr ( )");
        _clear_id_cache ( self );
        node . unlink ( );
        node . ownerDocument = self . ownerDocument;
        return  node;
        removeAttributeNodeNS = removeAttributeNode;
        pub fn hasAttribute ( &self, name )  {
        "Checks whether the element has an attribute with the specified name.

        Returns true if the element has an attribute with the specified name.
        Otherwise, returns false.
        ";
        if self . _attrs is None /* Option */ {
        return  false;
        return  name in self . _attrs;
        pub fn hasAttributeNS ( &self, namespaceURI , localName )  {
        if self . _attrsNS is None /* Option */ {
        return  false;
        return  ( namespaceURI , localName ) in self . _attrsNS;
        pub fn getElementsByTagName ( &self, name )  {
        "Returns all descendant elements with the given tag name.

        Returns the list of all descendant elements (not direct children
        only) with the specified tag name.
        ";
        return  _get_elements_by_tagName_helper ( self , name , NodeList ( ) );
        pub fn getElementsByTagNameNS ( &self, namespaceURI , localName )  {
        return  _get_elements_by_tagName_ns_helper (;
        self , namespaceURI , localName , NodeList ( ) );
        pub fn __repr__ ( self )  {
        return  "<DOM Element: %s at %#x>" % ( self . tagName , id ( self ) );
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        "Write an XML element to a file-like object

        Write the element to the writer object that must provide
        a write method (e.g. a file || StringIO object).
        ";
        writer . write ( indent + "<" + self . tagName );
        attrs = self . _get_attributes ( );
        for a_name in attrs . keys ( ) .iter() {
        writer . write ( " %s=\"" % a_name );
        _write_data ( writer , attrs [ a_name ] . value );
        writer . write ( "\"" );
        if self . childNodes {
        writer . write ( ">" );
        if ( len ( self . childNodes ) == 1 and {
        self . childNodes [ 0 ] . nodeType in (;
        Node . TEXT_NODE , Node . CDATA_SECTION_NODE ) ) ;
        self . childNodes [ 0 ] . writexml ( writer , "" , "" , "" );
        } else {
        writer . write ( newl );
        for node in self . childNodes .iter() {
        node . writexml ( writer , indent + addindent , addindent , newl );
        writer . write ( indent );
        writer . write ( "</%s>%s" % ( self . tagName , newl ) );
        } else {
        writer . write ( "/>%s" % ( newl ) );
        pub fn _get_attributes ( self )  {
        self . _ensure_attributes ( );
        return  NamedNodeMap ( self . _attrs , self . _attrsNS , self );
        pub fn hasAttributes ( self )  {
        if self . _attrs {
        return  true;
        } else {
        return  false;
        pub fn setIdAttribute ( &self, name )  {
        idAttr = self . getAttributeNode ( name );
        self . setIdAttributeNode ( idAttr );
        pub fn setIdAttributeNS ( &self, namespaceURI , localName )  {
        idAttr = self . getAttributeNodeNS ( namespaceURI , localName );
        self . setIdAttributeNode ( idAttr );
        pub fn setIdAttributeNode ( &self, idAttr )  {
        if idAttr is None /* Option */ || !self . isSameNode ( idAttr . ownerElement ) {
        panic!("xml . dom . NotFoundErr ( )");
        if _get_containing_entref ( self ) is !None /* Option */ {
        panic!("xml . dom . NoModificationAllowedErr ( )");
        if !idAttr . _is_id {
        idAttr . _is_id = true;
        self . _magic_id_nodes + = 1;
        self . ownerDocument . _magic_id_count + = 1;
        _clear_id_cache ( self );
        defproperty ( Element , "attributes" ,;
        doc = "NamedNodeMap of attributes on the element." );
        defproperty ( Element , "localName" ,;
        doc = "Namespace-local name of this element." );
        pub fn _set_attribute_node ( element , attr )  {
        _clear_id_cache ( element );
        element . _ensure_attributes ( );
        element . _attrs [ attr . name ] = attr;
        element . _attrsNS [ ( attr . namespaceURI , attr . localName ) ] = attr;
        attr . ownerElement = element;
        class Childless ;
        "Mixin that makes childless-ness easy to implement && avoids
    the complexity of the Node methods that deal with children.
    ";
        __slots__ = ( );
        attributes = None /* Option */;
        childNodes = EmptyNodeList ( );
        firstChild = None /* Option */;
        lastChild = None /* Option */;
        pub fn _get_firstChild ( self )  {
        return;
        pub fn _get_lastChild ( self )  {
        return;
        pub fn appendChild ( &self, node )  {
        panic!("xml . dom . HierarchyRequestErr (");
        self . nodeName + " nodes cannot have children" );
        pub fn hasChildNodes ( self )  {
        return  false;
        pub fn insertBefore ( &self, newChild , refChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        self . nodeName + " nodes do !have children" );
        pub fn removeChild ( &self, oldChild )  {
        panic!("xml . dom . NotFoundErr (");
        self . nodeName + " nodes do !have children" );
        pub fn normalize ( self )  {
        // pass
        pub fn replaceChild ( &self, newChild , oldChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        self . nodeName + " nodes do !have children" );
        class ProcessingInstruction ( Childless , Node ) ;
        nodeType = Node . PROCESSING_INSTRUCTION_NODE;
        __slots__ = ( "target" , "data" );
        pub fn __init__ ( &self, target , data )  {
        self . target = target;
        self . data = data;
        pub fn _get_nodeValue ( self )  {
        return  self . data;
        pub fn _set_nodeValue ( &self, value )  {
        self . data = value;
        nodeValue = property ( _get_nodeValue , _set_nodeValue );
        pub fn _get_nodeName ( self )  {
        return  self . target;
        pub fn _set_nodeName ( &self, value )  {
        self . target = value;
        nodeName = property ( _get_nodeName , _set_nodeName );
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        writer . write ( "%s<?%s %s?>%s" % ( indent , self . target , self . data , newl ) );
        class CharacterData ( Childless , Node ) ;
        __slots__ = ( "_data" , "ownerDocument" , "parentNode" , "previousSibling" , "nextSibling" );
        pub fn __init__ ( self )  {
        self . ownerDocument = self . parentNode = None /* Option */;
        self . previousSibling = self . nextSibling = None /* Option */;
        self . _data = "";
        Node . __init__ ( self );
        pub fn _get_length ( self )  {
        return  len ( self . data );
        __len__ = _get_length;
        pub fn _get_data ( self )  {
        return  self . _data;
        pub fn _set_data ( &self, data )  {
        self . _data = data;
        data = nodeValue = property ( _get_data , _set_data );
        pub fn __repr__ ( self )  {
        data = self . data;
        if len ( data ) > 10 {
        dotdotdot = "...";
        } else {
        dotdotdot = "";
        return  "<DOM %s node "%r%s">" % (;
        self . __class__ . __name__ , data [ 0 : 10 ] , dotdotdot );
        pub fn substringData ( &self, offset , count )  {
        if offset < 0 {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be negative" )");
        if offset >= len ( self . data ) {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be beyond end of data" )");
        if count < 0 {
        panic!("xml . dom . IndexSizeErr ( "count cannot be negative" )");
        return  self . data [ offset : offset + count ];
        pub fn appendData ( &self, arg )  {
        self . data = self . data + arg;
        pub fn insertData ( &self, offset , arg )  {
        if offset < 0 {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be negative" )");
        if offset >= len ( self . data ) {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be beyond end of data" )");
        if arg {
        self . data = "%s%s%s" % (;
        self . data [ : offset ] , arg , self . data [ offset : ] );
        pub fn deleteData ( &self, offset , count )  {
        if offset < 0 {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be negative" )");
        if offset >= len ( self . data ) {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be beyond end of data" )");
        if count < 0 {
        panic!("xml . dom . IndexSizeErr ( "count cannot be negative" )");
        if count {
        self . data = self . data [ : offset ] + self . data [ offset + count : ];
        pub fn replaceData ( &self, offset , count , arg )  {
        if offset < 0 {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be negative" )");
        if offset >= len ( self . data ) {
        panic!("xml . dom . IndexSizeErr ( "offset cannot be beyond end of data" )");
        if count < 0 {
        panic!("xml . dom . IndexSizeErr ( "count cannot be negative" )");
        if count {
        self . data = "%s%s%s" % (;
        self . data [ : offset ] , arg , self . data [ offset + count : ] );
        defproperty ( CharacterData , "length" , doc = "Length of the string data." );
        class Text ( CharacterData ) ;
        __slots__ = ( );
        nodeType = Node . TEXT_NODE;
        nodeName = "#text";
        attributes = None /* Option */;
        pub fn splitText ( &self, offset )  {
        if offset < 0 || offset > len ( self . data ) {
        panic!("xml . dom . IndexSizeErr ( "illegal offset value" )");
        newText = self . __class__ ( );
        newText . data = self . data [ offset : ];
        newText . ownerDocument = self . ownerDocument;
        next = self . nextSibling;
        if self . parentNode && self in self . parentNode . childNodes {
        if next is None /* Option */ {
        self . parentNode . appendChild ( newText );
        } else {
        self . parentNode . insertBefore ( newText , next );
        self . data = self . data [ : offset ];
        return  newText;
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        _write_data ( writer , "%s%s%s" % ( indent , self . data , newl ) );
        pub fn _get_wholeText ( self )  {
        L = [ self . data ];
        n = self . previousSibling;
        while n is !None /* Option */  {
        if n . nodeType in ( Node . TEXT_NODE , Node . CDATA_SECTION_NODE ) {
        L . insert ( 0 , n . data );
        n = n . previousSibling;
        } else {
        break;
        n = self . nextSibling;
        while n is !None /* Option */  {
        if n . nodeType in ( Node . TEXT_NODE , Node . CDATA_SECTION_NODE ) {
        L . append ( n . data );
        n = n . nextSibling;
        } else {
        break;
        return  "" . join ( L );
        pub fn replaceWholeText ( &self, content )  {
        parent = self . parentNode;
        n = self . previousSibling;
        while n is !None /* Option */  {
        if n . nodeType in ( Node . TEXT_NODE , Node . CDATA_SECTION_NODE ) {
        next = n . previousSibling;
        parent . removeChild ( n );
        n = next;
        } else {
        break;
        n = self . nextSibling;
        if !content {
        parent . removeChild ( self );
        while n is !None /* Option */  {
        if n . nodeType in ( Node . TEXT_NODE , Node . CDATA_SECTION_NODE ) {
        next = n . nextSibling;
        parent . removeChild ( n );
        n = next;
        } else {
        break;
        if content {
        self . data = content;
        return  self;
        } else {
        return;
        pub fn _get_isWhitespaceInElementContent ( self )  {
        if self . data . strip ( ) {
        return  false;
        elem = _get_containing_element ( self );
        if elem is None /* Option */ {
        return  false;
        info = self . ownerDocument . _get_elem_info ( elem );
        if info is None /* Option */ {
        return  false;
        } else {
        return  info . isElementContent ( );
        defproperty ( Text , "isWhitespaceInElementContent" ,;
        doc = "true iff this text node contains only whitespace";
        " && == in element content." );
        defproperty ( Text , "wholeText" ,;
        doc = "The text of all logically-adjacent text nodes." );
        pub fn _get_containing_element ( node )  {
        c = node . parentNode;
        while c is !None /* Option */  {
        if c . nodeType == Node . ELEMENT_NODE {
        return  c;
        c = c . parentNode;
        return;
        pub fn _get_containing_entref ( node )  {
        c = node . parentNode;
        while c is !None /* Option */  {
        if c . nodeType == Node . ENTITY_REFERENCE_NODE {
        return  c;
        c = c . parentNode;
        return;
        class Comment ( CharacterData ) ;
        nodeType = Node . COMMENT_NODE;
        nodeName = "#comment";
        pub fn __init__ ( &self, data )  {
        CharacterData . __init__ ( self );
        self . _data = data;
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        if "--" in self . data {
        panic!("ValueError ( "'--' is !allowed in a comment node" )");
        writer . write ( "%s<!--%s-->%s" % ( indent , self . data , newl ) );
        class CDATASection ( Text ) ;
        __slots__ = ( );
        nodeType = Node . CDATA_SECTION_NODE;
        nodeName = "#cdata-section";
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        if self . data . find ( "]]>" ) >= 0 {
        panic!("ValueError ( "']]>' !allowed in a CDATA section" )");
        writer . write ( "<![CDATA[%s]]>" % self . data );
        class ReadOnlySequentialNamedNodeMap ( object ) ;
        __slots__ = "_seq" ,;
        pub fn __init__ ( &self, seq = ( ) )  {
        self . _seq = seq;
        pub fn __len__ ( self )  {
        return  len ( self . _seq );
        pub fn _get_length ( self )  {
        return  len ( self . _seq );
        pub fn getNamedItem ( &self, name )  {
        for n in self . _seq .iter() {
        if n . nodeName == name {
        return  n;
        pub fn getNamedItemNS ( &self, namespaceURI , localName )  {
        for n in self . _seq .iter() {
        if n . namespaceURI == namespaceURI && n . localName == localName {
        return  n;
        pub fn __getitem__ ( &self, name_or_tuple )  {
        if isinstance ( name_or_tuple , tuple ) {
        node = self . getNamedItemNS ( * name_or_tuple );
        } else {
        node = self . getNamedItem ( name_or_tuple );
        if node is None /* Option */ {
        panic!("KeyError ( name_or_tuple )");
        return  node;
        pub fn item ( &self, index )  {
        if index < 0 {
        return;
        // try {
        return  self . _seq [ index ];
        // } catch  IndexError  {
        return;
        pub fn removeNamedItem ( &self, name )  {
        panic!("xml . dom . NoModificationAllowedErr (");
        "NamedNodeMap instance == read-only" );
        pub fn removeNamedItemNS ( &self, namespaceURI , localName )  {
        panic!("xml . dom . NoModificationAllowedErr (");
        "NamedNodeMap instance == read-only" );
        pub fn setNamedItem ( &self, node )  {
        panic!("xml . dom . NoModificationAllowedErr (");
        "NamedNodeMap instance == read-only" );
        pub fn setNamedItemNS ( &self, node )  {
        panic!("xml . dom . NoModificationAllowedErr (");
        "NamedNodeMap instance == read-only" );
        pub fn __getstate__ ( self )  {
        return  [ self . _seq ];
        pub fn __setstate__ ( &self, state )  {
        self . _seq = state [ 0 ];
        defproperty ( ReadOnlySequentialNamedNodeMap , "length" ,;
        doc = "Number of entries in the NamedNodeMap." );
        class Identified ;
        "Mix-in class that supports the publicId && systemId attributes.";
        __slots__ = "publicId" , "systemId";
        pub fn _identified_mixin_init ( &self, publicId , systemId )  {
        self . publicId = publicId;
        self . systemId = systemId;
        pub fn _get_publicId ( self )  {
        return  self . publicId;
        pub fn _get_systemId ( self )  {
        return  self . systemId;
        class DocumentType ( Identified , Childless , Node ) ;
        nodeType = Node . DOCUMENT_TYPE_NODE;
        nodeValue = None /* Option */;
        name = None /* Option */;
        publicId = None /* Option */;
        systemId = None /* Option */;
        internalSubset = None /* Option */;
        pub fn __init__ ( &self, qualifiedName )  {
        self . entities = ReadOnlySequentialNamedNodeMap ( );
        self . notations = ReadOnlySequentialNamedNodeMap ( );
        if qualifiedName {
        prefix , localname = _nssplit ( qualifiedName );
        self . name = localname;
        self . nodeName = self . name;
        pub fn _get_internalSubset ( self )  {
        return  self . internalSubset;
        pub fn cloneNode ( &self, deep )  {
        if self . ownerDocument is None /* Option */ {
        clone = DocumentType ( None /* Option */ );
        clone . name = self . name;
        clone . nodeName = self . name;
        operation = xml . dom . UserDataHandler . NODE_CLONED;
        if deep {
        clone . entities . _seq = [ ];
        clone . notations . _seq = [ ];
        for n in self . notations . _seq .iter() {
        notation = Notation ( n . nodeName , n . publicId , n . systemId );
        clone . notations . _seq . append ( notation );
        n . _call_user_data_handler ( operation , n , notation );
        for e in self . entities . _seq .iter() {
        entity = Entity ( e . nodeName , e . publicId , e . systemId ,;
        e . notationName );
        entity . actualEncoding = e . actualEncoding;
        entity . encoding = e . encoding;
        entity . version = e . version;
        clone . entities . _seq . append ( entity );
        e . _call_user_data_handler ( operation , e , entity );
        self . _call_user_data_handler ( operation , self , clone );
        return  clone;
        } else {
        return;
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" )  {
        writer . write ( "<!DOCTYPE " );
        writer . write ( self . name );
        if self . publicId {
        writer . write ( "%s  PUBLIC '%s'%s  '%s'";
        % ( newl , self . publicId , newl , self . systemId ) );
        } else if self . systemId {
        writer . write ( "%s  SYSTEM '%s'" % ( newl , self . systemId ) );
        if self . internalSubset is !None /* Option */ {
        writer . write ( " [" );
        writer . write ( self . internalSubset );
        writer . write ( "]" );
        writer . write ( ">" + newl );
        class Entity ( Identified , Node ) ;
        attributes = None /* Option */;
        nodeType = Node . ENTITY_NODE;
        nodeValue = None /* Option */;
        actualEncoding = None /* Option */;
        encoding = None /* Option */;
        version = None /* Option */;
        pub fn __init__ ( &self, name , publicId , systemId , notation )  {
        self . nodeName = name;
        self . notationName = notation;
        self . childNodes = NodeList ( );
        self . _identified_mixin_init ( publicId , systemId );
        pub fn _get_actualEncoding ( self )  {
        return  self . actualEncoding;
        pub fn _get_encoding ( self )  {
        return  self . encoding;
        pub fn _get_version ( self )  {
        return  self . version;
        pub fn appendChild ( &self, newChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        "cannot append children to an entity node" );
        pub fn insertBefore ( &self, newChild , refChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        "cannot insert children below an entity node" );
        pub fn removeChild ( &self, oldChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        "cannot remove children from an entity node" );
        pub fn replaceChild ( &self, newChild , oldChild )  {
        panic!("xml . dom . HierarchyRequestErr (");
        "cannot replace children of an entity node" );
        class Notation ( Identified , Childless , Node ) ;
        nodeType = Node . NOTATION_NODE;
        nodeValue = None /* Option */;
        pub fn __init__ ( &self, name , publicId , systemId )  {
        self . nodeName = name;
        self . _identified_mixin_init ( publicId , systemId );
        class DOMImplementation ( DOMImplementationLS ) ;
        _features = [ ( "core" , "1.0" ) ,;
        ( "core" , "2.0" ) ,;
        ( "core" , None /* Option */ ) ,;
        ( "xml" , "1.0" ) ,;
        ( "xml" , "2.0" ) ,;
        ( "xml" , None /* Option */ ) ,;
        ( "ls-load" , "3.0" ) ,;
        ( "ls-load" , None /* Option */ ) ,;
        ];
        pub fn hasFeature ( &self, feature , version )  {
        if version == "" {
        version = None /* Option */;
        return  ( feature . lower ( ) , version ) in self . _features;
        pub fn createDocument ( &self, namespaceURI , qualifiedName , doctype )  {
        if doctype && doctype . parentNode is !None /* Option */ {
        panic!("xml . dom . WrongDocumentErr (");
        "doctype object owned by another DOM tree" );
        doc = self . _create_document ( );
        add_root_element = !( namespaceURI == None /* Option */;
        and qualifiedName == None /* Option */;
        and doctype == None /* Option */ );
        if !qualifiedName && add_root_element {
        panic!("xml . dom . InvalidCharacterErr ( "Element with no name" )");
        if add_root_element {
        prefix , localname = _nssplit ( qualifiedName );
        if prefix == "xml" \ {
        and namespaceURI != "http://www.w3.org/XML/1998/namespace" ;
        panic!("xml . dom . NamespaceErr ( "illegal use of 'xml' prefix" )");
        if prefix && !namespaceURI {
        panic!("xml . dom . NamespaceErr (");
        "illegal use of prefix without namespaces" );
        element = doc . createElementNS ( namespaceURI , qualifiedName );
        if doctype {
        doc . appendChild ( doctype );
        doc . appendChild ( element );
        if doctype {
        doctype . parentNode = doctype . ownerDocument = doc;
        doc . doctype = doctype;
        doc . implementation = self;
        return  doc;
        pub fn createDocumentType ( &self, qualifiedName , publicId , systemId )  {
        doctype = DocumentType ( qualifiedName );
        doctype . publicId = publicId;
        doctype . systemId = systemId;
        return  doctype;
        pub fn getInterface ( &self, feature )  {
        if self . hasFeature ( feature , None /* Option */ ) {
        return  self;
        } else {
        return;
        pub fn _create_document ( self )  {
        return  Document ( );
        class ElementInfo ( object ) ;
        "Object that represents content-model information for an element.

    This implementation == !expected to be used in practice; DOM
    builders should provide implementations which do the right thing
    using information available to it.

    ";
        __slots__ = "tagName" ,;
        pub fn __init__ ( &self, name )  {
        self . tagName = name;
        pub fn getAttributeType ( &self, aname )  {
        return  _no_type;
        pub fn getAttributeTypeNS ( &self, namespaceURI , localName )  {
        return  _no_type;
        pub fn isElementContent ( self )  {
        return  false;
        pub fn isEmpty ( self )  {
        "Returns true iff this element == declared to have an EMPTY
        content model.";
        return  false;
        pub fn isId ( &self, aname )  {
        "Returns true iff the named attribute == a DTD-style ID.";
        return  false;
        pub fn isIdNS ( &self, namespaceURI , localName )  {
        "Returns true iff the identified attribute == a DTD-style ID.";
        return  false;
        pub fn __getstate__ ( self )  {
        return  self . tagName;
        pub fn __setstate__ ( &self, state )  {
        self . tagName = state;
        pub fn _clear_id_cache ( node )  {
        if node . nodeType == Node . DOCUMENT_NODE {
        node . _id_cache . clear ( );
        node . _id_search_stack = None /* Option */;
        } else if _in_document ( node ) {
        node . ownerDocument . _id_cache . clear ( );
        node . ownerDocument . _id_search_stack = None /* Option */;
        class Document ( Node , DocumentLS ) ;
        __slots__ = ( "_elem_info" , "doctype" ,;
        "_id_search_stack" , "childNodes" , "_id_cache" );
        _child_node_types = ( Node . ELEMENT_NODE , Node . PROCESSING_INSTRUCTION_NODE ,;
        Node . COMMENT_NODE , Node . DOCUMENT_TYPE_NODE );
        implementation = DOMImplementation ( );
        nodeType = Node . DOCUMENT_NODE;
        nodeName = "#document";
        nodeValue = None /* Option */;
        attributes = None /* Option */;
        parentNode = None /* Option */;
        previousSibling = nextSibling = None /* Option */;
        actualEncoding = None /* Option */;
        encoding = None /* Option */;
        standalone = None /* Option */;
        version = None /* Option */;
        strictErrorChecking = false;
        errorHandler = None /* Option */;
        documentURI = None /* Option */;
        _magic_id_count = 0;
        pub fn __init__ ( self )  {
        self . doctype = None /* Option */;
        self . childNodes = NodeList ( );
        self . _elem_info = { };
        self . _id_cache = { };
        self . _id_search_stack = None /* Option */;
        pub fn _get_elem_info ( &self, element )  {
        if element . namespaceURI {
        key = element . namespaceURI , element . localName;
        } else {
        key = element . tagName;
        return  self . _elem_info . get ( key );
        pub fn _get_actualEncoding ( self )  {
        return  self . actualEncoding;
        pub fn _get_doctype ( self )  {
        return  self . doctype;
        pub fn _get_documentURI ( self )  {
        return  self . documentURI;
        pub fn _get_encoding ( self )  {
        return  self . encoding;
        pub fn _get_errorHandler ( self )  {
        return  self . errorHandler;
        pub fn _get_standalone ( self )  {
        return  self . standalone;
        pub fn _get_strictErrorChecking ( self )  {
        return  self . strictErrorChecking;
        pub fn _get_version ( self )  {
        return  self . version;
        pub fn appendChild ( &self, node )  {
        if node . nodeType !in self . _child_node_types {
        panic!("xml . dom . HierarchyRequestErr (");
        "%s cannot be child of %s" % ( repr ( node ) , repr ( self ) ) );
        if node . parentNode is !None /* Option */ {
        node . parentNode . removeChild ( node );
        if node . nodeType == Node . ELEMENT_NODE \ {
        and self . _get_documentElement ( ) ;
        panic!("xml . dom . HierarchyRequestErr (");
        "two document elements disallowed" );
        return  Node . appendChild ( self , node );
        pub fn removeChild ( &self, oldChild )  {
        // try {
        self . childNodes . remove ( oldChild );
        // } catch  ValueError  {
        panic!("xml . dom . NotFoundErr ( )");
        oldChild . nextSibling = oldChild . previousSibling = None /* Option */;
        oldChild . parentNode = None /* Option */;
        if self . documentElement is oldChild {
        self . documentElement = None /* Option */;
        return  oldChild;
        pub fn _get_documentElement ( self )  {
        for node in self . childNodes .iter() {
        if node . nodeType == Node . ELEMENT_NODE {
        return  node;
        pub fn unlink ( self )  {
        if self . doctype is !None /* Option */ {
        self . doctype . unlink ( );
        self . doctype = None /* Option */;
        Node . unlink ( self );
        pub fn cloneNode ( &self, deep )  {
        if !deep {
        return;
        clone = self . implementation . createDocument ( None /* Option */ , None /* Option */ , None /* Option */ );
        clone . encoding = self . encoding;
        clone . standalone = self . standalone;
        clone . version = self . version;
        for n in self . childNodes .iter() {
        childclone = _clone_node ( n , deep , clone );
        assert childclone . ownerDocument . isSameNode ( clone );
        clone . childNodes . append ( childclone );
        if childclone . nodeType == Node . DOCUMENT_NODE {
        assert clone . documentElement == None /* Option */;
        } else if childclone . nodeType == Node . DOCUMENT_TYPE_NODE {
        assert clone . doctype == None /* Option */;
        clone . doctype = childclone;
        childclone . parentNode = clone;
        self . _call_user_data_handler ( xml . dom . UserDataHandler . NODE_CLONED ,;
        self , clone );
        return  clone;
        pub fn createDocumentFragment ( self )  {
        d = DocumentFragment ( );
        d . ownerDocument = self;
        return  d;
        pub fn createElement ( &self, tagName )  {
        e = Element ( tagName );
        e . ownerDocument = self;
        return  e;
        pub fn createTextNode ( &self, data )  {
        if !isinstance ( data , str ) {
        panic!("TypeError ( "node contents must be a string" )");
        t = Text ( );
        t . data = data;
        t . ownerDocument = self;
        return  t;
        pub fn createCDATASection ( &self, data )  {
        if !isinstance ( data , str ) {
        panic!("TypeError ( "node contents must be a string" )");
        c = CDATASection ( );
        c . data = data;
        c . ownerDocument = self;
        return  c;
        pub fn createComment ( &self, data )  {
        c = Comment ( data );
        c . ownerDocument = self;
        return  c;
        pub fn createProcessingInstruction ( &self, target , data )  {
        p = ProcessingInstruction ( target , data );
        p . ownerDocument = self;
        return  p;
        pub fn createAttribute ( &self, qName )  {
        a = Attr ( qName );
        a . ownerDocument = self;
        a . value = "";
        return  a;
        pub fn createElementNS ( &self, namespaceURI , qualifiedName )  {
        prefix , localName = _nssplit ( qualifiedName );
        e = Element ( qualifiedName , namespaceURI , prefix );
        e . ownerDocument = self;
        return  e;
        pub fn createAttributeNS ( &self, namespaceURI , qualifiedName )  {
        prefix , localName = _nssplit ( qualifiedName );
        a = Attr ( qualifiedName , namespaceURI , localName , prefix );
        a . ownerDocument = self;
        a . value = "";
        return  a;
        pub fn _create_entity ( &self, name , publicId , systemId , notationName )  {
        e = Entity ( name , publicId , systemId , notationName );
        e . ownerDocument = self;
        return  e;
        pub fn _create_notation ( &self, name , publicId , systemId )  {
        n = Notation ( name , publicId , systemId );
        n . ownerDocument = self;
        return  n;
        pub fn getElementById ( &self, id )  {
        if id in self . _id_cache {
        return  self . _id_cache [ id ];
        if !( self . _elem_info || self . _magic_id_count ) {
        return;
        stack = self . _id_search_stack;
        if stack is None /* Option */ {
        stack = [ self . documentElement ];
        self . _id_search_stack = stack;
        } else if !stack {
        return;
        result = None /* Option */;
        while stack  {
        node = stack . pop ( );
        stack . extend ( vec![ child.iter().map(|child| node . childNodes;
        if child . nodeType in _nodeTypes_with_children ] ) {
        info = self . _get_elem_info ( node );
        if info {
        for attr in node . attributes . values ( ) .iter() {
        if attr . namespaceURI {
        if info . isIdNS ( attr . namespaceURI , attr . localName ) {
        self . _id_cache [ attr . value ] = node;
        if attr . value == id {
        result = node;
        } else if !node . _magic_id_nodes {
        break;
        } else if info . isId ( attr . name ) {
        self . _id_cache [ attr . value ] = node;
        if attr . value == id {
        result = node;
        } else if !node . _magic_id_nodes {
        break;
        } else if attr . _is_id {
        self . _id_cache [ attr . value ] = node;
        if attr . value == id {
        result = node;
        } else if node . _magic_id_nodes == 1 {
        break;
        } else if node . _magic_id_nodes {
        for attr in node . attributes . values ( ) .iter() {
        if attr . _is_id {
        self . _id_cache [ attr . value ] = node;
        if attr . value == id {
        result = node;
        if result is !None /* Option */ {
        break;
        return  result;
        pub fn getElementsByTagName ( &self, name )  {
        return  _get_elements_by_tagName_helper ( self , name , NodeList ( ) );
        pub fn getElementsByTagNameNS ( &self, namespaceURI , localName )  {
        return  _get_elements_by_tagName_ns_helper (;
        self , namespaceURI , localName , NodeList ( ) );
        pub fn isSupported ( &self, feature , version )  {
        return  self . implementation . hasFeature ( feature , version );
        pub fn importNode ( &self, node , deep )  {
        if node . nodeType == Node . DOCUMENT_NODE {
        panic!("xml . dom . NotSupportedErr ( "cannot import document nodes" )");
        } else if node . nodeType == Node . DOCUMENT_TYPE_NODE {
        panic!("xml . dom . NotSupportedErr ( "cannot import document type nodes" )");
        return  _clone_node ( node , deep , self );
        pub fn writexml ( &self, writer , indent = "" , addindent = "" , newl = "" , encoding = None /* Option */ , {
        standalone = None /* Option */ ) ;
        declarations = [ ];
        if encoding {
        declarations . append ( format!("encoding="{encoding}"" ));
        if standalone is !None /* Option */ {
        declarations . append ( format!("standalone="{"yes" if standalone else "no"}"" ));
        writer . write ( format!("<?xml version="1.0" {" ".join(declarations)}?>{newl}" ));
        for node in self . childNodes .iter() {
        node . writexml ( writer , indent , addindent , newl );
        pub fn renameNode ( &self, n , namespaceURI , name )  {
        if n . ownerDocument is !self {
        panic!("xml . dom . WrongDocumentErr (");
        "cannot rename nodes from other documents;\n";
        "expected %s,\nfound %s" % ( self , n . ownerDocument ) );
        if n . nodeType !in ( Node . ELEMENT_NODE , Node . ATTRIBUTE_NODE ) {
        panic!("xml . dom . NotSupportedErr (");
        "renameNode() only applies to element && attribute nodes" );
        if namespaceURI != EMPTY_NAMESPACE {
        if ":" in name {
        prefix , localName = name . split ( ":" , 1 );
        if ( prefix == "xmlns" {
        and namespaceURI != xml . dom . XMLNS_NAMESPACE ) ;
        panic!("xml . dom . NamespaceErr (");
        "illegal use of 'xmlns' prefix" );
        } else {
        if ( name == "xmlns" {
        and namespaceURI != xml . dom . XMLNS_NAMESPACE;
        and n . nodeType == Node . ATTRIBUTE_NODE ) ;
        panic!("xml . dom . NamespaceErr (");
        "illegal use of the 'xmlns' attribute" );
        prefix = None /* Option */;
        localName = name;
        } else {
        prefix = None /* Option */;
        localName = None /* Option */;
        if n . nodeType == Node . ATTRIBUTE_NODE {
        element = n . ownerElement;
        if element is !None /* Option */ {
        is_id = n . _is_id;
        element . removeAttributeNode ( n );
        } else {
        element = None /* Option */;
        n . prefix = prefix;
        n . _localName = localName;
        n . namespaceURI = namespaceURI;
        n . nodeName = name;
        if n . nodeType == Node . ELEMENT_NODE {
        n . tagName = name;
        } else {
        n . name = name;
        if element is !None /* Option */ {
        element . setAttributeNode ( n );
        if is_id {
        element . setIdAttributeNode ( n );
        return  n;
        defproperty ( Document , "documentElement" ,;
        doc = "Top-level element of this document." );
        pub fn _clone_node ( node , deep , newOwnerDocument )  {
        "
    Clone a node && give it the new owner document.
    Called by Node.cloneNode && Document.importNode
    ";
        if node . ownerDocument . isSameNode ( newOwnerDocument ) {
        operation = xml . dom . UserDataHandler . NODE_CLONED;
        } else {
        operation = xml . dom . UserDataHandler . NODE_IMPORTED;
        if node . nodeType == Node . ELEMENT_NODE {
        clone = newOwnerDocument . createElementNS ( node . namespaceURI ,;
        node . nodeName );
        for attr in node . attributes . values ( ) .iter() {
        clone . setAttributeNS ( attr . namespaceURI , attr . nodeName , attr . value );
        a = clone . getAttributeNodeNS ( attr . namespaceURI , attr . localName );
        a . specified = attr . specified;
        if deep {
        for child in node . childNodes .iter() {
        c = _clone_node ( child , deep , newOwnerDocument );
        clone . appendChild ( c );
        } else if node . nodeType == Node . DOCUMENT_FRAGMENT_NODE {
        clone = newOwnerDocument . createDocumentFragment ( );
        if deep {
        for child in node . childNodes .iter() {
        c = _clone_node ( child , deep , newOwnerDocument );
        clone . appendChild ( c );
        } else if node . nodeType == Node . TEXT_NODE {
        clone = newOwnerDocument . createTextNode ( node . data );
        } else if node . nodeType == Node . CDATA_SECTION_NODE {
        clone = newOwnerDocument . createCDATASection ( node . data );
        } else if node . nodeType == Node . PROCESSING_INSTRUCTION_NODE {
        clone = newOwnerDocument . createProcessingInstruction ( node . target ,;
        node . data );
        } else if node . nodeType == Node . COMMENT_NODE {
        clone = newOwnerDocument . createComment ( node . data );
        } else if node . nodeType == Node . ATTRIBUTE_NODE {
        clone = newOwnerDocument . createAttributeNS ( node . namespaceURI ,;
        node . nodeName );
        clone . specified = true;
        clone . value = node . value;
        } else if node . nodeType == Node . DOCUMENT_TYPE_NODE {
        assert node . ownerDocument == !newOwnerDocument;
        operation = xml . dom . UserDataHandler . NODE_IMPORTED;
        clone = newOwnerDocument . implementation . createDocumentType (;
        node . name , node . publicId , node . systemId );
        clone . ownerDocument = newOwnerDocument;
        if deep {
        clone . entities . _seq = [ ];
        clone . notations . _seq = [ ];
        for n in node . notations . _seq .iter() {
        notation = Notation ( n . nodeName , n . publicId , n . systemId );
        notation . ownerDocument = newOwnerDocument;
        clone . notations . _seq . append ( notation );
        if hasattr ( n , "_call_user_data_handler" ) {
        n . _call_user_data_handler ( operation , n , notation );
        for e in node . entities . _seq .iter() {
        entity = Entity ( e . nodeName , e . publicId , e . systemId ,;
        e . notationName );
        entity . actualEncoding = e . actualEncoding;
        entity . encoding = e . encoding;
        entity . version = e . version;
        entity . ownerDocument = newOwnerDocument;
        clone . entities . _seq . append ( entity );
        if hasattr ( e , "_call_user_data_handler" ) {
        e . _call_user_data_handler ( operation , e , entity );
        } else {
        panic!("xml . dom . NotSupportedErr ( "Cannot clone node %s" % repr ( node ) )");
        if hasattr ( node , "_call_user_data_handler" ) {
        node . _call_user_data_handler ( operation , node , clone );
        return  clone;
        pub fn _nssplit ( qualifiedName )  {
        fields = qualifiedName . split ( ":" , 1 );
        if len ( fields ) == 2 {
        return  fields;
        } else {
        return  ( None /* Option */ , fields [ 0 ] );
        pub fn _do_pulldom_parse ( func , args , kwargs )  {
        events = func ( * args , ** kwargs );
        toktype , rootNode = events . getEvent ( );
        events . expandNode ( rootNode );
        events . clear ( );
        return  rootNode;
        pub fn parse ( file , parser = None /* Option */ , bufsize = None /* Option */ )  {
        "Parse a file into a DOM by filename || file object.";
        if parser is None /* Option */ && !bufsize {
        from xml . dom import expatbuilder;
        return  expatbuilder . parse ( file );
        } else {
        from xml . dom import pulldom;
        return  _do_pulldom_parse ( pulldom . parse , ( file , ) ,;
        { "parser" : parser , "bufsize" : bufsize } );
        pub fn parseString ( string , parser = None /* Option */ )  {
        "Parse a file into a DOM from a string.";
        if parser is None /* Option */ {
        from xml . dom import expatbuilder;
        return  expatbuilder . parseString ( string );
        } else {
        from xml . dom import pulldom;
        return  _do_pulldom_parse ( pulldom . parseString , ( string , ) ,;
        { "parser" : parser } );
        pub fn getDOMImplementation ( features = None /* Option */ )  {
        if features {
        if isinstance ( features , str ) {
        features = domreg . _parse_feature_string ( features );
        for f , v in features .iter() {
        if !Document . implementation . hasFeature ( f , v ) {
        return;
        return  Document . implementation;
}

