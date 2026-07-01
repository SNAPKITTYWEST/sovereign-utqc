//! # sovereign-resonance-word
//!
//! Resonance Word: 64-bit word with 7-bit class (0..95) + 57-bit payload.
//! Maps lattice elements to field elements.

use sovereign_field_simd::SimdGoldilocks;
use serde::{Deserialize, Serialize};

/// A 64-bit resonance word.
/// bits 0..6 = class (0..95), bits 7..63 = 57-bit payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResonanceWord(pub u64);

impl ResonanceWord {
    /// Neutral element.
    pub const NEUTRAL: Self = Self(0);

    /// Pack class and payload into a word.
    pub fn pack(class: u8, payload: u64) -> Self {
        assert!(class < 96, "class must be < 96");
        assert!(payload < (1 << 57), "payload must fit in 57 bits");
        Self((payload << 7) | (class as u64))
    }

    /// Unpack into (class, payload).
    pub fn unpack(self) -> (u8, u64) {
        let class = (self.0 & 0x7F) as u8;
        let payload = self.0 >> 7;
        (class, payload)
    }

    /// Get class.
    pub fn class(self) -> u8 {
        (self.0 & 0x7F) as u8
    }

    /// Get payload.
    pub fn payload(self) -> u64 {
        self.0 >> 7
    }

    /// Convert to Goldilocks field element.
    pub fn to_field(self) -> SimdGoldilocks {
        SimdGoldilocks::from_canonical(self.0)
    }

    /// Create from field element.
    pub fn from_field(f: SimdGoldilocks) -> Self {
        Self(f.to_canonical())
    }
}

/// Resonance class R96 (0..95).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct R96(pub u8);

impl R96 {
    /// Create from lattice (p, b).
    pub fn from_lattice(p: u8, b: u8) -> Self {
        Self((p * 2) + (b % 2))
    }

    /// Get class index.
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_roundtrip(class in 0u8..96, payload in 0u64..(1u64 << 57)) {
            let w = ResonanceWord::pack(class, payload);
            let (c, p) = w.unpack();
            prop_assert_eq!(c, class);
            prop_assert_eq!(p, payload);
        }
    }

    #[test]
    fn test_neutral() {
        let (c, p) = ResonanceWord::NEUTRAL.unpack();
        assert_eq!(c, 0);
        assert_eq!(p, 0);
    }

    #[test]
    fn test_r96_distribution() {
        let mut counts = [0u32; 96];
        for p in 0..48u8 {
            for b in 0u8..=255u8 {
                let class = R96::from_lattice(p, b);
                counts[class.0 as usize] += 1;
            }
        }
        // Each class should have 128 elements (48*256/96)
        for &c in &counts {
            assert_eq!(c, 128);
        }
    }
}
