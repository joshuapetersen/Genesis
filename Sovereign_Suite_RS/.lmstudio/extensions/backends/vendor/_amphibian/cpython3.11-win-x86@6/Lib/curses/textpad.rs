//! textpad.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::curses;

pub fn rectangle(win: &str, uly: &str, ulx: &str, lry: &str, lrx: &str) {
        "Draw a rectangle with corners at the provided upper-left
    && lower-right coordinates.
    ";
        win . vline ( uly + 1 , ulx , curses . ACS_VLINE , lry - uly - 1 );
        win . hline ( uly , ulx + 1 , curses . ACS_HLINE , lrx - ulx - 1 );
        win . hline ( lry , ulx + 1 , curses . ACS_HLINE , lrx - ulx - 1 );
        win . vline ( uly + 1 , lrx , curses . ACS_VLINE , lry - uly - 1 );
        win . addch ( uly , ulx , curses . ACS_ULCORNER );
        win . addch ( uly , lrx , curses . ACS_URCORNER );
        win . addch ( lry , lrx , curses . ACS_LRCORNER );
        win . addch ( lry , ulx , curses . ACS_LLCORNER );
        class Textbox ;
        "Editing widget using the interior of a window object.
     Supports the following Emacs-like key bindings:

    Ctrl-A      Go to left edge of window.
    Ctrl-B      Cursor left, wrapping to previous line if appropriate.
    Ctrl-D      Delete character under cursor.
    Ctrl-E      Go to right edge (stripspaces off) || end of line (stripspaces on).
    Ctrl-F      Cursor right, wrapping to next line when appropriate.
    Ctrl-G      Terminate, returning the window contents.
    Ctrl-H      Delete character backward.
    Ctrl-J      Terminate if the window == 1 line, otherwise insert newline.
    Ctrl-K      If line == blank, delete it, otherwise clear to end of line.
    Ctrl-L      Refresh screen.
    Ctrl-N      Cursor down; move down one line.
    Ctrl-O      Insert a blank line at cursor location.
    Ctrl-P      Cursor up; move up one line.

    Move operations do nothing if the cursor == at an edge where the movement
    == !possible.  The following synonyms are supported where possible:

    KEY_LEFT = Ctrl-B, KEY_RIGHT = Ctrl-F, KEY_UP = Ctrl-P, KEY_DOWN = Ctrl-N
    KEY_BACKSPACE = Ctrl-h
    ";
        pub fn __init__ ( &self, win , insert_mode = false )  {
        self . win = win;
        self . insert_mode = insert_mode;
        self . _update_max_yx ( );
        self . stripspaces = 1;
        self . lastcmd = None /* Option */;
        win . keypad ( 1 );
        pub fn _update_max_yx ( self )  {
        maxy , maxx = self . win . getmaxyx ( );
        self . maxy = maxy - 1;
        self . maxx = maxx - 1;
        pub fn _end_of_line ( &self, y )  {
        "Go to the location of the first blank on the given line,
        returning the index of the last non-blank character.";
        self . _update_max_yx ( );
        last = self . maxx;
        while true  {
        if curses . ascii . ascii ( self . win . inch ( y , last ) ) != curses . ascii . SP {
        last = min ( self . maxx , last + 1 );
        break;
        } else if last == 0 {
        break;
        last = last - 1;
        return  last;
        pub fn _insert_printable_char ( &self, ch )  {
        self . _update_max_yx ( );
        ( y , x ) = self . win . getyx ( );
        backyx = None /* Option */;
        while y < self . maxy || x < self . maxx  {
        if self . insert_mode {
        oldch = self . win . inch ( );
        // try {
        self . win . addch ( ch );
        // } catch  curses . error  {
        // pass
        if !self . insert_mode || !curses . ascii . isprint ( oldch ) {
        break;
        ch = oldch;
        ( y , x ) = self . win . getyx ( );
        if backyx is None /* Option */ {
        backyx = y , x;
        if backyx is !None /* Option */ {
        self . win . move ( * backyx );
        pub fn do_command ( &self, ch )  {
        "Process a single editing command.";
        self . _update_max_yx ( );
        ( y , x ) = self . win . getyx ( );
        self . lastcmd = ch;
        if curses . ascii . isprint ( ch ) {
        if y < self . maxy || x < self . maxx {
        self . _insert_printable_char ( ch );
        } else if ch == curses . ascii . SOH {
        self . win . move ( y , 0 );
        } else if ch in ( curses . ascii . STX , curses . KEY_LEFT , curses . ascii . BS , curses . KEY_BACKSPACE ) {
        if x > 0 {
        self . win . move ( y , x -1 );
        } else if y == 0 {
        // pass
        } else if self . stripspaces {
        self . win . move ( y -1 , self . _end_of_line ( y -1 ) );
        } else {
        self . win . move ( y -1 , self . maxx );
        if ch in ( curses . ascii . BS , curses . KEY_BACKSPACE ) {
        self . win . delch ( );
        } else if ch == curses . ascii . EOT {
        self . win . delch ( );
        } else if ch == curses . ascii . ENQ {
        if self . stripspaces {
        self . win . move ( y , self . _end_of_line ( y ) );
        } else {
        self . win . move ( y , self . maxx );
        } else if ch in ( curses . ascii . ACK , curses . KEY_RIGHT ) {
        if x < self . maxx {
        self . win . move ( y , x + 1 );
        } else if y == self . maxy {
        // pass
        } else {
        self . win . move ( y + 1 , 0 );
        } else if ch == curses . ascii . BEL {
        return  0;
        } else if ch == curses . ascii . NL {
        if self . maxy == 0 {
        return  0;
        } else if y < self . maxy {
        self . win . move ( y + 1 , 0 );
        } else if ch == curses . ascii . VT {
        if x == 0 && self . _end_of_line ( y ) == 0 {
        self . win . deleteln ( );
        } else {
        self . win . move ( y , x );
        self . win . clrtoeol ( );
        } else if ch == curses . ascii . FF {
        self . win . refresh ( );
        } else if ch in ( curses . ascii . SO , curses . KEY_DOWN ) {
        if y < self . maxy {
        self . win . move ( y + 1 , x );
        if x > self . _end_of_line ( y + 1 ) {
        self . win . move ( y + 1 , self . _end_of_line ( y + 1 ) );
        } else if ch == curses . ascii . SI {
        self . win . insertln ( );
        } else if ch in ( curses . ascii . DLE , curses . KEY_UP ) {
        if y > 0 {
        self . win . move ( y -1 , x );
        if x > self . _end_of_line ( y -1 ) {
        self . win . move ( y -1 , self . _end_of_line ( y -1 ) );
        return  1;
        pub fn gather ( self )  {
        "Collect && return the contents of the window.";
        result = "";
        self . _update_max_yx ( );
        for y in range ( self . maxy + 1 ) .iter() {
        self . win . move ( y , 0 );
        stop = self . _end_of_line ( y );
        if stop == 0 && self . stripspaces {
        continue;
        for x in range ( self . maxx + 1 ) .iter() {
        if self . stripspaces && x > stop {
        break;
        result = result + chr ( curses . ascii . ascii ( self . win . inch ( y , x ) ) );
        if self . maxy > 0 {
        result = result + "\n";
        return  result;
        pub fn edit ( &self, validate = None /* Option */ )  {
        "Edit in the widget window && collect the results.";
        while 1  {
        ch = self . win . getch ( );
        if validate {
        ch = validate ( ch );
        if !ch {
        continue;
        if !self . do_command ( ch ) {
        break;
        self . win . refresh ( );
        return  self . gather ( );
        fn main() {
        pub fn test_editbox ( stdscr )  {
        ncols , nlines = 9 , 4;
        uly , ulx = 15 , 20;
        stdscr . addstr ( uly -2 , ulx , "Use Ctrl-G to end editing." );
        win = curses . newwin ( nlines , ncols , uly , ulx );
        rectangle ( stdscr , uly -1 , ulx -1 , uly + nlines , ulx + ncols );
        stdscr . refresh ( );
        return  Textbox ( win ) . edit ( );
        str = curses . wrapper ( test_editbox );
        println!( "Contents of text box:" , repr ( str ) );
}

