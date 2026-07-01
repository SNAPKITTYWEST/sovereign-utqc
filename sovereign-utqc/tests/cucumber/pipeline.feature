Feature: sovereign-utqc pipeline
  As a quantum compiler
  I want to compile, verify, seal, and govern circuits
  So that every artifact is traceable and safe

  Scenario: Create a valid circuit
    Given a circuit with 2 qubits
    When I add a Hadamard gate on qubit 0
    And I add a CNOT gate with control 0 and target 1
    Then the circuit should have 2 gates
    And the circuit should validate

  Scenario: QFT compilation
    Given a circuit with 4 qubits
    When I compile a QFT circuit
    Then the circuit should have gates
    And the circuit should validate

  Scenario: Grover compilation
    Given a circuit with 3 qubits
    When I compile a Grover circuit with 1 solution
    Then the circuit should have gates
    And the circuit should validate

  Scenario: Linear resource check
    Given a circuit with 2 qubits
    When I add a Hadamard gate on qubit 0
    And I add a CNOT gate with control 0 and target 1
    Then the linear resource check should pass

  Scenario: Goldilocks field arithmetic
    Given a Goldilocks element 100
    When I add Goldilocks element 200
    Then the result should be Goldilocks element 300

  Scenario: WORM seal chain
    Given an empty WORM chain
    When I append seal "STEP_1" with payload "data1"
    And I append seal "STEP_2" with payload "data2"
    Then the chain should have 2 seals
    And the chain should verify

  Scenario: BDD equivalence check
    Given two identical circuits with 2 qubits and CNOT gate
    Then the BDD equivalence check should pass

  Scenario: Agent governance
    Given an agent governance system with 2 agents
    When I collect votes for artifact "test-artifact"
    Then the governance record should be approved

  Scenario: LaTeX export
    Given a circuit with 2 qubits
    When I add a Hadamard gate on qubit 0
    And I export the circuit to LaTeX
    Then the LaTeX document should contain "Circuit Equivalence"
