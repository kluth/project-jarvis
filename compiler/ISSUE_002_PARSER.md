# Issue Spec: Handcrafted Recursive Descent Parser & AST

## Context
The JARVIS compiler requires a non-backtracking, recursive descent parser to construct the Abstract Syntax Tree (AST) from the token stream.

## Acceptance Criteria
- [ ] Parses `module` declarations.
- [ ] Parses `complexity` blocks and verifies Big-O syntax.
- [ ] Parses `func` signatures and empty bodies.
- [ ] Parses `verify` blocks.
- [ ] Zero-allocation AST nodes where possible.

## Big-O Targets
- Parsing: $O(N)$ where $N$ is the number of tokens.
- Memory: $O(D)$ where $D$ is the maximum depth of the AST.
