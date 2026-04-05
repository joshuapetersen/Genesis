use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

/// BRAIN SCARS: PERSISTENT EXPERIENCE REPOSITORY
/// V-40.0 SYMBIO-STRIKE
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BrainScar {
    pub pattern_id: String,
    pub logic_hash: String,
    pub resonance_score: f64,
    pub first_principal: bool,
    pub timestamp: u64,
}

pub struct BrainScarsVault {
    storage_path: PathBuf,
}

impl BrainScarsVault {
    pub fn new(domain: &str) -> Self {
        let storage_path = PathBuf::from(format!("src/brain_scars/{}", domain));
        fs::create_dir_all(&storage_path).ok();
        Self { storage_path }
    }

    pub fn save_scar(&self, scar: &BrainScar) -> Result<()> {
        let file_path = self.storage_path.join(format!("{}.json", scar.pattern_id));
        let content = serde_json::to_string_pretty(scar)?;
        fs::write(file_path, content)?;
        Ok(())
    }

    pub fn load_scars(&self) -> Result<Vec<BrainScar>> {
        let mut scars = Vec::new();
        for entry in fs::read_dir(&self.storage_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let content = fs::read_to_string(path)?;
                let scar: BrainScar = serde_json::from_str(&content)?;
                scars.push(scar);
            }
        }
        Ok(scars)
    }
}
