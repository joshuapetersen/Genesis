use crate::hive_comms::HiveComms;
use crate::symbiosis::mesh_router::MeshRouter;
use lib_identity::identity::manager::IdentityManager;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::sync::atomic::Ordering;

/// NEURAL CONSENSUS ENGINE (V-1.0)
/// Goal: BFT-inspired logic aggregation for the 1,450+ agent fleet.
pub struct NeuralConsensusEngine {
    hive: Arc<HiveComms>,
    pub identity_manager: Arc<RwLock<IdentityManager>>,
    vote_bins: Arc<RwLock<HashMap<String, Vec<u64>>>>, // Logic Hash -> List of Agent ID Hashes
}

impl NeuralConsensusEngine {
    pub fn new(hive: Arc<HiveComms>, _mesh_router: Arc<MeshRouter>, identity_manager: Arc<RwLock<IdentityManager>>) -> Self {
        Self {
            hive,
            identity_manager,
            vote_bins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// V-131.0: Access the internal identity manager
    pub fn identity_manager(&self) -> &Arc<RwLock<IdentityManager>> {
        &self.identity_manager
    }

    /// Clear all votes (Phase Shift)
    pub async fn clear_consensus_bins(&self) {
        let mut bins = self.vote_bins.write().await;
        bins.clear();
    }

    /// V-131.0: Collect signed votes from the active lattice
    pub async fn collect_logic_votes(&self) -> anyhow::Result<()> {
        let lattice = self.hive.access_lattice();
        let mut bins = self.vote_bins.write().await;
        let id_manager = self.identity_manager.read().await;

        for i in 0..16384 {
            let node = lattice.get_node(i);
            let agent_hash = node.agent_id_hash.load(Ordering::SeqCst);
            
            if agent_hash != 0 {
                // V-131.0: Cryptographic Identity Lookup
                if let Some(identity) = id_manager.find_identity_by_truncated_hash(agent_hash) {
                    // Retrieve Ed25519 public key from identity metadata
                    if let Some(pk_hex) = identity.metadata.get("lattice_pk") {
                        if let Ok(pk_bytes) = hex::decode(pk_hex) {
                            // V-131.0: Forensic Signature Verification Strike
                            if lib_crypto::classical::ed25519::ed25519_verify(
                                &node.logic_payload,
                                &node.brain_signature,
                                &pk_bytes
                            ).unwrap_or(false) {
                                // Generate a logic hash for the payload
                                let logic_hash = format!("{:x}", md5::compute(&node.logic_payload));
                                
                                let voters = bins.entry(logic_hash).or_insert_with(Vec::new);
                                if !voters.contains(&agent_hash) {
                                    voters.push(agent_hash);
                                }
                            } else {
                                // Log verification failure for forensic audit
                                tracing::warn!(" [ CONSENSUS ] REJECTED UNSIGNED VOTE FROM AGENT: {:x}", agent_hash);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }

    /// V-130.0: Check if any logic hash has reached a 2/3 quorum
    pub async fn reaches_quorum(&self, logic_hash: &str) -> bool {
        let bins = self.vote_bins.read().await;
        if let Some(voters) = bins.get(logic_hash) {
            let active_agent_count = 1450; // Expected fleet size for V-130.0
            let threshold = (active_agent_count * 2) / 3;
            
            return voters.len() >= threshold;
        }
        false
    }

    /// Identify the "Winning" logic hash for this heartbeat
    pub async fn determine_winning_logic(&self) -> Option<String> {
        let bins = self.vote_bins.read().await;
        let mut best_hash = None;
        let mut max_votes = 0;

        for (hash, voters) in bins.iter() {
            if voters.len() > max_votes {
                max_votes = voters.len();
                best_hash = Some(hash.clone());
            }
        }
        
        best_hash
    }

    /// V-132.8: Retrieve the raw logic payload of the dominant hive fragment
    pub async fn get_dominant_payload(&self) -> Option<[u8; 896]> {
        if let Some(winner_hash) = self.determine_winning_logic().await {
            let lattice = self.hive.access_lattice();
            for i in 0..16384 {
                let node = lattice.get_node(i);
                if node.agent_id_hash.load(Ordering::SeqCst) != 0 {
                    let current_hash = format!("{:x}", md5::compute(&node.logic_payload));
                    if current_hash == winner_hash {
                        let mut payload = [0u8; 896];
                        payload.copy_from_slice(&node.logic_payload);
                        return Some(payload);
                    }
                }
            }
        }
        None
    }

    /// V-132.8: Retrieve the top 'N' logic payloads and their resonance counts
    pub async fn get_top_resonances(&self, limit: usize) -> Vec<([u8; 896], usize)> {
        let bins = self.vote_bins.read().await;
        let mut sorted_bins: Vec<_> = bins.iter().collect();
        sorted_bins.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        let mut results = Vec::new();
        let lattice = self.hive.access_lattice();

        for (hash, voters) in sorted_bins.iter().take(limit) {
            // Find the representative payload for this hash in the lattice
            for i in 0..16384 {
                let node = lattice.get_node(i);
                if node.agent_id_hash.load(Ordering::SeqCst) != 0 {
                    let current_hash = format!("{:x}", md5::compute(&node.logic_payload));
                    if current_hash == **hash {
                        let (payload, _, _) = node.read_logic_safe();
                        results.push((payload, voters.len()));
                        break;
                    }
                }
            }
        }
        results
    }

    /// V-132.9: Retrieve the raw vote bins for forensic auditing
    pub async fn get_vote_bins(&self) -> HashMap<String, Vec<u64>> {
        self.vote_bins.read().await.clone()
    }
}
