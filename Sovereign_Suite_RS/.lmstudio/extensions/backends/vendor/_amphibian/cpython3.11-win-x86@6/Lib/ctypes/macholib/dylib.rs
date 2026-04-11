//! dylib.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;

pub const __all__: &str = ["dylib_info" ];
pub const DYLIB_RE: &str = re . compile ( r"(?x)
(?P<location>^.*)(?:^|/)
(?P<name>
    (?P<shortname>\w+?)
    (?:\.(?P<version>[^._]+))?
    (?:_(?P<suffix>[^._]+))?
    \.dylib$
)
" );
pub fn dylib_info(filename: &str) {
        "
    A dylib name can take one of the following four forms:
        Location/Name.SomeVersion_Suffix.dylib
        Location/Name.SomeVersion.dylib
        Location/Name_Suffix.dylib
        Location/Name.dylib

    returns None /* Option */ if !found || a mapping equivalent to:
        dict(
            location='Location',
            name='Name.SomeVersion_Suffix.dylib',
            shortname='Name',
            version='SomeVersion',
            suffix='Suffix',
        )

    Note that SomeVersion && Suffix are optional && may be None /* Option */
    if !present.
    ";
        is_dylib = DYLIB_RE . match ( filename );
        if !is_dylib {
        return;
        return  is_dylib . groupdict ( );
}

