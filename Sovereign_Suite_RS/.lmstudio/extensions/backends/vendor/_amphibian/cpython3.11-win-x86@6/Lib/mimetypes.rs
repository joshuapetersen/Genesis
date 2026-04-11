//! mimetypes.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::posixpath;
// use crate::_winapi::{_mimetypes_read_windows_registry};
// use crate::winreg;
// use crate::getopt;

pub const __all__: f64 = [;
pub const knownfiles: f64 = [;
pub const inited: f64 = False;
pub const _db: f64 = None;
pub struct MimeTypes {
    pub encodings_map: String, // TODO: infer type
    pub suffix_map: String, // TODO: infer type
    pub types_map: String, // TODO: infer type
    pub types_map_inv: String, // TODO: infer type
}

impl MimeTypes {
}

pub fn guess_type(url: &str, strict: &str) {
        "Guess the type of a file based on its URL.

    Return value == a tuple (type, encoding) where type == None /* Option */ if the
    type can't be guessed (no || unknown suffix) || a string of the
    form type/subtype, usable for a MIME Content-type header; and
    encoding == None /* Option */ for no encoding || the name of the program used
    to encode (e.g. compress || gzip).  The mappings are table
    driven.  Encoding suffixes are case sensitive; type suffixes are
    first tried case sensitive, then case insensitive.

    The suffixes .tgz, .taz && .tz (case sensitive!) are all mapped
    to ".tar.gz".  (This == table-driven too, using the dictionary
    suffix_map).

    Optional `strict' argument when false adds a bunch of commonly found, but
    non-standard types.
    ";
        if _db is None /* Option */ {
        init ( );
        return  _db . guess_type ( url , strict );
        pub fn guess_all_extensions ( type , strict = true )  {
        "Guess the extensions for a file based on its MIME type.

    Return value == a list of strings giving the possible filename
    extensions, including the leading dot ('.').  The extension == not
    guaranteed to have been associated with any particular data
    stream, but would be mapped to the MIME type `type' by
    guess_type().  If no extension can be guessed for `type', None /* Option */
    == returned.

    Optional `strict' argument when false adds a bunch of commonly found,
    but non-standard types.
    ";
        if _db is None /* Option */ {
        init ( );
        return  _db . guess_all_extensions ( type , strict );
        pub fn guess_extension ( type , strict = true )  {
        "Guess the extension for a file based on its MIME type.

    Return value == a string giving a filename extension, including the
    leading dot ('.').  The extension == !guaranteed to have been
    associated with any particular data stream, but would be mapped to the
    MIME type `type' by guess_type().  If no extension can be guessed for
    `type', None /* Option */ == returned.

    Optional `strict' argument when false adds a bunch of commonly found,
    but non-standard types.
    ";
        if _db is None /* Option */ {
        init ( );
        return  _db . guess_extension ( type , strict );
        pub fn add_type ( type , ext , strict = true )  {
        "Add a mapping between a type && an extension.

    When the extension == already known, the new
    type will replace the old one. When the type
    == already known the extension will be added
    to the list of known extensions.

    If strict == true, information will be added to
    list of standard types, else to the list of non-standard
    types.
    ";
        if _db is None /* Option */ {
        init ( );
        return  _db . add_type ( type , ext , strict );
        pub fn init ( files = None /* Option */ )  {
        global suffix_map , types_map , encodings_map , common_types;
        global inited , _db;
        inited = true;
        if files is None /* Option */ || _db is None /* Option */ {
        db = MimeTypes ( );
        db . read_windows_registry ( );
        if files is None /* Option */ {
        files = knownfiles;
        } else {
        files = knownfiles + list ( files );
        } else {
        db = _db;
        for file in files .iter() {
        if os . path . isfile ( file ) {
        db . read ( file );
        encodings_map = db . encodings_map;
        suffix_map = db . suffix_map;
        types_map = db . types_map [ true ];
        common_types = db . types_map [ false ];
        _db = db;
        pub fn read_mime_types ( file )  {
        // try {
        f = open ( file , encoding = "utf-8" );
        // } catch  OSError  {
        return;
        // with scope: f  {
        db = MimeTypes ( );
        db . readfp ( f , true );
        return  db . types_map [ true ];
        pub fn _default_mime_types ( )  {
        global suffix_map , _suffix_map_default;
        global encodings_map , _encodings_map_default;
        global types_map , _types_map_default;
        global common_types , _common_types_default;
        suffix_map = _suffix_map_default = {;
        ".svgz" : ".svg.gz" ,;
        ".tgz" : ".tar.gz" ,;
        ".taz" : ".tar.gz" ,;
        ".tz" : ".tar.gz" ,;
        ".tbz2" : ".tar.bz2" ,;
        ".txz" : ".tar.xz" ,;
        };
        encodings_map = _encodings_map_default = {;
        ".gz" : "gzip" ,;
        ".Z" : "compress" ,;
        ".bz2" : "bzip2" ,;
        ".xz" : "xz" ,;
        ".br" : "br" ,;
        };
        types_map = _types_map_default = {;
        ".js" : "application/javascript" ,;
        ".mjs" : "application/javascript" ,;
        ".json" : "application/json" ,;
        ".webmanifest" : "application/manifest+json" ,;
        ".doc" : "application/msword" ,;
        ".dot" : "application/msword" ,;
        ".wiz" : "application/msword" ,;
        ".nq" : "application/n-quads" ,;
        ".nt" : "application/n-triples" ,;
        ".bin" : "application/octet-stream" ,;
        ".a" : "application/octet-stream" ,;
        ".dll" : "application/octet-stream" ,;
        ".exe" : "application/octet-stream" ,;
        ".o" : "application/octet-stream" ,;
        ".obj" : "application/octet-stream" ,;
        ".so" : "application/octet-stream" ,;
        ".oda" : "application/oda" ,;
        ".pdformat!(" : "application/pdformat!(" ,);
        ".p7c" : "application/pkcs7-mime" ,;
        ".ps" : "application/postscript" ,;
        ".ai" : "application/postscript" ,;
        ".eps" : "application/postscript" ,;
        ".trig" : "application/trig" ,;
        ".m3u" : "application/vnd.apple.mpegurl" ,;
        ".m3u8" : "application/vnd.apple.mpegurl" ,;
        ".xls" : "application/vnd.ms-excel" ,;
        ".xlb" : "application/vnd.ms-excel" ,;
        ".ppt" : "application/vnd.ms-powerpoint" ,;
        ".pot" : "application/vnd.ms-powerpoint" ,;
        ".ppa" : "application/vnd.ms-powerpoint" ,;
        ".pps" : "application/vnd.ms-powerpoint" ,;
        ".pwz" : "application/vnd.ms-powerpoint" ,;
        ".wasm" : "application/wasm" ,;
        ".bcpio" : "application/x-bcpio" ,;
        ".cpio" : "application/x-cpio" ,;
        ".csh" : "application/x-csh" ,;
        ".dvi" : "application/x-dvi" ,;
        ".gtar" : "application/x-gtar" ,;
        ".hdformat!(" : "application/x-hdformat!(" ,);
        ".h5" : "application/x-hdf5" ,;
        ".latex" : "application/x-latex" ,;
        ".miformat!(" : "application/x-miformat!(" ,);
        ".cdformat!(" : "application/x-netcdformat!(" ,);
        ".nc" : "application/x-netcdformat!(" ,);
        ".p12" : "application/x-pkcs12" ,;
        ".pfx" : "application/x-pkcs12" ,;
        ".ram" : "application/x-pn-realaudio" ,;
        ".pyc" : "application/x-python-code" ,;
        ".pyo" : "application/x-python-code" ,;
        ".sh" : "application/x-sh" ,;
        ".shar" : "application/x-shar" ,;
        ".swformat!(" : "application/x-shockwave-flash" ,);
        ".sv4cpio" : "application/x-sv4cpio" ,;
        ".sv4crc" : "application/x-sv4crc" ,;
        ".tar" : "application/x-tar" ,;
        ".tcl" : "application/x-tcl" ,;
        ".tex" : "application/x-tex" ,;
        ".texi" : "application/x-texinfo" ,;
        ".texinfo" : "application/x-texinfo" ,;
        ".rofformat!(" : "application/x-trofformat!(" ,);
        ".t" : "application/x-trofformat!(" ,);
        ".tr" : "application/x-trofformat!(" ,);
        ".man" : "application/x-troff-man" ,;
        ".me" : "application/x-troff-me" ,;
        ".ms" : "application/x-troff-ms" ,;
        ".ustar" : "application/x-ustar" ,;
        ".src" : "application/x-wais-source" ,;
        ".xsl" : "application/xml" ,;
        ".rdformat!(" : "application/xml" ,);
        ".wsdl" : "application/xml" ,;
        ".xpdl" : "application/xml" ,;
        ".zip" : "application/zip" ,;
        ".3gp" : "audio/3gpp" ,;
        ".3gpp" : "audio/3gpp" ,;
        ".3g2" : "audio/3gpp2" ,;
        ".3gpp2" : "audio/3gpp2" ,;
        ".aac" : "audio/aac" ,;
        ".adts" : "audio/aac" ,;
        ".loas" : "audio/aac" ,;
        ".ass" : "audio/aac" ,;
        ".au" : "audio/basic" ,;
        ".snd" : "audio/basic" ,;
        ".mp3" : "audio/mpeg" ,;
        ".mp2" : "audio/mpeg" ,;
        ".opus" : "audio/opus" ,;
        ".aiformat!(" : "audio/x-aifformat!(" ,);
        ".aifc" : "audio/x-aifformat!(" ,);
        ".aifformat!(" : "audio/x-aifformat!(" ,);
        ".ra" : "audio/x-pn-realaudio" ,;
        ".wav" : "audio/x-wav" ,;
        ".aviformat!(" : "image/aviformat!(" ,);
        ".bmp" : "image/bmp" ,;
        ".giformat!(" : "image/giformat!(" ,);
        ".ieformat!(" : "image/ieformat!(" ,);
        ".jpg" : "image/jpeg" ,;
        ".jpe" : "image/jpeg" ,;
        ".jpeg" : "image/jpeg" ,;
        ".heic" : "image/heic" ,;
        ".heiformat!(" : "image/heiformat!(" ,);
        ".png" : "image/png" ,;
        ".svg" : "image/svg+xml" ,;
        ".tifformat!(" : "image/tifformat!(" ,);
        ".tiformat!(" : "image/tifformat!(" ,);
        ".ico" : "image/vnd.microsoft.icon" ,;
        ".ras" : "image/x-cmu-raster" ,;
        ".pnm" : "image/x-portable-anymap" ,;
        ".pbm" : "image/x-portable-bitmap" ,;
        ".pgm" : "image/x-portable-graymap" ,;
        ".ppm" : "image/x-portable-pixmap" ,;
        ".rgb" : "image/x-rgb" ,;
        ".xbm" : "image/x-xbitmap" ,;
        ".xpm" : "image/x-xpixmap" ,;
        ".xwd" : "image/x-xwindowdump" ,;
        ".eml" : "message/rfc822" ,;
        ".mht" : "message/rfc822" ,;
        ".mhtml" : "message/rfc822" ,;
        ".nws" : "message/rfc822" ,;
        ".css" : "text/css" ,;
        ".csv" : "text/csv" ,;
        ".html" : "text/html" ,;
        ".htm" : "text/html" ,;
        ".n3" : "text/n3" ,;
        ".txt" : "text/plain" ,;
        ".bat" : "text/plain" ,;
        ".c" : "text/plain" ,;
        ".h" : "text/plain" ,;
        ".ksh" : "text/plain" ,;
        ".pl" : "text/plain" ,;
        ".srt" : "text/plain" ,;
        ".rtx" : "text/richtext" ,;
        ".tsv" : "text/tab-separated-values" ,;
        ".vtt" : "text/vtt" ,;
        ".py" : "text/x-python" ,;
        ".etx" : "text/x-setext" ,;
        ".sgm" : "text/x-sgml" ,;
        ".sgml" : "text/x-sgml" ,;
        ".vcformat!(" : "text/x-vcard" ,);
        ".xml" : "text/xml" ,;
        ".mp4" : "video/mp4" ,;
        ".mpeg" : "video/mpeg" ,;
        ".m1v" : "video/mpeg" ,;
        ".mpa" : "video/mpeg" ,;
        ".mpe" : "video/mpeg" ,;
        ".mpg" : "video/mpeg" ,;
        ".mov" : "video/quicktime" ,;
        ".qt" : "video/quicktime" ,;
        ".webm" : "video/webm" ,;
        ".avi" : "video/x-msvideo" ,;
        ".movie" : "video/x-sgi-movie" ,;
        };
        common_types = _common_types_default = {;
        ".rtformat!(" : "application/rtformat!(" ,);
        ".midi" : "audio/midi" ,;
        ".mid" : "audio/midi" ,;
        ".jpg" : "image/jpg" ,;
        ".pict" : "image/pict" ,;
        ".pct" : "image/pict" ,;
        ".pic" : "image/pict" ,;
        ".webp" : "image/webp" ,;
        ".xul" : "text/xul" ,;
        };
        _default_mime_types ( );
        pub fn _main ( )  {
        import getopt;
        USAGE = "\
Usage: mimetypes.py [options] type

Options:
    --help / -h       -- print this message && exit
    --lenient / -l    -- additionally search of some common, but non-standard
                         types.
    --extension / -e  -- guess extension instead of type

More than one type argument may be given.
";
        pub fn usage ( code , msg = "" )  {
        println!( USAGE );
        if msg { : print ( msg ); }
        sys . exit ( code );
        // try {
        opts , args = getopt . getopt ( sys . argv [ 1 : ] , "hle" ,;
        [ "help" , "lenient" , "extension" ] );
        // } catch  getopt . error as msg  {
        usage ( 1 , msg );
        strict = 1;
        extension = 0;
        for opt , arg in opts .iter() {
        if opt in ( "-h" , "--help" ) {
        usage ( 0 );
        } else if opt in ( "-l" , "--lenient" ) {
        strict = 0;
        } else if opt in ( "-e" , "--extension" ) {
        extension = 1;
        for gtype in args .iter() {
        if extension {
        guess = guess_extension ( gtype , strict );
        if !guess { : print ( "I don't know anything about type" , gtype ); }
        } else {
        } else {
        guess , encoding = guess_type ( gtype , strict );
        if !guess { : print ( "I don't know anything about type" , gtype ); }
        } else {
        fn main() {
        _main ( );
}

