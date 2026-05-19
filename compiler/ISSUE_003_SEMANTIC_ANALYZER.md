# Issue Spec: Complexity Semantic Analyzer

## Context
JARVIS requires that every function's declared Big-O complexity matches its actual implementation structure.

## Acceptance Criteria
- [ ] Traverses the AST and identifies `ComplexityBlock` and `Function` nodes.
- [ ] Analyzes loop nesting to calculate expected Big-O.
- [ ] Throws a compilation error if declared complexity != analyzed complexity.
- [ ] Supports $O(1)$ (no loops) and $O(N)$ (one non-nested loop).

## Big-O Targets
- Analysis: $O(N)$ where $N$ is the number of AST nodes.
