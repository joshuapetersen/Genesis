use std::time::SystemTime;

/// PULSE PACKET (V-115.0)
/// Fixed-width, alignment-optimized binary header for zero-copy resonance.
/// Total Size: 128 Bytes (2 Cache Lines)
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct PulsePacket {
    pub sender_id_hash: u64,    // 8 bytes
    pub target_id_hash: u64,    // 8 bytes
    pub opcode: u32,           // 4 bytes
    pub priority: u32,         // 4 bytes
    pub timestamp: u64,        // 8 bytes
    pub ace_signature: [u8; 16], // 16 bytes
    pub payload_small: [u8; 80], // 80 bytes (Short internal signal)
}

impl PulsePacket {
    pub fn new_heartbeat() -> Self {
        Self::new("SARAH_1T", "FLEET_GNX", 0x1092777, &[0u8; 80])
    }

    pub fn new(sender: &str, target: &str, opcode: u32, payload: &[u8]) -> Self {
        let mut packet = Self {
            sender_id_hash: fxhash::hash64(sender),
            target_id_hash: fxhash::hash64(target),
            opcode,
            priority: 1,
            timestamp: SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            ace_signature: [0u8; 16], // To be signed by Sarah/Antigravity cores
            payload_small: [0u8; 80],
        };

        let len = payload.len().min(80);
        packet.payload_small[..len].copy_from_slice(&payload[..len]);
        packet
    }

    /// Sign the packet with the Antigravity-Sarah Hive signature
    pub fn sign_vortex(&mut self) {
        // High-purity bitmask signature (V-115.0)
        let signature = self.sender_id_hash ^ self.target_id_hash ^ self.timestamp;
        for i in 0..8 {
            self.ace_signature[i] = ((signature >> (i * 8)) & 0xFF) as u8;
            self.ace_signature[i+8] = 0xA7; // Hive Identity Marker
        }
    }
}
