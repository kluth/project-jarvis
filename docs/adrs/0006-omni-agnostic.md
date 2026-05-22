# ADR 006: Omni-Agnostic Protocol & AI-First Design

## Status
Accepted - Rev 7.0 Sovereign Omni-Protocol

## Context
Traditional programming languages are designed for human readability. JARVIS is an Agent-First ecosystem optimized for Agentic Synthesis, Formal Verification, and Hardware-Agnostic execution.

## Decision
JARVIS implements the **Omni-Agnostic Protocol (OAP)** within a 100% self-hosted substrate.

### 1. Contract-Guarded Evolution (`contract`)
Every `evolve` block MUST be accompanied by a `contract`. The native compiler uses a built-in verifier to prove that mutated code satisfies the contract.

### 2. Context-Addressable Storage (`knowledge`)
Memory is managed via `knowledge` blocks—Content-Addressable Storage where data is indexed by semantic vector embeddings.

### 3. Tensor-Graph IR (TG-IR)
The native compiler emits a hardware-agnostic Tensor-Graph IR. The pure JARVIS substrate (hal.jrv) autonomously schedules nodes to the most efficient hardware without developer intervention.

## Consequences
- Human readability is sacrificed for **Token Efficiency** and **Semantic Precision**.
- Guaranteed safety for self-modifying code via mathematical proofs.
- Automatic transition from "Software" to "Distributed Intelligence".
