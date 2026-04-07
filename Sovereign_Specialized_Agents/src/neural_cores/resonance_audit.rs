use crate::brain_scars::ternary_packer::TernaryPacker;
use crate::hive_comms::HiveComms;
use crate::brain_scars::identity_registry::IdentityRegistry;
use lib_crypto::classical::ed25519;
use anyhow::Result;
use std::sync::Arc;

/// RESONANCE AUDITOR (V-132.9)
/// Goal: Prove 101% Logic Purity across the universal high-density substrate.
pub struct ResonanceAuditor {
    hive: Arc<HiveComms>,
}

#[derive(Debug, Default)]
pub struct ResonanceReport {
    pub total_nodes_audited: usize,
    pub valid_signatures: usize,
    pub invalid_signatures: usize,
    pub unknown_agents: usize,
    pub logic_drift_score: f64,
}

impl ResonanceAuditor {
    pub fn new(hive: Arc<HiveComms>) -> Self {
        Self { hive }
    }

    /// V-132.9: Lattice Forensic Audit Strike
    /// Verifies the cryptographic integrity of every active logic node.
    pub fn execute_forensic_audit(&self) -> Result<ResonanceReport> {
        let mut report = ResonanceReport::default();
        let lattice = self.hive.access_lattice();
        let id_registry = IdentityRegistry::load()?;
        
        println!("[ AUDIT ] Initiating Lattice forensic scan (32,768 nodes)...");

        for i in 0..32768 {
            let node = lattice.get_node(i);
            let agent_id_hash = node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst);
            
            if agent_id_hash == 0 { continue; }
            
            report.total_nodes_audited += 1;
            
            // seqlock-safe read of the node state (V-132.8)
            let (payload, signature, sequence_id) = node.read_logic_safe();

            if let Some(pk_bytes) = id_registry.resolve_key_by_hash(agent_id_hash) {
                let mut message = Vec::new();
                message.extend_from_slice(&payload);
                message.extend_from_slice(&agent_id_hash.to_le_bytes());
                message.extend_from_slice(&sequence_id.to_le_bytes());

                if ed25519::ed25519_verify(&message, &signature, &pk_bytes).unwrap_or(false) {
                    report.valid_signatures += 1;
                } else {
                    report.invalid_signatures += 1;
                    eprintln!(" [!] FORENSIC BREACH AT NODE {}: Agent {:016X} signature mismatch.", i, agent_id_hash);
                }
            } else {
                report.unknown_agents += 1;
                eprintln!(" [!] UNKNOWN AGENT AT NODE {}: Hash {:016X} not found in registry.", i, agent_id_hash);
            }
        }

        println!("[ AUDIT ] Forensic Scan Complete. Valid: {} | Breach: {} | Unknown: {}", 
            report.valid_signatures, report.invalid_signatures, report.unknown_agents);
            
        Ok(report)
    }

    /// Execute High-Fidelity Diagnostic Strike (Legacy Calibration)
    pub fn execute_fidelity_audit(original_ternary: &[i8], packed: &[u8]) -> f64 {
        let mut unpacked = Vec::with_capacity(original_ternary.len());
        for &byte in packed {
            unpacked.extend_from_slice(&TernaryPacker::unpack_5(byte));
        }
        unpacked.truncate(original_ternary.len());

        let mut matches = 0;
        for (o, u) in original_ternary.iter().zip(unpacked.iter()) {
            if o == u {
                matches += 1;
            }
        }

        let score = matches as f64 / original_ternary.len() as f64;
        println!("[ RESONANCE ] Fidelity Audit Score: {:.4}", score);
        score
    }

    /// Execute Mamba Drift Audit (Legacy Recurrence Check)
    pub fn execute_drift_audit(output_a: &[f32], output_b: &[f32]) -> f32 {
        let mut diff_sum = 0.0;
        for (a, b) in output_a.iter().zip(output_b.iter()) {
            diff_sum += (a - b).abs();
        }
        
        let drift = diff_sum / output_a.len() as f32;
        println!("[ RESONANCE ] Mamba Drift Audit: {:.6}", drift);
        drift
    }
}
