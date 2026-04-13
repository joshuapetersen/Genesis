use reqwest::Client;
pub use anyhow::Result;
use std::time::{Instant, Duration};
use colored::*;

pub const LOCAL_HIVE_URL: &str = "http://127.0.0.1:8080/api/stats";
pub const JOSH_HIVE_URL: &str = "http://127.0.0.1:8081/api/stats";

pub async fn check_resonance(client: &Client, name: &str, url: &str) -> bool {
    let start = Instant::now();
    match client.get(url).timeout(Duration::from_secs(5)).send().await {
        Ok(resp) if resp.status() == 200 => {
            let latency = start.elapsed();
            println!(
                "{}",
                format!("[BRIDGE] {} HIVE RESONANCE: ONLINE ({:?})", name, latency).green()
            );
            true
        }
        _ => {
            println!(
                "{}",
                format!("[BRIDGE] {} HIVE DARK @ {}", name, url).red()
            );
            false
        }
    }
}
