// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES — GOLDEN RATIO (φ) CONSTANTS & UTILITIES
//  Applied to: Helix Fluid Accelerator, Scheduler, Memory Substrate
//
//  φ = (1 + √5) / 2 ≈ 1.6180339887...
//  1/φ = φ - 1     ≈ 0.6180339887...   (PHI_INV)
//  φ²  = φ + 1     ≈ 2.6180339887...   (PHI_SQ)
//  1/φ²            ≈ 0.3819660112...   (PHI_INV_SQ)
//
//  KEY IDENTITY: φ = 1 + 1/φ  (self-referential — proven in tests)
// ═══════════════════════════════════════════════════════════════

// ───────────────────────────────────────────────────────────────
//  CONSTANTS
// ───────────────────────────────────────────────────────────────

/// φ — the Golden Ratio.
pub const PHI: f64 = 1.618033988749895;

/// 1/φ = φ - 1 ≈ 0.6180339887...
/// Used as memory confidence weight: gives a naturally convergent series.
/// Sum of (1/φ)^n for n=1..∞ = φ - 1 = 1/φ itself.  Self-sealing.
pub const PHI_INV: f64 = 0.6180339887498949;

/// φ² = φ + 1 ≈ 2.6180339887...
pub const PHI_SQ: f64 = 2.618033988749895;

/// 1/φ² ≈ 0.3819660112...
pub const PHI_INV_SQ: f64 = 0.38196601125010515;

/// 5 × φ ≈ 8.0901699437...
/// Mathematical basis: φ = (1+√5)/2 — the 5 that generates φ brought back in.
/// floor(5φ) = 8 = Fibonacci(6).  8/5 = 1.6 ≈ φ.
/// Used to define the Sovereign resonance threshold.
pub const PHI_5: f64 = 8.090169943749474;

/// 1 / (5φ) ≈ 0.12360679774997896
pub const PHI_5_INV: f64 = 0.12360679774997896;

/// Sovereign percussion density threshold = floor(5φ) = 8.
/// A bar hitting 8+ hard consonants achieves Sovereign Resonance.
/// 8 is Fibonacci(6); GCD(8,5) = 1; 8/5 = 1.6 ≈ φ.
pub const SOVEREIGN_DENSITY_THRESHOLD: usize = 8;

/// Memory confidence for Sovereign-depth queries: 1 - 1/(5φ) ≈ 0.8764.
/// Higher than Deep's 1/φ ≈ 0.618 — Sovereign responses have stronger retention.
/// Derived from 5φ: the further you are from ordinary, the longer you are remembered.
pub const SOVEREIGN_MEMORY_CONFIDENCE: f64 = 0.8763932022500211;

/// Golden Angle in degrees: 360° × (1 - 1/φ) = 360° / φ² ≈ 137.5077...°
///
/// Engineering significance for the Helix Fluid Accelerator:
/// This is the mathematically optimal inter-turn angular offset.
/// At this angle, each successive fluid pass is maximally separated
/// from every prior pass — no two turns ever share a resonant overlap.
/// Found in DNA double-helix, nautilus shells, sunflower seed packing.
pub const GOLDEN_ANGLE_DEG: f64 = 137.5077640500378;

/// Golden Angle in radians (GOLDEN_ANGLE_DEG × π/180).
pub const GOLDEN_ANGLE_RAD: f64 = 2.3999632297286535;

/// φ-proximity of the 7-12 motor ratio.
/// 7/12 = 0.5833...  |  1/φ = 0.6180...
/// Delta = 0.0347 — the 7-12 ratio is the closest achievable integer-pole
/// approximation to 1/φ at small pole counts.
pub const MOTOR_RATIO_DELTA_FROM_PHI_INV: f64 = 0.034700054916229; // |1/φ - 7/12|

// ───────────────────────────────────────────────────────────────
//  φ-CURVE BAR SCORING
// ───────────────────────────────────────────────────────────────

/// φ-curve percussion density score (0–80 range, same ceiling as validate_bar).
///
/// Replaces the hard-cap linear formula with a smooth asymptote:
///   score = (1 - 1/φ^density) × 80
///
/// Comparison of old (linear, hard-cap at 5) vs new (φ-curve):
///   density 1 → old 16   | new 30.6
///   density 2 → old 32   | new 49.4
///   density 3 → old 48   | new 61.1
///   density 5 → old 80   | new 72.8
///   density 8 → old 80   | new 78.5  (never abruptly cliffs)
///   density ∞ → old 80   | new 80.0  (same ceiling, smooth approach)
///
/// The φ-curve rewards early percussion hits more — matching how
/// a bar that opens hard actually SOUNDS more percussive than one
/// that loads density late.
pub fn phi_density_score(density: usize) -> u8 {
    if density == 0 {
        return 0;
    }
    let score = (1.0 - PHI_INV.powi(density as i32)) * 80.0;
    score.min(80.0) as u8
}

// ───────────────────────────────────────────────────────────────
//  MEMORY CONFIDENCE
// ───────────────────────────────────────────────────────────────

