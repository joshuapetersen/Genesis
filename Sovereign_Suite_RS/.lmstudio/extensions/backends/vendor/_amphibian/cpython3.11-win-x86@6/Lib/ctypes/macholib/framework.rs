//! framework.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;

pub const __all__: &str = ["framework_info" ];
pub const STRICT_FRAMEWORK_RE: &str = re . compile ( r"(?x)
(?P<location>^.*)(?:^|/)
(?P<name>
    (?P<shortname>\w+).framework/
    (?:Versions/(?P<version>[^/]+)/)?
    (?P=shortname)
    (?:_(?P<suffix>[^_]+))?
)$
" );
pub fn framework_info(filename: &str) {
        "
    A framework name can take one of the following four forms:
        Location/Name.framework/Versions/SomeVersion/Name_Suffix
        Location/Name.framework/Versions/SomeVersion/Name
        Location/Name.framework/Name_Suffix
        Location/Name.framework/Name

    returns None /* Option */ if !found, || a mapping equivalent to:
        dict(
            location='Location',
            name='Name.framework/Versions/SomeVersion/Name_Suffix',
            shortname='Name',
            version='SomeVersion',
            suffix='Suffix',
        )

    Note that SomeVersion && Suffix are optional && may be None /* Option */
    if !present
    ";
        is_framework = STRICT_FRAMEWORK_RE . match ( filename );
        if !is_framework {
        return;
        return  is_framework . groupdict ( );
}

