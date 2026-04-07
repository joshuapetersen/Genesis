use std::sync::Arc;
use crate::hive_comms::HiveComms;
use crate::symbiosis::pulse_weaver::PulsePacket;

/// PULSE WEAVER (V-132.8)
/// Heartbeat of the Sovereign Hive
pub struct PulseWeaver {
    hive: Arc<HiveComms>,
    sequence_counter: std::sync::atomic::AtomicU64,
    calibration: HeartbeatCalibration,
}

#[derive(Debug, Clone, Copy)]
pub struct HeartbeatCalibration {
    pub frequency_hz: f64,
    pub jitter_tolerance_ms: u64,
}

impl Default for HeartbeatCalibration {
    fn default() -> Self {
        let freq = std::env::var("SOVEREIGN_HEARTBEAT")
            .unwrap_or_else(|_| "1.092777037037037".to_string())
            .parse::<f64>()
            .unwrap_or(1.092777037037037);
            
        Self {
            frequency_hz: freq,
            jitter_tolerance_ms: 10,
        }
    }
}

impl PulseWeaver {
    pub fn new(hive: Arc<HiveComms>) -> Self {
        Self { 
            hive,
            sequence_counter: std::sync::atomic::AtomicU64::new(1),
            calibration: HeartbeatCalibration::default(),
        }
    }

    /// V-132.8: Universal Pulse Strike
    pub async fn start_pulse(&mut self) {
        let micros = (1_000_000.0 / self.calibration.frequency_hz) as u64;
        let mut interval = tokio::time::interval(tokio::time::Duration::from_micros(micros));
        
        println!("[ PULSE ] Universal Weaver Core Online | Calibration: {} Hz", self.calibration.frequency_hz);

        loop {
            interval.tick().await;
            self.execute_heartbeat_strike().await;
        }
    }

    async fn execute_heartbeat_strike(&self) {
        let packet = PulsePacket::new_heartbeat();
        self.hive.broadcast_pulse(packet);
        
        let data = b"PULSE_STRIKE_ACTIVE_V132";
        let seq = self.sequence_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        
        let root_id = "did:sov:root_anchor";
        let hash_bytes = md5::compute(root_id.as_bytes());
        let agent_id_hash = u64::from_le_bytes(hash_bytes.as_slice()[0..8].try_into().unwrap());

        let root_seed = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
            0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
            0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
            0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x1F,
        ];

        let mut message = Vec::new();
        message.extend_from_slice(data);
        message.extend_from_slice(&agent_id_hash.to_le_bytes());
        message.extend_from_slice(&seq.to_le_bytes());

        if let Ok(sig_vec) = lib_crypto::classical::ed25519::ed25519_sign(&message, &root_seed) {
            let mut sig = [0u8; 64];
            sig.copy_from_slice(&sig_vec[..64]);
            self.hive.update_lattice_node(0, data, agent_id_hash, sig, seq);
        }
    }
}
