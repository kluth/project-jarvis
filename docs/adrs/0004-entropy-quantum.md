# ADR 004: Entropy Budgeting & Quantum-Probabilistic Streams

## Status
Accepted - Rev 7.0 Native Entropy Budgeting

## Context
Deterministic Big-O is insufficient for battery-constrained environments and non-deterministic AI decision logic within a pure ecosystem.

## Decision
JARVIS implements **Entropy Budgeting** and **Probabilistic Branching** as native primitives.

### 1. Entropy Budgeting (`budget`)
Functions declare an energy/compute budget. The native compiler estimates instruction-level energy consumption (nanojoules) and fails if the static path exceeds the budget.

### 2. Probabilistic Streams (`prob`)
Instead of `if/else`, JARVIS supports regular `prob` blocks for stochastic AI state transitions, optimized for agentic generation.

### 3. State Superposition
Streams can exist in a "Superposed" state where the VM executes multiple paths in parallel (simulated or hardware-accelerated) and collapses the state based on a sensor trigger.

## Consequences
- Native support for Reinforcement Learning and Bayesian logic.
- Physical-world constraints (power) are now compiler-enforced primitives.
- Non-deterministic AI logic is structured and verifiable.
