use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};

/// IDENTITY REGISTRY (V-133.0)
/// Maps DIDs and Truncated Hashes to Ed25519 Public Keys for forensic verification.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct IdentityRegistry {
    pub identities: HashMap<String, [u8; 32]>, // JSON keys must be strings
    pub did_map: HashMap<String, u64>, // Map DID string to the hash for lookup
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
            // Register Core Anchors
            registry.register_identity("did:sov:root_anchor".to_string(), [0u8; 32])?; // Placeholder
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
        self.did_map.get(did).and_then(|hash| self.identities.get(&hash.to_string()).cloned())
    }

    pub fn resolve_key_by_hash(&self, agent_id_hash: u64) -> Option<[u8; 32]> {
        self.identities.get(&agent_id_hash.to_string()).cloned()
    }

    pub fn register_identity(&mut self, did: String, public_key: [u8; 32]) -> Result<()> {
        // V-133.0: Derive the 64-bit lattice hash from the identity ID (DID suffix or blake3)
        // For consistency with AgentFactory, we'll use the first 8 bytes of the blake3 hash of the DID
        let hash_bytes = lib_crypto::hash_blake3(did.as_bytes());
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&hash_bytes[0..8]);
        let agent_id_hash = u64::from_le_bytes(id_bytes);

        self.identities.insert(agent_id_hash.to_string(), public_key);
        self.did_map.insert(did, agent_id_hash);
        self.save()
    }

    /// Register directly by hash (used by Factory for new agents)
    pub fn register_by_hash(&mut self, agent_id_hash: u64, public_key: [u8; 32]) -> Result<()> {
        self.identities.insert(agent_id_hash.to_string(), public_key);
        self.save()
    }
}
