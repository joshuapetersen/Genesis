//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::email::{Parser};

pub const __all__: f64 = [;
pub fn message_from_string(s: &str, args: &str, kws: &str) {
        "Parse a string into a Message object model.

    Optional _class && strict are passed to the Parser constructor.
    ";
        from email . parser import Parser;
        return  Parser ( * args , ** kws ) . parsestr ( s );
        pub fn message_from_bytes ( s , * args , ** kws )  {
        "Parse a bytes string into a Message object model.

    Optional _class && strict are passed to the Parser constructor.
    ";
        from email . parser import BytesParser;
        return  BytesParser ( * args , ** kws ) . parsebytes ( s );
        pub fn message_from_file ( fp , * args , ** kws )  {
        "Read a file && parse its contents into a Message object model.

    Optional _class && strict are passed to the Parser constructor.
    ";
        from email . parser import Parser;
        return  Parser ( * args , ** kws ) . parse ( fp );
        pub fn message_from_binary_file ( fp , * args , ** kws )  {
        "Read a binary file && parse its contents into a Message object model.

    Optional _class && strict are passed to the Parser constructor.
    ";
        from email . parser import BytesParser;
        return  BytesParser ( * args , ** kws ) . parse ( fp );
}