/// φ-derived memory confidence weight for deep-pitch query storage.
///
/// Value: 1/φ ≈ 0.618...
///
/// Why this is better than 0.8:
///   - Sum of (1/φ)^n for n=1..∞ converges to exactly 1/φ itself.
///   - Memory traces weighted by 1/φ never accumulate to saturation.
///   - Older memories naturally fade by factor 1/φ per retrieval cycle.
///   - 0.8 weights can stack and exceed 1.0; 1/φ is self-bounding.
pub fn memory_confidence() -> f64 {
    PHI_INV
}

// ───────────────────────────────────────────────────────────────
//  FIBONACCI GENERATOR
// ───────────────────────────────────────────────────────────────

/// Generate the first `n` terms of the Fibonacci sequence.
/// Ratio of consecutive terms converges to φ as n → ∞.
pub fn fibonacci(n: usize) -> Vec<u64> {
    if n == 0 {
        return vec![];
    }
    let mut seq = vec![1u64, 1u64];
    while seq.len() < n {
        let len = seq.len();
        let next = seq[len - 1].saturating_add(seq[len - 2]);
        seq.push(next);
    }
    seq.truncate(n);
    seq
}

/// Ratio of the last two terms of a Fibonacci sequence of length n.
/// Converges to φ from below.
pub fn fibonacci_phi_approx(n: usize) -> f64 {
    let seq = fibonacci(n.max(2));
    let len = seq.len();
    seq[len - 1] as f64 / seq[len - 2] as f64
}

// ───────────────────────────────────────────────────────────────
//  φ IDENTITY VERIFICATION
// ───────────────────────────────────────────────────────────────

/// Verify the core φ identity: φ = 1 + 1/φ
/// Returns true if constants are consistent within float tolerance.
pub fn verify_phi_identity() -> bool {
    (PHI - (1.0 + PHI_INV)).abs() < 1e-12
}

/// Verify: φ² = φ + 1
pub fn verify_phi_sq_identity() -> bool {
    (PHI_SQ - (PHI + 1.0)).abs() < 1e-12
}

// ───────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phi5_equals_five_times_phi() {
        assert!((PHI_5 - 5.0 * PHI).abs() < 1e-10);
    }

    #[test]
    fn phi5_inv_times_phi5_equals_one() {
        assert!((PHI_5 * PHI_5_INV - 1.0).abs() < 1e-10);
    }

    #[test]
    fn sovereign_threshold_is_floor_of_phi5() {
        assert_eq!(SOVEREIGN_DENSITY_THRESHOLD, PHI_5.floor() as usize);
    }

    #[test]
    fn sovereign_memory_confidence_correct() {
        let expected = 1.0 - PHI_5_INV;
        assert!((SOVEREIGN_MEMORY_CONFIDENCE - expected).abs() < 1e-12);
        assert!(SOVEREIGN_MEMORY_CONFIDENCE > PHI_INV);
        assert!(SOVEREIGN_MEMORY_CONFIDENCE < 1.0);
    }

    #[test]
    fn phi_identity_holds() {
        assert!(verify_phi_identity(), "φ ≠ 1 + 1/φ — constant precision failure");
    }

    #[test]
    fn phi_sq_identity_holds() {
        assert!(verify_phi_sq_identity(), "φ² ≠ φ + 1");
    }

    #[test]
    fn phi_inv_plus_phi_inv_sq_equals_one() {
        // 1/φ + 1/φ² = 1  (fundamental φ partition)
        let sum = PHI_INV + PHI_INV_SQ;
        assert!((sum - 1.0).abs() < 1e-12, "1/φ + 1/φ² = {}, not 1.0", sum);
    }

    #[test]
    fn golden_angle_correct() {
        // 360 × (1 - 1/φ) = 360 × 1/φ² = GOLDEN_ANGLE_DEG
        let computed = 360.0 * PHI_INV_SQ;
        assert!((computed - GOLDEN_ANGLE_DEG).abs() < 1e-8);
    }

    #[test]
    fn motor_ratio_close_to_phi_inv() {
        // |7/12 - 1/φ| < 0.04
        let motor = 7.0_f64 / 12.0;
        assert!((motor - PHI_INV).abs() < 0.04,
            "7/12={:.6} unexpectedly far from 1/φ={:.6}", motor, PHI_INV);
    }

    #[test]
    fn phi_density_score_zero_for_no_hits() {
        assert_eq!(phi_density_score(0), 0);
    }

    #[test]
    fn phi_density_score_increases_with_density() {
        for d in 1..8 {
            assert!(phi_density_score(d + 1) > phi_density_score(d),
                "score did not increase from density {} to {}", d, d + 1);
        }
    }

    #[test]
    fn phi_density_score_never_exceeds_80() {
        for d in 0..=20 {
            assert!(phi_density_score(d) <= 80,
                "density {} exceeded ceiling 80: got {}", d, phi_density_score(d));
        }
    }

    #[test]
    fn memory_confidence_is_phi_inv() {
        assert!((memory_confidence() - PHI_INV).abs() < f64::EPSILON);
    }

    #[test]
    fn fibonacci_first_ten() {
        let seq = fibonacci(10);
        assert_eq!(seq, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    #[test]
    fn fibonacci_ratio_approaches_phi() {
        let approx = fibonacci_phi_approx(30);
        assert!((approx - PHI).abs() < 0.0001,
            "Fib ratio {:.6} not close enough to φ={:.6}", approx, PHI);
    }
}
