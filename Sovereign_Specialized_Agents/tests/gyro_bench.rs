use sovereign_specialized_agents::neural_cores::vortex_ffi::VortexEngine;
use std::time::Instant;

#[test]
fn test_vortex_pulse_latency() {
    println!("[ BENCHMARK ] Vortex Hardware Pulse Strike");
    let mut engine = VortexEngine::new();
    let mut hidden = vec![0.0f32; 2560];
    
    let start = Instant::now();
    let layers = 34;
    
    for layer in 0..layers {
        engine.process_layer(&mut hidden, layer, 0);
    }
    
    let duration = start.elapsed();
    println!("[ BENCHMARK ] 34-Layer Forward Pass: {:?}", duration);
    
    // 1.092777 Hz = ~915ms target. Hardware should be much faster.
    assert!(duration.as_millis() < 915, "Hardware pulse exceeds 1.09Hz limit!");
}
