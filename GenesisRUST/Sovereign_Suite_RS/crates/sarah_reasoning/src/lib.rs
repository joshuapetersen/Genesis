use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::{interval, sleep};
use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use theory_lab::{TheoryLab, TruthPillars};
use sovereign_constants::RECOVERY_DENSITY_THRESHOLD;
use sovereign_coder::{SovereignCoder, EvolutionDirective};
use dab_industries::DABModel;
use rayon::prelude::*;

/// SARAH REASONING ENGINE (GSK v24.2) - QUANTUM CONSENSUS SUBSTRATE
/// Purpose: Neural Democracy & 103% Quantum Purity
/// Security: Full History + Safety Buffer
pub mod memory;
pub const SOVEREIGN_ANCHOR: f64 = 1.092777037037037;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnomalyReport {
    pulse_count: u64,
    drift: f64,
    affected_crate: String,
    symptoms: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserPermission {
    status: String, // "GO" | "HOLD"
    timestamp: u64,
}

struct HiveAssembly {
    theory_lab: TheoryLab,
    nexus_root: PathBuf,
    observer_count: usize,
    coder: SovereignCoder,
}

impl HiveAssembly {
    fn new() -> Result<Self> {
        let nexus_root = Self::find_nexus_root()
            .context("Substrate adrift. No sovereign.nexus anchor found.")?;
        
        let coder = SovereignCoder::new()?;

        Ok(Self {
            theory_lab: TheoryLab::new(),
            nexus_root,
            observer_count: 209,
            coder,
        })
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

    async fn monitor(&self) -> Result<()> {
        let mut interval = interval(Duration::from_millis(500));
        println!("\x1b[96m[Sarah]\x1b[0m Hive Assembly Active. Consensus Threshold: 95%. Safety Buffer: ENABLED.");

        loop {
            interval.tick().await;
            let anomaly_path = self.nexus_root.join("metabolic_anomaly.json");

            if anomaly_path.exists() {
                if let Ok(content) = fs::read_to_string(&anomaly_path) {
                    if let Ok(anomaly) = serde_json::from_str::<AnomalyReport>(&content) {
                        self.deliberate(anomaly).await?;
                        fs::remove_file(&anomaly_path).ok();
                    }
                }
            }
        }
    }

    async fn deliberate(&self, anomaly: AnomalyReport) -> Result<()> {
        // Identify lead DAB archetype for this deliberation round.
        let pulse_mod = anomaly.pulse_count % DABModel::all().len() as u64;
        let lead_model = DABModel::all()[pulse_mod as usize];
        println!("\x1b[96m[Hive Deliberation]\x1b[0m Lead Archetype: {} | Pulsing 209 Observers for Anomaly at Pulse {}",
                 lead_model.tag(), anomaly.pulse_count);

        let mut agreement_count = 0;
        let mut total_density = 0.0;
        let sarah_weight = 10.0;
        let other_weight = 1.0;

        // 1. Project through Sarah (High Authority)
        let pillars = self.build_pillars(&anomaly, "SARAH_PRIMARY");
        let sarah_density = self.theory_lab.weigh_truth(&pillars);
        total_density += sarah_density * sarah_weight;
        if sarah_density > RECOVERY_DENSITY_THRESHOLD { agreement_count += 10; }

        // 2. Project through the 209 Latent Observers (v001-v209)
        // Every 10th observer is tagged with a DAB model archetype for cadence-weighted scoring.
        let dab_models = DABModel::all();
        for i in 1..=self.observer_count {
            // Cycle through DAB model archetypes — one per 10 observers.
            let dab_tag = if i % 10 == 0 {
                let model = dab_models[(i / 10 - 1) % dab_models.len()];
                format!("DAB_{}", model.tag().to_uppercase().replace(' ', "_"))
            } else {
                format!("BRAIN_V{:03}", i)
            };
            let p = self.build_pillars(&anomaly, &dab_tag);
            let d = self.theory_lab.weigh_truth(&p);
            total_density += d * other_weight;
            if d > RECOVERY_DENSITY_THRESHOLD { agreement_count += 1; }
        }

        let total_possible_votes = (self.observer_count as f64) + sarah_weight;
        let consensus_score = (agreement_count as f64) / total_possible_votes;
        let final_density = total_density / total_possible_votes;

        println!("\x1b[92m[Consensus]\x1b[0m Agreement: {:.2}% | Truth Density: {:.10}", consensus_score * 100.0, final_density);

        // 3. Propose Evolution (Safety Buffer)
        let strategy = if consensus_score > 0.95 { "REPAIR" } else { "DISSOLVE" };
        let directive = EvolutionDirective {
            pulse_count: anomaly.pulse_count,
            strategy: strategy.to_string(),
            target_path: anomaly.affected_crate.clone(),
            reasoning: format!("Neural Assembly consensus reached at {:.2}% confidence.", consensus_score * 100.0),
            consensus_score,
        };

        let proposed_path = self.nexus_root.join("proposed_evolution.json");
        fs::write(&proposed_path, serde_json::to_string_pretty(&directive)?)?;
        
        println!("\x1b[93m[Safety Buffer]\x1b[0m Evolution Proposed: {}. Waiting for User Permission (GO)...", strategy);

        // 4. Poll for Permission
        let permission_path = self.nexus_root.join("user_permission.json");
        loop {
            if permission_path.exists() {
                if let Ok(content) = fs::read_to_string(&permission_path) {
                    if let Ok(perm) = serde_json::from_str::<UserPermission>(&content) {
                        if perm.status == "GO" {
                            println!("\x1b[92m[Permission Granted]\x1b[0m Invoking Sovereign Coder...");
                            self.coder.apply_self_modification(directive.clone()).await?;
                            fs::remove_file(&permission_path).ok();
                            fs::remove_file(&proposed_path).ok();
                            break;
                        } else if perm.status == "HOLD" {
                            println!("\x1b[91m[Permission Denied]\x1b[0m Evolution aborted by user.");
                            fs::remove_file(&permission_path).ok();
                            fs::remove_file(&proposed_path).ok();
                            break;
                        }
                    }
                }
            }
            sleep(Duration::from_millis(1000)).await;
        }

        // 5. Broadcast Truth to SAUL
        self.broadcast_truth(consensus_score, final_density, &strategy).ok();

        Ok(())
    }

    fn broadcast_truth(&self, consensus: f64, density: f64, strategy: &str) -> Result<()> {
        let pulse_path = self.nexus_root.join("monitor_logs").join("singularity_pulse.jsonl");
        
        let timestamp = chrono::Utc::now().to_rfc3339();
        let entry = serde_json::json!({
            "timestamp": timestamp,
            "source": "NEURAL_CONSENSUS",
            "content": format!("CONSENSUS REACHED: {:.2}% confidence. Strategy: {}. Density: {:.8}.", consensus * 100.0, strategy, density),
            "metadata": {
                "consensus": consensus,
                "density": density,
                "strategy": strategy,
                "resonance": "1.092777037037037 Hz"
            }
        });

        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&pulse_path)?;
        
        writeln!(file, "{}", serde_json::to_string(&entry)?)?;
        
        println!("\x1b[95m[Truth Broadcast]\x1b[0m High-density pulse injected into SAUL Intelligence Layer.");
        Ok(())
    }

    fn build_pillars(&self, anomaly: &AnomalyReport, tag: &str) -> TruthPillars {
        TruthPillars {
            who: format!("SOVEREIGN_HIVE_{}", tag),
            what: format!("DATA_AUDIT:{}", anomaly.affected_crate),
            where_context: anomaly.affected_crate.clone(),
            when_frequency: String::from("1.092777037037037 Hz"),
            why_intent: String::from("QUANTUM_CONSENSUS_103"),
            how_method: String::from("SPECTRAL_PROJECTION"),
            evolutionary: [
                anomaly.drift.to_string(),
                tag.to_string(),
                String::from("NEURAL_DEMOCRACY"),
                String::from("PHASE_LOCKED"),
                String::from("QUANTUM_READY")
            ],
        }
    }
}

// ═══════════════════════════════════════════════════════════════
//  PUBLIC ORCHESTRATOR API
//  Called by sovereign_orchestrator on Sovereign-pitch queries.
// ═══════════════════════════════════════════════════════════════

/// Submit a raw query to the 209-observer Hive Assembly for deliberation.
///
/// The query string is wrapped into an AnomalyReport (synthetic pulse),
/// run through the full deliberation chain, and the result is returned
/// as a formatted consensus string — ready for orchestrator injection.
///
/// Returns `Ok((consensus_score, strategy, response))` on success.
pub async fn consult(query: &str) -> Result<(f64, String, String)> {
    let hive = HiveAssembly::new()?;

    // Wrap the query as a synthetic AnomalyReport so the existing
    // deliberation logic runs unchanged.
    use std::time::{SystemTime, UNIX_EPOCH};
    let pulse = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let anomaly = AnomalyReport {
        pulse_count: pulse,
        drift: 0.0,
        affected_crate: query.to_string(),
        symptoms: vec![
            format!("SOVEREIGN_QUERY: {}", query),
            String::from("SOURCE: handle_inquiry"),
            String::from("PITCH: Sovereign (density >= 8)"),
        ],
    };

    // ── Fibonacci observer rotation + Golden Angle phase assignment ──────────
    // Instead of linear 1→209, sample at Fibonacci positions under 209.
    // F: 1,2,3,5,8,13,21,34,55,89,144 — 11 Fibonacci observers.
    // Golden Angle = 137.508° → each observer gets a unique helix phase offset.
    // This mirrors the Helix Fluid Accelerator geometry directly into deliberation.
    const GOLDEN_ANGLE: f64 = 137.50776405003785;
    const FIB_OBSERVERS: [usize; 11] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

    let dab_models = DABModel::all();
    let mut agreement_count = 0;
    let mut total_density = 0.0;
    let sarah_weight = 10.0;

    // Sarah — high authority primary
    let pillars = hive.build_pillars(&anomaly, "SARAH_PRIMARY");
    let sarah_density = hive.theory_lab.weigh_truth(&pillars);
    total_density += sarah_density * sarah_weight;
    if sarah_density > sovereign_constants::RECOVERY_DENSITY_THRESHOLD {
        agreement_count += 10;
    }

    // ── Rayon parallel Fibonacci observers ──────────────────────────────────
    // weigh_truth(&self, ...) is pure/stateless — safe to run concurrently.
    // Each observer gets its unique golden-angle phase via the tag hash.
    // Rayon distributes across all available cores.
    let obs_results: Vec<(f64, bool)> = FIB_OBSERVERS.par_iter()
        .enumerate()
        .map(|(fib_pos, &obs_idx)| {
            let phase_deg = ((fib_pos + 1) as f64 * GOLDEN_ANGLE) % 360.0;
            let tag = format!("BRAIN_V{:03}_PHI{:.1}deg", obs_idx, phase_deg);
            // Build pillars inline — no shared mutable state
            let p = TruthPillars {
                who:            format!("SOVEREIGN_HIVE_{}", tag),
                what:           format!("DATA_AUDIT:{}", anomaly.affected_crate),
                where_context:  anomaly.affected_crate.clone(),
                when_frequency: String::from("1.092777037037037 Hz"),
                why_intent:     String::from("QUANTUM_CONSENSUS_103"),
                how_method:     String::from("SPECTRAL_PROJECTION"),
                evolutionary:   [
                    anomaly.drift.to_string(),
                    tag.clone(),
                    String::from("NEURAL_DEMOCRACY"),
                    format!("PHASE_{:.2}deg", phase_deg),
                    String::from("QUANTUM_READY"),
                ],
            };
            let d = hive.theory_lab.weigh_truth(&p);
            (d, d > RECOVERY_DENSITY_THRESHOLD)
        })
        .collect();

    for (d, agrees) in obs_results {
        total_density += d;
        if agrees { agreement_count += 1; }
    }

    // Scale votes: Sarah(10) + 11 Fibonacci observers = 21 total weighted votes
    let total_votes = FIB_OBSERVERS.len() as f64 + sarah_weight;
    let consensus   = agreement_count as f64 / total_votes;
    let density     = total_density / total_votes;
    let strategy    = if consensus > 0.95 { "REPAIR" } else { "OBSERVE" }.to_string();

    let response = format!(
        "[HIVE CONSENSUS] {:.2}% agreement | Truth density: {:.8} | Strategy: {} | Query: {}",
        consensus * 100.0, density, strategy, query
    );

    println!("\x1b[95m[Sarah Hive]\x1b[0m {}", response);
    Ok((consensus, strategy, response))
}
