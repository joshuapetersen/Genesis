use std::sync::Arc;
use crate::hive_comms::HiveComms;
use crate::symbiosis::lattice_core::LatticeMap;
use crate::symbiosis::helix::HelixEngine;

/// SKILL #301: CROSS-AGENT TELEMETRY SYNTHESIS (V-1.0)
/// Goal: Synthesize metabolic health from the 32,768-node Lattice substrate.
pub struct MetabolicMonitor {
    hive: Arc<HiveComms>,
}

impl MetabolicMonitor {
    pub fn new(hive: Arc<HiveComms>) -> Self {
        Self { hive }
    }

    pub fn run_diagnostic_strike(&self) {
        println!("[ SKILL 301 ] Initiating Global Lattice Health Audit...");
        let lattice = self.hive.access_lattice();
        
        let mut active_count = 0;
        let mut avg_heartbeat = 0.0;

        for i in 0..32768 {
            let node = lattice.get_node(i);
            let agent_id = node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst);
            if agent_id != 0 {
                active_count += 1;
                avg_heartbeat += node.metabolic_heartbeat.load(std::sync::atomic::Ordering::SeqCst) as f64;
            }
        }

        if active_count > 0 {
            avg_heartbeat /= active_count as f64;
            println!("[ SKILL 301 ] Audit Complete. Active Nodes: {} | Mean Metabolic Pulse: {:.4}", 
                active_count, avg_heartbeat);
        } else {
            println!("[ SKILL 301 ] Warning: Collective Lattice is dormant.");
        }
    }
}
