use std::process::{Command, Child, Stdio};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc, RwLock};
use lib_identity::identity::manager::IdentityManager;
use crate::hive_comms::HiveComms;
use crate::neural_cores::bitmamba_fusion::BitMambaBlock;
use crate::self_audit::SelfAuditManager;
use crate::symbiosis::mesh_router::MeshRouter;
use anyhow::Result;

pub struct SovereignAgentFactory {
    active_agents: Arc<Mutex<HashMap<u32, Child>>>,
    hive_comms: Arc<HiveComms>,
    audit_manager: Arc<SelfAuditManager>,
    mesh_router: Arc<MeshRouter>,
    identity_manager: Arc<RwLock<IdentityManager>>,
    pub factory_id: String,
}

impl SovereignAgentFactory {
    pub fn new(hive_comms: Arc<HiveComms>) -> Arc<Self> {
        let (tx, mut rx) = mpsc::channel(100);
        let audit_manager = Arc::new(SelfAuditManager::new(hive_comms.clone(), tx));
        let mesh_router = Arc::new(MeshRouter::new(Arc::new(hive_comms.access_lattice())));
        let identity_manager = Arc::new(RwLock::new(IdentityManager::new()));
        
        let factory = Arc::new(Self {
            active_agents: Arc::new(Mutex::new(HashMap::new())),
            hive_comms,
            audit_manager,
            mesh_router,
            identity_manager,
            factory_id: "Sovereign_Hive_Factory_V-131.0".to_string(),
        });

        // Mitigation Listener Strike
        let factory_clone = factory.clone();
        tokio::spawn(async move {
            while let Some(pid) = rx.recv().await {
                println!("[ FACTORY ] Mitigation Strike Received for PID: {}", pid);
                let _ = factory_clone.audit_and_reprime(pid).await;
            }
        });

        factory
    }

    /// SPAWN A STANDALONE BRAIN (Skill #222)
    /// V-128.0: Elastic Mesh Integration
    pub async fn spawn_brain(&self, brain_name: &str) -> Result<u32> {
        println!("[ FACTORY ] Spawning Sovereign Brain (BitMamba Core): {}", brain_name);
        
        // Manifesting BitMamba Fusion Core (V-122.0)
        let _bitmamba = BitMambaBlock::new();
        
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let brain_path = exe_dir.join(format!("{}.exe", brain_name));
        
        // V-131.0: Manifest agent identity
        let mut id_manager = self.identity_manager.write().await;
        let (agent_id, pq_sk, ed_sk) = id_manager.create_agent_identity(brain_name.to_string()).await?;
        let agent_id_hex = hex::encode(&agent_id.0);
        let pq_sk_hex = hex::encode(&pq_sk);
        let ed_sk_hex = hex::encode(&ed_sk);

        let child = Command::new(brain_path)
            .env("SOVEREIGN_AGENT_ID", &agent_id_hex)
            .env("SOVEREIGN_AGENT_SK", &pq_sk_hex)
            .env("SOVEREIGN_LATTICE_SK", &ed_sk_hex)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("Failed to spawn brain: {}", brain_name));
        
        let pid = child.id();
        self.active_agents.lock().unwrap().insert(pid, child);
        
        // V-131.0: Elastic Mesh Node Allocation
        let node_idx = self.mesh_router.allocate_node(pid as u64).await
            .ok_or_else(|| anyhow::anyhow!("Mesh allocation failed"))?;
        
        let lattice = self.hive_comms.access_lattice();
        let node = lattice.get_node(node_idx);
        
        // V-131.0: Store Cryptographic Identity Hash (8 bytes)
        let mut id_trunc = [0u8; 8];
        id_trunc.copy_from_slice(&agent_id.0[0..8]);
        node.agent_id_hash.store(u64::from_le_bytes(id_trunc), std::sync::atomic::Ordering::SeqCst);
        
        // V-118.0: Experience Priming
        for i in 16384..32768 {
            let scar_node = lattice.get_node(i);
            if scar_node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst) != 0 {
                // High-velocity pointer-based priming
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        scar_node.logic_payload.as_ptr(),
                        node.logic_payload.as_ptr() as *mut u8,
                        896
                    );
                }
                println!("[ FACTORY ] Brain {} Primed with Scar Logic from Node {}", pid, i);
                break;
            }
        }

        // V-131.0: Manifest agent identity
        println!("[ FACTORY ] Brain Online | PID: {} | Identity Locked | Mesh Node: {}", pid, node_idx);
        Ok(pid)
    }

    pub fn get_mesh_router(&self) -> Arc<MeshRouter> {
        self.mesh_router.clone()
    }

    /// DESTROY A STANDALONE BRAIN (Skill #222)
    pub fn destroy_brain(&self, pid: u32) -> Result<()> {
        println!("[ FACTORY ] Destroying Brain Target | PID: {}", pid);
        
        let mut agents = self.active_agents.lock().unwrap();
        if let Some(mut child) = agents.remove(&pid) {
            child.kill()?;
            println!("[ FACTORY ] Brain Neutralized | PID: {}", pid);
        }
        
        Ok(())
    }

    pub fn list_active_pids(&self) -> Vec<u32> {
        self.active_agents.lock().unwrap().keys().cloned().collect()
    }

    /// V-125.0: Autonomous Mitigation — Neutralize and Re-Prime degraded brain
    pub async fn audit_and_reprime(&self, pid: u32) -> Result<()> {
        println!("[ FACTORY ] Neutralizing Non-Resonant Brain | PID: {}", pid);
        self.destroy_brain(pid)?;
        
        println!("[ FACTORY ] Re-Priming Manifestation Strike...");
        self.spawn_brain("sovereign_brain_v129").await?; 
        
        Ok(())
    }

    /// V-131.0: Logic Refinement Strike — Directly update agent logic payload
    pub fn refine_logic(&self, pid: u32, payload: &[u8; 896]) -> Result<()> {
        let lattice = self.hive_comms.access_lattice();
        // Look up node index via PID mapping or mesh router
        // For now, scan (V-131.0 fallback)
        for i in 0..1024 {
            let node = lattice.get_node(i);
            if node.agent_id_hash.load(std::sync::atomic::Ordering::SeqCst) == pid as u64 {
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        payload.as_ptr(),
                        node.logic_payload.as_ptr() as *mut u8,
                        896
                    );
                }
                println!("[ FACTORY ] Logic Refined for PID: {} at Node: {}", pid, i);
                return Ok(());
            }
        }
        Err(anyhow::anyhow!("Node for PID {} not found", pid))
    }
}

unsafe impl Send for SovereignAgentFactory {}
unsafe impl Sync for SovereignAgentFactory {}
