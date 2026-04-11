//! xmlbuilder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::copy;
// use crate::xml::{NodeFilter};
// use crate::urllib;
// use crate::posixpath;

pub const __all__: &str = ["DOMBuilder" ,"DOMEntityResolver" ,"DOMInputSource" ];
pub struct Options {
    pub _options: String, // TODO: infer type
    pub entityResolver: String, // TODO: infer type
    pub errorHandler: String, // TODO: infer type
    pub filter: String, // TODO: infer type
    pub _opener: String, // TODO: infer type
    pub byteStream: String, // TODO: infer type
    pub characterStream: String, // TODO: infer type
    pub stringData: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub publicId: String, // TODO: infer type
    pub systemId: String, // TODO: infer type
    pub baseURI: String, // TODO: infer type
}

impl Options {
}

pub struct DOMBuilder {
    pub _options: String, // TODO: infer type
    pub entityResolver: String, // TODO: infer type
    pub errorHandler: String, // TODO: infer type
    pub filter: String, // TODO: infer type
    pub _opener: String, // TODO: infer type
    pub byteStream: String, // TODO: infer type
    pub characterStream: String, // TODO: infer type
    pub stringData: String, // TODO: infer type
    pub encoding: String, // TODO: infer type
    pub publicId: String, // TODO: infer type
    pub systemId: String, // TODO: infer type
    pub baseURI: String, // TODO: infer type
}

impl DOMBuilder {
}

pub fn _name_xform(name: &str) {
        return  name . lower ( ) . replace ( "-" , "_" );
        class DOMEntityResolver ( object ) ;
        __slots__ = "_opener" ,;
        pub fn resolveEntity ( &self, publicId , systemId )  {
        assert systemId == !None /* Option */;
        source = DOMInputSource ( );
        source . publicId = publicId;
        source . systemId = systemId;
        source . byteStream = self . _get_opener ( ) . open ( systemId );
        source . encoding = self . _guess_media_encoding ( source );
        import posixpath , urllib . parse;
        parts = urllib . parse . urlparse ( systemId );
        scheme , netloc , path , params , query , fragment = parts;
        if path && !path . endswith ( "/" ) {
        path = posixpath . dirname ( path ) + "/";
        parts = scheme , netloc , path , params , query , fragment;
        source . baseURI = urllib . parse . urlunparse ( parts );
        return  source;
        pub fn _get_opener ( self )  {
        // try {
        return  self . _opener;
        // } catch  AttributeError  {
        self . _opener = self . _create_opener ( );
        return  self . _opener;
        pub fn _create_opener ( self )  {
        import urllib . request;
        return  urllib . request . build_opener ( );
        pub fn _guess_media_encoding ( &self, source )  {
        info = source . byteStream . info ( );
        if "Content-Type" in info {
        for param in info . getplist ( ) .iter() {
        if param . startswith ( "charset=" ) {
        return  param . split ( "=" , 1 ) [ 1 ] . lower ( );
        class DOMInputSource ( object ) ;
        __slots__ = ( "byteStream" , "characterStream" , "stringData" ,;
        "encoding" , "publicId" , "systemId" , "baseURI" );
        pub fn __init__ ( self )  {
        self . byteStream = None /* Option */;
        self . characterStream = None /* Option */;
        self . stringData = None /* Option */;
        self . encoding = None /* Option */;
        self . publicId = None /* Option */;
        self . systemId = None /* Option */;
        self . baseURI = None /* Option */;
        pub fn _get_byteStream ( self )  {
        return  self . byteStream;
        pub fn _set_byteStream ( &self, byteStream )  {
        self . byteStream = byteStream;
        pub fn _get_characterStream ( self )  {
        return  self . characterStream;
        pub fn _set_characterStream ( &self, characterStream )  {
        self . characterStream = characterStream;
        pub fn _get_stringData ( self )  {
        return  self . stringData;
        pub fn _set_stringData ( &self, data )  {
        self . stringData = data;
        pub fn _get_encoding ( self )  {
        return  self . encoding;
        pub fn _set_encoding ( &self, encoding )  {
        self . encoding = encoding;
        pub fn _get_publicId ( self )  {
        return  self . publicId;
        pub fn _set_publicId ( &self, publicId )  {
        self . publicId = publicId;
        pub fn _get_systemId ( self )  {
        return  self . systemId;
        pub fn _set_systemId ( &self, systemId )  {
        self . systemId = systemId;
        pub fn _get_baseURI ( self )  {
        return  self . baseURI;
        pub fn _set_baseURI ( &self, uri )  {
        self . baseURI = uri;
        class DOMBuilderFilter ;
        "Element filter which can be used to tailor construction of
    a DOM instance.
    ";
        FILTER_ACCEPT = 1;
        FILTER_REJECT = 2;
        FILTER_SKIP = 3;
        FILTER_INTERRUPT = 4;
        whatToShow = NodeFilter . SHOW_ALL;
        pub fn _get_whatToShow ( self )  {
        return  self . whatToShow;
        pub fn acceptNode ( &self, element )  {
        return  self . FILTER_ACCEPT;
        pub fn startContainer ( &self, element )  {
        return  self . FILTER_ACCEPT;
        del NodeFilter;
        class DocumentLS ;
        "Mixin to create documents that conform to the load/save spec.";
        async_ = false;
        pub fn _get_async ( self )  {
        return  false;
        pub fn _set_async ( &self, flag )  {
        if flag {
        panic!("xml . dom . NotSupportedErr (");
        "asynchronous document loading == !supported" );
        pub fn abort ( self )  {
        panic!("NotImplementedError (");
        "haven't figured out what this means yet" );
        pub fn load ( &self, uri )  {
        panic!("NotImplementedError ( "haven't written this yet" )");
        pub fn loadXML ( &self, source )  {
        panic!("NotImplementedError ( "haven't written this yet" )");
        pub fn saveXML ( &self, snode )  {
        if snode is None /* Option */ {
        snode = self;
        } else if snode . ownerDocument is !self {
        panic!("xml . dom . WrongDocumentErr ( )");
        return  snode . toxml ( );
        class DOMImplementationLS ;
        MODE_SYNCHRONOUS = 1;
        MODE_ASYNCHRONOUS = 2;
        pub fn createDOMBuilder ( &self, mode , schemaType )  {
        if schemaType is !None /* Option */ {
        panic!("xml . dom . NotSupportedErr (");
        "schemaType !yet supported" );
        if mode == self . MODE_SYNCHRONOUS {
        return  DOMBuilder ( );
        if mode == self . MODE_ASYNCHRONOUS {
        panic!("xml . dom . NotSupportedErr (");
        "asynchronous builders are !supported" );
        panic!("ValueError ( "unknown value for mode" )");
        pub fn createDOMWriter ( self )  {
        panic!("NotImplementedError (");
        "the writer interface hasn't been written yet!" );
        pub fn createDOMInputSource ( self )  {
        return  DOMInputSource ( );
}

