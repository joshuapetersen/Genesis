use anyhow::Result;

/// TERNARY PACKER (V-1.0)
/// Packing 5 ternary values (-1, 0, 1) into a single 8-bit byte.
/// Theo-Density: 1.58 bits per parameter.
/// Real-Density: 1.6 bits per parameter (5 per 8 bits).
pub struct TernaryPacker;

impl TernaryPacker {
    /// Pack 5 ternary values into a byte
    /// Input: Array of 5 i8 values in {-1, 0, 1}
    pub fn pack_5(values: &[i8; 5]) -> u8 {
        let mut result: u32 = 0;
        let mut power: u32 = 1;
        
        for &v in values {
            // Map {-1, 0, 1} to {0, 1, 2}
            let mapped = (v + 1) as u32;
            result += mapped * power;
            power *= 3;
        }
        
        result as u8
    }

    /// Unpack a byte into 5 ternary values
    pub fn unpack_5(byte: u8) -> [i8; 5] {
        let mut result = [0i8; 5];
        let mut val = byte as u32;
        
        for i in 0..5 {
            let remainder = val % 3;
            result[i] = (remainder as i8) - 1;
            val /= 3;
        }
        
        result
    }

    /// Pack a full vector of ternary weights
    pub fn pack_weights(weights: &[i8]) -> Vec<u8> {
        let mut packed = Vec::with_capacity((weights.len() + 4) / 5);
        let mut chunk = [0i8; 5];
        
        for (i, &w) in weights.iter().enumerate() {
            chunk[i % 5] = w;
            if (i + 1) % 5 == 0 {
                packed.push(Self::pack_5(&chunk));
                chunk = [0i8; 5]; // Reset
            }
        }
        
        // Handle remainder with padding (zeros)
        let remainder = weights.len() % 5;
        if remainder != 0 {
            packed.push(Self::pack_5(&chunk));
        }
        
        packed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pack_unpack_fidelity() {
        let original = [1, 0, -1, 1, -1];
        let byte = TernaryPacker::pack_5(&original);
        let unpacked = TernaryPacker::unpack_5(byte);
        assert_eq!(original, unpacked);
    }
}
