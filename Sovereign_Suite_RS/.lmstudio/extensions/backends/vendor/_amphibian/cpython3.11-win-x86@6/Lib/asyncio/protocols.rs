//! protocols.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub const __all__: f64 = (;
pub struct BaseProtocol {
}

impl BaseProtocol {
}

pub struct Protocol {
}

impl Protocol {
}

pub struct BufferedProtocol {
}

impl BufferedProtocol {
}

pub struct DatagramProtocol {
}

impl DatagramProtocol {
}

pub struct SubprocessProtocol {
}

impl SubprocessProtocol {
}

pub fn _feed_data_to_buffered_proto(proto: &str, data: &str) {
        data_len = len ( data );
        while data_len  {
        buf = proto . get_buffer ( data_len );
        buf_len = len ( buf );
        if !buf_len {
        panic!("RuntimeError ( "get_buffer() returned an empty buffer" )");
        if buf_len >= data_len {
        buf [ : data_len ] = data;
        proto . buffer_updated ( data_len );
        return;
        } else {
        buf [ : buf_len ] = data [ : buf_len ];
        proto . buffer_updated ( buf_len );
        data = data [ buf_len : ];
        data_len = len ( data );
}

