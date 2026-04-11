//! undo.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::string;
// use crate::idlelib::{Delegator};
// use crate::pprint::{pprint};
// use crate::tkinter::{Toplevel, Text, Button};
// use crate::unittest::{main};

pub struct UndoDelegator {
    pub was_saved: String, // TODO: infer type
    pub pointer: String, // TODO: infer type
    pub undolist: String, // TODO: infer type
    pub undoblock: String, // TODO: infer type
    pub saved: String, // TODO: infer type
    pub can_merge: String, // TODO: infer type
    pub saved_change_hook: String, // TODO: infer type
    pub marks_before: String, // TODO: infer type
    pub marks_after: String, // TODO: infer type
    pub index1: String, // TODO: infer type
    pub index2: String, // TODO: infer type
    pub chars: String, // TODO: infer type
    pub tags: String, // TODO: infer type
    pub cmds: String, // TODO: infer type
    pub depth: String, // TODO: infer type
}

impl UndoDelegator {
}

pub struct Command {
    pub marks_before: String, // TODO: infer type
    pub marks_after: String, // TODO: infer type
    pub index1: String, // TODO: infer type
    pub index2: String, // TODO: infer type
    pub chars: String, // TODO: infer type
    pub tags: String, // TODO: infer type
    pub cmds: String, // TODO: infer type
    pub depth: String, // TODO: infer type
}

impl Command {
}

pub struct InsertCommand {
    pub marks_before: String, // TODO: infer type
    pub index1: String, // TODO: infer type
    pub index2: String, // TODO: infer type
    pub marks_after: String, // TODO: infer type
    pub chars: String, // TODO: infer type
    pub cmds: String, // TODO: infer type
    pub depth: String, // TODO: infer type
}

impl InsertCommand {
}

pub struct DeleteCommand {
    pub marks_before: String, // TODO: infer type
    pub index1: String, // TODO: infer type
    pub index2: String, // TODO: infer type
    pub chars: String, // TODO: infer type
    pub marks_after: String, // TODO: infer type
    pub cmds: String, // TODO: infer type
    pub depth: String, // TODO: infer type
}

impl DeleteCommand {
}

pub struct CommandSequence {
    pub cmds: String, // TODO: infer type
    pub depth: String, // TODO: infer type
}

impl CommandSequence {
}

pub fn _undo_delegator(parent: &str) {
        from tkinter import Toplevel , Text , Button;
        from idlelib . percolator import Percolator;
        top = Toplevel ( parent );
        top . title ( "Test UndoDelegator" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 175 ) );
        text = Text ( top , height = 10 );
        text . pack ( );
        text . focus_set ( );
        p = Percolator ( text );
        d = UndoDelegator ( );
        p . insertfilter ( d );
        undo = Button ( top , text = "Undo" , command = || {  d . undo_event ( None /* Option */ ) ) };
        undo . pack ( side = "left" );
        redo = Button ( top , text = "Redo" , command = || {  d . redo_event ( None /* Option */ ) ) };
        redo . pack ( side = "left" );
        dump = Button ( top , text = "Dump" , command = || {  d . dump_event ( None /* Option */ ) ) };
        dump . pack ( side = "left" );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_undo" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _undo_delegator );
}

