//! tty.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::termios::{};

pub const __all__: &str = ["setraw" ,"setcbreak" ];
pub const IFLAG: u64 = 0;
pub const OFLAG: u64 = 1;
pub const CFLAG: u64 = 2;
pub const LFLAG: u64 = 3;
pub const ISPEED: u64 = 4;
pub const OSPEED: u64 = 5;
pub const CC: u64 = 6;
pub fn setraw(fd: &str, when: &str, TCSAFLUSH: &str) {
        "Put terminal into a raw mode.";
        mode = tcgetattr ( fd );
        mode [ IFLAG ] = mode [ IFLAG ] & ~ ( BRKINT | ICRNL | INPCK | ISTRIP | IXON );
        mode [ OFLAG ] = mode [ OFLAG ] & ~ ( OPOST );
        mode [ CFLAG ] = mode [ CFLAG ] & ~ ( CSIZE | PARENB );
        mode [ CFLAG ] = mode [ CFLAG ] | CS8;
        mode [ LFLAG ] = mode [ LFLAG ] & ~ ( ECHO | ICANON | IEXTEN | ISIG );
        mode [ CC ] [ VMIN ] = 1;
        mode [ CC ] [ VTIME ] = 0;
        tcsetattr ( fd , when , mode );
        pub fn setcbreak ( fd , when = TCSAFLUSH )  {
        "Put terminal into a cbreak mode.";
        mode = tcgetattr ( fd );
        mode [ LFLAG ] = mode [ LFLAG ] & ~ ( ECHO | ICANON );
        mode [ CC ] [ VMIN ] = 1;
        mode [ CC ] [ VTIME ] = 0;
        tcsetattr ( fd , when , mode );
}

