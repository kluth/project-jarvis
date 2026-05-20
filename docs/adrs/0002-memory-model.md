# ADR 002: Memory & Concurrency Model

## Status
Proposed

## Context
Voice-first AI requires zero-latency context switching and lock-free data flow.

## Decision
JARVIS implements a **Deterministic Stream-Graph Concurrency Model**.

### Memory Management
- **Static Memory (MCU Profile):** No heap. All buffers are pre-allocated at compile time based on graph topology.
- **Region-Based (Cluster Profile):** Dynamic allocation is confined to specific "Life-Regions" that are bulk-freed, eliminating fragmentation and GC pauses.
- **Linear Ownership:** Similar to Rust, but enforced through a simpler "Pass-or-Process" semantic for streams.

### Concurrency
- **Lock-Free Channels:** Built-in atomic ring buffers for stream communication.
- **No Threads:** The substrate uses an asynchronous executor that polls stream-graphs directly on bare metal.
- **NUMA Awareness:** The "Macro" profile automatically maps graph nodes to CPU cores to minimize cache misses.

## Consequences
- Zero context-switch overhead.
- Total determinism in audio processing pipelines.
- Safety from race conditions by design.
