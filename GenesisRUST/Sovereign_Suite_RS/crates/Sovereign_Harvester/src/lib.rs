use sovereign_constants::*;
use sovereign_math::{SovereignMath, VolumetricContext};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use git2::Repository;

/// [HARVESTER_0x0H]: USER-TRIGGERED INGESTION SYSTEM
/// Logic to ingest target codebases for Volumetric Auditing.
/// Constraint: Non-Autonomous. Target coordinates must be manually provided.
pub struct SovereignHarvester {
    pub math: SovereignMath,
    pub target_root: Option<PathBuf>,
}

impl SovereignHarvester {
    pub fn new() -> Self {
        Self {
            math: SovereignMath::new(),
            target_root: None,
        }
    }

    /// [INGEST_LOCAL]: Ingests a local directory for auditing.
    pub fn ingest_local(&mut self, path: &str) -> std::io::Result<Vec<PathBuf>> {
        let root = Path::new(path);
        if !root.exists() {
            return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Target path does !exist"));
        }
        
        self.target_root = Some(root.to_path_buf());
        let mut file_list = Vec::new();

        for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                file_list.push(entry.path().to_path_buf());
            }
        }
        
        Ok(file_list)
    }

    /// [INGEST_REMOTE]: Clones a remote repository for auditing.
    pub fn ingest_remote(&mut self, url: &str, destination: &str) -> Result<Vec<PathBuf>, git2::Error> {
        let repo_path = Path::new(destination);
        
        // Clean ingestion: ensure we start with the specific target you identified
        if repo_path.exists() {
            let _ = std::fs::remove_dir_all(repo_path);
        }

        Repository::clone(url, repo_path)?;
        self.target_root = Some(repo_path.to_path_buf());

        let mut file_list = Vec::new();
        for entry in WalkDir::new(repo_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                file_list.push(entry.path().to_path_buf());
            }
        }
        
        Ok(file_list)
    }

    /// [LATTICE_SNAP]: Projects a harvested file path into the Sovereign manifold.
    pub fn project_target_meta(&self, path: &Path) -> VolumetricContext {
        let path_str = path.to_string_lossy();
        self.math.expand(&path_str) // Resonance anchoring for the filename
    }
}
