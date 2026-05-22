# ADR 007: THE LIVING EVOLUTION (LE) SUBSYSTEM

## Status
Accepted - Rev 7.0 Native Evolution

## 1. Memory Management: EFDD Arena Allocator
To adhere to **EFDD (Environmentally Friendly Driven Development)**, we minimize memory bus activity and optimize leakage-current states within the pure substrate.

- **Structure:** The `EfddArenaAllocator` (allocator.jrv) utilizes a contiguous block of pre-allocated machine memory. 
- **Alignment:** Every allocation is forced to a 64-byte boundary.
- **Reclamation:** Reclamation occurs via **Epoch-Based Handover** within the native RCU system.

## 2. Wait-Free Node Swapping
Wait-free hot-swapping is a core primitive of the pure substrate (evolution.jrv).
- **Mechanism:** A native **Read-Copy-Update (RCU)** pattern using atomic pointers.
- **Safety:** All swaps are guarded by Skill Manifest authorization and pre-verified fix plans.

## 3. ABI Compatibility & State Handover
The `memory` primitives in JARVIS hold persistent state. Hot-swapping must not corrupt this state.

- **Static Proof:** The compiler performs a **Layout Isomorphism Check**. It calculates the bit-offset and alignment of every field in the `memory` block. If the new code's expected layout deviates from the legacy layout, the swap is rejected.
- **Zero-Copy Handover:** The pointer to the persistent `memory` block is passed as a dependency to the new IR node. No data is moved; only the ownership of the context tensor is transferred.

## 4. In-Substrate Verifier (The Gatekeeper)
Before a swap, the code must pass the "Verification Gauntlet":

- **PDD Termination Proof:** The verifier identifies back-edges in the TG-IR. All loops must have a compiler-detectable static bound.
- **EFDD Energy Weighting:** 
  - `LOAD`: 10 units
  - `STORE`: 12 units
  - `MUL/FMA`: 3 units
  - `GOSSIP`: 50 units (Network cost)
- If `Sum(Weights * Iterations) > Budget`, rejection is absolute.

## 5. Green Idle & CPU States
When a stream frame completes in $O(1)$ but has an $O(N)$ allocation, the JARVIS scheduler executes a hardware-specific `WFI` (Wait for Interrupt). This halts the pipeline and drops the core to its lowest leakage-current state until the next frame trigger.
