# CONTRIBUTING TO PROJECT JARVIS

## The OMEGA Standard

### 1. Performance Driven Development (PDD)
All code MUST declare its mathematical complexity in the documentation/signature. 
Example: `func process_stream(data: Stream) -> Result [Complexity: O(N)]`
The compiler will verify this. Mismatched complexity is a failure.

### 2. Zero-Dependency Mandate
Do not add `Cargo.toml` dependencies. All algorithms must be handcrafted to ensure bare-metal optimization and zero bloat.

### 3. Test-Driven Development (TDD)
Tests are language primitives. A module without verified test suites will not compile.

### 4. Git-Flow
- All changes via `feature/` or `core/` branches.
- Use Conventional Commits: `feat(compiler): ...`, `fix(kernel): ...`, `docs(spec): ...`.
