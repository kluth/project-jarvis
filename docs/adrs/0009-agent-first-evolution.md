# ARCHITECTURAL DECISION RECORD: AGENT-FIRST EVOLUTION (ZERO-INSPIRATION)

## 1. Context & Problem Statement
Project JARVIS is designed for autonomous evolution. However, traditional human-centric programming paradigms often introduce ambiguity, complex "syntactic sugar," and unstructured error reporting that increase the "token penalty" and hallucination risk for AI agents. To achieve true 1:1 symbiosis, the language and its tooling must be optimized for machine-to-machine interaction.

## 2. Proposed Solution: The Zero Paradigm
Inspired by `zerolang`, we are evolving JARVIS toward an "Agent-First" architecture. This involves stripping away human-centric convenience in favor of deterministic regularity and actionable compiler feedback.

### 2.1. Zero-Sugar Syntax
We enforce a strict, regular grammar. By removing implicit behaviors and multiple ways to achieve the same goal, we reduce the search space for LLMs, leading to higher generation accuracy and lower token consumption.
- **Explicit Type Bounds:** No implicit conversion or inference that could mask architectural violations.
- **Unambiguous Grammars:** One logical intent maps to exactly one syntactic structure.

### 2.2. Actionable Fix Plans (NCI-V2)
The Neuro-Compiler Interface (NCI) is upgraded from a reporter to a collaborator. When an `eTDD` (Enforced TDD) or `PDD` (Performance Driven Development) constraint is violated, the compiler generates a structured JSON "Fix Plan."
- **AST Deltas:** The plan contains the exact nodes to be modified.
- **Verification Guarantee:** The proposed fix is pre-verified against the IR substrate before being offered to the agent.

### 2.3. Structural Introspection (Graph & Energy)
To support autonomous PDD and EFDD (Environmentally Friendly Driven Development), the compiler exposes its internal state as structured metadata.
- **`--graph`:** Emits the Tensor-Graph IR as a queryable JSON dependency graph.
- **`--energy`:** Provides nanojoule-level profiling data to ensure compliance with opcode-level energy budgets.

## 3. Consequence & Trade-offs
- **Human Readability:** While still readable, the lack of "clever" syntax may feel verbose to human developers. This is an intentional trade-off to prioritize agent performance.
- **Autonomy:** Agents can now repair codebase violations with minimal human intervention by consuming "Fix Plans" directly.
- **Deterministic Scaling:** The use of Skill Manifests (`.jrv-skill`) ensures that the Multi-Agent Gateway (MAG) can verify agent capabilities before delegating tasks within the swarm.

## 4. Status
**Accepted** - Integrated into GEMINI.md Rev 6.0.
