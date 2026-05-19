# Issue Spec: Handcrafted Lexer & Minimal Tokenizer

## Context
The JARVIS compiler needs a high-performance, zero-dependency lexer to transform `.jrv` source into a token stream.

## Acceptance Criteria
- [ ] Supports core keywords: `module`, `complexity`, `func`, `verify`, `test`.
- [ ] Handles PDD Big-O notations: `O(1)`, `O(N)`, `O(N log N)`.
- [ ] Implements zero-allocation tokenization (using `&str` slices).
- [ ] Verified via internal Rust TDD.

## Big-O Targets
- Tokenization: $O(N)$ where $N$ is the number of source characters.
- Space Complexity: $O(K)$ where $K$ is the peak token size (reusable buffers).
