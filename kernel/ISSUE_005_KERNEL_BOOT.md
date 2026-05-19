# Issue Spec: JARVIS Microkernel Bootstrapping (no_std)

## Context
Project JARVIS needs a bare-metal kernel to execute stream-graphs with zero latency.

## Acceptance Criteria
- [ ] `no_std` Rust environment.
- [ ] Minimal scheduler for asynchronous stream polling.
- [ ] Static allocation for Stream-Graph nodes.

## Big-O Targets
- Scheduling: $O(1)$ context switch (Poll-based).
