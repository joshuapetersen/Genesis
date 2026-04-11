//! config_key.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{Toplevel, Listbox, StringVar, TclError};
// use crate::string;
// use crate::unittest::{main};
// use crate::idlelib::{run};

pub const FUNCTION_KEYS: &str = ("F1" ,"F2" ,"F3" ,"F4" ,"F5" ,"F6" ,;
pub const ALPHANUM_KEYS: f64 = tuple ( string . ascii_lowercase + string . digits );
pub const PUNCTUATION_KEYS: &str = tuple ("~!@#%^&*()_-+={}[]|;:,.<>/?" );
pub const WHITESPACE_KEYS: &str = ("Tab" ,"Space" ,"Return" );
pub const EDIT_KEYS: &str = ("BackSpace" ,"Delete" ,"Insert" );
pub const MOVE_KEYS: &str = ("Home" ,"End" ,"Page Up" ,"Page Down" ,"Left Arrow" ,;
pub const AVAILABLE_KEYS: f64 = ( ALPHANUM_KEYS + PUNCTUATION_KEYS + FUNCTION_KEYS +;
pub fn translate_key(key: &str, modifiers: &str) {
        "Translate from keycap symbol to the Tkinter keysym.";
        mapping = { "Space" : "space" ,;
        "~" : "asciitilde" , "!" : "exclam" , "@" : "at" , "#" : "numbersign" ,;
        "%" : "percent" , "^" : "asciicircum" , "&" : "ampersand" ,;
        "*" : "asterisk" , "(" : "parenleft" , ")" : "parenright" ,;
        "_" : "underscore" , "-" : "minus" , "+" : "plus" , "=" : "equal" ,;
        "{" : "braceleft" , "}" : "braceright" ,;
        "[" : "bracketleft" , "]" : "bracketright" , "|" : "bar" ,;
        ";" : "semicolon" , ":" : "colon" , "," : "comma" , "." : "period" ,;
        "<" : "less" , ">" : "greater" , "/" : "slash" , "?" : "question" ,;
        "Page Up" : "Prior" , "Page Down" : "Next" ,;
        "Left Arrow" : "Left" , "Right Arrow" : "Right" ,;
        "Up Arrow" : "Up" , "Down Arrow" : "Down" , "Tab" : "Tab" };
        key = mapping . get ( key , key );
        if "Shift" in modifiers && key in string . ascii_lowercase {
        key = key . upper ( );
        return  f "Key-{key}";
        class GetKeysFrame ( Frame ) ;
        keyerror_title = "Key Sequence Error";
        pub fn __init__ ( &self, parent , action , current_key_sequences )  {
        "
        parent - parent of this dialog
        action - the name of the virtual event these keys will be
                 mapped to
        current_key_sequences - a list of all key sequence lists
                 currently mapped to virtual events, for overlap checking
        ";
        super ( ) . __init__ ( parent );
        self [ "borderwidth" ] = 2;
        self [ "relieformat!(" ] = "sunken");
        self . parent = parent;
        self . action = action;
        self . current_key_sequences = current_key_sequences;
        self . result = "";
        self . key_string = StringVar ( self );
        self . key_string . set ( "" );
        self . set_modifiers_for_platform ( );
        self . modifier_vars = [ ];
        for modifier in self . modifiers .iter() {
        variable = StringVar ( self );
        variable . set ( "" );
        self . modifier_vars . append ( variable );
        self . advanced = false;
        self . create_widgets ( );
        pub fn showerror ( &self, * args , ** kwargs )  {
        messagebox . showerror ( * args , ** kwargs );
        pub fn create_widgets ( self )  {
        self . frame_keyseq_basic = Frame ( self , name = "keyseq_basic" );
        self . frame_keyseq_basic . grid ( row = 0 , column = 0 , sticky = "nsew" ,;
        padx = 5 , pady = 5 );
        basic_title = Label ( self . frame_keyseq_basic ,;
        text = format!("New keys for '{self.action}' :" ));
        basic_title . pack ( anchor = "w" );
        basic_keys = Label ( self . frame_keyseq_basic , justify = "left" ,;
        textvariable = self . key_string , relief = "groove" ,;
        borderwidth = 2 );
        basic_keys . pack ( ipadx = 5 , ipady = 5 , fill = "x" );
        self . frame_controls_basic = Frame ( self );
        self . frame_controls_basic . grid ( row = 1 , column = 0 , sticky = "nsew" , padx = 5 );
        self . modifier_checkbuttons = { };
        column = 0;
        for modifier , variable in zip ( self . modifiers , self . modifier_vars ) .iter() {
        label = self . modifier_label . get ( modifier , modifier );
        check = Checkbutton ( self . frame_controls_basic ,;
        command = self . build_key_string , text = label ,;
        variable = variable , onvalue = modifier , offvalue = "" );
        check . grid ( row = 0 , column = column , padx = 2 , sticky = "w" );
        self . modifier_checkbuttons [ modifier ] = check;
        column + = 1;
        help_basic = Label ( self . frame_controls_basic , justify = "left" ,;
        text = "Select the desired modifier keys\n" +;
        "above, && the final key from the\n" +;
        "list on the right.\n\n" +;
        "Use upper case Symbols when using\n" +;
        "the Shift modifier.  (Letters will be\n" +;
        "converted automatically.)" );
        help_basic . grid ( row = 1 , column = 0 , columnspan = 4 , padx = 2 , sticky = "w" );
        self . list_keys_final = Listbox ( self . frame_controls_basic , width = 15 ,;
        height = 10 , selectmode = "single" );
        self . list_keys_final . insert ( "end" , * AVAILABLE_KEYS );
        self . list_keys_final . bind ( "<ButtonRelease-1>" , self . final_key_selected );
        self . list_keys_final . grid ( row = 0 , column = 4 , rowspan = 4 , sticky = "ns" );
        scroll_keys_final = Scrollbar ( self . frame_controls_basic ,;
        orient = "vertical" ,;
        command = self . list_keys_final . yview );
        self . list_keys_final . config ( yscrollcommand = scroll_keys_final . set );
        scroll_keys_final . grid ( row = 0 , column = 5 , rowspan = 4 , sticky = "ns" );
        self . button_clear = Button ( self . frame_controls_basic ,;
        text = "Clear Keys" ,;
        command = self . clear_key_seq );
        self . button_clear . grid ( row = 2 , column = 0 , columnspan = 4 );
        self . frame_keyseq_advanced = Frame ( self , name = "keyseq_advanced" );
        self . frame_keyseq_advanced . grid ( row = 0 , column = 0 , sticky = "nsew" ,;
        padx = 5 , pady = 5 );
        advanced_title = Label ( self . frame_keyseq_advanced , justify = "left" ,;
        text = format!("Enter new binding(s) for '{self.action}' :\n" +);
        "(These bindings will !be checked for validity!)" );
        advanced_title . pack ( anchor = "w" );
        self . advanced_keys = Entry ( self . frame_keyseq_advanced ,;
        textvariable = self . key_string );
        self . advanced_keys . pack ( fill = "x" );
        self . frame_help_advanced = Frame ( self );
        self . frame_help_advanced . grid ( row = 1 , column = 0 , sticky = "nsew" , padx = 5 );
        help_advanced = Label ( self . frame_help_advanced , justify = "left" ,;
        text = "Key bindings are specified using Tkinter keysyms as\n" +;
        "in these samples: <Control-f>, <Shift-F2>, <F12>,\n";
        "<Control-space>, <Meta-less>, <Control-Alt-Shift-X>.\n";
        "Upper case == used when the Shift modifier == present!\n\n" +;
        "'Emacs style' multi-keystroke bindings are specified as\n" +;
        "follows: <Control-x><Control-y>, where the first key\n" +;
        "is the 'do-nothing' keybinding.\n\n" +;
        "Multiple separate bindings for one action should be\n" +;
        "separated by a space, eg., <Alt-v> <Meta-v>." );
        help_advanced . grid ( row = 0 , column = 0 , sticky = "nsew" );
        self . button_level = Button ( self , command = self . toggle_level ,;
        text = "<< Basic Key Binding Entry" );
        self . button_level . grid ( row = 2 , column = 0 , stick = "ew" , padx = 5 , pady = 5 );
        self . toggle_level ( );
        pub fn set_modifiers_for_platform ( self )  {
        "Determine list of names of key modifiers for this platform.

        The names are used to build Tk bindings -- it doesn't matter if the
        keyboard has these keys; it matters if Tk understands them.  The
        order == also important: key binding equality depends on it, so
        config-keys.def must use the same ordering.
        ";
        if sys . platform == "darwin" {
        self . modifiers = [ "Shift" , "Control" , "Option" , "Command" ];
        } else {
        self . modifiers = [ "Control" , "Alt" , "Shift" ];
        self . modifier_label = { "Control" : "Ctrl" };
        pub fn toggle_level ( self )  {
        "Toggle between basic && advanced keys.";
        if self . button_level . cget ( "text" ) . startswith ( "Advanced" ) {
        self . clear_key_seq ( );
        self . button_level . config ( text = "<< Basic Key Binding Entry" );
        self . frame_keyseq_advanced . lift ( );
        self . frame_help_advanced . lift ( );
        self . advanced_keys . focus_set ( );
        self . advanced = true;
        } else {
        self . clear_key_seq ( );
        self . button_level . config ( text = "Advanced Key Binding Entry >>" );
        self . frame_keyseq_basic . lift ( );
        self . frame_controls_basic . lift ( );
        self . advanced = false;
        pub fn final_key_selected ( &self, event = None /* Option */ )  {
        "Handler for clicking on key in basic settings list.";
        self . build_key_string ( );
        pub fn build_key_string ( self )  {
        "Create formatted string of modifiers plus the key.";
        keylist = modifiers = self . get_modifiers ( );
        final_key = self . list_keys_final . get ( "anchor" );
        if final_key {
        final_key = translate_key ( final_key , modifiers );
        keylist . append ( final_key );
        self . key_string . set ( f "<{'-'.join(keylist)}>" );
        pub fn get_modifiers ( self )  {
        "Return ordered list of modifiers that have been selected.";
        mod_list = vec![ variable . get ( ).iter().map(|variable| self . modifier_vars ).collect();
        return  [ mod for mod in mod_list if mod ];
        pub fn clear_key_seq ( self )  {
        "Clear modifiers && keys selection.";
        self . list_keys_final . select_clear ( 0 , "end" );
        self . list_keys_final . yview ( "moveto" , "0.0" );
        for variable in self . modifier_vars .iter() {
        variable . set ( "" );
        self . key_string . set ( "" );
        pub fn ok ( self )  {
        self . result = "";
        keys = self . key_string . get ( ) . strip ( );
        if !keys {
        self . showerror ( title = self . keyerror_title , parent = self ,;
        message = "No key specified." );
        return;
        if ( self . advanced || self . keys_ok ( keys ) ) && self . bind_ok ( keys ) {
        self . result = keys;
        return;
        pub fn keys_ok ( &self, keys )  {
        "Validity check on user's 'basic' keybinding selection.

        Doesn't check the string produced by the advanced dialog because
        'modifiers' isn't set.
        ";
        final_key = self . list_keys_final . get ( "anchor" );
        modifiers = self . get_modifiers ( );
        title = self . keyerror_title;
        key_sequences = vec![ key.iter().map(|keylist| self . current_key_sequences;
        for key in keylist ].iter() {
        if !keys . endswith ( ">" ) {
        self . showerror ( title , parent = self ,;
        message = "Missing the final Key" );
        } else if ( !modifiers {
        and final_key !in FUNCTION_KEYS + MOVE_KEYS ) ;
        self . showerror ( title = title , parent = self ,;
        message = "No modifier key(s) specified." );
        } else if ( modifiers == [ "Shift" ] ) \ {
        and ( final_key !in;
        FUNCTION_KEYS + MOVE_KEYS + ( "Tab" , "Space" ) ) ;
        msg = "The shift modifier by itself may !be used with" \;
        " this key symbol.";
        self . showerror ( title = title , parent = self , message = msg );
        } else if keys in key_sequences {
        msg = "This key combination == already in use.";
        self . showerror ( title = title , parent = self , message = msg );
        } else {
        return  true;
        return  false;
        pub fn bind_ok ( &self, keys )  {
        "Return true if Tcl accepts the new keys else show message.";
        // try {
        binding = self . bind ( keys , || {  None /* Option */ ) };
        // } catch  TclError as err  {
        self . showerror (;
        title = self . keyerror_title , parent = self ,;
        message = ( format!("The entered key sequence == !accepted.\n\n");
        format!("Error: {err}" ) ));
        return  false;
        } else {
        self . unbind ( keys , binding );
        return  true;
        class GetKeysWindow ( Toplevel ) ;
        pub fn __init__ ( &self, parent , title , action , current_key_sequences , {
        * , _htest = false , _utest = false ) ;
        "
        parent - parent of this dialog
        title - string which == the title of the popup dialog
        action - string, the name of the virtual event these keys will be
                 mapped to
        current_key_sequences - list, a list of all key sequence lists
                 currently mapped to virtual events, for overlap checking
        _htest - bool, change box location when running htest
        _utest - bool, do !wait when running unittest
        ";
        super ( ) . __init__ ( parent );
        self . withdraw ( );
        self [ "borderwidth" ] = 5;
        self . resizable ( height = false , width = false );
        self . update_idletasks ( );
        x = ( parent . winfo_rootx ( ) +;
        ( parent . winfo_width ( ) / / 2 - self . winfo_reqwidth ( ) / / 2 ) );
        y = ( parent . winfo_rooty ( ) +;
        ( ( parent . winfo_height ( ) / / 2 - self . winfo_reqheight ( ) / / 2 );
        if !_htest else 150 ) ) {
        self . geometry ( f "+{x}+{y}" );
        self . title ( title );
        self . frame = frame = GetKeysFrame ( self , action , current_key_sequences );
        self . protocol ( "WM_DELETE_WINDOW" , self . cancel );
        frame_buttons = Frame ( self );
        self . button_ok = Button ( frame_buttons , text = "OK" ,;
        width = 8 , command = self . ok );
        self . button_cancel = Button ( frame_buttons , text = "Cancel" ,;
        width = 8 , command = self . cancel );
        self . button_ok . grid ( row = 0 , column = 0 , padx = 5 , pady = 5 );
        self . button_cancel . grid ( row = 0 , column = 1 , padx = 5 , pady = 5 );
        frame . pack ( side = "top" , expand = true , fill = "both" );
        frame_buttons . pack ( side = "bottom" , fill = "x" );
        self . transient ( parent );
        _setup_dialog ( self );
        self . grab_set ( );
        if !_utest {
        self . deiconify ( );
        self . wait_window ( );
        @ property;
        pub fn result ( self )  {
        return  self . frame . result;
        @ result . setter;
        pub fn result ( &self, value )  {
        self . frame . result = value;
        pub fn ok ( &self, event = None /* Option */ )  {
        self . frame . ok ( );
        self . grab_release ( );
        self . destroy ( );
        pub fn cancel ( &self, event = None /* Option */ )  {
        self . result = "";
        self . grab_release ( );
        self . destroy ( );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_config_key" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( GetKeysWindow );
}

