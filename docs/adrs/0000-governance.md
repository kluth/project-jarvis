# ADR 000: Open-Source Governance & Repository Architecture

## Status
Proposed

## Context
Project JARVIS requires a flagship open-source structure that facilitates scaling from 8-bit MCUs to distributed supercomputers while enforcing architectural perfection.

## Decision
We adopt a monorepo structure with strict isolation between the toolchain, the kernel, and the language specifications.

### Repository Layout
- `/compiler`: Handcrafted Rust-based OMNI-Target compiler.
- `/kernel`: Bare-metal JARVIS OS kernel.
- `/specs`: Formal language and PDD definitions.
- `/docs/adrs`: Architecture Decision Records.
- `/stdlib`: The JARVIS core library (Test-First).

### Governance Model
- **Licensing:** MIT License for maximum adoption.
- **Contribution:** Strict PDD (Performance Driven Development) enforcement. All PRs must include Big-O verification and 100% test coverage.
- **Zero-Dependency:** Third-party libraries are forbidden in the core toolchain.

## Consequences
- High initial development overhead due to zero-dependency mandate.
- Guaranteed long-term maintainability and hardware portability.
- Compiler-level rejection of non-performant code.
