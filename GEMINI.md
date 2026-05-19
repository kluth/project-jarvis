# THE OMEGA ARCHITECTURAL STANDARD: PROJECT JARVIS (REV. 2.0)

This document is the supreme law of Project JARVIS. Any violation results in immediate rejection by the kernel or the CI gatekeeper.

## 1. Core Mandates

### 1.1. Language Substrate
- **Rust-Exclusive Toolchain:** Pure Rust only. `no_std` mandatory for kernel/runtime.
- **Zero-Dependency Policy:** No external crates. All logic must be handcrafted.

### 1.2. Performance Driven Development (PDD)
- **Mandatory Big-O Signatures:** Every function MUST declare complexity: `// Time: O(N), Space: O(1)`.
- **Static Verification:** The verifier rejects code exceeding these bounds.

### 1.3. Environmentally Friendly Driven Development (EFDD)
- **Entropy Budgeting:** `budget { power: X_nj }` blocks are mandatory.
- **Wait-Free Architecture:** Mutexes and Spinlocks are strictly FORBIDDEN. Use atomic primitives or lock-free structures.
- **Cache-Line Alignment:** All structures $\ge 64$ bytes MUST be aligned to 64 bytes to minimize bus energy.

### 1.4. Eudaimonia Driven Development (EuDD)
- **Formal Contracts:** `contract { ensures X; }` must guard all state mutations.
- **Human Sovereignty:** Code that extracts private data without explicit, verifiable consent is mathematically rejected.

## 2. Repository Governance & CI Laws

### 2.1. Mutation Testing (The TDD Gatekeeper)
- The CI pipeline MUST perform AST mutation testing. If a test suite fails to detect a mutation, the Pull Request is REJECTED.

### 2.2. Cryptographic Integrity
- 100% of commits MUST be cryptographically signed (GPG/SSH). Unsigned commits are automatically purged.

### 2.3. The ADR-First Law
- No structural changes without an Architecture Decision Record (ADR) including PDD/EFDD pre-calculations.

### 2.4. Hermetic Builds & Entropy Control
- **Determinism:** Bit-for-bit reproducible binaries are mandatory.
- **Scout Rule:** A PR MUST NOT increase the module's overall cyclomatic entropy density.

### 2.5. DDD & Git-Flow
- **Domain-Driven Design:** Every GitHub Issue must define the Ubiquitous Language and context map.
- **Conventional Commits:** `feat:`, `fix:`, `refactor:`, `perf:`, `chore:`.

## 3. Engineering Rigor
- **Verify Blocks:** Tests are syntax primitives. No binary emission without passing `verify` blocks.
- **Zero Technical Debt:** Code is a mathematical guarantee, not a "guess."

**JARVIS: EFFICIENCY IS MORALITY.**
