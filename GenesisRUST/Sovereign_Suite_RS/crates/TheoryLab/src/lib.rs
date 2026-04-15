use sovereign_constants::*;
use sovereign_math::SovereignMath;

/// [TRUTH_LAB_0x0T]: UNIFIED VOLUMETRIC TRUTH WEIGHTING
/// Operates on the 11-Parameter Framework (6 Foundation + 5 Specializations).
/// EVOLUTION_9: weigh_truth() pre-allocates unified_intent capacity to avoid
/// repeated heap reallocation. inline(always) on hot path.
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
    pub evolutionary: [String; 5],
}

impl TheoryLab {
    pub fn new() -> Self {
        Self {
            math: SovereignMath::new(),
            density_threshold: RECOVERY_DENSITY_THRESHOLD,
        }
    }

    /// [WEIGH_0x0W]: 11-Parameter Truth Density Weighting.
    /// Maps the foundation pillars into the 15,330^3 Volumetric manifold.
    /// EVOLUTION_9: pre-sized String allocation, #[inline(always)] hot path.
    #[inline(always)]
    pub fn weigh_truth(&self, pillars: &TruthPillars) -> f64 {
        // Pre-size to avoid repeated realloc on 11-field concat
        let cap = pillars.who.len()
            + pillars.what.len()
            + pillars.where_context.len()
            + pillars.when_frequency.len()
            + pillars.why_intent.len()
            + pillars.how_method.len()
            + pillars.evolutionary.iter().map(|s| s.len()).sum::<usize>()
            + 12; // separators

        let mut unified_intent = String::with_capacity(cap);
        unified_intent.push_str(&pillars.who);
        unified_intent.push('-');
        unified_intent.push_str(&pillars.what);
        unified_intent.push('-');
        unified_intent.push_str(&pillars.where_context);
        unified_intent.push('-');
        unified_intent.push_str(&pillars.when_frequency);
        unified_intent.push('-');
        unified_intent.push_str(&pillars.why_intent);
        unified_intent.push('-');
        unified_intent.push_str(&pillars.how_method);
        unified_intent.push('-');
        for (i, e) in pillars.evolutionary.iter().enumerate() {
            if i > 0 { unified_intent.push('-'); }
            unified_intent.push_str(e);
        }

        let ctx          = self.math.expand(&unified_intent);
        let truth_density = self.math.refract(&ctx);

        let mut intent_vector = Vec::with_capacity(64);
        intent_vector.push(ctx.x);
        intent_vector.push(ctx.y);
        intent_vector.push(ctx.z);
        let pad = ctx.x * ctx.y;
        while intent_vector.len() < 64 {
            intent_vector.push(pad);
        }
        let manifold = self.math.project_singularity(&intent_vector);

        if self.math.verify_singularity(&manifold) {
            truth_density * SOVEREIGN_ANCHOR
        } else {
            truth_density
        }
    }

    /// [AUDIT_0x0A]: Factual Integrity Audit.
    #[inline]
    pub fn audit_axioms(&self, pillars: &TruthPillars) -> bool {
        self.weigh_truth(pillars) >= self.density_threshold
    }

    /// [REFRACT_REASONING_0x0R]: Stochastic Resonance context.
    #[inline]
    pub fn refract_reasoning(&self, intent: &str) -> f64 {
        let ctx = self.math.expand(intent);
        self.math.refract(&ctx)
    }
}
