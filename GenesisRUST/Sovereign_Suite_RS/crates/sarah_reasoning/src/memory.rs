use crate::SOVEREIGN_ANCHOR;
use sovereign_math::{SovereignMath};
use sovereign_hdc::Hypervector;
use chrono::Utc;
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;

/// [MEMORY_0x10K]: HOLOGRAPHIC RESONANCE MEMORY (Zenith Upgrade: 10,240-bit)
/// Calculates memory recall based on Hamming Similarity within the holographic manifold.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub content: String,
    pub holographic_signature: Hypervector,
    pub importance: f32,
    pub timestamp: i64,
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

    /// [RECALL_0x10R]: Holographic Resonance Search (Hamming Distance)
    pub fn recall(&self, query: &str, limit: usize) -> Vec<&MemoryEntry> {
        let query_hv = self.math.holographic_expand(query);
        
        let mut scored: Vec<(f64, &MemoryEntry)> = self.memories.par_iter()
            .map(|entry| {
                let similarity = query_hv.similarity(&entry.holographic_signature);
                (similarity, entry)
            })
            .filter(|(score, _)| *score > 0.4) // Orthogonality threshold
            .collect();

        // Sort by highest holographic similarity
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(_, entry)| entry).collect()
    }

    /// [REMEMBER_0x10W]: Holographic Encoding
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
