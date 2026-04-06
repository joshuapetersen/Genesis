use std::sync::Arc;
use tokio::time::{interval, Duration};
use crate::brain_scars::BrainScarVault;
use crate::hive_comms::HiveComms;
use crate::symbiosis::consensus::NeuralConsensusEngine;
use std::sync::atomic::Ordering;

/// LOGIC CRYSTALLIZER (V-1.0)
/// Goal: Autonomous persistence of high-resonance lattice logic.
pub struct LogicCrystallizer {
    vault: Arc<BrainScarVault>,
    hive: Arc<HiveComms>,
    consensus: Arc<NeuralConsensusEngine>,
}

impl LogicCrystallizer {
    pub fn new(vault: Arc<BrainScarVault>, hive: Arc<HiveComms>, consensus: Arc<NeuralConsensusEngine>) -> Self {
        Self { vault, hive, consensus }
    }

    /// V-129.0: Start the crystallization heartbeat
    pub async fn start_crystallization(&self) {
        // Aligned to 1.092777 Hz (approx. 915ms)
        let mut interval = interval(Duration::from_micros(915100));
        println!("[ CRYSTALLIZER ] Logic Crystallizer Online | Frequency: 1.092777 Hz");

        loop {
            interval.tick().await;
            if let Err(e) = self.execute_crystallization_strike().await {
                eprintln!("[ CRYSTALLIZER ] Strike Error: {:?}", e);
            }
        }
    }

    /// V-131.0: Scan the lattice and persist nodes that reached consensus with identity verification
    pub async fn execute_crystallization_strike(&self) -> anyhow::Result<()> {
        // 1. Collect and verify signatures for the heartbeat
        self.consensus.collect_logic_votes().await?;
        
        // 2. Identify winning logic (2/3 quorum confirmed)
        if let Some(winning_hash) = self.consensus.determine_winning_logic().await {
            println!("[ CRYSTALLIZER ] Hive Consensus Achieved | Logic Hash: {}", winning_hash);
            
            let lattice = self.hive.access_lattice();
            // Find a node containing the winning logic and persist it
            for i in 0..16384 {
                let node = lattice.get_node(i);
                if node.agent_id_hash.load(Ordering::SeqCst) != 0 {
                    let current_hash = format!("{:x}", md5::compute(&node.logic_payload));
                    if current_hash == winning_hash {
                        // V-131.0: Final Identity Verification Strike
                        let id_manager = self.consensus.identity_manager().read().await;
                        let mut agent_did = None;
                        
                        if let Some(identity) = id_manager.find_identity_by_truncated_hash(node.agent_id_hash.load(Ordering::SeqCst)) {
                            agent_did = Some(identity.did.clone());
                            println!("[ CRYSTALLIZER ] Identity Verified: {} | Striking Vault for Node {}", identity.did, i);
                        } else {
                            println!("[ CRYSTALLIZER ] Identity Lookup Failed for Node {} | Using Anonymous", i);
                        }
                        
                        // Crystallization Strike: Persist to Vault with forensic ID
                        self.vault.persist_lattice_refinement(i, "sovereign_identity_consensus", agent_did)?;
                        break;
                    }
                }
            }
        }
        
        // 3. Clear bins for next heartbeat
        self.consensus.clear_consensus_bins().await;
        
        Ok(())
    }
}
