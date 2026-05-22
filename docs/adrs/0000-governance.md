# ADR 000: Open-Source Governance & Repository Architecture

## Status
Accepted - Rev 7.0 Total Substrate Purity

## Context
Project JARVIS requires a flagship open-source structure that facilitates scaling from 8-bit MCUs to distributed supercomputers while enforcing architectural perfection in a 100% self-hosted environment.

## Decision
We adopt a monorepo structure implemented strictly in pure `.jrv`.

### Repository Layout
- `/compiler`: Native `.jrv` OMNI-Target compiler.
- `/substrate`: Pure `.jrv` bare-metal JARVIS Universal Substrate.
- `/specs`: Formal language and PDD definitions.
- `/docs/adrs`: Architecture Decision Records.

### Governance Model
- **Licensing:** MIT License for maximum adoption.
- **Contribution:** Strict PDD (Performance Driven Development) enforcement. All PRs must include Big-O verification and 100% test coverage.
- **Zero-Dependency:** Third-party libraries are forbidden in the core toolchain.

## Consequences
- High initial development overhead due to zero-dependency mandate.
- Guaranteed long-term maintainability and hardware portability.
- Compiler-level rejection of non-performant code.
