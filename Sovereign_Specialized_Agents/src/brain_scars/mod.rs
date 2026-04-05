use serde::{Serialize, Deserialize};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LogicFragment {
    pub id: String,
    pub domain: String,
    pub raw_logic: String,
    pub score: f64,
    pub source: String,
    pub timestamp: u64,
}

pub struct BrainScarVault {
    base_path: PathBuf,
}

impl BrainScarVault {
    pub fn new() -> Result<Self> {
        let base_path = PathBuf::from("C:\\GENESIS\\brain_scars");
        if !base_path.exists() {
            fs::create_dir_all(&base_path)?;
            fs::create_dir_all(base_path.join("research"))?;
            fs::create_dir_all(base_path.join("coding"))?;
            fs::create_dir_all(base_path.join("security"))?;
            fs::create_dir_all(base_path.join("internet"))?;
            fs::create_dir_all(base_path.join("theory"))?;
        }
        Ok(Self { base_path })
    }

    pub fn store_fragment(&self, fragment: LogicFragment) -> Result<()> {
        let domain_path = self.base_path.join(&fragment.domain);
        if !domain_path.exists() {
            fs::create_dir_all(&domain_path)?;
        }
        
        let file_path = domain_path.join(format!("{}.json", fragment.id));
        let encoded = serde_json::to_string_pretty(&fragment)?;
        fs::write(file_path, encoded)?;
        Ok(())
    }

    pub fn load_fragments(&self, domain: &str) -> Result<Vec<LogicFragment>> {
        let domain_path = self.base_path.join(domain);
        if !domain_path.exists() {
            return Ok(vec![]);
        }

        let mut fragments = Vec::new();
        for entry in fs::read_dir(domain_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "json") {
                let content = fs::read_to_string(path)?;
                let fragment: LogicFragment = serde_json::from_str(&content)?;
                fragments.push(fragment);
            }
        }
        Ok(fragments)
    }

    pub fn get_highest_scored(&self, domain: &str) -> Result<Option<LogicFragment>> {
        let mut fragments = self.load_fragments(domain)?;
        fragments.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Ok(fragments.into_iter().next())
    }

    /// MEILISEARCH APERTURE: Keyword-based local search
    pub fn query(&self, query_str: &str) -> Result<Vec<LogicFragment>> {
        let mut results = Vec::new();
        let query_lower = query_str.to_lowercase();
        
        // Recursively search all domains
        for domain in ["research", "coding", "security", "internet"] {
            let fragments = self.load_fragments(domain)?;
            for fragment in fragments {
                if fragment.id.to_lowercase().contains(&query_lower) || 
                   fragment.raw_logic.to_lowercase().contains(&query_lower) {
                    results.push(fragment);
                }
            }
        }
        
        // Sort results by score (highest-principal first)
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        Ok(results)
    }
}
