//! editor.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::importlib;
// use std::fs;
// use regex::Regex;
// use std::env;
// use crate::traceback;
// use crate::tkinter::{};
// use crate::idlelib::{idleConf};
// use crate::winreg;
// use crate::subprocess;
// use crate::unittest::{main};

pub const TK_TABWIDTH_DEFAULT: u64 = 8;
pub const _py_version: &str = " (%s)" % platform . python_version ( );
pub const darwin: &str = sys . platform =="darwin";
pub fn _sphinx_version() {
        "Format sys.version_info to produce the Sphinx version string used to install the chm docs";
        major , minor , micro , level , serial = sys . version_info;
        release = format!("{major}{minor}");
        release + = format!("{micro}");
        if level == "candidate" {
        release + = format!("rc{serial}");
        } else if level != "final" {
        release + = format!("{level[0]}{serial}");
        return  release;
        class EditorWindow ;
        from idlelib . percolator import Percolator;
        from idlelib . colorizer import ColorDelegator , color_config;
        from idlelib . undo import UndoDelegator;
        from idlelib . iomenu import IOBinding , encoding;
        from idlelib import mainmenu;
        from idlelib . statusbar import MultiStatusBar;
        from idlelib . autocomplete import AutoComplete;
        from idlelib . autoexpand import AutoExpand;
        from idlelib . calltip import Calltip;
        from idlelib . codecontext import CodeContext;
        from idlelib . sidebar import LineNumbers;
        from idlelib . format import FormatParagraph , FormatRegion , Indents , Rstrip;
        from idlelib . parenmatch import ParenMatch;
        from idlelib . zoomheight import ZoomHeight;
        filesystemencoding = sys . getfilesystemencoding ( );
        help_url = None /* Option */;
        allow_code_context = true;
        allow_line_numbers = true;
        user_input_insert_tags = None /* Option */;
        pub fn __init__ ( &self, flist = None /* Option */ , filename = None /* Option */ , key = None /* Option */ , root = None /* Option */ )  {
        from idlelib . runscript import ScriptBinding;
        if EditorWindow . help_url is None /* Option */ {
        dochome = os . path . join ( sys . base_prefix , "Doc" , "index.html" );
        if sys . platform . count ( "linux" ) {
        pyver = "python-docs-" + "%s.%s.%s" % sys . version_info [ : 3 ];
        if os . path . isdir ( "/var/www/html/python/" ) {
        dochome = "/var/www/html/python/index.html";
        } else {
        basepath = "/usr/share/doc/";
        dochome = os . path . join ( basepath , pyver ,;
        "Doc" , "index.html" );
        } else if sys . platform [ {
        import winreg;
        docfile = "";
        KEY = ( rformat!("Software\Python\PythonCore\{sys.winver}");
        r "\Help\Main Python Documentation" );
        // try {
        docfile = winreg . QueryValue ( winreg . HKEY_CURRENT_USER , KEY );
        // } catch  FileNotFoundError  {
        // try {
        docfile = winreg . QueryValue ( winreg . HKEY_LOCAL_MACHINE ,;
        KEY );
        // } catch  FileNotFoundError  {
        // pass
        if os . path . isfile ( docfile ) {
        dochome = docfile;
        } else if sys . platform == "darwin" {
        dochome = os . path . join ( sys . base_prefix ,;
        "Resources/English.lproj/Documentation/index.html" );
        dochome = os . path . normpath ( dochome );
        if os . path . isfile ( dochome ) {
        EditorWindow . help_url = dochome;
        if sys . platform == "darwin" {
        EditorWindow . help_url = "file://" + EditorWindow . help_url;
        } else {
        EditorWindow . help_url = ( "https://docs.python.org/%d.%d/";
        % sys . version_info [ : 2 ] );
        self . flist = flist;
        root = root || flist . root;
        self . root = root;
        self . menubar = Menu ( root );
        self . top = top = window . ListedToplevel ( root , menu = self . menubar );
        if flist {
        self . tkinter_vars = flist . vars;
        self . top . instance_dict = flist . inversedict;
        } else {
        self . tkinter_vars = { };
        self . top . instance_dict = { };
        self . recent_files_path = idleConf . userdir && os . path . join (;
        idleConf . userdir , "recent-files.lst" );
        self . prompt_last_line = "";
        self . text_frame = text_frame = Frame ( top );
        self . vbar = vbar = Scrollbar ( text_frame , name = "vbar" );
        width = idleConf . GetOption ( "main" , "EditorWindow" , "width" , type = "int" );
        text_options = {;
        "name" : "text" ,;
        "padx" : 5 ,;
        "wrap" : "none" ,;
        "highlightthickness" : 0 ,;
        "width" : width ,;
        "tabstyle" : "wordprocessor" ,;
        "height" : idleConf . GetOption (;
        "main" , "EditorWindow" , "height" , type = "int" ) ,;
        };
        self . text = text = MultiCallCreator ( Text ) ( text_frame , ** text_options );
        self . top . focused_widget = self . text;
        self . createmenubar ( );
        self . apply_bindings ( );
        self . top . protocol ( "WM_DELETE_WINDOW" , self . close );
        self . top . bind ( "<<close-window>>" , self . close_event );
        if macosx . isAquaTk ( ) {
        text . bind ( "<<close-window>>" , self . close_event );
        text . bind ( "<Control-Button-1>" , self . right_menu_event );
        text . bind ( "<2>" , self . right_menu_event );
        } else {
        text . bind ( "<3>" , self . right_menu_event );
        text . bind ( "<MouseWheel>" , wheel_event );
        if text . _windowingsystem == "x11" {
        text . bind ( "<Button-4>" , wheel_event );
        text . bind ( "<Button-5>" , wheel_event );
        text . bind ( "<Configure>" , self . handle_winconfig );
        text . bind ( "<<cut>>" , self . cut );
        text . bind ( "<<copy>>" , self . copy );
        text . bind ( "<<paste>>" , self . paste );
        text . bind ( "<<center-insert>>" , self . center_insert_event );
        text . bind ( "<<help>>" , self . help_dialog );
        text . bind ( "<<python-docs>>" , self . python_docs );
        text . bind ( "<<about-idle>>" , self . about_dialog );
        text . bind ( "<<open-config-dialog>>" , self . config_dialog );
        text . bind ( "<<open-module>>" , self . open_module_event );
        text . bind ( "<<do-nothing>>" , |event | {  "break" ) };
        text . bind ( "<<select-all>>" , self . select_all );
        text . bind ( "<<remove-selection>>" , self . remove_selection );
        text . bind ( "<<find>>" , self . find_event );
        text . bind ( "<<find-again>>" , self . find_again_event );
        text . bind ( "<<find-in-files>>" , self . find_in_files_event );
        text . bind ( "<<find-selection>>" , self . find_selection_event );
        text . bind ( "<<replace>>" , self . replace_event );
        text . bind ( "<<goto-line>>" , self . goto_line_event );
        text . bind ( "<<smart-backspace>>" , self . smart_backspace_event );
        text . bind ( "<<newline-and-indent>>" , self . newline_and_indent_event );
        text . bind ( "<<smart-indent>>" , self . smart_indent_event );
        self . fregion = fregion = self . FormatRegion ( self );
        text . bind ( "<<indent-region>>" , fregion . indent_region_event );
        text . bind ( "<<dedent-region>>" , fregion . dedent_region_event );
        text . bind ( "<<comment-region>>" , fregion . comment_region_event );
        text . bind ( "<<uncomment-region>>" , fregion . uncomment_region_event );
        text . bind ( "<<tabify-region>>" , fregion . tabify_region_event );
        text . bind ( "<<untabify-region>>" , fregion . untabify_region_event );
        indents = self . Indents ( self );
        text . bind ( "<<toggle-tabs>>" , indents . toggle_tabs_event );
        text . bind ( "<<change-indentwidth>>" , indents . change_indentwidth_event );
        text . bind ( "<Left>" , self . move_at_edge_if_selection ( 0 ) );
        text . bind ( "<Right>" , self . move_at_edge_if_selection ( 1 ) );
        text . bind ( "<<del-word-left>>" , self . del_word_left );
        text . bind ( "<<del-word-right>>" , self . del_word_right );
        text . bind ( "<<beginning-of-line>>" , self . home_callback );
        if flist {
        flist . inversedict [ self ] = key;
        if key {
        flist . dict [ key ] = self;
        text . bind ( "<<open-new-window>>" , self . new_callback );
        text . bind ( "<<close-all-windows>>" , self . flist . close_all_callback );
        text . bind ( "<<open-class-browser>>" , self . open_module_browser );
        text . bind ( "<<open-path-browser>>" , self . open_path_browser );
        text . bind ( "<<open-turtle-demo>>" , self . open_turtle_demo );
        self . set_status_bar ( );
        text_frame . pack ( side = LEFT , fill = BOTH , expand = 1 );
        text_frame . rowconfigure ( 1 , weight = 1 );
        text_frame . columnconfigure ( 1 , weight = 1 );
        vbar [ "command" ] = self . handle_yview;
        vbar . grid ( row = 1 , column = 2 , sticky = NSEW );
        text [ "yscrollcommand" ] = vbar . set;
        text [ "font" ] = idleConf . GetFont ( self . root , "main" , "EditorWindow" );
        text . grid ( row = 1 , column = 1 , sticky = NSEW );
        text . focus_set ( );
        self . set_width ( );
        usespaces = idleConf . GetOption ( "main" , "Indent" ,;
        "use-spaces" , type = "bool" );
        self . usetabs = !usespaces;
        self . tabwidth = 8;
        self . indentwidth = self . tabwidth;
        self . set_notabs_indentwidth ( );
        if !hasattr ( idleConf , "blink_off_time" ) {
        idleConf . blink_off_time = self . text [ "insertofftime" ];
        self . update_cursor_blink ( );
        self . num_context_lines = 50 , 500 , 5000000;
        self . per = per = self . Percolator ( text );
        self . undo = undo = self . UndoDelegator ( );
        per . insertfilter ( undo );
        text . undo_block_start = undo . undo_block_start;
        text . undo_block_stop = undo . undo_block_stop;
        undo . set_saved_change_hook ( self . saved_change_hook );
        self . io = io = self . IOBinding ( self );
        io . set_filename_change_hook ( self . filename_change_hook );
        self . good_load = false;
        self . set_indentation_params ( false );
        self . color = None /* Option */;
        self . code_context = None /* Option */;
        self . line_numbers = None /* Option */;
        if filename {
        if os . path . exists ( filename ) && !os . path . isdir ( filename ) {
        if io . loadfile ( filename ) {
        self . good_load = true;
        is_py_src = self . ispythonsource ( filename );
        self . set_indentation_params ( is_py_src );
        } else {
        io . set_filename ( filename );
        self . good_load = true;
        self . ResetColorizer ( );
        self . saved_change_hook ( );
        self . update_recent_files_list ( );
        self . load_extensions ( );
        menu = self . menudict . get ( "window" );
        if menu {
        end = menu . index ( "end" );
        if end is None /* Option */ {
        end = -1;
        if end >= 0 {
        menu . add_separator ( );
        end = end + 1;
        self . wmenu_end = end;
        window . register_callback ( self . postwindowsmenu );
        self . askinteger = simpledialog . askinteger;
        self . askyesno = messagebox . askyesno;
        self . showerror = messagebox . showerror;
        text . event_add ( "<<autocomplete>>" , "<Key-Tab>" );
        text . event_add ( "<<try-open-completions>>" , "<KeyRelease-period>" ,;
        "<KeyRelease-slash>" , "<KeyRelease-backslash>" );
        text . event_add ( "<<try-open-calltip>>" , "<KeyRelease-parenleft>" );
        text . event_add ( "<<refresh-calltip>>" , "<KeyRelease-parenright>" );
        text . event_add ( "<<paren-closed>>" , "<KeyRelease-parenright>" ,;
        "<KeyRelease-bracketright>" , "<KeyRelease-braceright>" );
        autocomplete = self . AutoComplete ( self , self . user_input_insert_tags );
        text . bind ( "<<autocomplete>>" , autocomplete . autocomplete_event );
        text . bind ( "<<try-open-completions>>" ,;
        autocomplete . try_open_completions_event );
        text . bind ( "<<force-open-completions>>" ,;
        autocomplete . force_open_completions_event );
        text . bind ( "<<expand-word>>" , self . AutoExpand ( self ) . expand_word_event );
        text . bind ( "<<format-paragraph>>" ,;
        self . FormatParagraph ( self ) . format_paragraph_event );
        parenmatch = self . ParenMatch ( self );
        text . bind ( "<<flash-paren>>" , parenmatch . flash_paren_event );
        text . bind ( "<<paren-closed>>" , parenmatch . paren_closed_event );
        scriptbinding = ScriptBinding ( self );
        text . bind ( "<<check-module>>" , scriptbinding . check_module_event );
        text . bind ( "<<run-module>>" , scriptbinding . run_module_event );
        text . bind ( "<<run-custom>>" , scriptbinding . run_custom_event );
        text . bind ( "<<do-rstrip>>" , self . Rstrip ( self ) . do_rstrip );
        self . ctip = ctip = self . Calltip ( self );
        text . bind ( "<<try-open-calltip>>" , ctip . try_open_calltip_event );
        text . bind ( "<<refresh-calltip>>" , ctip . refresh_calltip_event );
        text . bind ( "<<force-open-calltip>>" , ctip . force_open_calltip_event );
        text . bind ( "<<zoom-height>>" , self . ZoomHeight ( self ) . zoom_height_event );
        if self . allow_code_context {
        self . code_context = self . CodeContext ( self );
        text . bind ( "<<toggle-code-context>>" ,;
        self . code_context . toggle_code_context_event );
        } else {
        self . update_menu_state ( "options" , "*ode*ontext" , "disabled" );
        if self . allow_line_numbers {
        self . line_numbers = self . LineNumbers ( self );
        if idleConf . GetOption ( "main" , "EditorWindow" , {
        "line-numbers-default" , type = "bool" ) ;
        self . toggle_line_numbers_event ( );
        text . bind ( "<<toggle-line-numbers>>" , self . toggle_line_numbers_event );
        } else {
        self . update_menu_state ( "options" , "*ine*umbers" , "disabled" );
        pub fn handle_winconfig ( &self, event = None /* Option */ )  {
        self . set_width ( );
        pub fn set_width ( self )  {
        text = self . text;
        inner_padding = sum ( map ( text . tk . getint , [ text . cget ( "border" ) ,;
        text . cget ( "padx" ) ] ) );
        pixel_width = text . winfo_width ( ) - 2 * inner_padding;
        zero_char_width = \;
        Font ( text , font = text . cget ( "font" ) ) . measure ( "0" );
        self . width = pixel_width / / zero_char_width;
        pub fn new_callback ( &self, event )  {
        dirname , basename = self . io . defaultfilename ( );
        self . flist . new ( dirname );
        return  "break";
        pub fn home_callback ( &self, event )  {
        if ( event . state & 4 ) != 0 && event . keysym == "Home" {
        return;
        if self . text . index ( "iomark" ) && \ {
        self . text . compare ( "iomark" , "<=" , "insert lineend" ) && \;
        self . text . compare ( "insert linestart" , "<=" , "iomark" ) :;
        insertpt = int ( self . text . index ( "iomark" ) . split ( "." ) [ 1 ] );
        } else {
        line = self . text . get ( "insert linestart" , "insert lineend" );
        for insertpt in range ( len ( line ) ) .iter() {
        if line [ insertpt ] !in ( " " , "\t" ) {
        break;
        } else {
        insertpt = len ( line );
        lineat = int ( self . text . index ( "insert" ) . split ( "." ) [ 1 ] );
        if insertpt == lineat {
        insertpt = 0;
        dest = "insert linestart+" + str ( insertpt ) + "c";
        if ( event . state & 1 ) == 0 {
        self . text . tag_remove ( "sel" , "1.0" , "end" );
        } else {
        if !self . text . index ( "sel.first" ) {
        self . text . mark_set ( "my_anchor" , "insert" );
        } else {
        if self . text . compare ( self . text . index ( "sel.first" ) , "<" , {
        self . text . index ( "insert" ) ) :;
        self . text . mark_set ( "my_anchor" , "sel.first" );
        } else {
        self . text . mark_set ( "my_anchor" , "sel.last" );
        first = self . text . index ( dest );
        last = self . text . index ( "my_anchor" );
        if self . text . compare ( first , ">" , last ) {
        first , last = last , first;
        self . text . tag_remove ( "sel" , "1.0" , "end" );
        self . text . tag_add ( "sel" , first , last );
        self . text . mark_set ( "insert" , dest );
        self . text . see ( "insert" );
        return  "break";
        pub fn set_status_bar ( self )  {
        self . status_bar = self . MultiStatusBar ( self . top );
        sep = Frame ( self . top , height = 1 , borderwidth = 1 , background = "grey75" );
        if sys . platform == "darwin" {
        self . status_bar . set_label ( "_padding1" , "    " , side = RIGHT );
        self . status_bar . set_label ( "column" , "Col: ?" , side = RIGHT );
        self . status_bar . set_label ( "line" , "Ln: ?" , side = RIGHT );
        self . status_bar . pack ( side = BOTTOM , fill = X );
        sep . pack ( side = BOTTOM , fill = X );
        self . text . bind ( "<<set-line-and-column>>" , self . set_line_and_column );
        self . text . event_add ( "<<set-line-and-column>>" ,;
        "<KeyRelease>" , "<ButtonRelease>" );
        self . text . after_idle ( self . set_line_and_column );
        pub fn set_line_and_column ( &self, event = None /* Option */ )  {
        line , column = self . text . index ( INSERT ) . split ( "." );
        self . status_bar . set_label ( "column" , "Col: %s" % column );
        self . status_bar . set_label ( "line" , "Ln: %s" % line );
        " Menu definitions && functions.
    * self.menubar - the always visible horizontal menu bar.
    * mainmenu.menudefs - a list of tuples, one for each menubar item.
      Each tuple pairs a lower-case name && list of dropdown items.
      Each item == a name, virtual event pair || None /* Option */ for separator.
    * mainmenu.default_keydefs - maps events to keys.
    * text.keydefs - same.
    * cls.menu_specs - menubar name, titlecase display form pairs
      with Alt-hotkey indicator.  A subset of menudefs items.
    * self.menudict - map menu name to dropdown menu.
    * self.recent_files_menu - 2nd level cascade in the file cascade.
    * self.wmenu_end - set in __init__ (purpose unclear).

    createmenubar, postwindowsmenu, update_menu_label, update_menu_state,
    ApplyKeybings (2nd part), reset_help_menu_entries,
    _extra_help_callback, update_recent_files_list,
    apply_bindings, fill_menus, (other functions?)
    ";
        menu_specs = [;
        ( "file" , "_File" ) ,;
        ( "edit" , "_Edit" ) ,;
        ( "format" , "F_ormat" ) ,;
        ( "run" , "_Run" ) ,;
        ( "options" , "_Options" ) ,;
        ( "window" , "_Window" ) ,;
        ( "help" , "_Help" ) ,;
        ];
        pub fn createmenubar ( self )  {
        "Populate the menu bar widget for the editor window.

        Each option on the menubar == itself a cascade-type Menu widget
        with the menubar as the parent.  The names, labels, && menu
        shortcuts for the menubar items are stored in menu_specs.  Each
        submenu == subsequently populated in fill_menus(), except for
        'Recent Files' which == added to the File menu here.

        Instance variables:
        menubar: Menu widget containing first level menu items.
        menudict: Dictionary of {menuname: Menu instance} items.  The keys
            represent the valid menu items for this window && may be a
            subset of all the menudefs available.
        recent_files_menu: Menu widget contained within the 'file' menudict.
        ";
        mbar = self . menubar;
        self . menudict = menudict = { };
        for name , label in self . menu_specs .iter() {
        underline , label = prepstr ( label );
        postcommand = getattr ( self , format!("{name}_menu_postcommand" , None /* Option */ ));
        menudict [ name ] = menu = Menu ( mbar , name = name , tearoff = 0 ,;
        postcommand = postcommand );
        mbar . add_cascade ( label = label , menu = menu , underline = underline );
        if macosx . isCarbonTk ( ) {
        menudict [ "application" ] = menu = Menu ( mbar , name = "apple" ,;
        tearoff = 0 );
        mbar . add_cascade ( label = "IDLE" , menu = menu );
        self . fill_menus ( );
        self . recent_files_menu = Menu ( self . menubar , tearoff = 0 );
        self . menudict [ "file" ] . insert_cascade ( 3 , label = "Recent Files" ,;
        underline = 0 ,;
        menu = self . recent_files_menu );
        self . base_helpmenu_length = self . menudict [ "help" ] . index ( END );
        self . reset_help_menu_entries ( );
        pub fn postwindowsmenu ( self )  {
        "Callback to register window.

        Only called when Window menu exists.
        ";
        menu = self . menudict [ "window" ];
        end = menu . index ( "end" );
        if end is None /* Option */ {
        end = -1;
        if end > self . wmenu_end {
        menu . delete ( self . wmenu_end + 1 , end );
        window . add_windows_to_menu ( menu );
        pub fn update_menu_label ( &self, menu , index , label )  {
        "Update label for menu item at index.";
        menuitem = self . menudict [ menu ];
        menuitem . entryconfig ( index , label = label );
        pub fn update_menu_state ( &self, menu , index , state )  {
        "Update state for menu item at index.";
        menuitem = self . menudict [ menu ];
        menuitem . entryconfig ( index , state = state );
        pub fn handle_yview ( &self, event , * args )  {
        "Handle scrollbar.";
        if event == "moveto" {
        fraction = float ( args [ 0 ] );
        lines = ( round ( self . getlineno ( "end" ) * fraction ) -;
        self . getlineno ( "@0,0" ) );
        event = "scroll";
        args = ( lines , "units" );
        self . text . yview ( event , * args );
        return  "break";
        rmenu = None /* Option */;
        pub fn right_menu_event ( &self, event )  {
        text = self . text;
        newdex = text . index ( format!("@{event.x},{event.y}" ));
        // try {
        in_selection = ( text . compare ( "sel.first" , "<=" , newdex ) and;
        text . compare ( newdex , "<=" , "sel.last" ) );
        // } catch  TclError  {
        in_selection = false;
        if !in_selection {
        text . tag_remove ( "sel" , "1.0" , "end" );
        text . mark_set ( "insert" , newdex );
        if !self . rmenu {
        self . make_rmenu ( );
        rmenu = self . rmenu;
        self . event = event;
        iswin = sys . platform [ : 3 ] == "win";
        if iswin {
        text . config ( cursor = "arrow" );
        for item in self . rmenu_specs .iter() {
        // try {
        label , eventname , verify_state = item;
        // } catch  ValueError  {
        continue;
        if verify_state is None /* Option */ {
        continue;
        state = getattr ( self , verify_state ) ( );
        rmenu . entryconfigure ( label , state = state );
        rmenu . tk_popup ( event . x_root , event . y_root );
        if iswin {
        self . text . config ( cursor = "ibeam" );
        return  "break";
        rmenu_specs = [;
        ( "Close" , "<<close-window>>" , None /* Option */ ) ,;
        ];
        pub fn make_rmenu ( self )  {
        rmenu = Menu ( self . text , tearoff = 0 );
        for item in self . rmenu_specs .iter() {
        label , eventname = item [ 0 ] , item [ 1 ];
        if label is !None /* Option */ {
        pub fn command ( text = self . text , eventname = eventname )  {
        text . event_generate ( eventname );
        rmenu . add_command ( label = label , command = command );
        } else {
        rmenu . add_separator ( );
        self . rmenu = rmenu;
        pub fn rmenu_check_cut ( self )  {
        return  self . rmenu_check_copy ( );
        pub fn rmenu_check_copy ( self )  {
        // try {
        indx = self . text . index ( "sel.first" );
        // } catch  TclError  {
        return  "disabled";
        } else {
        return  "normal" if indx else "disabled";
        pub fn rmenu_check_paste ( self )  {
        // try {
        self . text . tk . call ( "tk::GetSelection" , self . text , "CLIPBOARD" );
        // } catch  TclError  {
        return  "disabled";
        } else {
        return  "normal";
        pub fn about_dialog ( &self, event = None /* Option */ )  {
        "Handle Help 'About IDLE' event.";
        help_about . AboutDialog ( self . top );
        return  "break";
        pub fn config_dialog ( &self, event = None /* Option */ )  {
        "Handle Options 'Configure IDLE' event.";
        configdialog . ConfigDialog ( self . top , "Settings" );
        return  "break";
        pub fn help_dialog ( &self, event = None /* Option */ )  {
        "Handle Help 'IDLE Help' event.";
        if self . root {
        parent = self . root;
        } else {
        parent = self . top;
        help . show_idlehelp ( parent );
        return  "break";
        pub fn python_docs ( &self, event = None /* Option */ )  {
        if sys . platform [ { : 3 ] == "win" ; }
        // try {
        os . startfile ( self . help_url );
        // } catch  OSError as why  {
        messagebox . showerror ( title = "Document Start Failure" ,;
        message = str ( why ) , parent = self . text );
        } else {
        webbrowser . open ( self . help_url );
        return  "break";
        pub fn cut ( &self, event )  {
        self . text . event_generate ( "<<Cut>>" );
        return  "break";
        pub fn copy ( &self, event )  {
        if !self . text . tag_ranges ( "sel" ) {
        return;
        self . text . event_generate ( "<<Copy>>" );
        return  "break";
        pub fn paste ( &self, event )  {
        self . text . event_generate ( "<<Paste>>" );
        self . text . see ( "insert" );
        return  "break";
        pub fn select_all ( &self, event = None /* Option */ )  {
        self . text . tag_add ( "sel" , "1.0" , "end-1c" );
        self . text . mark_set ( "insert" , "1.0" );
        self . text . see ( "insert" );
        return  "break";
        pub fn remove_selection ( &self, event = None /* Option */ )  {
        self . text . tag_remove ( "sel" , "1.0" , "end" );
        self . text . see ( "insert" );
        return  "break";
        pub fn move_at_edge_if_selection ( &self, edge_index )  {
        "Cursor move begins at start || end of selection

        When a left/right cursor key == pressed create && return to Tkinter a
        function which causes a cursor move from the associated edge of the
        selection.

        ";
        self_text_index = self . text . index;
        self_text_mark_set = self . text . mark_set;
        edges_table = ( "sel.first+1c" , "sel.last-1c" );
        pub fn move_at_edge ( event )  {
        if ( event . state & 5 ) == 0 {
        // try {
        self_text_index ( "sel.first" );
        self_text_mark_set ( "insert" , edges_table [ edge_index ] );
        // } catch  TclError  {
        // pass
        return  move_at_edge;
        pub fn del_word_left ( &self, event )  {
        self . text . event_generate ( "<Meta-Delete>" );
        return  "break";
        pub fn del_word_right ( &self, event )  {
        self . text . event_generate ( "<Meta-d>" );
        return  "break";
        pub fn find_event ( &self, event )  {
        search . find ( self . text );
        return  "break";
        pub fn find_again_event ( &self, event )  {
        search . find_again ( self . text );
        return  "break";
        pub fn find_selection_event ( &self, event )  {
        search . find_selection ( self . text );
        return  "break";
        pub fn find_in_files_event ( &self, event )  {
        grep . grep ( self . text , self . io , self . flist );
        return  "break";
        pub fn replace_event ( &self, event )  {
        replace . replace ( self . text );
        return  "break";
        pub fn goto_line_event ( &self, event )  {
        text = self . text;
        lineno = query . Goto (;
        text , "Go To Line" ,;
        "Enter a positive integer\n";
        "('big' = end of file):";
        ) . result;
        if lineno is !None /* Option */ {
        text . tag_remove ( "sel" , "1.0" , "end" );
        text . mark_set ( "insert" , format!("{lineno}.0" ));
        text . see ( "insert" );
        self . set_line_and_column ( );
        return  "break";
        pub fn open_module ( self )  {
        "Get module name from user && open it.

        Return module path || None /* Option */ for calls by open_module_browser
        when latter == !invoked in named editor window.
        ";
        // try {
        name = self . text . get ( "sel.first" , "sel.last" ) . strip ( );
        // } catch  TclError  {
        name = "";
        file_path = query . ModuleName (;
        self . text , "Open Module" ,;
        "Enter the name of a Python module\n";
        "to search on sys.path && open:" ,;
        name ) . result;
        if file_path is !None /* Option */ {
        if self . flist {
        self . flist . open ( file_path );
        } else {
        self . io . loadfile ( file_path );
        return  file_path;
        pub fn open_module_event ( &self, event )  {
        self . open_module ( );
        return  "break";
        pub fn open_module_browser ( &self, event = None /* Option */ )  {
        filename = self . io . filename;
        if !( self . __class__ . __name__ == "PyShellEditorWindow" {
        and filename ) ;
        filename = self . open_module ( );
        if filename is None /* Option */ {
        return  "break";
        from idlelib import browser;
        browser . ModuleBrowser ( self . root , filename );
        return  "break";
        pub fn open_path_browser ( &self, event = None /* Option */ )  {
        from idlelib import pathbrowser;
        pathbrowser . PathBrowser ( self . root );
        return  "break";
        pub fn open_turtle_demo ( &self, event = None /* Option */ )  {
        import subprocess;
        cmd = [ sys . executable ,;
        "-c" ,;
        "from turtledemo.__main__ import main; main()" ];
        subprocess . Popen ( cmd , shell = false );
        return  "break";
        pub fn gotoline ( &self, lineno )  {
        if lineno is !None /* Option */ && lineno > 0 {
        self . text . mark_set ( "insert" , "%d.0" % lineno );
        self . text . tag_remove ( "sel" , "1.0" , "end" );
        self . text . tag_add ( "sel" , "insert" , "insert +1l" );
        self . center ( );
        pub fn ispythonsource ( &self, filename )  {
        if !filename || os . path . isdir ( filename ) {
        return  true;
        base , ext = os . path . splitext ( os . path . basename ( filename ) );
        if os . path . normcase ( ext ) in py_extensions {
        return  true;
        line = self . text . get ( "1.0" , "1.0 lineend" );
        return  line . startswith ( "#!" ) && "python" in line;
        pub fn close_hook ( self )  {
        if self . flist {
        self . flist . unregister_maybe_terminate ( self );
        self . flist = None /* Option */;
        pub fn set_close_hook ( &self, close_hook )  {
        self . close_hook = close_hook;
        pub fn filename_change_hook ( self )  {
        if self . flist {
        self . flist . filename_changed_edit ( self );
        self . saved_change_hook ( );
        self . top . update_windowlist_registry ( self );
        self . ResetColorizer ( );
        pub fn _addcolorizer ( self )  {
        if self . color {
        return;
        if self . ispythonsource ( self . io . filename ) {
        self . color = self . ColorDelegator ( );
        if self . color {
        self . per . insertfilterafter ( filter = self . color , after = self . undo );
        pub fn _rmcolorizer ( self )  {
        if !self . color {
        return;
        self . color . removecolors ( );
        self . per . removefilter ( self . color );
        self . color = None /* Option */;
        pub fn ResetColorizer ( self )  {
        "Update the color theme";
        self . _rmcolorizer ( );
        self . _addcolorizer ( );
        EditorWindow . color_config ( self . text );
        if self . code_context is !None /* Option */ {
        self . code_context . update_highlight_colors ( );
        if self . line_numbers is !None /* Option */ {
        self . line_numbers . update_colors ( );
        IDENTCHARS = string . ascii_letters + string . digits + "_";
        pub fn colorize_syntax_error ( &self, text , pos )  {
        text . tag_add ( "ERROR" , pos );
        char = text . get ( pos );
        if char && char in self . IDENTCHARS {
        text . tag_add ( "ERROR" , pos + " wordstart" , pos );
        if "\n" == text . get ( pos ) {
        text . mark_set ( "insert" , pos );
        } else {
        text . mark_set ( "insert" , pos + "+1c" );
        text . see ( pos );
        pub fn update_cursor_blink ( self )  {
        "Update the cursor blink configuration.";
        cursorblink = idleConf . GetOption (;
        "main" , "EditorWindow" , "cursor-blink" , type = "bool" );
        if !cursorblink {
        self . text [ "insertofftime" ] = 0;
        } else {
        self . text [ "insertofftime" ] = idleConf . blink_off_time;
        pub fn ResetFont ( self )  {
        "Update the text widgets' font if it == changed";
        if self . code_context is !None /* Option */ {
        self . code_context . update_font ( );
        if self . line_numbers is !None /* Option */ {
        self . line_numbers . update_font ( );
        new_font = idleConf . GetFont ( self . root , "main" , "EditorWindow" );
        self . text [ "font" ] = new_font;
        self . set_width ( );
        pub fn RemoveKeybindings ( self )  {
        "Remove the virtual, configurable keybindings.

        Leaves the default Tk Text keybindings.
        ";
        self . mainmenu . default_keydefs = keydefs = idleConf . GetCurrentKeySet ( );
        for event , keylist in keydefs . items ( ) .iter() {
        self . text . event_delete ( event , * keylist );
        for extensionName in self . get_standard_extension_names ( ) .iter() {
        xkeydefs = idleConf . GetExtensionBindings ( extensionName );
        if xkeydefs {
        for event , keylist in xkeydefs . items ( ) .iter() {
        self . text . event_delete ( event , * keylist );
        pub fn ApplyKeybindings ( self )  {
        "Apply the virtual, configurable keybindings.

        Alse update hotkeys to current keyset.
        ";
        self . mainmenu . default_keydefs = keydefs = idleConf . GetCurrentKeySet ( );
        self . apply_bindings ( );
        for extensionName in self . get_standard_extension_names ( ) .iter() {
        xkeydefs = idleConf . GetExtensionBindings ( extensionName );
        if xkeydefs {
        self . apply_bindings ( xkeydefs );
        menuEventDict = { };
        for menu in self . mainmenu . menudefs .iter() {
        menuEventDict [ menu [ 0 ] ] = { };
        for item in menu [ 1 ] .iter() {
        if item {
        menuEventDict [ menu [ 0 ] ] [ prepstr ( item [ 0 ] ) [ 1 ] ] = item [ 1 ];
        for menubarItem in self . menudict .iter() {
        menu = self . menudict [ menubarItem ];
        end = menu . index ( END );
        if end is None /* Option */ {
        continue;
        end + = 1;
        for index in range ( 0 , end ) .iter() {
        if menu . type ( index ) == "command" {
        accel = menu . entrycget ( index , "accelerator" );
        if accel {
        itemName = menu . entrycget ( index , "label" );
        event = "";
        if menubarItem in menuEventDict {
        if itemName in menuEventDict [ menubarItem ] {
        event = menuEventDict [ menubarItem ] [ itemName ];
        if event {
        accel = get_accelerator ( keydefs , event );
        menu . entryconfig ( index , accelerator = accel );
        pub fn set_notabs_indentwidth ( self )  {
        "Update the indentwidth if changed && !using tabs in this window";
        if !self . usetabs {
        self . indentwidth = idleConf . GetOption ( "main" , "Indent" , "num-spaces" ,;
        type = "int" );
        pub fn reset_help_menu_entries ( self )  {
        "Update the additional help entries on the Help menu.";
        help_list = idleConf . GetAllExtraHelpSourcesList ( );
        helpmenu = self . menudict [ "help" ];
        helpmenu_length = helpmenu . index ( END );
        if helpmenu_length > self . base_helpmenu_length {
        helpmenu . delete ( ( self . base_helpmenu_length + 1 ) , helpmenu_length );
        if help_list {
        helpmenu . add_separator ( );
        for entry in help_list .iter() {
        cmd = self . _extra_help_callback ( entry [ 1 ] );
        helpmenu . add_command ( label = entry [ 0 ] , command = cmd );
        self . menudict [ "help" ] = helpmenu;
        pub fn _extra_help_callback ( &self, resource )  {
        "Return a callback that loads resource (file || web page).";
        pub fn display_extra_help ( helpfile = resource )  {
        if !helpfile . startswith ( ( "www" , "http" ) ) {
        helpfile = os . path . normpath ( helpfile );
        if sys . platform [ { : 3 ] == "win" ; }
        // try {
        os . startfile ( helpfile );
        // } catch  OSError as why  {
        messagebox . showerror ( title = "Document Start Failure" ,;
        message = str ( why ) , parent = self . text );
        } else {
        webbrowser . open ( helpfile );
        return  display_extra_help;
        pub fn update_recent_files_list ( &self, new_file = None /* Option */ )  {
        "Load && update the recent files list && menus";
        rf_list = [ ];
        file_path = self . recent_files_path;
        if file_path && os . path . exists ( file_path ) {
        // with scope: open ( file_path , {
        encoding = "utf_8" , errors = "replace" ) as rf_list_file ;
        rf_list = rf_list_file . readlines ( );
        if new_file {
        new_file = os . path . abspath ( new_file ) + "\n";
        if new_file in rf_list {
        rf_list . remove ( new_file );
        rf_list . insert ( 0 , new_file );
        bad_paths = [ ];
        for path in rf_list .iter() {
        if "\0" in path || !os . path . exists ( path [ 0 { : -1 ] ) ; }
        bad_paths . append ( path );
        rf_list = vec![ path.iter().map(|path| rf_list if path !in bad_paths ).collect();
        ulchars = "1234567890ABCDEFGHIJK";
        rf_list = rf_list [ 0 : len ( ulchars ) ];
        if file_path {
        // try {
        // with scope: open ( file_path , "w" , {
        encoding = "utf_8" , errors = "replace" ) as rf_file ;
        rf_file . writelines ( rf_list );
        // } catch  OSError as err  {
        if !getattr ( self . root , "recentfiles_message" , false ) {
        self . root . recentfiles_message = true;
        messagebox . showwarning ( title = "IDLE Warning" ,;
        message = "Cannot save Recent Files list to disk.\n";
        format!("  {err}\n");
        "Select OK to continue." ,;
        parent = self . text );
        for instance in self . top . instance_dict .iter() {
        menu = instance . recent_files_menu;
        menu . delete ( 0 , END );
        for i , file_name in enumerate ( rf_list ) .iter() {
        file_name = file_name . rstrip ( );
        callback = instance . __recent_file_callback ( file_name );
        menu . add_command ( label = ulchars [ i ] + " " + file_name ,;
        command = callback ,;
        underline = 0 );
        pub fn __recent_file_callback ( &self, file_name )  {
        pub fn open_recent_file ( fn_closure = file_name )  {
        self . io . open ( editFile = fn_closure );
        return  open_recent_file;
        pub fn saved_change_hook ( self )  {
        short = self . short_title ( );
        long = self . long_title ( );
        if short && long && !macosx . isCocoaTk ( ) {
        title = short + " - " + long + _py_version;
        } else if short {
        title = short;
        } else if long {
        title = long;
        } else {
        title = "untitled";
        icon = short || long || title;
        if !self . get_saved ( ) {
        title = "*%s*" % title;
        icon = "*%s" % icon;
        self . top . wm_title ( title );
        self . top . wm_iconname ( icon );
        if macosx . isCocoaTk ( ) {
        self . top . wm_attributes ( "-titlepath" , long );
        self . top . wm_attributes ( "-modified" , !self . get_saved ( ) );
        pub fn get_saved ( self )  {
        return  self . undo . get_saved ( );
        pub fn set_saved ( &self, flag )  {
        self . undo . set_saved ( flag );
        pub fn reset_undo ( self )  {
        self . undo . reset_undo ( );
        pub fn short_title ( self )  {
        filename = self . io . filename;
        return  os . path . basename ( filename ) if filename else "untitled";
        pub fn long_title ( self )  {
        return  self . io . filename || "";
        pub fn center_insert_event ( &self, event )  {
        self . center ( );
        return  "break";
        pub fn center ( &self, mark = "insert" )  {
        text = self . text;
        top , bot = self . getwindowlines ( );
        lineno = self . getlineno ( mark );
        height = bot - top;
        newtop = max ( 1 , lineno - height / / 2 );
        text . yview ( float ( newtop ) );
        pub fn getwindowlines ( self )  {
        text = self . text;
        top = self . getlineno ( "@0,0" );
        bot = self . getlineno ( "@0,65535" );
        if top == bot && text . winfo_height ( ) == 1 {
        height = int ( text [ "height" ] );
        bot = top + height - 1;
        return  top , bot;
        pub fn getlineno ( &self, mark = "insert" )  {
        text = self . text;
        return  int ( float ( text . index ( mark ) ) );
        pub fn get_geometry ( self )  {
        "Return (width, height, x, y)";
        geom = self . top . wm_geometry ( );
        m = re . match ( r "(\d+)x(\d+)\+(-?\d+)\+(-?\d+)" , geom );
        return  list ( map ( int , m . groups ( ) ) );
        pub fn close_event ( &self, event )  {
        self . close ( );
        return  "break";
        pub fn maybesave ( self )  {
        if self . io {
        if !self . get_saved ( ) {
        if self . top . state ( ) != "normal" {
        self . top . deiconify ( );
        self . top . lower ( );
        self . top . lift ( );
        return  self . io . maybesave ( );
        pub fn close ( self )  {
        // try {
        reply = self . maybesave ( );
        if str ( reply ) != "cancel" {
        self . _close ( );
        return  reply;
        // } catch  AttributeError  {
        // pass
        pub fn _close ( self )  {
        if self . io . filename {
        self . update_recent_files_list ( new_file = self . io . filename );
        window . unregister_callback ( self . postwindowsmenu );
        self . unload_extensions ( );
        self . io . close ( );
        self . io = None /* Option */;
        self . undo = None /* Option */;
        if self . color {
        self . color . close ( );
        self . color = None /* Option */;
        self . text = None /* Option */;
        self . tkinter_vars = None /* Option */;
        self . per . close ( );
        self . per = None /* Option */;
        self . top . destroy ( );
        if self . close_hook {
        self . close_hook ( );
        pub fn load_extensions ( self )  {
        self . extensions = { };
        self . load_standard_extensions ( );
        pub fn unload_extensions ( self )  {
        for ins in list ( self . extensions . values ( ) ) .iter() {
        if hasattr ( ins , "close" ) {
        ins . close ( );
        self . extensions = { };
        pub fn load_standard_extensions ( self )  {
        for name in self . get_standard_extension_names ( ) .iter() {
        // try {
        self . load_extension ( name );
        // } catch   {
        println!( "Failed to load extension" , repr ( name ) );
        traceback . print_exc ( );
        pub fn get_standard_extension_names ( self )  {
        return  idleConf . GetExtensions ( editor_only = true );
        extfiles = {;
        "ZzDummy" : "zzdummy" ,;
        };
        pub fn load_extension ( &self, name )  {
        fname = self . extfiles . get ( name , name );
        // try {
        // try {
        mod = importlib . import_module ( "." + fname , package = __package__ );
        // } catch  ( ImportError , TypeError )  {
        mod = importlib . import_module ( fname );
        // } catch  ImportError  {
        println!( "\nFailed to import extension: " , name );
        panic!("");
        cls = getattr ( mod , name );
        keydefs = idleConf . GetExtensionBindings ( name );
        if hasattr ( cls , "menudefs" ) {
        self . fill_menus ( cls . menudefs , keydefs );
        ins = cls ( self );
        self . extensions [ name ] = ins;
        if keydefs {
        self . apply_bindings ( keydefs );
        for vevent in keydefs .iter() {
        methodname = vevent . replace ( "-" , "_" );
        while methodname [ : 1 ] == "<"  {
        methodname = methodname [ 1 : ];
        while methodname [ -1 : ] == ">"  {
        methodname = methodname [ : -1 ];
        methodname = methodname + "_event";
        if hasattr ( ins , methodname ) {
        self . text . bind ( vevent , getattr ( ins , methodname ) );
        pub fn apply_bindings ( &self, keydefs = None /* Option */ )  {
        "Add events with keys to self.text.";
        if keydefs is None /* Option */ {
        keydefs = self . mainmenu . default_keydefs;
        text = self . text;
        text . keydefs = keydefs;
        for event , keylist in keydefs . items ( ) .iter() {
        if keylist {
        text . event_add ( event , * keylist );
        pub fn fill_menus ( &self, menudefs = None /* Option */ , keydefs = None /* Option */ )  {
        "Fill in dropdown menus used by this window.

        Items whose name begins with '!' become checkbuttons.
        Other names indicate commands.  None /* Option */ becomes a separator.
        ";
        if menudefs is None /* Option */ {
        menudefs = self . mainmenu . menudefs;
        if keydefs is None /* Option */ {
        keydefs = self . mainmenu . default_keydefs;
        menudict = self . menudict;
        text = self . text;
        for mname , entrylist in menudefs .iter() {
        menu = menudict . get ( mname );
        if !menu {
        continue;
        for entry in entrylist .iter() {
        if entry is None /* Option */ {
        menu . add_separator ( );
        } else {
        label , eventname = entry;
        checkbutton = ( label [ : 1 ] == "!" );
        if checkbutton {
        label = label [ 1 : ];
        underline , label = prepstr ( label );
        accelerator = get_accelerator ( keydefs , eventname );
        pub fn command ( text = text , eventname = eventname )  {
        text . event_generate ( eventname );
        if checkbutton {
        var = self . get_var_obj ( eventname , BooleanVar );
        menu . add_checkbutton ( label = label , underline = underline ,;
        command = command , accelerator = accelerator ,;
        variable = var );
        } else {
        menu . add_command ( label = label , underline = underline ,;
        command = command ,;
        accelerator = accelerator );
        pub fn getvar ( &self, name )  {
        var = self . get_var_obj ( name );
        if var {
        value = var . get ( );
        return  value;
        } else {
        panic!("NameError ( name )");
        pub fn setvar ( &self, name , value , vartype = None /* Option */ )  {
        var = self . get_var_obj ( name , vartype );
        if var {
        var . set ( value );
        } else {
        panic!("NameError ( name )");
        pub fn get_var_obj ( &self, eventname , vartype = None /* Option */ )  {
        "Return a tkinter variable instance for the event.
        ";
        var = self . tkinter_vars . get ( eventname );
        if !var && vartype {
        self . tkinter_vars [ eventname ] = var = vartype ( self . text );
        return  var;
        pub fn is_char_in_string ( &self, text_index )  {
        if self . color {
        return  self . text . tag_prevrange ( "TODO" , text_index ) || \;
        "STRING" in self . text . tag_names ( text_index );
        } else {
        return  1;
        pub fn get_selection_indices ( self )  {
        // try {
        first = self . text . index ( "sel.first" );
        last = self . text . index ( "sel.last" );
        return  first , last;
        // } catch  TclError  {
        return  None /* Option */ , None /* Option */;
        pub fn get_tk_tabwidth ( self )  {
        current = self . text [ "tabs" ] || TK_TABWIDTH_DEFAULT;
        return  int ( current );
        pub fn set_tk_tabwidth ( &self, newtabwidth )  {
        text = self . text;
        if self . get_tk_tabwidth ( ) != newtabwidth {
        pixels = text . tk . call ( "font" , "measure" , text [ "font" ] ,;
        "-displayoformat!(" , text . master ,);
        "n" * newtabwidth );
        text . configure ( tabs = pixels );
        pub fn set_indentation_params ( &self, is_py_src , guess = true )  {
        if is_py_src && guess {
        i = self . guess_indent ( );
        if 2 <= i <= 8 {
        self . indentwidth = i;
        if self . indentwidth != self . tabwidth {
        self . usetabs = false;
        self . set_tk_tabwidth ( self . tabwidth );
        pub fn smart_backspace_event ( &self, event )  {
        text = self . text;
        first , last = self . get_selection_indices ( );
        if first && last {
        text . delete ( first , last );
        text . mark_set ( "insert" , first );
        return  "break";
        chars = text . get ( "insert linestart" , "insert" );
        if chars == "" {
        if text . compare ( "insert" , ">" , "1.0" ) {
        text . delete ( "insert-1c" );
        } else {
        text . bell ( );
        return  "break";
        if chars [ -1 ] !in " \t" {
        text . delete ( "insert-1c" );
        return  "break";
        tabwidth = self . tabwidth;
        have = len ( chars . expandtabs ( tabwidth ) );
        assert have > 0;
        want = ( ( have - 1 ) / / self . indentwidth ) * self . indentwidth;
        ncharsdeleted = 0;
        while true  {
        chars = chars [ : -1 ];
        ncharsdeleted = ncharsdeleted + 1;
        have = len ( chars . expandtabs ( tabwidth ) );
        if have <= want || chars [ -1 ] !in " \t" {
        break;
        text . undo_block_start ( );
        text . delete ( "insert-%dc" % ncharsdeleted , "insert" );
        if have < want {
        text . insert ( "insert" , " " * ( want - have ) ,;
        self . user_input_insert_tags );
        text . undo_block_stop ( );
        return  "break";
        pub fn smart_indent_event ( &self, event )  {
        text = self . text;
        first , last = self . get_selection_indices ( );
        text . undo_block_start ( );
        // try {
        if first && last {
        if index2line ( first ) != index2line ( last ) {
        return  self . fregion . indent_region_event ( event );
        text . delete ( first , last );
        text . mark_set ( "insert" , first );
        prefix = text . get ( "insert linestart" , "insert" );
        raw , effective = get_line_indent ( prefix , self . tabwidth );
        if raw == len ( prefix ) {
        self . reindent_to ( effective + self . indentwidth );
        } else {
        if self . usetabs {
        pad = "\t";
        } else {
        effective = len ( prefix . expandtabs ( self . tabwidth ) );
        n = self . indentwidth;
        pad = " " * ( n - effective % n );
        text . insert ( "insert" , pad , self . user_input_insert_tags );
        text . see ( "insert" );
        return  "break";
        // } finally {
        text . undo_block_stop ( );
        pub fn newline_and_indent_event ( &self, event )  {
        "Insert a newline && indentation after Enter keypress event.

        Properly position the cursor on the new line based on information
        from the current line.  This takes into account if the current line
        == a shell prompt, == empty, has selected text, contains a block
        opener, contains a block closer, == a continuation line, or
        == inside a string.
        ";
        text = self . text;
        first , last = self . get_selection_indices ( );
        text . undo_block_start ( );
        // try {
        if first && last {
        text . delete ( first , last );
        text . mark_set ( "insert" , first );
        line = text . get ( "insert linestart" , "insert" );
        i , n = 0 , len ( line );
        while i < n && line [ i ] in " \t"  {
        i + = 1;
        if i == n {
        text . insert ( "insert linestart" , "\n" ,;
        self . user_input_insert_tags );
        return  "break";
        indent = line [ : i ];
        i = 0;
        while line && line [ -1 ] in " \t"  {
        line = line [ : -1 ];
        i + = 1;
        if i {
        text . delete ( "insert - %d chars" % i , "insert" );
        while text . get ( "insert" ) in " \t"  {
        text . delete ( "insert" );
        text . insert ( "insert" , "\n" , self . user_input_insert_tags );
        lno = index2line ( text . index ( "insert" ) );
        y = pyparse . Parser ( self . indentwidth , self . tabwidth );
        if !self . prompt_last_line {
        for context in self . num_context_lines .iter() {
        startat = max ( lno - context , 1 );
        startatindex = repr ( startat ) + ".0";
        rawtext = text . get ( startatindex , "insert" );
        y . set_code ( rawtext );
        bod = y . find_good_parse_start (;
        self . _build_char_in_string_func ( startatindex ) );
        if bod is !None /* Option */ || startat == 1 {
        break;
        y . set_lo ( bod || 0 );
        } else {
        r = text . tag_prevrange ( "console" , "insert" );
        if r {
        startatindex = r [ 1 ];
        } else {
        startatindex = "1.0";
        rawtext = text . get ( startatindex , "insert" );
        y . set_code ( rawtext );
        y . set_lo ( 0 );
        c = y . get_continuation_type ( );
        if c != pyparse . C_NONE {
        if c == pyparse . C_STRING_FIRST_LINE {
        // pass
        } else if c == pyparse . C_STRING_NEXT_LINES {
        text . insert ( "insert" , indent , self . user_input_insert_tags );
        } else if c == pyparse . C_BRACKET {
        self . reindent_to ( y . compute_bracket_indent ( ) );
        } else if c == pyparse . C_BACKSLASH {
        if y . get_num_lines_in_stmt ( ) > 1 {
        text . insert ( "insert" , indent ,;
        self . user_input_insert_tags );
        } else {
        self . reindent_to ( y . compute_backslash_indent ( ) );
        } else {
        assert 0 , format!("bogus continuation type {c!r}");
        return  "break";
        indent = y . get_base_indent_string ( );
        text . insert ( "insert" , indent , self . user_input_insert_tags );
        if y . is_block_opener ( ) {
        self . smart_indent_event ( event );
        } else if indent && y . is_block_closer ( ) {
        self . smart_backspace_event ( event );
        return  "break";
        // } finally {
        text . see ( "insert" );
        text . undo_block_stop ( );
        pub fn _build_char_in_string_func ( &self, startindex )  {
        pub fn inner ( offset , _startindex = startindex , {
        _icis = self . is_char_in_string ) ;
        return  _icis ( _startindex + "+%dc" % offset );
        return  inner;
        pub fn _make_blanks ( &self, n )  {
        if self . usetabs {
        ntabs , nspaces = divmod ( n , self . tabwidth );
        return  "\t" * ntabs + " " * nspaces;
        } else {
        return  " " * n;
        pub fn reindent_to ( &self, column )  {
        text = self . text;
        text . undo_block_start ( );
        if text . compare ( "insert linestart" , "!=" , "insert" ) {
        text . delete ( "insert linestart" , "insert" );
        if column {
        text . insert ( "insert" , self . _make_blanks ( column ) ,;
        self . user_input_insert_tags );
        text . undo_block_stop ( );
        pub fn guess_indent ( self )  {
        opener , indented = IndentSearcher ( self . text ) . run ( );
        if opener && indented {
        raw , indentsmall = get_line_indent ( opener , self . tabwidth );
        raw , indentlarge = get_line_indent ( indented , self . tabwidth );
        } else {
        indentsmall = indentlarge = 0;
        return  indentlarge - indentsmall;
        pub fn toggle_line_numbers_event ( &self, event = None /* Option */ )  {
        if self . line_numbers is None /* Option */ {
        return;
        if self . line_numbers . is_shown {
        self . line_numbers . hide_sidebar ( );
        menu_label = "Show";
        } else {
        self . line_numbers . show_sidebar ( );
        menu_label = "Hide";
        self . update_menu_label ( menu = "options" , index = "*ine*umbers" ,;
        label = format!("{menu_label} Line Numbers" ));
        pub fn index2line ( index )  {
        return  int ( float ( index ) );
        _line_indent_re = re . compile ( r "[ \t]*" );
        pub fn get_line_indent ( line , tabwidth )  {
        "Return a line's indentation as (# chars, effective # of spaces).

    The effective # of spaces == the length after properly "expanding"
    the tabs into spaces, as done by str.expandtabs(tabwidth).
    ";
        m = _line_indent_re . match ( line );
        return  m . end ( ) , len ( m . group ( ) . expandtabs ( tabwidth ) );
        class IndentSearcher ;
        "Manage initial indent guess, returned by run method.";
        pub fn __init__ ( &self, text )  {
        self . text = text;
        self . i = self . finished = 0;
        self . blkopenline = self . indentedline = None /* Option */;
        pub fn readline ( self )  {
        if self . finished {
        return  "";
        i = self . i = self . i + 1;
        mark = repr ( i ) + ".0";
        if self . text . compare ( mark , ">=" , "end" ) {
        return  "";
        return  self . text . get ( mark , mark + " lineend+1c" );
        pub fn tokeneater ( &self, type , token , start , end , line , {
        INDENT = tokenize . INDENT ,;
        NAME = tokenize . NAME ,;
        OPENERS = ( "class" , "deformat!(" , "for" , "iformat!(" , "match" , "try" ,);
        "while" , "with" ) ) ;
        if self . finished {
        // pass
        } else if type == NAME && token in OPENERS {
        self . blkopenline = line;
        } else if type == INDENT && self . blkopenline {
        self . indentedline = line;
        self . finished = 1;
        pub fn run ( self )  {
        "Return 2 lines containing block opener && and indent.

        Either the indent line || both may be None /* Option */.
        ";
        // try {
        tokens = tokenize . generate_tokens ( self . readline );
        for token in tokens .iter() {
        self . tokeneater ( * token );
        // } catch  ( tokenize . TokenError , SyntaxError )  {
        // pass
        return  self . blkopenline , self . indentedline;
        pub fn prepstr ( s )  {
        "Extract the underscore from a string.

    For example, prepstr("Co_py") returns (2, "Copy").

    Args:
        s: String with underscore.

    Returns:
        Tuple of (position of underscore, string without underscore).
    ";
        i = s . find ( "_" );
        if i >= 0 {
        s = s [ : i ] + s [ i + 1 : ];
        return  i , s;
        keynames = {;
        "bracketleft" : "[" ,;
        "bracketright" : "]" ,;
        "slash" : "/" ,;
        };
        pub fn get_accelerator ( keydefs , eventname )  {
        "Return a formatted string for the keybinding of an event.

    Convert the first keybinding for a given event to a form that
    can be displayed as an accelerator on the menu.

    Args:
        keydefs: Dictionary of valid events to keybindings.
        eventname: Event to retrieve keybinding for.

    Returns:
        Formatted string of the keybinding.
    ";
        keylist = keydefs . get ( eventname );
        if ( !keylist ) || ( macosx . isCocoaTk ( ) && eventname in { {
        "<<open-module>>" ,;
        "<<goto-line>>" ,;
        "<<change-indentwidth>>" } ) ;
        return  "";
        s = keylist [ 0 ];
        s = re . sub ( r "-[a-z]\b" , |m | {  m . group ( ) . upper ( ) , s ) };
        s = re . sub ( r "\b\w+\b" , |m | {  keynames . get ( m . group ( ) , m . group ( ) ) , s ) };
        s = re . sub ( "Key-" , "" , s );
        s = re . sub ( "Cancel" , "Ctrl-Break" , s );
        s = re . sub ( "Control-" , "Ctrl-" , s );
        s = re . sub ( "-" , "+" , s );
        s = re . sub ( "><" , " " , s );
        s = re . sub ( "<" , "" , s );
        s = re . sub ( ">" , "" , s );
        return  s;
        pub fn fixwordbreaks ( root )  {
        tk = root . tk;
        tk . call ( "tcl_wordBreakAfter" , "a b" , 0 );
        tk . call ( "set" , "tcl_wordchars" , r "\w" );
        tk . call ( "set" , "tcl_nonwordchars" , r "\W" );
        pub fn _editor_window ( parent )  {
        root = parent;
        fixwordbreaks ( root );
        if sys . argv [ 1 { : ] ; }
        filename = sys . argv [ 1 ];
        } else {
        filename = None /* Option */;
        macosx . setupApp ( root , None /* Option */ );
        edit = EditorWindow ( root = root , filename = filename );
        text = edit . text;
        text [ "height" ] = 10;
        for i in range ( 20 ) .iter() {
        text . insert ( "insert" , "  " * i + str ( i ) + "\n" );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_editor" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _editor_window );
}

