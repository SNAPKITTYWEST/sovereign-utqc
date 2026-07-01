# sovereign-utqc

**Non-recursive universal trusted quantum compiler.**

```
UTQC-NONREC-001
No recursion. No vibes. Every circuit compiles, verifies, seals, and governs.
Ω←⌹∧○∧◇∧△∧⬡
```

## What It Is

sovereign-utqc is a non-recursive universal trusted quantum compiler. It combines:

- **Real quantum algorithms** — QFT, Grover, Shor, QPE
- **Linear resource safety** — qubits consumed exactly once
- **Goldilocks arithmetic** — p = 2^64 - 2^32 + 1 (PLONK field)
- **BDD verification** — circuit equivalence checking
- **WORM-sealed artifacts** — immutable audit chain
- **Agent-governed release** — multi-party approval

## Architecture

```
Circuit DSL
→ Linear Resource Check
→ Goldilocks Field Normalize
→ Quantum Algorithm Compile
→ BDD Equivalence Verify
→ WORM Seal
→ Agent Governance Vote
→ LaTeX Proof Export
```

Non-recursive execution rule:

```rust
pub trait Pass {
    type Input;
    type Output;
    fn name(&self) -> &'static str;
    fn run(&self, input: Self::Input) -> Result<Self::Output, UtqcError>;
}
```

## Crates

| Crate | Purpose |
|-------|---------|
| utqc-core | Circuit IR — Gate, Qubit, Circuit, Measurement |
| utqc-goldilocks | Goldilocks field arithmetic |
| utqc-linear | Linear type resource rules |
| utqc-quantum | QFT, Grover, Shor, QPE |
| utqc-bdd | BDD equivalence verification |
| utqc-coxeter | Coxeter group and octonion math |
| utqc-worm | WORM-sealed artifact chains |
| utqc-agent | Agent governance hooks |
| utqc-paper | LaTeX theorem export |

## Usage

```rust
use utqc_core::{Circuit, Gate, Qubit, SingleGate, Pass};
use utqc_quantum::Qft;
use utqc_worm::WormSeal;

// Build a QFT circuit
let circuit = Qft::circuit(4, 0)?;

// Seal it
let seal = WormSeal::seal("QFT_4", &format!("{:?}", circuit), circuit.depth() as u64);
```

## Test

```bash
cargo test --workspace
```

## License

MIT OR Apache-2.0

---

ERRANT_GENESIS_001 — Forth is the metal. Prolog is the law. Linear types are the vow. WORM is the memory. Ω
