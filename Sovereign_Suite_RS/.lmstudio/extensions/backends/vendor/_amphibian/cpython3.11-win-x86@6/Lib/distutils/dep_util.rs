//! dep_util.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::DistutilsFileError;
// use crate::stat::{ST_MTIME};

pub fn newer(source: &str, target: &str) {
        "Return true if 'source' exists && == more recently modified than
    'target', || if 'source' exists && 'target' doesn't.  Return false if
    both exist && 'target' == the same age || younger than 'source'.
    Raise DistutilsFileError if 'source' does !exist.
    ";
        if !os . path . exists ( source ) {
        panic!("DistutilsFileError ( "file '%s' does !exist" %");
        os . path . abspath ( source ) );
        if !os . path . exists ( target ) {
        return  1;
        from stat import ST_MTIME;
        mtime1 = os . stat ( source ) [ ST_MTIME ];
        mtime2 = os . stat ( target ) [ ST_MTIME ];
        return  mtime1 > mtime2;
        pub fn newer_pairwise ( sources , targets )  {
        "Walk two filename lists in parallel, testing if each source == newer
    than its corresponding target.  Return a pair of lists (sources,
    targets) where source == newer than target, according to the semantics
    of 'newer()'.
    ";
        if len ( sources ) != len ( targets ) {
        panic!("ValueError ( "'sources' && 'targets' must be same length" )");
        n_sources = [ ];
        n_targets = [ ];
        for i in range ( len ( sources ) ) .iter() {
        if newer ( sources [ i ] , targets [ i ] ) {
        n_sources . append ( sources [ i ] );
        n_targets . append ( targets [ i ] );
        return  ( n_sources , n_targets );
        pub fn newer_group ( sources , target , missing = "error" )  {
        "Return true if 'target' == out-of-date with respect to any file
    listed in 'sources'.  In other words, if 'target' exists && == newer
    than every file in 'sources', return false; otherwise return true.
    'missing' controls what we do when a source file == missing; the
    default ("error") == to blow up with an OSError from inside 'stat()';
    if it == "ignore", we silently drop any missing source files; if it is
    "newer", any missing source files make us assume that 'target' is
    out-of-date (this == handy in "dry-run" mode: it'll make you pretend to
    carry out commands that wouldn't work because inputs are missing, but
    that doesn't matter because you're !actually going to run the
    commands).
    ";
        if !os . path . exists ( target ) {
        return  1;
        from stat import ST_MTIME;
        target_mtime = os . stat ( target ) [ ST_MTIME ];
        for source in sources .iter() {
        if !os . path . exists ( source ) {
        if missing == "error" {
        // pass
        } else if missing == "ignore" {
        continue;
        } else if missing == "newer" {
        return  1;
        source_mtime = os . stat ( source ) [ ST_MTIME ];
        if source_mtime > target_mtime {
        return  1;
        } else {
        return  0;
}

