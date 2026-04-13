use std::sync::Arc;
use tokio::sync::Semaphore;
use anyhow::{Result, Context, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::Deserialize;

/// UNIVERSAL SOVEREIGN ENGINE (USE) - PATH RE-SEATER (SPR)
/// 32-THREAD PARALLEL ASYNC HIGH-SPEED RE-ANCHORING

const THREAD_COUNT: usize = 32;

#[derive(Deserialize)]
pub struct NexusConfig {
    pub substrate: SubstrateConfig,
}

#[derive(Deserialize)]
pub struct SubstrateConfig {
    pub priority_roots: Vec<String>,
}

pub struct PathReSeater {
    semaphore: Arc<Semaphore>,
    nexus_root: PathBuf,
    config: NexusConfig,
}

impl PathReSeater {
    pub fn new() -> Result<Self> {
        let nexus_root = Self::find_nexus_root()
            .context("Substrate is adrift. No sovereign.nexus anchor found.")?;
        
        let config_raw = fs::read_to_string(nexus_root.join("sovereign.nexus"))?;
        let config: NexusConfig = toml::from_str(&config_raw)?;

        println!("\x1b[95m[Re-Seater]\x1b[0m Universal Re-Anchoring Engine Ignite.");
        println!("\x1b[95m[Nexus]\x1b[0m Anchored at {:?}", nexus_root);

        Ok(Self {
            semaphore: Arc::new(Semaphore::new(THREAD_COUNT)),
            nexus_root,
            config,
        })
    }

    fn find_nexus_root() -> Option<PathBuf> {
        let mut curr = std::env::current_dir().ok()?;
        loop {
            if curr.join("sovereign.nexus").exists() {
                return Some(curr);
            }
            if !curr.pop() { break; }
        }
        None
    }

    /// Primary Unified Audit and Fix
    pub async fn audit_and_fix(&self) -> Result<usize> {
        let mut files_to_scan = Vec::new();

        for rel_root in &self.config.substrate.priority_roots {
            let abs_root = self.nexus_root.join(rel_root);
            if !abs_root.exists() { continue; }
            
            println!("\x1b[96m[Indexing]\x1b[0m Scanning domain: {:?}", rel_root);
            for entry in WalkDir::new(abs_root).into_iter().filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != "target" && name != "node_modules" && name != ".git" && 
                !name.ends_with(".exe") && !name.ends_with(".dll")
            }).filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files_to_scan.push(entry.path().to_path_buf());
                }
            }
        }

        println!("\x1b[95m[Pulse]\x1b[0m Found {} candidate files. Auditing for legacy paths...", files_to_scan.len());

        let mut job_handles = Vec::new();
        let target_root_str = self.nexus_root.to_string_lossy().to_string();

        for path in files_to_scan {
            let permit = self.semaphore.clone().acquire_owned().await.unwrap();
            let root_clone = target_root_str.clone();
            job_handles.push(tokio::spawn(async move {
                let res = Self::re_seat_file(&path, &root_clone);
                drop(permit);
                res
            }));
        }

        let mut fixed_count = 0;
        for handle in job_handles {
            if let Ok(Ok(true)) = handle.await {
                fixed_count += 1;
            }
        }

        if fixed_count > 0 {
            println!("\x1b[92m[SUCCESS]\x1b[0m Universal Re-Anchoring Complete. {} files modernized.", fixed_count);
        } else {
            println!("\x1b[90m[Re-Seater]\x1b[0m Zero legacy paths detected. Substrate is pristine.");
        }
        
        Ok(fixed_count)
    }

    fn re_seat_file(path: &Path, nexus_root: &str) -> Result<bool> {
        let content = fs::read_to_string(path);
        if content.is_err() { return Ok(false); }
        let mut content = content.unwrap();

        let mut changed = false;

        // UNIVERSAL MAPPING LOGIC
        let replacements = vec![
            (r"C:\GenesisOS_Core\rust\Sovereign_Suite_RS", format!(r"{}\rust\Sovereign_Suite_RS", nexus_root)),
            (r"C:\GenesisOS_Core\rust\GenesisRUST", format!(r"{}\rust\GenesisRUST", nexus_root)),
            (r"C:\GenesisOS_Core\desktop\07_INTERFACE_Frontend", format!(r"{}\desktop\07_INTERFACE_Frontend", nexus_root)),
            (r"C:\GenesisOS_Core\desktop\sarah_ui", format!(r"{}\desktop\sarah_ui", nexus_root)),
            (r"C:\GenesisOS_Core\rust\SarahCore_Crates", format!(r"{}\rust\SarahCore_Crates", nexus_root)),
            (r"C:\GenesisOS_Core\rust", format!(r"{}\rust", nexus_root)),
            (r"C:\GenesisOS_Core", format!(r"{}", nexus_root)),
        ];

        for (legacy, modern) in replacements {
            if content.contains(legacy) {
                content = content.replace(legacy, &modern);
                changed = true;
            }
        }

        if changed {
            fs::write(path, content)?;
            println!("\x1b[92m[FIXED]\x1b[0m {:?}", path.file_name().unwrap());
        }

        Ok(changed)
    }
}
