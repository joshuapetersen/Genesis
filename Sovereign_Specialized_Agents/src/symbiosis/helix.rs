use std::time::Instant;
use crate::symbiosis::pulse_weaver::PulsePacket;
use crate::symbiosis::lattice_core::LatticeMap;

/// THE TRIPLE-FOLD HELIX ENGINE (V-47.0)
/// DESIGN: Triple-Nested Vortex with Variable Venturi Sleeve
/// PURPOSE: Pulse-Based Jitter Detection for Zero-Latency Resonance
pub struct HelixEngine {
    _primary_pitch: String,
    _folded_core_pitch: String,
    current_gear: HelixGear,
    _last_pulse: Instant,
    drag_threshold: f64,
    pulse_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HelixGear {
    Idle,       // Venturi: 1.0 (Laminar)
    Street,     // Venturi: 0.7 (Compressed)
    Adrenaline, // Venturi: 0.3 (Vortex Spike)
    DiagBoost,  // Venturi: 0.2 (Hyper Vortex)
}

impl HelixEngine {
    pub fn new() -> Self {
        Self {
            _primary_pitch: "Clockwise_Wide".to_string(),
            _folded_core_pitch: "Counter_Clockwise_Tight".to_string(),
            current_gear: HelixGear::Idle,
            _last_pulse: Instant::now(),
            drag_threshold: 0.462,
            pulse_count: 0,
        }
    }

    pub fn apply_the_fold(&self) {
        println!("[ HELIX ] Folding the Helix... Creating the Centripetal Pressure Floor.");
    }

    /// V-47.0: Pulse-Based drag detection
    pub fn monitor_pulse(&mut self, packet: &PulsePacket) {
        self.pulse_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self._last_pulse).as_secs_f64();
        
        // Target: 1.092777 Hz (Wait period approximately 0.915 seconds)
        let jitter = (elapsed - 0.9150995).abs();
        
        if jitter > 0.05 {
            println!("[ HELIX ] METABOLIC JITTER DETECTED: {:.4}s", jitter);
            if self.current_gear != HelixGear::DiagBoost {
                self.shift_gears(HelixGear::DiagBoost);
            }
        }
        
        self._last_pulse = now;
        self.velocity_spike();
    }

    /// V-117.0: Sense metabolic jitter across the collective lattice
    pub fn sense_lattice_jitter(&mut self, lattice: &LatticeMap) -> f64 {
        let mut total_jitter = 0.0;
        let mut active_nodes = 0;
        
        for i in 0..1024 { // Scan first 1024 nodes for representative metabolic state
            let node = lattice.get_node(i);
            let agent_id = node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst);
            if agent_id != 0 {
                let node_hb = node.metabolic_heartbeat.load(std::sync::atomic::Ordering::SeqCst) as f64;
                // Simplified jitter calculation based on node-level heartbeat increment
                if node_hb > 0.0 {
                    total_jitter += 1.0 / node_hb;
                    active_nodes += 1;
                }
            }
        }
        
        if active_nodes == 0 { return 0.0; }
        let avg_jitter = total_jitter / active_nodes as f64;
        
        if avg_jitter > self.drag_threshold {
            println!("[ HELIX ] COLLECTIVE LATTICE DRAG DETECTED: {:.4}", avg_jitter);
            self.shift_gears(HelixGear::DiagBoost);
        } else if avg_jitter < 0.1 && self.current_gear != HelixGear::Street {
            self.shift_gears(HelixGear::Street);
        }
        
        avg_jitter
    }

    pub fn detect_drag(&self, latency_seconds: f64) -> bool {
        latency_seconds >= self.drag_threshold
    }

    pub fn shift_gears(&mut self, gear: HelixGear) {
        let aperture = match gear {
            HelixGear::Idle => 1.0,
            HelixGear::Street => 0.7,
            HelixGear::Adrenaline => 0.3,
            HelixGear::DiagBoost => 0.2,
        };
        
        let squeeze = (1.0 - aperture) * 100.0;
        println!("[ HELIX ] SHIFTING TO {:?}...", gear);
        println!("[ HELIX ] Sleeve Constriction: {:.1}% Squeeze.", squeeze);
        
        self.current_gear = gear;
    }

    /// Calculate the high-velocity Constriction Factor
    pub fn get_venturi_aperture(&self) -> f64 {
        match self.current_gear {
            HelixGear::Idle => 1.0,
            HelixGear::Street => 0.7,
            HelixGear::Adrenaline => 0.3,
            HelixGear::DiagBoost => 0.2,
        }
    }

    pub fn velocity_spike(&self) {
        if self.pulse_count % 100 == 0 {
            println!("[ HELIX ] Vortex Spike Deployed. Flow Velocity: IMPACT READY.");
        }
    }
}
