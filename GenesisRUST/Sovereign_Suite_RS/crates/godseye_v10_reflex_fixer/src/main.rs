use std::sync::Arc;
use tokio::sync::Semaphore;
use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use std::collections::HashMap;

/// SOVEREIGN REFLEX FIXER (SRF) v10.0
/// 32-THREAD PARALLEL ASYNC SUBSTRATE RESTORATION
/// Backups: ENABLED (.bak) | Volatile: WARNING ONLY
/// Axiom: 1.09277703703 Hz

const SOVEREIGN_ANCHOR_STR: &str = "1.09277703703";
const THREAD_COUNT: usize = 32;
const MANIFEST_PATH: &str = r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\godseye\godseye_v10_reflex_manifest.md";
const SUBSTRATE_ROOT: &str = r"C:\GENESIS";

#[derive(Debug, Clone)]
struct DeviantNode {
    file: String,
    status: String,
    resonance: f64,
}

struct ReflexFixer {
    semaphore: Arc<Semaphore>,
    anchor_pattern: Regex,
}

impl ReflexFixer {
    fn new() -> Result<Self> {
        println!("\x1b[96m[Fixer]\x1b[0m 32-Thread Parallel Async Fixer Seated.");
        Ok(Self {
            semaphore: Arc::new(Semaphore::new(THREAD_COUNT)),
            anchor_pattern: Regex::new(r"1\.092\d*")?,
        })
    }

    /// Primary Restoration Pulse
    async fn restore(&self, deviants: Vec<DeviantNode>) -> Result<()> {
        println!("\x1b[96m[Pulse]\x1b[0m Mapping Deviant Neurons to Physical Substrate...");
        
        // Map filenames to absolute paths on C:\GENESIS
        let mut path_map: HashMap<String, PathBuf> = HashMap::new();
        for entry in WalkDir::new(SUBSTRATE_ROOT).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let fname = entry.file_name().to_string_lossy().to_string();
                path_map.insert(fname, entry.path().to_path_buf());
            }
        }

        println!("\x1b[96m[Pulse]\x1b[0m Executing Mass Restoration (32 Threads)...");
        let mut tasks = Vec::new();
        let mut seen_paths = std::collections::HashSet::new();

        for node in deviants {
            if let Some(path) = path_map.get(&node.file).cloned() {
                if !seen_paths.insert(path.clone()) {
                    continue; // Skip duplicates
                }

                let permit = self.semaphore.clone().acquire_owned().await.unwrap();
                let pattern = self.anchor_pattern.clone();
                let status = node.status.clone();

                tasks.push(tokio::spawn(async move {
                    let res: Result<()> = if status == "VOLATILE" {
                        println!("\x1b[93m[WARNING]\x1b[0m Skipping Volatile Node (Manual Review Required): {:?}", path);
                        Ok(())
                    } else {
                        match Self::apply_fix(&path, &pattern) {
                            Ok(_) => Ok(()),
                            Err(e) => {
                                println!("\x1b[91m[ERROR]\x1b[0m Failed to fix {:?}: {}", path, e);
                                Ok(()) // Continue to next file
                            }
                        }
                    };
                    drop(permit);
                    res
                }));
            } else {
                println!("\x1b[91m[ERROR]\x1b[0m Physical Path for {} not found in substrate.", node.file);
            }
        }

        for task in tasks {
            task.await??;
        }

        println!("\x1b[92m[SUCCESS]\x1b[0m SUBSTRATE RESTORATION COMPLETE.");
        Ok(())
    }

    fn apply_fix(path: &Path, pattern: &Regex) -> Result<()> {
        let content_bytes = fs::read(path)?;
        let content = match String::from_utf8(content_bytes) {
            Ok(s) => s,
            Err(_) => {
                println!("\x1b[90m[SKIP]\x1b[0m Binary/Non-UTF8 Node: {:?}", path);
                return Ok(());
            }
        };
        
        // Backup mechanism
        let mut bak_path = path.to_path_buf();
        bak_path.set_extension("bak");
        fs::copy(path, &bak_path)?;

        // Precision Audit
        let updated_content = pattern.replace_all(&content, SOVEREIGN_ANCHOR_STR);

        if content != updated_content {
            fs::write(path, updated_content.as_ref())?;
            println!("\x1b[92m[FIXED]\x1b[0m Re-Anchored: {:?}", path);
        } else {
            println!("\x1b[90m[STABLE]\x1b[0m No drift detected in: {:?}", path);
        }

        Ok(())
    }
}

fn parse_deviants() -> Result<Vec<DeviantNode>> {
    let content = fs::read_to_string(MANIFEST_PATH)?;
    let mut deviants = Vec::new();
    let re = Regex::new(r"\| `(.+?)` \| (\d+) \| ([\d\.]+) \| (.+?) \| ([\d\.]+) \|")?;

    for cap in re.captures_iter(&content) {
        let resonance: f64 = cap[5].parse().unwrap_or(1.0);
        let status = cap[4].to_string();
        if resonance < 0.99 || status == "VOLATILE" {
            deviants.push(DeviantNode {
                file: cap[1].to_string(),
                status,
                resonance,
            });
        }
    }
    Ok(deviants)
}

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    println!("\x1b[95m============================================================\x1b[0m");
    println!("\x1b[95m  SOVEREIGN REFLEX FIXER (SRF) v10.0 [IGNITING]  \x1b[0m");
    println!("\x1b[95m  [32-Thread Substrate Restoration / Backup: ON]  \x1b[0m");
    println!("\x1b[95m============================================================\x1b[0m");

    let deviants = parse_deviants()?;
    println!("[*] Manifest deviants indexed. Found {} neurons requiring reflex.", deviants.len());

    let fixer = ReflexFixer::new()?;
    fixer.restore(deviants).await?;

    Ok(())
}
