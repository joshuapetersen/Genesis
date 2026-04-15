// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES CORE FRAMEWORK — Stable Build 2026
//  Architect: Josh | Partner: Sarah (Gemini Handshake Protocol)
//  Lineage: Derik & Dylan (Baritone D-Lineage)
// ═══════════════════════════════════════════════════════════════

pub mod engineering;
pub mod scheduler;
pub mod phi;

/// The phonetic percussion target consonants — P, B, T, D, K, G.
/// Priority order matches punch weight in the cadence engine.
pub const PERCUSSION_CONSONANTS: &[char] = &['P', 'B', 'T', 'D', 'K', 'G'];

// ───────────────────────────────────────────────────────────────
//  LYRIC PROTOCOLS
// ───────────────────────────────────────────────────────────────

/// Core rules that govern every bar produced by D.A.B. Industries.
#[derive(Debug, Clone)]
pub struct LyricsProtocols {
    /// Prioritise P, B, T, D, K, G for percussive impact.
    pub phonetic_rule: &'static str,
    /// Imperfect & slant rhymes for grit — no clean sing-song.
    pub authenticity_rule: &'static str,
    /// Heartbeat syncopation: staccato verses / sustained choruses.
    pub cadence: &'static str,
    /// Active rhyme schemes used per song.
    pub rhyme_scheme: &'static [&'static str],
}

impl LyricsProtocols {
    pub fn new() -> Self {
        Self {
            phonetic_rule:     "Phonetic Percussion: Prioritize P, B, T, D, K, G",
            authenticity_rule: "Imperfect & Slant Rhymes for Grit",
            cadence:           "Heartbeat Syncopation: Staccato Verses / Sustained Choruses",
            rhyme_scheme:      &["Cross-rhyme", "Internal"],
        }
    }

    /// Returns true if the bar opens with a percussion consonant.
    pub fn opens_on_beat(&self, bar: &str) -> bool {
        bar.trim_start()
            .chars()
            .next()
            .map(|c| PERCUSSION_CONSONANTS.contains(&c.to_ascii_uppercase()))
            .unwrap_or(false)
    }

    /// Counts how many percussion hits are in a bar.
    pub fn percussion_density(&self, bar: &str) -> usize {
        bar.chars()
            .filter(|c| PERCUSSION_CONSONANTS.contains(&c.to_ascii_uppercase()))
            .count()
    }
}

// ───────────────────────────────────────────────────────────────
//  MODEL ROSTER
// ───────────────────────────────────────────────────────────────

/// Every named model that D.A.B. Industries fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DABModel {
    FastBoom,
    SlowBoom,
    Architect,
    Jr,
    Kid,
    Inspiration,
    JoeDiffie,
    TheJoker,
    LilFlow,
    Tess,
}

impl DABModel {
    /// Human-readable tag used in logs and dispatch.
    pub fn tag(self) -> &'static str {
        match self {
            Self::FastBoom    => "Fast Boom",
            Self::SlowBoom    => "Slow Boom",
            Self::Architect   => "Architect",
            Self::Jr          => "Jr",
            Self::Kid         => "Kid",
            Self::Inspiration => "Inspiration",
            Self::JoeDiffie   => "Joe Diffie",
            Self::TheJoker    => "The Joker",
            Self::LilFlow     => "Lil Flow",
            Self::Tess        => "Tess",
        }
    }

    /// Full ordered roster — same sequence as the founding manifest.
    pub fn all() -> &'static [Self] {
        &[
            Self::FastBoom, Self::SlowBoom, Self::Architect,
            Self::Jr, Self::Kid, Self::Inspiration,
            Self::JoeDiffie, Self::TheJoker, Self::LilFlow, Self::Tess,
        ]
    }
}

// ───────────────────────────────────────────────────────────────
//  PARTNER REGISTRY
// ───────────────────────────────────────────────────────────────

/// Named partner roles in the D-Lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DABPartner {
    Josh,
    DylanBaritone,
    SarahSisterModel,
}

impl DABPartner {
    pub fn tag(self) -> &'static str {
        match self {
            Self::Josh             => "Josh",
            Self::DylanBaritone    => "Dylan (Baritone Partner)",
            Self::SarahSisterModel => "Sarah (Sister-Model)",
        }
    }

    pub fn all() -> &'static [Self] {
        &[Self::Josh, Self::DylanBaritone, Self::SarahSisterModel]
    }
}

// ───────────────────────────────────────────────────────────────
//  LYRIC STRUCTURE: Observation → Reaction → Action
// ───────────────────────────────────────────────────────────────

/// One bar in the DAB pipeline.
/// Rule: no abstract metaphors — use physical objects.
#[derive(Debug, Clone)]
pub struct Bar {
    /// The written line.
    pub text: String,
    /// Which phase of the O→R→A arc this bar occupies.
    pub phase: LyricPhase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LyricPhase {
    Observation,
    Reaction,
    Action,
}

impl LyricPhase {
    pub fn label(self) -> &'static str {
        match self {
            Self::Observation => "OBSERVATION",
            Self::Reaction    => "REACTION",
            Self::Action      => "ACTION",
        }
    }
}

