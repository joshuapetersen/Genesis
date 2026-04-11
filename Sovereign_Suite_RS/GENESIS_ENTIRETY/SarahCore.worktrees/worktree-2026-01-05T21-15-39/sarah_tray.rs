//! sarah_tray.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::webbrowser;
// use crate::requests;
// use crate::pathlib::{Path};
// use crate::pystray;
// use crate::MenuItem;
// use crate::Image;

pub const API_BASE: &str = "http://127.0.0.1:8000";
pub const DASH_URL: &str = f"{API_BASE}/ui";
pub fn create_image() {
        img = Image . new ( "RGB" , ( 64 , 64 ) , color = ( 0 , 0 , 0 , 0 ) );
        d = ImageDraw . Draw ( img );
        d . ellipse ( ( 8 , 8 , 56 , 56 ) , fill = ( 59 , 130 , 246 ) );
        d . ellipse ( ( 18 , 18 , 46 , 46 ) , fill = ( 255 , 255 , 255 ) );
        return  img;
        pub fn open_dashboard ( icon , item )  {
        webbrowser . open ( DASH_URL );
        pub fn send_command ( cmd  {  str ) ; }
        // try {
        r = requests . post ( format!("{API_BASE}/command" , json = { "command" : cmd } , timeout = 5 ));
        if r . ok {
        return  r . json ( );
        } else {
        return  { "error" : r . text };
        // } catch  Exception as e  {
        return  { "error" : str ( e ) };
        pub fn quick_command ( icon , item , cmd )  {
        send_command ( cmd );
        pub fn quit_app ( icon , item )  {
        icon . stop ( );
        pub fn build_menu ( )  {
        return  (;
        item ( "Open Dashboard" , open_dashboard ) ,;
        item ( "Begin Linux Assimilation" , |icon , i | {  quick_command ( icon , i , "Begin Linux Assimilation" ) ) , };
        item ( "List USB" , |icon , i | {  quick_command ( icon , i , "list usb devices" ) ) , };
        item ( "Refresh Handshake" , |icon , i | {  quick_command ( icon , i , "platform bridge status" ) ) , };
        item ( "Quit" , quit_app ) ,;
        );
        pub fn main ( )  {
        icon = pystray . Icon ( "SarahPrime" , create_image ( ) , "Sarah Prime" , menu = pystray . Menu ( * build_menu ( ) ) );
        icon . run ( );
        fn main() {
        main ( );
}

