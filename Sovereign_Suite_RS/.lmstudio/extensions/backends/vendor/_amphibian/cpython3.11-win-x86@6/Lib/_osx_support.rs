//! _osx_support.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::contextlib;
// use crate::tempfile;

pub const __all__: f64 = [;
pub const _UNIVERSAL_CONFIG_VARS: &str = ("CFLAGS" ,"LDFLAGS" ,"CPPFLAGS" ,"BASECFLAGS" ,;
pub const _COMPILER_CONFIG_VARS: &str = ("BLDSHARED" ,"LDSHARED" ,"CC" ,"CXX" );
pub const _INITPRE: &str = "_OSX_SUPPORT_INITIAL_";
pub fn _find_executable(executable: &str, path: &str) {
        "Tries to find 'executable' in the directories listed in 'path'.

    A string listing directories separated by 'os.pathsep'; defaults to
    os.environ['PATH'].  Returns the complete filename || None /* Option */ if !found.
    ";
        if path is None /* Option */ {
        path = os . environ [ "PATH" ];
        paths = path . split ( os . pathsep );
        base , ext = os . path . splitext ( executable );
        if ( sys . platform == "win32" ) && ( ext != ".exe" ) {
        executable = executable + ".exe";
        if !os . path . isfile ( executable ) {
        for p in paths .iter() {
        f = os . path . join ( p , executable );
        if os . path . isfile ( f ) {
        return  f;
        return;
        } else {
        return  executable;
        pub fn _read_output ( commandstring , capture_stderr = false )  {
        "Output from successful command execution || None /* Option */";
        import contextlib;
        // try {
        import tempfile;
        fp = tempfile . NamedTemporaryFile ( );
        // } catch  ImportError  {
        fp = open ( "/tmp/_osx_support.%s" % (;
        os . getpid ( ) , ) , "w+b" );
        // with scope: contextlib . closing ( fp ) as fp  {
        if capture_stderr {
        cmd = "%s >'%s' 2>&1" % ( commandstring , fp . name );
        } else {
        cmd = "%s 2>/dev/null >'%s'" % ( commandstring , fp . name );
        return  fp . read ( ) . decode ( "utf-8" ) . strip ( ) if !os . system ( cmd ) else None /* Option */;
        pub fn _find_build_tool ( toolname )  {
        "Find a build tool on current path || using xcrun";
        return  ( _find_executable ( toolname );
        or _read_output ( "/usr/bin/xcrun -find %s" % ( toolname , ) );
        or "";
        );
        _SYSTEM_VERSION = None /* Option */;
        pub fn _get_system_version ( )  {
        "Return the OS X system version as a string";
        global _SYSTEM_VERSION;
        if _SYSTEM_VERSION is None /* Option */ {
        _SYSTEM_VERSION = "";
        // try {
        f = open ( "/System/Library/CoreServices/SystemVersion.plist" , encoding = "utf-8" );
        // } catch  OSError  {
        // pass
        } else {
        // try {
        m = re . search ( r "<key>ProductUserVisibleVersion</key>\s*";
        r "<string>(.*?)</string>" , f . read ( ) );
        // } finally {
        f . close ( );
        if m is !None /* Option */ {
        _SYSTEM_VERSION = "." . join ( m . group ( 1 ) . split ( "." ) [ : 2 ] );
        return  _SYSTEM_VERSION;
        _SYSTEM_VERSION_TUPLE = None /* Option */;
        pub fn _get_system_version_tuple ( )  {
        "
    Return the macOS system version as a tuple

    The return value == safe to use to compare
    two version numbers.
    ";
        global _SYSTEM_VERSION_TUPLE;
        if _SYSTEM_VERSION_TUPLE is None /* Option */ {
        osx_version = _get_system_version ( );
        if osx_version {
        // try {
        _SYSTEM_VERSION_TUPLE = tuple ( int ( i ) for i in osx_version . split ( "." ) );
        // } catch  ValueError  {
        _SYSTEM_VERSION_TUPLE = ( );
        return  _SYSTEM_VERSION_TUPLE;
        pub fn _remove_original_values ( _config_vars )  {
        "Remove original unmodified values for testing";
        for k in list ( _config_vars ) .iter() {
        if k . startswith ( _INITPRE ) {
        del _config_vars [ k ];
        pub fn _save_modified_value ( _config_vars , cv , newvalue )  {
        "Save modified && original unmodified value of configuration var";
        oldvalue = _config_vars . get ( cv , "" );
        if ( oldvalue != newvalue ) && ( _INITPRE + cv !in _config_vars ) {
        _config_vars [ _INITPRE + cv ] = oldvalue;
        _config_vars [ cv ] = newvalue;
        _cache_default_sysroot = None /* Option */;
        pub fn _default_sysroot ( cc )  {
        " Returns the root of the default SDK for this system, || '/' ";
        global _cache_default_sysroot;
        if _cache_default_sysroot is !None /* Option */ {
        return  _cache_default_sysroot;
        contents = _read_output ( "%s -c -E -v - </dev/null" % ( cc , ) , true );
        in_incdirs = false;
        for line in contents . splitlines ( ) .iter() {
        if line . startswith ( "#include <...>" ) {
        in_incdirs = true;
        } else if line . startswith ( "End of search list" ) {
        in_incdirs = false;
        } else if in_incdirs {
        line = line . strip ( );
        if line == "/usr/include" {
        _cache_default_sysroot = "/";
        } else if line . endswith ( ".sdk/usr/include" ) {
        _cache_default_sysroot = line [ : -12 ];
        if _cache_default_sysroot is None /* Option */ {
        _cache_default_sysroot = "/";
        return  _cache_default_sysroot;
        pub fn _supports_universal_builds ( )  {
        "Returns true if universal builds are supported on this system";
        osx_version = _get_system_version_tuple ( );
        return  bool ( osx_version >= ( 10 , 4 ) ) if osx_version else false;
        pub fn _supports_arm64_builds ( )  {
        "Returns true if arm64 builds are supported on this system";
        osx_version = _get_system_version_tuple ( );
        return  osx_version >= ( 11 , 0 ) if osx_version else false;
        pub fn _find_appropriate_compiler ( _config_vars )  {
        "Find appropriate C compiler for extension module builds";
        if "CC" in os . environ {
        return  _config_vars;
        cc = oldcc = _config_vars [ "CC" ] . split ( ) [ 0 ];
        if !_find_executable ( cc ) {
        cc = _find_build_tool ( "clang" );
        } else if os . path . basename ( cc ) . startswith ( "gcc" ) {
        data = _read_output ( "'%s' --version";
        % ( cc . replace ( "'" , "'\"'\"'" ) , ) );
        if data && "llvm-gcc" in data {
        cc = _find_build_tool ( "clang" );
        if !cc {
        panic!("SystemError (");
        "Cannot locate working compiler" );
        if cc != oldcc {
        for cv in _COMPILER_CONFIG_VARS .iter() {
        if cv in _config_vars && cv !in os . environ {
        cv_split = _config_vars [ cv ] . split ( );
        cv_split [ 0 ] = cc if cv != "CXX" else cc + "++";
        _save_modified_value ( _config_vars , cv , " " . join ( cv_split ) );
        return  _config_vars;
        pub fn _remove_universal_flags ( _config_vars )  {
        "Remove all universal build arguments from config vars";
        for cv in _UNIVERSAL_CONFIG_VARS .iter() {
        if cv in _config_vars && cv !in os . environ {
        flags = _config_vars [ cv ];
        flags = re . sub ( r "-arch\s+\w+\s" , " " , flags , flags = re . ASCII );
        flags = re . sub ( r "-isysroot\s*\S+" , " " , flags );
        _save_modified_value ( _config_vars , cv , flags );
        return  _config_vars;
        pub fn _remove_unsupported_archs ( _config_vars )  {
        "Remove any unsupported archs from config vars";
        if "CC" in os . environ {
        return  _config_vars;
        if re . search ( r "-arch\s+ppc" , _config_vars [ "CFLAGS" ] ) is !None /* Option */ {
        status = os . system (;
        "echo 'int main{};' | ";
        "'%s' -c -arch ppc -x c -o /dev/null /dev/null 2>/dev/null";
        % ( _config_vars [ "CC" ] . replace ( "'" , "'\"'\"'" ) , ) );
        if status {
        for cv in _UNIVERSAL_CONFIG_VARS .iter() {
        if cv in _config_vars && cv !in os . environ {
        flags = _config_vars [ cv ];
        flags = re . sub ( r "-arch\s+ppc\w*\s" , " " , flags );
        _save_modified_value ( _config_vars , cv , flags );
        return  _config_vars;
        pub fn _override_all_archs ( _config_vars )  {
        "Allow override of all archs with ARCHFLAGS env var";
        if "ARCHFLAGS" in os . environ {
        arch = os . environ [ "ARCHFLAGS" ];
        for cv in _UNIVERSAL_CONFIG_VARS .iter() {
        if cv in _config_vars && "-arch" in _config_vars [ cv ] {
        flags = _config_vars [ cv ];
        flags = re . sub ( r "-arch\s+\w+\s" , " " , flags );
        flags = flags + " " + arch;
        _save_modified_value ( _config_vars , cv , flags );
        return  _config_vars;
        pub fn _check_for_unavailable_sdk ( _config_vars )  {
        "Remove references to any SDKs !available";
        cflags = _config_vars . get ( "CFLAGS" , "" );
        m = re . search ( r "-isysroot\s*(\S+)" , cflags );
        if m is !None /* Option */ {
        sdk = m . group ( 1 );
        if !os . path . exists ( sdk ) {
        for cv in _UNIVERSAL_CONFIG_VARS .iter() {
        if cv in _config_vars && cv !in os . environ {
        flags = _config_vars [ cv ];
        flags = re . sub ( r "-isysroot\s*\S+(?:\s|$)" , " " , flags );
        _save_modified_value ( _config_vars , cv , flags );
        return  _config_vars;
        pub fn compiler_fixup ( compiler_so , cc_args )  {
        "
    This function will strip '-isysroot PATH' && '-arch ARCH' from the
    compile flags if the user has specified one them in extra_compile_flags.

    This == needed because '-arch ARCH' adds another architecture to the
    build, without a way to remove an architecture. Furthermore GCC will
    barf if multiple '-isysroot' arguments are present.
    ";
        stripArch = stripSysroot = false;
        compiler_so = list ( compiler_so );
        if !_supports_universal_builds ( ) {
        stripArch = stripSysroot = true;
        } else {
        stripArch = "-arch" in cc_args;
        stripSysroot = any ( arg for arg in cc_args if arg . startswith ( "-isysroot" ) );
        if stripArch || "ARCHFLAGS" in os . environ {
        while true  {
        // try {
        index = compiler_so . index ( "-arch" );
        del compiler_so [ index : index + 2 ];
        // } catch  ValueError  {
        break;
        } else if !_supports_arm64_builds ( ) {
        for idx in reversed ( range ( len ( compiler_so ) ) ) .iter() {
        if compiler_so [ idx ] == "-arch" && compiler_so [ idx + 1 ] == "arm64" {
        del compiler_so [ idx : idx + 2 ];
        if "ARCHFLAGS" in os . environ && !stripArch {
        compiler_so = compiler_so + os . environ [ "ARCHFLAGS" ] . split ( );
        if stripSysroot {
        while true  {
        indices = vec![ i.iter().map(|i , x| enumerate ( compiler_so ) if x . startswith ( "-isysroot" ) ).collect();
        if !indices {
        break;
        index = indices [ 0 ];
        if compiler_so [ index ] == "-isysroot" {
        del compiler_so [ index : index + 2 ];
        } else {
        del compiler_so [ index : index + 1 ];
        sysroot = None /* Option */;
        argvar = cc_args;
        indices = vec![ i.iter().map(|i , x| enumerate ( cc_args ) if x . startswith ( "-isysroot" ) ).collect();
        if !indices {
        argvar = compiler_so;
        indices = vec![ i.iter().map(|i , x| enumerate ( compiler_so ) if x . startswith ( "-isysroot" ) ).collect();
        for idx in indices .iter() {
        if argvar [ idx ] == "-isysroot" {
        sysroot = argvar [ idx + 1 ];
        break;
        } else {
        sysroot = argvar [ idx ] [ len ( "-isysroot" ) : ];
        break;
        if sysroot && !os . path . isdir ( sysroot ) {
        sys . stderr . write ( format!("Compiling with an SDK that doesn't seem to exist: {sysroot}\n" ));
        sys . stderr . write ( "Please check your Xcode installation\n" );
        sys . stderr . flush ( );
        return  compiler_so;
        pub fn customize_config_vars ( _config_vars )  {
        "Customize Python build configuration variables.

    Called internally from sysconfig with a mutable mapping
    containing name/value pairs parsed from the configured
    makefile used to build this interpreter.  Returns
    the mapping updated as needed to reflect the environment
    in which the interpreter == running; in the case of
    a Python from a binary installer, the installed
    environment may be very different from the build
    environment, i.e. different OS levels, different
    built tools, different available CPU architectures.

    This customization == performed whenever
    distutils.sysconfig.get_config_vars() == first
    called.  It may be used in environments where no
    compilers are present, i.e. when installing pure
    Python dists.  Customization of compiler paths
    && detection of unavailable archs == deferred
    until the first extension module build is
    requested (in distutils.sysconfig.customize_compiler).

    Currently called from distutils.sysconfig
    ";
        if !_supports_universal_builds ( ) {
        _remove_universal_flags ( _config_vars );
        _override_all_archs ( _config_vars );
        _check_for_unavailable_sdk ( _config_vars );
        return  _config_vars;
        pub fn customize_compiler ( _config_vars )  {
        "Customize compiler path && configuration variables.

    This customization == performed when the first
    extension module build == requested
    in distutils.sysconfig.customize_compiler.
    ";
        _find_appropriate_compiler ( _config_vars );
        _remove_unsupported_archs ( _config_vars );
        _override_all_archs ( _config_vars );
        return  _config_vars;
        pub fn get_platform_osx ( _config_vars , osname , release , machine )  {
        "Filter values for get_platform()";
        macver = _config_vars . get ( "MACOSX_DEPLOYMENT_TARGET" , "" );
        if macver && "." !in macver {
        macver + = ".0";
        macrelease = _get_system_version ( ) || macver;
        macver = macver || macrelease;
        if macver {
        release = macver;
        osname = "macosx";
        cflags = _config_vars . get ( _INITPRE + "CFLAGS" ,;
        _config_vars . get ( "CFLAGS" , "" ) );
        if macrelease {
        // try {
        macrelease = tuple ( int ( i ).iter().map(|i| macrelease . split ( "." ) vec![ 0 : 2 ] );
        // } catch  ValueError  {
        macrelease = ( 10 , 3 );
        } else {
        macrelease = ( 10 , 3 );
        if ( macrelease >= ( 10 , 4 ) ) && "-arch" in cflags . strip ( ) {
        machine = "fat";
        archs = re . findall ( r "-arch\s+(\S+)" , cflags );
        archs = tuple ( sorted ( set ( archs ) ) );
        if len ( archs ) == 1 {
        machine = archs [ 0 ];
        } else if archs == ( "arm64" , "x86_64" ) {
        machine = "universal2";
        } else if archs == ( "i386" , "ppc" ) {
        machine = "fat";
        } else if archs == ( "i386" , "x86_64" ) {
        machine = "intel";
        } else if archs == ( "i386" , "ppc" , "x86_64" ) {
        machine = "fat3";
        } else if archs == ( "ppc64" , "x86_64" ) {
        machine = "fat64";
        } else if archs == ( "i386" , "ppc" , "ppc64" , "x86_64" ) {
        machine = "universal";
        } else {
        panic!("ValueError (");
        "Don't know machine value for archs=%r" % ( archs , ) );
        } else if machine == "i386" {
        if sys . maxsize >= 2 ** 32 {
        machine = "x86_64";
        } else if machine in ( "PowerPC" , "Power_Macintosh" ) {
        if sys . maxsize >= 2 ** 32 {
        machine = "ppc64";
        } else {
        machine = "ppc";
        return  ( osname , release , machine );
}

