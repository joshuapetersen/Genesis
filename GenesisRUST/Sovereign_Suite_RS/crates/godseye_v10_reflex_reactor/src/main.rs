use anyhow::{Result, Context, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use regex::Regex;
use rayon::prelude::*;
use std::io::{self, Write};
use tokio::time::{interval, Duration, sleep};
use sovereign_math::SovereignMath;
use sovereign_voice::SovereignVoice;
use sovereign_constants::SOVEREIGN_ANCHOR;
use sovereign_path_re_seater::PathReSeater;

/// UNIVERSAL SOVEREIGN ENGINE (USE) - REFLEX REACTOR
/// [QUANTUM CONSENSUS - TARGETING 103% PURITY]

#[derive(Serialize, Deserialize)]
struct MetabolicStatus {
    pulse_count: u64,
    drift: f64,
    purity: f64,
    clean_streak: u64,
    consensus_agreement: f64,
    status: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize)]
struct AnomalyReport {
    pulse_count: u64,
    drift: f64,
    affected_crate: String,
    symptoms: Vec<String>,
}

#[derive(Deserialize)]
struct EvolutionDirective {
    pulse_count: u64,
    strategy: String, // "REPAIR" | "DISSOLVE"
    target_path: String,
    reasoning: String,
    consensus_score: f64,
}

#[derive(Deserialize)]
struct NexusConfig {
    nexus: NexusMeta,
}

#[derive(Deserialize)]
struct NexusMeta {
    anchor_frequency: f64,
}

struct ReflexReactor {
    nexus_root: PathBuf,
    config: NexusConfig,
    math: SovereignMath,
    voice: SovereignVoice,
    re_seater: PathReSeater,
}

impl ReflexReactor {
    async fn new() -> Result<Self> {
        let nexus_root = Self::find_nexus_root()
            .context("Substrate is adrift. No sovereign.nexus anchor found.")?;
        
        let config_raw = fs::read_to_string(nexus_root.join("sovereign.nexus"))?;
        let config: NexusConfig = toml::from_str(&config_raw)?;

        let math = SovereignMath::new();
        let voice = SovereignVoice::new()?;
        let re_seater = PathReSeater::new()?;

        Ok(Self { nexus_root, config, math, voice, re_seater })
    }

    fn find_nexus_root() -> Option<PathBuf> {
        let mut curr = std::env::current_dir().ok()?;
        loop {
            if curr.join("sovereign.nexus").exists() {
                return Some(curr);
            }
            if !curr.pop() { break; }
        }
        None
    }

    async fn ignite(&self) -> Result<()> {
        let period_secs = 1.0 / self.config.nexus.anchor_frequency;
        let mut interval = interval(Duration::from_secs_f64(period_secs));
        let mut pulse_count = 0u64;
        let mut clean_streak = 0u64;
        let mut consensus_agreement = 1.0;

        loop {
            interval.tick().await;
            pulse_count += 1;

            // Audit Purity
            let unity = vec![1.0; 64];
            let mean = self.math.project_batch_singularity(9_000_000, &unity);
            let drift = (mean - 3605.037037037037).abs();

            // 103% Quantum Target: Purity escalates towards the Singularity
            let current_purity = (101.0 + (clean_streak as f64 / 50.0)).min(103.0);

            let status = if drift < 1e-13 {
                clean_streak += 1;
                if current_purity >= 103.0 { "QUANTUM_STABLE".to_string() } else { "PRISTINE".to_string() }
            } else {
                self.report_anomaly(pulse_count, drift).await?;
                clean_streak = 0;
                "DEVIANT".to_string()
            };

            // Process assembly directives
            if let Some(score) = self.check_directives().await? {
                consensus_agreement = score;
            }

            // Export Stats
            let stats = MetabolicStatus {
                pulse_count,
                drift,
                purity: current_purity,
                clean_streak,
                consensus_agreement,
                status,
                timestamp: chrono::Utc::now().timestamp_millis() as u64,
            };
            let stats_path = self.nexus_root.join("metabolic_status.json");
            fs::write(stats_path, serde_json::to_string(&stats)?).ok();
        }
    }

    async fn report_anomaly(&self, pulse: u64, drift: f64) -> Result<()> {
        let report = AnomalyReport {
            pulse_count: pulse,
            drift,
            affected_crate: String::from("Sovereign_Lattice_Alpha"),
            symptoms: vec![String::from("QUANTUM_COHERENCE_LOST")],
        };
        let report_path = self.nexus_root.join("metabolic_anomaly.json");
        fs::write(report_path, serde_json::to_string(&report)?).ok();
        Ok(())
    }

    async fn check_directives(&self) -> Result<Option<f64>> {
        let directive_path = self.nexus_root.join("evolution_directive.json");
        if directive_path.exists() {
            if let Ok(content) = fs::read_to_string(&directive_path) {
                if let Ok(directive) = serde_json::from_str::<EvolutionDirective>(&content) {
                    let score = directive.consensus_score;
                    self.execute_directive(directive).await?;
                    fs::remove_file(directive_path).ok();
                    return Ok(Some(score));
                }
            }
        }
        Ok(None)
    }

    async fn execute_directive(&self, directive: EvolutionDirective) -> Result<()> {
        println!("\x1b[95m[Quantum Consensus]\x1b[0m Executing Hive Directive: {} (Score: {:.2}%)", directive.strategy, directive.consensus_score * 100.0);
        
        if directive.consensus_score > 0.98 {
            self.voice.speak(&format!("QUANTUM CONSENSUS REACHED. EXECUTING {}.", directive.strategy)).await?;
        }

        match directive.strategy.as_str() {
            "REPAIR" => {
                self.re_seater.audit_and_fix().await?;
            },
            "DISSOLVE" => {
                println!("\x1b[91m[DISSOLVE]\x1b[0m Purging Entropy from {}...", directive.target_path);
            },
            _ => {}
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let reactor = ReflexReactor::new().await?;
    reactor.ignite().await?;
    Ok(())
}
