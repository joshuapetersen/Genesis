use sovereign_specialized_agents::hive_comms::HiveComms;
use sovereign_specialized_agents::agent_factory::SovereignAgentFactory;
use sovereign_specialized_agents::pulse_weaver::PulseWeaver;
use sovereign_specialized_agents::brain_scars::BrainScarVault;
use sovereign_specialized_agents::brain_scars::crystallizer::LogicCrystallizer;
use sovereign_specialized_agents::symbiosis::consensus::NeuralConsensusEngine;
use anyhow::Result;
use std::sync::Arc;

/// SOVEREIGN SPECIALIZED AGENTS KERNEL (V-131.0)
/// Finalized autonomous neural organism.
#[tokio::main]
async fn main() -> Result<()> {
    println!("--------------------------------------------------");
    println!("   SOVEREIGN SPECIALIZED AGENTS KERNEL V-132.0    ");
    println!("   [ CALIBRATION ] 1.09277703703 HZ (Forensic)    ");
    println!("--------------------------------------------------");

    // 1. Initialize Hive Communications (Lattice Substrate)
    let hive_comms = Arc::new(HiveComms::connect());

    // 2. Initialize Agent Factory & Identity Management (V-131.0)
    let agent_factory = SovereignAgentFactory::new(hive_comms.clone());
    
    // 3. Initialize Hive Synthesis (Consensus Engine)
    let consensus_engine = Arc::new(NeuralConsensusEngine::new(
        hive_comms.clone(), 
        agent_factory.get_mesh_router(),
        agent_factory.identity_manager.clone()
    ));

    // 4. Initialize crystallization (Forensic Persistence)
    let scar_vault = Arc::new(BrainScarVault::new(hive_comms.clone())?);
    let crystallizer = Arc::new(LogicCrystallizer::new(scar_vault, hive_comms.clone(), consensus_engine.clone()));

    println!("[!] Hive substrate initialized.");

    // Heartbeat Strike
    let heart_hive = hive_comms.clone();
    tokio::spawn(async move {
        let mut pulse = PulseWeaver::new(heart_hive);
        pulse.start_pulse().await;
    });

    // Crystallization Strike
    let crys_clone = crystallizer.clone();
    tokio::spawn(async move {
        crys_clone.start_crystallization().await;
    });

    // Keep the kernel alive
    println!("[ KERNEL ] Sovereign Neural Fabric Active at 1.092777 Hz.");
    tokio::signal::ctrl_c().await?;
    
    Ok(())
}
