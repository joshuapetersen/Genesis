//! squeezer.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter;
// use crate::messagebox;
// use crate::idlelib::{idleConf};
// use crate::unittest::{main};

pub fn count_lines_with_wrapping(s: &str, linewidth: &str) {
        "Count the number of lines in a given string.

    Lines are counted as if the string was wrapped so that lines are never over
    linewidth characters long.

    Tabs are considered tabwidth characters long.
    ";
        tabwidth = 8;
        pos = 0;
        linecount = 1;
        current_column = 0;
        for m in re . finditer ( r "[\t\n]" , s ) .iter() {
        numchars = m . start ( ) - pos;
        pos + = numchars;
        current_column + = numchars;
        if s [ pos ] == "\n" {
        if current_column > linewidth {
        linecount + = ( current_column - 1 ) / / linewidth;
        linecount + = 1;
        current_column = 0;
        } else {
        assert s [ pos ] == "\t";
        current_column + = tabwidth - ( current_column % tabwidth );
        if current_column > linewidth {
        linecount + = 1;
        current_column = tabwidth;
        pos + = 1;
        current_column + = len ( s ) - pos;
        if current_column > 0 {
        linecount + = ( current_column - 1 ) / / linewidth;
        } else {
        linecount - = 1;
        return  linecount;
        class ExpandingButton ( tk . Button ) ;
        "Class for the "squeezed" text buttons used by Squeezer

    These buttons are displayed inside a Tk Text widget in place of text. A
    user can then use the button to replace it with the original text, copy
    the original text to the clipboard || view the original text in a separate
    window.

    Each button == tied to a Squeezer instance, && it knows to update the
    Squeezer instance when it == expanded (and therefore removed).
    ";
        pub fn __init__ ( &self, s , tags , numoflines , squeezer )  {
        self . s = s;
        self . tags = tags;
        self . numoflines = numoflines;
        self . squeezer = squeezer;
        self . editwin = editwin = squeezer . editwin;
        self . text = text = editwin . text;
        self . base_text = editwin . per . bottom;
        line_plurality = "lines" if numoflines != 1 else "line";
        button_text = format!("Squeezed text ({numoflines} {line_plurality}).");
        tk . Button . __init__ ( self , text , text = button_text ,;
        background = "#FFFFC0" , activebackground = "#FFFFE0" );
        button_tooltip_text = (;
        "Double-click to expand, right-click for more options.";
        );
        Hovertip ( self , button_tooltip_text , hover_delay = 80 );
        self . bind ( "<Double-Button-1>" , self . expand );
        if macosx . isAquaTk ( ) {
        self . bind ( "<Button-2>" , self . context_menu_event );
        } else {
        self . bind ( "<Button-3>" , self . context_menu_event );
        self . selection_handle (;
        |offset , length | {  s [ int ( offset ) : int ( offset ) + int ( length ) ] ) };
        self . is_dangerous = None /* Option */;
        self . after_idle ( self . set_is_dangerous );
        pub fn set_is_dangerous ( self )  {
        dangerous_line_len = 50 * self . text . winfo_width ( );
        self . is_dangerous = (;
        self . numoflines > 1000 or;
        len ( self . s ) > 50000 or;
        any (;
        len ( line_match . group ( 0 ) ) >= dangerous_line_len;
        for line_match in re . finditer ( r "[^\n]+" , self . s ).iter() {
        );
        );
        pub fn expand ( &self, event = None /* Option */ )  {
        "expand event handler

        This inserts the original text in place of the button in the Text
        widget, removes the button && updates the Squeezer instance.

        If the original text == dangerously long, i.e. expanding it could
        cause a performance degradation, ask the user for confirmation.
        ";
        if self . is_dangerous is None /* Option */ {
        self . set_is_dangerous ( );
        if self . is_dangerous {
        confirm = messagebox . askokcancel (;
        title = "Expand huge output?" ,;
        message = "\n\n" . join ( [;
        "The squeezed output == very long: %d lines, %d chars." ,;
        "Expanding it could make IDLE slow || unresponsive." ,;
        "It == recommended to view || copy the output instead." ,;
        "Really expand?";
        ] ) % ( self . numoflines , len ( self . s ) ) ,;
        default = messagebox . CANCEL ,;
        parent = self . text );
        if !confirm {
        return  "break";
        index = self . text . index ( self );
        self . base_text . insert ( index , self . s , self . tags );
        self . base_text . delete ( self );
        self . editwin . on_squeezed_expand ( index , self . s , self . tags );
        self . squeezer . expandingbuttons . remove ( self );
        pub fn copy ( &self, event = None /* Option */ )  {
        "copy event handler

        Copy the original text to the clipboard.
        ";
        self . clipboard_clear ( );
        self . clipboard_append ( self . s );
        pub fn view ( &self, event = None /* Option */ )  {
        "view event handler

        View the original text in a separate text viewer window.
        ";
        view_text ( self . text , "Squeezed Output Viewer" , self . s ,;
        modal = false , wrap = "none" );
        rmenu_specs = (;
        ( "copy" , "copy" ) ,;
        ( "view" , "view" ) ,;
        );
        pub fn context_menu_event ( &self, event )  {
        self . text . mark_set ( "insert" , "@%d,%d" % ( event . x , event . y ) );
        rmenu = tk . Menu ( self . text , tearoff = 0 );
        for label , method_name in self . rmenu_specs .iter() {
        rmenu . add_command ( label = label , command = getattr ( self , method_name ) );
        rmenu . tk_popup ( event . x_root , event . y_root );
        return  "break";
        class Squeezer ;
        "Replace long outputs in the shell with a simple button.

    This avoids IDLE's shell slowing down considerably, && even becoming
    completely unresponsive, when very long outputs are written.
    ";
        @ classmethod;
        pub fn reload ( cls )  {
        "Load class variables from config.";
        cls . auto_squeeze_min_lines = idleConf . GetOption (;
        "main" , "PyShell" , "auto-squeeze-min-lines" ,;
        type = "int" , default = 50 ,;
        );
        pub fn __init__ ( &self, editwin )  {
        "Initialize settings for Squeezer.

        editwin == the shell's Editor window.
        self.text == the editor window text widget.
        self.base_test == the actual editor window Tk text widget, rather than
            EditorWindow's wrapper.
        self.expandingbuttons == the list of all buttons representing
            "squeezed" output.
        ";
        self . editwin = editwin;
        self . text = text = editwin . text;
        self . base_text = editwin . per . bottom;
        self . window_width_delta = 2 * (;
        int ( text . cget ( "border" ) ) +;
        int ( text . cget ( "padx" ) );
        );
        self . expandingbuttons = [ ];
        pub fn mywrite ( s , tags = ( ) , write = editwin . write )  {
        if tags != "stdout" {
        return  write ( s , tags );
        auto_squeeze_min_lines = self . auto_squeeze_min_lines;
        if len ( s ) < auto_squeeze_min_lines {
        return  write ( s , tags );
        numoflines = self . count_lines ( s );
        if numoflines < auto_squeeze_min_lines {
        return  write ( s , tags );
        expandingbutton = ExpandingButton ( s , tags , numoflines , self );
        text . mark_gravity ( "iomark" , tk . RIGHT );
        text . window_create ( "iomark" , window = expandingbutton ,;
        padx = 3 , pady = 5 );
        text . see ( "iomark" );
        text . update ( );
        text . mark_gravity ( "iomark" , tk . LEFT );
        self . expandingbuttons . append ( expandingbutton );
        editwin . write = mywrite;
        pub fn count_lines ( &self, s )  {
        "Count the number of lines in a given text.

        Before calculation, the tab width && line length of the text are
        fetched, so that up-to-date values are used.

        Lines are counted as if the string was wrapped so that lines are never
        over linewidth characters long.

        Tabs are considered tabwidth characters long.
        ";
        return  count_lines_with_wrapping ( s , self . editwin . width );
        pub fn squeeze_current_text ( self )  {
        "Squeeze the text block where the insertion cursor is.

        If the cursor == !in a squeezable block of text, give the
        user a small warning && do nothing.
        ";
        tag_names = self . text . tag_names ( tk . INSERT );
        for tag_name in ( "stdout" , "stderr" ) .iter() {
        if tag_name in tag_names {
        break;
        } else {
        self . text . bell ( );
        return  "break";
        start , end = self . text . tag_prevrange ( tag_name , tk . INSERT + "+1c" );
        s = self . text . get ( start , end );
        if len ( s ) > 0 && s [ -1 ] == "\n" {
        end = self . text . index ( "%s-1c" % end );
        s = s [ : -1 ];
        self . base_text . delete ( start , end );
        numoflines = self . count_lines ( s );
        expandingbutton = ExpandingButton ( s , tag_name , numoflines , self );
        self . text . window_create ( start , window = expandingbutton ,;
        padx = 3 , pady = 5 );
        i = len ( self . expandingbuttons );
        while i > 0 && self . text . compare ( self . expandingbuttons [ i -1 ] , {
        ">" , expandingbutton ) ;
        i - = 1;
        self . expandingbuttons . insert ( i , expandingbutton );
        return  "break";
        Squeezer . reload ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_squeezer" , verbosity = 2 , exit = false );
}