// ───────────────────────────────────────────────────────────────
//  D.A.B. INDUSTRIES — MAIN STRUCT
// ───────────────────────────────────────────────────────────────

/// The D.A.B. Industries engine.  One instance per session.
pub struct DABIndustries {
    /// Founding owner — Derik.
    pub owner: &'static str,
    /// Partner registry (static slice — no heap allocation needed).
    pub partners: &'static [DABPartner],
    /// Full model roster.
    pub models: &'static [DABModel],
    /// Governing lyric protocols.
    pub protocols: LyricsProtocols,
}

impl DABIndustries {
    /// Construct the canonical D.A.B. Industries instance.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            owner:     "Derik",
            partners:  &[
                DABPartner::Josh,
                DABPartner::DylanBaritone,
                DABPartner::SarahSisterModel,
            ],
            models:    &[
                DABModel::FastBoom, DABModel::SlowBoom, DABModel::Architect,
                DABModel::Jr, DABModel::Kid, DABModel::Inspiration,
                DABModel::JoeDiffie, DABModel::TheJoker, DABModel::LilFlow, DABModel::Tess,
            ],
            protocols: LyricsProtocols::new(),
        }
    }

    /// Print operational header to stdout.
    pub fn execute_lyric_logic(&self) {
        println!("Initializing D.A.B. Industries Logic...");
        println!("Owner         : {}", self.owner);
        println!("Partners      : {}", self.partners.iter().map(|p| p.tag()).collect::<Vec<_>>().join(", "));
        println!("Models        : {}", self.models.iter().map(|m| m.tag()).collect::<Vec<_>>().join(", "));
        println!("Phonetic Rule : {}", self.protocols.phonetic_rule);
        println!("Authenticity  : {}", self.protocols.authenticity_rule);
        println!("Cadence       : {}", self.protocols.cadence);
        println!("Rhyme Schemes : {}", self.protocols.rhyme_scheme.join(", "));
        println!("---");
        println!("Current Mode  : Aggressive Freedom / High-Octane Fuel");
        println!("Structure     : Observation -> Reaction -> Action");
        println!("Warning       : No abstract metaphors. Use physical objects.");
    }

    /// Run a bar through the protocol validator.
    /// Returns a score 0–100 based on percussion density and ORA phase tagging.
    pub fn validate_bar(&self, bar: &Bar) -> u8 {
        let density = self.protocols.percussion_density(&bar.text);
        // φ-curve: (1 - 1/φ^density) × 80 — smooth asymptote, no hard cap at 5 hits.
        let density_score = crate::phi::phi_density_score(density);
        // Phase bonus: all three phases in sequence earn extra points.
        let phase_bonus: u8 = match bar.phase {
            LyricPhase::Observation => 5,
            LyricPhase::Reaction    => 10,
            LyricPhase::Action      => 20,
        };
        density_score.saturating_add(phase_bonus).min(100)
    }

    /// Dispatch a named model to process a bar.
    /// Returns the model tag and validation score for logging.
    pub fn dispatch(&self, model: DABModel, bar: &Bar) -> (&'static str, u8) {
        let score = self.validate_bar(bar);
        println!(
            "[{}] Phase={} | Score={}/100 | \"{}\"",
            model.tag(), bar.phase.label(), score, bar.text
        );
        (model.tag(), score)
    }
}

impl Default for DABIndustries {
    fn default() -> Self {
        Self::new()
    }
}

// ───────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_is_derik() {
        assert_eq!(DABIndustries::new().owner, "Derik");
    }

    #[test]
    fn ten_models_registered() {
        assert_eq!(DABModel::all().len(), 10);
    }

    #[test]
    fn three_partners_registered() {
        assert_eq!(DABPartner::all().len(), 3);
    }

    #[test]
    fn percussion_density_counts_correctly() {
        let proto = LyricsProtocols::new();
        // "Boots hit the pavement, tires bite the tar"
        let bar = "Boots hit the pavement, tires bite the tar";
        let d = proto.percussion_density(bar);
        assert!(d > 0, "Expected percussion hits, got 0");
    }

    #[test]
    fn validate_bar_scores_action_phase_highest() {
        let dab = DABIndustries::new();
        let obs = Bar { text: "Boots on the ground".into(), phase: LyricPhase::Observation };
        let act = Bar { text: "Boots on the ground".into(), phase: LyricPhase::Action };
        assert!(dab.validate_bar(&act) > dab.validate_bar(&obs));
    }

    #[test]
    fn dispatch_returns_correct_model_tag() {
        let dab = DABIndustries::new();
        let bar = Bar { text: "Barrel kicked the door down".into(), phase: LyricPhase::Action };
        let (tag, _score) = dab.dispatch(DABModel::FastBoom, &bar);
        assert_eq!(tag, "Fast Boom");
    }
}
