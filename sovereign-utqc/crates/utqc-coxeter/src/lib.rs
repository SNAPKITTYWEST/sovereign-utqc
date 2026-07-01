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
        let n = self.rank;
        let o = &self.orders;
        // Returns true iff every (i,j) pair has exactly the expected order:
        // pairs listed in `edges` get their specified weight, all others get 2.
        let clean = |edges: &[(usize, usize, usize)]| -> bool {
            for i in 0..n {
                for j in (i + 1)..n {
                    let expected = edges
                        .iter()
                        .find(|&&(a, b, _)| (a == i && b == j) || (a == j && b == i))
                        .map_or(2, |&(_, _, w)| w);
                    if o[i][j] != expected {
                        return false;
                    }
                }
            }
            true
        };
        match n {
            1 => "A1",
            2 => match o[0][1] {
                2 => "A1 x A1",
                3 => "A2",
                4 => "B2",
                6 => "G2",
                _ => "Non-crystallographic",
            },
            3 => {
                if clean(&[(0, 1, 3), (1, 2, 3)]) { "A3" }
                else if clean(&[(0, 1, 4), (1, 2, 3)]) { "B3" }
                else if clean(&[(0, 1, 3), (1, 2, 4)]) { "C3" }
                else { "Other" }
            }
            4 => {
                if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3)]) { "A4" }
                else if clean(&[(0, 1, 4), (1, 2, 3), (2, 3, 3)]) { "B4" }
                else if clean(&[(0, 1, 3), (1, 2, 4), (2, 3, 3)]) { "F4" }
                // D4: star at node 2 — three branches (0,1,3) all connected to 2
                else if clean(&[(0, 2, 3), (1, 2, 3), (2, 3, 3)]) { "D4" }
                else { "Other" }
            }
            5 => {
                if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3)]) { "A5" }
                else if clean(&[(0, 1, 4), (1, 2, 3), (2, 3, 3), (3, 4, 3)]) { "B5" }
                // D5: fork at node 2 — chain 0-1-2, branches 2-3 and 2-4
                else if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (2, 4, 3)]) { "D5" }
                else { "Other" }
            }
            6 => {
                if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3)]) { "A6" }
                else if clean(&[(0, 1, 4), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3)]) { "B6" }
                // D6: fork at node 3 — chain 0-1-2-3, branches 3-4 and 3-5
                else if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (3, 5, 3)]) { "D6" }
                // E6: chain 0-1-2-3-4, branch at node 2 to node 5
                else if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (2, 5, 3)]) { "E6" }
                else { "Other" }
            }
            7 => {
                if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3), (5, 6, 3)]) { "A7" }
                else if clean(&[(0, 1, 4), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3), (5, 6, 3)]) { "B7" }
                // D7: fork at node 4 — chain 0-1-2-3-4, branches 4-5 and 4-6
                else if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3), (4, 6, 3)]) { "D7" }
                // E7: chain 0-1-2-3-4-5, branch at node 3 to node 6
                else if clean(&[(0, 1, 3), (1, 2, 3), (2, 3, 3), (3, 4, 3), (4, 5, 3), (3, 6, 3)]) { "E7" }
                else { "Other" }
            }
            8 => {
                if clean(&[(0,1,3),(1,2,3),(2,3,3),(3,4,3),(4,5,3),(5,6,3),(6,7,3)]) { "A8" }
                else if clean(&[(0,1,4),(1,2,3),(2,3,3),(3,4,3),(4,5,3),(5,6,3),(6,7,3)]) { "B8" }
                // D8: fork at node 5 — chain 0-1-2-3-4-5, branches 5-6 and 5-7
                else if clean(&[(0,1,3),(1,2,3),(2,3,3),(3,4,3),(4,5,3),(5,6,3),(5,7,3)]) { "D8" }
                // E8: chain 0-1-2-3-4-5-6, branch at node 2 to node 7
                else if clean(&[(0,1,3),(1,2,3),(2,3,3),(3,4,3),(4,5,3),(5,6,3),(2,7,3)]) { "E8" }
                else { "Other" }
            }
            _ => {
                let an: Vec<(usize, usize, usize)> = (0..n - 1).map(|i| (i, i + 1, 3)).collect();
                if clean(&an) { return "A_n"; }
                let bn: Vec<(usize, usize, usize)> = core::iter::once((0, 1, 4))
                    .chain((1..n - 1).map(|i| (i, i + 1, 3)))
                    .collect();
                if clean(&bn) { return "B_n"; }
                if n >= 4 {
                    // D_n: chain 0..n-3, then fork from n-3 to n-2 and n-1
                    let dn: Vec<(usize, usize, usize)> = (0..n - 3)
                        .map(|i| (i, i + 1, 3))
                        .chain(core::iter::once((n - 3, n - 2, 3)))
                        .chain(core::iter::once((n - 3, n - 1, 3)))
                        .collect();
                    if clean(&dn) { return "D_n"; }
                }
                "General"
            }
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

    /// Octonion multiplication (full Cayley-Dickson, Fano-plane table).
    pub fn mul(self, other: Self) -> Self {
        let x = self.components;
        let y = other.components;
        let result = [
            x[0]*y[0] - x[1]*y[1] - x[2]*y[2] - x[3]*y[3]
                       - x[4]*y[4] - x[5]*y[5] - x[6]*y[6] - x[7]*y[7],
            x[0]*y[1] + x[1]*y[0] + x[2]*y[3] - x[3]*y[2]
                       + x[5]*y[4] - x[4]*y[5] + x[7]*y[6] - x[6]*y[7],
            x[0]*y[2] - x[1]*y[3] + x[2]*y[0] + x[3]*y[1]
                       + x[6]*y[4] - x[7]*y[5] - x[4]*y[6] + x[5]*y[7],
            x[0]*y[3] + x[1]*y[2] - x[2]*y[1] + x[3]*y[0]
                       + x[7]*y[4] + x[6]*y[5] - x[5]*y[6] - x[4]*y[7],
            x[0]*y[4] - x[1]*y[5] - x[2]*y[6] - x[3]*y[7]
                       + x[4]*y[0] + x[5]*y[1] + x[6]*y[2] + x[7]*y[3],
            x[0]*y[5] + x[1]*y[4] - x[2]*y[7] + x[3]*y[6]
                       - x[4]*y[1] + x[5]*y[0] - x[6]*y[3] + x[7]*y[2],
            x[0]*y[6] + x[1]*y[7] + x[2]*y[4] - x[3]*y[5]
                       - x[4]*y[2] + x[5]*y[3] + x[6]*y[0] - x[7]*y[1],
            x[0]*y[7] - x[1]*y[6] + x[2]*y[5] + x[3]*y[4]
                       - x[4]*y[3] - x[5]*y[2] + x[6]*y[1] + x[7]*y[0],
        ];
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
        // A3: linear chain 0-1-2; m[0][1]=3, m[1][2]=3, m[0][2]=2 (non-adjacent)
        let mut g = CoxeterGroup::new(3);
        g.set_order(0, 1, 3).unwrap();
        g.set_order(1, 2, 3).unwrap();
        // m[0][2] stays at default 2 — nodes 0 and 2 are non-adjacent in A3
        assert!(g.is_finite());
        assert_eq!(g.weyl_type(), "A3");

        // B3: same chain but first edge has order 4
        let mut b3 = CoxeterGroup::new(3);
        b3.set_order(0, 1, 4).unwrap();
        b3.set_order(1, 2, 3).unwrap();
        assert_eq!(b3.weyl_type(), "B3");

        // E6: chain 0-1-2-3-4 with branch at node 2 to node 5
        let mut e6 = CoxeterGroup::new(6);
        e6.set_order(0, 1, 3).unwrap();
        e6.set_order(1, 2, 3).unwrap();
        e6.set_order(2, 3, 3).unwrap();
        e6.set_order(3, 4, 3).unwrap();
        e6.set_order(2, 5, 3).unwrap();
        assert_eq!(e6.weyl_type(), "E6");
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
