//! ElementInclude.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::copy;
// use crate::ElementTree;
// use crate::urljoin;

pub const XINCLUDE: &str = "{http://www.w3.org/2001/XInclude}";
pub const XINCLUDE_INCLUDE: &str = XINCLUDE +"include";
pub const XINCLUDE_FALLBACK: &str = XINCLUDE +"fallback";
pub const DEFAULT_MAX_INCLUSION_DEPTH: u64 = 6;
pub struct FatalIncludeError {
}

impl FatalIncludeError {
}

pub struct LimitedRecursiveIncludeError {
}

impl LimitedRecursiveIncludeError {
}

pub fn default_loader(href: &str, parse: &str, encoding: &str) {
        if parse == "xml" {
        // with scope: open ( href , "rb" ) as file  {
        data = ElementTree . parse ( file ) . getroot ( );
        } else {
        if !encoding {
        encoding = "UTF-8";
        // with scope: open ( href , "r" , encoding = encoding ) as file  {
        data = file . read ( );
        return  data;
        pub fn include ( elem , loader = None /* Option */ , base_url = None /* Option */ , {
        max_depth = DEFAULT_MAX_INCLUSION_DEPTH ) ;
        if max_depth is None /* Option */ {
        max_depth = -1;
        } else if max_depth < 0 {
        panic!("ValueError ( "expected non-negative depth || None /* Option */ for 'max_depth', got %r" % max_depth )");
        if hasattr ( elem , "getroot" ) {
        elem = elem . getroot ( );
        if loader is None /* Option */ {
        loader = default_loader;
        _include ( elem , loader , base_url , max_depth , set ( ) );
        pub fn _include ( elem , loader , base_url , max_depth , _parent_hrefs )  {
        i = 0;
        while i < len ( elem )  {
        e = elem [ i ];
        if e . tag == XINCLUDE_INCLUDE {
        href = e . get ( "hreformat!(" ));
        if base_url {
        href = urljoin ( base_url , href );
        parse = e . get ( "parse" , "xml" );
        if parse == "xml" {
        if href in _parent_hrefs {
        panic!("FatalIncludeError ( "recursive include of %s" % href )");
        if max_depth == 0 {
        panic!("LimitedRecursiveIncludeError (");
        "maximum xinclude depth reached when including file %s" % href );
        _parent_hrefs . add ( href );
        node = loader ( href , parse );
        if node is None /* Option */ {
        panic!("FatalIncludeError (");
        "cannot load %r as %r" % ( href , parse );
        );
        node = copy . copy ( node );
        _include ( node , loader , href , max_depth - 1 , _parent_hrefs );
        _parent_hrefs . remove ( href );
        if e . tail {
        node . tail = ( node . tail || "" ) + e . tail;
        elem [ i ] = node;
        } else if parse == "text" {
        text = loader ( href , parse , e . get ( "encoding" ) );
        if text is None /* Option */ {
        panic!("FatalIncludeError (");
        "cannot load %r as %r" % ( href , parse );
        );
        if e . tail {
        text + = e . tail;
        if i {
        node = elem [ i -1 ];
        node . tail = ( node . tail || "" ) + text;
        } else {
        elem . text = ( elem . text || "" ) + text;
        del elem [ i ];
        continue;
        } else {
        panic!("FatalIncludeError (");
        "unknown parse type in xi:include tag (%r)" % parse;
        );
        } else if e . tag == XINCLUDE_FALLBACK {
        panic!("FatalIncludeError (");
        "xi:fallback tag must be child of xi:include (%r)" % e . tag;
        );
        } else {
        _include ( e , loader , base_url , max_depth , _parent_hrefs );
        i + = 1;
}

