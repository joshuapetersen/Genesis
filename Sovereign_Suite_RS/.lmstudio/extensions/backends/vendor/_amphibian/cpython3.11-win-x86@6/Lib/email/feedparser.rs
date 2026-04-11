//! feedparser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::email::{errors};
// use std::collections::{deque};
// use crate::io::{StringIO};

pub const __all__: &str = ["FeedParser" ,"BytesFeedParser" ];
pub const NLCRE: &str = re . compile ( r"\r\n|\r|\n" );
pub const NLCRE_bol: &str = re . compile ( r"(\r\n|\r|\n)" );
pub const NLCRE_eol: &str = re . compile ( r"(\r\n|\r|\n)\Z" );
pub const NLCRE_crack: &str = re . compile ( r"(\r\n|\r|\n)" );
pub const headerRE: &str = re . compile ( r"^(From |[\041-\071\073-\176]*:|[\t ])" );
pub const EMPTYSTRING: &str = "";
pub const NL: &str = "\n";
pub const NeedMoreData: f64 = object ( );
pub struct BufferedSubFile {
    pub _partial: String, // TODO: infer type
    pub _lines: String, // TODO: infer type
    pub _eofstack: String, // TODO: infer type
    pub _closed: String, // TODO: infer type
    pub policy: String, // TODO: infer type
    pub _old_style_factory: String, // TODO: infer type
    pub _factory: String, // TODO: infer type
    pub _input: String, // TODO: infer type
    pub _msgstack: String, // TODO: infer type
    pub _parse: String, // TODO: infer type
    pub _cur: String, // TODO: infer type
    pub _last: String, // TODO: infer type
    pub _headersonly: String, // TODO: infer type
}

impl BufferedSubFile {
    pub fn new() -> Self {
        self . _partial = StringIO ( newline = "" );
        self . _lines = deque ( );
        self . _eofstack = [ ];
        self . _closed = false;
    }

}

