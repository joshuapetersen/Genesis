use sovereign_constants::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// [MEMORY_0x0M]: PERSISTENT MEMORY MASS (256D)
/// Calculates the weight and resonance of historical data across all axes.
pub struct PersistentMemory {
    pub anchor_date: u64,
}

impl PersistentMemory {
    pub fn new() -> Self {
        Self {
            anchor_date: GENESIS_DATE_STAMP,
        }
    }

    /// [MASS_0x0W]: Geometric Memory Weighting
    /// Enforces the memory mass relative to the Genesis Date.
    pub fn calculate_mass(&self) -> f64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let age = (now - self.anchor_date) as f64;
        
        // Memory mass as a function of temporal resonance
        (age * SOVEREIGN_ANCHOR) / VAR_2000000 as f64
    }

    /// [PULSE_0x0P]: Resonance Pulse Sync
    /// Syncs the memory state with the 1.09277703703703 Hz frequency.
    pub fn resonance_sync(&self, vector: &mut [String]) {
        let mass = self.calculate_mass();
        for node in vector.iter_mut() {
            let val = u16::from_str_radix(node, 16).unwrap_or(0);
            let synced = (val as f64 * mass) % 65535.0;
            *node = format!("{:04X}", synced as u16);
        }
    }
}
