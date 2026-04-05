pub mod hub;
pub mod ingestor;
pub mod supabase_mcp;

pub use hub::SkillsHub;
pub use ingestor::{SarahSkill, SkillIngestor, SkillMetadata};
pub use supabase_mcp::integrate_supabase_substrate;
