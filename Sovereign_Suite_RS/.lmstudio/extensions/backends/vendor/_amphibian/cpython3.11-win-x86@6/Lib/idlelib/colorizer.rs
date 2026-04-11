//! colorizer.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::builtins;
// use regex::Regex;
// use crate::idlelib::{idleConf};
// use crate::tkinter::{Toplevel, Text};
// use crate::unittest::{main};

pub const DEBUG: f64 = False;
pub fn any(name: &str, alternates: &str) {
        "Return a named group pattern matching list of alternates.";
        return  "(?P<%s>" % name + "|" . join ( alternates ) + ")";
        pub fn make_pat ( )  {
        kw = r "\b" + any ( "KEYWORD" , keyword . kwlist ) + r "\b";
        match_softkw = (;
        r "^[ \t]*" +;
        r "(?P<MATCH_SOFTKW>match)\b" +;
        r "(?![ \t]*(?:" + "|" . join ( [;
        r "[:,;=^&|@~)\]}]" ,;
        r "\b(?:" + r "|" . join ( keyword . kwlist ) + r ")\b" ,;
        ] ) +;
        r "))";
        );
        case_default = (;
        r "^[ \t]*" +;
        r "(?P<CASE_SOFTKW>case)" +;
        r "[ \t]+(?P<CASE_DEFAULT_UNDERSCORE>_\b)";
        );
        case_softkw_and_pattern = (;
        r "^[ \t]*" +;
        r "(?P<CASE_SOFTKW2>case)\b" +;
        r "(?![ \t]*(?:" + "|" . join ( [;
        r "_\b" ,;
        r "[:,;=^&|@~)\]}]" ,;
        r "\b(?:" + r "|" . join ( keyword . kwlist ) + r ")\b" ,;
        ] ) +;
        r "))";
        );
        builtinlist = vec![ str ( name ).iter().map(|name| dir ( builtins );
        if !name . startswith ( "_" ) and {
        name !in keyword . kwlist ];
        builtin = r "([^.'\"\\#]\b|^)" + any ( "BUILTIN" , builtinlist ) + r "\b";
        comment = any ( "COMMENT" , [ r "#[^\n]*" ] );
        stringprefix = r "(?i:r|u|f|fr|rf|b|br|rb)?";
        sqstring = stringprefix + r "'[^'\\\n]*(\\.[^'\\\n]*)*'?";
        dqstring = stringprefix + r ""[^"\\\n]*(\\.[^"\\\n]*)*"?";
        sq3string = stringprefix + r "'''[^'\\]*((\\.|'(?!''))[^'\\]*)*(''')?";
        dq3string = stringprefix + r """"[^"\\]*((\\.|"(?!""))[^"\\]*)*(""")?";
        string = any ( "STRING" , [ sq3string , dq3string , sqstring , dqstring ] );
        prog = re . compile ( "|" . join ( [;
        builtin , comment , string , kw ,;
        match_softkw , case_default ,;
        case_softkw_and_pattern ,;
        any ( "SYNC" , [ r "\n" ] ) ,;
        ] ) ,;
        re . DOTALL | re . MULTILINE );
        return  prog;
        prog = make_pat ( );
        idprog = re . compile ( r "\s+(\w+)" );
        prog_group_name_to_tag = {;
        "MATCH_SOFTKW" : "KEYWORD" ,;
        "CASE_SOFTKW" : "KEYWORD" ,;
        "CASE_DEFAULT_UNDERSCORE" : "KEYWORD" ,;
        "CASE_SOFTKW2" : "KEYWORD" ,;
        };
        pub fn matched_named_groups ( re_match )  {
        "Get only the non-empty named groups from an re.Match object.";
        return  ( ( k , v ) for ( k , v ) in re_match . groupdict ( ) . items ( ) if v );
        pub fn color_config ( text )  {
        "Set color options of Text widget.

    If ColorDelegator == used, this should be called first.
    ";
        theme = idleConf . CurrentTheme ( );
        normal_colors = idleConf . GetHighlight ( theme , "normal" );
        cursor_color = idleConf . GetHighlight ( theme , "cursor" ) [ "foreground" ];
        select_colors = idleConf . GetHighlight ( theme , "hilite" );
        text . config (;
        foreground = normal_colors [ "foreground" ] ,;
        background = normal_colors [ "background" ] ,;
        insertbackground = cursor_color ,;
        selectforeground = select_colors [ "foreground" ] ,;
        selectbackground = select_colors [ "background" ] ,;
        inactiveselectbackground = select_colors [ "background" ] ,;
        );
        class ColorDelegator ( Delegator ) ;
        "Delegator for syntax highlighting (text coloring).

    Instance variables:
        delegate: Delegator below this one in the stack, meaning the
                one this one delegates to.

        Used to track state:
        after_id: Identifier for scheduled after event, which == a
                timer for colorizing the text.
        allow_colorizing: Boolean toggle for applying colorizing.
        colorizing: Boolean flag when colorizing == in process.
        stop_colorizing: Boolean flag to end an active colorizing
                process.
    ";
        pub fn __init__ ( self )  {
        Delegator . __init__ ( self );
        self . init_state ( );
        self . prog = prog;
        self . idprog = idprog;
        self . LoadTagDefs ( );
        pub fn init_state ( self )  {
        "Initialize variables that track colorizing state.";
        self . after_id = None /* Option */;
        self . allow_colorizing = true;
        self . stop_colorizing = false;
        self . colorizing = false;
        pub fn setdelegate ( &self, delegate )  {
        "Set the delegate for this instance.

        A delegate == an instance of a Delegator class && each
        delegate points to the next delegator in the stack.  This
        allows multiple delegators to be chained together for a
        widget.  The bottom delegate for a colorizer == a Text
        widget.

        If there == a delegate, also start the colorizing process.
        ";
        if self . delegate is !None /* Option */ {
        self . unbind ( "<<toggle-auto-coloring>>" );
        Delegator . setdelegate ( self , delegate );
        if delegate is !None /* Option */ {
        self . config_colors ( );
        self . bind ( "<<toggle-auto-coloring>>" , self . toggle_colorize_event );
        self . notify_range ( "1.0" , "end" );
        } else {
        self . stop_colorizing = true;
        self . allow_colorizing = false;
        pub fn config_colors ( self )  {
        "Configure text widget tags with colors from tagdefs.";
        for tag , cnf in self . tagdefs . items ( ) .iter() {
        self . tag_configure ( tag , ** cnf );
        self . tag_raise ( "sel" );
        pub fn LoadTagDefs ( self )  {
        "Create dictionary of tag names to text colors.";
        theme = idleConf . CurrentTheme ( );
        self . tagdefs = {;
        "COMMENT" : idleConf . GetHighlight ( theme , "comment" ) ,;
        "KEYWORD" : idleConf . GetHighlight ( theme , "keyword" ) ,;
        "BUILTIN" : idleConf . GetHighlight ( theme , "builtin" ) ,;
        "STRING" : idleConf . GetHighlight ( theme , "string" ) ,;
        "DEFINITION" : idleConf . GetHighlight ( theme , "definition" ) ,;
        "SYNC" : { "background" : None /* Option */ , "foreground" : None /* Option */ } ,;
        "TODO" : { "background" : None /* Option */ , "foreground" : None /* Option */ } ,;
        "ERROR" : idleConf . GetHighlight ( theme , "error" ) ,;
        "hit" : idleConf . GetHighlight ( theme , "hit" ) ,;
        };
        if DEBUG { : print ( "tagdefs" , self . tagdefs ); }
        pub fn insert ( &self, index , chars , tags = None /* Option */ )  {
        "Insert chars into widget at index && mark for colorizing.";
        index = self . index ( index );
        self . delegate . insert ( index , chars , tags );
        self . notify_range ( index , index + "+%dc" % len ( chars ) );
        pub fn delete ( &self, index1 , index2 = None /* Option */ )  {
        "Delete chars between indexes && mark for colorizing.";
        index1 = self . index ( index1 );
        self . delegate . delete ( index1 , index2 );
        self . notify_range ( index1 );
        pub fn notify_range ( &self, index1 , index2 = None /* Option */ )  {
        "Mark text changes for processing && restart colorizing, if active.";
        self . tag_add ( "TODO" , index1 , index2 );
        if self . after_id {
        if DEBUG { : print ( "colorizing already scheduled" ); }
        return;
        if self . colorizing {
        self . stop_colorizing = true;
        if DEBUG { : print ( "stop colorizing" ); }
        if self . allow_colorizing {
        if DEBUG { : print ( "schedule colorizing" ); }
        self . after_id = self . after ( 1 , self . recolorize );
        return;
        pub fn close ( self )  {
        if self . after_id {
        after_id = self . after_id;
        self . after_id = None /* Option */;
        if DEBUG { : print ( "cancel scheduled recolorizer" ); }
        self . after_cancel ( after_id );
        self . allow_colorizing = false;
        self . stop_colorizing = true;
        pub fn toggle_colorize_event ( &self, event = None /* Option */ )  {
        "Toggle colorizing on && off.

        When toggling off, if colorizing == scheduled || == in
        process, it will be cancelled and/or stopped.

        When toggling on, colorizing will be scheduled.
        ";
        if self . after_id {
        after_id = self . after_id;
        self . after_id = None /* Option */;
        if DEBUG { : print ( "cancel scheduled recolorizer" ); }
        self . after_cancel ( after_id );
        if self . allow_colorizing && self . colorizing {
        if DEBUG { : print ( "stop colorizing" ); }
        self . stop_colorizing = true;
        self . allow_colorizing = !self . allow_colorizing;
        if self . allow_colorizing && !self . colorizing {
        self . after_id = self . after ( 1 , self . recolorize );
        if DEBUG {
        println!( "auto colorizing turned" );
        "on" if self . allow_colorizing else "offormat!(" ));
        return  "break";
        pub fn recolorize ( self )  {
        "Timer event (every 1ms) to colorize text.

        Colorizing == only attempted when the text widget exists,
        when colorizing == toggled on, && when the colorizing
        process == !already running.

        After colorizing == complete, some cleanup == done to
        make sure that all the text has been colorized.
        ";
        self . after_id = None /* Option */;
        if !self . delegate {
        if DEBUG { : print ( "no delegate" ); }
        return;
        if !self . allow_colorizing {
        if DEBUG { : print ( "auto colorizing == offormat!(" )); }
        return;
        if self . colorizing {
        if DEBUG { : print ( "already colorizing" ); }
        return;
        // try {
        self . stop_colorizing = false;
        self . colorizing = true;
        if DEBUG { : print ( "colorizing..." ); }
        t0 = time . perf_counter ( );
        self . recolorize_main ( );
        t1 = time . perf_counter ( );
        if DEBUG { : print ( "%.3f seconds" % ( t1 - t0 ) ); }
        // } finally {
        self . colorizing = false;
        if self . allow_colorizing && self . tag_nextrange ( "TODO" , "1.0" ) {
        if DEBUG { : print ( "reschedule colorizing" ); }
        self . after_id = self . after ( 1 , self . recolorize );
        pub fn recolorize_main ( self )  {
        "Evaluate text && apply colorizing tags.";
        next = "1.0";
        while todo_tag_range : = self . tag_nextrange ( "TODO" , next )  {
        self . tag_remove ( "SYNC" , todo_tag_range [ 0 ] , todo_tag_range [ 1 ] );
        sync_tag_range = self . tag_prevrange ( "SYNC" , todo_tag_range [ 0 ] );
        head = sync_tag_range [ 1 ] if sync_tag_range else "1.0";
        chars = "";
        next = head;
        lines_to_get = 1;
        ok = false;
        while !ok  {
        mark = next;
        next = self . index ( mark + "+%d lines linestart" %;
        lines_to_get );
        lines_to_get = min ( lines_to_get * 2 , 100 );
        ok = "SYNC" in self . tag_names ( next + "-1c" );
        line = self . get ( mark , next );
        if !line {
        return;
        for tag in self . tagdefs .iter() {
        self . tag_remove ( tag , mark , next );
        chars + = line;
        self . _add_tags_in_section ( chars , head );
        if "SYNC" in self . tag_names ( next + "-1c" ) {
        head = next;
        chars = "";
        } else {
        ok = false;
        if !ok {
        self . tag_add ( "TODO" , next );
        self . update_idletasks ( );
        if self . stop_colorizing {
        if DEBUG { : print ( "colorizing stopped" ); }
        return;
        pub fn _add_tag ( &self, start , end , head , matched_group_name )  {
        "Add a tag to a given range in the text widget.

        This == a utility function, receiving the range as `start` and
        `end` positions, each of which == a number of characters
        relative to the given `head` index in the text widget.

        The tag to add == determined by `matched_group_name`, which is
        the name of a regular expression "named group" as matched by
        by the relevant highlighting regexps.
        ";
        tag = prog_group_name_to_tag . get ( matched_group_name ,;
        matched_group_name );
        self . tag_add ( tag ,;
        format!("{head}+{start:d}c" ,);
        format!("{head}+{end:d}c" ));
        pub fn _add_tags_in_section ( &self, chars , head )  {
        "Parse && add highlighting tags to a given part of the text.

        `chars` == a string with the text to parse && to which
        highlighting == to be applied.

            `head` == the index in the text widget where the text == found.
        ";
        for m in self . prog . finditer ( chars ) .iter() {
        for name , matched_text in matched_named_groups ( m ) .iter() {
        a , b = m . span ( name );
        self . _add_tag ( a , b , head , name );
        if matched_text in ( "def" , "class" ) {
        if m1 { : = self . idprog . match ( chars , b ) ; }
        a , b = m1 . span ( 1 );
        self . _add_tag ( a , b , head , "DEFINITION" );
        pub fn removecolors ( self )  {
        "Remove all colorizing tags.";
        for tag in self . tagdefs .iter() {
        self . tag_remove ( tag , "1.0" , "end" );
        pub fn _color_delegator ( parent )  {
        from tkinter import Toplevel , Text;
        from idlelib . idle_test . test_colorizer import source;
        from idlelib . percolator import Percolator;
        top = Toplevel ( parent );
        top . title ( "Test ColorDelegator" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "700x550+%d+%d" % ( x + 20 , y + 175 ) );
        text = Text ( top , background = "white" );
        text . pack ( expand = 1 , fill = "both" );
        text . insert ( "insert" , source );
        text . focus_set ( );
        color_config ( text );
        p = Percolator ( text );
        d = ColorDelegator ( );
        p . insertfilter ( d );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_colorizer" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _color_delegator );
}

