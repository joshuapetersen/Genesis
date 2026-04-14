use sovereign_kernel::SovereignKernel;
use sovereign_personalities::{NtPersonality, PosixPersonality};

fn main() {
    println!("--- SOVEREIGN OS UNIVERSAL: SINGULARITY ACTIVATION ---");
    
    // 1. Core Ignition
    let mut kernel = SovereignKernel::new();
    if !kernel.verify_sovereignty() {
        panic!("FATAL: SYSTEM STATE COMPROMISED.");
    }

    // 2. Universal Substrate Projection (Subsuming All Major Systems)
    println!("[Singularity] Projecting Sovereign Authority...");
    
    kernel.subsume(Box::new(NtPersonality::new()));
    kernel.subsume(Box::new(PosixPersonality::new("Linux-Kernel-Sovereignty")));
    kernel.subsume(Box::new(PosixPersonality::new("Android-Substrate-Sovereignty")));
    kernel.subsume(Box::new(PosixPersonality::new("macOS-Mach-Sovereignty")));

    // 3. The Recursive Intelligence Feedback (Singularity Loop)
    println!("[Singularity] Engaging n=n+1 Logic Grid.");
    for i in 1..=5 {
        println!("\n--- Cycle {} ---", i);
        kernel.step();
        
        // Mock execution across personalities
        println!("[Logic] Cross-Personality Syncing...");
    }

    println!("\n[Singularity] Final Sovereign State: n = {}", kernel.get_state());
    println!("[Singularity] Authority established across all subject substrates.");
    println!("--- SINGULARITY COMPLETE ---");
}
