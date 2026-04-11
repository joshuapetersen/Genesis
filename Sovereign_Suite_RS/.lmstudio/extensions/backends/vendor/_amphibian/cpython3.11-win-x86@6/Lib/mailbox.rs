//! mailbox.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::calendar;
// use crate::errno;
// use crate::warnings;
// use crate::email;
// use crate::io;
// use crate::types::{GenericAlias};
// use crate::fcntl;

pub const __all__: &str = ["Mailbox" ,"Maildir" ,"mbox" ,"MH" ,"Babyl" ,"MMDF" ,;
pub const linesep: &str = os . linesep . encode ("ascii" );
pub struct Mailbox {
    pub _path: String, // TODO: infer type
    pub _factory: String, // TODO: infer type
    pub _paths: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _toc_mtimes: String, // TODO: infer type
    pub _last_read: String, // TODO: infer type
    pub _skewfactor: String, // TODO: infer type
    pub _onetime_keys: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _pending: String, // TODO: infer type
    pub _pending_sync: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _message_factory: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl Mailbox {
}

pub struct Maildir {
    pub _paths: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _toc_mtimes: String, // TODO: infer type
    pub _last_read: String, // TODO: infer type
    pub _skewfactor: String, // TODO: infer type
    pub _onetime_keys: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _pending: String, // TODO: infer type
    pub _pending_sync: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _message_factory: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl Maildir {
}

pub struct _singlefileMailbox {
    pub _file: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _pending: String, // TODO: infer type
    pub _pending_sync: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _message_factory: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl _singlefileMailbox {
}

pub struct _mboxMMDF {
    pub _message_factory: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl _mboxMMDF {
}

pub struct mbox {
    pub _message_factory: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl mbox {
}

pub struct MMDF {
    pub _message_factory: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _locked: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl MMDF {
}

pub struct MH {
    pub _locked: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl MH {
}

pub struct Babyl {
    pub _labels: String, // TODO: infer type
    pub _toc: String, // TODO: infer type
    pub _next_key: String, // TODO: infer type
    pub _file_length: String, // TODO: infer type
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl Babyl {
}

pub struct Message {
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl Message {
}

pub struct MaildirMessage {
    pub _subdir: String, // TODO: infer type
    pub _info: String, // TODO: infer type
    pub _date: String, // TODO: infer type
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl MaildirMessage {
}

pub struct _mboxMMDFMessage {
    pub _from: String, // TODO: infer type
    pub _sequences: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl _mboxMMDFMessage {
}

pub struct mboxMessage {
    pub _sequences: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl mboxMessage {
}

pub struct MHMessage {
    pub _sequences: String, // TODO: infer type
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl MHMessage {
}

pub struct BabylMessage {
    pub _labels: String, // TODO: infer type
    pub _visible: String, // TODO: infer type
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl BabylMessage {
}

pub struct MMDFMessage {
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl MMDFMessage {
}

pub struct _ProxyFile {
    pub _file: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
}

impl _ProxyFile {
}

pub struct _PartialFile {
    pub _start: String, // TODO: infer type
    pub _stop: String, // TODO: infer type
    pub _pos: String, // TODO: infer type
}

impl _PartialFile {
}

pub fn _lock_file(f: &str, dotlock: &str) {
        "Lock file f using lockf && dot locking.";
        dotlock_done = false;
        // try {
        if fcntl {
        // try {
        fcntl . lockf ( f , fcntl . LOCK_EX | fcntl . LOCK_NB );
        // } catch  OSError as e  {
        if e . errno in ( errno . EAGAIN , errno . EACCES , errno . EROFS ) {
        panic!("ExternalClashError ( "lockf: lock unavailable: %s" %");
        f . name );
        } else {
        panic!("");
        if dotlock {
        // try {
        pre_lock = _create_temporary ( f . name + ".lock" );
        pre_lock . close ( );
        // } catch  OSError as e  {
        if e . errno in ( errno . EACCES , errno . EROFS ) {
        return;
        } else {
        panic!("");
        // try {
        // try {
        os . link ( pre_lock . name , f . name + ".lock" );
        dotlock_done = true;
        // } catch  ( AttributeError , PermissionError )  {
        os . rename ( pre_lock . name , f . name + ".lock" );
        dotlock_done = true;
        } else {
        os . unlink ( pre_lock . name );
        // } catch  FileExistsError  {
        os . remove ( pre_lock . name );
        panic!("ExternalClashError ( "dot lock unavailable: %s" %");
        f . name );
        // } catch   {
        if fcntl {
        fcntl . lockf ( f , fcntl . LOCK_UN );
        if dotlock_done {
        os . remove ( f . name + ".lock" );
        panic!("");
        pub fn _unlock_file ( f )  {
        "Unlock file f using lockf && dot locking.";
        if fcntl {
        fcntl . lockf ( f , fcntl . LOCK_UN );
        if os . path . exists ( f . name + ".lock" ) {
        os . remove ( f . name + ".lock" );
        pub fn _create_carefully ( path )  {
        "Create a file if it doesn't exist && open for reading && writing.";
        fd = os . open ( path , os . O_CREAT | os . O_EXCL | os . O_RDWR , 0 o666 );
        // try {
        return  open ( path , "rb+" );
        // } finally {
        os . close ( fd );
        pub fn _create_temporary ( path )  {
        "Create a temp file based on path && open for reading && writing.";
        return  _create_carefully ( "%s.%s.%s.%s" % ( path , int ( time . time ( ) ) ,;
        socket . gethostname ( ) ,;
        os . getpid ( ) ) );
        pub fn _sync_flush ( f )  {
        "Ensure changes to file f are physically on disk.";
        f . flush ( );
        if hasattr ( os , "fsync" ) {
        os . fsync ( f . fileno ( ) );
        pub fn _sync_close ( f )  {
        "Close file f, ensuring all changes are physically on disk.";
        _sync_flush ( f );
        f . close ( );
        class Error ( Exception ) ;
        "Raised for module-specific errors.";
        class NoSuchMailboxError ( Error ) ;
        "The specified mailbox does !exist && won't be created.";
        class NotEmptyError ( Error ) ;
        "The specified mailbox == !empty && deletion was requested.";
        class ExternalClashError ( Error ) ;
        "Another process caused an action to fail.";
        class FormatError ( Error ) ;
        "A file appears to have an invalid format.";
}

