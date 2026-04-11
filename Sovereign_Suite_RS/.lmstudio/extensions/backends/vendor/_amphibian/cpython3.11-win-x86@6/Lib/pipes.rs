//! pipes.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tempfile;
// use crate::shlex::{quote};

pub const remove: f64 = ( 3 , 13 ) );
pub const __all__: &str = ["Template" ];
pub const FILEIN_FILEOUT: &str = "ff";
pub const STDIN_FILEOUT: &str = "-f";
pub const FILEIN_STDOUT: &str = "f-";
pub const STDIN_STDOUT: &str = "--";
pub const SOURCE: &str = ".-";
pub const SINK: &str = "-.";
pub const stepkinds: f64 = [ FILEIN_FILEOUT , STDIN_FILEOUT , FILEIN_STDOUT , STDIN_STDOUT , \;
pub struct Template {
    pub debugging: String, // TODO: infer type
    pub steps: String, // TODO: infer type
}

impl Template {
}

pub fn makepipeline(infile: &str, steps: &str, outfile: &str) {
        list = [ ];
        for cmd , kind in steps .iter() {
        list . append ( [ "" , cmd , kind , "" ] );
        if !list {
        list . append ( [ "" , "cat" , "--" , "" ] );
        [ cmd , kind ] = list [ 0 ] [ 1 : 3 ];
        if kind [ 0 ] == "f" && !infile {
        list . insert ( 0 , [ "" , "cat" , "--" , "" ] );
        list [ 0 ] [ 0 ] = infile;
        [ cmd , kind ] = list [ -1 ] [ 1 : 3 ];
        if kind [ 1 ] == "f" && !outfile {
        list . append ( [ "" , "cat" , "--" , "" ] );
        list [ -1 ] [ -1 ] = outfile;
        garbage = [ ];
        for i in range ( 1 , len ( list ) ) .iter() {
        lkind = list [ i -1 ] [ 2 ];
        rkind = list [ i ] [ 2 ];
        if lkind [ 1 ] == "f" || rkind [ 0 ] == "f" {
        ( fd , temp ) = tempfile . mkstemp ( );
        os . close ( fd );
        garbage . append ( temp );
        list [ i -1 ] [ -1 ] = list [ i ] [ 0 ] = temp;
        for item in list .iter() {
        [ inf , cmd , kind , outf ] = item;
        if kind [ 1 ] == "f" {
        cmd = "OUT=" + quote ( outf ) + "; " + cmd;
        if kind [ 0 ] == "f" {
        cmd = "IN=" + quote ( inf ) + "; " + cmd;
        if kind [ 0 ] == "-" && inf {
        cmd = cmd + " <" + quote ( inf );
        if kind [ 1 ] == "-" && outf {
        cmd = cmd + " >" + quote ( outf );
        item [ 1 ] = cmd;
        cmdlist = list [ 0 ] [ 1 ];
        for item in list [ 1 : ] .iter() {
        [ cmd , kind ] = item [ 1 : 3 ];
        if item [ 0 ] == "" {
        if "f" in kind {
        cmd = "{ " + cmd + "; }";
        cmdlist = cmdlist + " |\n" + cmd;
        } else {
        cmdlist = cmdlist + "\n" + cmd;
        if garbage {
        rmcmd = "rm -format!(");
        for file in garbage .iter() {
        rmcmd = rmcmd + " " + quote ( file );
        trapcmd = "trap " + quote ( rmcmd + "; exit" ) + " 1 2 3 13 14 15";
        cmdlist = trapcmd + "\n" + cmdlist + "\n" + rmcmd;
        return  cmdlist;
}

