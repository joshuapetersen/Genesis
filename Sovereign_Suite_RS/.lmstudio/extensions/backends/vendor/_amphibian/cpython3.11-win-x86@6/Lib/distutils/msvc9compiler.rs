//! msvc9compiler.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::distutils::{DistutilsExecError, DistutilsPlatformError, \};
// use crate::winreg;

pub const RegOpenKeyEx: f64 = winreg . OpenKeyEx;
pub const RegEnumKey: f64 = winreg . EnumKey;
pub const RegEnumValue: f64 = winreg . EnumValue;
pub const RegError: f64 = winreg . error;
pub const HKEYS: f64 = ( winreg . HKEY_USERS ,;
pub const NATIVE_WIN64: &str = ( sys . platform =="win32" and sys . maxsize > 2 ** 32 );
pub const PLAT_TO_VCVARS: f64 = {;
pub struct Reg {
    pub macros: String, // TODO: infer type
    pub vsbase: String, // TODO: infer type
    pub __version: String, // TODO: infer type
    pub __root: String, // TODO: infer type
    pub __paths: String, // TODO: infer type
    pub plat_name: String, // TODO: infer type
    pub __arch: String, // TODO: infer type
    pub initialized: String, // TODO: infer type
    pub cc: String, // TODO: infer type
    pub linker: String, // TODO: infer type
    pub lib: String, // TODO: infer type
    pub rc: String, // TODO: infer type
    pub mc: String, // TODO: infer type
    pub preprocess_options: String, // TODO: infer type
    pub compile_options: String, // TODO: infer type
    pub compile_options_debug: String, // TODO: infer type
    pub ldflags_shared: String, // TODO: infer type
    pub ldflags_shared_debug: String, // TODO: infer type
    pub ldflags_static: String, // TODO: infer type
}

impl Reg {
}

pub struct MacroExpander {
    pub macros: String, // TODO: infer type
    pub vsbase: String, // TODO: infer type
    pub __version: String, // TODO: infer type
    pub __root: String, // TODO: infer type
    pub __paths: String, // TODO: infer type
    pub plat_name: String, // TODO: infer type
    pub __arch: String, // TODO: infer type
    pub initialized: String, // TODO: infer type
    pub cc: String, // TODO: infer type
    pub linker: String, // TODO: infer type
    pub lib: String, // TODO: infer type
    pub rc: String, // TODO: infer type
    pub mc: String, // TODO: infer type
    pub preprocess_options: String, // TODO: infer type
    pub compile_options: String, // TODO: infer type
    pub compile_options_debug: String, // TODO: infer type
    pub ldflags_shared: String, // TODO: infer type
    pub ldflags_shared_debug: String, // TODO: infer type
    pub ldflags_static: String, // TODO: infer type
}

impl MacroExpander {
    pub fn new(version: &str) -> Self {
        self . macros = { };
        self . vsbase = VS_BASE % version;
        self . load_macros ( version );
    }

    pub fn get_build_version(&self) {
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
        pub fn removeDuplicates ( variable )  {
        "Remove duplicate values of an environment variable.
    ";
        oldList = variable . split ( os . pathsep );
        newList = [ ];
        for i in oldList .iter() {
        if i !in newList {
        newList . append ( i );
        newVariable = os . pathsep . join ( newList );
        return  newVariable;
        pub fn find_vcvarsall ( version )  {
        "Find the vcvarsall.bat file

    At first it tries to find the productdir of VS 2008 in the registry. If
    that fails it falls back to the VS90COMNTOOLS env var.
    ";
        vsbase = VS_BASE % version;
        // try {
        productdir = Reg . get_value ( r "%s\Setup\VC" % vsbase ,;
        "productdir" );
        // } catch  KeyError  {
        log . debug ( "Unable to find productdir in registry" );
        productdir = None /* Option */;
        if !productdir || !os . path . isdir ( productdir ) {
        toolskey = "VS%0.f0COMNTOOLS" % version;
        toolsdir = os . environ . get ( toolskey , None /* Option */ );
        if toolsdir && os . path . isdir ( toolsdir ) {
        productdir = os . path . join ( toolsdir , os . pardir , os . pardir , "VC" );
        productdir = os . path . abspath ( productdir );
        if !os . path . isdir ( productdir ) {
        log . debug ( "%s == !a valid directory" % productdir );
        return;
        } else {
        log . debug ( "Env var %s == !set || invalid" % toolskey );
        if !productdir {
        log . debug ( "No productdir found" );
        return;
        vcvarsall = os . path . join ( productdir , "vcvarsall.bat" );
        if os . path . isfile ( vcvarsall ) {
        return  vcvarsall;
        log . debug ( "Unable to find vcvarsall.bat" );
        return;
        pub fn query_vcvarsall ( version , arch = "x86" )  {
        "Launch vcvarsall.bat && read the settings from its environment
    ";
        vcvarsall = find_vcvarsall ( version );
        interesting = { "include" , "lib" , "libpath" , "path" };
        result = { };
        if vcvarsall is None /* Option */ {
        panic!("DistutilsPlatformError ( "Unable to find vcvarsall.bat" )");
        log . debug ( "Calling 'vcvarsall.bat %s' (version=%s)" , arch , version );
        popen = subprocess . Popen ( ""%s" %s & set" % ( vcvarsall , arch ) ,;
        stdout = subprocess . PIPE ,;
        stderr = subprocess . PIPE );
        // try {
        stdout , stderr = popen . communicate ( );
        if popen . wait ( ) != 0 {
        panic!("DistutilsPlatformError ( stderr . decode ( "mbcs" ) )");
        stdout = stdout . decode ( "mbcs" );
        for line in stdout . split ( "\n" ) .iter() {
        line = Reg . convert_mbcs ( line );
        if "=" !in line {
        continue;
        line = line . strip ( );
        key , value = line . split ( "=" , 1 );
        key = key . lower ( );
        if key in interesting {
        if value . endswith ( os . pathsep ) {
        value = value [ : -1 ];
        result [ key ] = removeDuplicates ( value );
        // } finally {
        popen . stdout . close ( );
        popen . stderr . close ( );
        if len ( result ) != len ( interesting ) {
        panic!("ValueError ( str ( list ( result . keys ( ) ) ) )");
        return  result;
        VERSION = get_build_version ( );
        if VERSION < 8.0 {
        panic!("DistutilsPlatformError ( "VC %0.1f is !supported by this module" % VERSION )");
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
        self . __version = VERSION;
        self . __root = r "Software\Microsoft\VisualStudio";
        self . __paths = [ ];
        self . plat_name = None /* Option */;
        self . __arch = None /* Option */;
        self . initialized = false;
        pub fn initialize ( &self, plat_name = None /* Option */ )  {
        assert !self . initialized , "don't init multiple times";
        if plat_name is None /* Option */ {
        plat_name = get_platform ( );
        ok_plats = "win32" , "win-amd64";
        if plat_name !in ok_plats {
        panic!("DistutilsPlatformError ( "--plat-name must be one of %s" %");
        ( ok_plats , ) );
        if "DISTUTILS_USE_SDK" in os . environ && "MSSdk" in os . environ && self . find_exe ( "cl.exe" ) {
        self . cc = "cl.exe";
        self . linker = "link.exe";
        self . lib = "lib.exe";
        self . rc = "rc.exe";
        self . mc = "mc.exe";
        } else {
        if plat_name == get_platform ( ) || plat_name == "win32" {
        plat_spec = PLAT_TO_VCVARS [ plat_name ];
        } else {
        plat_spec = PLAT_TO_VCVARS [ get_platform ( ) ] + "_" + \;
        PLAT_TO_VCVARS [ plat_name ];
        vc_env = query_vcvarsall ( VERSION , plat_spec );
        self . __paths = vc_env [ "path" ] . split ( os . pathsep );
        os . environ [ "lib" ] = vc_env [ "lib" ];
        os . environ [ "include" ] = vc_env [ "include" ];
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
        // try {
        for p in os . environ [ "path" ] . split ( ";" ) .iter() {
        self . __paths . append ( p );
        // } catch  KeyError  {
        // pass
        self . __paths = normalize_and_reduce_paths ( self . __paths );
        os . environ [ "path" ] = ";" . join ( self . __paths );
        self . preprocess_options = None /* Option */;
        if self . __arch == "x86" {
        self . compile_options = [ "/nologo" , "/Ox" , "/MD" , "/W3" ,;
        "/DNDEBUG" ];
        self . compile_options_debug = [ "/nologo" , "/Od" , "/MDd" , "/W3" ,;
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
        build_temp = os . path . dirname ( objects [ 0 ] );
        if export_symbols is !None /* Option */ {
        ( dll_name , dll_ext ) = os . path . splitext (;
        os . path . basename ( output_filename ) );
        implib_file = os . path . join (;
        build_temp ,;
        self . library_filename ( dll_name ) );
        ld_args . append ( "/IMPLIB:" + implib_file );
        self . manifest_setup_ldargs ( output_filename , build_temp , ld_args );
        if extra_preargs {
        ld_args [ : 0 ] = extra_preargs;
        if extra_postargs {
        ld_args . extend ( extra_postargs );
        self . mkpath ( os . path . dirname ( output_filename ) );
        // try {
        self . spawn ( [ self . linker ] + ld_args );
        // } catch  DistutilsExecError as msg  {
        panic!("LinkError ( msg )");
        mfinfo = self . manifest_get_embed_info ( target_desc , ld_args );
        if mfinfo is !None /* Option */ {
        mffilename , mfid = mfinfo;
        out_arg = "-outputresource:%s;%s" % ( output_filename , mfid );
        // try {
        self . spawn ( [ "mt.exe" , "-nologo" , "-manifest" ,;
        mffilename , out_arg ] );
        // } catch  DistutilsExecError as msg  {
        panic!("LinkError ( msg )");
        } else {
        log . debug ( "skipping %s (up-to-date)" , output_filename );
        pub fn manifest_setup_ldargs ( &self, output_filename , build_temp , ld_args )  {
        temp_manifest = os . path . join (;
        build_temp ,;
        os . path . basename ( output_filename ) + ".manifest" );
        ld_args . append ( "/MANIFESTFILE:" + temp_manifest );
        pub fn manifest_get_embed_info ( &self, target_desc , ld_args )  {
        for arg in ld_args .iter() {
        if arg . startswith ( "/MANIFESTFILE:" ) {
        temp_manifest = arg . split ( ":" , 1 ) [ 1 ];
        break;
        } else {
        return;
        if target_desc == CCompiler . EXECUTABLE {
        mfid = 1;
        } else {
        mfid = 2;
        temp_manifest = self . _remove_visual_c_ref ( temp_manifest );
        if temp_manifest is None /* Option */ {
        return;
        return  temp_manifest , mfid;
        pub fn _remove_visual_c_ref ( &self, manifest_file )  {
        // try {
        manifest_f = open ( manifest_file );
        // try {
        manifest_buf = manifest_f . read ( );
        // } finally {
        manifest_f . close ( );
        pattern = re . compile (;
        r "<assemblyIdentity.*?name=("|')Microsoft\." \;
        r "VC\d{2}\.CRT("|').*?(/>|</assemblyIdentity>)" ,;
        re . DOTALL );
        manifest_buf = re . sub ( pattern , "" , manifest_buf );
        pattern = r "<dependentAssembly>\s*</dependentAssembly>";
        manifest_buf = re . sub ( pattern , "" , manifest_buf );
        pattern = re . compile (;
        r "<assemblyIdentity.*?name=(?:"|')(.+?)(?:"|')";
        r ".*?(?:/>|</assemblyIdentity>)" , re . DOTALL );
        if re . search ( pattern , manifest_buf ) is None /* Option */ {
        return;
        manifest_f = open ( manifest_file , "w" );
        // try {
        manifest_f . write ( manifest_buf );
        return  manifest_file;
        // } finally {
        manifest_f . close ( );
        // } catch  OSError  {
        // pass
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
    }

}

