use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

/// IDENTITY REGISTRY (V-132.8)
/// Maps DIDs to Ed25519 Public Keys for forensic verification.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IdentityRegistry {
    pub identities: HashMap<String, [u8; 32]>,
}

pub struct RegistryConfig {
    pub base_path: PathBuf,
}

impl Default for RegistryConfig {
    fn default() -> Self {
        let root = std::env::var("SOVEREIGN_DATA_ROOT")
            .unwrap_or_else(|_| "C:\\GENESIS".to_string());
        Self {
            base_path: PathBuf::from(root).join("brain_scars"),
        }
    }
}

impl IdentityRegistry {
    fn get_path() -> PathBuf {
        RegistryConfig::default().base_path.join("identities.json")
    }

    pub fn load() -> Result<Self> {
        let path = Self::get_path();
        if !path.exists() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut registry = Self::default();
            // Register the Legacy Migration Auditor (Phase 44 Calibration)
            let migration_pk = [
                0x08, 0x7D, 0xFE, 0xE7, 0xCD, 0x8B, 0x97, 0xF2, 
                0x78, 0x2D, 0xF8, 0x22, 0x79, 0xD2, 0x89, 0x47,
                0xBE, 0x01, 0x16, 0xEB, 0xFA, 0xE4, 0xDD, 0x95,
                0x4C, 0x29, 0xEB, 0x5D, 0xEC, 0x9F, 0xA5, 0x0C,
            ];
            registry.identities.insert("did:sov:legacy_migrator_v132".to_string(), migration_pk);
            
            // Register the Root Forensic Anchor (Phase 45 Calibration)
            let root_pk = [
                0x03, 0xA1, 0x07, 0xBF, 0xF3, 0xCE, 0x10, 0xBE, 
                0x1D, 0x70, 0xDD, 0x18, 0xE7, 0x4B, 0xC0, 0x99,
                0x67, 0xE4, 0xD6, 0x30, 0x9B, 0xA5, 0x0D, 0x5F,
                0x1D, 0xDC, 0x86, 0x64, 0x12, 0x55, 0x31, 0xB8,
            ];
            registry.identities.insert("did:sov:root_anchor".to_string(), root_pk);

            // Register Sarah-1T's forensic proof anchor
            let sarah_pk = [
                0x0D, 0x75, 0x50, 0x75, 0x4E, 0x08, 0x00, 0xA5, 
                0xD2, 0x37, 0xEE, 0xF5, 0x82, 0x60, 0x35, 0x76,
                0x6B, 0x9B, 0x3E, 0x5A, 0x15, 0x86, 0x8A, 0x94,
                0x0A, 0xB2, 0x89, 0x95, 0x87, 0x88, 0xE3, 0xB0,
            ];
            registry.identities.insert("did:sov:sarah_1t".to_string(), sarah_pk);

            registry.save()?;
            return Ok(registry);
        }

        let data = fs::read_to_string(path)?;
        let registry = serde_json::from_str(&data)?;
        Ok(registry)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::get_path();
        let data = serde_json::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn resolve_key(&self, did: &str) -> Option<[u8; 32]> {
        self.identities.get(did).cloned()
    }

    pub fn resolve_key_by_hash(&self, agent_id_hash: u64) -> Option<[u8; 32]> {
        for (did, public_key) in &self.identities {
            let hash_bytes = md5::compute(did.as_bytes());
            let computed_hash = u64::from_le_bytes(hash_bytes.as_slice()[0..8].try_into().unwrap());
            if computed_hash == agent_id_hash {
                return Some(*public_key);
            }
        }
        None
    }

    pub fn register_identity(&mut self, did: String, public_key: [u8; 32]) -> Result<()> {
        self.identities.insert(did, public_key);
        self.save()
    }
}
