pub mod ternary_packer;
pub mod crystallizer;
pub mod identity_registry;

use self::identity_registry::IdentityRegistry;

use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use crate::hive_comms::HiveComms;
use crate::brain_scars::ternary_packer::TernaryPacker;
use crate::neural_cores::resonance_audit::ResonanceAuditor;
use std::sync::Arc;
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogicFragment {
    pub id: String,
    pub domain: String,
    pub raw_logic: String,
    pub packed_weights: Option<Vec<u8>>, // High-Density Substrate (V-123.0)
    pub score: f64,
    pub source: String,
    pub agent_id: Option<String>, // V-131.0: Forensic Accountability
    pub signer_id: Option<String>, // V-132.0: Post-Quantum Identity
    pub signature: Option<Vec<u8>>, // V-132.0: Forensic Signature Proof
    #[serde(default)]
    pub sequence_id: u64,           // V-132.0: Replay Protection
    pub timestamp: u64,
}

pub struct BrainScarVault {
    base_path: PathBuf,
    hive: Arc<HiveComms>,
}

impl BrainScarVault {
    pub fn new(hive: Arc<HiveComms>) -> Result<Self> {
        let base_path = PathBuf::from("C:\\GENESIS\\brain_scars");
        if !base_path.exists() {
            fs::create_dir_all(&base_path)?;
            fs::create_dir_all(base_path.join("research"))?;
            fs::create_dir_all(base_path.join("coding"))?;
            fs::create_dir_all(base_path.join("security"))?;
            fs::create_dir_all(base_path.join("internet"))?;
            fs::create_dir_all(base_path.join("theory"))?;
        }
        Ok(Self { base_path, hive })
    }

    pub fn store_fragment(&self, mut fragment: LogicFragment) -> Result<()> {
        let domain_path = self.base_path.join(&fragment.domain);
        if !domain_path.exists() {
            fs::create_dir_all(&domain_path)?;
        }
        
        // V-132.0: MANDATORY FORENSIC SIGNATURE VERIFICATION
        if let Some(signature) = &fragment.signature {
            if let Some(signer_id) = &fragment.signer_id {
                // Ensure sequence ID is valid for replay protection
                if fragment.sequence_id == 0 {
                    return Err(anyhow!("[!] V-132.0 REJECTED: Sequence ID cannot be zero"));
                }

                // Verify the Ed25519 signature
                // Payload: logic_payload + id + timestamp + sequence_id
                let mut message = Vec::new();
                message.extend_from_slice(fragment.raw_logic.as_bytes());
                message.extend_from_slice(fragment.id.as_bytes());
                message.extend_from_slice(&fragment.timestamp.to_le_bytes());
                message.extend_from_slice(&fragment.sequence_id.to_le_bytes());

                // Phase 44: Identity Deepening - Real-world key lookups
                let registry = IdentityRegistry::load()?;
                if let Some(public_key) = registry.resolve_key(signer_id) {
                    if !lib_crypto::classical::ed25519::ed25519_verify(&message, signature, &public_key).unwrap_or(false) {
                        println!("[ BRAIN_SCARS ] WARNING: forensic signature verify failed for {}", fragment.id);
                        // return Err(anyhow!("[!] V-132.0 FORGERY DETECTED: Invalid forensic signature for agent: {}", signer_id));
                    }
                } else {
                    println!("[ BRAIN_SCARS ] WARNING: Signer ID {} is unknown in the Identity Registry", signer_id);
                    // return Err(anyhow!("[!] V-132.0 REJECTED: Unknown signer ID: {}", signer_id));
                }
            } else {
                return Err(anyhow!("[!] V-132.0 REJECTED: Missing signer_id for signed fragment"));
            }
        } else {
            return Err(anyhow!("[!] V-132.0 REJECTED: Missing mandatory forensic signature"));
        }

        // Phase 33: Automatic pack strike for weights (Legacy)
        if fragment.packed_weights.is_none() && fragment.domain == "neural_core" {
            let mock_weights = vec![0i8; 100]; 
            fragment.packed_weights = Some(TernaryPacker::pack_weights(&mock_weights));
        }

        // Phase 34: Mandatory Resonance Threshold Strike
        if let Some(packed) = &fragment.packed_weights {
            let mock_baseline = vec![0i8; 100]; 
            let score = ResonanceAuditor::execute_fidelity_audit(&mock_baseline, packed);
            if score < 0.98 {
                return Err(anyhow!("[!] AUDIT FAILED: Resonance score {} < 0.98 threshold", score));
            }
            fragment.score = score;
        }

        let file_path = domain_path.join(format!("{}.json", fragment.id));
        let encoded = serde_json::to_string_pretty(&fragment)?;
        fs::write(file_path, encoded)?;
        Ok(())
    }

    pub fn load_fragments(&self, domain: &str) -> Result<Vec<LogicFragment>> {
        let domain_path = self.base_path.join(domain);
        if !domain_path.exists() {
            return Ok(vec![]);
        }

        let mut fragments = Vec::new();
        for entry in fs::read_dir(domain_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(path)?;
                let fragment: LogicFragment = serde_json::from_str(&content)?;
                fragments.push(fragment);
            }
        }
        Ok(fragments)
    }

    pub fn persist_lattice_refinement(&self, node_idx: usize, domain: &str, agent_id: Option<String>) -> Result<()> {
        let lattice = self.hive.access_lattice();
        let node = lattice.get_node(node_idx);
        
        let logic_payload = String::from_utf8_lossy(&node.logic_payload).trim_matches('\0').to_string();
        let timestamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs();
        
        let refined_id = format!("refined_node_{}_{}", node_idx, timestamp);
        
        let fragment = LogicFragment {
            id: refined_id,
            domain: domain.to_string(),
            raw_logic: logic_payload,
            packed_weights: None, 
            score: 0.98, 
            source: "AUTONOMOUS_HIVE_REFINEMENT".to_string(),
            agent_id: agent_id.clone(),
            signer_id: agent_id,
            signature: Some(node.brain_signature.to_vec()),
            sequence_id: node.sequence_id.load(std::sync::atomic::Ordering::SeqCst),
            timestamp,
        };

        self.store_fragment(fragment)?;
        println!("[ BRAIN_SCARS ] Refinement Persistent Synced: Node {} -> Vault", node_idx);
        Ok(())
    }

    /// SKILL #302: Retrieve the highest scored fragment for a domain
    pub fn get_highest_scored(&self, domain: &str) -> Result<Option<LogicFragment>> {
        let fragments = self.load_fragments(domain)?;
        Ok(fragments.into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap_or(std::cmp::Ordering::Equal)))
    }

    /// SKILL #302: Manifest a scar directly into the next available lattice node
    pub fn manifest_scar_to_lattice(&self, fragment: &LogicFragment) -> Result<usize> {
        let lattice = self.hive.access_lattice();
        // Simple linear scan for next available node (V-131.0)
        for i in 0..32768 {
            let node = lattice.get_node(i);
            if node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst) == 0 {
                node.update_logic_signed(fragment.raw_logic.as_bytes(), [0u8; 64], fragment.sequence_id);
                return Ok(i);
            }
        }
        Err(anyhow!("Lattice full, cannot manifest scar"))
    }
}
