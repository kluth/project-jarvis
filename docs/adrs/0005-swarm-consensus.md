# ADR 005: Swarm Consensus & Genetic Feedback Loops

## Status
Proposed

## Context
AI-first languages must operate across clusters of sensors and compute nodes without manual synchronization bloat.

## Decision
JARVIS will implement **Swarm Consensus** and **Metric-Driven Feedback**.

### 1. Swarm Consensus (`sync`)
The `sync` block ensures that the state of a `memory` variable is consistent across the swarm using a specific protocol.
Example:
```jrv
sync (protocol: Raft) {
    memory SwarmState: i32 = 0;
}
```
The compiler verifies the latency cost ($O(N \log N)$ network overhead) and includes it in the Entropy Budget.

### 2. Genetic Feedback (`feedback`)
Modules can now include a `feedback` block that the VM uses to store execution metrics (latency, energy, accuracy). The compiler reads this "Genetic Log" during the next compilation cycle to steer `evolve` block implementations.

### 3. Native Gossip Primitives
Syntax for unstructured state propagation: `gossip(state, topology: Grid);`

## Consequences
- Decentralized AI state management is now a language feature.
- The compiler/VM loop becomes a closed-circuit optimization system.
- Explicit Big-O modeling of network consensus protocols.
