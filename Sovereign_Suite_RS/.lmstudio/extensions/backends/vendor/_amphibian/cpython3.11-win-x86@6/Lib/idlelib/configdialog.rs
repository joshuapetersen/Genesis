//! configdialog.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter::{Toplevel, Listbox, Canvas};
// use crate::messagebox;
// use crate::idlelib::{idleConf, ConfigChanges};
// use crate::unittest::{main};

pub const changes: f64 = ConfigChanges ( );
pub const reloadables: f64 = ( AutoComplete , CodeContext , ParenMatch , FormatParagraph ,;
pub struct ConfigDialog {
    pub parent: String, // TODO: infer type
    pub frame: String, // TODO: infer type
    pub note: String, // TODO: infer type
    pub extpage: String, // TODO: infer type
    pub highpage: String, // TODO: infer type
    pub fontpage: String, // TODO: infer type
    pub keyspage: String, // TODO: infer type
    pub winpage: String, // TODO: infer type
    pub shedpage: String, // TODO: infer type
    pub buttons: String, // TODO: infer type
    pub highlight_sample: String, // TODO: infer type
    pub font_name: String, // TODO: infer type
    pub font_size: String, // TODO: infer type
    pub font_bold: String, // TODO: infer type
    pub fontlist: String, // TODO: infer type
    pub sizelist: String, // TODO: infer type
    pub bold_toggle: String, // TODO: infer type
    pub font_sample: String, // TODO: infer type
    pub cd: String, // TODO: infer type
    pub style: String, // TODO: infer type
    pub theme_elements: String, // TODO: infer type
    pub builtin_name: String, // TODO: infer type
    pub custom_name: String, // TODO: infer type
    pub fg_bg_toggle: String, // TODO: infer type
    pub color: String, // TODO: infer type
    pub theme_source: String, // TODO: infer type
    pub highlight_target: String, // TODO: infer type
    pub frame_color_set: String, // TODO: infer type
    pub button_set_color: String, // TODO: infer type
    pub targetlist: String, // TODO: infer type
    pub fg_on: String, // TODO: infer type
    pub bg_on: String, // TODO: infer type
    pub button_save_custom: String, // TODO: infer type
    pub builtin_theme_on: String, // TODO: infer type
    pub custom_theme_on: String, // TODO: infer type
    pub builtinlist: String, // TODO: infer type
    pub customlist: String, // TODO: infer type
    pub button_delete_custom: String, // TODO: infer type
    pub theme_message: String, // TODO: infer type
    pub keyset_source: String, // TODO: infer type
    pub keybinding: String, // TODO: infer type
    pub bindingslist: String, // TODO: infer type
    pub button_new_keys: String, // TODO: infer type
    pub builtin_keyset_on: String, // TODO: infer type
    pub custom_keyset_on: String, // TODO: infer type
    pub button_delete_custom_keys: String, // TODO: infer type
    pub button_save_custom_keys: String, // TODO: infer type
    pub keys_message: String, // TODO: infer type
    pub digits_only: String, // TODO: infer type
    pub startup_edit: String, // TODO: infer type
    pub win_width: String, // TODO: infer type
    pub win_height: String, // TODO: infer type
    pub indent_spaces: String, // TODO: infer type
    pub cursor_blink: String, // TODO: infer type
    pub autocomplete_wait: String, // TODO: infer type
    pub paren_style: String, // TODO: infer type
    pub flash_delay: String, // TODO: infer type
    pub paren_bell: String, // TODO: infer type
    pub format_width: String, // TODO: infer type
    pub startup_editor_on: String, // TODO: infer type
    pub startup_shell_on: String, // TODO: infer type
    pub win_width_int: String, // TODO: infer type
    pub win_height_int: String, // TODO: infer type
    pub indent_chooser: String, // TODO: infer type
    pub cursor_blink_bool: String, // TODO: infer type
    pub auto_wait_int: String, // TODO: infer type
    pub paren_style_type: String, // TODO: infer type
    pub paren_flash_time: String, // TODO: infer type
    pub bell_on: String, // TODO: infer type
    pub format_width_int: String, // TODO: infer type
    pub auto_squeeze_min_lines: String, // TODO: infer type
    pub autosave: String, // TODO: infer type
    pub line_numbers_default: String, // TODO: infer type
    pub context_lines: String, // TODO: infer type
    pub auto_squeeze_min_lines_int: String, // TODO: infer type
    pub save_ask_on: String, // TODO: infer type
    pub save_auto_on: String, // TODO: infer type
    pub line_numbers_default_bool: String, // TODO: infer type
    pub context_int: String, // TODO: infer type
    pub ext_defaultCfg: String, // TODO: infer type
    pub ext_userCfg: String, // TODO: infer type
    pub is_int: String, // TODO: infer type
    pub extension_names: String, // TODO: infer type
    pub frame_help: String, // TODO: infer type
    pub extension_list: String, // TODO: infer type
    pub details_frame: String, // TODO: infer type
    pub config_frame: String, // TODO: infer type
    pub current_extension: String, // TODO: infer type
    pub outerframe: String, // TODO: infer type
    pub tabbed_page_set: String, // TODO: infer type
    pub extensions: String, // TODO: infer type
    pub helplist: String, // TODO: infer type
    pub button_helplist_edit: String, // TODO: infer type
    pub button_helplist_add: String, // TODO: infer type
    pub button_helplist_remove: String, // TODO: infer type
    pub user_helplist: String, // TODO: infer type
    pub untraced: String, // TODO: infer type
    pub traced: String, // TODO: infer type
    pub interior: String, // TODO: infer type
}

impl ConfigDialog {
}

pub const font_sample_text: f64 = (;
pub struct FontPage {
    pub highlight_sample: String, // TODO: infer type
    pub font_name: String, // TODO: infer type
    pub font_size: String, // TODO: infer type
    pub font_bold: String, // TODO: infer type
    pub fontlist: String, // TODO: infer type
    pub sizelist: String, // TODO: infer type
    pub bold_toggle: String, // TODO: infer type
    pub font_sample: String, // TODO: infer type
    pub extpage: String, // TODO: infer type
    pub cd: String, // TODO: infer type
    pub style: String, // TODO: infer type
    pub theme_elements: String, // TODO: infer type
    pub builtin_name: String, // TODO: infer type
    pub custom_name: String, // TODO: infer type
    pub fg_bg_toggle: String, // TODO: infer type
    pub color: String, // TODO: infer type
    pub theme_source: String, // TODO: infer type
    pub highlight_target: String, // TODO: infer type
    pub frame_color_set: String, // TODO: infer type
    pub button_set_color: String, // TODO: infer type
    pub targetlist: String, // TODO: infer type
    pub fg_on: String, // TODO: infer type
    pub bg_on: String, // TODO: infer type
    pub button_save_custom: String, // TODO: infer type
    pub builtin_theme_on: String, // TODO: infer type
    pub custom_theme_on: String, // TODO: infer type
    pub builtinlist: String, // TODO: infer type
    pub customlist: String, // TODO: infer type
    pub button_delete_custom: String, // TODO: infer type
    pub theme_message: String, // TODO: infer type
    pub keyset_source: String, // TODO: infer type
    pub keybinding: String, // TODO: infer type
    pub bindingslist: String, // TODO: infer type
    pub button_new_keys: String, // TODO: infer type
    pub builtin_keyset_on: String, // TODO: infer type
    pub custom_keyset_on: String, // TODO: infer type
    pub button_delete_custom_keys: String, // TODO: infer type
    pub button_save_custom_keys: String, // TODO: infer type
    pub keys_message: String, // TODO: infer type
    pub digits_only: String, // TODO: infer type
    pub startup_edit: String, // TODO: infer type
    pub win_width: String, // TODO: infer type
    pub win_height: String, // TODO: infer type
    pub indent_spaces: String, // TODO: infer type
    pub cursor_blink: String, // TODO: infer type
    pub autocomplete_wait: String, // TODO: infer type
    pub paren_style: String, // TODO: infer type
    pub flash_delay: String, // TODO: infer type
    pub paren_bell: String, // TODO: infer type
    pub format_width: String, // TODO: infer type
    pub startup_editor_on: String, // TODO: infer type
    pub startup_shell_on: String, // TODO: infer type
    pub win_width_int: String, // TODO: infer type
    pub win_height_int: String, // TODO: infer type
    pub indent_chooser: String, // TODO: infer type
    pub cursor_blink_bool: String, // TODO: infer type
    pub auto_wait_int: String, // TODO: infer type
    pub paren_style_type: String, // TODO: infer type
    pub paren_flash_time: String, // TODO: infer type
    pub bell_on: String, // TODO: infer type
    pub format_width_int: String, // TODO: infer type
    pub auto_squeeze_min_lines: String, // TODO: infer type
    pub autosave: String, // TODO: infer type
    pub line_numbers_default: String, // TODO: infer type
    pub context_lines: String, // TODO: infer type
    pub auto_squeeze_min_lines_int: String, // TODO: infer type
    pub save_ask_on: String, // TODO: infer type
    pub save_auto_on: String, // TODO: infer type
    pub line_numbers_default_bool: String, // TODO: infer type
    pub context_int: String, // TODO: infer type
    pub ext_defaultCfg: String, // TODO: infer type
    pub ext_userCfg: String, // TODO: infer type
    pub is_int: String, // TODO: infer type
    pub extension_names: String, // TODO: infer type
    pub frame_help: String, // TODO: infer type
    pub extension_list: String, // TODO: infer type
    pub details_frame: String, // TODO: infer type
    pub config_frame: String, // TODO: infer type
    pub current_extension: String, // TODO: infer type
    pub outerframe: String, // TODO: infer type
    pub tabbed_page_set: String, // TODO: infer type
    pub extensions: String, // TODO: infer type
    pub helplist: String, // TODO: infer type
    pub button_helplist_edit: String, // TODO: infer type
    pub button_helplist_add: String, // TODO: infer type
    pub button_helplist_remove: String, // TODO: infer type
    pub user_helplist: String, // TODO: infer type
    pub untraced: String, // TODO: infer type
    pub traced: String, // TODO: infer type
    pub interior: String, // TODO: infer type
}

impl FontPage {
    pub fn new(master: &str, highpage: &str) -> Self {
        super ( ) . __init__ ( master );
        self . highlight_sample = highpage . highlight_sample;
        self . create_page_font ( );
        self . load_font_cfg ( );
    }

    pub fn is_int(&self, s: &str) {
        "Return 's == blank || represents an int'";
        if !s {
        return  true;
        // try {
        int ( s );
        return  true;
        // } catch  ValueError  {
        return  false;
        class VerticalScrolledFrame ( Frame ) ;
        "A pure Tkinter vertically scrollable frame.

    * Use the 'interior' attribute to place widgets inside the scrollable frame
    * Construct && pack/place/grid normally
    * This frame only allows vertical scrolling
    ";
        pub fn __init__ ( &self, parent , * args , ** kw )  {
        Frame . __init__ ( self , parent , * args , ** kw );
        vscrollbar = Scrollbar ( self , orient = VERTICAL );
        vscrollbar . pack ( fill = Y , side = RIGHT , expand = FALSE );
        canvas = Canvas ( self , borderwidth = 0 , highlightthickness = 0 ,;
        yscrollcommand = vscrollbar . set , width = 240 );
        canvas . pack ( side = LEFT , fill = BOTH , expand = TRUE );
        vscrollbar . config ( command = canvas . yview );
        canvas . xview_moveto ( 0 );
        canvas . yview_moveto ( 0 );
        self . interior = interior = Frame ( canvas );
        interior_id = canvas . create_window ( 0 , 0 , window = interior , anchor = NW );
        pub fn _configure_interior ( event )  {
        size = ( interior . winfo_reqwidth ( ) , interior . winfo_reqheight ( ) );
        canvas . config ( scrollregion = "0 0 %s %s" % size );
        interior . bind ( "<Configure>" , _configure_interior );
        pub fn _configure_canvas ( event )  {
        if interior . winfo_reqwidth ( ) != canvas . winfo_width ( ) {
        canvas . itemconfigure ( interior_id , width = canvas . winfo_width ( ) );
        canvas . bind ( "<Configure>" , _configure_canvas );
        return;
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_configdialog" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( ConfigDialog );
    }

}

