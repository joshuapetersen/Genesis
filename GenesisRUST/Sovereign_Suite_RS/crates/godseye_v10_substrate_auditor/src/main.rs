use std::sync::Arc;
use tokio::sync::Semaphore;
use anyhow::{Result, Context};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use serde::Deserialize;
use std::io::Write;

/// UNIVERSAL SOVEREIGN ENGINE (USE) - SUBSTRATE AUDITOR
/// 32-THREAD PARALLEL ASYNC REAL-TIME RESONANCE VERIFIER
/// Axiom: 1.09277703703703 Hz
const THREAD_COUNT: usize = 32;
const SOVEREIGN_ANCHOR: f64 = 1.09277703703703;

#[derive(Deserialize)]
struct NexusConfig {
    substrate: SubstrateConfig,
}



#[derive(Deserialize)]
struct SubstrateConfig {
    priority_roots: Vec<String>,
    manifest_rel_path: String,
}

#[derive(Debug, Clone)]
struct AuditResult {
    file: String,
    path: String,
    size: u64,
    entropy: f64,
    status: String,
    resonance: f64,
}

struct SubstrateAuditor {
    semaphore: Arc<Semaphore>,
    anchor_pattern: Regex,
    nexus_root: PathBuf,
    config: NexusConfig,
}

