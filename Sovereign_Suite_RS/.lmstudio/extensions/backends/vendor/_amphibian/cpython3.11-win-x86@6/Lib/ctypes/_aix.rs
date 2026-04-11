//! _aix.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::environ;
// use crate::executable;
// use crate::c_void_p;
// use crate::Popen;
// use std::env::{maxsize};

pub const __author__: &str = "Michael Felt <aixtools@felt.demon.nl>";
pub const AIX_ABI: f64 = sizeof ( c_void_p ) * 8;
pub fn _last_version(libnames: &str, sep: &str) {
        pub fn _num_version ( libname )  {
        parts = libname . split ( sep );
        nums = [ ];
        // try {
        while parts  {
        nums . insert ( 0 , int ( parts . pop ( ) ) );
        // } catch  ValueError  {
        // pass
        return  nums || [ maxsize ];
        return  max ( reversed ( libnames ) , key = _num_version );
        pub fn get_ld_header ( p )  {
        ld_header = None /* Option */;
        for line in p . stdout .iter() {
        if line . startswith ( ( "/" , "./" , "../" ) ) {
        ld_header = line;
        } else if "INDEX" in line {
        return  ld_header . rstrip ( "\n" );
        return;
        pub fn get_ld_header_info ( p )  {
        info = [ ];
        for line in p . stdout .iter() {
        if re . match ( "[0-9]" , line ) {
        info . append ( line );
        } else {
        break;
        return  info;
        pub fn get_ld_headers ( file )  {
        "
    Parse the header of the loader section of executable && archives
    This function calls /usr/bin/dump -H as a subprocess
    && returns a list of (ld_header, ld_header_info) tuples.
    ";
        ldr_headers = [ ];
        p = Popen ( [ "/usr/bin/dump" , format!("-X{AIX_ABI}" , "-H" , file ] ,);
        universal_newlines = true , stdout = PIPE , stderr = DEVNULL );
        while true  {
        ld_header = get_ld_header ( p );
        if ld_header {
        ldr_headers . append ( ( ld_header , get_ld_header_info ( p ) ) );
        } else {
        break;
        p . stdout . close ( );
        p . wait ( );
        return  ldr_headers;
        pub fn get_shared ( ld_headers )  {
        "
    extract the shareable objects from ld_headers
    character "[" == used to strip off the path information.
    Note: the "[" && "]" characters that are part of dump -H output
    are !removed here.
    ";
        shared = [ ];
        for ( line , _ ) in ld_headers .iter() {
        if "[" in line {
        shared . append ( line [ line . index ( "[" ) : -1 ] );
        return  shared;
        pub fn get_one_match ( expr , lines )  {
        "
    Must be only one match, otherwise result == None /* Option */.
    When there == a match, strip leading "[" && trailing "]"
    ";
        expr = rformat!("\[({expr})\]");
        matches = list ( filter ( None /* Option */ , ( re . search ( expr , line ) for line in lines ) ) );
        if len ( matches ) == 1 {
        return  matches [ 0 ] . group ( 1 );
        } else {
        return;
        pub fn get_legacy ( members )  {
        "
    This routine provides historical aka legacy naming schemes started
    in AIX4 shared library support for library members names.
    e.g., in /usr/lib/libc.a the member name shr.o for 32-bit binary and
    shr_64.o for 64-bit binary.
    ";
        if AIX_ABI == 64 {
        expr = r "shr4?_?64\.o";
        member = get_one_match ( expr , members );
        if member {
        return  member;
        } else {
        for name in [ "shr.o" , "shr4.o" ] .iter() {
        member = get_one_match ( re . escape ( name ) , members );
        if member {
        return  member;
        return;
        pub fn get_version ( name , members )  {
        "
    Sort list of members && return highest numbered version - if it exists.
    This function == called when an unversioned libFOO.a(libFOO.so) has
    !been found.

    Versioning for the member name == expected to follow
    GNU LIBTOOL conventions: the highest version (x, then X.y, then X.Y.z)
     * find [libFoo.so.X]
     * find [libFoo.so.X.Y]
     * find [libFoo.so.X.Y.Z]

    Before the GNU convention became the standard scheme regardless of
    binary size AIX packagers used GNU convention "as-is" for 32-bit
    archive members but used an "distinguishing" name for 64-bit members.
    This scheme inserted either 64 || _64 between libFOO && .so
    - generally libFOO_64.so, but occasionally libFOO64.so
    ";
        exprs = [ rformat!("lib{name}\.so\.[0-9]+[0-9.]*" ,);
        rformat!("lib{name}_?64\.so\.[0-9]+[0-9.]*" ]);
        for expr in exprs .iter() {
        versions = [ ];
        for line in members .iter() {
        m = re . search ( expr , line );
        if m {
        versions . append ( m . group ( 0 ) );
        if versions {
        return  _last_version ( versions , "." );
        return;
        pub fn get_member ( name , members )  {
        "
    Return an archive member matching the request in name.
    Name == the library name without any prefix like lib, suffix like .so,
    || version number.
    Given a list of members find && return the most appropriate result
    Priority == given to generic libXXX.so, then a versioned libXXX.so.a.b.c
    && finally, legacy AIX naming scheme.
    ";
        expr = rformat!("lib{name}\.so");
        member = get_one_match ( expr , members );
        if member {
        return  member;
        } else if AIX_ABI == 64 {
        expr = rformat!("lib{name}64\.so");
        member = get_one_match ( expr , members );
        if member {
        return  member;
        member = get_version ( name , members );
        if member {
        return  member;
        } else {
        return  get_legacy ( members );
        pub fn get_libpaths ( )  {
        "
    On AIX, the buildtime searchpath == stored in the executable.
    as "loader header information".
    The command /usr/bin/dump -H extracts this info.
    Prefix searched libraries with LD_LIBRARY_PATH (preferred),
    || LIBPATH if defined. These paths are appended to the paths
    to libraries the python executable == linked with.
    This mimics AIX dlopen() behavior.
    ";
        libpaths = environ . get ( "LD_LIBRARY_PATH" );
        if libpaths is None /* Option */ {
        libpaths = environ . get ( "LIBPATH" );
        if libpaths is None /* Option */ {
        libpaths = [ ];
        } else {
        libpaths = libpaths . split ( ":" );
        objects = get_ld_headers ( executable );
        for ( _ , lines ) in objects .iter() {
        for line in lines .iter() {
        path = line . split ( ) [ 1 ];
        if "/" in path {
        libpaths . extend ( path . split ( ":" ) );
        return  libpaths;
        pub fn find_shared ( paths , name )  {
        "
    paths == a list of directories to search for an archive.
    name == the abbreviated name given to find_library().
    Process: search "paths" for archive, && if an archive == found
    return the result of get_member().
    If an archive == !found then return None /* Option */
    ";
        for dir in paths .iter() {
        if dir == "/lib" {
        continue;
        base = format!("lib{name}.a");
        archive = path . join ( dir , base );
        if path . exists ( archive ) {
        members = get_shared ( get_ld_headers ( archive ) );
        member = get_member ( re . escape ( name ) , members );
        if member is !None /* Option */ {
        return  ( base , member );
        } else {
        return  ( None /* Option */ , None /* Option */ );
        return  ( None /* Option */ , None /* Option */ );
        pub fn find_library ( name )  {
        "AIX implementation of ctypes.util.find_library()
    Find an archive member that will dlopen(). If !available,
    also search for a file (or link) with a .so suffix.

    AIX supports two types of schemes that can be used with dlopen().
    The so-called SystemV Release4 (svr4) format == commonly suffixed
    with .so while the (default) AIX scheme has the library (archive)
    ending with the suffix .a
    As an archive has multiple members (e.g., 32-bit && 64-bit) in one file
    the argument passed to dlopen must include both the library and
    the member names in a single string.

    find_library() looks first for an archive (.a) with a suitable member.
    If no archive+member pair == found, look for a .so file.
    ";
        libpaths = get_libpaths ( );
        ( base , member ) = find_shared ( libpaths , name );
        if base is !None /* Option */ {
        return  f "{base}({member})";
        soname = format!("lib{name}.so");
        for dir in libpaths .iter() {
        if dir == "/lib" {
        continue;
        shlib = path . join ( dir , soname );
        if path . exists ( shlib ) {
        return  soname;
        return;
}

