use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;
use crate::neural_cores::resonance_audit::ResonanceAuditor;
use crate::hive_comms::HiveComms;

/// SELF-AUDIT MANAGER (V-1.0)
/// Goal: Autonomous background diagnostics for the agent fleet.
pub struct SelfAuditManager {
    hive_comms: Arc<HiveComms>,
    mitigation_tx: mpsc::Sender<u32>, // Channel to factory
}

impl SelfAuditManager {
    pub fn new(hive_comms: Arc<HiveComms>, mitigation_tx: mpsc::Sender<u32>) -> Self {
        Self { hive_comms, mitigation_tx }
    }

    /// V-125.0: Start the autonomous resonance strike loop
    pub async fn start_audit_loop(&self, active_pids: Vec<u32>) {
        let hive = self.hive_comms.clone();
        let tx = self.mitigation_tx.clone();
        
        tokio::spawn(async move {
            println!("[ SELF-AUDIT ] Autonomous Resonance Loop Initiated.");
            
            loop {
                for pid in &active_pids {
                    let node_idx = *pid as usize % 32768;
                    let lattice = hive.access_lattice();
                    let node = lattice.get_node(node_idx);
                    
                    let packed_weights = node.logic_payload.to_vec();
                    let mock_baseline = vec![0i8; packed_weights.len() * 5]; 
                    
                    let score = ResonanceAuditor::execute_fidelity_audit(&mock_baseline, &packed_weights);
                    
                    if score < 0.98 {
                        println!("[ SELF-AUDIT ] WARNING: Brain {} degradation detected (Score: {:.4}). Triggering mitigation strike...", pid, score);
                        let _ = tx.send(*pid).await;
                    }
                }
                
                sleep(Duration::from_secs(60)).await;
            }
        });
    }
}
