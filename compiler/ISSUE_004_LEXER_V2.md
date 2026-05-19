# Issue Spec: Lexer Expansion (Types, Ops, Control Flow)

## Context
To make JARVIS a full-featured super language, the lexer must support data types, variables, arithmetic operators, and control flow keywords.

## Acceptance Criteria
- [ ] Supports primitives: `i32`, `f32`, `Stream`
- [ ] Supports declarations: `let`, `return`
- [ ] Supports control flow: `if`, `else`, `for`, `while`
- [ ] Supports operators: `+`, `-`, `*`, `/`, `=`, `==`, `,`, `:`, `;`
- [ ] Supports numeric literals (integers and floats)

## Big-O Targets
- Tokenization: $O(N)$
