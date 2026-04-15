use serde::{Serialize, Deserialize};

/// [HDC_0x100K]: 102,400-BIT HOLOGRAPHIC VECTOR
/// AXIOM: Concepts are distributed bit-patterns. Binding is XOR.
#[repr(align(64))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hypervector {
    pub data: Vec<u64>,
}

impl Hypervector {
    pub fn new(data: Vec<u64>) -> Self {
        assert_eq!(data.len(), 1600, "Hypervector must be 102,400 bits (1600 words)");
        Self { data }
    }

    /// Generate a random hypervector (Zero-centered distribution of bits)
    pub fn random() -> Self {
        let mut data = Vec::with_capacity(1600);
        for _ in 0..1600 {
            data.push(rand::random::<u64>());
        }
        Self { data }
    }

    /// [BINDING]: XOR operation (Concept A * Concept B = Concept C)
    pub fn bind(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(1600);
        for i in 0..1600 {
            result.push(self.data[i] ^ other.data[i]);
        }
        Self { data: result }
    }

    /// [PERMUTATION]: Cyclic shift (Concept sequence)
    pub fn rotate(&self, shift: usize) -> Self {
        const BITS: usize = 102400;
        let s = shift % BITS;
        if s == 0 { return self.clone(); }

        let mut result = vec![0u64; 1600];
        let word_shift = s / 64;
        let bit_shift = s % 64;

        for i in 0..1600 {
            let src_idx = (i + 1600 - word_shift) % 1600;
            let next_src_idx = (i + 1600 - word_shift - 1) % 1600;
            
            let val = self.data[src_idx] << bit_shift;
            let carry = if bit_shift > 0 { self.data[next_src_idx] >> (64 - bit_shift) } else { 0 };
            result[i] = val | carry;
        }
        Self { data: result }
    }

    /// [SIMILARITY]: Hamming Distance (0.0 to 1.0)
    pub fn similarity(&self, other: &Self) -> f64 {
        let mut total_popcount = 0;
        for i in 0..1600 {
            total_popcount += (self.data[i] ^ other.data[i]).count_ones();
        }
        1.0 - (total_popcount as f64 / 102400.0)
    }

    /// [SIMILARITY_FAST]: 8-wide unrolled Hamming -- auto-vectorized to AVX2 on x86.
    /// Processes 512 bits per loop iteration. ~4-8x throughput over scalar.
    #[inline]
    pub fn similarity_fast(&self, other: &Self) -> f64 {
        let a = &self.data;
        let b = &other.data;
        let mut pc = 0u32;
        let mut i = 0usize;
        while i + 8 <= 1600 {
            pc += (a[i  ] ^ b[i  ]).count_ones();
            pc += (a[i+1] ^ b[i+1]).count_ones();
            pc += (a[i+2] ^ b[i+2]).count_ones();
            pc += (a[i+3] ^ b[i+3]).count_ones();
            pc += (a[i+4] ^ b[i+4]).count_ones();
            pc += (a[i+5] ^ b[i+5]).count_ones();
            pc += (a[i+6] ^ b[i+6]).count_ones();
            pc += (a[i+7] ^ b[i+7]).count_ones();
            i += 8;
        }
        while i < 1600 { pc += (a[i] ^ b[i]).count_ones(); i += 1; }
        1.0 - (pc as f64 / 102400.0)
    }

    /// Fast threshold check -- short-circuits as soon as similarity is impossible.
    /// O(early_exit) instead of O(1600). Best case: 1 iteration.
    #[inline]
    pub fn similarity_threshold(&self, other: &Self, threshold: f64) -> bool {
        let max_mm = ((1.0 - threshold) * 102400.0) as u32;
        let mut pc = 0u32;
        for i in 0..1600 {
            pc += (self.data[i] ^ other.data[i]).count_ones();
            if pc > max_mm { return false; }
        }
        true
    }
}

/// [BUNDLING]: Majority-vote addition of many hypervectors
pub struct Bundle {
    counts: Vec<i32>,
}

impl Bundle {
    pub fn new() -> Self {
        Self { counts: vec![0; 102400] }
    }

    pub fn add(&mut self, hv: &Hypervector) {
        for i in 0..102400 {
            let word_idx = i / 64;
            let bit_idx = i % 64;
            let bit = (hv.data[word_idx] >> bit_idx) & 1;
            if bit == 1 {
                self.counts[i] += 1;
            } else {
                self.counts[i] -= 1;
            }
        }
    }

    pub fn finalize(&self) -> Hypervector {
        let mut data = vec![0u64; 1600];
        for i in 0..102400 {
            if self.counts[i] > 0 {
                let word_idx = i / 64;
                let bit_idx = i % 64;
                data[word_idx] |= 1 << bit_idx;
            }
        }
        Hypervector { data }
    }
}
