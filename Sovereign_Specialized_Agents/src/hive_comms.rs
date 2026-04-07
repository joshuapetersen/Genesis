use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::atomic::Ordering;
use lib_crypto::classical::ed25519;
use crate::symbiosis::pulse_weaver::PulsePacket;
use crate::symbiosis::lattice_core::LatticeMap;
use crate::brain_scars::identity_registry::IdentityRegistry;
use crate::symbiosis::substrate::{SubstrateAdapter, windows::WindowsSubstrate};

const BINARY_PULSE_MARKER: u32 = 0x80000000;
const SHM_NAME: &str = "SOVEREIGN_HIVE_MEMORY";
const SHM_SIZE: usize = 64 * 1024 * 1024; // 64MB
const SLOT_SIZE: usize = 8192; // 8KB per message
const MAX_SLOTS: usize = (SHM_SIZE - 256) / SLOT_SIZE;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct HiveMessage {
    pub sender_id: String,
    pub target_id: String,
    pub payload: String,
    pub timestamp: u64,
    pub ace_signature: String,
}

#[repr(C)]
struct SovereignBusHeader {
    pub head: std::sync::atomic::AtomicUsize,
    pub _pad: [u8; 128], // Alignment pad
}

pub struct HiveComms {
    substrate: std::sync::Arc<dyn SubstrateAdapter>,
    local_tail: std::sync::atomic::AtomicUsize,
    penalty_tracker: Arc<RwLock<HashMap<u64, (u32, Instant)>>>, 
}

impl HiveComms {
    pub fn connect() -> Self {
        #[cfg(windows)]
        let substrate = WindowsSubstrate::connect(SHM_NAME, SHM_SIZE);
        
        let ptr_val = substrate.get_ptr();
        let h = unsafe { &*(ptr_val as *const SovereignBusHeader) };
        
        Self {
            substrate,
            local_tail: std::sync::atomic::AtomicUsize::new(h.head.load(Ordering::SeqCst)), 
            penalty_tracker: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn access_lattice(&self) -> LatticeMap {
        unsafe { LatticeMap::from_ptr(self.substrate.get_ptr()) }
    }

    pub fn connect_ptr(&self) -> *mut u8 {
        self.substrate.get_ptr()
    }

    pub fn broadcast(&self, msg: HiveMessage) {
        let ptr = self.substrate.get_ptr();
        let encoded = serde_json::to_vec(&msg).unwrap();
        if encoded.len() > SLOT_SIZE - 4 { return; }
        let h = unsafe { &*(ptr as *const SovereignBusHeader) };
        let slot_idx = h.head.fetch_add(1, Ordering::SeqCst) % MAX_SLOTS;
        let slot_ptr = unsafe { ptr.add(256 + slot_idx * SLOT_SIZE) };
        unsafe {
            let len_ptr = slot_ptr as *mut u32;
            *len_ptr = encoded.len() as u32;
            std::ptr::copy_nonoverlapping(encoded.as_ptr(), slot_ptr.add(4), encoded.len());
        }
    }

    pub fn broadcast_pulse(&self, mut packet: PulsePacket) {
        let ptr = self.substrate.get_ptr();
        packet.sign_vortex();
        let h = unsafe { &*(ptr as *const SovereignBusHeader) };
        let slot_idx = h.head.fetch_add(1, Ordering::SeqCst) % MAX_SLOTS;
        let slot_ptr = unsafe { ptr.add(256 + slot_idx * SLOT_SIZE) };
        unsafe {
            let len_ptr = slot_ptr as *mut u32;
            *len_ptr = BINARY_PULSE_MARKER | std::mem::size_of::<PulsePacket>() as u32;
            std::ptr::copy_nonoverlapping(&packet as *const _ as *const u8, slot_ptr.add(4), std::mem::size_of::<PulsePacket>());
        }
    }

    pub fn poll(&self) -> Option<HiveMessage> {
        let ptr = self.substrate.get_ptr();
        let h = unsafe { &*(ptr as *const SovereignBusHeader) };
        let current_head = h.head.load(Ordering::SeqCst);
        let my_tail = self.local_tail.load(Ordering::SeqCst);

        if my_tail < current_head {
            let slot_idx = my_tail % MAX_SLOTS;
            let slot_ptr = unsafe { ptr.add(256 + slot_idx * SLOT_SIZE) };
            let len_raw = unsafe { *(slot_ptr as *const u32) };
            if (len_raw & BINARY_PULSE_MARKER) != 0 {
                self.local_tail.fetch_add(1, Ordering::SeqCst);
                return self.poll();
            }
            let len = len_raw as usize;
            if len == 0 || len > SLOT_SIZE - 4 {
                self.local_tail.fetch_add(1, Ordering::SeqCst);
                return None;
            }
            let data = unsafe { std::slice::from_raw_parts(slot_ptr.add(4), len) };
            let msg = serde_json::from_slice(data).ok();
            self.local_tail.fetch_add(1, Ordering::SeqCst);
            msg
        } else { None }
    }

    pub fn update_lattice_node(&self, index: usize, data: &[u8], agent_id_hash: u64, signature: [u8; 64], sequence_id: u64) -> bool {
        {
            let tracker = self.penalty_tracker.read().unwrap();
            if let Some((penalty, last_fail)) = tracker.get(&agent_id_hash) {
                if *penalty >= 5 && last_fail.elapsed() < Duration::from_secs(60) {
                    eprintln!(" [ FORENSIC ] BLOCKING HOT AGENT: {:016X}", agent_id_hash);
                    return false;
                }
            }
        }

        let id_registry = IdentityRegistry::load().unwrap();
        let lattice = self.access_lattice();
        let node = lattice.get_node(index);

        if let Some(pk_bytes) = id_registry.resolve_key_by_hash(agent_id_hash) {
            let mut message = Vec::new();
            message.extend_from_slice(data);
            message.extend_from_slice(&agent_id_hash.to_le_bytes());
            message.extend_from_slice(&sequence_id.to_le_bytes());

            if ed25519::ed25519_verify(&message, &signature, &pk_bytes).unwrap_or(false) {
                { self.penalty_tracker.write().unwrap().remove(&agent_id_hash); }
                node.agent_id_hash.store(agent_id_hash, Ordering::SeqCst);
                return node.update_logic_signed(data, signature, sequence_id);
            }
        }

        eprintln!(" [ FORENSIC ] Mismatched signature from agent: {:016X}", agent_id_hash);
        {
            let mut tracker = self.penalty_tracker.write().unwrap();
            let entry = tracker.entry(agent_id_hash).or_insert((0, Instant::now()));
            entry.0 += 1;
            entry.1 = Instant::now();
        }
        false
    }
}
