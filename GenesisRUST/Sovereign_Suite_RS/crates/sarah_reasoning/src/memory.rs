use sovereign_math::SovereignMath;
use sovereign_hdc::Hypervector;
use chrono::Utc;
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use std::path::Path;

// φ_inv = 0.6180339887... — the natural decay rate.
// Memories fade at φ_inv^age_days: after 1 day they retain 61.8% weight,
// after 7 days 2.2%, after 14 days ~0.04%. Matches the helix fade geometry.
const PHI_INV: f64 = 0.6180339887498949;

/// [MEMORY_0x10K]: HOLOGRAPHIC RESONANCE MEMORY (Zenith Upgrade: 10,240-bit)
/// Calculates memory recall based on Hamming Similarity within the holographic manifold.
/// φ-decay: confidence degrades at 1/φ per day — old memories fade naturally.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub content: String,
    pub holographic_signature: Hypervector,
    pub importance: f32,
    pub timestamp: i64,
}

impl MemoryEntry {
    /// Effective importance after φ-decay.
    /// effective = importance × φ_inv^age_days
    /// At age=0: full weight. At age=7: ~2.2% weight.
    pub fn effective_importance(&self, now_ts: i64) -> f32 {
        let age_secs = (now_ts - self.timestamp).max(0);
        let age_days = age_secs as f64 / 86_400.0;
        let decay = PHI_INV.powf(age_days);
        (self.importance as f64 * decay) as f32
    }

    /// True if this memory has effectively faded (weight < 1% of original).
    pub fn is_faded(&self, now_ts: i64) -> bool {
        self.effective_importance(now_ts) < self.importance * 0.01
    }
}

pub struct PersistentMemory {
    pub math: SovereignMath,
    pub memories: Vec<MemoryEntry>,
}

impl PersistentMemory {
    pub fn new() -> Self {
        Self {
            math: SovereignMath::new(),
            memories: Vec::new(),
        }
    }

    /// Boot from disk if a previous session was saved.
    /// Returns Self::new() if no save file found.
    pub fn load(path: impl AsRef<Path>) -> Self {
        let mut inst = Self::new();
        if let Ok(raw) = std::fs::read_to_string(path) {
            if let Ok(entries) = serde_json::from_str::<Vec<MemoryEntry>>(&raw) {
                let now = Utc::now().timestamp();
                // Prune faded memories on load — no point carrying dead weight.
                inst.memories = entries.into_iter()
                    .filter(|e| !e.is_faded(now))
                    .collect();
                println!("\x1b[96m[Memory]\x1b[0m Loaded {} active memories from disk.", inst.memories.len());
            }
        }
        inst
    }

    /// Persist current memory to disk. Call periodically or on shutdown.
    pub fn save(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let now = Utc::now().timestamp();
        // Only save non-faded entries.
        let live: Vec<&MemoryEntry> = self.memories.iter()
            .filter(|e| !e.is_faded(now))
            .collect();
        let raw = serde_json::to_string_pretty(&live)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        std::fs::write(path, raw)
    }

    /// Prune faded memories in-place. Call during idle cycles.
    pub fn prune_faded(&mut self) {
        let now = Utc::now().timestamp();
        let before = self.memories.len();
        self.memories.retain(|e| !e.is_faded(now));
        let pruned = before - self.memories.len();
        if pruned > 0 {
            println!("\x1b[93m[Memory]\x1b[0m Pruned {} faded memories (φ-decay).", pruned);
        }
    }

    /// [RECALL_0x10R]: Holographic Resonance Search (Hamming Distance + φ-decay weighting).
    /// Scores each memory by `holographic_similarity × effective_importance`.
    /// Faded memories naturally rank lower even if semantically similar.
    pub fn recall(&self, query: &str, limit: usize) -> Vec<&MemoryEntry> {
        let query_hv = self.math.holographic_expand(query);
        let now      = Utc::now().timestamp();

        let mut scored: Vec<(f64, &MemoryEntry)> = self.memories.par_iter()
            .map(|entry| {
                let sim    = query_hv.similarity(&entry.holographic_signature);
                let weight = entry.effective_importance(now) as f64;
                // Combined score: semantic similarity × φ-decayed importance
                (sim * weight, entry)
            })
            .filter(|(score, _)| *score > 0.05) // Lower floor — decay already handles relevance
            .collect();

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(_, entry)| entry).collect()
    }

    /// [REMEMBER_0x10W]: Holographic Encoding with φ-decay timestamp.
    /// Projects raw data into the 10,240-bit HDC manifold.
    pub fn remember(&mut self, content: &str, importance: f32) {
        let signature = self.math.holographic_expand(content);
        self.memories.push(MemoryEntry {
            content: content.to_string(),
            holographic_signature: signature,
            importance,
            timestamp: Utc::now().timestamp(),
        });
    }
}
