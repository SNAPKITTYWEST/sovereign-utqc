//! # utqc-goldilocks
//!
//! Goldilocks field arithmetic (p = 2^64 - 2^32 + 1).
//! Used in PLONK and other ZK-proof systems.

use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Goldilocks field error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GoldilocksError {
    /// Division by zero.
    #[error("division by zero")]
    DivisionByZero,
}

/// Goldilocks field element (mod p = 2^64 - 2^32 + 1).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Goldilocks(pub u64);

impl Goldilocks {
    /// The Goldilocks prime.
    pub const P: u64 = 18_446_744_069_414_584_321;
    /// Zero element.
    pub const ZERO: Self = Self(0);
    /// One element.
    pub const ONE: Self = Self(1);

    /// Create a new element, reducing mod p.
    pub fn new(val: u64) -> Self {
        Self(Self::reduce(val))
    }

    /// Reduce mod p.
    fn reduce(val: u64) -> u64 {
        if val >= Self::P {
            val % Self::P
        } else {
            val
        }
    }

    /// Addition.
    pub fn add(self, other: Self) -> Self {
        let (s, overflow) = self.0.overflowing_add(other.0);
        let mut result = s;
        if overflow || result >= Self::P {
            result = result.wrapping_sub(Self::P);
        }
        Self(result)
    }

    /// Subtraction.
    pub fn sub(self, other: Self) -> Self {
        let (s, overflow) = self.0.overflowing_sub(other.0);
        if overflow {
            Self(s.wrapping_add(Self::P))
        } else {
            Self(s)
        }
    }

    /// Multiplication.
    pub fn mul(self, other: Self) -> Self {
        let result = (self.0 as u128) * (other.0 as u128);
        Self((result % Self::P as u128) as u64)
    }

    /// Power.
    pub fn pow(self, exp: u64) -> Self {
        let mut result = Self::ONE;
        let mut base = self;
        let mut e = exp;
        while e > 0 {
            if e & 1 == 1 {
                result = result.mul(base);
            }
            base = base.mul(base);
            e >>= 1;
        }
        result
    }

    /// Multiplicative inverse (Fermat's little theorem with square-and-multiply).
    pub fn inv(self) -> Result<Self, GoldilocksError> {
        if self.0 == 0 {
            return Err(GoldilocksError::DivisionByZero);
        }
        // a^(P-2) mod P = a^(-1) mod P (Fermat's little theorem)
        // Use square-and-multiply with u128 arithmetic
        let base = self.0 as u128;
        let p = Self::P as u128;
        let mut result: u128 = 1;
        let mut b = base;
        let mut exp = Self::P - 2; // u64

        while exp > 0 {
            if exp & 1 == 1 {
                result = result.wrapping_mul(b) % p;
            }
            b = b.wrapping_mul(b) % p;
            exp >>= 1;
        }

        Ok(Self(result as u64))
    }

    /// Division.
    pub fn div(self, other: Self) -> Result<Self, GoldilocksError> {
        Ok(self.mul(other.inv()?))
    }

    /// Is zero?
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for Goldilocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GF({})", self.0)
    }
}

impl fmt::Display for Goldilocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::ops::Add for Goldilocks {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}

impl std::ops::Sub for Goldilocks {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.sub(rhs)
    }
}

impl std::ops::Mul for Goldilocks {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let a = Goldilocks::new(100);
        let b = Goldilocks::new(200);
        assert_eq!(a.add(b), Goldilocks(300));
    }

    #[test]
    fn test_mul() {
        let a = Goldilocks::new(123);
        let b = Goldilocks::new(456);
        assert_eq!(a.mul(b), Goldilocks(123 * 456));
    }

    #[test]
    fn test_inv() {
        let a = Goldilocks::new(42);
        let a_inv = a.inv().unwrap();
        let product = a.mul(a_inv);
        eprintln!("a = {:?}", a);
        eprintln!("a_inv = {:?}", a_inv);
        eprintln!("product = {:?}", product);
        assert_eq!(product, Goldilocks::ONE);
    }

    #[test]
    fn test_sub() {
        let a = Goldilocks::new(100);
        let b = Goldilocks::new(200);
        let c = a.sub(b);
        // 100 - 200 mod p = p - 100
        assert_eq!(c, Goldilocks(Goldilocks::P - 100));
    }
}
