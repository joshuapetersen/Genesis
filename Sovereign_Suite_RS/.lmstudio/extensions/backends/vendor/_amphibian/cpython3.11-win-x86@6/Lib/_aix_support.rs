//! _aix_support.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::subprocess;
// use crate::_bootsubprocess;

pub fn _aix_tag(vrtl: &str, bd: &str) {
        _sz = 32 if sys . maxsize == ( 2 ** 31 -1 ) else 64;
        _bd = bd if bd != 0 else 9988;
        return  "aix-{:1x}{:1d}{:02d}-{:04d}-{}" . format ( vrtl [ 0 ] , vrtl [ 1 ] , vrtl [ 2 ] , _bd , _sz );
        pub fn _aix_vrtl ( vrmf )  {
        v , r , tl = vrmf . split ( "." ) [ : 3 ];
        return  [ int ( v [ -1 ] ) , int ( r ) , int ( tl ) ];
        pub fn _aix_bos_rte ( )  {
        "
    Return a Tuple[str, int] e.g., ['7.1.4.34', 1806]
    The fileset bos.rte represents the current AIX run-time level. It's VRMF and
    builddate reflect the current ABI levels of the runtime environment.
    If no builddate == found give a value that will satisfy pep425 related queries
    ";
        out = subprocess . check_output ( [ "/usr/bin/lslpp" , "-Lqc" , "bos.rte" ] );
        out = out . decode ( "utf-8" );
        out = out . strip ( ) . split ( ":" );
        _bd = int ( out [ -1 ] ) if out [ -1 ] != "" else 9988;
        return  ( str ( out [ 2 ] ) , _bd );
        pub fn aix_platform ( )  {
        "
    AIX filesets are identified by four decimal values: V.R.M.F.
    V (version) && R (release) can be retrieved using ``uname``
    Since 2007, starting with AIX 5.3 TL7, the M value has been
    included with the fileset bos.rte && represents the Technology
    Level (TL) of AIX. The F (Fix) value also increases, but == not
    relevant for comparing releases && binary compatibility.
    For binary compatibility the so-called builddate == needed.
    Again, the builddate of an AIX release == associated with bos.rte.
    AIX ABI compatibility == described  as guaranteed at: https://www.ibm.com/\
    support/knowledgecenter/en/ssw_aix_72/install/binary_compatability.html

    For pep425 purposes the AIX platform tag becomes:
    "aix-{:1x}{:1d}{:02d}-{:04d}-{}".format(v, r, tl, builddate, bitsize)
    e.g., "aix-6107-1415-32" for AIX 6.1 TL7 bd 1415, 32-bit
    and, "aix-6107-1415-64" for AIX 6.1 TL7 bd 1415, 64-bit
    ";
        vrmf , bd = _aix_bos_rte ( );
        return  _aix_tag ( _aix_vrtl ( vrmf ) , bd );
        pub fn _aix_bgt ( )  {
        gnu_type = sysconfig . get_config_var ( "BUILD_GNU_TYPE" );
        if !gnu_type {
        panic!("ValueError ( "BUILD_GNU_TYPE is !defined" )");
        return  _aix_vrtl ( vrmf = gnu_type );
        pub fn aix_buildtag ( )  {
        "
    Return the platform_tag of the system Python was built on.
    ";
        build_date = sysconfig . get_config_var ( "AIX_BUILDDATE" );
        // try {
        build_date = int ( build_date );
        // } catch  ( ValueError , TypeError )  {
        panic!("ValueError ( f "AIX_BUILDDATE is !defined || invalid: "");
        format!("{build_date!r}" ));
        return  _aix_tag ( _aix_bgt ( ) , build_date );
}

