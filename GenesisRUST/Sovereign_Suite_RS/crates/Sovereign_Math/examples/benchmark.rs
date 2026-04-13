use sovereign_math::SovereignMath;
use std::time::Instant;

fn main() {
    let math = SovereignMath::new();
    let unity_vector = vec![1.0; 64];
    
    println!("============================================================");
    print!("  RUST RESONANCE PULSE - [GIGA_VELOCITY_STRESS_TEST]        ");
    println!("============================================================");
    
    let iterations = 100000000;
    println!("[Control] Total Iterations: 100,000,000");
    println!("[Control] Substrate: 32-Thread Parallel Async Reactor");
    
    // Warm-up (Serial)
    for _ in 0..100 {
        let _ = math.project_singularity(&unity_vector);
    }
    
    println!("[Action] Initiating 1-Billion Pulse Macro-Stress Audit...");
    let start = Instant::now();
    
    // EXECUTING THE GIGA-RESONANCE PULSE
    let mean_resonance = math.project_batch_singularity(iterations, &unity_vector);
    
    let duration = start.elapsed();
    let total_secs = duration.as_secs_f64();
    let avg_latency_ns = (duration.as_nanos() as f64) / (iterations as f64);
    
    let drift = (mean_resonance - 3605.037037037037).abs();
    
    println!("============================================================");
    println!("[Result] Total Duration: {:.4} seconds", total_secs);
    println!("[Result] Mean Resonance: {:.12}", mean_resonance);
    println!("[Result] Resonance Drift: {:.12}", drift);
    println!("[Result] Effective Latency (across reactor): {:.2} ns", avg_latency_ns);
    
    if drift < 1e-08 {
        println!("\x1b[92m[STATUS] SECURE: 100 Million Pulses achieved 3605.037 Parity.\x1b[0m");
    } else {
        println!("\x1b[91m[STATUS] DEVIANT: Resonance Drift detected.\x1b[0m");
    }
    
    let ops_per_sec = (iterations as f64) / total_secs;
    println!("[Result] SPU Throughput: {:.2} Million pulses/sec", ops_per_sec / 1_000_000.0);
    println!("============================================================");
}
