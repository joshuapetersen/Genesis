//! test_millisecond_timing.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::pytest;
// use crate::Millisecond_Timing::{MillisecondTimer};

pub fn test_reconcile_within_buffer_prefers_predictive() {
        actual = MillisecondTimer . get_unix_ms ( );
        report = MillisecondTimer . reconcile_predictive_time ( actual + 100 , buffer_ms = 200 );
        assert report [ "predictive_within_buffer" ] is true;
        assert report [ "authoritative_source" ] == "predictive";
        assert report [ "authoritative_unix_ms" ] == report [ "predictive_unix_ms" ];
        pub fn test_reconcile_outside_buffer_prefers_actual ( ) {
        actual = MillisecondTimer . get_unix_ms ( );
        report = MillisecondTimer . reconcile_predictive_time ( actual + 2000 , buffer_ms = 200 );
        assert report [ "predictive_within_buffer" ] is false;
        assert report [ "authoritative_source" ] == "actual";
        assert report [ "authoritative_unix_ms" ] == report [ "actual_unix_ms" ];
        pub fn test_sovereign_time_check_includes_drift_and_device_flag ( ) {
        report = MillisecondTimer . sovereign_time_reality_check ( "PC_TERMINAL" , drift_threshold_ms = 500 );
        assert report [ "device_allowed" ] is true;
        assert "drift_report" in report;
        assert isinstance ( report [ "drift_report" ] . get ( "drift_ok" ) , bool );
}

