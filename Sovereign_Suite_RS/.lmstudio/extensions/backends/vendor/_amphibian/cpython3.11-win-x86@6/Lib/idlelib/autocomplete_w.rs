//! autocomplete_w.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::platform;
// use crate::tkinter::{};
// use crate::idlelib::{FILES, ATTRS};
// use crate::unittest::{main};

pub const HIDE_VIRTUAL_EVENT_NAME: &str = "<<autocompletewindow-hide>>";
pub const HIDE_FOCUS_OUT_SEQUENCE: &str = "<FocusOut>";
pub const HIDE_SEQUENCES: &str = ( HIDE_FOCUS_OUT_SEQUENCE ,"<ButtonPress>" );
pub const KEYPRESS_VIRTUAL_EVENT_NAME: &str = "<<autocompletewindow-keypress>>";
pub const KEYPRESS_SEQUENCES: &str = ("<Key>" ,"<Key-BackSpace>" ,"<Key-Return>" ,"<Key-Tab>" ,;
pub const KEYRELEASE_VIRTUAL_EVENT_NAME: &str = "<<autocompletewindow-keyrelease>>";
pub const KEYRELEASE_SEQUENCE: &str = "<KeyRelease>";
pub const LISTUPDATE_SEQUENCE: &str = "<B1-ButtonRelease>";
pub const WINCONFIG_SEQUENCE: &str = "<Configure>";
pub const DOUBLECLICK_SEQUENCE: &str = "<B1-Double-ButtonRelease>";
pub struct AutoCompleteWindow {
    pub widget: String, // TODO: infer type
    pub tags: String, // TODO: infer type
    pub autocompletewindow: String, // TODO: infer type
    pub listbox: String, // TODO: infer type
    pub scrollbar: String, // TODO: infer type
    pub origselforeground: String, // TODO: infer type
    pub origselbackground: String, // TODO: infer type
    pub completions: String, // TODO: infer type
    pub morecompletions: String, // TODO: infer type
    pub mode: String, // TODO: infer type
    pub start: String, // TODO: infer type
    pub startindex: String, // TODO: infer type
    pub lasttypedstart: String, // TODO: infer type
    pub userwantswindow: String, // TODO: infer type
    pub hideid: String, // TODO: infer type
    pub keypressid: String, // TODO: infer type
    pub listupdateid: String, // TODO: infer type
    pub winconfigid: String, // TODO: infer type
    pub keyreleaseid: String, // TODO: infer type
    pub doubleclickid: String, // TODO: infer type
    pub lastkey_was_tab: String, // TODO: infer type
    pub is_configuring: String, // TODO: infer type
    pub hideaid: String, // TODO: infer type
    pub hidewid: String, // TODO: infer type
}

impl AutoCompleteWindow {
    pub fn new(widget: &str, tags: &str) -> Self {
        self . widget = widget;
        self . tags = tags;
        self . autocompletewindow = self . listbox = self . scrollbar = None /* Option */;
        self . origselforeground = self . origselbackground = None /* Option */;
        self . completions = None /* Option */;
        self . morecompletions = None /* Option */;
        self . mode = None /* Option */;
        self . start = None /* Option */;
        self . startindex = None /* Option */;
        self . lasttypedstart = None /* Option */;
        self . userwantswindow = None /* Option */;
        self . hideid = self . keypressid = self . listupdateid = \;
        self . winconfigid = self . keyreleaseid = self . doubleclickid = None /* Option */;
        self . lastkey_was_tab = false;
        self . is_configuring = false;
    }

}

