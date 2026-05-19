# ADR 003: Neuro-Symbolic Primitives & Self-Evolution

## Status
Proposed

## Context
Standard procedural languages lack the vocabulary to describe persistent AI state and autonomous algorithmic optimization.

## Decision
JARVIS will implement "Temporal Memory" and "Evolving Blocks" as first-class citizens.

### 1. Temporal Memory (`memory`)
Variables declared as `memory` persist between function calls across a stream. The compiler manages the lifecycle of these tensors, ensuring zero-copy access during real-time processing.

### 2. Evolving Blocks (`evolve`)
An `evolve` block signals to the compiler that the internal logic can be mutated for performance. During compilation, JARVIS-LLVM (or custom backend) will benchmark multiple implementation candidates (e.g., SIMD vs. Neural vs. DSP) to find the optimal execution path for the target hardware.

### 3. Neural-Mapping (`neural`)
Syntax for direct mapping of tensor operations to NPU/GPU kernels.
Example: `let res = data @ weights;` (Neural Dot Product).

## Consequences
- The compiler becomes an active partner in algorithmic design.
- Direct hardware utilization for AI workloads without external libraries.
- Predictable temporal state for voice-first applications.
