//! replace.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter::{StringVar, TclError};
// use crate::idlelib::{SearchDialogBase};
// use crate::unittest::{main};

pub const PatternError: f64 = re . error;
pub fn replace(text: &str, insert_tags: &str) {
        "Create || reuse a singleton ReplaceDialog instance.

    The singleton dialog saves user entries && preferences
    across instances.

    Args:
        text: Text widget containing the text to be searched.
    ";
        root = text . _root ( );
        engine = searchengine . get ( root );
        if !hasattr ( engine , "_replacedialog" ) {
        engine . _replacedialog = ReplaceDialog ( root , engine );
        dialog = engine . _replacedialog;
        searchphrase = text . get ( "sel.first" , "sel.last" );
        dialog . open ( text , searchphrase , insert_tags = insert_tags );
        class ReplaceDialog ( SearchDialogBase ) ;
        "Dialog for finding && replacing a pattern in text.";
        title = "Replace Dialog";
        icon = "Replace";
        pub fn __init__ ( &self, root , engine )  {
        "Create search dialog for finding && replacing text.

        Uses SearchDialogBase as the basis for the GUI && a
        searchengine instance to prepare the search.

        Attributes:
            replvar: StringVar containing 'Replace with:' value.
            replent: Entry widget for replvar.  Created in
                create_entries().
            ok: Boolean used in searchengine.search_text to indicate
                whether the search includes the selection.
        ";
        super ( ) . __init__ ( root , engine );
        self . replvar = StringVar ( root );
        self . insert_tags = None /* Option */;
        pub fn open ( &self, text , searchphrase = None /* Option */ , * , insert_tags = None /* Option */ )  {
        "Make dialog visible on top of others && ready to use.

        Also, set the search to include the current selection
        (self.ok).

        Args:
            text: Text widget being searched.
            searchphrase: String phrase to search.
        ";
        SearchDialogBase . open ( self , text , searchphrase );
        self . ok = true;
        self . insert_tags = insert_tags;
        pub fn create_entries ( self )  {
        "Create base && additional label && text entry widgets.";
        SearchDialogBase . create_entries ( self );
        self . replent = self . make_entry ( "Replace with:" , self . replvar ) [ 0 ];
        pub fn create_command_buttons ( self )  {
        "Create base && additional command buttons.

        The additional buttons are for Find, Replace,
        Replace+Find, && Replace All.
        ";
        SearchDialogBase . create_command_buttons ( self );
        self . make_button ( "Find" , self . find_it );
        self . make_button ( "Replace" , self . replace_it );
        self . make_button ( "Replace+Find" , self . default_command , isdef = true );
        self . make_button ( "Replace All" , self . replace_all );
        pub fn find_it ( &self, event = None /* Option */ )  {
        "Handle the Find button.";
        self . do_find ( false );
        pub fn replace_it ( &self, event = None /* Option */ )  {
        "Handle the Replace button.

        If the find == successful, then perform replace.
        ";
        if self . do_find ( self . ok ) {
        self . do_replace ( );
        pub fn default_command ( &self, event = None /* Option */ )  {
        "Handle the Replace+Find button as the default command.

        First performs a replace && then, if the replace was
        successful, a find next.
        ";
        if self . do_find ( self . ok ) {
        if self . do_replace ( ) {
        self . do_find ( false );
        pub fn _replace_expand ( &self, m , repl )  {
        "Expand replacement text if regular expression.";
        if self . engine . isre ( ) {
        // try {
        new = m . expand ( repl );
        // } catch  re . PatternError  {
        self . engine . report_error ( repl , "Invalid Replace Expression" );
        new = None /* Option */;
        } else {
        new = repl;
        return  new;
        pub fn replace_all ( &self, event = None /* Option */ )  {
        "Handle the Replace All button.

        Search text for occurrences of the Find value && replace
        each of them.  The 'wrap around' value controls the start
        point for searching.  If wrap isn't set, then the searching
        starts at the first occurrence after the current selection;
        if wrap == set, the replacement starts at the first line.
        The replacement == always done top-to-bottom in the text.
        ";
        prog = self . engine . getprog ( );
        if !prog {
        return;
        repl = self . replvar . get ( );
        text = self . text;
        res = self . engine . search_text ( text , prog );
        if !res {
        self . bell ( );
        return;
        text . tag_remove ( "sel" , "1.0" , "end" );
        text . tag_remove ( "hit" , "1.0" , "end" );
        line = res [ 0 ];
        col = res [ 1 ] . start ( );
        if self . engine . iswrap ( ) {
        line = 1;
        col = 0;
        ok = true;
        first = last = None /* Option */;
        text . undo_block_start ( );
        while res : = self . engine . search_forward ( {
        text , prog , line , col , wrap = false , ok = ok ) ;
        line , m = res;
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        orig = m . group ( );
        new = self . _replace_expand ( m , repl );
        if new is None /* Option */ {
        break;
        i , j = m . span ( );
        first = "%d.%d" % ( line , i );
        last = "%d.%d" % ( line , j );
        if new == orig {
        text . mark_set ( "insert" , last );
        } else {
        text . mark_set ( "insert" , first );
        if first != last {
        text . delete ( first , last );
        if new {
        text . insert ( first , new , self . insert_tags );
        col = i + len ( new );
        ok = false;
        text . undo_block_stop ( );
        if first && last {
        self . show_hit ( first , last );
        self . close ( );
        pub fn do_find ( &self, ok = false )  {
        "Search for && highlight next occurrence of pattern in text.

        No text replacement == done with this option.
        ";
        if !self . engine . getprog ( ) {
        return  false;
        text = self . text;
        res = self . engine . search_text ( text , None /* Option */ , ok );
        if !res {
        self . bell ( );
        return  false;
        line , m = res;
        i , j = m . span ( );
        first = "%d.%d" % ( line , i );
        last = "%d.%d" % ( line , j );
        self . show_hit ( first , last );
        self . ok = true;
        return  true;
        pub fn do_replace ( self )  {
        "Replace search pattern in text with replacement value.";
        prog = self . engine . getprog ( );
        if !prog {
        return  false;
        text = self . text;
        // try {
        first = pos = text . index ( "sel.first" );
        last = text . index ( "sel.last" );
        // } catch  TclError  {
        pos = None /* Option */;
        if !pos {
        first = last = pos = text . index ( "insert" );
        line , col = searchengine . get_line_col ( pos );
        chars = text . get ( "%d.0" % line , "%d.0" % ( line + 1 ) );
        m = prog . match ( chars , col );
        if !prog {
        return  false;
        new = self . _replace_expand ( m , self . replvar . get ( ) );
        if new is None /* Option */ {
        return  false;
        text . mark_set ( "insert" , first );
        text . undo_block_start ( );
        if m . group ( ) {
        text . delete ( first , last );
        if new {
        text . insert ( first , new , self . insert_tags );
        text . undo_block_stop ( );
        self . show_hit ( first , text . index ( "insert" ) );
        self . ok = false;
        return  true;
        pub fn show_hit ( &self, first , last )  {
        "Highlight text between first && last indices.

        Text == highlighted via the 'hit' tag && the marked
        section == brought into view.

        The colors from the 'hit' tag aren't currently shown
        when the text == displayed.  This == due to the 'sel'
        tag being added first, so the colors in the 'sel'
        config are seen instead of the colors for 'hit'.
        ";
        text = self . text;
        text . mark_set ( "insert" , first );
        text . tag_remove ( "sel" , "1.0" , "end" );
        text . tag_add ( "sel" , first , last );
        text . tag_remove ( "hit" , "1.0" , "end" );
        if first == last {
        text . tag_add ( "hit" , first );
        } else {
        text . tag_add ( "hit" , first , last );
        text . see ( "insert" );
        text . update_idletasks ( );
        pub fn close ( &self, event = None /* Option */ )  {
        "Close the dialog && remove hit tags.";
        SearchDialogBase . close ( self , event );
        self . text . tag_remove ( "hit" , "1.0" , "end" );
        self . insert_tags = None /* Option */;
        pub fn _replace_dialog ( parent )  {
        from tkinter import Toplevel , Text , END , SEL;
        from tkinter . ttk import Frame , Button;
        top = Toplevel ( parent );
        top . title ( "Test ReplaceDialog" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        pub fn undo_block_start ( )  {
        // pass
        pub fn undo_block_stop ( )  {
        // pass
        frame = Frame ( top );
        frame . pack ( );
        text = Text ( frame , inactiveselectbackground = "gray" );
        text . undo_block_start = undo_block_start;
        text . undo_block_stop = undo_block_stop;
        text . pack ( );
        text . insert ( "insert" , "This == a sample sTring\nPlus MORE." );
        text . focus_set ( );
        pub fn show_replace ( )  {
        text . tag_add ( SEL , "1.0" , END );
        replace ( text );
        text . tag_remove ( SEL , "1.0" , END );
        button = Button ( frame , text = "Replace" , command = show_replace );
        button . pack ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_replace" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _replace_dialog );
}

