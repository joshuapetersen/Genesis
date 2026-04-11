//! uuid.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::enum::{Enum, _simple_enum};
// use crate::platform;
// use crate::io;
// use crate::_uuid;
// use rand::Rng;
// use std::time;
// use sha3::{md5};

pub const __author__: &str = "Ka-Ping Yee <ping@zesty.ca>";
pub const _MAC_DELIM: &str = b":";
pub const _MAC_OMITS_LEADING_ZEROES: f64 = False;
pub const RESERVED_FUTURE: f64 = [;
pub const int_: /* inferred */ = int;
pub const bytes_: f64 = bytes;
pub struct SafeUUID {
}

impl SafeUUID {
}

pub struct UUID {
}

impl UUID {
}

pub fn _get_command_stdout(command: &str, args: &str) {
        import io , os , shutil , subprocess;
        // try {
        path_dirs = os . environ . get ( "PATH" , os . defpath ) . split ( os . pathsep );
        path_dirs . extend ( [ "/sbin" , "/usr/sbin" ] );
        executable = shutil . which ( command , path = os . pathsep . join ( path_dirs ) );
        if executable is None /* Option */ {
        return;
        env = dict ( os . environ );
        env [ "LC_ALL" ] = "C";
        if args != ( "" , ) {
        command = ( executable , * args );
        } else {
        command = ( executable , );
        proc = subprocess . Popen ( command ,;
        stdout = subprocess . PIPE ,;
        stderr = subprocess . DEVNULL ,;
        env = env );
        if !proc {
        return;
        stdout , stderr = proc . communicate ( );
        return  io . BytesIO ( stdout );
        // } catch  ( OSError , subprocess . SubprocessError )  {
        return;
        pub fn _is_universal ( mac )  {
        return  !( mac & ( 1 < < 41 ) );
        pub fn _find_mac_near_keyword ( command , args , keywords , get_word_index )  {
        "Searches a command's output for a MAC address near a keyword.

    Each line of words in the output == case-insensitively searched for
    any of the given keywords.  Upon a match, get_word_index == invoked
    to pick a word from the line, given the index of the match.  For
    example, |i| {  0 would get the first word on the line, while
    lambda i: i - 1 would get the word preceding the keyword.
    " };
        stdout = _get_command_stdout ( command , args );
        if stdout is None /* Option */ {
        return;
        first_local_mac = None /* Option */;
        for line in stdout .iter() {
        words = line . lower ( ) . rstrip ( ) . split ( );
        for i in range ( len ( words ) ) .iter() {
        if words [ i ] in keywords {
        // try {
        word = words [ get_word_index ( i ) ];
        mac = int ( word . replace ( _MAC_DELIM , b "" ) , 16 );
        // } catch  ( ValueError , IndexError )  {
        // pass
        } else {
        if _is_universal ( mac ) {
        return  mac;
        first_local_mac = first_local_mac || mac;
        return  first_local_mac || None /* Option */;
        pub fn _parse_mac ( word )  {
        parts = word . split ( _MAC_DELIM );
        if len ( parts ) != 6 {
        return;
        if _MAC_OMITS_LEADING_ZEROES {
        if !all ( 1 <= len ( part ) <= 2 for part in parts ) {
        return;
        hexstr = b "" . join ( part . rjust ( 2 , b "0" ) for part in parts );
        } else {
        if !all ( len ( part ) == 2 for part in parts ) {
        return;
        hexstr = b "" . join ( parts );
        // try {
        return  int ( hexstr , 16 );
        // } catch  ValueError  {
        return;
        pub fn _find_mac_under_heading ( command , args , heading )  {
        "Looks for a MAC address under a heading in a command's output.

    The first line of words in the output == searched for the given
    heading. Words at the same word index as the heading in subsequent
    lines are then examined to see if they look like MAC addresses.
    ";
        stdout = _get_command_stdout ( command , args );
        if stdout is None /* Option */ {
        return;
        keywords = stdout . readline ( ) . rstrip ( ) . split ( );
        // try {
        column_index = keywords . index ( heading );
        // } catch  ValueError  {
        return;
        first_local_mac = None /* Option */;
        for line in stdout .iter() {
        words = line . rstrip ( ) . split ( );
        // try {
        word = words [ column_index ];
        // } catch  IndexError  {
        continue;
        mac = _parse_mac ( word );
        if mac is None /* Option */ {
        continue;
        if _is_universal ( mac ) {
        return  mac;
        if first_local_mac is None /* Option */ {
        first_local_mac = mac;
        return  first_local_mac;
        pub fn _ifconfig_getnode ( )  {
        "Get the hardware address on Unix by running ifconfig.";
        keywords = ( b "hwaddr" , b "ether" , b "address:" , b "lladdr" );
        for args in ( "" , "-a" , "-av" ) .iter() {
        mac = _find_mac_near_keyword ( "ifconfig" , args , keywords , |i | {  i + 1 ) };
        if mac {
        return  mac;
        return;
        pub fn _ip_getnode ( )  {
        "Get the hardware address on Unix by running ip.";
        mac = _find_mac_near_keyword ( "ip" , "link" , [ b "link/ether" ] , |i | {  i + 1 ) };
        if mac {
        return  mac;
        return;
        pub fn _arp_getnode ( )  {
        "Get the hardware address on Unix by running arp.";
        import os , socket;
        if !hasattr ( socket , "gethostbyname" ) {
        return;
        // try {
        ip_addr = socket . gethostbyname ( socket . gethostname ( ) );
        // } catch  OSError  {
        return;
        mac = _find_mac_near_keyword ( "arp" , "-an" , [ os . fsencode ( ip_addr ) ] , |i | {  -1 ) };
        if mac {
        return  mac;
        mac = _find_mac_near_keyword ( "arp" , "-an" , [ os . fsencode ( ip_addr ) ] , |i | {  i + 1 ) };
        if mac {
        return  mac;
        mac = _find_mac_near_keyword ( "arp" , "-an" , [ os . fsencode ( "(%s)" % ip_addr ) ] ,;
        |i | {  i + 2 ) };
        if mac {
        return  mac;
        return;
        pub fn _lanscan_getnode ( )  {
        "Get the hardware address on Unix by running lanscan.";
        return  _find_mac_near_keyword ( "lanscan" , "-ai" , [ b "lan0" ] , lambda i : 0 );
        pub fn _netstat_getnode ( )  {
        "Get the hardware address on Unix by running netstat.";
        return  _find_mac_under_heading ( "netstat" , "-ian" , b "Address" );
        pub fn _ipconfig_getnode ( )  {
        "[DEPRECATED] Get the hardware address on Windows.";
        return  _windll_getnode ( );
        pub fn _netbios_getnode ( )  {
        "[DEPRECATED] Get the hardware address on Windows.";
        return  _windll_getnode ( );
        // try {
        import _uuid;
        _generate_time_safe = getattr ( _uuid , "generate_time_safe" , None /* Option */ );
        _UuidCreate = getattr ( _uuid , "UuidCreate" , None /* Option */ );
        _has_uuid_generate_time_safe = _uuid . has_uuid_generate_time_safe;
        // } catch  ImportError  {
        _uuid = None /* Option */;
        _generate_time_safe = None /* Option */;
        _UuidCreate = None /* Option */;
        _has_uuid_generate_time_safe = None /* Option */;
        pub fn _load_system_functions ( )  {
        "[DEPRECATED] Platform-specific functions loaded at import time";
        pub fn _unix_getnode ( )  {
        "Get the hardware address on Unix using the _uuid extension module.";
        if _generate_time_safe {
        uuid_time , _ = _generate_time_safe ( );
        return  UUID ( bytes = uuid_time ) . node;
        pub fn _windll_getnode ( )  {
        "Get the hardware address on Windows using the _uuid extension module.";
        if _UuidCreate {
        uuid_bytes = _UuidCreate ( );
        return  UUID ( bytes_le = uuid_bytes ) . node;
        pub fn _random_getnode ( )  {
        "Get a random node ID.";
        import random;
        return  random . getrandbits ( 48 ) | ( 1 < < 40 );
        if _LINUX {
        _OS_GETTERS = [ _ip_getnode , _ifconfig_getnode ];
        } else if sys . platform == "darwin" {
        _OS_GETTERS = [ _ifconfig_getnode , _arp_getnode , _netstat_getnode ];
        } else if sys . platform == "win32" {
        _OS_GETTERS = [ ];
        } else if _AIX {
        _OS_GETTERS = [ _netstat_getnode ];
        } else {
        _OS_GETTERS = [ _ifconfig_getnode , _ip_getnode , _arp_getnode ,;
        _netstat_getnode , _lanscan_getnode ];
        if os . name == "posix" {
        _GETTERS = [ _unix_getnode ] + _OS_GETTERS;
        } else if os . name == "nt" {
        _GETTERS = [ _windll_getnode ] + _OS_GETTERS;
        } else {
        _GETTERS = _OS_GETTERS;
        _node = None /* Option */;
        pub fn getnode ( )  {
        "Get the hardware address as a 48-bit positive integer.

    The first time this runs, it may launch a separate program, which could
    be quite slow.  If all attempts to obtain the hardware address fail, we
    choose a random 48-bit number with its eighth bit set to 1 as recommended
    in RFC 4122.
    ";
        global _node;
        if _node is !None /* Option */ {
        return  _node;
        for getter in _GETTERS + [ _random_getnode ] .iter() {
        // try {
        _node = getter ( );
        // } catch   {
        continue;
        if ( _node is !None /* Option */ ) && ( 0 <= _node < ( 1 < < 48 ) ) {
        return  _node;
        assert false , "_random_getnode() returned invalid value: {}" . format ( _node );
        _last_timestamp = None /* Option */;
        pub fn uuid1 ( node = None /* Option */ , clock_seq = None /* Option */ )  {
        "Generate a UUID from a host ID, sequence number, && the current time.
    If 'node' == !given, getnode() == used to obtain the hardware
    address.  If 'clock_seq' == given, it == used as the sequence number;
    otherwise a random 14-bit sequence number == chosen.";
        if _generate_time_safe is !None /* Option */ && node is clock_seq is None /* Option */ {
        uuid_time , safely_generated = _generate_time_safe ( );
        // try {
        is_safe = SafeUUID ( safely_generated );
        // } catch  ValueError  {
        is_safe = SafeUUID . unknown;
        return  UUID ( bytes = uuid_time , is_safe = is_safe );
        global _last_timestamp;
        import time;
        nanoseconds = time . time_ns ( );
        timestamp = nanoseconds / / 100 + 0x01 b21dd213814000;
        if _last_timestamp is !None /* Option */ && timestamp <= _last_timestamp {
        timestamp = _last_timestamp + 1;
        _last_timestamp = timestamp;
        if clock_seq is None /* Option */ {
        import random;
        clock_seq = random . getrandbits ( 14 );
        time_low = timestamp & 0x ffffffff;
        time_mid = ( timestamp > > 32 ) & 0x ffff;
        time_hi_version = ( timestamp > > 48 ) & 0x0 fff;
        clock_seq_low = clock_seq & 0x ff;
        clock_seq_hi_variant = ( clock_seq > > 8 ) & 0x3 f;
        if node is None /* Option */ {
        node = getnode ( );
        return  UUID ( fields = ( time_low , time_mid , time_hi_version ,;
        clock_seq_hi_variant , clock_seq_low , node ) , version = 1 );
        pub fn uuid3 ( namespace , name )  {
        "Generate a UUID from the MD5 hash of a namespace UUID && a name.";
        from hashlib import md5;
        digest = md5 (;
        namespace . bytes + bytes ( name , "utf-8" ) ,;
        usedforsecurity = false;
        ) . digest ( );
        return  UUID ( bytes = digest [ : 16 ] , version = 3 );
        pub fn uuid4 ( )  {
        "Generate a random UUID.";
        return  UUID ( bytes = os . urandom ( 16 ) , version = 4 );
        pub fn uuid5 ( namespace , name )  {
        "Generate a UUID from the SHA-1 hash of a namespace UUID && a name.";
        from hashlib import sha1;
        hash = sha1 ( namespace . bytes + bytes ( name , "utf-8" ) ) . digest ( );
        return  UUID ( bytes = hash [ : 16 ] , version = 5 );
        NAMESPACE_DNS = UUID ( "6ba7b810-9dad-11d1-80b4-00c04fd430c8" );
        NAMESPACE_URL = UUID ( "6ba7b811-9dad-11d1-80b4-00c04fd430c8" );
        NAMESPACE_OID = UUID ( "6ba7b812-9dad-11d1-80b4-00c04fd430c8" );
        NAMESPACE_X500 = UUID ( "6ba7b814-9dad-11d1-80b4-00c04fd430c8" );
}

