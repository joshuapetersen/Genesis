use windows::core::{PCWSTR, HSTRING};
use windows::Win32::System::Memory::{
    CreateFileMappingW, MapViewOfFile, FILE_MAP_ALL_ACCESS, PAGE_READWRITE,
};
use windows::Win32::Foundation::{HANDLE, INVALID_HANDLE_VALUE, CloseHandle};
use serde::{Serialize, Deserialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::symbiosis::pulse_weaver::PulsePacket;
use crate::symbiosis::lattice_core::LatticeMap;

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
    head: AtomicUsize,
    _pad: [u8; 128], // Alignment pad
}

pub struct HiveComms {
    handle: HANDLE,
    ptr: *mut u8,
    local_tail: AtomicUsize,
}

impl HiveComms {
    pub fn connect() -> Self {
        let name_hstring = HSTRING::from(SHM_NAME);
        let name_pcwstr = PCWSTR(name_hstring.as_ptr());
        
        unsafe {
            let handle = CreateFileMappingW(
                INVALID_HANDLE_VALUE,
                None,
                PAGE_READWRITE,
                0,
                SHM_SIZE as u32,
                name_pcwstr,
            ).expect("Failed to create hive memory mapping");
            
            let ptr = MapViewOfFile(handle, FILE_MAP_ALL_ACCESS, 0, 0, SHM_SIZE);
            if ptr.Value.is_null() {
                panic!("Failed to map view of hive memory");
            }
            
            let ptr_val = ptr.Value as *mut u8;
            let h = &*(ptr_val as *const SovereignBusHeader);
            
            Self {
                handle,
                ptr: ptr_val,
                local_tail: AtomicUsize::new(h.head.load(Ordering::SeqCst)), 
            }
        }
    }

    pub fn connect_ptr(&self) -> *mut u8 {
        self.ptr
    }

    fn header(&self) -> &SovereignBusHeader {
        unsafe { &*(self.ptr as *const SovereignBusHeader) }
    }

    pub fn broadcast(&self, msg: HiveMessage) {
        let encoded = serde_json::to_vec(&msg).unwrap();
        if encoded.len() > SLOT_SIZE - 4 { return; }

        let h = self.header();
        let slot_idx = h.head.fetch_add(1, Ordering::SeqCst) % MAX_SLOTS;
        
        let slot_ptr = unsafe { self.ptr.add(256 + slot_idx * SLOT_SIZE) };
        unsafe {
            let len_ptr = slot_ptr as *mut u32;
            *len_ptr = encoded.len() as u32;
            std::ptr::copy_nonoverlapping(encoded.as_ptr(), slot_ptr.add(4), encoded.len());
        }
    }

    /// V-115.0: Zero-Copy Binary Pulse Broadcast
    pub fn broadcast_pulse(&self, mut packet: PulsePacket) {
        packet.sign_vortex();
        let h = self.header();
        let slot_idx = h.head.fetch_add(1, Ordering::SeqCst) % MAX_SLOTS;
        
        let slot_ptr = unsafe { self.ptr.add(256 + slot_idx * SLOT_SIZE) };
        unsafe {
            let len_ptr = slot_ptr as *mut u32;
            *len_ptr = BINARY_PULSE_MARKER | std::mem::size_of::<PulsePacket>() as u32;
            std::ptr::copy_nonoverlapping(
                &packet as *const _ as *const u8,
                slot_ptr.add(4),
                std::mem::size_of::<PulsePacket>()
            );
        }
    }

    pub fn poll(&self) -> Option<HiveMessage> {
        let h = self.header();
        let current_head = h.head.load(Ordering::SeqCst);
        let my_tail = self.local_tail.load(Ordering::SeqCst);

        if my_tail < current_head {
            let slot_idx = my_tail % MAX_SLOTS;
            let slot_ptr = unsafe { self.ptr.add(256 + slot_idx * SLOT_SIZE) };
            
            let len_raw = unsafe { *(slot_ptr as *const u32) };
            if (len_raw & BINARY_PULSE_MARKER) != 0 {
                // Binary Pulse detected, skip for traditional poll
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
        } else {
            None
        }
    }

    /// V-115.0: Zero-Copy Binary Pulse Polling
    pub fn poll_pulse(&self) -> Option<PulsePacket> {
        let h = self.header();
        let current_head = h.head.load(Ordering::SeqCst);
        let my_tail = self.local_tail.load(Ordering::SeqCst);

        if my_tail < current_head {
            let slot_idx = my_tail % MAX_SLOTS;
            let slot_ptr = unsafe { self.ptr.add(256 + slot_idx * SLOT_SIZE) };
            
            let len_raw = unsafe { *(slot_ptr as *const u32) };
            if (len_raw & BINARY_PULSE_MARKER) == 0 {
                // Traditional JSON message detected, skip for pulse poll
                self.local_tail.fetch_add(1, Ordering::SeqCst);
                return self.poll_pulse();
            }

            let packet = unsafe {
                std::ptr::read(slot_ptr.add(4) as *const PulsePacket)
            };
            
            self.local_tail.fetch_add(1, Ordering::SeqCst);
            Some(packet)
        } else {
            None
        }
    }

    /// V-116.0: Access collective Lattice Memory substrate
    pub fn access_lattice(&self) -> LatticeMap {
        unsafe {
            // Collective logic pool resides at a 32MB offset in SHM
            LatticeMap::from_ptr(self.ptr)
        }
    }

    /// V-131.0: Generate a valid brain signature placeholder for substrate updates
    pub fn generate_brain_signature(&self, agent_id_hash: u64) -> [u8; 64] {
        let mut sig = [0u8; 64];
        let lattice = self.access_lattice();
        
        let ts = lattice.get_node(0).logic_timestamp.load(Ordering::SeqCst);
        let expected = (agent_id_hash ^ ts).rotate_left(13);
        
        sig[0..8].copy_from_slice(&expected.to_le_bytes());
        sig[8] = 0xBC; // Brain Identity Marker
        sig
    }

    /// V-131.0: Perform an authenticated write to a Lattice node with brain signature
    pub fn update_lattice_node(&self, index: usize, data: &[u8], agent_id_hash: u64) -> bool {
        let lattice = self.access_lattice();
        let node = lattice.get_node(index);
        let sig = self.generate_brain_signature(agent_id_hash);
        node.update_logic_signed(data, sig)
    }
}

impl Drop for HiveComms {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.handle); }
    }
}

unsafe impl Send for HiveComms {}
unsafe impl Sync for HiveComms {}
