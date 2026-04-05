use serde::{Serialize, Deserialize};
use serde_json;
use sha2::{Sha256, Digest};
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WormEntry {
    pub timestamp: f64,
    pub thought: serde_json::Value,
    pub prev_hash: String,
    pub hash: String,
}

pub struct SovereignWorm {
    vault_path: PathBuf,
    chain_hash: Arc<Mutex<String>>,
    sender: mpsc::Sender<WormEntry>,
}

impl SovereignWorm {
    pub async fn new(vault_path: &str) -> Self {
        let path = PathBuf::from(vault_path);
        let (tx, mut rx) = mpsc::channel::<WormEntry>(100);
        let chain_hash = Arc::new(Mutex::new("0".repeat(64)));

        let s_path = path.clone();
        let s_chain_hash = Arc::clone(&chain_hash);

        // Load existing chain to seat the hash
        if s_path.exists() {
            if let Ok(file) = File::open(&s_path) {
                let reader = BufReader::new(file);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        if let Ok(entry) = serde_json::from_str::<WormEntry>(&l) {
                            let mut lock = s_chain_hash.lock().unwrap();
                            *lock = entry.hash;
                        }
                    }
                }
            }
        } else {
             // Genesis logic would go here if file didn't exist
        }

        // Background flush worker (Rust version of _flush_worker)
        let worker_path = s_path.clone();
        tokio::spawn(async move {
            while let Some(entry) = rx.recv().await {
                if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&worker_path) {
                    if let Ok(json) = serde_json::to_string(&entry) {
                        let _ = writeln!(file, "{}", json);
                    }
                }
            }
        });

        SovereignWorm {
            vault_path: path,
            chain_hash,
            sender: tx,
        }
    }

    pub async fn seal(&self, thought: serde_json::Value) -> String {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64();
        let mut lock = self.chain_hash.lock().unwrap();
        let prev_hash = lock.clone();

        let mut entry = WormEntry {
            timestamp,
            thought,
            prev_hash: prev_hash.clone(),
            hash: String::new(),
        };

        // Calculate Hash
        let mut hasher = Sha256::new();
        let entry_json = serde_json::json!({
            "timestamp": entry.timestamp,
            "thought": entry.thought,
            "prev_hash": entry.prev_hash
        });
        hasher.update(serde_json::to_string(&entry_json).unwrap());
        let hash_str = format!("{:x}", hasher.finalize());
        
        entry.hash = hash_str.clone();
        *lock = hash_str.clone();

        let _ = self.sender.send(entry).await;
        hash_str
    }
}
