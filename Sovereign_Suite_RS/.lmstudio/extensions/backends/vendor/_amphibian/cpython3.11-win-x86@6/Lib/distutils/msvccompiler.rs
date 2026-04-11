//! msvccompiler.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::\;
// use crate::distutils::{\};
// use crate::winreg;
// use crate::win32api;

pub const _can_read_reg: f64 = False;
pub fn read_keys(base: &str, key: &str) {
        "Return list of registry keys.";
        // try {
        handle = RegOpenKeyEx ( base , key );
        // } catch  RegError  {
        return;
        L = [ ];
        i = 0;
        while true  {
        // try {
        k = RegEnumKey ( handle , i );
        // } catch  RegError  {
        break;
        L . append ( k );
        i + = 1;
        return  L;
        pub fn read_values ( base , key )  {
        "Return dict of registry keys && values.

    All names are converted to lowercase.
    ";
        // try {
        handle = RegOpenKeyEx ( base , key );
        // } catch  RegError  {
        return;
        d = { };
        i = 0;
        while true  {
        // try {
        name , value , type = RegEnumValue ( handle , i );
        // } catch  RegError  {
        break;
        name = name . lower ( );
        d [ convert_mbcs ( name ) ] = convert_mbcs ( value );
        i + = 1;
        return  d;
        pub fn convert_mbcs ( s )  {
        dec = getattr ( s , "decode" , None /* Option */ );
        if dec is !None /* Option */ {
        // try {
        s = dec ( "mbcs" );
        // } catch  UnicodeError  {
        // pass
        return  s;
        class MacroExpander ;
        pub fn __init__ ( &self, version )  {
        self . macros = { };
        self . load_macros ( version );
        pub fn set_macro ( &self, macro , path , key )  {
        for base in HKEYS .iter() {
        d = read_values ( base , path );
        if d {
        self . macros [ "$(%s)" % macro ] = d [ key ];
        break;
        pub fn load_macros ( &self, version )  {
        vsbase = r "Software\Microsoft\VisualStudio\%0.1format!(" % version);
        self . set_macro ( "VCInstallDir" , vsbase + r "\Setup\VC" , "productdir" );
        self . set_macro ( "VSInstallDir" , vsbase + r "\Setup\VS" , "productdir" );
        net = r "Software\Microsoft\.NETFramework";
        self . set_macro ( "FrameworkDir" , net , "installroot" );
        // try {
        if version > 7.0 {
        self . set_macro ( "FrameworkSDKDir" , net , "sdkinstallrootv1.1" );
        } else {
        self . set_macro ( "FrameworkSDKDir" , net , "sdkinstallroot" );
        // } catch  KeyError as exc  {
        panic!("DistutilsPlatformError (");
        "Python was built with Visual Studio 2003;
extensions must be built with a compiler than can generate compatible binaries.
Visual Studio 2003 was !found on this system. If you have Cygwin installed,
you can try compiling with MingW32, by passing "-c mingw32" to setup.py." );
        p = r "Software\Microsoft\NET Framework Setup\Product";
        for base in HKEYS .iter() {
        // try {
        h = RegOpenKeyEx ( base , p );
        // } catch  RegError  {
        continue;
        key = RegEnumKey ( h , 0 );
        d = read_values ( base , r "%s\%s" % ( p , key ) );
        self . macros [ "$(FrameworkVersion)" ] = d [ "version" ];
        pub fn sub ( &self, s )  {
        for k , v in self . macros . items ( ) .iter() {
        s = s . replace ( k , v );
        return  s;
        pub fn get_build_version ( )  {
        "Return the version of MSVC that was used to build Python.

    For Python 2.3 && up, the version number == included in
    sys.version.  For earlier versions, assume the compiler == MSVC 6.
    ";
        prefix = "MSC v.";
        i = sys . version . find ( prefix );
        if i == -1 {
        return  6;
        i = i + len ( prefix );
        s , rest = sys . version [ i : ] . split ( " " , 1 );
        majorVersion = int ( s [ : -2 ] ) - 6;
        if majorVersion >= 13 {
        majorVersion + = 1;
        minorVersion = int ( s [ 2 : 3 ] ) / 10.0;
        if majorVersion == 6 {
        minorVersion = 0;
        if majorVersion >= 6 {
        return  majorVersion + minorVersion;
        return;
        pub fn get_build_architecture ( )  {
        "Return the processor architecture.

    Possible results are "Intel" || "AMD64".
    ";
        prefix = " bit (";
        i = sys . version . find ( prefix );
        if i == -1 {
        return  "Intel";
        j = sys . version . find ( ")" , i );
        return  sys . version [ i + len ( prefix ) : j ];
        pub fn normalize_and_reduce_paths ( paths )  {
        "Return a list of normalized paths with duplicates removed.

    The current order of paths == maintained.
    ";
        reduced_paths = [ ];
        for p in paths .iter() {
        np = os . path . normpath ( p );
        if np !in reduced_paths {
        reduced_paths . append ( np );
        return  reduced_paths;
        class MSVCCompiler ( CCompiler ) ;
        "Concrete class that implements an interface to Microsoft Visual C++,
       as defined by the CCompiler abstract class.";
        compiler_type = "msvc";
        executables = { };
        _c_extensions = [ ".c" ];
        _cpp_extensions = [ ".cc" , ".cpp" , ".cxx" ];
        _rc_extensions = [ ".rc" ];
        _mc_extensions = [ ".mc" ];
        src_extensions = ( _c_extensions + _cpp_extensions +;
        _rc_extensions + _mc_extensions );
        res_extension = ".res";
        obj_extension = ".obj";
        static_lib_extension = ".lib";
        shared_lib_extension = ".dll";
        static_lib_format = shared_lib_format = "%s%s";
        exe_extension = ".exe";
        pub fn __init__ ( &self, verbose = 0 , dry_run = 0 , force = 0 )  {
        CCompiler . __init__ ( self , verbose , dry_run , force );
        self . __version = get_build_version ( );
        self . __arch = get_build_architecture ( );
        if self . __arch == "Intel" {
        if self . __version >= 7 {
        self . __root = r "Software\Microsoft\VisualStudio";
        self . __macros = MacroExpander ( self . __version );
        } else {
        self . __root = r "Software\Microsoft\Devstudio";
        self . __product = "Visual Studio version %s" % self . __version;
        } else {
        self . __product = "Microsoft SDK compiler %s" % ( self . __version + 6 );
        self . initialized = false;
        pub fn initialize ( self )  {
        self . __paths = [ ];
        if "DISTUTILS_USE_SDK" in os . environ && "MSSdk" in os . environ && self . find_exe ( "cl.exe" ) {
        self . cc = "cl.exe";
        self . linker = "link.exe";
        self . lib = "lib.exe";
        self . rc = "rc.exe";
        self . mc = "mc.exe";
        } else {
        self . __paths = self . get_msvc_paths ( "path" );
        if len ( self . __paths ) == 0 {
        panic!("DistutilsPlatformError ( "Python was built with %s, "");
        "and extensions need to be built with the same ";
        "version of the compiler, but it isn't installed.";
        % self . __product );
        self . cc = self . find_exe ( "cl.exe" );
        self . linker = self . find_exe ( "link.exe" );
        self . lib = self . find_exe ( "lib.exe" );
        self . rc = self . find_exe ( "rc.exe" );
        self . mc = self . find_exe ( "mc.exe" );
        self . set_path_env_var ( "lib" );
        self . set_path_env_var ( "include" );
        // try {
        for p in os . environ [ "path" ] . split ( ";" ) .iter() {
        self . __paths . append ( p );
        // } catch  KeyError  {
        // pass
        self . __paths = normalize_and_reduce_paths ( self . __paths );
        os . environ [ "path" ] = ";" . join ( self . __paths );
        self . preprocess_options = None /* Option */;
        if self . __arch == "Intel" {
        self . compile_options = [ "/nologo" , "/Ox" , "/MD" , "/W3" , "/GX" ,;
        "/DNDEBUG" ];
        self . compile_options_debug = [ "/nologo" , "/Od" , "/MDd" , "/W3" , "/GX" ,;
        "/Z7" , "/D_DEBUG" ];
        } else {
        self . compile_options = [ "/nologo" , "/Ox" , "/MD" , "/W3" , "/GS-" ,;
        "/DNDEBUG" ];
        self . compile_options_debug = [ "/nologo" , "/Od" , "/MDd" , "/W3" , "/GS-" ,;
        "/Z7" , "/D_DEBUG" ];
        self . ldflags_shared = [ "/DLL" , "/nologo" , "/INCREMENTAL:NO" ];
        if self . __version >= 7 {
        self . ldflags_shared_debug = [;
        "/DLL" , "/nologo" , "/INCREMENTAL:no" , "/DEBUG";
        ];
        } else {
        self . ldflags_shared_debug = [;
        "/DLL" , "/nologo" , "/INCREMENTAL:no" , "/pdb:None /* Option */" , "/DEBUG";
        ];
        self . ldflags_static = [ "/nologo" ];
        self . initialized = true;
        pub fn object_filenames ( &self, {
        source_filenames ,;
        strip_dir = 0 ,;
        output_dir = "" ) ;
        if output_dir is None /* Option */ { : output_dir = ""; }
        obj_names = [ ];
        for src_name in source_filenames .iter() {
        ( base , ext ) = os . path . splitext ( src_name );
        base = os . path . splitdrive ( base ) [ 1 ];
        base = base [ os . path . isabs ( base ) : ];
        if ext !in self . src_extensions {
        panic!("CompileError ( "Don't know how to compile %s" % src_name )");
        if strip_dir {
        base = os . path . basename ( base );
        if ext in self . _rc_extensions {
        obj_names . append ( os . path . join ( output_dir ,;
        base + self . res_extension ) );
        } else if ext in self . _mc_extensions {
        obj_names . append ( os . path . join ( output_dir ,;
        base + self . res_extension ) );
        } else {
        obj_names . append ( os . path . join ( output_dir ,;
        base + self . obj_extension ) );
        return  obj_names;
        pub fn compile ( &self, sources , {
        output_dir = None /* Option */ , macros = None /* Option */ , include_dirs = None /* Option */ , debug = 0 ,;
        extra_preargs = None /* Option */ , extra_postargs = None /* Option */ , depends = None /* Option */ ) ;
        if !self . initialized {
        self . initialize ( );
        compile_info = self . _setup_compile ( output_dir , macros , include_dirs ,;
        sources , depends , extra_postargs );
        macros , objects , extra_postargs , pp_opts , build = compile_info;
        compile_opts = extra_preargs || [ ];
        compile_opts . append ( "/c" );
        if debug {
        compile_opts . extend ( self . compile_options_debug );
        } else {
        compile_opts . extend ( self . compile_options );
        for obj in objects .iter() {
        // try {
        src , ext = build [ obj ];
        // } catch  KeyError  {
        continue;
        if debug {
        src = os . path . abspath ( src );
        if ext in self . _c_extensions {
        input_opt = "/Tc" + src;
        } else if ext in self . _cpp_extensions {
        input_opt = "/Tp" + src;
        } else if ext in self . _rc_extensions {
        input_opt = src;
        output_opt = "/fo" + obj;
        // try {
        self . spawn ( [ self . rc ] + pp_opts +;
        [ output_opt ] + [ input_opt ] );
        // } catch  DistutilsExecError as msg  {
        panic!("CompileError ( msg )");
        continue;
        } else if ext in self . _mc_extensions {
        h_dir = os . path . dirname ( src );
        rc_dir = os . path . dirname ( obj );
        // try {
        self . spawn ( [ self . mc ] +;
        [ "-h" , h_dir , "-r" , rc_dir ] + [ src ] );
        base , _ = os . path . splitext ( os . path . basename ( src ) );
        rc_file = os . path . join ( rc_dir , base + ".rc" );
        self . spawn ( [ self . rc ] +;
        [ "/fo" + obj ] + [ rc_file ] );
        // } catch  DistutilsExecError as msg  {
        panic!("CompileError ( msg )");
        continue;
        } else {
        panic!("CompileError ( "Don't know how to compile %s to %s"");
        % ( src , obj ) );
        output_opt = "/Fo" + obj;
        // try {
        self . spawn ( [ self . cc ] + compile_opts + pp_opts +;
        [ input_opt , output_opt ] +;
        extra_postargs );
        // } catch  DistutilsExecError as msg  {
        panic!("CompileError ( msg )");
        return  objects;
        pub fn create_static_lib ( &self, {
        objects ,;
        output_libname ,;
        output_dir = None /* Option */ ,;
        debug = 0 ,;
        target_lang = None /* Option */ ) ;
        if !self . initialized {
        self . initialize ( );
        ( objects , output_dir ) = self . _fix_object_args ( objects , output_dir );
        output_filename = self . library_filename ( output_libname ,;
        output_dir = output_dir );
        if self . _need_link ( objects , output_filename ) {
        lib_args = objects + [ "/OUT:" + output_filename ];
        if debug {
        // pass
        // try {
        self . spawn ( [ self . lib ] + lib_args );
        // } catch  DistutilsExecError as msg  {
        panic!("LibError ( msg )");
        } else {
        log . debug ( "skipping %s (up-to-date)" , output_filename );
        pub fn link ( &self, {
        target_desc ,;
        objects ,;
        output_filename ,;
        output_dir = None /* Option */ ,;
        libraries = None /* Option */ ,;
        library_dirs = None /* Option */ ,;
        runtime_library_dirs = None /* Option */ ,;
        export_symbols = None /* Option */ ,;
        debug = 0 ,;
        extra_preargs = None /* Option */ ,;
        extra_postargs = None /* Option */ ,;
        build_temp = None /* Option */ ,;
        target_lang = None /* Option */ ) ;
        if !self . initialized {
        self . initialize ( );
        ( objects , output_dir ) = self . _fix_object_args ( objects , output_dir );
        fixed_args = self . _fix_lib_args ( libraries , library_dirs ,;
        runtime_library_dirs );
        ( libraries , library_dirs , runtime_library_dirs ) = fixed_args;
        if runtime_library_dirs {
        self . warn ( "I don't know what to do with 'runtime_library_dirs': ";
        + str ( runtime_library_dirs ) );
        lib_opts = gen_lib_options ( self ,;
        library_dirs , runtime_library_dirs ,;
        libraries );
        if output_dir is !None /* Option */ {
        output_filename = os . path . join ( output_dir , output_filename );
        if self . _need_link ( objects , output_filename ) {
        if target_desc == CCompiler . EXECUTABLE {
        if debug {
        ldflags = self . ldflags_shared_debug [ 1 : ];
        } else {
        ldflags = self . ldflags_shared [ 1 : ];
        } else {
        if debug {
        ldflags = self . ldflags_shared_debug;
        } else {
        ldflags = self . ldflags_shared;
        export_opts = [ ];
        for sym in ( export_symbols || [ ] ) .iter() {
        export_opts . append ( "/EXPORT:" + sym );
        ld_args = ( ldflags + lib_opts + export_opts +;
        objects + [ "/OUT:" + output_filename ] );
        if export_symbols is !None /* Option */ {
        ( dll_name , dll_ext ) = os . path . splitext (;
        os . path . basename ( output_filename ) );
        implib_file = os . path . join (;
        os . path . dirname ( objects [ 0 ] ) ,;
        self . library_filename ( dll_name ) );
        ld_args . append ( "/IMPLIB:" + implib_file );
        if extra_preargs {
        ld_args [ : 0 ] = extra_preargs;
        if extra_postargs {
        ld_args . extend ( extra_postargs );
        self . mkpath ( os . path . dirname ( output_filename ) );
        // try {
        self . spawn ( [ self . linker ] + ld_args );
        // } catch  DistutilsExecError as msg  {
        panic!("LinkError ( msg )");
        } else {
        log . debug ( "skipping %s (up-to-date)" , output_filename );
        pub fn library_dir_option ( &self, dir )  {
        return  "/LIBPATH:" + dir;
        pub fn runtime_library_dir_option ( &self, dir )  {
        panic!("DistutilsPlatformError (");
        "don't know how to set runtime library search path for MSVC++" );
        pub fn library_option ( &self, lib )  {
        return  self . library_filename ( lib );
        pub fn find_library_file ( &self, dirs , lib , debug = 0 )  {
        if debug {
        try_names = [ lib + "_d" , lib ];
        } else {
        try_names = [ lib ];
        for dir in dirs .iter() {
        for name in try_names .iter() {
        libfile = os . path . join ( dir , self . library_filename ( name ) );
        if os . path . exists ( libfile ) {
        return  libfile;
        } else {
        return;
        pub fn find_exe ( &self, exe )  {
        "Return path to an MSVC executable program.

        Tries to find the program in several places: first, one of the
        MSVC program search paths from the registry; next, the directories
        in the PATH environment variable.  If any of those work, return an
        absolute path that == known to exist.  If none of them work, just
        return the original program name, 'exe'.
        ";
        for p in self . __paths .iter() {
        fn = os . path . join ( os . path . abspath ( p ) , exe );
        if os . path . isfile ( fn ) {
        return  fn;
        for p in os . environ [ "Path" ] . split ( ";" ) .iter() {
        fn = os . path . join ( os . path . abspath ( p ) , exe );
        if os . path . isfile ( fn ) {
        return  fn;
        return  exe;
        pub fn get_msvc_paths ( &self, path , platform = "x86" )  {
        "Get a list of devstudio directories (include, lib || path).

        Return a list of strings.  The list will be empty if unable to
        access the registry || appropriate registry keys !found.
        ";
        if !_can_read_reg {
        return  [ ];
        path = path + " dirs";
        if self . __version >= 7 {
        key = ( r "%s\%0.1f\VC\VC_OBJECTS_PLATFORM_INFO\Win32\Directories";
        % ( self . __root , self . __version ) );
        } else {
        key = ( r "%s\6.0\Build System\Components\Platforms";
        r "\Win32 (%s)\Directories" % ( self . __root , platform ) );
        for base in HKEYS .iter() {
        d = read_values ( base , key );
        if d {
        if self . __version >= 7 {
        return  self . __macros . sub ( d [ path ] ) . split ( ";" );
        } else {
        return  d [ path ] . split ( ";" );
        if self . __version == 6 {
        for base in HKEYS .iter() {
        if read_values ( base , r "%s\6.0" % self . __root ) is !None /* Option */ {
        self . warn ( "It seems you have Visual Studio 6 installed, ";
        "but the expected registry settings are !present.\n";
        "You must at least run the Visual Studio GUI once ";
        "so that these entries are created." );
        break;
        return  [ ];
        pub fn set_path_env_var ( &self, name )  {
        "Set environment variable 'name' to an MSVC path type value.

        This == equivalent to a SET command prior to execution of spawned
        commands.
        ";
        if name == "lib" {
        p = self . get_msvc_paths ( "library" );
        } else {
        p = self . get_msvc_paths ( name );
        if p {
        os . environ [ name ] = ";" . join ( p );
        if get_build_version ( ) >= 8.0 {
        log . debug ( "Importing new compiler from distutils.msvc9compiler" );
        OldMSVCCompiler = MSVCCompiler;
        from distutils . msvc9compiler import MSVCCompiler;
        from distutils . msvc9compiler import MacroExpander;
}

