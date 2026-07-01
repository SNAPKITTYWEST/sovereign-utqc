//! # sovereign-prime-mask
//!
//! 64-bit prime mask for prime-gated indexing.
//! Bit k is set if the k-th prime in P_64 is attached.

use serde::{Deserialize, Serialize};

/// The first 64 primes.
pub const P_64: [u64; 64] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151,
    157, 163, 167, 173, 179, 181, 191, 193, 197, 199, 211, 223, 227, 229, 233,
    239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293, 307, 311,
];

/// 64-bit prime mask.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PrimeMask(pub u64);

impl PrimeMask {
    /// Empty mask.
    pub const EMPTY: Self = Self(0);

    /// Full mask.
    pub const FULL: Self = Self(u64::MAX);

    /// Bitwise AND.
    pub fn and(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    /// Bitwise OR.
    pub fn or(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    /// Bitwise XOR.
    pub fn xor(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    /// Bitwise NOT.
    pub fn not(self) -> Self {
        Self(!self.0)
    }

    /// Is bit k set?
    pub fn is_prime_set(self, k: usize) -> bool {
        if k >= 64 { return false; }
        (self.0 & (1 << k)) != 0
    }

    /// Set bit k.
    pub fn set(self, k: usize) -> Self {
        if k >= 64 { return self; }
        Self(self.0 | (1 << k))
    }

    /// Clear bit k.
    pub fn clear(self, k: usize) -> Self {
        if k >= 64 { return self; }
        Self(self.0 & !(1 << k))
    }

    /// Count set bits.
    pub fn count_ones(self) -> u32 {
        self.0.count_ones()
    }

    /// Get the k-th prime value.
    pub fn prime_value(k: usize) -> u64 {
        if k >= 64 { return 0; }
        P_64[k]
    }

    /// Product of set primes.
    pub fn product(self) -> u128 {
        let mut result: u128 = 1;
        for k in 0..64 {
            if self.is_prime_set(k) {
                result = result.wrapping_mul(P_64[k] as u128);
            }
        }
        result
    }

    /// Convert to u64.
    pub fn to_u64(self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_and_or_xor(a in 0u64..u64::MAX, b in 0u64..u64::MAX) {
            let ma = PrimeMask(a);
            let mb = PrimeMask(b);
            // a & b | a & ~b = a
            let result = ma.and(mb).or(ma.and(mb.not()));
            prop_assert_eq!(result, ma);
        }
    }
}
