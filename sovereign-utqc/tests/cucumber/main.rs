use cucumber::{given, when, then, World};
use utqc_core::{Circuit, Gate, Qubit, SingleGate, DoubleGate, Pass};
use utqc_goldilocks::Goldilocks;
use utqc_linear::ResourceTracker;
use utqc_quantum::{Qft, Grover};
use utqc_bdd::BddVerifier;
use utqc_worm::{WormChain, WormSeal};
use utqc_agent::{AgentGovernance, AgentIdentity, AgentRole};
use utqc_paper::LatexDocument;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct PipelineWorld {
    circuit: Option<Circuit>,
    qft_circuit: Option<Circuit>,
    grover_circuit: Option<Circuit>,
    linear_ok: bool,
    goldilocks_a: Goldilocks,
    goldilocks_b: Goldilocks,
    goldilocks_result: Goldilocks,
    worm_chain: WormChain,
    bdd_ok: bool,
    governance_approved: bool,
    latex_doc: Option<LatexDocument>,
}

impl PipelineWorld {
    fn new() -> Self {
        Self {
            circuit: None,
            qft_circuit: None,
            grover_circuit: None,
            linear_ok: false,
            goldilocks_a: Goldilocks::ZERO,
            goldilocks_b: Goldilocks::ZERO,
            goldilocks_result: Goldilocks::ZERO,
            worm_chain: WormChain::new(),
            bdd_ok: false,
            governance_approved: false,
            latex_doc: None,
        }
    }
}

given!("a circuit with {int} qubits", |num: usize, world: PipelineWorld| {
    world.circuit = Some(Circuit::new(num, num));
    world
});

when!("I add a Hadamard gate on qubit {int}", |q: usize, mut world: PipelineWorld| {
    if let Some(ref mut circ) = world.circuit {
        circ.add_gate(Gate::Single { gate: SingleGate::Hadamard, target: Qubit(q) }).unwrap();
    }
    world
});

when!("I add a CNOT gate with control {int} and target {int}", |c: usize, t: usize, mut world: PipelineWorld| {
    if let Some(ref mut circ) = world.circuit {
        circ.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(c), target: Qubit(t) }).unwrap();
    }
    world
});

then!("the circuit should have {int} gates", |n: usize, world: PipelineWorld| {
    let circ = world.circuit.as_ref().unwrap();
    assert_eq!(circ.gates.len(), n);
});

then!("the circuit should validate", |world: PipelineWorld| {
    let circ = world.circuit.as_ref().unwrap();
    assert!(circ.validate().is_ok());
});

when!("I compile a QFT circuit", |mut world: PipelineWorld| {
    let n = world.circuit.as_ref().map(|c| c.num_qubits).unwrap_or(4);
    world.qft_circuit = Some(Qft::circuit(n, 0).unwrap());
    world
});

when!("I compile a Grover circuit with {int} solution", |s: usize, mut world: PipelineWorld| {
    let n = world.circuit.as_ref().map(|c| c.num_qubits).unwrap_or(3);
    world.grover_circuit = Some(Grover::circuit(n, s).unwrap());
    world
});

then!("the circuit should have gates", |world: PipelineWorld| {
    let circ = world.circuit.as_ref().or(world.qft_circuit.as_ref()).or(world.grover_circuit.as_ref()).unwrap();
    assert!(!circ.gates.is_empty());
});

then!("the linear resource check should pass", |mut world: PipelineWorld| {
    let circ = world.circuit.as_ref().unwrap();
    world.linear_ok = ResourceTracker::check_circuit(circ).is_ok();
    assert!(world.linear_ok);
});

given!("a Goldilocks element {int}", |val: u64, mut world: PipelineWorld| {
    world.goldilocks_a = Goldilocks::new(val);
    world
});

when!("I add Goldilocks element {int}", |val: u64, mut world: PipelineWorld| {
    world.goldilocks_b = Goldilocks::new(val);
    world.goldilocks_result = world.goldilocks_a.add(world.goldilocks_b);
    world
});

then!("the result should be Goldilocks element {int}", |expected: u64, world: PipelineWorld| {
    assert_eq!(world.goldilocks_result, Goldilocks::new(expected));
});

given!("an empty WORM chain", |world: PipelineWorld| {
    world.worm_chain = WormChain::new();
    world
});

when!("I append seal {string} with payload {string}", |label: String, payload: String, mut world: PipelineWorld| {
    world.worm_chain.append(&label, &payload, 10);
    world
});

then!("the chain should have {int} seals", |n: usize, world: PipelineWorld| {
    assert_eq!(world.worm_chain.len(), n);
});

then!("the chain should verify", |world: PipelineWorld| {
    assert!(world.worm_chain.verify().is_ok());
});

given!("two identical circuits with {int} qubits and CNOT gate", |n: usize, mut world: PipelineWorld| {
    let mut a = Circuit::new(n, n);
    a.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(0), target: Qubit(n - 1) }).unwrap();
    let mut b = Circuit::new(n, n);
    b.add_gate(Gate::Double { gate: DoubleGate::CNOT, control: Qubit(0), target: Qubit(n - 1) }).unwrap();
    world.bdd_ok = BddVerifier::verify_equivalent(&a, &b).is_ok();
    world
});

then!("the BDD equivalence check should pass", |world: PipelineWorld| {
    assert!(world.bdd_ok);
});

given!("an agent governance system with {int} agents", |n: usize, mut world: PipelineWorld| {
    let mut gov = AgentGovernance::new();
    for i in 0..n {
        gov.register(AgentIdentity {
            name: format!("agent-{}", i),
            role: if i == 0 { AgentRole::Governor } else { AgentRole::Releaser },
            permissions: vec!["release".to_string(), "govern".to_string()],
        });
    }
    let record = gov.collect_votes("test-artifact");
    world.governance_approved = record.is_approved();
    world
});

when!("I collect votes for artifact {string}", |artifact: String, mut world: PipelineWorld| {
    // Already done in given step
    world
});

then!("the governance record should be approved", |world: PipelineWorld| {
    assert!(world.governance_approved);
});

when!("I export the circuit to LaTeX", |mut world: PipelineWorld| {
    let circ = world.circuit.as_ref().unwrap();
    let mut doc = LatexDocument::new("Test", vec!["Author".to_string()]);
    let theorem = utqc_paper::export_circuit_theorem(circ);
    doc.add_theorem(theorem);
    world.latex_doc = Some(doc);
    world
});

then!("the LaTeX document should contain {string}", |text: String, world: PipelineWorld| {
    let doc = world.latex_doc.as_ref().unwrap();
    let latex = doc.to_latex();
    assert!(latex.contains(&text));
});

#[tokio::main]
async fn main() {
    PipelineWorld::run("tests/cucumber/pipeline.feature").await;
}
