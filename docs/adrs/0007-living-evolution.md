# ARCHITECTURAL BLUEPRINT: LIVING EVOLUTION (LE) SUBSYSTEM

## 1. Memory Management: EFDD Arena Allocator
To adhere to **EFDD (Environmentally Friendly Driven Development)**, we must minimize memory bus activity and keep DRAM banks in low-power states by maximizing data locality.

- **Structure:** The `EfddArenaAllocator` utilizes a single contiguous block of pre-allocated bare-metal memory. 
- **Alignment:** Every allocation is forced to a 64-byte boundary (Cache-Line size). This prevents "False Sharing" at the hardware level and ensures that a single NPU/CPU burst fetch retrieves the entire node header, minimizing the energy-per-instruction (EPI) penalty.
- **Reclamation:** We avoid non-deterministic GC. Reclamation occurs via **Epoch-Based Handover**. When a node is hot-swapped, it enters a "Retiring" state. It is only physically deallocated once the global Epoch counter has advanced beyond the point where any active stream-graph thread could hold a reference.

## 2. Wait-Free Node Swapping (RCU/Epochs)
Traditional locking (Mutex/Spinlock) is an **EFDD violation** as it burns cycles without productive output.

- **Mechanism:** We use a **Read-Copy-Update (RCU)** pattern.
- **Operation:**
  1. The "Living Evolution" compiler prepares a new node in the Arena.
  2. The Verifier proves compliance (TDD/PDD/EFDD).
  3. An `AtomicPtr::swap` operation ($O(1)$) replaces the active execution pointer.
- **Safety:** We use `Acquire/Release` memory barriers. `Release` ensures that all data written to the new node is visible to all cores *before* the pointer becomes public. `Acquire` ensures that the execution engine sees the most recent stable implementation.

## 3. ABI Compatibility & State Handover
The `memory` primitives in JARVIS hold persistent state. Hot-swapping must not corrupt this state.

- **Static Proof:** The compiler performs a **Layout Isomorphism Check**. It calculates the bit-offset and alignment of every field in the `memory` block. If the new code's expected layout deviates from the legacy layout, the swap is rejected.
- **Zero-Copy Handover:** The pointer to the persistent `memory` block is passed as a dependency to the new IR node. No data is moved; only the ownership of the context tensor is transferred.

## 4. In-Kernel Verifier (The Gatekeeper)
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
