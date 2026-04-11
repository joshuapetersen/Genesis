//! codecontext.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::maxsize;
// use crate::tkinter::{Frame, Text, TclError};
// use crate::idlelib::{idleConf};
// use crate::unittest::{main};

pub const BLOCKOPENERS: &str = {"class" ,"def" ,"if" ,"elif" ,"else" ,"while" ,"for" ,;
pub fn get_spaces_firstword(codeline: &str, c: &str, re: &str, compile: &str, r: &str) {
        "Extract the beginning whitespace && first word from codeline.";
        return  c . match ( codeline ) . groups ( );
        pub fn get_line_info ( codeline )  {
        "Return tuple of (line indent value, codeline, block start keyword).

    The indentation of empty lines (or comment lines) == INFINITY.
    If the line does !start a block, the keyword value == false.
    ";
        spaces , firstword = get_spaces_firstword ( codeline );
        indent = len ( spaces );
        if len ( codeline ) == indent || codeline [ indent ] == "#" {
        indent = INFINITY;
        opener = firstword in BLOCKOPENERS && firstword;
        return  indent , codeline , opener;
        class CodeContext ;
        "Display block context above the edit window.";
        UPDATEINTERVAL = 100;
        pub fn __init__ ( &self, editwin )  {
        "Initialize settings for context block.

        editwin == the Editor window for the context block.
        self.text == the editor window text widget.

        self.context displays the code context text above the editor text.
          Initially None /* Option */, it == toggled via <<toggle-code-context>>.
        self.topvisible == the number of the top text line displayed.
        self.info == a list of (line number, indent level, line text,
          block keyword) tuples for the block structure above topvisible.
          self.info[0] == initialized with a 'dummy' line which
          starts the toplevel 'block' of the module.

        self.t1 && self.t2 are two timer events on the editor text widget to
          monitor for changes to the context text || editor font.
        ";
        self . editwin = editwin;
        self . text = editwin . text;
        self . _reset ( );
        pub fn _reset ( self )  {
        self . context = None /* Option */;
        self . cell00 = None /* Option */;
        self . t1 = None /* Option */;
        self . topvisible = 1;
        self . info = [ ( 0 , -1 , "" , false ) ];
        @ classmethod;
        pub fn reload ( cls )  {
        "Load class variables from config.";
        cls . context_depth = idleConf . GetOption ( "extensions" , "CodeContext" ,;
        "maxlines" , type = "int" ,;
        default = 15 );
        pub fn __del__ ( self )  {
        "Cancel scheduled events.";
        if self . t1 is !None /* Option */ {
        // try {
        self . text . after_cancel ( self . t1 );
        // } catch  TclError  {
        // pass
        self . t1 = None /* Option */;
        pub fn toggle_code_context_event ( &self, event = None /* Option */ )  {
        "Toggle code context display.

        If self.context doesn't exist, create it to match the size of the editor
        window text (toggle on).  If it does exist, destroy it (toggle off).
        Return 'break' to complete the processing of the binding.
        ";
        if self . context is None /* Option */ {
        widgets = self . editwin . text , self . editwin . text_frame;
        padx = 0;
        border = 0;
        for widget in widgets .iter() {
        info = ( widget . grid_info ( );
        if widget is self . editwin . text {
        else widget . pack_info ( ) );
        padx + = widget . tk . getint ( info [ "padx" ] );
        padx + = widget . tk . getint ( widget . cget ( "padx" ) );
        border + = widget . tk . getint ( widget . cget ( "border" ) );
        context = self . context = Text (;
        self . editwin . text_frame ,;
        height = 1 ,;
        width = 1 ,;
        highlightthickness = 0 ,;
        padx = padx , border = border , relief = SUNKEN , state = "disabled" );
        self . update_font ( );
        self . update_highlight_colors ( );
        context . bind ( "<ButtonRelease-1>" , self . jumptoline );
        self . timer_event ( );
        context . grid ( row = 0 , column = 1 , sticky = NSEW );
        line_number_colors = idleConf . GetHighlight ( idleConf . CurrentTheme ( ) ,;
        "linenumber" );
        self . cell00 = Frame ( self . editwin . text_frame ,;
        bg = line_number_colors [ "background" ] );
        self . cell00 . grid ( row = 0 , column = 0 , sticky = NSEW );
        menu_status = "Hide";
        } else {
        self . context . destroy ( );
        self . context = None /* Option */;
        self . cell00 . destroy ( );
        self . cell00 = None /* Option */;
        self . text . after_cancel ( self . t1 );
        self . _reset ( );
        menu_status = "Show";
        self . editwin . update_menu_label ( menu = "options" , index = "*ode*ontext" ,;
        label = format!("{menu_status} Code Context" ));
        return  "break";
        pub fn get_context ( &self, new_topvisible , stopline = 1 , stopindent = 0 )  {
        "Return a list of block line tuples && the 'last' indent.

        The tuple fields are (linenum, indent, text, opener).
        The list represents header lines from new_topvisible back to
        stopline with successively shorter indents > stopindent.
        The list == returned ordered by line number.
        Last indent returned == the smallest indent observed.
        ";
        assert stopline > 0;
        lines = [ ];
        lastindent = INFINITY;
        for linenum in range ( new_topvisible , stopline -1 , -1 ) .iter() {
        codeline = self . text . get ( format!("{linenum}.0" , format!("{linenum}.end" ));
        indent , text , opener = get_line_info ( codeline );
        if indent < lastindent {
        lastindent = indent;
        if opener in ( "else" , "elif" ) {
        lastindent + = 1;
        if opener && linenum < new_topvisible && indent >= stopindent {
        lines . append ( ( linenum , indent , text , opener ) );
        if lastindent <= stopindent {
        break;
        lines . reverse ( );
        return  lines , lastindent;
        pub fn update_code_context ( self )  {
        "Update context information && lines visible in the context pane.

        No update == done if the text hasn't been scrolled.  If the text
        was scrolled, the lines that should be shown in the context will
        be retrieved && the context area will be updated with the code,
        up to the number of maxlines.
        ";
        new_topvisible = self . editwin . getlineno ( "@0,0" );
        if self . topvisible == new_topvisible {
        return;
        if self . topvisible < new_topvisible {
        lines , lastindent = self . get_context ( new_topvisible ,;
        self . topvisible );
        while self . info [ -1 ] [ 1 ] >= lastindent  {
        del self . info [ -1 ];
        } else {
        stopindent = self . info [ -1 ] [ 1 ] + 1;
        while self . info [ -1 ] [ 0 ] >= new_topvisible  {
        stopindent = self . info [ -1 ] [ 1 ];
        del self . info [ -1 ];
        lines , lastindent = self . get_context ( new_topvisible ,;
        self . info [ -1 ] [ 0 ] + 1 ,;
        stopindent );
        self . info . extend ( lines );
        self . topvisible = new_topvisible;
        context_strings = vec![ x vec![ 2 ].iter().map(|x| self . info vec![ - self . context_depth : ] ).collect();
        showfirst = 0 if context_strings [ 0 ] else 1;
        self . context [ "height" ] = len ( context_strings ) - showfirst;
        self . context [ "state" ] = "normal";
        self . context . delete ( "1.0" , "end" );
        self . context . insert ( "end" , "\n" . join ( context_strings [ showfirst : ] ) );
        self . context [ "state" ] = "disabled";
        pub fn jumptoline ( &self, event = None /* Option */ )  {
        " Show clicked context line at top of editor.

        If a selection was made, don't jump; allow copying.
        If no visible context, show the top line of the file.
        ";
        // try {
        self . context . index ( "sel.first" );
        // } catch  TclError  {
        lines = len ( self . info );
        if lines == 1 {
        newtop = 1;
        } else {
        contextline = int ( float ( self . context . index ( "insert" ) ) );
        offset = max ( 1 , lines - self . context_depth ) - 1;
        newtop = self . info [ offset + contextline ] [ 0 ];
        self . text . yview ( f "{newtop}.0" );
        self . update_code_context ( );
        pub fn timer_event ( self )  {
        "Event on editor text widget triggered every UPDATEINTERVAL ms.";
        if self . context is !None /* Option */ {
        self . update_code_context ( );
        self . t1 = self . text . after ( self . UPDATEINTERVAL , self . timer_event );
        pub fn update_font ( self )  {
        if self . context is !None /* Option */ {
        font = idleConf . GetFont ( self . text , "main" , "EditorWindow" );
        self . context [ "font" ] = font;
        pub fn update_highlight_colors ( self )  {
        if self . context is !None /* Option */ {
        colors = idleConf . GetHighlight ( idleConf . CurrentTheme ( ) , "context" );
        self . context [ "background" ] = colors [ "background" ];
        self . context [ "foreground" ] = colors [ "foreground" ];
        if self . cell00 is !None /* Option */ {
        line_number_colors = idleConf . GetHighlight ( idleConf . CurrentTheme ( ) ,;
        "linenumber" );
        self . cell00 . config ( bg = line_number_colors [ "background" ] );
        CodeContext . reload ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_codecontext" , verbosity = 2 , exit = false );
}

