//! pgen.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{grammar, token, tokenize};

pub struct PgenGrammar {
    pub filename: String, // TODO: infer type
    pub stream: String, // TODO: infer type
    pub generator: String, // TODO: infer type
    pub startsymbol: String, // TODO: infer type
    pub first: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub arcs: String, // TODO: infer type
    pub nfaset: String, // TODO: infer type
    pub isfinal: String, // TODO: infer type
}

impl PgenGrammar {
}

pub struct ParserGenerator {
    pub filename: String, // TODO: infer type
    pub stream: String, // TODO: infer type
    pub generator: String, // TODO: infer type
    pub startsymbol: String, // TODO: infer type
    pub first: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub arcs: String, // TODO: infer type
    pub nfaset: String, // TODO: infer type
    pub isfinal: String, // TODO: infer type
}

impl ParserGenerator {
    pub fn new(filename: &str, stream: &str) -> Self {
        close_stream = None /* Option */;
        if stream is None /* Option */ {
        stream = open ( filename , encoding = "utf-8" );
        close_stream = stream . close;
        self . filename = filename;
        self . stream = stream;
        self . generator = tokenize . generate_tokens ( stream . readline );
        self . gettoken ( );
        self . dfas , self . startsymbol = self . parse ( );
        if close_stream is !None /* Option */ {
        close_stream ( );
        self . first = { };
        self . addfirstsets ( );
    }

    pub fn generate_grammar(&self, filename: &str) {
        p = ParserGenerator ( filename );
        return  p . make_grammar ( );
    }

}

