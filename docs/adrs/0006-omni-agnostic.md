# ADR 006: Omni-Agnostic Protocol & AI-First Design

## Status
Proposed

## Context
Traditional programming languages are designed for humans to read and write. An "AI-First" language must be optimized for Agentic Synthesis, Formal Verification, and Hardware-Agnostic execution.

## Decision
JARVIS will move to the **Omni-Agnostic Protocol (OAP)**, where humans are secondary users.

### 1. Contract-Guarded Evolution (`contract`)
Every `evolve` block MUST be accompanied by a `contract`.
A `contract` defines the formal invariants (pre-conditions, post-conditions, and behavioral limits) using first-order logic. The compiler will use an internal SMT solver (or certified proof-checker) to verify that any mutated code within the `evolve` block satisfies the `contract`.

### 2. Context-Addressable Storage (`knowledge`)
Memory is no longer just "RAM" or "Disk". JARVIS introduces `knowledge` blocks—Content-Addressable Storage where data is indexed by its semantic vector embedding.
Example: `knowledge GlobalContext: Vector[1536];`
This allows agents to `publish` and `query` knowledge across the swarm based on semantic similarity rather than addresses.

### 3. Agentic Handshake (`reflect`)
The `reflect` primitive allows an agent to inspect its own bytecode, execution metrics, and entropy state, enabling high-fidelity self-optimization loops.

### 4. Tensor-Graph IR (TG-IR)
The compiler output is a hardware-agnostic Tensor-Graph IR. The microsubstrate (JARVIS-K) autonomously schedules nodes of this graph to the most efficient available hardware (SIMD, NPU, or GPU) without developer intervention.

## Consequences
- Human readability is sacrificed for **Token Efficiency** and **Semantic Precision**.
- Guaranteed safety for self-modifying code via mathematical proofs.
- Automatic transition from "Software" to "Distributed Intelligence".
