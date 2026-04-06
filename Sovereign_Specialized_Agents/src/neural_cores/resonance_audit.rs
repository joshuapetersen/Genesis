use crate::brain_scars::ternary_packer::TernaryPacker;
use anyhow::Result;

/// RESONANCE AUDITOR (V-1.0)
/// Goal: Prove 101% Logic Purity across the compressed high-density substrate.
pub struct ResonanceAuditor;

impl ResonanceAuditor {
    /// Execute High-Fidelity Diagnostic Strike
    /// Compares packed/unpacked weights vs quantized baseline.
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

    /// Execute Mamba Drift Audit
    /// Measures variance in SSD recurrence outputs.
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
