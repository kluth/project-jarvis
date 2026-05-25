# Project JARVIS: The Sovereign AI-Native Substrate

![Substrate Purity](https://img.shields.io/badge/JARVIS-Total_Purity-blueviolet)
![Agent First](https://img.shields.io/badge/Agent-First-green)
![Self Hosting](https://img.shields.io/badge/Sovereign-Self_Hosting-blue)
![CI](https://github.com/kluth/project-jarvis/actions/workflows/rust-ci.yml/badge.svg)
![Dashboard](https://github.com/kluth/project-jarvis/actions/workflows/dashboard.yml/badge.svg)

JARVIS is a 100% self-hosting, omni-architecture programming language and execution substrate. The ecosystem is implemented in native `.jrv` code with a Rust-based Stage-0 bootstrap compiler.

## 🚀 Sovereign Features

- **Agent-First Regularity (Zero Sugar):** A deterministic grammar optimized for machine-to-machine symbiosis and token efficiency.
- **Autonomous Self-Healing:** The compiler generates machine-apply-ready **Fix Plans** to resolve architectural violations.
- **Omni-Architecture:** Target any device from microcontrollers to cloud nodes via the pure **TG-IR** chain.
- **eTDD & PDD:** Mandatory mathematical proofs for every line of sovereign code.
- **Economic Governor:** Runtime cost-aware execution routing with fixed-point micro-USD calculations.
- **Deterministic DAG Runtime:** Append-only error deltas with SHA-256 state memoization.

## 🏗️ Project Structure

```
project-jarvis/
├── compiler/          # Rust Stage-0 bootstrap compiler
│   ├── src/           # Lexer, parser, codegen, governor engine, MMU, runtime executor
│   └── tests/         # Integration test suite (36+ tests)
├── runtime/           # Display/windowing substrate (minifb)
├── substrate/         # TG-IR substrate (Rust + .jrv drivers)
├── specs/             # Formal grammar and architecture specifications
├── docs/adrs/         # Architecture Decision Records (12 ADRs)
├── tools/             # jrvc_stage0.sh, dashboard generator, demos
└── .github/           # CI/CD — 5 workflows + dependabot
```

## 🦀 Phase 1: Governor Engine (Complete)

The E-4.Secure spec implementation adds a fully tested runtime layer:

| Module | Description | Tests |
|--------|-------------|-------|
| `fixed_types` | Fixed-width type aliases (Hash256, CurrencyUSD64, etc.) | 4 |
| `struct_layouts` | Cache-line-aligned structs (ExecutionSignature 68B, ErrorLedgerEntry 1040B) | 6 |
| `memory_mgmt` | Blacklist registry (4096 slots), error ledger (16), 64KB data registers | 4 |
| `governor_engine` | Thermal/VRAM/OPEX gates with fixed-point micro-USD routing | 4 |
| `runtime_executor` | DAG step executor with idempotency, retry, fallback | 2 |
| `e2e tests` | Spec contract string matching + economic halt verification | 2 |

**CLI:** `jrvc --governor`, `jrvc --blacklist-demo`, `jrvc --full-diagnostics`, `jrvc --version`

## ⚡ Quick Start

```bash
# Build the Stage-0 compiler
cargo build -p jarvis-compiler

# Run diagnostics
./target/debug/jarvis-compiler --full-diagnostics

# Run tests
cargo test --workspace

# Bootstrap via Stage-0 script
bash tools/jrvc_stage0.sh --bootstrap
```

## 🔧 CI/CD Pipeline

| Workflow | Trigger | What It Does |
|----------|---------|-------------|
| Rust Compiler CI | Push/PR to main | Build + test all crates, CLI smoke tests |
| Quality Gates | Push/PR to main | Lint, format, security audit, code stats |
| Scientific Dashboard | Push to main / weekly | Real diagnostics output, auto-commits to README |
| Sovereign CI | Push to main | Legacy verification pipeline |
| Release | Tag push `v*` | Cross-platform binary builds, GitHub Release |

## 📐 Architecture

- **Stage-0 Bootstrap:** Rust compiler (13 modules, 36+ tests)
- **Target:** Self-hosting `.jrv` compiler (Phase 2+)
- **Runtime Model:** Deterministic DAG with SHA-256 memoization, append-only error deltas
- **Resource Management:** Economic governor with thermal/VRAM/financial gates
- **CI:** Dual-path verification (Rust tests + Stage-0 bootstrap script)