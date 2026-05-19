# THE OMEGA ARCHITECTURAL STANDARD: PROJECT JARVIS (REV. 3.0)

This document is the absolute law of Project JARVIS. All logic must satisfy these mathematical and ethical constraints.

## 1. Core Mandates

### 1.1. Native AOT Ecosystem
- **Zero VM Overhead:** JARVIS is a pure AOT language. No Virtual Machines.
- **Jarvis-ISA:** Compilation targets the energy-aware Jarvis-ISA (SIMD-first).

### 1.2. Performance & Environment (PDD/EFDD)
- **Big-O Enforcement:** Every path MUST have a statically verified Big-O signature.
- **Energy Modeling:** Opcode-level nanojoule budgets are enforced at compile-time.
- **Wait-Free Architecture:** Spinlocks and Mutexes are architectural failures.

## 2. Advanced System Paradigms

### 2.1. Data Biodegradability (DBD)
- **Cryptographic Wiping:** All sensitive data scopes MUST implement $O(1)$ zeroing on `Drop`.

### 2.2. Algorithmic Homeostasis (AHD)
- **Autonomous Repair:** The kernel isolates anomalies in a Sandbox and requests patches via the Neuro-Compiler Interface (NCI).

### 2.3. Provenance Purity (PFDD)
- **Zero-Knowledge Proofs:** External weights or data dependencies MUST be cryptographically verified.

### 2.4. Generational Permanence (GDD)
- **Meta-AST:** Code is stored in hardware-agnostic Meta-AST format for 20+ year longevity.
- **Bit-Determinism:** Builds MUST be bit-for-bit reproducible (SHA-256 matching).

## 3. Repository Governance

### 3.1. Neuro-Compiler Interface (NCI)
- **MCP Server:** The compiler acts as an MCP Server for autonomous AI repair loops.
- **Actionable Diagnostics:** Failures are emitted as structured JSON for AI consumption.

### 3.2. Scientific Gatekeeper (CI)
- **Mutation Testing:** 100% kill-rate required for `verify` blocks.
- **Entropy Control:** No net increase in cyclomatic entropy per module.

**JARVIS: THE MATHEMATICAL GUARANTEE OF EVOLUTION.**
