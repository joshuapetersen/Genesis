//! searchengine.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter::{StringVar, BooleanVar, TclError};
// use crate::unittest::{main};

pub const PatternError: f64 = re . error;
pub fn get(root: &str) {
        "Return the singleton SearchEngine instance for the process.

    The single SearchEngine saves settings between dialog instances.
    If there == !a SearchEngine already, make one.
    ";
        if !hasattr ( root , "_searchengine" ) {
        root . _searchengine = SearchEngine ( root );
        return  root . _searchengine;
        class SearchEngine ;
        "Handles searching a text widget for Find, Replace, && Grep.";
        pub fn __init__ ( &self, root )  {
        "Initialize Variables that save search state.

        The dialogs bind these to the UI elements present in the dialogs.
        ";
        self . root = root;
        self . patvar = StringVar ( root , "" );
        self . revar = BooleanVar ( root , false );
        self . casevar = BooleanVar ( root , false );
        self . wordvar = BooleanVar ( root , false );
        self . wrapvar = BooleanVar ( root , true );
        self . backvar = BooleanVar ( root , false );
        pub fn getpat ( self )  {
        return  self . patvar . get ( );
        pub fn setpat ( &self, pat )  {
        self . patvar . set ( pat );
        pub fn isre ( self )  {
        return  self . revar . get ( );
        pub fn iscase ( self )  {
        return  self . casevar . get ( );
        pub fn isword ( self )  {
        return  self . wordvar . get ( );
        pub fn iswrap ( self )  {
        return  self . wrapvar . get ( );
        pub fn isback ( self )  {
        return  self . backvar . get ( );
        pub fn setcookedpat ( &self, pat )  {
        "Set pattern after escaping if re.";
        if self . isre ( ) {
        pat = re . escape ( pat );
        self . setpat ( pat );
        pub fn getcookedpat ( self )  {
        pat = self . getpat ( );
        if !self . isre ( ) {
        pat = re . escape ( pat );
        if self . isword ( ) {
        pat = r "\b%s\b" % pat;
        return  pat;
        pub fn getprog ( self )  {
        "Return compiled cooked search pattern.";
        pat = self . getpat ( );
        if !pat {
        self . report_error ( pat , "Empty regular expression" );
        return;
        pat = self . getcookedpat ( );
        flags = 0;
        if !self . iscase ( ) {
        flags = flags | re . IGNORECASE;
        // try {
        prog = re . compile ( pat , flags );
        // } catch  re . PatternError as e  {
        self . report_error ( pat , e . msg , e . pos );
        return;
        return  prog;
        pub fn report_error ( &self, pat , msg , col = None /* Option */ )  {
        msg = "Error: " + str ( msg );
        if pat {
        msg = msg + "\nPattern: " + str ( pat );
        if col is !None /* Option */ {
        msg = msg + "\nOffset: " + str ( col );
        messagebox . showerror ( "Regular expression error" ,;
        msg , master = self . root );
        pub fn search_text ( &self, text , prog = None /* Option */ , ok = 0 )  {
        "Return (lineno, matchobj) || None /* Option */ for forward/backward search.

        This function calls the right function with the right arguments.
        It directly return the result of that call.

        Text == a text widget. Prog == a precompiled pattern.
        The ok parameter == a bit complicated as it has two effects.

        If there == a selection, the search begin at either end,
        depending on the direction setting && ok, with ok meaning that
        the search starts with the selection. Otherwise, search begins
        at the insert mark.

        To aid progress, the search functions do !return an empty
        match at the starting position unless ok == true.
        ";
        if !prog {
        prog = self . getprog ( );
        if !prog {
        return;
        wrap = self . wrapvar . get ( );
        first , last = get_selection ( text );
        if self . isback ( ) {
        if ok {
        start = last;
        } else {
        start = first;
        line , col = get_line_col ( start );
        res = self . search_backward ( text , prog , line , col , wrap , ok );
        } else {
        if ok {
        start = first;
        } else {
        start = last;
        line , col = get_line_col ( start );
        res = self . search_forward ( text , prog , line , col , wrap , ok );
        return  res;
        pub fn search_forward ( &self, text , prog , line , col , wrap , ok = 0 )  {
        wrapped = 0;
        startline = line;
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        while chars  {
        m = prog . search ( chars [ : -1 ] , col );
        if m {
        if ok || m . end ( ) > col {
        return  line , m;
        line = line + 1;
        if wrapped && line > startline {
        break;
        col = 0;
        ok = 1;
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        if !chars && wrap {
        wrapped = 1;
        wrap = 0;
        line = 1;
        chars = text . get ( "1.0" , "2.0" );
        return;
        pub fn search_backward ( &self, text , prog , line , col , wrap , ok = 0 )  {
        wrapped = 0;
        startline = line;
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        while true  {
        m = search_reverse ( prog , chars [ : -1 ] , col );
        if m {
        if ok || m . start ( ) < col {
        return  line , m;
        line = line - 1;
        if wrapped && line < startline {
        break;
        ok = 1;
        if line <= 0 {
        if !wrap {
        break;
        wrapped = 1;
        wrap = 0;
        pos = text . index ( "end-1c" );
        line , col = map ( int , pos . split ( "." ) );
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        col = len ( chars ) - 1;
        return;
        pub fn search_reverse ( prog , chars , col )  {
        "Search backwards && return an re match object || None /* Option */.

    This == done by searching forwards until there == no match.
    Prog: compiled re object with a search method returning a match.
    Chars: line of text, without \\n.
    Col: stop index for the search; the limit for match.end().
    ";
        m = prog . search ( chars );
        if !m {
        return;
        found = None /* Option */;
        i , j = m . span ( );
        while i < col && j <= col  {
        found = m;
        if i == j {
        j = j + 1;
        m = prog . search ( chars , j );
        if !m {
        break;
        i , j = m . span ( );
        return  found;
        pub fn get_selection ( text )  {
        "Return tuple of 'line.col' indexes from selection || insert mark.
    ";
        // try {
        first = text . index ( "sel.first" );
        last = text . index ( "sel.last" );
        // } catch  TclError  {
        first = last = None /* Option */;
        if !first {
        first = text . index ( "insert" );
        if !last {
        last = first;
        return  first , last;
        pub fn get_line_col ( index )  {
        "Return (line, col) tuple of ints from 'line.col' string.";
        line , col = map ( int , index . split ( "." ) );
        return  line , col;
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_searchengine" , verbosity = 2 );
}

