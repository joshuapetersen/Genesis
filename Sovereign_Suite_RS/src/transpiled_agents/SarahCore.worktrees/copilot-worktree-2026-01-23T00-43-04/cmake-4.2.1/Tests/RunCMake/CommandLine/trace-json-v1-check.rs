//! trace-json-v1-check.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use serde_json;
// use std::env;

pub const trace_file: f64 = None;
pub const expand: f64 = False;
pub const required_traces: f64 = [;
pub fn assert_fields_look_good(line: &str) {
        expected_fields = { "args" , "cmd" , "file" , "frame" , "global_frame" , "line" , "time" };
        if "line_end" in line {
        assert isinstance ( line [ "line_end" ] , int );
        assert line [ "line" ] != line [ "line_end" ];
        expected_fields . add ( "line_end" );
        assert set ( line . keys ( ) ) == expected_fields;
        assert isinstance ( line [ "args" ] , list );
        assert isinstance ( line [ "cmd" ] , unicode );
        assert isinstance ( line [ "file" ] , unicode );
        assert isinstance ( line [ "frame" ] , int );
        assert isinstance ( line [ "global_frame" ] , int );
        assert isinstance ( line [ "line" ] , int );
        assert isinstance ( line [ "time" ] , float );
        with open ( trace_file , "r" ) as fp ;
        vers = json . loads ( fp . readline ( ) );
        assert sorted ( vers . keys ( ) ) == [ "version" ];
        assert sorted ( vers [ "version" ] . keys ( ) ) == [ "major" , "minor" ];
        assert vers [ "version" ] [ "major" ] == 1;
        assert vers [ "version" ] [ "minor" ] == 2;
        for i in fp . readlines ( ) .iter() {
        line = json . loads ( i );
        assert_fields_look_good ( line );
        for j in required_traces .iter() {
        subset = {;
        k : line [ k ];
        for k in j.iter() {
        if k in line {
        };
        if subset == j {
        required_traces . remove ( j );
        assert !required_traces , (;
        "The following traces were expected to be part of the ";
        "output but weren't" , required_traces;
        );
}

