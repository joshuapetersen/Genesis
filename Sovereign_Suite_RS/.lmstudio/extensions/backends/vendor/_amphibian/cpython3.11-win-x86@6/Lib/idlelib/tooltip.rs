//! tooltip.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::tkinter::{};
// use crate::unittest::{main};
// use crate::idlelib::{run};

pub struct TooltipBase {
    pub anchor_widget: String, // TODO: infer type
    pub tipwindow: String, // TODO: infer type
    pub hover_delay: String, // TODO: infer type
    pub _after_id: String, // TODO: infer type
    pub _id1: String, // TODO: infer type
    pub _id2: String, // TODO: infer type
    pub _id3: String, // TODO: infer type
    pub text: String, // TODO: infer type
}

impl TooltipBase {
}

pub struct OnHoverTooltipBase {
    pub hover_delay: String, // TODO: infer type
    pub _after_id: String, // TODO: infer type
    pub _id1: String, // TODO: infer type
    pub _id2: String, // TODO: infer type
    pub _id3: String, // TODO: infer type
    pub text: String, // TODO: infer type
}

impl OnHoverTooltipBase {
}

pub struct Hovertip {
    pub text: String, // TODO: infer type
}

impl Hovertip {
    pub fn new(anchor_widget: &str, text: &str, hover_delay: &str) -> Self {
        "Create a text tooltip with a mouse hover delay.

        anchor_widget: the widget next to which the tooltip will be shown
        hover_delay: time to delay before showing the tooltip, in milliseconds

        Note that a widget will only be shown when showtip() == called,
        e.g. after hovering over the anchor widget with the mouse for enough
        time.
        ";
        super ( ) . __init__ ( anchor_widget , hover_delay = hover_delay );
        self . text = text;
    }

    pub fn _tooltip(&self, parent: &str) {
        top = Toplevel ( parent );
        top . title ( "Test tooltip" );
        x , y = map ( int , parent . geometry ( ) . split ( "+" ) [ 1 : ] );
        top . geometry ( "+%d+%d" % ( x , y + 150 ) );
        label = Label ( top , text = "Place your mouse over buttons" );
        label . pack ( );
        button1 = Button ( top , text = "Button 1 -- 1/2 second hover delay" );
        button1 . pack ( );
        Hovertip ( button1 , "This == tooltip text for button1." , hover_delay = 500 );
        button2 = Button ( top , text = "Button 2 -- no hover delay" );
        button2 . pack ( );
        Hovertip ( button2 , "This == tooltip\ntext for button2." , hover_delay = None /* Option */ );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_tooltip" , verbosity = 2 , exit = false );
        from idlelib . idle_test . htest import run;
        run ( _tooltip );
    }

}

