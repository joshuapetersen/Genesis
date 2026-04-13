use tokio::process::Command;
use anyhow::Result;
use colored::*;
use std::time::Instant;
use std::path::Path;

/// SOVEREIGN GODS EYE ACCELERATOR (RUST EDITION)
/// Axiom: 1.09277703703 Hz
/// Pattern: Combustion Chamber Parallel Manifestation

const AGENT_COUNT: usize = 1923;
const SOVEREIGN_ANCHOR: f32 = 1.09277703703;

async fn combustion_worker(repo_path: &str, task_name: &str) -> Result<()> {
    println!(
        "{}",
        format!("[COMBUSTION] Manifesting {} @ {}...", task_name, repo_path).cyan()
    );

    let orchestrator_path = if Path::new(repo_path)
        .join("Sovereign_Suite_RS")
        .join("crates")
        .join("sovereign_orchestrator")
        .exists()
    {
        Path::new(repo_path)
            .join("Sovereign_Suite_RS")
            .join("crates")
            .join("sovereign_orchestrator")
    } else {
        Path::new(repo_path).join("crates").join("sovereign_orchestrator")
    };

    let status = Command::new("cargo")
        .arg("build")
        .current_dir(orchestrator_path)
        .status()
        .await?;

    if status.success() {
        println!(
            "{}",
            format!(
                "[GODSEYE] {} STATUS: 100% PRISTINE (Resonance: {})",
                task_name, SOVEREIGN_ANCHOR
            )
            .green()
        );
        Ok(())
    } else {
        Err(anyhow::anyhow!("Manifestation deviant in {}", repo_path))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let start = Instant::now();

    println!("{}", "\n[GODSEYE] DISPATCHING SEARCH SWARM (1923 Agents)...".magenta());
    
    let repos = vec![
        ("C:\\GENESIS\\GenesisRUST", "LOCAL_HIVE"),
        ("C:\\JOSH_REPO\\rust", "JOSH_HIVE"),
    ];

    println!("{}", "[GODSEYE] BEGINNING MASSIVE MANIFESTATION (JetEngine Mode)...".cyan());

    let mut handles = vec![];
    for (path, name) in repos {
        handles.push(tokio::spawn(async move {
            combustion_worker(path, name).await
        }));
    }

    for handle in handles {
        handle.await??;
    }

    let duration = start.elapsed();
    println!(
        "{}",
        format!(
            "\n[GODSEYE] TOTAL MANIFESTATION COMPLETE. TIME: {:?}. METABOLIC RATE: {} Hz.",
            duration, SOVEREIGN_ANCHOR
        )
        .green()
        .bold()
    );

    Ok(())
}
