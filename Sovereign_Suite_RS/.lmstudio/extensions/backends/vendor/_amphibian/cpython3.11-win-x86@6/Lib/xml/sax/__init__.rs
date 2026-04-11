//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::.::{InputSource};
// use crate::io;
// use crate::xml;
// use std::fs;
// use std::env;
// use crate::org::{imp};

pub fn parse(source: &str, handler: &str, errorHandler: &str, ErrorHandler: &str) {
        parser = make_parser ( );
        parser . setContentHandler ( handler );
        parser . setErrorHandler ( errorHandler );
        parser . parse ( source );
        pub fn parseString ( string , handler , errorHandler = ErrorHandler ( ) )  {
        import io;
        if errorHandler is None /* Option */ {
        errorHandler = ErrorHandler ( );
        parser = make_parser ( );
        parser . setContentHandler ( handler );
        parser . setErrorHandler ( errorHandler );
        inpsrc = InputSource ( );
        if isinstance ( string , str ) {
        inpsrc . setCharacterStream ( io . StringIO ( string ) );
        } else {
        inpsrc . setByteStream ( io . BytesIO ( string ) );
        parser . parse ( inpsrc );
        default_parser_list = [ "xml.sax.expatreader" ];
        _false = 0;
        if _false {
        import xml . sax . expatreader;
        import os , sys;
        if !sys . flags . ignore_environment && "PY_SAX_PARSER" in os . environ {
        default_parser_list = os . environ [ "PY_SAX_PARSER" ] . split ( "," );
        del os;
        _key = "python.xml.sax.parser";
        if sys . platform [ { : 4 ] == "java" && sys . registry . containsKey ( _key ) ; }
        default_parser_list = sys . registry . getProperty ( _key ) . split ( "," );
        pub fn make_parser ( parser_list = ( ) )  {
        "Creates && returns a SAX parser.

    Creates the first parser it == able to instantiate of the ones
    given in the iterable created by chaining parser_list and
    default_parser_list.  The iterables must contain the names of Python
    modules containing both a SAX parser && a create_parser function.";
        for parser_name in list ( parser_list ) + default_parser_list .iter() {
        // try {
        return  _create_parser ( parser_name );
        // } catch  ImportError  {
        import sys;
        if parser_name in sys . modules {
        panic!("");
        // } catch  SAXReaderNotAvailable  {
        // pass
        panic!("SAXReaderNotAvailable ( "No parsers found" , None /* Option */ )");
        if sys . platform [ { : 4 ] == "java" ; }
        pub fn _create_parser ( parser_name )  {
        from org . python . core import imp;
        drv_module = imp . importName ( parser_name , 0 , globals ( ) );
        return  drv_module . create_parser ( );
        } else {
        pub fn _create_parser ( parser_name )  {
        drv_module = __import__ ( parser_name , { } , { } , [ "create_parser" ] );
        return  drv_module . create_parser ( );
        del sys;
}

