use sovereign_math::SovereignMath;
use sovereign_hdc::Hypervector;
use chrono::Utc;
use rayon::prelude::*;
use rayon::iter::IntoParallelRefIterator;
use std::path::Path;

const PHI_INV: f64 = 0.6180339887498949;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryEntry {
    pub content: String,
    pub holographic_signature: Hypervector,
    pub importance: f32,
    pub timestamp: i64,
}

impl MemoryEntry {
    pub fn effective_importance(&self, now_ts: i64) -> f32 {
        let age_secs = (now_ts - self.timestamp).max(0);
        let age_days = age_secs as f64 / 86_400.0;
        let decay = PHI_INV.powf(age_days);
        (self.importance as f64 * decay) as f32
    }
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
        Self { math: SovereignMath::new(), memories: Vec::new() }
    }
    pub fn load(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let memories: Vec<MemoryEntry> = serde_json::from_str(&data).unwrap_or_default();
        Ok(Self { math: SovereignMath::new(), memories })
    }
    pub fn save(&self, path: impl AsRef<Path>) -> std::io::Result<()> {
        let data = serde_json::to_string_pretty(&self.memories).unwrap_or_default();
        std::fs::write(path, data)
    }
    pub fn prune_faded(&mut self) {
        let now = Utc::now().timestamp();
        let before = self.memories.len();
        self.memories.retain(|e| !e.is_faded(now));
        let pruned = before - self.memories.len();
        if pruned > 0 {
            println!("\x1b[93m[Memory]\x1b[0m Pruned {} faded memories (phi-decay).", pruned);
        }
    }

    // EVOLUTION_8: Topic Cluster Engine
    // k-means over holographic signatures. k = sqrt(n).min(16).
    // Seeded at Fibonacci indices for phi-geometric spread.
    // recall_clustered() is O(k + cluster_size) vs O(n) for full scan.
    pub fn build_clusters(&self) -> Vec<(Hypervector, Vec<usize>)> {
        let n = self.memories.len();
        if n < 4 { return vec![]; }
        let k = ((n as f64).sqrt().ceil() as usize).min(16).max(2);

        let mut fib_idx = Vec::new();
        let (mut a, mut b) = (0usize, 1usize);
        while a < n && fib_idx.len() < k { fib_idx.push(a); let t = a + b; a = b; b = t; }
        while fib_idx.len() < k { fib_idx.push((fib_idx.len() * (n / k)).min(n - 1)); }
        fib_idx.truncate(k);

        let mut centroids: Vec<Hypervector> = fib_idx.iter()
            .map(|&i| self.memories[i].holographic_signature.clone())
            .collect();
        let mut assignments = vec![0usize; n];

        for _it in 0..3 {
            for (i, mem) in self.memories.iter().enumerate() {
                let best = centroids.iter().enumerate()
                    .max_by(|(_, ca), (_, cb)| {
                        mem.holographic_signature.similarity_fast(ca)
                            .partial_cmp(&mem.holographic_signature.similarity_fast(cb))
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(c, _)| c).unwrap_or(0);
                assignments[i] = best;
            }
            for c in 0..k {
                let members: Vec<&Hypervector> = self.memories.iter().enumerate()
                    .filter(|(i, _)| assignments[*i] == c)
                    .map(|(_, m)| &m.holographic_signature)
                    .collect();
                if members.is_empty() { continue; }
                let mc = members.len();
                let mut new_words = vec![0u64; 1600];
                for wi in 0..1600 {
                    let mut word = 0u64;
                    for bit in 0..64usize {
                        let ones = members.iter().filter(|hv| (hv.data[wi] >> bit) & 1 == 1).count();
                        if ones * 2 >= mc { word |= 1u64 << bit; }
                    }
                    new_words[wi] = word;
                }
                centroids[c] = Hypervector::new(new_words);
            }
        }
        let mut clusters: Vec<(Hypervector, Vec<usize>)> = (0..k)
            .map(|c| (centroids[c].clone(), (0..n).filter(|&i| assignments[i] == c).collect()))
            .collect();
        clusters.retain(|(_, m)| !m.is_empty());
        clusters
    }

    pub fn recall_clustered(
        &self, query: &str, limit: usize,
        clusters: &[(Hypervector, Vec<usize>)],
    ) -> Vec<&MemoryEntry> {
        if clusters.is_empty() { return self.recall(query, limit); }
        let query_hv = self.math.holographic_expand(query);
        let now = Utc::now().timestamp();
        let best_members = clusters.iter()
            .max_by(|(ca, _), (cb, _)| {
                query_hv.similarity_fast(ca)
                    .partial_cmp(&query_hv.similarity_fast(cb))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(_, m)| m.as_slice())
            .unwrap_or(&[]);
        let mut scored: Vec<(f64, &MemoryEntry)> = best_members.iter()
            .filter_map(|&i| self.memories.get(i))
            .map(|e| (query_hv.similarity_fast(&e.holographic_signature) * e.effective_importance(now) as f64, e))
            .filter(|(s, _)| *s > 0.05)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(_, e)| e).collect()
    }

    pub fn recall(&self, query: &str, limit: usize) -> Vec<&MemoryEntry> {
        let query_hv = self.math.holographic_expand(query);
        let now = Utc::now().timestamp();
        let mut scored: Vec<(f64, &MemoryEntry)> = self.memories.par_iter()
            .map(|e| (query_hv.similarity_fast(&e.holographic_signature) * e.effective_importance(now) as f64, e))
            .filter(|(s, _)| *s > 0.05)
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        scored.into_iter().take(limit).map(|(_, e)| e).collect()
    }

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