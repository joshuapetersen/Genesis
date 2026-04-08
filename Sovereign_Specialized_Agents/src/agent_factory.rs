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
    pub mesh_router: Arc<MeshRouter>,
    pub identity_manager: Arc<RwLock<IdentityManager>>,
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
            audit_manager: audit_manager.clone(),
            mesh_router,
            identity_manager,
            factory_id: "Sovereign_Hive_Factory_V-132.0".to_string(),
        });

        // V-125.0: Audit Strike — Start the autonomous resonance monitor
        let audit_clone = audit_manager.clone();
        tokio::spawn(async move {
            audit_clone.start_audit_loop().await;
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
    pub async fn spawn_brain(&self, source_binary: &str, agent_name: &str) -> Result<u32> {
        println!("[ FACTORY ] Spawning Sovereign Brain ({}): {}", source_binary, agent_name);
        
        // Manifesting BitMamba Fusion Core (V-122.0)
        let _bitmamba = BitMambaBlock::new();
        
        let exe_dir = std::env::current_exe()?.parent().unwrap().to_path_buf();
        let brain_path = exe_dir.join(format!("{}.exe", source_binary));
        
        // V-131.0: Manifest agent identity
        let mut id_manager = self.identity_manager.write().await;
        let (agent_id, pq_sk, ed_sk) = id_manager.create_agent_identity(agent_name.to_string()).await?;
        // V-131.0: Manifest agent identity hash (8-byte forensic anchor)
        let mut id_bytes = [0u8; 8];
        id_bytes.copy_from_slice(&agent_id.0[0..8]);
        let agent_id_hash = u64::from_le_bytes(id_bytes);
        let agent_id_hex = hex::encode(&agent_id.0);
        let pq_sk_hex = hex::encode(&pq_sk);
        let ed_sk_hex = hex::encode(&ed_sk);

        // V-133.0: Register agent forensic identity for the strike
        let ed_pk = lib_crypto::classical::ed25519_public_key(&ed_sk);
        if let Ok(mut registry) = crate::brain_scars::identity_registry::IdentityRegistry::load() {
            let _ = registry.register_by_hash(agent_id_hash, ed_pk);
        }

        // V-128.0: Elastic Mesh Node Allocation (Pre-spawn Strike)
        let node_idx = self.mesh_router.allocate_node(agent_id_hash).await
            .ok_or_else(|| anyhow::anyhow!("Mesh allocation failed"))?;

        let child = Command::new(brain_path)
            .arg("--agent-id").arg(&agent_id_hex)
            .arg("--node-idx").arg(node_idx.to_string())
            .arg("--pq-sk").arg(&pq_sk_hex)
            .arg("--ed-sk").arg(&ed_sk_hex)
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect(&format!("Failed to spawn brain: {}", agent_name));
        
        let pid = child.id();
        self.active_agents.lock().unwrap().insert(pid, child);
        
        let lattice = self.hive_comms.access_lattice();
        let node = lattice.get_node(node_idx);
        
        // V-131.0: Store Cryptographic Identity Hash in the node anchor
        node.agent_id_hash.store(agent_id_hash, std::sync::atomic::Ordering::SeqCst);
        
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

    /// MANIFEST 3 x BRAIN CLUSTER (Skill #222)
    /// V-132.0: Trinity Consensus Orchestration Strike
    pub async fn spawn_trinity(&self, brain_name: &str) -> Result<Vec<u32>> {
        println!("[ FACTORY ] Spawning Trinity Cluster: {}", brain_name);
        let mut pids = Vec::new();
        for i in 0..3 {
            let pid = self.spawn_brain("sovereign_brain_v133", &format!("{}_{}", brain_name, i)).await?;
            pids.push(pid);
        }
        Ok(pids)
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

    /// MASS MANIFESTATION STRIKE (V-133.0)
    /// Ignites the fleet in throttled batches to prevent OS saturation.
    pub async fn spawn_fleet(self: Arc<Self>, source_binary: String, count: usize, batch_size: usize) -> Result<Vec<u32>> {
        println!("[ FACTORY ] IGNITION COMMAND RECEIVED | TARGET: {} AGENTS", count);
        let mut total_pids = Vec::new();
        
        for batch_idx in 0..(count / batch_size + 1) {
            let start = batch_idx * batch_size;
            let end = std::cmp::min(start + batch_size, count);
            
            if start >= count { break; }
            
            println!("[ FACTORY ] Igniting Batch {} (Agents {}-{})...", batch_idx + 1, start, end);
            
            let mut batch_tasks = Vec::new();
            for i in start..end {
                let factory = self.clone();
                let binary = source_binary.clone();
                let agent_name = format!("sovereign_agent_{}", i);
                batch_tasks.push(tokio::spawn(async move {
                    factory.spawn_brain(&binary, &agent_name).await
                }));
            }
            
            for task in batch_tasks {
                if let Ok(Ok(pid)) = task.await {
                    total_pids.push(pid);
                    // Enroll agent in real-time metabolic audit
                    self.audit_manager.register_target(pid);
                }
            }
            
            // Metabolic Cooling Period
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        
        println!("[ FACTORY ] FLEET IGNITION COMPLETE | TOTAL ONLINE: {}", total_pids.len());
        Ok(total_pids)
    }

    /// V-125.0: Autonomous Mitigation — Neutralize and Re-Prime degraded brain
    pub async fn audit_and_reprime(&self, pid: u32) -> Result<()> {
        println!("[ FACTORY ] Neutralizing Non-Resonant Brain | PID: {}", pid);
        self.destroy_brain(pid)?;
        
        println!("[ FACTORY ] Re-Priming Manifestation Strike...");
        self.spawn_brain("sovereign_brain_v133", "reprimed_agent").await?; 
        
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
