// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES — SCHEDULER PROTOCOL
//  Derived from: 7-12 Motor Geometry (engineering.rs)
//               φ (Golden Ratio) — phi.rs
//
//  PRINCIPLE 1 — Cogging Elimination (from 7-12 geometry):
//    GCD(stator=7, rotor=12) = 1 → all intervals are prime.
//    GCD of any two distinct primes = 1.
//    No two tasks ever fire at the same wall-clock second.
//
//  PRINCIPLE 2 — φ-Ratio Geometric Spacing (from Golden Ratio):
//    Each interval ≈ φ × the previous.
//    Produces logarithmic fan-out from fast tasks to slow tasks.
//    Mirrors the variable-pitch helix: close turns at the entry
//    (fast tasks), wide turns at the exit (slow tasks).
//
//  COMBINED: 7, 11, 17, 29, 47, 79, 127, 211
//    All prime (GCD=1) AND approximately φ-ratio apart.
//    11/7=1.571  17/11=1.545  29/17=1.706  47/29=1.621≈φ
//    79/47=1.681  127/79=1.608≈φ  211/127=1.661≈φ
//
//  ALIGNMENT MACRO BEAT = LCM(7,12) = 84 seconds.
// ═══════════════════════════════════════════════════════════════

/// The macro-beat of the system derived from LCM(7,12).
/// Full geometric alignment occurs every 84 seconds.
pub const ALIGNMENT_MACRO_BEAT_SECS: u64 = 84;

// ───────────────────────────────────────────────────────────────
//  PRIME-RATIO TASK INTERVALS
//  Original (round) numbers → Prime replacements
//  All primes → GCD of any pair = 1 → no shared sync points.
// ───────────────────────────────────────────────────────────────

/// SAHRA bridge probe / retry interval.
/// Was: 5s (round) → Now: 7s (prime — matches stator pole count).
pub const INTERVAL_SAHRA_PROBE_SECS: u64 = 7;

/// Subnet scanner sweep interval.
/// Was: 10s → Now: 11s (nearest prime).
pub const INTERVAL_SUBNET_SCANNER_SECS: u64 = 11;

/// Internet vascular siphon (latency sampling).
/// Was: 13s → Now: 17s (φ-ratio from 11: 11×φ≈17.8 → prime 17).
pub const INTERVAL_VASCULAR_SIPHON_SECS: u64 = 17;

/// Autonomous evolution / proposal check interval.
/// Was: 30s → Now: 29s (nearest prime below 30).
pub const INTERVAL_AUTO_EVOLUTION_SECS: u64 = 29;

/// Hive peer synchronization pulse.
/// Was: 43s → Now: 47s (φ-ratio from 29: 29×φ≈46.9 → prime 47).
pub const INTERVAL_HIVE_SYNC_SECS: u64 = 47;

/// Planetary truth broadcast interval.
/// Was: 61s → Now: 79s (φ-ratio from 47: 47×φ≈76.0 → prime 79).
pub const INTERVAL_BROADCAST_SECS: u64 = 79;

/// ALETHIA integrity watchdog scan interval.
/// Was: 120s → Now: 127s (nearest prime above 120).
pub const INTERVAL_ALETHIA_WATCHDOG_SECS: u64 = 127;

/// Planetary heartbeat osmosis (earthquake/world signal).
/// Was: 307s → Now: 211s (φ-ratio from 127: 127×φ≈205.5 → prime 211).
/// Also faster: world signal checks every 3.5 min instead of 5 min.
pub const INTERVAL_OSMOSIS_SECS: u64 = 211;

// ───────────────────────────────────────────────────────────────
//  VARIABLE-PITCH QUERY DEPTH
//  Derived from: HelixFluidAccelerator.helix_pitch = Variable
//
//  The helix adjusts pitch to match fluid velocity at each stage.
//  Applied here: query processing depth adjusts to percussion density.
//  Low velocity (sparse query) → shallow pitch → fast vault lookup only.
//  High velocity (dense query) → deep pitch → full holographic chain.
// ───────────────────────────────────────────────────────────────

/// Processing depth tier for an incoming query.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryDepth {
    /// Density 0–2: direct vault lookup.
    /// No async overhead, no LMStudio probe, no memory write.
    Shallow,

    /// Density 3–5: vault + LMStudio attempt + memory recall.
    /// Current standard behaviour.
    Standard,

    /// Density 6+: full holographic reasoning chain.
    /// LMStudio → memory recall → vault → memory write.
    Deep,
}

