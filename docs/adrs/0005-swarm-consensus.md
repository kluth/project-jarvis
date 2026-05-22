# ADR 005: Swarm Consensus & Genetic Feedback Loops

## Status
Accepted - Rev 7.0 Native Swarm Consensus

## Context
AI-first languages must operate across clusters of compute nodes without manual synchronization bloat in a pure substrate.

## Decision
JARVIS implements **Swarm Consensus** and **Metric-Driven Feedback** as native language features.

### 1. Swarm Consensus (`sync`)
The `sync` block ensures that the state of a `memory` variable is consistent across the swarm using native protocols (e.g., Raft). The compiler verifies the latency cost and includes it in the Entropy Budget.

### 2. Native Gossip Primitives
Syntax for unstructured state propagation: `gossip "target.bridge";`

## Consequences
- Decentralized AI state management is now a language feature.
- The compiler/VM loop becomes a closed-circuit optimization system.
- Explicit Big-O modeling of network consensus protocols.
