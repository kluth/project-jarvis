# ADR 004: Entropy Budgeting & Quantum-Probabilistic Streams

## Status
Proposed

## Context
Deterministic Big-O is insufficient for battery-constrained MCUs and non-deterministic AI decision logic.

## Decision
JARVIS will implement **Entropy Budgeting** and **Probabilistic Branching**.

### 1. Entropy Budgeting (`budget`)
Functions can declare an energy/compute budget.
Example: `budget 500uJ { ... }`
The compiler estimates instruction-level energy consumption based on the hardware profile and fails if the static path exceeds the budget.

### 2. Probabilistic Streams (`prob`)
Instead of `if/else`, JARVIS supports `prob` blocks for stochastic AI state transitions.
Example:
```jrv
prob {
    0.8 -> { // 80% chance
        execute candidate_a()
    }
    0.2 -> { // 20% chance
        execute candidate_b()
    }
}
```

### 3. State Superposition
Streams can exist in a "Superposed" state where the VM executes multiple paths in parallel (simulated or hardware-accelerated) and collapses the state based on a sensor trigger.

## Consequences
- Native support for Reinforcement Learning and Bayesian logic.
- Physical-world constraints (power) are now compiler-enforced primitives.
- Non-deterministic AI logic is structured and verifiable.
