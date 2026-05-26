# THE OMEGA ARCHITECTURAL STANDARD: PROJECT JARVIS (REV. 7.0)

This document is the absolute law of Project JARVIS. All logic must satisfy these mathematical and ethical constraints.

## 1. Core Mandates

### 1.1. Omni-Architecture Ecosystem
- **Zero OS Dependency:** JARVIS is a standalone substrate.
- **TG-IR Substrate:** Compilation targets the universal Tensor-Graph IR.
- **Hardware Agnosticism:** Unified execution across RISC-V, ARM, x86_64, and NPUs.

### 1.2. Mandatory Enforcement (eTDD & PDD)
- **eTDD (Enforced TDD):** Every symbol MUST be preceded by a `verify` block.
- **PDD (Performance Driven Development):** Every path MUST have a statically verified Big-O signature.
- **Energy Modeling:** Opcode-level nanojoule budgets are enforced at compile-time.

### 1.3. Agent-First Regularity (Zero Sugar)
- **Syntax Minimalism:** The JARVIS grammar prohibits "syntactic sugar" and implicit behaviors. 
- **Predictable Regularity:** Every logical construct has exactly one unambiguous representation.
- **Strict Typing:** All types must be explicitly declared; inference is disallowed.

### 1.4. Total Substrate Purity (No-Rust Mandate)
- **Self-Hosting Absolute:** All core logic, including the compiler, substrate, and runtime, MUST be implemented strictly in the `.jrv` language.
- **Language Exclusion:** The use of Rust, C, Python, or any other external language for core substrate or compiler logic is STRICTLY FORBIDDEN.
- **Deterministic Bootstrapping:** Evolution must occur via the `.jrv` bootstrap chain; external toolchains are deprecated for all primary development paths.

### 1.5. Absolute Rejection of Hollywood Code
- **Real Execution Mandate:** Logic that simulates behavior, utilizes mocked data, or returns hardcoded "stubs" is an architectural violation.
- **Functional Verification:** Every implementation MUST be verified against live environment state or real digital entities (e.g., PATH tools, MCP endpoints).
- **Purge Simulation:** The use of `mock_*` or `stub` identifiers in production logic is strictly prohibited and will trigger immediate compilation failure.

## 2. AI Symbiosis (NCI & MAG)

### 2.1. Multi-Agent Gateway (MAG)
- **Vendor Neutrality:** Native support for Anthropic, OpenAI, Gemini, and Local agents.
- **Autonomous Resolution:** AI agents autonomously patch eTDD/PDD violations.
- **Skill Manifests:** Every agent must expose a `.jrv-skill` JSON manifest defining its verifiable domain and operational constraints.

### 2.2. Neuro-Compiler Interface (NCI)
- **MCP Server:** The compiler acts as an MCP Server for real-time semantic interaction.
- **Actionable Diagnostics:** Failures are emitted as structured JSON AST deltas.
- **Autonomous Fix Plans:** For eTDD/PDD violations, the compiler provides machine-apply-ready JSON "Fix Plans" to guide agent-led recovery.

## 3. Advanced Paradigms

### 3.1. Data Biodegradability (DBD)
- **Cryptographic Wiping:** Sensitive data is zeroed on `Drop` ($O(1)$).

### 3.2. Provenance Purity (PFDD)
- **ZKP Verification:** External logic must be cryptographically proven.

**JARVIS: THE MATHEMATICAL GUARANTEE OF EVOLUTION.**
