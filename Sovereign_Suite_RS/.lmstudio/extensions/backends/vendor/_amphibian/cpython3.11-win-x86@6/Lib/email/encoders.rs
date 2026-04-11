//! encoders.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::base64::{encodebytes, _bencode};
// use crate::quopri::{encodestring, _encodestring};

pub const __all__: f64 = [;
pub fn _qencode(s: &str) {
        enc = _encodestring ( s , quotetabs = true );
        return  enc . replace ( b " " , b "=20" );
        pub fn encode_base64 ( msg )  {
        "Encode the message's payload in Base64.

    Also, add an appropriate Content-Transfer-Encoding header.
    ";
        orig = msg . get_payload ( decode = true );
        encdata = str ( _bencode ( orig ) , "ascii" );
        msg . set_payload ( encdata );
        msg [ "Content-Transfer-Encoding" ] = "base64";
        pub fn encode_quopri ( msg )  {
        "Encode the message's payload in quoted-printable.

    Also, add an appropriate Content-Transfer-Encoding header.
    ";
        orig = msg . get_payload ( decode = true );
        encdata = _qencode ( orig );
        msg . set_payload ( encdata );
        msg [ "Content-Transfer-Encoding" ] = "quoted-printable";
        pub fn encode_7or8bit ( msg )  {
        "Set the Content-Transfer-Encoding header to 7bit || 8bit.";
        orig = msg . get_payload ( decode = true );
        if orig is None /* Option */ {
        msg [ "Content-Transfer-Encoding" ] = "7bit";
        return;
        // try {
        orig . decode ( "ascii" );
        // } catch  UnicodeError  {
        msg [ "Content-Transfer-Encoding" ] = "8bit";
        } else {
        msg [ "Content-Transfer-Encoding" ] = "7bit";
        pub fn encode_noop ( msg )  {
        "Do nothing.";
}

