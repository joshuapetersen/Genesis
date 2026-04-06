//! configureLog-v1-check.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::check_index::{};
// use std::fs;

pub fn check_objects(o: &str) {
        assert is_list ( o );
        assert len ( o ) == 1;
        check_index_object ( o [ 0 ] , "configureLog" , 1 , 0 , check_object_configureLog );
        pub fn check_object_configureLog ( o ) {
        assert sorted ( o . keys ( ) ) == [ "eventKindNames" , "kind" , "path" , "version" ];
        path = o [ "path" ];
        assert matches ( path , "^.*/CMakeFiles/CMakeConfigureLog\\.yaml$" );
        assert os . path . exists ( path );
        eventKindNames = o [ "eventKindNames" ];
        assert is_list ( eventKindNames );
        assert sorted ( eventKindNames ) == [ "find-v1" , "find_package-v1" , "message-v1" , "try_compile-v1" , "try_run-v1" ];
        assert is_dict ( index );
        assert sorted ( index . keys ( ) ) == [ "cmake" , "objects" , "reply" ];
        check_objects ( index [ "objects" ] );
}

