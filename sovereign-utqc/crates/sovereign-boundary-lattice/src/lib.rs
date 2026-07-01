//! # sovereign-boundary-lattice
//!
//! Boundary Lattice G = P x B, |G| = 12,288.
//! URef subgroup with 11 commuting involutions.
//! 6 anchors, 6 orbits of size 2048.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Lattice error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LatticeError {
    /// Index out of bounds.
    #[error("index {0} out of bounds (max 12287)")]
    IndexOutOfBounds(usize),
}

/// Element in the Boundary Lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LatticeElement {
    /// Prime index (0..47).
    pub p: u8,
    /// Byte index (0..255).
    pub b: u8,
}

impl LatticeElement {
    /// Total elements.
    pub const SIZE: usize = 12_288;

    /// Create new element.
    pub fn new(p: u8, b: u8) -> Self {
        Self { p, b }
    }

    /// Convert to flat index.
    pub fn to_index(self) -> usize {
        (self.p as usize * 256) + self.b as usize
    }

    /// Convert from flat index.
    pub fn from_index(index: usize) -> Self {
        Self {
            p: (index / 256) as u8,
            b: (index % 256) as u8,
        }
    }
}

/// URef subgroup: 11 commuting involutions.
#[derive(Debug, Clone)]
pub struct URef {
    /// Generator XOR masks.
    pub generators: [u64; 11],
}

impl URef {
    /// Canonical generators (powers of 2).
    pub fn canonical() -> Self {
        Self { generators: [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] }
    }

    /// Apply involution mask to element.
    pub fn apply(&self, element: LatticeElement, mask: u16) -> LatticeElement {
        let mut index = element.to_index();
        for i in 0..11 {
            if (mask & (1 << i)) != 0 {
                index ^= self.generators[i] as usize;
            }
        }
        LatticeElement::from_index(index % LatticeElement::SIZE)
    }
}

/// The 6 anchors.
pub const ANCHORS: [LatticeElement; 6] = [
    LatticeElement { p: 0, b: 0 },
    LatticeElement { p: 8, b: 0 },
    LatticeElement { p: 16, b: 0 },
    LatticeElement { p: 24, b: 0 },
    LatticeElement { p: 32, b: 0 },
    LatticeElement { p: 40, b: 0 },
];

/// Orbit certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitCertificate {
    /// Anchor element.
    pub anchor: LatticeElement,
    /// Orbit size.
    pub size: usize,
}

/// Lattice certificate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatticeCertificate {
    /// Total elements found.
    pub total_elements: usize,
    /// Orbit sizes per anchor.
    pub orbit_sizes: [usize; 6],
    /// Is the URef action free?
    pub is_free_action: bool,
}

impl LatticeCertificate {
    /// Verify the lattice structure.
    pub fn verify() -> Self {
        let u_ref = URef::canonical();
        let mut orbit_sizes = [0; 6];
        let mut seen = std::collections::HashSet::new();
        let mut is_free_action = true;

        for (i, &anchor) in ANCHORS.iter().enumerate() {
            let mut orbit_count = 0;
            for mask in 0..2048u16 {
                let transformed = u_ref.apply(anchor, mask);
                if !seen.insert(transformed) {
                    is_free_action = false;
                }
                orbit_count += 1;
            }
            orbit_sizes[i] = orbit_count;
        }

        Self { total_elements: seen.len(), orbit_sizes, is_free_action }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_tiling() {
        let cert = LatticeCertificate::verify();
        assert_eq!(cert.total_elements, 12_288);
        assert!(cert.is_free_action);
        for &size in &cert.orbit_sizes {
            assert_eq!(size, 2048);
        }
    }

    #[test]
    fn test_roundtrip() {
        for i in 0..12_288 {
            let elem = LatticeElement::from_index(i);
            assert_eq!(elem.to_index(), i);
        }
    }
}
