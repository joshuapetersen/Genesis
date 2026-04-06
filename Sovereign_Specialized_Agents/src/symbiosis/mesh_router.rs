use crate::symbiosis::lattice_core::{LatticeMap, LatticeNode};
use std::sync::atomic::{AtomicU32, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// MESH ROUTER (V-1.0)
/// Goal: Dynamic node orchestration for the 1,450+ agent fleet.
pub struct MeshRouter {
    lattice_map: Arc<LatticeMap>,
    allocation_table: Arc<RwLock<HashMap<u64, usize>>>, // Agent ID Hash -> Node Index
    next_available_node: AtomicU32,
}

impl MeshRouter {
    pub fn new(lattice_map: Arc<LatticeMap>) -> Self {
        Self {
            lattice_map,
            allocation_table: Arc::new(RwLock::new(HashMap::new())),
            next_available_node: AtomicU32::new(0),
        }
    }

    /// V-128.0: Allocate a lattice node for a specialized agent
    pub async fn allocate_node(&self, agent_id_hash: u64) -> Option<usize> {
        let mut table = self.allocation_table.write().await;
        
        if let Some(&index) = table.get(&agent_id_hash) {
            return Some(index);
        }

        let index = self.next_available_node.fetch_add(1, Ordering::SeqCst) as usize;
        if index >= 32768 {
            println!("[ MESH ] FATAL: Lattice exhaustion | Capacity: 32,768 exceeded.");
            return None;
        }

        table.insert(agent_id_hash, index);
        
        // Initialize node metadata
        let node = self.lattice_map.get_node(index);
        node.agent_id_hash.store(agent_id_hash, Ordering::SeqCst);
        
        println!("[ MESH ] Allocated Node {} for Agent Hash {}", index, agent_id_hash);
        Some(index)
    }

    /// Retrieve node for a specific agent
    pub async fn get_agent_node(&self, agent_id_hash: u64) -> Option<&'static LatticeNode> {
        let table = self.allocation_table.read().await;
        // The LatticeMap returns nodes from a 'static slice, so this is safe.
        table.get(&agent_id_hash).map(|&idx| self.lattice_map.get_node(idx))
    }

    /// V-128.0: Execute cross-agent logic synthesis
    pub async fn synthesize_cross_logic(&self, source_hash: u64, target_hash: u64, payload: &[u8]) -> bool {
        let source_node = self.get_agent_node(source_hash).await;
        let target_node = self.get_agent_node(target_hash).await;

        if let (Some(_src), Some(target)) = (source_node, target_node) {
            // High-velocity logic transfer via unsafe pointer strike (aligned with lattice_core pattern)
            let len = payload.len().min(896);
            unsafe {
                let payload_ptr = target.logic_payload.as_ptr() as *mut u8;
                std::ptr::copy_nonoverlapping(payload.as_ptr(), payload_ptr, len);
            }
            
            target.logic_timestamp.store(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64,
                Ordering::SeqCst
            );
            return true;
        }
        false
    }
}
