/// UNIVERSAL LOGIC SYNTHESIZER (V-132.8)
/// Goal: Autonomously merge high-resonance logic fragments using generic, platform-agnostic bit-level voting.

pub trait UniversalSynthesizer {
    /// Perform a weighted resonance synthesis on arbitrary logic slices.
    /// All slices must be of equal length.
    fn synthesize_generic(fragments: &[(&[u8], usize)]) -> Vec<u8>;
}

pub struct LogicSynthesizer;

impl UniversalSynthesizer for LogicSynthesizer {
    /// V-132.8: Weighted Resonance Synthesis Strike (Universal Edition)
    /// Composes a single logic manifest from multiple parent fragments based on resonance amplitude.
    fn synthesize_generic(fragments: &[(&[u8], usize)]) -> Vec<u8> {
        if fragments.is_empty() {
            return Vec::new();
        }
        
        let len = fragments[0].0.len();
        if fragments.len() == 1 {
            return fragments[0].0.to_vec();
        }

        let mut composite = vec![0u8; len];
        let total_resonance: usize = fragments.iter().map(|f| f.1).sum();

        // 1. PERFORM BIT-LEVEL WEIGHTED SYNTHESIS
        for i in 0..len {
            let mut bit_counts = [0usize; 8];
            
            for (payload, weight) in fragments {
                if i < payload.len() {
                    let byte = payload[i];
                    for bit_idx in 0..8 {
                        if (byte >> bit_idx) & 1 == 1 {
                            bit_counts[bit_idx] += weight;
                        }
                    }
                }
            }

            let mut composite_byte = 0u8;
            for bit_idx in 0..8 {
                if bit_counts[bit_idx] > total_resonance / 2 {
                    composite_byte |= 1 << bit_idx;
                }
            }
            composite[i] = composite_byte;
        }

        composite
    }
}

impl LogicSynthesizer {
    /// Legacy compatibility for fixed-width lattice nodes (896 bytes)
    pub fn synthesize(fragments: &Vec<([u8; 896], usize)>) -> [u8; 896] {
        let generic_fragments: Vec<(&[u8], usize)> = fragments.iter()
            .map(|(data, weight)| (data.as_slice(), *weight))
            .collect();
            
        let composite_vec = <Self as UniversalSynthesizer>::synthesize_generic(&generic_fragments);
        let mut composite = [0u8; 896];
        let copy_len = composite_vec.len().min(896);
        composite[..copy_len].copy_from_slice(&composite_vec[..copy_len]);
        composite
    }
}
