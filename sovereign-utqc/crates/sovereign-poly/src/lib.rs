//! # sovereign-poly
//!
//! Polynomial arithmetic over the Goldilocks field.

use sovereign_field_simd::SimdGoldilocks;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Polynomial error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum PolyError {
    /// Degree mismatch.
    #[error("degree mismatch")]
    DegreeMismatch,
}

/// Polynomial represented as coefficient vector.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Polynomial {
    /// Coefficients [a0, a1, a2, ...] for a0 + a1*x + a2*x^2 + ...
    pub coeffs: Vec<SimdGoldilocks>,
}

impl Polynomial {
    /// Create a zero polynomial.
    pub fn zero() -> Self {
        Self { coeffs: vec![SimdGoldilocks::ZERO] }
    }

    /// Create from coefficients.
    pub fn new(coeffs: Vec<SimdGoldilocks>) -> Self {
        Self { coeffs }
    }

    /// Degree of the polynomial.
    pub fn degree(&self) -> usize {
        self.coeffs.len().saturating_sub(1)
    }

    /// Evaluate at x.
    pub fn eval(&self, x: SimdGoldilocks) -> SimdGoldilocks {
        let mut result = SimdGoldilocks::ZERO;
        for (i, c) in self.coeffs.iter().enumerate() {
            result = result.add(c.mul(x.pow(i as u64)));
        }
        result
    }

    /// Polynomial addition.
    pub fn add(&self, other: &Self) -> Self {
        let len = self.coeffs.len().max(other.coeffs.len());
        let mut coeffs = Vec::with_capacity(len);
        for i in 0..len {
            let a = self.coeffs.get(i).copied().unwrap_or(SimdGoldilocks::ZERO);
            let b = other.coeffs.get(i).copied().unwrap_or(SimdGoldilocks::ZERO);
            coeffs.push(a.add(b));
        }
        Self { coeffs }
    }

    /// Polynomial multiplication.
    pub fn mul(&self, other: &Self) -> Self {
        let len = self.coeffs.len() + other.coeffs.len() - 1;
        let mut coeffs = vec![SimdGoldilocks::ZERO; len];
        for (i, a) in self.coeffs.iter().enumerate() {
            for (j, b) in other.coeffs.iter().enumerate() {
                coeffs[i + j] = coeffs[i + j].add(a.mul(*b));
            }
        }
        Self { coeffs }
    }

    /// Polynomial derivative.
    pub fn derivative(&self) -> Self {
        if self.coeffs.len() <= 1 {
            return Self::zero();
        }
        let coeffs: Vec<SimdGoldilocks> = self.coeffs[1..].iter()
            .enumerate()
            .map(|(i, c)| c.mul(SimdGoldilocks::new((i + 1) as u64)))
            .collect();
        Self { coeffs }
    }

    /// Roots via evaluation on a domain.
    pub fn find_roots(&self, domain: &[SimdGoldilocks]) -> Vec<SimdGoldilocks> {
        domain.iter().filter(|&&x| self.eval(x).is_zero()).copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_eval() {
        // f(x) = 2 + 3x
        let p = Polynomial::new(vec![
            SimdGoldilocks::new(2),
            SimdGoldilocks::new(3),
        ]);
        assert_eq!(p.eval(SimdGoldilocks::new(0)), SimdGoldilocks::new(2));
        assert_eq!(p.eval(SimdGoldilocks::new(1)), SimdGoldilocks::new(5));
        assert_eq!(p.eval(SimdGoldilocks::new(2)), SimdGoldilocks::new(8));
    }

    #[test]
    fn test_poly_mul() {
        // (1 + x)(1 + x) = 1 + 2x + x^2
        let a = Polynomial::new(vec![SimdGoldilocks::new(1), SimdGoldilocks::new(1)]);
        let b = Polynomial::new(vec![SimdGoldilocks::new(1), SimdGoldilocks::new(1)]);
        let c = a.mul(&b);
        assert_eq!(c.coeffs.len(), 3);
        assert_eq!(c.coeffs[0], SimdGoldilocks::ONE);
        assert_eq!(c.coeffs[1], SimdGoldilocks::new(2));
        assert_eq!(c.coeffs[2], SimdGoldilocks::ONE);
    }

    #[test]
    fn test_poly_derivative() {
        // f(x) = 3 + 2x + x^2
        let p = Polynomial::new(vec![
            SimdGoldilocks::new(3),
            SimdGoldilocks::new(2),
            SimdGoldilocks::ONE,
        ]);
        let d = p.derivative(); // f'(x) = 2 + 2x
        assert_eq!(d.coeffs[0], SimdGoldilocks::new(2));
        assert_eq!(d.coeffs[1], SimdGoldilocks::new(2));
    }
}
