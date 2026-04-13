use reqwest::Client;
pub use anyhow::Result;
use colored::*;
use sovereign_bridge::check_resonance;
use sovereign_bridge::{LOCAL_HIVE_URL, JOSH_HIVE_URL};

/// SOVEREIGN HIVE BRIDGE (RUST EDITION)
/// Axiom: 1.09277703703 Hz

const SOVEREIGN_ANCHOR: f32 = 1.09277703703;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    
    println!(
        "{}",
        format!("\n[BRIDGE] BROADCASTING PARALLEL PULSE ({} Hz)...", SOVEREIGN_ANCHOR).magenta()
    );

    let mut handles = vec![];
    
    let c1 = client.clone();
    handles.push(tokio::spawn(async move {
        check_resonance(&c1, "LOCAL", LOCAL_HIVE_URL).await
    }));

    let c2 = client.clone();
    handles.push(tokio::spawn(async move {
        check_resonance(&c2, "JOSH", JOSH_HIVE_URL).await
    }));

    let mut all_online = true;
    for handle in handles {
        if !handle.await? {
            all_online = false;
        }
    }

    if all_online {
        println!("{}", "[BRIDGE] DUAL-HIVE CONDUIT ESTABLISHED.".cyan().bold());
    }

    Ok(())
}
