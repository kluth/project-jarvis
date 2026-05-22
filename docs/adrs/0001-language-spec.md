# ADR 001: Language Specification & PDD Mechanics

## Status
Accepted - Rev 7.0 Total Substrate Purity

## Context
JARVIS needs a language that is both hardware-agnostic and AI-native, with strict enforcement of efficiency within a 100% self-hosted ecosystem.

## Decision
The JARVIS language (ext: `.jrv`) utilizes a block-structured syntax with mandatory performance annotations and Zero Sugar regularity.

### Core Syntax Principles
- **Mandatory PDD:** Every function requires a `complexity` block.
- **Forced TDD:** Every module requires a `verify` block containing unit tests.
- **Memory Safety:** Ownership is tracked via native linear logic in the pure compiler.
- **Stream-First:** Real-time audio and tensor data are treated as continuous sovereign streams.

### Compiler Verification
The native JARVIS compiler performs:
1. Lexical Analysis.
2. AST Construction.
3. **Complexity Verification:** Static analysis of loops and recursions to match declared Big-O.
4. **Test Execution:** Run `verify` blocks during compilation. Failure to pass tests halts binary emission.
5. **TG-IR Generation:** Direct emission of the universal Tensor-Graph IR.

## Consequences
- Impossible to commit unoptimized or untested code.
- Guaranteed predictable latency for real-time AI tasks.
