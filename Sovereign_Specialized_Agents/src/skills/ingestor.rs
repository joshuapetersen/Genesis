use std::fs;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub risk: String,
    pub source: String,
    pub date_added: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SarahSkill {
    pub metadata: SkillMetadata,
    pub instructions: String,
    pub resonance: f64, // 1.09277703703703 HZ
}

pub struct SkillIngestor;

impl SkillIngestor {
    pub fn ingest_skill(path: &Path) -> Option<SarahSkill> {
        let content = fs::read_to_string(path).ok()?;
        let parts: Vec<&str> = content.split("---").collect();
        
        if parts.len() < 3 { return None; }
        
        // Stage 1: Frontmatter Ingestion (YAML)
        let metadata: SkillMetadata = serde_yaml::from_str(parts[1]).ok()?;
        
        // Stage 2: Instruction Ingestion (Markdown)
        let instructions = parts[2].trim().to_string();
        
        Some(SarahSkill {
            metadata,
            instructions,
            resonance: 1.09277703703703,
        })
    }
}
