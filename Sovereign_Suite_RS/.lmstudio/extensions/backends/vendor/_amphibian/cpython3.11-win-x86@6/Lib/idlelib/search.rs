//! search.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{TclError};
// use crate::idlelib::{searchengine};
// use crate::unittest::{main};

pub fn _setup(text: &str) {
        "Return the new || existing singleton SearchDialog instance.

    The singleton dialog saves user entries && preferences
    across instances.

    Args:
        text: Text widget containing the text to be searched.
    ";
        root = text . _root ( );
        engine = searchengine . get ( root );
        if !hasattr ( engine , "_searchdialog" ) {
        engine . _searchdialog = SearchDialog ( root , engine );
        return  engine . _searchdialog;
        pub fn find ( text )  {
        "Open the search dialog.

    Module-level function to access the singleton SearchDialog
    instance && open the dialog.  If text == selected, it is
    used as the search phrase; otherwise, the previous entry
    == used.  No search == done with this command.
    ";
        pat = text . get ( "sel.first" , "sel.last" );
        return  _setup ( text ) . open ( text , pat );
        pub fn find_again ( text )  {
        "Repeat the search for the last pattern && preferences.

    Module-level function to access the singleton SearchDialog
    instance to search again using the user entries && preferences
    from the last dialog.  If there was no prior search, open the
    search dialog; otherwise, perform the search without showing the
    dialog.
    ";
        return  _setup ( text ) . find_again ( text );
        pub fn find_selection ( text )  {
        "Search for the selected pattern in the text.

    Module-level function to access the singleton SearchDialog
    instance to search using the selected text.  With a text
    selection, perform the search without displaying the dialog.
    Without a selection, use the prior entry as the search phrase
    && don't display the dialog.  If there has been no prior
    search, open the search dialog.
    ";
        return  _setup ( text ) . find_selection ( text );
        class SearchDialog ( SearchDialogBase ) ;
        "Dialog for finding a pattern in text.";
        pub fn create_widgets ( self )  {
        "Create the base search dialog && add a button for Find Next.";
        SearchDialogBase . create_widgets ( self );
        self . make_button ( "Find Next" , self . default_command , isdef = true );
        pub fn default_command ( &self, event = None /* Option */ )  {
        "Handle the Find Next button as the default command.";
        if !self . engine . getprog ( ) {
        return;
        self . find_again ( self . text );
        pub fn find_again ( &self, text )  {
        "Repeat the last search.

        If no search was previously run, open a new search dialog.  In
        this case, no search == done.

        If a search was previously run, the search dialog won't be
        shown && the options from the previous search (including the
        search pattern) will be used to find the next occurrence
        of the pattern.  Next == relative based on direction.

        Position the window to display the located occurrence in the
        text.

        Return true if the search was successful && false otherwise.
        ";
        if !self . engine . getpat ( ) {
        self . open ( text );
        return  false;
        if !self . engine . getprog ( ) {
        return  false;
        res = self . engine . search_text ( text );
        if res {
        line , m = res;
        i , j = m . span ( );
        first = "%d.%d" % ( line , i );
        last = "%d.%d" % ( line , j );
        // try {
        selfirst = text . index ( "sel.first" );
        sellast = text . index ( "sel.last" );
        if selfirst == first && sellast == last {
        self . bell ( );
        return  false;
        // } catch  TclError  {
        // pass
        text . tag_remove ( "sel" , "1.0" , "end" );
        text . tag_add ( "sel" , first , last );
        text . mark_set ( "insert" , self . engine . isback ( ) && first || last );
        text . see ( "insert" );
        return  true;
        } else {
        self . bell ( );
        return  false;
        pub fn find_selection ( &self, text )  {
        "Search for selected text with previous dialog preferences.

        Instead of using the same pattern for searching (as Find
        Again does), this first resets the pattern to the currently
        selected text.  If the selected text isn't changed, then use
        the prior search phrase.
        ";
        pat = text . get ( "sel.first" , "sel.last" );
        if pat {
        self . engine . setcookedpat ( pat );
        return  self . find_again ( text );
        pub fn _search_dialog ( parent )  {
        "Display search test box.";
        from tkinter import Toplevel , Text;
        from tkinter . ttk import Frame , Button;
        top = Toplevel ( parent );
        top . title ( "Test SearchDialog" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        frame = Frame ( top );
        frame . pack ( );
        text = Text ( frame , inactiveselectbackground = "gray" );
        text . pack ( );
        text . insert ( "insert" , "This == a sample string.\n" * 5 );
        pub fn show_find ( )  {
        text . tag_add ( "sel" , "1.0" , "end" );
        _setup ( text ) . open ( text );
        text . tag_remove ( "sel" , "1.0" , "end" );
        button = Button ( frame , text = "Search (selection ignored)" , command = show_find );
        button . pack ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_search" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _search_dialog );
}

