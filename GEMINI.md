# THE OMEGA ARCHITECTURAL STANDARD: PROJECT JARVIS

This document serves as the foundational law for all Gemini-led development on Project JARVIS. These rules are absolute. Any deviation is a critical failure.

## 1. Core Mandates

### 1.1. Language Substrate
- **Rust-Exclusive Toolchain:** The compiler, runtime, and kernel MUST be built in pure Rust. No TypeScript, No C++, No external parser generators.
- **Zero-Dependency Policy:** Third-party crates are forbidden. All logic (Lexer, Parser, AST, VM) must be handcrafted at the bit-level to ensure maximum optimization.

### 1.2. Performance Driven Development (PDD)
- **Mandatory Big-O Signatures:** Every function must declare its mathematical complexity (e.g., $O(1)$, $O(N)$). 
- **Static Verification:** The compiler MUST reject any code where the analyzed complexity exceeds the declared complexity.
- **Entropy Budgeting:** All modules must adhere to physical energy/compute budgets (`budget` blocks) verified at compile-time.

### 1.3. Syntax-Level TDD
- **Verification Primitives:** Tests are first-class language citizens (`verify` blocks).
- **Forced Compilation:** A module without passing test suites is physically incapable of emitting a binary.

## 2. Advanced Paradigms

### 2.1. Neuro-Symbolic Computation
- **Temporal Memory:** Support for persistent AI state (`memory`) across stream executions.
- **Self-Evolution:** `evolve` blocks allow the compiler to autonomously mutate and benchmark logic.
- **Formal Contract Guards:** All self-modifying code MUST satisfy a first-order logic `contract`.

### 2.2. Distributed Swarm Intelligence
- **Native Consensus:** `sync` blocks enforce Raft/Gossip protocols at the language level.
- **Knowledge Addressability:** Support for semantically-indexed storage (`knowledge`) using vector embeddings.

### 2.3. Omni-Agnostic Execution
- **AI-First Design:** Syntax is optimized for agentic synthesis and token efficiency, not human readability.
- **Hardware Agnosticism:** TG-IR (Tensor-Graph IR) allows seamless execution across NPU, GPU, and MCU without manual kernel mapping.

## 3. Engineering Rigor
- **Git-Flow Enforcement:** Strict adherence to `feature/` and `core/` branching.
- **Conventional Commits:** Every change must follow the semantic commit standard.
- **High-Class Maintenance:** No shortcuts. No technical debt. No legacy assumptions.

**JARVIS IS A MATHEMATICAL GUARANTEE OF EFFICIENCY.**
