use std::sync::Arc;
use tokio::sync::Semaphore;
use anyhow::{Result, Context};
use std::fs;
use std::path::{Path, PathBuf};
use fs_extra::dir::CopyOptions;
use regex::Regex;

/// SOVEREIGN SUBSTRATE MIGRATOR (SSM) v10.0 [DEFINITIVE RUST EDITION]
/// 32-THREAD PARALLEL ASYNC HIGH-SPEED MIGRATION
/// Target: C:\GENESIS\GenesisRUST\Sovereign_Suite_RS

const TARGET_ROOT: &str = r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS";
const THREAD_COUNT: usize = 32;

struct MigrationTask {
    source: PathBuf,
    destination: PathBuf,
}

struct SubstrateMigrator {
    semaphore: Arc<Semaphore>,
}

impl SubstrateMigrator {
    fn new() -> Self {
        println!("\x1b[95m[Migrator]\x1b[0m 32-Thread Async Migration Engine Active.");
        Self {
            semaphore: Arc::new(Semaphore::new(THREAD_COUNT)),
        }
    }

    /// Primary Unified Migration
    async fn migrate(&self) -> Result<()> {
        println!("\x1b[95m[Phase 1]\x1b[0m Establishing Unified Root: {}", TARGET_ROOT);
        fs::create_dir_all(format!(r"{}\rust", TARGET_ROOT))?;
        fs::create_dir_all(format!(r"{}\desktop", TARGET_ROOT))?;
        fs::create_dir_all(format!(r"{}\godseye", TARGET_ROOT))?;

        let tasks = vec![
            // RUST CORE
            MigrationTask {
                source: PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\rust\Sovereign_Suite_RS"),
                destination: PathBuf::from(format!(r"{}\rust\Sovereign_Suite_RS", TARGET_ROOT)),
            },
            MigrationTask {
                source: PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\rust\GenesisRUST"),
                destination: PathBuf::from(format!(r"{}\rust\GenesisRUST", TARGET_ROOT)),
            },
            MigrationTask {
                source: PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\rust\SarahCore_Crates"),
                destination: PathBuf::from(format!(r"{}\rust\SarahCore_Crates", TARGET_ROOT)),
            },
            // DESKTOP UI
            MigrationTask {
                source: PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\desktop\sarah_ui"),
                destination: PathBuf::from(format!(r"{}\desktop\sarah_ui", TARGET_ROOT)),
            },
            MigrationTask {
                source: PathBuf::from(r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\desktop\07_INTERFACE_Frontend\frontend"),
                destination: PathBuf::from(format!(r"{}\desktop\07_INTERFACE_Frontend", TARGET_ROOT)),
            },
            // GODSEYE MANIFEST
            MigrationTask {
                source: PathBuf::from(r"C:\SarahCore\GodsEye\godseye_v10_reflex_manifest.md"),
                destination: PathBuf::from(format!(r"{}\godseye\godseye_v10_reflex_manifest.md", TARGET_ROOT)),
            },
        ];

        println!("\x1b[95m[Phase 2]\x1b[0m Performing 32-Thread Substrate Migration...");
        let mut job_handles = Vec::new();

        for task in tasks {
            if !task.source.exists() {
                println!("\x1b[91m[SKIP]\x1b[0m Source not found: {:?}", task.source);
                continue;
            }

            let permit = self.semaphore.clone().acquire_owned().await.unwrap();
            job_handles.push(tokio::spawn(async move {
                let res = Self::execute_migration(task);
                drop(permit);
                res
            }));
        }

        for handle in job_handles {
            handle.await??;
        }

        println!("\x1b[95m[Phase 3]\x1b[0m Re-Anchoring Paths in Reflex Toolchain...");
        self.re_anchor_tools()?;

        Ok(())
    }

    fn execute_migration(task: MigrationTask) -> Result<()> {
        println!("\x1b[96m[MOVE]\x1b[0m {:?} -> {:?}", task.source.file_name().unwrap(), task.destination);
        
        if task.source.is_dir() {
            fs::create_dir_all(&task.destination)?;
            Self::rapid_copy(&task.source, &task.destination)?;
        } else {
            fs::copy(&task.source, &task.destination)?;
        }
        
        Ok(())
    }

    /// High-Speed Source Copy (Excluding Build Bloat)
    fn rapid_copy(source: &Path, destination: &Path) -> Result<()> {
        use walkdir::WalkDir;
        
        for entry in WalkDir::new(source).into_iter().filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            name != "target" && name != "node_modules" && name != ".git" && name != ".venv"
        }).filter_map(|e| e.ok()) {
            let path = entry.path();
            let rel_path = path.strip_prefix(source)?;
            let dest_path = destination.join(rel_path);

            if path.is_dir() {
                fs::create_dir_all(&dest_path)?;
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(path, &dest_path)?;
            }
        }
        Ok(())
    }

    fn re_anchor_tools(&self) -> Result<()> {
        let new_manifest = format!(r"{}\godseye\godseye_v10_reflex_manifest.md", TARGET_ROOT);
        let tools_dir = format!(r"{}\rust\Sovereign_Suite_RS\crates", TARGET_ROOT);
        
        let tool_names = vec![
            "godseye_v10_reflex_reactor",
            "godseye_v10_reflex_fixer",
            "godseye_v10_substrate_auditor",
        ];

        let manifest_regex = Regex::new(r#"const MANIFEST_PATH: &str = r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\godseye\\godseye_v10_reflex_manifest\.md";"#)?;
        let substrate_regex = Regex::new(r#"const SUBSTRATES: &\[&str\] = &\[r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS\rust", r"C:\GENESIS\GenesisRUST\Sovereign_Suite_RS"\];"#)?;

        for tool in tool_names {
            let path = PathBuf::from(format!(r"{}\{}\src\main.rs", tools_dir, tool));
            if path.exists() {
                let mut content = fs::read_to_string(&path)?;
                
                // Update Manifest Path
                let new_manifest_line = format!(r#"const MANIFEST_PATH: &str = r"{}";"#, new_manifest);
                content = manifest_regex.replace(&content, new_manifest_line.as_str()).to_string();
                
                // Update Substrate Roots (for Auditor)
                if tool == "godseye_v10_substrate_auditor" {
                    let new_substrates = format!(r#"const SUBSTRATES: &[&str] = &[r"{}"];"#, TARGET_ROOT);
                    content = substrate_regex.replace(&content, new_substrates.as_str()).to_string();
                }

                fs::write(&path, content)?;
                println!("\x1b[92m[RE-ANCHORED]\x1b[0m Paths updated in tool: {}", tool);
            }
        }
        Ok(())
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    println!("\x1b[93m============================================================\x1b[0m");
    println!("\x1b[93m  SOVEREIGN SUBSTRATE MIGRATOR (SSM) v10 [IGNITING]  \x1b[0m");
    println!("\x1b[93m  [100% Rust-Primary Transition / 32-Thread Async]  \x1b[0m");
    println!("\x1b[93m============================================================\x1b[0m");

    let migrator = SubstrateMigrator::new();
    migrator.migrate().await?;

    println!("\x1b[92m[SUCCESS]\x1b[0m Substrate 100% Re-Anchored to {}.", TARGET_ROOT);
    Ok(())
}
