//! cmake.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::dataclasses::{dataclass};
// use /* typing */::{Any, List, Tuple, Type, cast};
// use crate::sphinx;
// use crate::docutils::{io, nodes};
// use crate::pygments::{bygroups};

pub const logger: f64 = logging . getLogger ( __name__ );
pub const sig_end_re: &str = re . compile ( r"(?<=[)])\n" );
pub struct ObjectEntry {
    pub re_start: String, // TODO: infer type
    pub desc: String, // TODO: infer type
    pub titles: String, // TODO: infer type
    pub targetname: String, // TODO: infer type
    pub targetnames: String, // TODO: infer type
    pub break_style: String, // TODO: infer type
}

impl ObjectEntry {
}

pub struct CMakeModule {
    pub re_start: String, // TODO: infer type
    pub desc: String, // TODO: infer type
    pub titles: String, // TODO: infer type
    pub targetname: String, // TODO: infer type
    pub targetnames: String, // TODO: infer type
    pub break_style: String, // TODO: infer type
}

impl CMakeModule {
}

pub struct _cmake_index_entry {
    pub desc: String, // TODO: infer type
    pub titles: String, // TODO: infer type
    pub targetname: String, // TODO: infer type
    pub targetnames: String, // TODO: infer type
    pub break_style: String, // TODO: infer type
}

impl _cmake_index_entry {
    pub fn new(desc: &str) -> Self {
        self . desc = desc;
    }

    pub fn setup(&self, app: &str) {
        app . add_directive ( "cmake-module" , CMakeModule );
        app . add_transform ( CMakeTransform );
        app . add_transform ( CMakeXRefTransform );
        app . add_domain ( CMakeDomain );
        return { "parallel_read_safe" : true };
    }

}

