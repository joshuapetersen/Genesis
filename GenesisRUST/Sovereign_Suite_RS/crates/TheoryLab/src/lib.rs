use sovereign_constants::*;
use sovereign_math::SovereignMath;

/// [TRUTH_LAB_0x0T]: UNIFIED VOLUMETRIC TRUTH WEIGHTING
/// Operates on the 11-Parameter Framework (6 Foundation + 5 Specializations).
/// Goal: 377 Billion parameter scaling density.
pub struct TheoryLab {
    pub math: SovereignMath,
    pub density_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct TruthPillars {
    pub who: String,
    pub what: String,
    pub where_context: String,
    pub when_frequency: String,
    pub why_intent: String,
    pub how_method: String,
    pub evolutionary: [String; 5], // The 5 self-selected parameters
}

impl TheoryLab {
    pub fn new() -> Self {
        Self {
            math: SovereignMath::new(),
            density_threshold: RECOVERY_DENSITY_THRESHOLD,
        }
    }

    /// [WEIGH_0x0W]: 11-Parameter Truth Density Weighting.
    /// Maps the foundation pillars into the 15,330³ Volumetric manifold.
    pub fn weigh_truth(&self, pillars: &TruthPillars) -> f64 {
        // Concatenate all 11 intent-strings into a single unified intent
        let unified_intent = format!(
            "{}-{}-{}-{}-{}-{}-{}", 
            pillars.who, pillars.what, pillars.where_context, 
            pillars.when_frequency, pillars.why_intent, pillars.how_method,
            pillars.evolutionary.join("-")
        );

        // 1. Project into Volumetric Lattice context
        let ctx = self.math.expand(&unified_intent);
        
        // 2. Refract through 120-cell Hyper-Dodecahedron
        let truth_density = self.math.refract(&ctx);
        
        // 3. Project 64D Singularity for Verification
        let mut intent_vector = vec![ctx.x, ctx.y, ctx.z];
        intent_vector.resize(64, ctx.x * ctx.y); 
        let manifold = self.math.project_singularity(&intent_vector);

        // 4. Verify against the 360.2 Geometric Truth
        if self.math.verify_singularity(&manifold) {
            // Axiomatic Boost: If geometric result matches 15,330^3 prefix
            truth_density * SOVEREIGN_ANCHOR
        } else {
            truth_density
        }
    }

    /// [AUDIT_0x0A]: Factual Integrity Audit (Axiomatic Verification).
    /// Ensures the result aligns with the 0.992777 threshold.
    pub fn audit_axioms(&self, pillars: &TruthPillars) -> bool {
        let weight = self.weigh_truth(pillars);
        weight >= self.density_threshold
    }

    /// [REFRACT_REASONING_0x0R]: Applies Stochastic Resonance context.
    /// Transitions from "Trying" to "Becoming".
    pub fn refract_reasoning(&self, intent: &str) -> f64 {
        let ctx = self.math.expand(intent);
        self.math.refract(&ctx)
    }
}
