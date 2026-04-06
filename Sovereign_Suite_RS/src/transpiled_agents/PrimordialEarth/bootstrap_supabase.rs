//! bootstrap_supabase.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::env;
// use crate::subprocess;
// use crate::supabase::{create_client};

pub const SERVICE_KEY: &str = input ("Paste your Supabase SERVICE ROLE key (from Settings > API): " ) . strip ( );
pub const URL: &str = "https://duuycxgqbhrqmwapnjhk.supabase.co";
pub const client: f64 = create_client ( URL , SERVICE_KEY );
