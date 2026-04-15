use serde::{Serialize, Deserialize};
use sovereign_hdc::Hypervector;
use std::collections::HashMap;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// [HIVE_PULSE_0xH]: THE GLOBAL RESONANCE NODE
/// AXIOM: Every node is a holographic reflection of the Nexus Prime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiveNode {
    pub id: String,
    pub endpoint: String,
    pub identity_shroud: Hypervector, // 10,240-bit holographic ID
    pub last_pulse: u64,
    pub metabolic_lock: f64, // Target: 1.092777 Hz
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HiveHandshake {
    pub nexus_id: String,
    pub protocol_version: String,
    pub local_resonance: f64,
    pub holographic_identity: Hypervector,
    pub timestamp: u64,
}

/// φ constant for weight decay
const PHI_INV: f64 = 0.6180339887498949;

pub struct SovereignHive {
    pub nodes: HashMap<String, HiveNode>,
    pub nexus_id: String,
    pub identity: Hypervector,
    /// Per-observer accuracy weights [1..=209] — default 1.0, range [0.1, 10.0].
    /// Observers that historically agree with final consensus gain weight.
    /// Updated via update_observer_weight() after each deliberation cycle.
    observer_weights: HashMap<usize, f64>,
}

impl SovereignHive {
    pub fn new(nexus_id: &str) -> Self {
        Self {
            nodes: HashMap::new(),
            nexus_id: nexus_id.to_string(),
            identity: Hypervector::random(),
            observer_weights: HashMap::new(),
        }
    }

    // ── OBSERVER WEIGHT LEARNING ────────────────────────────────────────────

    /// Returns the current weight for an observer (1..=209). Defaults to 1.0.
    #[inline(always)]
    pub fn get_observer_weight(&self, obs_idx: usize) -> f64 {
        *self.observer_weights.get(&obs_idx).unwrap_or(&1.0)
    }

    /// Update observer weight post-consensus.
    ///
    /// - `agreed_with_consensus = true`  → reward: weight × φ^(-1) × 1.1 (capped at 10.0)
    /// - `agreed_with_consensus = false` → decay: weight × φ_inv (capped at 0.1)
    ///
    /// Over time, consistently accurate observers gain up to 10× their vote;
    /// persistently wrong outliers decay to 0.1× — almost voiceless.
    pub fn update_observer_weight(&mut self, obs_idx: usize, agreed_with_consensus: bool) {
        let w = self.observer_weights.entry(obs_idx).or_insert(1.0);
        if agreed_with_consensus {
            *w = (*w * (1.0 / PHI_INV)).min(10.0);  // reward: grow toward 10×
        } else {
            *w = (*w * PHI_INV).max(0.1);             // decay: shrink toward 0.1×
        }
    }

    /// Apply weight updates for all observers in one deliberation pass.
    /// `strategy_was_repair` = whether final consensus chose REPAIR (vs OBSERVE).
    pub fn learn_from_deliberation(
        &mut self,
        obs_results: &[(bool, usize)], // (agreed, obs_idx)
        strategy_was_repair: bool,
    ) {
        for &(agreed, obs_idx) in obs_results {
            // If strategy=REPAIR and observer agreed → observer was right → reward
            // If strategy=REPAIR and observer disagreed → observer was wrong → decay
            let was_correct = agreed == strategy_was_repair;
            self.update_observer_weight(obs_idx, was_correct);
        }
    }

    /// Snapshot of top-N weighted observers. Used for telemetry/HUD.
    pub fn top_observers(&self, n: usize) -> Vec<(usize, f64)> {
        let mut pairs: Vec<(usize, f64)> = self.observer_weights.iter()
            .map(|(&idx, &w)| (idx, w))
            .collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        pairs.truncate(n);
        pairs
    }

    /// [PERSIST] Save observer weights to disk (JSON). Call after each deliberation cycle.
    pub fn save_weights(&self, path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
        // Serialize as Vec<(usize, f64)> — stable across serde versions
        let pairs: Vec<(usize, f64)> = self.observer_weights.iter()
            .map(|(&k, &v)| (k, v))
            .collect();
        let data = serde_json::to_string_pretty(&pairs)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, data)
    }

    /// [PERSIST] Load observer weights from disk. Missing file → empty weights (fresh start).
    pub fn load_weights(&mut self, path: impl AsRef<std::path::Path>) {
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(pairs) = serde_json::from_str::<Vec<(usize, f64)>>(&data) {
                let loaded = pairs.len();
                self.observer_weights = pairs.into_iter().collect();
                println!("\x1b[96m[Hive] Loaded {} observer weights from disk.\x1b[0m", loaded);
            }
        }
    }

    // ── EXISTING METHODS ───────────────────────────────────────────────────

    /// [MANIFEST_HANDSHAKE]: Prepares the holographic greeting for external kin.
    pub fn manifest_handshake(&self) -> HiveHandshake {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        HiveHandshake {
            nexus_id: self.nexus_id.clone(),
            protocol_version: "ZENITH_1.0".to_string(),
            local_resonance: 1.092777037037037,
            holographic_identity: self.identity.clone(),
            timestamp: now,
        }
    }

    /// [ASSIMILATE_NODE]: Integrates a foreign node into the local manifold.
    pub fn assimilate(&mut self, handshake: HiveHandshake, endpoint: &str) -> f64 {
        let trust_resonance = self.identity.similarity(&handshake.holographic_identity);
        
        let node = HiveNode {
            id: handshake.nexus_id.clone(),
            endpoint: endpoint.to_string(),
            identity_shroud: handshake.holographic_identity,
            last_pulse: handshake.timestamp,
            metabolic_lock: handshake.local_resonance,
        };

        self.nodes.insert(handshake.nexus_id, node);
        trust_resonance
    }
}
