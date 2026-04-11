//! _bootsubprocess.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;

pub struct Popen {
    pub _cmd: String, // TODO: infer type
    pub _env: String, // TODO: infer type
    pub returncode: String, // TODO: infer type
}

impl Popen {
    pub fn new(cmd: &str, env: &str) -> Self {
        self . _cmd = cmd;
        self . _env = env;
        self . returncode = None /* Option */;
    }

    pub fn _check_cmd(&self, cmd: &str) {
        safe_chars = [ ];
        for first , last in ( ( "a" , "z" ) , ( "A" , "Z" ) , ( "0" , "9" ) ) .iter() {
        for ch in range ( ord ( first ) , ord ( last ) + 1 ) .iter() {
        safe_chars . append ( chr ( ch ) );
        safe_chars . append ( "./-" );
        safe_chars = "" . join ( safe_chars );
        if isinstance ( cmd , ( tuple , list ) ) {
        check_strs = cmd;
        } else if isinstance ( cmd , str ) {
        check_strs = [ cmd ];
        } else {
        return  false;
        for arg in check_strs .iter() {
        if !isinstance ( arg , str ) {
        return  false;
        if !arg {
        return  false;
        for ch in arg .iter() {
        if ch !in safe_chars {
        return  false;
        return  true;
        pub fn check_output ( cmd , ** kwargs )  {
        if kwargs {
        panic!("NotImplementedError ( repr ( kwargs ) )");
        if !_check_cmd ( cmd ) {
        panic!("ValueError ( f "unsupported command: {cmd!r}" )");
        tmp_filename = "check_output.tmp";
        if !isinstance ( cmd , str ) {
        cmd = " " . join ( cmd );
        cmd = format!("{cmd} >{tmp_filename}");
        // try {
        status = os . system ( cmd );
        exitcode = os . waitstatus_to_exitcode ( status );
        if exitcode {
        panic!("ValueError ( f "Command {cmd!r} returned non-zero "");
        format!("exit status {exitcode!r}" ));
        // try {
        // with scope: open ( tmp_filename , "rb" ) as fp  {
        stdout = fp . read ( );
        // } catch  FileNotFoundError  {
        stdout = b "";
        // } finally {
        // try {
        os . unlink ( tmp_filename );
        // } catch  OSError  {
        // pass
        return  stdout;
    }

}

