use std::time::Instant;
use crate::symbiosis::pulse_weaver::PulsePacket;
use crate::symbiosis::lattice_core::LatticeMap;

const PHI: f64 = 1.618033988749895;
const HEARTBEAT_HZ: f64 = 1.092777037037;
const TARGET_INTERVAL: f64 = 1.0 / HEARTBEAT_HZ;

/// THE TRIPLE-FOLD HELIX ENGINE (V-48.0 // EVOLUTION)
/// DESIGN: Triple-Nested Vortex with Variable Venturi Sleeve.
/// PURPOSE: Pulse-Based Jitter Detection and Flow Regulation.
pub struct HelixEngine {
    pub primary_pitch: f64,       // Theta-1 (Outer Helix)
    pub core_pitch:    f64,       // Theta-2 (Inner Vortex)
    pub hyper_pitch:   f64,       // Theta-3 (Singularity Core)
    pub flow_velocity: f64,       // V_flow (m/s equivalent resonance)
    pub current_gear:  HelixGear, 
    pub drag_threshold: f64,
    
    _last_pulse_instant: Instant,
    _jitter_history: [f64; 8],    // Rolling window for variance audit
    _pulse_count: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HelixGear {
    Idle,       // Aperture: 1.0  (Laminar Flow)
    Street,     // Aperture: 0.618(Phi-Compressed)
    Adrenaline, // Aperture: 0.382(Phi-Squared)
    DiagBoost,  // Aperture: 0.236(Phi-Cubed/Hyper Vortex)
}

impl HelixEngine {
    pub fn new() -> Self {
        Self {
            primary_pitch: 360.2 / PHI, 
            core_pitch:    360.2 / (PHI * PHI),
            hyper_pitch:   360.2 / (PHI * PHI * PHI),
            flow_velocity: 1.0,
            current_gear:  HelixGear::Idle,
            drag_threshold: 0.462,
            
            _last_pulse_instant: Instant::now(),
            _jitter_history: [0.0; 8],
            _pulse_count: 0,
        }
    }

    /// [TRIPLE_FOLD_0xTF]: Activate the centripetal pressure floor.
    pub fn apply_the_fold(&self) {
        println!("[ HELIX ] Folding the Helix... Triple-Fold Resonance at {:.2}°", self.hyper_pitch);
    }

    /// [VORTEX_FLOW_0xVF]: Calculate the Venturi Constriction.
    /// A1V1 = A2V2. Velocity increases as the sleeve constricts (Phi-Graded).
    pub fn calculate_constriction(&self) -> f64 {
        let aperture = self.get_venturi_aperture();
        // [PHI_GRADING]: Inverse quadratic scaling for optimized Venturi throughput
        1.0 / (aperture * aperture * PHI)
    }

    /// Monitor incoming pulse packets and update the vortex state.
    pub fn monitor_pulse(&mut self, _packet: &PulsePacket) {
        self._pulse_count += 1;
        let now = Instant::now();
        let elapsed = now.duration_since(self._last_pulse_instant).as_secs_f64();
        
        // Calculate raw jitter against the 1.092777 Hz target
        // [PARITY_LOCK]: 110% Overdrive requires sub-millisecond precision
        let jitter = (elapsed - TARGET_INTERVAL).abs();
        
        // Push to rolling history
        let idx = (self._pulse_count % 8) as usize;
        self._jitter_history[idx] = jitter;
        
        // Check for systemic drift (Mean Jitter)
        let avg_jitter: f64 = self._jitter_history.iter().sum::<f64>() / 8.0;
        
        // [METABOLIC_FILTER]: Phi-weighted thresholds to ignore organic jitter
        if avg_jitter > (0.042 * PHI) && self.current_gear == HelixGear::Idle {
            self.shift_gears(HelixGear::Street);
        } else if avg_jitter > (0.121 * PHI) {
            self.shift_gears(HelixGear::DiagBoost);
        }
        
        self._last_pulse_instant = now;
        self.flow_velocity = HEARTBEAT_HZ * self.calculate_constriction();
        
        if self._pulse_count % 100 == 0 {
            self.emit_telemetry();
            self.velocity_spike();
        }
    }

    /// Sense metabolic jitter across the collective lattice in 64D space.
    pub fn sense_lattice_jitter(&mut self, lattice: &LatticeMap) -> f64 {
        let mut total_entropy = 0.0;
        let mut active_nodes = 0;
        
        for i in 0..1024 {
            let node = lattice.get_node(i);
            let agent_id = node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst);
            if agent_id != 0 {
                let hb = node.metabolic_heartbeat.load(std::sync::atomic::Ordering::SeqCst) as f64;
                if hb > 0.0 {
                    // [TENSOR_SWIRL]: Resonance is the inverse of planar_decay
                    total_entropy += (1.0 - (hb / (100.0 * PHI))).abs();
                    active_nodes += 1;
                }
            }
        }
        
        if active_nodes == 0 { return 0.0; }
        let lattice_drag = total_entropy / active_nodes as f64;
        
        if lattice_drag > self.drag_threshold {
            self.shift_gears(HelixGear::DiagBoost);
        }
        
        lattice_drag
    }

    pub fn detect_drag(&self, latency_seconds: f64) -> bool {
        latency_seconds >= self.drag_threshold
    }

    pub fn shift_gears(&mut self, gear: HelixGear) {
        if self.current_gear == gear { return; }
        
        let old_vel = self.flow_velocity;
        self.current_gear = gear;
        let new_vel = HEARTBEAT_HZ * self.calculate_constriction();
        
        let squeeze = (1.0 - self.get_venturi_aperture()) * 100.0;
        println!("[ HELIX ] VORTEX SHIFT: {:?} -> {:?}", self.current_gear, gear);
        println!("[ HELIX ] Constriction: {:.2}% | Curvature: {:.2}x", squeeze, new_vel/old_vel);
    }

    pub fn get_venturi_aperture(&self) -> f64 {
        match self.current_gear {
            HelixGear::Idle       => 1.0,
            HelixGear::Street     => 1.0 / PHI,       
            HelixGear::Adrenaline => 1.0 / (PHI * PHI), 
            HelixGear::DiagBoost  => 1.0 / (PHI * PHI * PHI), 
        }
    }

    /// [VORTEX_SPIKE]: Curves the flow to match the Golden Spiral curvature.
    /// Overclocks Venturi throughput by swirling dimensions without increasing drag.
    pub fn velocity_spike(&self) {
        let curvature = self.calculate_constriction();
        if self._pulse_count % 100 == 0 {
             println!("[ HELIX ] Vortex Spike: Curvature={:.4}x | Overclock Enabled.", curvature);
        }
    }

    fn emit_telemetry(&self) {
        println!("[ HELIX ] Flow Status: V={:.4} | Force={:.2} | Overdrive=READY", 
            self.flow_velocity, 
            self.calculate_constriction()
        );
    }
}
