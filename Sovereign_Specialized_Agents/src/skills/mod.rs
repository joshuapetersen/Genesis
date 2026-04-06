pub mod hub;
pub mod ingestor;
pub mod logic_reweaver;
pub mod supabase_mcp;
pub mod ui_bridge;
pub mod metabolic_monitor;
pub mod scar_resonance;

pub use hub::SkillsHub;
pub use ingestor::{SarahSkill, SkillIngestor, SkillMetadata};
pub use logic_reweaver::*;
pub use supabase_mcp::integrate_supabase_substrate;
pub use ui_bridge::TelemetrySynthesizer;