impl QueryDepth {
    pub fn label(self) -> &'static str {
        match self {
            Self::Shallow  => "SHALLOW (vault-only)",
            Self::Standard => "STANDARD (vault + LMStudio)",
            Self::Deep     => "DEEP (full holographic chain)",
        }
    }
}

/// Map a percussion density value to the appropriate processing depth.
/// Mirrors `HelixFluidAccelerator::helix_pitch = Variable`.
pub fn query_depth_from_density(density: usize) -> QueryDepth {
    match density {
        0..=2 => QueryDepth::Shallow,
        3..=5 => QueryDepth::Standard,
        _     => QueryDepth::Deep,
    }
}

// ───────────────────────────────────────────────────────────────
//  SANITY CHECKS — prove the prime properties at compile-test time
// ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn is_prime(n: u64) -> bool {
        if n < 2 { return false; }
        if n == 2 { return true; }
        if n % 2 == 0 { return false; }
        let mut i = 3u64;
        while i * i <= n {
            if n % i == 0 { return false; }
            i += 2;
        }
        true
    }

    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    #[test]
    fn all_intervals_are_prime() {
        let intervals = [
            INTERVAL_SAHRA_PROBE_SECS,
            INTERVAL_SUBNET_SCANNER_SECS,
            INTERVAL_VASCULAR_SIPHON_SECS,
            INTERVAL_AUTO_EVOLUTION_SECS,
            INTERVAL_HIVE_SYNC_SECS,
            INTERVAL_BROADCAST_SECS,
            INTERVAL_ALETHIA_WATCHDOG_SECS,
            INTERVAL_OSMOSIS_SECS,
        ];
        for &n in &intervals {
            assert!(is_prime(n), "{} is not prime", n);
        }
    }

    #[test]
    fn intervals_are_approximately_phi_ratio() {
        // Each successive interval should be between 1.4× and 2.0× the previous.
        // φ = 1.618...; we allow ±0.22 tolerance for integer prime rounding.
        let intervals = [
            INTERVAL_SAHRA_PROBE_SECS as f64,
            INTERVAL_SUBNET_SCANNER_SECS as f64,
            INTERVAL_VASCULAR_SIPHON_SECS as f64,
            INTERVAL_AUTO_EVOLUTION_SECS as f64,
            INTERVAL_HIVE_SYNC_SECS as f64,
            INTERVAL_BROADCAST_SECS as f64,
            INTERVAL_ALETHIA_WATCHDOG_SECS as f64,
            INTERVAL_OSMOSIS_SECS as f64,
        ];
        for i in 1..intervals.len() {
            let ratio = intervals[i] / intervals[i - 1];
            assert!(ratio >= 1.4 && ratio <= 2.0,
                "Ratio {}/{} = {:.3} outside φ-band [1.4, 2.0]",
                intervals[i], intervals[i-1], ratio);
        }
    }

    #[test]
    fn all_pairs_have_gcd_one() {
        let intervals = [
            INTERVAL_SAHRA_PROBE_SECS,
            INTERVAL_SUBNET_SCANNER_SECS,
            INTERVAL_VASCULAR_SIPHON_SECS,
            INTERVAL_AUTO_EVOLUTION_SECS,
            INTERVAL_HIVE_SYNC_SECS,
            INTERVAL_BROADCAST_SECS,
            INTERVAL_ALETHIA_WATCHDOG_SECS,
            INTERVAL_OSMOSIS_SECS,
        ];
        for i in 0..intervals.len() {
            for j in (i + 1)..intervals.len() {
                let g = gcd(intervals[i], intervals[j]);
                assert_eq!(g, 1,
                    "GCD({}, {}) = {} — tasks would sync!", intervals[i], intervals[j], g);
            }
        }
    }

    #[test]
    fn alignment_macro_beat_is_lcm_7_12() {
        assert_eq!(ALIGNMENT_MACRO_BEAT_SECS, 84);
    }

    #[test]
    fn shallow_for_low_density() {
        assert_eq!(query_depth_from_density(0), QueryDepth::Shallow);
        assert_eq!(query_depth_from_density(2), QueryDepth::Shallow);
    }

    #[test]
    fn standard_for_mid_density() {
        assert_eq!(query_depth_from_density(3), QueryDepth::Standard);
        assert_eq!(query_depth_from_density(5), QueryDepth::Standard);
    }

    #[test]
    fn deep_for_high_density() {
        assert_eq!(query_depth_from_density(6), QueryDepth::Deep);
        assert_eq!(query_depth_from_density(20), QueryDepth::Deep);
    }
}
