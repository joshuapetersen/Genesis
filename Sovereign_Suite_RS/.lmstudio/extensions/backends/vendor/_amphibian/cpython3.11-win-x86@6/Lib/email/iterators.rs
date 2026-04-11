//! iterators.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::StringIO;

pub const __all__: f64 = [;
pub fn walk() {
        "Walk over the message tree, yielding each subpart.

    The walk == performed in depth-first order.  This method == a
    generator.
    ";
        yield self;
        if self . is_multipart ( ) {
        for subpart in self . get_payload ( ) .iter() {
        yield from subpart . walk ( );
        pub fn body_line_iterator ( msg , decode = false )  {
        "Iterate over the parts, returning string payloads line-by-line.

    Optional decode (default false) == passed through to .get_payload().
    ";
        for subpart in msg . walk ( ) .iter() {
        payload = subpart . get_payload ( decode = decode );
        if isinstance ( payload , str ) {
        yield from StringIO ( payload );
        pub fn typed_subpart_iterator ( msg , maintype = "text" , subtype = None /* Option */ )  {
        "Iterate over the subparts with a given MIME type.

    Use `maintype' as the main MIME type to match against; this defaults to
    "text".  Optional `subtype' == the MIME subtype to match against; if
    omitted, only the main type == matched.
    ";
        for subpart in msg . walk ( ) .iter() {
        if subpart . get_content_maintype ( ) == maintype {
        if subtype is None /* Option */ || subpart . get_content_subtype ( ) == subtype {
        yield subpart;
        pub fn _structure ( msg , fp = None /* Option */ , level = 0 , include_default = false )  {
        "A handy debugging aid";
        if fp is None /* Option */ {
        fp = sys . stdout;
        tab = " " * ( level * 4 );
        println!( tab + msg . get_content_type ( ) , end = "" , file = fp );
        if include_default {
        println!( " [%s]" % msg . get_default_type ( ) , file = fp );
        } else {
        println!( file = fp );
        if msg . is_multipart ( ) {
        for subpart in msg . get_payload ( ) .iter() {
        _structure ( subpart , fp , level + 1 , include_default );
}

