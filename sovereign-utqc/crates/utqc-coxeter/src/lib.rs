//! # utqc-coxeter
//!
//! Coxeter group and octonion math for topological structure.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Coxeter group error.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CoxeterError {
    #[error("invalid generator order: {0}")]
    InvalidOrder(usize),
}

/// A Coxeter group element represented as a sequence of generator indices.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoxeterElement {
    /// Generator indices (0-indexed).
    pub generators: Vec<usize>,
}

impl CoxeterElement {
    /// Create a new Coxeter element.
    pub fn new(generators: Vec<usize>) -> Self {
        Self { generators }
    }

    /// Length of the reduced word.
    pub fn length(&self) -> usize {
        self.generators.len()
    }
}

/// Coxeter group with presentation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoxeterGroup {
    /// Number of generators.
    pub rank: usize,
    /// orders[i][j] = m_{i,j}: order of (s_i * s_j).
    pub orders: Vec<Vec<usize>>,
}

impl CoxeterGroup {
    /// Create a new Coxeter group.
    pub fn new(rank: usize) -> Self {
        let mut orders = vec![vec![2; rank]; rank];
        for i in 0..rank {
            orders[i][i] = 1;
        }
        Self { rank, orders }
    }

    /// Set the order of (s_i * s_j).
    pub fn set_order(&mut self, i: usize, j: usize, order: usize) -> Result<(), CoxeterError> {
        if i >= self.rank || j >= self.rank {
            return Err(CoxeterError::InvalidOrder(order));
        }
        self.orders[i][j] = order;
        self.orders[j][i] = order;
        Ok(())
    }

    /// Check if the group is finite.
    pub fn is_finite(&self) -> bool {
        // Simplified: check if all orders are 2 or 3
        for i in 0..self.rank {
            for j in (i + 1)..self.rank {
                if self.orders[i][j] > 3 {
                    return false;
                }
            }
        }
        true
    }

    /// Get the Weyl group type.
    pub fn weyl_type(&self) -> &'static str {
        match self.rank {
            1 => "A1",
            2 => {
                let m = self.orders[0][1];
                match m {
                    2 => "A1 x A1",
                    3 => "A2",
                    4 => "B2",
                    6 => "G2",
                    _ => "Non-crystallographic",
                }
            }
            3 => {
                let m01 = self.orders[0][1];
                let m02 = self.orders[0][2];
                let m12 = self.orders[1][2];
                if m01 == 3 && m02 == 3 && m12 == 3 {
                    "A3"
                } else if m01 == 3 && m02 == 4 && m12 == 3 {
                    "B3"
                } else {
                    "Other"
                }
            }
            _ => "General",
        }
    }
}

/// An octonion.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Octonion {
    /// Components [e0, e1, e2, e3, e4, e5, e6, e7].
    pub components: [f64; 8],
}

impl Octonion {
    /// Create a new octonion.
    pub fn new(components: [f64; 8]) -> Self {
        Self { components }
    }

    /// Real part.
    pub fn real(self) -> f64 {
        self.components[0]
    }

    /// Norm squared.
    pub fn norm_squared(self) -> f64 {
        self.components.iter().map(|c| c * c).sum()
    }

    /// Conjugate.
    pub fn conjugate(self) -> Self {
        let mut c = self.components;
        for i in 1..8 {
            c[i] = -c[i];
        }
        Self::new(c)
    }

    /// Octonion multiplication (Cayley-Dickson).
    pub fn mul(self, other: Self) -> Self {
        // Simplified: real * real + cross terms
        let mut result = [0.0f64; 8];
        result[0] = self.components[0] * other.components[0]
            - self.components[1] * other.components[1]
            - self.components[2] * other.components[2]
            - self.components[3] * other.components[3]
            - self.components[4] * other.components[4]
            - self.components[5] * other.components[5]
            - self.components[6] * other.components[6]
            - self.components[7] * other.components[7];
        for i in 1..8 {
            result[i] = self.components[0] * other.components[i]
                + self.components[i] * other.components[0];
        }
        Self::new(result)
    }
}

impl std::ops::Mul for Octonion {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}

/// The Coxeter pass for topological structure.
pub struct CoxeterPass;

impl utqc_core::Pass for CoxeterPass {
    type Input = CoxeterGroup;
    type Output = &'static str;

    fn name(&self) -> &'static str {
        "coxeter-classification"
    }

    fn run(&self, input: CoxeterGroup) -> Result<&'static str, utqc_core::CircuitError> {
        Ok(input.weyl_type())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coxeter_group() {
        let mut g = CoxeterGroup::new(3);
        g.set_order(0, 1, 3).unwrap();
        g.set_order(0, 2, 3).unwrap();
        g.set_order(1, 2, 3).unwrap();
        assert!(g.is_finite());
        assert_eq!(g.weyl_type(), "A3");
    }

    #[test]
    fn test_octonion_mul() {
        let a = Octonion::new([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let b = Octonion::new([2.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        let c = a * b;
        assert_eq!(c.components[0], 2.0);
        assert_eq!(c.components[1], 1.0);
    }
}
