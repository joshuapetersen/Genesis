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

pub struct SovereignHive {
    pub nodes: HashMap<String, HiveNode>,
    pub nexus_id: String,
    pub identity: Hypervector,
}

impl SovereignHive {
    pub fn new(nexus_id: &str) -> Self {
        Self {
            nodes: HashMap::new(),
            nexus_id: nexus_id.to_string(),
            identity: Hypervector::random(),
        }
    }

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
        // Calculate the "Trust Resonance" using Hamming Similarity
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
