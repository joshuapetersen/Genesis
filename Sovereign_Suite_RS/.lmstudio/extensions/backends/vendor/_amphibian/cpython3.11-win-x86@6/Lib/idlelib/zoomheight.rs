//! zoomheight.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use regex::Regex;
// use crate::tkinter;
// use crate::unittest::{main};

pub struct WmInfoGatheringError {
    pub editwin: String, // TODO: infer type
    pub top: String, // TODO: infer type
}

impl WmInfoGatheringError {
}

pub struct ZoomHeight {
    pub editwin: String, // TODO: infer type
    pub top: String, // TODO: infer type
}

impl ZoomHeight {
}

pub fn get_window_geometry(top: &str) {
        geom = top . wm_geometry ( );
        m = re . match ( r "(\d+)x(\d+)\+(-?\d+)\+(-?\d+)" , geom );
        return  tuple ( map ( int , m . groups ( ) ) );
        pub fn set_window_geometry ( top , geometry )  {
        top . wm_geometry ( "{:d}x{:d}+{:d}+{:d}" . format ( * geometry ) );
        fn main() {
        from unittest import main;
        main ( "idlelib.idle_test.test_zoomheight" , verbosity = 2 , exit = false );
}

