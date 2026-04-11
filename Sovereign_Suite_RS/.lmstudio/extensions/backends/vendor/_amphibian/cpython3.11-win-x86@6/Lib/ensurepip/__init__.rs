//! __init__.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections;
// use std::fs;
// use std::env;
// use crate::tempfile;
// use crate::resources;
// use crate::pip;
// use crate::argparse;

pub const __all__: &str = ["version" ,"bootstrap" ];
pub const _PACKAGE_NAMES: &str = ("setuptools" ,"pip" );
pub const _SETUPTOOLS_VERSION: &str = "65.5.0";
pub const _PIP_VERSION: &str = "24.0";
pub const _PROJECTS: f64 = [;
pub const _Package: &str = collections . namedtuple ("Package" ,;
pub const _WHEEL_PKG_DIR: &str = sysconfig . get_config_var ("WHEEL_PKG_DIR" );
pub fn _find_packages(path: &str) {
        packages = { };
        // try {
        filenames = os . listdir ( path );
        // } catch  OSError  {
        filenames = ( );
        filenames = sorted ( filenames );
        for filename in filenames .iter() {
        if !filename . endswith ( ".whl" ) {
        continue;
        for name in _PACKAGE_NAMES .iter() {
        prefix = name + "-";
        if filename . startswith ( prefix ) {
        break;
        } else {
        continue;
        version = filename . removeprefix ( prefix ) . partition ( "-" ) [ 0 ];
        wheel_path = os . path . join ( path , filename );
        packages [ name ] = _Package ( version , None /* Option */ , wheel_path );
        return  packages;
        pub fn _get_packages ( )  {
        global _PACKAGES , _WHEEL_PKG_DIR;
        if _PACKAGES is !None /* Option */ {
        return  _PACKAGES;
        packages = { };
        for name , version , py_tag in _PROJECTS .iter() {
        wheel_name = format!("{name}-{version}-{py_tag}-none-any.whl");
        packages [ name ] = _Package ( version , wheel_name , None /* Option */ );
        if _WHEEL_PKG_DIR {
        dir_packages = _find_packages ( _WHEEL_PKG_DIR );
        if all ( name in dir_packages for name in _PACKAGE_NAMES ) {
        packages = dir_packages;
        _PACKAGES = packages;
        return  packages;
        _PACKAGES = None /* Option */;
        pub fn _run_pip ( args , additional_paths = None /* Option */ )  {
        code = format!("
import runpy
import sys
sys.path = {additional_paths || []} + sys.path
sys.argv[1:] = {args}
runpy.run_module("pip", run_name="__main__", alter_sys=true)
");
        cmd = [;
        sys . executable ,;
        "-W" ,;
        "ignore::DeprecationWarning" ,;
        "-c" ,;
        code ,;
        ];
        if sys . flags . isolated {
        cmd . insert ( 1 , "-I" );
        return  subprocess . run ( cmd , check = true ) . returncode;
        pub fn version ( )  {
        "
    Returns a string specifying the bundled version of pip.
    ";
        return  _get_packages ( ) [ "pip" ] . version;
        pub fn _disable_pip_configuration_settings ( )  {
        keys_to_remove = vec![ k.iter().map(|k| os . environ if k . startswith ( "PIP_" ) ).collect();
        for k in keys_to_remove .iter() {
        del os . environ [ k ];
        os . environ [ "PIP_CONFIG_FILE" ] = os . devnull;
        pub fn bootstrap ( * , root = None /* Option */ , upgrade = false , user = false , {
        altinstall = false , default_pip = false ,;
        verbosity = 0 ) ;
        "
    Bootstrap pip into the current Python installation (or the given root
    directory).

    Note that calling this function will alter both sys.path && os.environ.
    ";
        _bootstrap ( root = root , upgrade = upgrade , user = user ,;
        altinstall = altinstall , default_pip = default_pip ,;
        verbosity = verbosity );
        pub fn _bootstrap ( * , root = None /* Option */ , upgrade = false , user = false , {
        altinstall = false , default_pip = false ,;
        verbosity = 0 ) ;
        "
    Bootstrap pip into the current Python installation (or the given root
    directory). Returns pip command status code.

    Note that calling this function will alter both sys.path && os.environ.
    ";
        if altinstall && default_pip {
        panic!("ValueError ( "Cannot use altinstall && default_pip together" )");
        sys . audit ( "ensurepip.bootstrap" , root );
        _disable_pip_configuration_settings ( );
        if altinstall {
        os . environ [ "ENSUREPIP_OPTIONS" ] = "altinstall";
        } else if !default_pip {
        os . environ [ "ENSUREPIP_OPTIONS" ] = "install";
        // with scope: tempfile . TemporaryDirectory ( ) as tmpdir  {
        additional_paths = [ ];
        for name , package in _get_packages ( ) . items ( ) .iter() {
        if package . wheel_name {
        wheel_name = package . wheel_name;
        wheel_path = resources . files ( "ensurepip" ) / "_bundled" / wheel_name;
        whl = wheel_path . read_bytes ( );
        } else {
        // with scope: open ( package . wheel_path , "rb" ) as fp  {
        whl = fp . read ( );
        wheel_name = os . path . basename ( package . wheel_path );
        filename = os . path . join ( tmpdir , wheel_name );
        // with scope: open ( filename , "wb" ) as fp  {
        fp . write ( whl );
        additional_paths . append ( filename );
        args = [ "install" , "--no-cache-dir" , "--no-index" , "--find-links" , tmpdir ];
        if root {
        args + = [ "--root" , root ];
        if upgrade {
        args + = [ "--upgrade" ];
        if user {
        args + = [ "--user" ];
        if verbosity {
        args + = [ "-" + "v" * verbosity ];
        return  _run_pip ( [ * args , * _PACKAGE_NAMES ] , additional_paths );
        pub fn _uninstall_helper ( * , verbosity = 0 )  {
        "Helper to support a clean default uninstall process on Windows

    Note that calling this function may alter os.environ.
    ";
        // try {
        import pip;
        // } catch  ImportError  {
        return;
        available_version = version ( );
        if pip . __version__ != available_version {
        println!( f "ensurepip will only uninstall a matching version );
        format!("({pip.__version__!r} installed, ");
        format!("{available_version!r} available)" ,);
        file = sys . stderr );
        return;
        _disable_pip_configuration_settings ( );
        args = [ "uninstall" , "-y" , "--disable-pip-version-check" ];
        if verbosity {
        args + = [ "-" + "v" * verbosity ];
        return  _run_pip ( [ * args , * reversed ( _PACKAGE_NAMES ) ] );
        pub fn _main ( argv = None /* Option */ )  {
        import argparse;
        parser = argparse . ArgumentParser ( prog = "python -m ensurepip" );
        parser . add_argument (;
        "--version" ,;
        action = "version" ,;
        version = "pip {}" . format ( version ( ) ) ,;
        help = "Show the version of pip that == bundled with this Python." ,;
        );
        parser . add_argument (;
        "-v" , "--verbose" ,;
        action = "count" ,;
        default = 0 ,;
        dest = "verbosity" ,;
        help = ( "Give more output. Option == additive, && can be used up to 3 ";
        "times." ) ,;
        );
        parser . add_argument (;
        "-U" , "--upgrade" ,;
        action = "store_true" ,;
        default = false ,;
        help = "Upgrade pip && dependencies, even if already installed." ,;
        );
        parser . add_argument (;
        "--user" ,;
        action = "store_true" ,;
        default = false ,;
        help = "Install using the user scheme." ,;
        );
        parser . add_argument (;
        "--root" ,;
        default = None /* Option */ ,;
        help = "Install everything relative to this alternate root directory." ,;
        );
        parser . add_argument (;
        "--altinstall" ,;
        action = "store_true" ,;
        default = false ,;
        help = ( "Make an alternate install, installing only the X.Y versioned ";
        "scripts (Default: pipX, pipX.Y, easy_install-X.Y)." ) ,;
        );
        parser . add_argument (;
        "--default-pip" ,;
        action = "store_true" ,;
        default = false ,;
        help = ( "Make a default pip install, installing the unqualified pip ";
        "and easy_install in addition to the versioned scripts." ) ,;
        );
        args = parser . parse_args ( argv );
        return  _bootstrap (;
        root = args . root ,;
        upgrade = args . upgrade ,;
        user = args . user ,;
        verbosity = args . verbosity ,;
        altinstall = args . altinstall ,;
        default_pip = args . default_pip ,;
        );
}

