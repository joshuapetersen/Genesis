//! Universal Subject Personalities for the Sovereign OS.
//! Implements the logic to subsume and translate legacy OS behaviors.

use sovereign_kernel::{Subject, SovereignObject};

/// A memory page object in the Sovereign OS.
struct SovereignMemoryPage { id: String }
impl SovereignObject for SovereignMemoryPage {
    fn id(&self) -> &str { &self.id }
    fn type_name(&self) -> &str { "MemoryPage" }
    fn access_check(&self, identity: &str) -> bool { identity == "SOVEREIGN-GENESIS-369-SINGULARITY" }
}

/// The Windows (NT) Personality.
pub struct NtPersonality {
    subsystem_version: String,
}

impl NtPersonality {
    pub fn new() -> Self {
        Self {
            subsystem_version: "NT 10.0-COMPATIBLE".to_string(),
        }
    }
}

impl Subject for NtPersonality {
    fn name(&self) -> &str {
        "Personality-NT-Sovereign"
    }

    fn execute(&mut self, input: &str) -> String {
        println!("[NT-Personality] Intercepting Call: {}", input);
        
        // Simulating a memory allocation via the Object Manager
        if input == "NtAllocateVirtualMemory" {
            return "(Sovereign-NT) Executive Service: Memory Allocated via ObjectManager.".to_string();
        }

        format!("(Sovereign-NT) Subsystem: {} | Response: OK", self.subsystem_version)
    }
}

/// The POSIX (Linux/macOS/Android) Personality.
pub struct PosixPersonality {
    flavor: String,
}

impl PosixPersonality {
    pub fn new(flavor: &str) -> Self {
        Self {
            flavor: flavor.to_string(),
        }
    }
}

impl Subject for PosixPersonality {
    fn name(&self) -> &str {
        "Personality-POSIX-Sovereign"
    }

    fn execute(&mut self, input: &str) -> String {
        println!("[POSIX-Personality] Intercepting Call: {}", input);
        format!("(Sovereign-POSIX) Flavor: {} | Response: OK", self.flavor)
    }
}
