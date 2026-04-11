use crate::brain_scars::LogicFragment;
use anyhow::Result;

/// SUPERIORITY EVALUATOR (V-134.0)
/// First-principles fitness scoring for evolved logic payloads.
/// Three independent criteria bound to the 1.092777 Hz Metabolic Heartbeat.
pub struct SuperiorityEvaluator;

impl SuperiorityEvaluator {
    /// Evaluate if a mutated fragment is superior to the current hive champion.
    pub fn evaluate_superiority(hive_logic: &LogicFragment, expanded_logic: &LogicFragment) -> bool {
        if expanded_logic.score > hive_logic.score {
            return true;
        }
        // Tie-breaker: favor newer generations for exploration
        expanded_logic.timestamp > hive_logic.timestamp
    }

    /// Score a raw logic payload [0..1] using three independent criteria.
    ///
    /// 1. SHANNON ENTROPY  — diverse bit patterns = higher information density
    ///    Perfect: H = 8 bits/symbol (uniform distribution) → score 1.0
    ///    Degenerate: all same byte → H = 0 → score 0.0
    ///
    /// 2. TERNARY BALANCE  — packed ternary {-1, 0, +1} should be equidistributed
    ///    TernaryPacker packs 5 trit values per byte using base-3 encoding.
    ///    Balanced: each trit value ~33.3% of all trits → score 1.0
    ///
    /// 3. HEARTBEAT RESONANCE — XOR checksum modular alignment to SOVEREIGN_HEARTBEAT
    ///    The 1.092777 Hz metabolic constant anchors the logic pattern's period.
    ///    Resonance = how closely the running XOR sum aligns to floor(len * 1.092777).
    ///
    /// Final score = arithmetic mean of the three criteria, in [0, 1].
    pub fn score_logic_payload(payload: &[u8]) -> f64 {
        if payload.is_empty() { return 0.0; }

        // ── 1. SHANNON ENTROPY ───────────────────────────────────────────
        let mut freq = [0u64; 256];
        for &b in payload {
            freq[b as usize] += 1;
        }
        let n = payload.len() as f64;
        let entropy: f64 = freq.iter()
            .filter(|&&c| c > 0)
            .map(|&c| { let p = c as f64 / n; -p * p.log2() })
            .sum();
        // Max possible entropy for u8 = log2(256) = 8.0 bits
        let entropy_score = (entropy / 8.0).min(1.0);

        // ── 2. TERNARY BALANCE ───────────────────────────────────────────
        // Each byte represents 5 base-3 trits: values 0,1,2 (mapped from -1,0,+1)
        // We count trit value frequencies across all bytes.
        let mut trit_counts = [0u64; 3]; // trit -1, 0, +1 (stored as 0,1,2)
        for &b in payload {
            let mut val = b as u64;
            for _ in 0..5 {
                trit_counts[(val % 3) as usize] += 1;
                val /= 3;
            }
        }
        let total_trits: u64 = trit_counts.iter().sum();
        let trit_balance_score = if total_trits == 0 {
            0.0
        } else {
            let ideal = total_trits as f64 / 3.0;
            let deviation: f64 = trit_counts.iter()
                .map(|&c| ((c as f64 - ideal) / ideal).abs())
                .sum::<f64>() / 3.0;
            (1.0 - deviation).max(0.0)
        };

        // ── 3. HEARTBEAT RESONANCE ───────────────────────────────────────
        // Running XOR folded into u64, aligned to the metabolic period.
        // SOVEREIGN_HEARTBEAT = 1.092777 Hz → period = 915ms
        // We detect resonance as: xor_sum mod period_target == 0 (within tolerance)
        const HEARTBEAT_PULSE: f64 = 1.092777037037;
        let period_target = (payload.len() as f64 * HEARTBEAT_PULSE) as u64;

        let xor_sum: u64 = payload
            .chunks(8)
            .map(|chunk| {
                let mut b = [0u8; 8];
                b[..chunk.len()].copy_from_slice(chunk);
                u64::from_le_bytes(b)
            })
            .fold(0u64, |acc, x| acc ^ x);

        let resonance_score = if period_target == 0 {
            0.5 // neutral if undefined
        } else {
            let remainder = (xor_sum % period_target.max(1)) as f64;
            let normalized = remainder / period_target as f64;
            // Score peaks at 0 (in phase) and is lowest at 0.5 (anti-phase)
            // Use a cosine window: score = 0.5 + 0.5*cos(2π*normalized)
            0.5 + 0.5 * (2.0 * std::f64::consts::PI * normalized).cos()
        };

        // ── COMPOSITE SCORE (arithmetic mean) ────────────────────────────
        let composite = (entropy_score + trit_balance_score + resonance_score) / 3.0;
        composite.clamp(0.0, 1.0)
    }

    /// Prepare logic for Sovereign Adaptation
    pub async fn prepare_adaptation(&self, _target_logic: &LogicFragment) -> Result<String> {
        println!("[ EVALUATOR ] Striping logic for Sovereign Adaption Strike...");
        Ok("PREPARED_LOGIC_STREAM".to_string())
    }
}
