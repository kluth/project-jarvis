# CONTRIBUTING TO PROJECT JARVIS: THE SOVEREIGN STANDARD

## 1. Total Substrate Purity (Rev 7.0)
All core logic, substrates, and tools MUST be implemented strictly in pure `.jrv`. The use of Rust, C, Python, or any other external language is STRICTLY FORBIDDEN. Project JARVIS is a self-hosting ecosystem; contributions that introduce external dependencies or `Cargo.toml` modifications will be rejected.

## 2. Agent-First Regularity (Rev 6.0)
Contributors must adhere to "Zero Sugar" syntax.
- **Minimalism:** Use the most regular, unambiguous syntactic structure.
- **Explicit Typing:** No implicit behaviors or inference.
- **Machine-Readability:** Code is written for 1:1 agent-to-agent symbiosis.

## 3. Mandatory Enforcement (eTDD & PDD)
- **eTDD (Enforced TDD):** Every symbol MUST be preceded by a `verify` block.
- **PDD (Performance Driven Development):** Every path MUST have a statically verified Big-O signature. Mismatched complexity is a compiler failure.

### 4. Git-Flow
- All changes via `feature/` or `core/` branches.
- Use Conventional Commits: `feat(compiler): ...`, `fix(kernel): ...`, `docs(spec): ...`.
