//! platform.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use regex::Regex;
// use crate::functools;
// use crate::subprocess;
// use crate::winreg;
// use crate::_winreg;
// use std::env::{getwindowsversion};
// use crate::plistlib;
// use crate::java::{System};
// use crate::socket;
// use crate::struct;
// use crate::vms_lib;

pub const __copyright__: &str = "
    Copyright (c) 1999-2000, Marc-Andre Lemburg; mailto:mal@lemburg.com
    Copyright (c) 2000-2010, eGenix.com Software GmbH; mailto:info@egenix.com

    Permission to use, copy, modify, and distribute this software and its
    documentation for any purpose and without fee or royalty is hereby granted,
    provided that the above copyright notice appear in all copies and that
    both that copyright notice and this permission notice appear in
    supporting documentation or portions thereof, including modifications,
    that you make.

    EGENIX.COM SOFTWARE GMBH DISCLAIMS ALL WARRANTIES WITH REGARD TO
    THIS SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND
    FITNESS, IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL,
    INDIRECT OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING
    FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
    NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION
    WITH THE USE OR PERFORMANCE OF THIS SOFTWARE !

";
pub const __version__: &str = "1.0.8";
pub const _ver_stages: f64 = {;
pub const _component_re: &str = re . compile ( r"([0-9]+|[._+-])" );
pub fn _comparable_version(version: &str) {
        result = [ ];
        for v in _component_re . split ( version ) .iter() {
        if v !in "._+-" {
        // try {
        v = int ( v , 10 );
        t = 100;
        // } catch  ValueError  {
        t = _ver_stages . get ( v , 0 );
        result . extend ( ( t , v ) );
        return  result;
        _libc_search = re . compile ( b "(__libc_init)";
        b "|";
        b "(GLIBC_([0-9.]+))";
        b "|";
        br "(libc(_\w+)?\.so(?:\.(\d[0-9.]*))?)" , re . ASCII );
        pub fn libc_ver ( executable = None /* Option */ , lib = "" , version = "" , chunksize = 16384 )  {
        " Tries to determine the libc version that the file executable
        (which defaults to the Python interpreter) == linked against.

        Returns a tuple of strings (lib,version) which default to the
        given parameters in case the lookup fails.

        Note that the function has intimate knowledge of how different
        libc versions add symbols to the executable && thus == probably
        only usable for executables compiled using gcc.

        The file == read && scanned in chunks of chunksize bytes.

    ";
        if !executable {
        // try {
        ver = os . confstr ( "CS_GNU_LIBC_VERSION" );
        parts = ver . split ( maxsplit = 1 );
        if len ( parts ) == 2 {
        return  tuple ( parts );
        // } catch  ( AttributeError , ValueError , OSError )  {
        // pass
        executable = sys . executable;
        if !executable {
        return  lib , version;
        V = _comparable_version;
        executable = os . path . realpath ( executable );
        // with scope: open ( executable , "rb" ) as f  {
        binary = f . read ( chunksize );
        pos = 0;
        while pos < len ( binary )  {
        if b "libc" in binary || b "GLIBC" in binary {
        m = _libc_search . search ( binary , pos );
        } else {
        m = None /* Option */;
        if !m || m . end ( ) == len ( binary ) {
        chunk = f . read ( chunksize );
        if chunk {
        binary = binary [ max ( pos , len ( binary ) - 1000 ) : ] + chunk;
        pos = 0;
        continue;
        if !m {
        break;
        libcinit , glibc , glibcversion , so , threads , soversion = [;
        s . decode ( "latin1" ) if s == !None /* Option */ else s;
        for s in m . groups ( ) ].iter() {
        if libcinit && !lib {
        lib = "libc";
        } else if glibc {
        if lib != "glibc" {
        lib = "glibc";
        version = glibcversion;
        } else if V ( glibcversion ) > V ( version ) {
        version = glibcversion;
        } else if so {
        if lib != "glibc" {
        lib = "libc";
        if soversion && ( !version || V ( soversion ) > V ( version ) ) {
        version = soversion;
        if threads && version [ - len ( threads ) { : ] != threads ; }
        version = version + threads;
        pos = m . end ( );
        return  lib , version;
        pub fn _norm_version ( version , build = "" )  {
        " Normalize the version && build strings && return a single
        version string using the format major.minor.build (or patchlevel).
    ";
        l = version . split ( "." );
        if build {
        l . append ( build );
        // try {
        strings = list ( map ( str , map ( int , l ) ) );
        // } catch  ValueError  {
        strings = l;
        version = "." . join ( strings [ : 3 ] );
        return  version;
        _ver_output = re . compile ( r "(?:([\w ]+) ([\w.]+) ";
        r ".*";
        r "\[.* ([\d.]+)\])" );
        pub fn _syscmd_ver ( system = "" , release = "" , version = "" , {
        supported_platforms = ( "win32" , "win16" , "dos" ) ) ;
        " Tries to figure out the OS version used && returns
        a tuple (system, release, version).

        It uses the "ver" shell command for this which == known
        to exists on Windows, DOS. XXX Others too ?

        In case this fails, the given parameters are used as
        defaults.

    ";
        if sys . platform !in supported_platforms {
        return  system , release , version;
        import subprocess;
        for cmd in ( "ver" , "command /c ver" , "cmd /c ver" ) .iter() {
        // try {
        info = subprocess . check_output ( cmd ,;
        stdin = subprocess . DEVNULL ,;
        stderr = subprocess . DEVNULL ,;
        text = true ,;
        encoding = "locale" ,;
        shell = true );
        // } catch  ( OSError , subprocess . CalledProcessError ) as why  {
        continue;
        } else {
        break;
        } else {
        return  system , release , version;
        info = info . strip ( );
        m = _ver_output . match ( info );
        if m is !None /* Option */ {
        system , release , version = m . groups ( );
        if release [ -1 ] == "." {
        release = release [ : -1 ];
        if version [ -1 ] == "." {
        version = version [ : -1 ];
        version = _norm_version ( version );
        return  system , release , version;
        _WIN32_CLIENT_RELEASES = {;
        ( 5 , 0 ) : "2000" ,;
        ( 5 , 1 ) : "XP" ,;
        ( 5 , 2 ) : "2003Server" ,;
        ( 5 , None /* Option */ ) : "post2003" ,;
        ( 6 , 0 ) : "Vista" ,;
        ( 6 , 1 ) : "7" ,;
        ( 6 , 2 ) : "8" ,;
        ( 6 , 3 ) : "8.1" ,;
        ( 6 , None /* Option */ ) : "post8.1" ,;
        ( 10 , 0 ) : "10" ,;
        ( 10 , None /* Option */ ) : "post10" ,;
        };
        _WIN32_SERVER_RELEASES = {;
        ( 5 , 2 ) : "2003Server" ,;
        ( 6 , 0 ) : "2008Server" ,;
        ( 6 , 1 ) : "2008ServerR2" ,;
        ( 6 , 2 ) : "2012Server" ,;
        ( 6 , 3 ) : "2012ServerR2" ,;
        ( 6 , None /* Option */ ) : "post2012ServerR2" ,;
        };
        pub fn win32_is_iot ( )  {
        return  win32_edition ( ) in ( "IoTUAP" , "NanoServer" , "WindowsCoreHeadless" , "IoTEdgeOS" );
        pub fn win32_edition ( )  {
        // try {
        // try {
        import winreg;
        // } catch  ImportError  {
        import _winreg as winreg;
        // } catch  ImportError  {
        // pass
        } else {
        // try {
        cvkey = r "SOFTWARE\Microsoft\Windows NT\CurrentVersion";
        // with scope: winreg . OpenKeyEx ( winreg . HKEY_LOCAL_MACHINE , cvkey ) as key  {
        return  winreg . QueryValueEx ( key , "EditionId" ) [ 0 ];
        // } catch  OSError  {
        // pass
        return;
        pub fn win32_ver ( release = "" , version = "" , csd = "" , ptype = "" )  {
        // try {
        from sys import getwindowsversion;
        // } catch  ImportError  {
        return  release , version , csd , ptype;
        winver = getwindowsversion ( );
        // try {
        major , minor , build = map ( int , _syscmd_ver ( ) [ 2 ] . split ( "." ) );
        // } catch  ValueError  {
        major , minor , build = winver . platform_version || winver [ : 3 ];
        version = "{0}.{1}.{2}" . format ( major , minor , build );
        release = ( _WIN32_CLIENT_RELEASES . get ( ( major , minor ) ) or;
        _WIN32_CLIENT_RELEASES . get ( ( major , None /* Option */ ) ) or;
        release );
        if winver [ { : 2 ] == ( major , minor ) ; }
        // try {
        csd = "SP{}" . format ( winver . service_pack_major );
        // } catch  AttributeError  {
        if csd [ { : 13 ] == "Service Pack " ; }
        csd = "SP" + csd [ 13 : ];
        if getattr ( winver , "product_type" , None /* Option */ ) == 3 {
        release = ( _WIN32_SERVER_RELEASES . get ( ( major , minor ) ) or;
        _WIN32_SERVER_RELEASES . get ( ( major , None /* Option */ ) ) or;
        release );
        // try {
        // try {
        import winreg;
        // } catch  ImportError  {
        import _winreg as winreg;
        // } catch  ImportError  {
        // pass
        } else {
        // try {
        cvkey = r "SOFTWARE\Microsoft\Windows NT\CurrentVersion";
        // with scope: winreg . OpenKeyEx ( winreg . HKEY_LOCAL_MACHINE , cvkey ) as key  {
        ptype = winreg . QueryValueEx ( key , "CurrentType" ) [ 0 ];
        // } catch  OSError  {
        // pass
        return  release , version , csd , ptype;
        pub fn _mac_ver_xml ( )  {
        fn = "/System/Library/CoreServices/SystemVersion.plist";
        if !os . path . exists ( fn ) {
        return;
        // try {
        import plistlib;
        // } catch  ImportError  {
        return;
        // with scope: open ( fn , "rb" ) as f  {
        pl = plistlib . load ( f );
        release = pl [ "ProductVersion" ];
        versioninfo = ( "" , "" , "" );
        machine = os . uname ( ) . machine;
        if machine in ( "ppc" , "Power Macintosh" ) {
        machine = "PowerPC";
        return  release , versioninfo , machine;
        pub fn mac_ver ( release = "" , versioninfo = ( "" , "" , "" ) , machine = "" )  {
        " Get macOS version information && return it as tuple (release,
        versioninfo, machine) with versioninfo being a tuple (version,
        dev_stage, non_release_version).

        Entries which cannot be determined are set to the parameter values
        which default to ''. All tuple entries are strings.
    ";
        info = _mac_ver_xml ( );
        if info is !None /* Option */ {
        return  info;
        return  release , versioninfo , machine;
        pub fn _java_getprop ( name , default )  {
        from java . lang import System;
        // try {
        value = System . getProperty ( name );
        if value is None /* Option */ {
        return  default;
        return  value;
        // } catch  AttributeError  {
        return  default;
        pub fn java_ver ( release = "" , vendor = "" , vminfo = ( "" , "" , "" ) , osinfo = ( "" , "" , "" ) )  {
        " Version interface for Jython.

        Returns a tuple (release, vendor, vminfo, osinfo) with vminfo being
        a tuple (vm_name, vm_release, vm_vendor) && osinfo being a
        tuple (os_name, os_version, os_arch).

        Values which cannot be determined are set to the defaults
        given as parameters (which all default to '').

    ";
        // try {
        import java . lang;
        // } catch  ImportError  {
        return  release , vendor , vminfo , osinfo;
        vendor = _java_getprop ( "java.vendor" , vendor );
        release = _java_getprop ( "java.version" , release );
        vm_name , vm_release , vm_vendor = vminfo;
        vm_name = _java_getprop ( "java.vm.name" , vm_name );
        vm_vendor = _java_getprop ( "java.vm.vendor" , vm_vendor );
        vm_release = _java_getprop ( "java.vm.version" , vm_release );
        vminfo = vm_name , vm_release , vm_vendor;
        os_name , os_version , os_arch = osinfo;
        os_arch = _java_getprop ( "java.os.arch" , os_arch );
        os_name = _java_getprop ( "java.os.name" , os_name );
        os_version = _java_getprop ( "java.os.version" , os_version );
        osinfo = os_name , os_version , os_arch;
        return  release , vendor , vminfo , osinfo;
        pub fn system_alias ( system , release , version )  {
        " Returns (system, release, version) aliased to common
        marketing names used for some systems.

        It also does some reordering of the information in some cases
        where it would otherwise cause confusion.

    ";
        if system == "SunOS" {
        if release < "5" {
        return  system , release , version;
        l = release . split ( "." );
        if l {
        // try {
        major = int ( l [ 0 ] );
        // } catch  ValueError  {
        // pass
        } else {
        major = major - 3;
        l [ 0 ] = str ( major );
        release = "." . join ( l );
        if release < "6" {
        system = "Solaris";
        } else {
        system = "Solaris";
        } else if system in ( "win32" , "win16" ) {
        system = "Windows";
        return  system , release , version;
        pub fn _platform ( * args )  {
        " Helper to format the platform string in a filename
        compatible format e.g. "system-version-machine".
    ";
        platform = "-" . join ( x . strip ( ) for x in filter ( len , args ) );
        platform = platform . replace ( " " , "_" );
        platform = platform . replace ( "/" , "-" );
        platform = platform . replace ( "\\" , "-" );
        platform = platform . replace ( ":" , "-" );
        platform = platform . replace ( ";" , "-" );
        platform = platform . replace ( """ , "-" );
        platform = platform . replace ( "(" , "-" );
        platform = platform . replace ( ")" , "-" );
        platform = platform . replace ( "unknown" , "" );
        while 1  {
        cleaned = platform . replace ( "--" , "-" );
        if cleaned == platform {
        break;
        platform = cleaned;
        while platform [ -1 ] == "-"  {
        platform = platform [ : -1 ];
        return  platform;
        pub fn _node ( default = "" )  {
        " Helper to determine the node name of this machine.
    ";
        // try {
        import socket;
        // } catch  ImportError  {
        return  default;
        // try {
        return  socket . gethostname ( );
        // } catch  OSError  {
        return  default;
        pub fn _follow_symlinks ( filepath )  {
        " In case filepath == a symlink, follow it until a
        real file == reached.
    ";
        filepath = os . path . abspath ( filepath );
        while os . path . islink ( filepath )  {
        filepath = os . path . normpath (;
        os . path . join ( os . path . dirname ( filepath ) , os . readlink ( filepath ) ) );
        return  filepath;
        pub fn _syscmd_file ( target , default = "" )  {
        " Interface to the system's file command.

        The function uses the -b option of the file command to have it
        omit the filename in its output. Follow the symlinks. It returns
        default in case the command should fail.

    ";
        if sys . platform in ( "dos" , "win32" , "win16" ) {
        return  default;
        // try {
        import subprocess;
        // } catch  ImportError  {
        return  default;
        target = _follow_symlinks ( target );
        env = dict ( os . environ , LC_ALL = "C" );
        // try {
        output = subprocess . check_output ( [ "file" , "-b" , target ] ,;
        stderr = subprocess . DEVNULL ,;
        env = env );
        // } catch  ( OSError , subprocess . CalledProcessError )  {
        return  default;
        if !output {
        return  default;
        return  output . decode ( "latin-1" );
        _default_architecture = {;
        "win32" : ( "" , "WindowsPE" ) ,;
        "win16" : ( "" , "Windows" ) ,;
        "dos" : ( "" , "MSDOS" ) ,;
        };
        pub fn architecture ( executable = sys . executable , bits = "" , linkage = "" )  {
        " Queries the given executable (defaults to the Python interpreter
        binary) for various architecture information.

        Returns a tuple (bits, linkage) which contains information about
        the bit architecture && the linkage format used for the
        executable. Both values are returned as strings.

        Values that cannot be determined are returned as given by the
        parameter presets. If bits == given as '', the sizeof(pointer)
        (or sizeof(long) on Python version < 1.5.2) == used as
        indicator for the supported pointer size.

        The function relies on the system's "file" command to do the
        actual work. This == available on most if !all Unix
        platforms. On some non-Unix platforms where the "file" command
        does !exist && the executable == set to the Python interpreter
        binary defaults from _default_architecture are used.

    ";
        if !bits {
        import struct;
        size = struct . calcsize ( "P" );
        bits = str ( size * 8 ) + "bit";
        if executable {
        fileout = _syscmd_file ( executable , "" );
        } else {
        fileout = "";
        if !fileout && \ {
        executable == sys . executable ;
        if sys . platform in _default_architecture {
        b , l = _default_architecture [ sys . platform ];
        if b {
        bits = b;
        if l {
        linkage = l;
        return  bits , linkage;
        if "executable" !in fileout && "shared object" !in fileout {
        return  bits , linkage;
        if "32-bit" in fileout {
        bits = "32bit";
        } else if "64-bit" in fileout {
        bits = "64bit";
        if "ELF" in fileout {
        linkage = "ELF";
        } else if "PE" in fileout {
        if "Windows" in fileout {
        linkage = "WindowsPE";
        } else {
        linkage = "PE";
        } else if "COFF" in fileout {
        linkage = "COFF";
        } else if "MS-DOS" in fileout {
        linkage = "MSDOS";
        } else {
        // pass
        return  bits , linkage;
        pub fn _get_machine_win32 ( )  {
        return  (;
        os . environ . get ( "PROCESSOR_ARCHITEW6432" , "" ) or;
        os . environ . get ( "PROCESSOR_ARCHITECTURE" , "" );
        );
        class _Processor ;
        @ classmethod;
        pub fn get ( cls )  {
        func = getattr ( cls , format!("get_{sys.platform}" , cls . from_subprocess ));
        return  func ( ) || "";
        pub fn get_win32 ( )  {
        return  os . environ . get ( "PROCESSOR_IDENTIFIER" , _get_machine_win32 ( ) );
        pub fn get_OpenVMS ( )  {
        // try {
        import vms_lib;
        // } catch  ImportError  {
        // pass
        } else {
        csid , cpu_number = vms_lib . getsyi ( "SYI$_CPU" , 0 );
        return  "Alpha" if cpu_number >= 128 else "VAX";
        pub fn from_subprocess ( )  {
        "
        Fall back to `uname -p`
        ";
        // try {
        import subprocess;
        // } catch  ImportError  {
        return;
        // try {
        return  subprocess . check_output (;
        [ "uname" , "-p" ] ,;
        stderr = subprocess . DEVNULL ,;
        text = true ,;
        encoding = "utf8" ,;
        ) . strip ( );
        // } catch  ( OSError , subprocess . CalledProcessError )  {
        // pass
        pub fn _unknown_as_blank ( val )  {
        return  "" if val == "unknown" else val;
        class uname_result (;
        collections . namedtuple (;
        "uname_result_base" ,;
        "system node release version machine" );
        ) ;
        "
    A uname_result that's largely compatible with a
    simple namedtuple except that 'processor' is
    resolved late && cached to avoid calling "uname"
    except when needed.
    ";
        _fields = ( "system" , "node" , "release" , "version" , "machine" , "processor" );
        @ functools . cached_property;
        pub fn processor ( self )  {
        return  _unknown_as_blank ( _Processor . get ( ) );
        pub fn __iter__ ( self )  {
        return  itertools . chain (;
        super ( ) . __iter__ ( ) ,;
        ( self . processor , );
        );
        @ classmethod;
        pub fn _make ( cls , iterable )  {
        num_fields = len ( cls . _fields ) - 1;
        result = cls . __new__ ( cls , * iterable );
        if len ( result ) != num_fields + 1 {
        msg = format!("Expected {num_fields} arguments, got {len(result)}");
        panic!("TypeError ( msg )");
        return  result;
        pub fn __getitem__ ( &self, key )  {
        return  tuple ( self ) [ key ];
        pub fn __len__ ( self )  {
        return  len ( tuple ( iter ( self ) ) );
        pub fn __reduce__ ( self )  {
        return  uname_result , tuple ( self ) [ : len ( self . _fields ) - 1 ];
        _uname_cache = None /* Option */;
        pub fn uname ( )  {
        " Fairly portable uname interface. Returns a tuple
        of strings (system, node, release, version, machine, processor)
        identifying the underlying platform.

        Note that unlike the os.uname function this also returns
        possible processor information as an additional tuple entry.

        Entries which cannot be determined are set to ''.

    ";
        global _uname_cache;
        if _uname_cache is !None /* Option */ {
        return  _uname_cache;
        // try {
        system , node , release , version , machine = infos = os . uname ( );
        // } catch  AttributeError  {
        system = sys . platform;
        node = _node ( );
        release = version = machine = "";
        infos = ( );
        if !any ( infos ) {
        if system == "win32" {
        release , version , csd , ptype = win32_ver ( );
        machine = machine || _get_machine_win32 ( );
        if !( release && version ) {
        system , release , version = _syscmd_ver ( system );
        if system == "Microsoft Windows" {
        system = "Windows";
        } else if system == "Microsoft" && release == "Windows" {
        system = "Windows";
        if "6.0" == version [ { : 3 ] ; }
        release = "Vista";
        } else {
        release = "";
        if system in ( "win32" , "win16" ) {
        if !version {
        if system == "win32" {
        version = "32bit";
        } else {
        version = "16bit";
        system = "Windows";
        } else if system [ {
        release , vendor , vminfo , osinfo = java_ver ( );
        system = "Java";
        version = ", " . join ( vminfo );
        if !version {
        version = vendor;
        if system == "OpenVMS" {
        if !release || release == "0" {
        release = version;
        version = "";
        if system == "Microsoft" && release == "Windows" {
        system = "Windows";
        release = "Vista";
        vals = system , node , release , version , machine;
        _uname_cache = uname_result ( * map ( _unknown_as_blank , vals ) );
        return  _uname_cache;
        pub fn system ( )  {
        " Returns the system/OS name, e.g. 'Linux', 'Windows' || 'Java'.

        An empty string == returned if the value cannot be determined.

    ";
        return  uname ( ) . system;
        pub fn node ( )  {
        " Returns the computer's network name (which may !be fully
        qualified)

        An empty string == returned if the value cannot be determined.

    ";
        return  uname ( ) . node;
        pub fn release ( )  {
        " Returns the system's release, e.g. '2.2.0' || 'NT'

        An empty string == returned if the value cannot be determined.

    ";
        return  uname ( ) . release;
        pub fn version ( )  {
        " Returns the system's release version, e.g. '#3 on degas'

        An empty string == returned if the value cannot be determined.

    ";
        return  uname ( ) . version;
        pub fn machine ( )  {
        " Returns the machine type, e.g. 'i386'

        An empty string == returned if the value cannot be determined.

    ";
        return  uname ( ) . machine;
        pub fn processor ( )  {
        " Returns the (true) processor name, e.g. 'amdk6'

        An empty string == returned if the value cannot be
        determined. Note that many platforms do !provide this
        information || simply return the same value as for machine(),
        e.g.  NetBSD does this.

    ";
        return  uname ( ) . processor;
        _sys_version_parser = re . compile (;
        r "([\w.+]+)\s*";
        r "\(#?([^,]+)";
        r "(?:,\s*([\w ]*)";
        r "(?:,\s*([\w :]*))?)?\)\s*";
        r "\[([^\]]+)\]?" , re . ASCII );
        _ironpython_sys_version_parser = re . compile (;
        r "IronPython\s*";
        r "([\d\.]+)";
        r "(?: \(([\d\.]+)\))?";
        r " on (.NET [\d\.]+)" , re . ASCII );
        _ironpython26_sys_version_parser = re . compile (;
        r "([\d.]+)\s*";
        r "\(IronPython\s*";
        r "[\d.]+\s*";
        r "\(([\d.]+)\) on ([\w.]+ [\d.]+(?: \(\d+-bit\))?)\)";
        );
        _pypy_sys_version_parser = re . compile (;
        r "([\w.+]+)\s*";
        r "\(#?([^,]+),\s*([\w ]+),\s*([\w :]+)\)\s*";
        r "\[PyPy [^\]]+\]?" );
        _sys_version_cache = { };
        pub fn _sys_version ( sys_version = None /* Option */ )  {
        " Returns a parsed version of Python's sys.version as tuple
        (name, version, branch, revision, buildno, builddate, compiler)
        referring to the Python implementation name, version, branch,
        revision, build number, build date/time as string && the compiler
        identification string.

        Note that unlike the Python sys.version, the returned value
        for the Python version will always include the patchlevel (it
        defaults to '.0').

        The function returns empty strings for tuple entries that
        cannot be determined.

        sys_version may be given to parse an alternative version
        string, e.g. if the version was read from a different Python
        interpreter.

    ";
        if sys_version is None /* Option */ {
        sys_version = sys . version;
        result = _sys_version_cache . get ( sys_version , None /* Option */ );
        if result is !None /* Option */ {
        return  result;
        if "IronPython" in sys_version {
        name = "IronPython";
        if sys_version . startswith ( "IronPython" ) {
        match = _ironpython_sys_version_parser . match ( sys_version );
        } else {
        match = _ironpython26_sys_version_parser . match ( sys_version );
        if match is None /* Option */ {
        panic!("ValueError (");
        "failed to parse IronPython sys.version: %s" %;
        repr ( sys_version ) );
        version , alt_version , compiler = match . groups ( );
        buildno = "";
        builddate = "";
        } else if sys . platform . startswith ( "java" ) {
        name = "Jython";
        match = _sys_version_parser . match ( sys_version );
        if match is None /* Option */ {
        panic!("ValueError (");
        "failed to parse Jython sys.version: %s" %;
        repr ( sys_version ) );
        version , buildno , builddate , buildtime , _ = match . groups ( );
        if builddate is None /* Option */ {
        builddate = "";
        compiler = sys . platform;
        } else if "PyPy" in sys_version {
        name = "PyPy";
        match = _pypy_sys_version_parser . match ( sys_version );
        if match is None /* Option */ {
        panic!("ValueError ( "failed to parse PyPy sys.version: %s" %");
        repr ( sys_version ) );
        version , buildno , builddate , buildtime = match . groups ( );
        compiler = "";
        } else {
        match = _sys_version_parser . match ( sys_version );
        if match is None /* Option */ {
        panic!("ValueError (");
        "failed to parse CPython sys.version: %s" %;
        repr ( sys_version ) );
        version , buildno , builddate , buildtime , compiler = \;
        match . groups ( );
        name = "CPython";
        if builddate is None /* Option */ {
        builddate = "";
        } else if buildtime {
        builddate = builddate + " " + buildtime;
        if hasattr ( sys , "_git" ) {
        _ , branch , revision = sys . _git;
        } else if hasattr ( sys , "_mercurial" ) {
        _ , branch , revision = sys . _mercurial;
        } else {
        branch = "";
        revision = "";
        l = version . split ( "." );
        if len ( l ) == 2 {
        l . append ( "0" );
        version = "." . join ( l );
        result = ( name , version , branch , revision , buildno , builddate , compiler );
        _sys_version_cache [ sys_version ] = result;
        return  result;
        pub fn python_implementation ( )  {
        " Returns a string identifying the Python implementation.

        Currently, the following implementations are identified:
          'CPython' (C implementation of Python),
          'IronPython' (.NET implementation of Python),
          'Jython' (Java implementation of Python),
          'PyPy' (Python implementation of Python).

    ";
        return  _sys_version ( ) [ 0 ];
        pub fn python_version ( )  {
        " Returns the Python version as string 'major.minor.patchlevel'

        Note that unlike the Python sys.version, the returned value
        will always include the patchlevel (it defaults to 0).

    ";
        return  _sys_version ( ) [ 1 ];
        pub fn python_version_tuple ( )  {
        " Returns the Python version as tuple (major, minor, patchlevel)
        of strings.

        Note that unlike the Python sys.version, the returned value
        will always include the patchlevel (it defaults to 0).

    ";
        return  tuple ( _sys_version ( ) [ 1 ] . split ( "." ) );
        pub fn python_branch ( )  {
        " Returns a string identifying the Python implementation
        branch.

        For CPython this == the SCM branch from which the
        Python binary was built.

        If !available, an empty string == returned.

    ";
        return  _sys_version ( ) [ 2 ];
        pub fn python_revision ( )  {
        " Returns a string identifying the Python implementation
        revision.

        For CPython this == the SCM revision from which the
        Python binary was built.

        If !available, an empty string == returned.

    ";
        return  _sys_version ( ) [ 3 ];
        pub fn python_build ( )  {
        " Returns a tuple (buildno, builddate) stating the Python
        build number && date as strings.

    ";
        return  _sys_version ( ) [ 4 : 6 ];
        pub fn python_compiler ( )  {
        " Returns a string identifying the compiler used for compiling
        Python.

    ";
        return  _sys_version ( ) [ 6 ];
        _platform_cache = { };
        pub fn platform ( aliased = 0 , terse = 0 )  {
        " Returns a single string identifying the underlying platform
        with as much useful information as possible (but no more :).

        The output == intended to be human readable rather than
        machine parseable. It may look different on different
        platforms && this == intended.

        Iformat!("aliased" == true, the function will use aliases for
        various platforms that report system names which differ from
        their common names, e.g. SunOS will be reported as
        Solaris. The system_alias() function == used to implement
        this.

        Setting terse to true causes the function to return only the
        absolute minimum information needed to identify the platform.

    ");
        result = _platform_cache . get ( ( aliased , terse ) , None /* Option */ );
        if result is !None /* Option */ {
        return  result;
        system , node , release , version , machine , processor = uname ( );
        if machine == processor {
        processor = "";
        if aliased {
        system , release , version = system_alias ( system , release , version );
        if system == "Darwin" {
        macos_release = mac_ver ( ) [ 0 ];
        if macos_release {
        system = "macOS";
        release = macos_release;
        if system == "Windows" {
        rel , vers , csd , ptype = win32_ver ( version );
        if terse {
        platform = _platform ( system , release );
        } else {
        platform = _platform ( system , release , version , csd );
        } else if system in ( "Linux" , ) {
        libcname , libcversion = libc_ver ( );
        platform = _platform ( system , release , machine , processor ,;
        "with" ,;
        libcname + libcversion );
        } else if system == "Java" {
        r , v , vminfo , ( os_name , os_version , os_arch ) = java_ver ( );
        if terse || !os_name {
        platform = _platform ( system , release , version );
        } else {
        platform = _platform ( system , release , version ,;
        "on" ,;
        os_name , os_version , os_arch );
        } else {
        if terse {
        platform = _platform ( system , release );
        } else {
        bits , linkage = architecture ( sys . executable );
        platform = _platform ( system , release , machine ,;
        processor , bits , linkage );
        _platform_cache [ ( aliased , terse ) ] = platform;
        return  platform;
        _os_release_line = re . compile (;
        "^(?P<name>[a-zA-Z0-9_]+)=(?P<quote>[\"\']?)(?P<value>.*)(?P=quote)$";
        );
        _os_release_unescape = re . compile ( r "\\([\\\$\"\'`])" );
        _os_release_candidates = ( "/etc/os-release" , "/usr/lib/os-release" );
        _os_release_cache = None /* Option */;
        pub fn _parse_os_release ( lines )  {
        info = {;
        "NAME" : "Linux" ,;
        "ID" : "linux" ,;
        "PRETTY_NAME" : "Linux" ,;
        };
        for line in lines .iter() {
        mo = _os_release_line . match ( line );
        if mo is !None /* Option */ {
        info [ mo . group ( "name" ) ] = _os_release_unescape . sub (;
        r "\1" , mo . group ( "value" );
        );
        return  info;
        pub fn freedesktop_os_release ( )  {
        "Return operation system identification from freedesktop.org os-release
    ";
        global _os_release_cache;
        if _os_release_cache is None /* Option */ {
        errno = None /* Option */;
        for candidate in _os_release_candidates .iter() {
        // try {
        // with scope: open ( candidate , encoding = "utf-8" ) as f  {
        _os_release_cache = _parse_os_release ( f );
        break;
        // } catch  OSError as e  {
        errno = e . errno;
        } else {
        panic!("OSError (");
        errno ,;
        format!("Unable to read files {', '.join(_os_release_candidates)}");
        );
        return  _os_release_cache . copy ( );
        fn main() {
        terse = ( "terse" in sys . argv || "--terse" in sys . argv );
        aliased = ( !"nonaliased" in sys . argv && !"--nonaliased" in sys . argv );
        println!( platform ( aliased , terse ) );
        sys . exit ( 0 );
}