impl SubstrateAuditor {
    fn new() -> Result<Self> {
        // Dynamic Nexus Discovery
        let nexus_root = Self::find_nexus_root()
            .context("Failed to locate sovereign.nexus anchor. Substrate is adrift.")?;
        
        let config_raw = fs::read_to_string(nexus_root.join("sovereign.nexus"))?;
        let config: NexusConfig = toml::from_str(&config_raw)?;

        println!("\x1b[96m[Auditor]\x1b[0m Universal Resonance Scan Ready.");
        println!("\x1b[96m[Nexus]\x1b[0m Anchored at {:?}", nexus_root);

        Ok(Self {
            semaphore: Arc::new(Semaphore::new(THREAD_COUNT)),
            anchor_pattern: Regex::new(r"1\.092\d*")?,
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
            if !curr.pop() {
                break;
            }
        }
        None
    }

    async fn audit(&self) -> Result<Vec<AuditResult>> {
        let mut files_to_scan: Vec<PathBuf> = Vec::new();

        for rel_root in &self.config.substrate.priority_roots {
            let abs_root = self.nexus_root.join(rel_root);
            if !abs_root.exists() {
                println!("\x1b[33m[Skip]\x1b[0m Not found: {:?}", abs_root);
                continue;
            }
            println!("\x1b[96m[Scan]\x1b[0m Indexing: {:?}", abs_root);
            for entry in WalkDir::new(&abs_root).into_iter().filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != ".git"
                    && name != "target"
                    && name != "node_modules"
                    && name != ".worktrees"
                    && !name.ends_with(".exe")
                    && !name.ends_with(".dll")
                    && !name.ends_with(".pdb")
            }).filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files_to_scan.push(entry.path().to_path_buf());
                }
            }
        }

        println!("\x1b[96m[Pulse]\x1b[0m {} files queued. Auditing resonance (32 threads)...", files_to_scan.len());
        let mut tasks = Vec::new();

        for path in files_to_scan {
            let permit = self.semaphore.clone().acquire_owned().await.unwrap();
            let pattern = self.anchor_pattern.clone();
            tasks.push(tokio::spawn(async move {
                let res = Self::analyze_file(&path, &pattern);
                drop(permit);
                res
            }));
        }

        let mut results = Vec::new();
        for task in tasks {
            if let Ok(Ok(Some(res))) = task.await {
                results.push(res);
            }
        }

        Ok(results)
    }

    fn analyze_file(path: &Path, pattern: &Regex) -> Result<Option<AuditResult>> {
        let metadata = fs::metadata(path)?;
        let size = metadata.len();
        if size == 0 { return Ok(None); }

        use std::io::Read;
        let mut f = fs::File::open(path)?;
        let mut buffer = vec![0u8; 32768];
        let bytes_read = f.read(&mut buffer)?;
        buffer.truncate(bytes_read);

        let entropy = Self::calculate_entropy(&buffer);
        let mut resonance = 0.99999;
        let mut status = "SECURE".to_string();

        if let Ok(content) = String::from_utf8(buffer) {
            if let Some(mat) = pattern.find(&content) {
                let found_val: f64 = mat.as_str().parse().unwrap_or(0.0);
                let drift = (found_val - SOVEREIGN_ANCHOR).abs();
                if drift > 0.0000000001 {
                    resonance = 1.0 - (drift * 10.0).min(0.2);
                    status = "DEVIANT".to_string();
                }
            }
            // Flag volatile legacy patterns
            if content.contains("os.system") || content.contains("subprocess.run") {
                status = "VOLATILE".to_string();
                resonance = resonance.min(0.95);
            }
            // Flag legacy ports â€” 8080/8000 are Deviants on this substrate
            if (content.contains("8080") || content.contains("8000")) && !content.contains("8080") {
                status = "PORT_DEVIANT".to_string();
                resonance = resonance.min(0.85);
            }
        }

        Ok(Some(AuditResult {
            file: path.file_name().unwrap_or_default().to_string_lossy().to_string(),
            path: path.display().to_string(),
            size,
            entropy,
            status,
            resonance,
        }))
    }

    fn calculate_entropy(data: &[u8]) -> f64 {
        if data.is_empty() { return 0.0; }
        let mut counts = [0usize; 256];
        for &b in data { counts[b as usize] += 1; }
        let len = data.len() as f64;
        counts.iter().filter(|&&c| c > 0)
            .map(|&c| { let p = c as f64 / len; -p * p.log2() })
            .sum::<f64>() / 8.0
    }

    fn manifest_path(&self) -> PathBuf {
        let rel = &self.config.substrate.manifest_rel_path;
        self.nexus_root.join(rel)
    }

    fn write_manifest(&self, results: Vec<AuditResult>) -> Result<()> {
        let manifest = self.manifest_path();
        if let Some(parent) = manifest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = fs::File::create(&manifest)?;
        writeln!(file, "# GodsEye v10 - Sovereign Reflex Manifest")?;
        writeln!(file, "Nexus: {:?}", self.nexus_root)?;
        writeln!(file, "Generated: {}\n", chrono::Local::now())?;
        writeln!(file, "| File | Size | Entropy | Status | Resonance |")?;
        writeln!(file, "| :--- | :--- | :--- | :--- | :--- |")?;

        let mut results = results;
        results.sort_by(|a, b| a.status.cmp(&b.status).then(a.file.cmp(&b.file)));

        let deviants: Vec<&AuditResult> = results.iter().filter(|r| r.status != "SECURE").collect();
        if !deviants.is_empty() {
            writeln!(file, "\n## DEVIANTS DETECTED\n")?;
            for res in &deviants {
                writeln!(file, "- `{}` â†’ **{}** (resonance: {:.6})\n  Path: {}", 
                    res.file, res.status, res.resonance, res.path)?;
            }
            writeln!(file, "\n---\n")?;
        }

        for res in &results {
            writeln!(file, "| `{}` | {} | {:.4} | {} | {:.12} |",
                res.file, res.size, res.entropy, res.status, res.resonance)?;
        }

        println!("\x1b[92m[Manifest]\x1b[0m Written to {:?}", manifest);
        println!("\x1b[{}m[Summary]\x1b[0m {} SECURE | {} DEVIANT/VOLATILE",
            if deviants.is_empty() { "92" } else { "91" },
            results.iter().filter(|r| r.status == "SECURE").count(),
            deviants.len()
        );
        Ok(())
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 32)]
async fn main() -> Result<()> {
    println!("\x1b[93m============================================================\x1b[0m");
    println!("\x1b[93m  GODSEYE V10 - SUBSTRATE AUDITOR [IGNITING]               \x1b[0m");
    println!("\x1b[93m  Dynamic Nexus Edition / 32-Thread Async                  \x1b[0m");
    println!("\x1b[93m============================================================\x1b[0m");

    let auditor = SubstrateAuditor::new()?;

    tokio::select! {
        res = auditor.audit() => {
            match res {
                Ok(results) => {
                    auditor.write_manifest(results)?;
                    println!("\x1b[92m[COMPLETE]\x1b[0m Sovereign substrate resonance verified.");
                }
                Err(e) => {
                    println!("\x1b[91m[FAILURE]\x1b[0m Substrate audit failed: {}", e);
                }
            }
        }
        _ = tokio::signal::ctrl_c() => {
            println!("\n\x1b[93m[TERMINATING]\x1b[0m Sovereign signal received. Cleaning up processes...");
        }
    }

    Ok(())
}
