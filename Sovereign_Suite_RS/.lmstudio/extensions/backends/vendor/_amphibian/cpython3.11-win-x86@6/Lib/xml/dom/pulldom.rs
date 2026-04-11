//! pulldom.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::xml;
// use crate::io::{StringIO};

pub const START_ELEMENT: &str = "START_ELEMENT";
pub const END_ELEMENT: &str = "END_ELEMENT";
pub const COMMENT: &str = "COMMENT";
pub const START_DOCUMENT: &str = "START_DOCUMENT";
pub const END_DOCUMENT: &str = "END_DOCUMENT";
pub const PROCESSING_INSTRUCTION: &str = "PROCESSING_INSTRUCTION";
pub const IGNORABLE_WHITESPACE: &str = "IGNORABLE_WHITESPACE";
pub const CHARACTERS: &str = "CHARACTERS";
pub struct PullDOM {
    pub documentFactory: String, // TODO: infer type
    pub firstEvent: String, // TODO: infer type
    pub lastEvent: String, // TODO: infer type
    pub elementStack: String, // TODO: infer type
    pub push: String, // TODO: infer type
    pub pop: String, // TODO: infer type
    pub _ns_contexts: String, // TODO: infer type
    pub _current_context: String, // TODO: infer type
    pub pending_events: String, // TODO: infer type
    pub _locator: String, // TODO: infer type
    pub _xmlns_attrs: String, // TODO: infer type
    pub document: String, // TODO: infer type
    pub stream: String, // TODO: infer type
    pub parser: String, // TODO: infer type
    pub bufsize: String, // TODO: infer type
    pub getEvent: String, // TODO: infer type
    pub pulldom: String, // TODO: infer type
}

impl PullDOM {
}

pub struct ErrorHandler {
    pub stream: String, // TODO: infer type
    pub parser: String, // TODO: infer type
    pub bufsize: String, // TODO: infer type
    pub getEvent: String, // TODO: infer type
    pub pulldom: String, // TODO: infer type
}

impl ErrorHandler {
    pub fn warning(&self, exception: &str) {
        println!( exception );
        pub fn error ( &self, exception )  {
        panic!("exception");
        pub fn fatalError ( &self, exception )  {
        panic!("exception");
    }

    pub fn parse(&self, stream_or_string: &str, parser: &str, bufsize: &str) {
        if bufsize is None /* Option */ {
        bufsize = default_bufsize;
        if isinstance ( stream_or_string , str ) {
        stream = open ( stream_or_string , "rb" );
        } else {
        stream = stream_or_string;
        if !parser {
        parser = xml . sax . make_parser ( );
        return  DOMEventStream ( stream , parser , bufsize );
        pub fn parseString ( string , parser = None /* Option */ )  {
        from io import StringIO;
        bufsize = len ( string );
        buf = StringIO ( string );
        if !parser {
        parser = xml . sax . make_parser ( );
        return  DOMEventStream ( buf , parser , bufsize );
    }

}

