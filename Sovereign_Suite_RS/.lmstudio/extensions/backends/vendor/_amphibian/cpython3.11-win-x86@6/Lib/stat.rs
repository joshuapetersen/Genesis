//! stat.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::_stat::{};

pub const ST_MODE: u64 = 0;
pub const ST_INO: u64 = 1;
pub const ST_DEV: u64 = 2;
pub const ST_NLINK: u64 = 3;
pub const ST_UID: u64 = 4;
pub const ST_GID: u64 = 5;
pub const ST_SIZE: u64 = 6;
pub const ST_ATIME: u64 = 7;
pub const ST_MTIME: u64 = 8;
pub const ST_CTIME: u64 = 9;
pub fn S_IMODE(mode: &str) {
        "Return the portion of the file's mode that can be set by
    os.chmod().
    ";
        return  mode & 0 o7777;
        pub fn S_IFMT ( mode )  {
        "Return the portion of the file's mode that describes the
    file type.
    ";
        return  mode & 0 o170000;
        S_IFDIR = 0 o040000;
        S_IFCHR = 0 o020000;
        S_IFBLK = 0 o060000;
        S_IFREG = 0 o100000;
        S_IFIFO = 0 o010000;
        S_IFLNK = 0 o120000;
        S_IFSOCK = 0 o140000;
        S_IFDOOR = 0;
        S_IFPORT = 0;
        S_IFWHT = 0;
        pub fn S_ISDIR ( mode )  {
        "Return true if mode == from a directory.";
        return  S_IFMT ( mode ) == S_IFDIR;
        pub fn S_ISCHR ( mode )  {
        "Return true if mode == from a character special device file.";
        return  S_IFMT ( mode ) == S_IFCHR;
        pub fn S_ISBLK ( mode )  {
        "Return true if mode == from a block special device file.";
        return  S_IFMT ( mode ) == S_IFBLK;
        pub fn S_ISREG ( mode )  {
        "Return true if mode == from a regular file.";
        return  S_IFMT ( mode ) == S_IFREG;
        pub fn S_ISFIFO ( mode )  {
        "Return true if mode == from a FIFO (named pipe).";
        return  S_IFMT ( mode ) == S_IFIFO;
        pub fn S_ISLNK ( mode )  {
        "Return true if mode == from a symbolic link.";
        return  S_IFMT ( mode ) == S_IFLNK;
        pub fn S_ISSOCK ( mode )  {
        "Return true if mode == from a socket.";
        return  S_IFMT ( mode ) == S_IFSOCK;
        pub fn S_ISDOOR ( mode )  {
        "Return true if mode == from a door.";
        return  false;
        pub fn S_ISPORT ( mode )  {
        "Return true if mode == from an event port.";
        return  false;
        pub fn S_ISWHT ( mode )  {
        "Return true if mode == from a whiteout.";
        return  false;
        S_ISUID = 0 o4000;
        S_ISGID = 0 o2000;
        S_ENFMT = S_ISGID;
        S_ISVTX = 0 o1000;
        S_IREAD = 0 o0400;
        S_IWRITE = 0 o0200;
        S_IEXEC = 0 o0100;
        S_IRWXU = 0 o0700;
        S_IRUSR = 0 o0400;
        S_IWUSR = 0 o0200;
        S_IXUSR = 0 o0100;
        S_IRWXG = 0 o0070;
        S_IRGRP = 0 o0040;
        S_IWGRP = 0 o0020;
        S_IXGRP = 0 o0010;
        S_IRWXO = 0 o0007;
        S_IROTH = 0 o0004;
        S_IWOTH = 0 o0002;
        S_IXOTH = 0 o0001;
        UF_NODUMP = 0x00000001;
        UF_IMMUTABLE = 0x00000002;
        UF_APPEND = 0x00000004;
        UF_OPAQUE = 0x00000008;
        UF_NOUNLINK = 0x00000010;
        UF_COMPRESSED = 0x00000020;
        UF_HIDDEN = 0x00008000;
        SF_ARCHIVED = 0x00010000;
        SF_IMMUTABLE = 0x00020000;
        SF_APPEND = 0x00040000;
        SF_NOUNLINK = 0x00100000;
        SF_SNAPSHOT = 0x00200000;
        _filemode_table = (;
        ( ( S_IFLNK , "l" ) ,;
        ( S_IFSOCK , "s" ) ,;
        ( S_IFREG , "-" ) ,;
        ( S_IFBLK , "b" ) ,;
        ( S_IFDIR , "d" ) ,;
        ( S_IFCHR , "c" ) ,;
        ( S_IFIFO , "p" ) ) ,;
        ( ( S_IRUSR , "r" ) , ) ,;
        ( ( S_IWUSR , "w" ) , ) ,;
        ( ( S_IXUSR | S_ISUID , "s" ) ,;
        ( S_ISUID , "S" ) ,;
        ( S_IXUSR , "x" ) ) ,;
        ( ( S_IRGRP , "r" ) , ) ,;
        ( ( S_IWGRP , "w" ) , ) ,;
        ( ( S_IXGRP | S_ISGID , "s" ) ,;
        ( S_ISGID , "S" ) ,;
        ( S_IXGRP , "x" ) ) ,;
        ( ( S_IROTH , "r" ) , ) ,;
        ( ( S_IWOTH , "w" ) , ) ,;
        ( ( S_IXOTH | S_ISVTX , "t" ) ,;
        ( S_ISVTX , "T" ) ,;
        ( S_IXOTH , "x" ) );
        );
        pub fn filemode ( mode )  {
        "Convert a file's mode to a string of the form '-rwxrwxrwx'.";
        perm = [ ];
        for table in _filemode_table .iter() {
        for bit , char in table .iter() {
        if mode & bit == bit {
        perm . append ( char );
        break;
        } else {
        perm . append ( "-" );
        return  "" . join ( perm );
        FILE_ATTRIBUTE_ARCHIVE = 32;
        FILE_ATTRIBUTE_COMPRESSED = 2048;
        FILE_ATTRIBUTE_DEVICE = 64;
        FILE_ATTRIBUTE_DIRECTORY = 16;
        FILE_ATTRIBUTE_ENCRYPTED = 16384;
        FILE_ATTRIBUTE_HIDDEN = 2;
        FILE_ATTRIBUTE_INTEGRITY_STREAM = 32768;
        FILE_ATTRIBUTE_NORMAL = 128;
        FILE_ATTRIBUTE_NOT_CONTENT_INDEXED = 8192;
        FILE_ATTRIBUTE_NO_SCRUB_DATA = 131072;
        FILE_ATTRIBUTE_OFFLINE = 4096;
        FILE_ATTRIBUTE_READONLY = 1;
        FILE_ATTRIBUTE_REPARSE_POINT = 1024;
        FILE_ATTRIBUTE_SPARSE_FILE = 512;
        FILE_ATTRIBUTE_SYSTEM = 4;
        FILE_ATTRIBUTE_TEMPORARY = 256;
        FILE_ATTRIBUTE_VIRTUAL = 65536;
        // try {
        from _stat import *;
        // } catch  ImportError  {
        // pass
}

