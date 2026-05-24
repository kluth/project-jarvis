# ADR 0010: Driver Architecture and Ambient Assimilation Subsystem

## Status
Accepted

## Context
Project JARVIS requires a sovereign, hardware-agnostic driver ecosystem that can autonomously assimilate ambient devices while maintaining strict substrate purity (Rev 7.0), isolation, and energy discipline.

## Decisions

### 1. KDL/UDL Split over Monolithic Driver Model
- **Decision**: All drivers are split into a Kernel-Mode Layer (KDL) and a User-Mode Layer (UDL).
- **Rationale**: Ensures stability through isolation. If the UDL (API logic, shader compilation) crashes, the KDL (raw hardware access) remains stable.
- **Alternatives**: Monolithic drivers (high performance but zero isolation).
- **Trade-offs**: Slightly higher latency in the TG-IR command stream, but absolute substrate integrity.

### 2. Probe/Negotiate/Defer over Static Capability Tables
- **Decision**: Drivers use a dynamic `probe() → negotiate() → defer_until()` lifecycle.
- **Rationale**: Enables self-regulation. The system adapts to available hardware features at runtime rather than hardcoding assumptions.
- **Alternatives**: Compile-time feature flags or hardcoded PID/VID matching.
- **Trade-offs**: Complexity in initialization logic is offset by total future-proofing.

### 3. AST-level Template Instantiation over String Synthesis
- **Decision**: Driver synthesis for ambient devices uses AST-level template manipulation.
- **Rationale**: Allows the Neuro-Compiler Interface (NCI) to statically verify synthesized code before injection, preventing "injection" attacks at the driver level.
- **Alternatives**: String concatenation or procedural code generation.
- **Trade-offs**: Synthesis takes more energy than string concatenation but provides mathematical safety.

### 4. Three-stage Fingerprinting over ID Database Matching
- **Decision**: Device identification uses Profile Match → Capability Probe → Behavioral Fingerprinting.
- **Rationale**: Identifies unknown or generic devices that do not exist in a static database.
- **Alternatives**: Simple MAC OUI or PID/VID lookup.
- **Trade-offs**: Behavioral analysis requires an observation window, delaying initial assimilation for unknown devices.

### 5. Multi-level Trust Model over Binary Trusted/Untrusted
- **Decision**: Trust levels: Untrusted, Observed, Trusted, Sovereign.
- **Rationale**: Provides granular security. Most ambient devices start as Observed or Untrusted; only locally-declared or operator-proven devices reach Trusted/Sovereign status.
- **Alternatives**: All-or-nothing ACLs.
- **Trade-offs**: Requires more complex policy evaluation in the trust engine.

### 6. Heartbeat + SUSPENDED State over Immediate Eviction
- **Decision**: Devices transition through ACTIVE → SUSPENDED → EVICTED states based on heartbeats.
- **Rationale**: Wireless/Ambient devices often experience temporary link loss. Preserving state in SUSPENDED enables fast-path re-activation.
- **Alternatives**: Immediate deletion on link loss.
- **Trade-offs**: Memory is held for suspended devices until eviction timeout occurs.

## Consequences
- Total autonomy in hardware discovery and assimilation.
- Guaranteed stability of the JARVIS substrate.
- Verifiable energy and complexity budgets for every driver operation.
