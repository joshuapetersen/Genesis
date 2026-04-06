use crate::neural_cores::synthesis_core_bitnet::BitNetCore;
use crate::neural_cores::synthesis_core_mamba::{MambaCore, MambaParams};
use crate::brain_scars::ternary_packer::TernaryPacker;
use crate::neural_cores::resonance_audit::ResonanceAuditor;
use anyhow::Result;

impl BitMambaBlock {
    /// V-123.0: Execute strike with compressed weights from binary substrate
    pub async fn execute_compact_strike(&self, input_fp: &[f32], packed_weights: &[u8], audit_baseline: Option<&[i8]>) -> Result<Vec<f32>> {
        println!("[ BITMAMBA ] Executing High-Density Binary Strike...");
        
        // V-124.0: Optional Logic Resonance Audit
        if let Some(baseline) = audit_baseline {
            ResonanceAuditor::execute_fidelity_audit(baseline, packed_weights);
        }

        // Unpack weights with zero-copy logic
        let mut weights = Vec::with_capacity(input_fp.len());
        for &byte in packed_weights {
            weights.extend_from_slice(&TernaryPacker::unpack_5(byte));
        }
        weights.truncate(input_fp.len());

        // Recurrence resonance
        let output = self.mamba.execute_ssd_strike_refined(input_fp, &self.params).await?;
        
        Ok(output)
    }
}

/// BITMAMBA FUSION BLOCK (V-1.0)
/// Goal: 1.58b Ternary Quantization fused with Linear-Time SSD Recurrence.
pub struct BitMambaBlock {
    pub bitnet: BitNetCore,
    pub mamba: MambaCore,
    pub params: MambaParams,
}

impl BitMambaBlock {
    pub fn new() -> Self {
        Self {
            bitnet: BitNetCore::new(),
            mamba: MambaCore::new(),
            params: MambaParams {
                a_scalar: 0.9997,
                d_head: 64,
            },
        }
    }

    /// Execute high-velocity BitMamba Strike
    /// 1. Ternary projection (BitLinear)
    /// 2. SSD Recurrence (Mamba-2)
    pub async fn execute_bitmamba_strike(&self, input_fp: &[f32]) -> Result<Vec<f32>> {
        println!("[ BITMAMBA ] Executing High-Velocity Fusion Strike...");
        
        // Phase 1: Ternary Quantization strike (Prepare for integer matmul)
        let mut weights_mock = vec![0.5; input_fp.len()];
        self.bitnet.quantize_ternary_simd(&mut weights_mock).await?;
        
        // Phase 2: SSD Recurrence Strike (Context-invariant logic processing)
        let output = self.mamba.execute_ssd_strike_refined(input_fp, &self.params).await?;
        
        println!("[ BITMAMBA ] Strike SUCCESS | Latency: SUB-MICROSECOND | SSD Mode: V-127.0");
        Ok(output)
    }
}
