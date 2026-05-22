# ADR 002: Memory & Concurrency Model

## Status
Accepted - Rev 7.0 Pure JARVIS Semantics

## Context
Voice-first AI requires zero-latency context switching and lock-free data flow within a pure machine-native ecosystem.

## Decision
JARVIS implements a **Deterministic Stream-Graph Concurrency Model** using native `.jrv` primitives.

### Memory Management
- **Static Memory (MCU Profile):** No heap. All buffers are pre-allocated at compile time based on graph topology.
- **Region-Based (Cluster Profile):** Dynamic allocation is confined to specific "Life-Regions" that are bulk-freed, eliminating fragmentation and GC pauses.
- **Linear Ownership:** Strict "Pass-or-Process" semantics for sovereign streams.

### Concurrency
- **Lock-Free Channels:** Built-in atomic ring buffers for stream communication.
- **No Threads:** The substrate uses an asynchronous executor that polls stream-graphs directly on bare metal.
- **NUMA Awareness:** The "Macro" profile automatically maps graph nodes to CPU cores to minimize cache misses.

## Consequences
- Zero context-switch overhead.
- Total determinism in audio processing pipelines.
- Safety from race conditions by design.
