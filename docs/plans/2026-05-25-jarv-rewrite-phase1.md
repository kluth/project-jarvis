# JRV Rewrite — Phase 1: Governor Engine & Fixed-Width Runtime

> **For Hermes:** Use subagent-driven-development to implement this plan task-by-task.

**Goal:** Implement the core Governor Engine, Memory Management Unit, and fixed-width type system from Issue #191 (E-4.Secure), extending the existing Rust compiler as Stage-0 bootstrap.

**Architecture:** The existing Rust `jrvc` compiler stays as Stage-0 bootstrap. We add new types, memory registries, and the Governor Engine alongside the existing code. Later phases will deprecate Rust when a self-hosting JRV compiler exists.

**Bootstrap Strategy:**
1. Keep Rust compiler as Stage-0 bootstrap (not purged yet)
2. Extend `compiler/src/` with the new E-4.Secure modules in Rust
3. CLI tool `jrvc` gains new flags (`--governor`, `--blacklist`, `--execute`)
4. The new runtime modules live in `compiler/src/` as pure Rust structs and functions
5. The old syntax (module/func/budget) and new syntax (workflow/step/gate) coexist during transition

**Tech Stack:** Rust (Stage-0), existing `jarvis_compiler` crate, SHA-256 via `sha2` crate

---

## Task 1: Add Fixed-Width Primitive Types

**Objective:** Define the exact fixed-width type aliases from E-4.Secure Section 1 in a new module

**Files:**
- Create: `compiler/src/fixed_types.rs`
- Modify: `compiler/src/lib.rs` (add module declaration)

**Step 1: Create `compiler/src/fixed_types.rs`**

```rust
// Fixed-width primitive specifications (E-4.Secure Section 1)
// All sizes and padding match the spec exactly.

/// Binary-encoded raw SHA-256 (32 bytes, no hex string overhead)
pub type Hash256 = [u8; 32];

/// Maximum 65,535 execution blocks per DAG topology
pub type NodeIndex = u16;

/// Contextual execution path token for loop unrolling tracking
pub type CallSiteOffset = u32;

/// IEEE 754 single-precision floating point temperature value
pub type Celsius32 = f32;

/// Machine memory address sizes up to 16 Exabytes
pub type MemoryBytes = u64;

/// Fixed-point scaling factor = 10^-6 (Micro-USD parsing accuracy)
pub type CurrencyUSD64 = i64;

/// Bounded scalar range [0.0..1.0]
pub type ScalingFactor = f32;
```

**Step 2: Register module in `compiler/src/lib.rs`**

Add `pub mod fixed_types;` to the crate root.

**Step 3: Run cargo build**

```bash
cd /opt/data/project-jarvis/compiler
cargo build 2>&1
```

Expected: `Compiling jarvis-compiler ... Finished`

**Step 4: Commit**

```bash
git add compiler/src/fixed_types.rs compiler/src/lib.rs
git commit -m "feat: add fixed-width primitive types (E-4.Secure)"
```

---

## Task 2: Define Fixed-Width Struct Layouts

**Objective:** Define all data structures from E-4.Secure with exact sizes, padding, and cache-line alignment

**Files:**
- Create: `compiler/src/struct_layouts.rs`
- Modify: `compiler/src/lib.rs`

**Step 1: Create `compiler/src/struct_layouts.rs`**

```rust
use crate::fixed_types::*;

// ============================================================
// EXPLICIT ALLOCATION STRUCTURE LAYOUTS (E-4.Secure Section 1)
// ============================================================

/// 68 Bytes — Invariant tracking identification signature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionSignature {
    pub node_index: NodeIndex,                    // 2 bytes
    _padding_0: [u8; 2],                          // 2 bytes
    pub canonical_input_hash: Hash256,            // 32 bytes
    pub call_site_context_hash: Hash256,          // 32 bytes
    // Total: 68 bytes
}

/// 108 Bytes — Execution payload with retry tracking
#[derive(Debug, Clone)]
pub struct ExecutionPayload {
    pub signature: ExecutionSignature,            // 68 bytes
    pub retry_attempt: u32,                       // 4 bytes — zero-indexed retry counter
    pub compiled_payload_hash: Hash256,           // 32 bytes
    pub error_ledger_count: u16,                  // 2 bytes
    _padding_1: [u8; 2],                          // 2 bytes
    // Total: 108 bytes
}

/// 1040 Bytes — Append-only error tracking entry
#[derive(Debug, Clone)]
pub struct ErrorLedgerEntry {
    pub error_timestamp: u64,                     // 8 bytes — POSIX ms
    pub originating_node: NodeIndex,              // 2 bytes
    pub error_code_enum: u16,                     // 2 bytes
    pub error_payload_size: u32,                  // 4 bytes
    pub error_payload_buffer: [u8; 1024],         // 1024 bytes — static buffer
    // Total: 1040 bytes
}

/// 32 Bytes — System telemetry snapshot
#[derive(Debug, Clone, Copy)]
pub struct SystemMetrics {
    pub core_temperature_celsius: Celsius32,      // 4 bytes
    pub die_junction_temperature_celsius: Celsius32, // 4 bytes
    pub available_vram_bytes: MemoryBytes,        // 8 bytes
    pub total_allocatable_vram_bytes: MemoryBytes, // 8 bytes
    pub pcie_bandwidth_utilization_pct: f32,      // 4 bytes
    _padding_2: [u8; 4],                          // 4 bytes
    // Total: 32 bytes
}

/// 24 Bytes — Static analysis step coefficients
#[derive(Debug, Clone, Copy)]
pub struct StepCoefficients {
    pub estimated_tokens_input: u64,              // 8 bytes
    pub estimated_tokens_output: u64,             // 8 bytes
    pub estimated_duration_seconds: f32,          // 4 bytes
    pub kv_cache_affinity_coefficient: ScalingFactor, // 4 bytes
    // Total: 24 bytes
}

/// 56 Bytes — Dynamic execution context
#[derive(Debug, Clone, Copy)]
pub struct DynamicContext {
    pub metrics: SystemMetrics,                   // 32 bytes
    pub opex_limit_micro_usd: CurrencyUSD64,      // 8 bytes
    pub cloud_proxy_cost_per_m_tokens: CurrencyUSD64, // 8 bytes
    pub hardware_amortization_cost_per_hour: CurrencyUSD64, // 8 bytes
    // Total: 56 bytes
}

/// A single DAG step node
#[derive(Debug, Clone)]
pub struct StepNode {
    pub identifier: String,
    pub identifier_index: u16,
    pub callable_target: String,
    pub fault_policy: Option<RetryPolicy>,
}

/// Retry policy for a step node
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub strategy: RetryStrategy,
    pub backoff_factor_seconds: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetryStrategy {
    AppendDelta,
    FallbackRouting,
}

/// Execution profile enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionProfile {
    ProfileHighPerformance,
    ProfileEdgeCompute,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signature_size() {
        assert_eq!(std::mem::size_of::<ExecutionSignature>(), 68);
    }

    #[test]
    fn test_payload_size() {
        assert_eq!(std::mem::size_of::<ExecutionPayload>(), 108);
    }

    #[test]
    fn test_error_ledger_entry_size() {
        assert_eq!(std::mem::size_of::<ErrorLedgerEntry>(), 1040);
    }

    #[test]
    fn test_system_metrics_size() {
        assert_eq!(std::mem::size_of::<SystemMetrics>(), 32);
    }

    #[test]
    fn test_step_coefficients_size() {
        assert_eq!(std::mem::size_of::<StepCoefficients>(), 24);
    }

    #[test]
    fn test_dynamic_context_size() {
        assert_eq!(std::mem::size_of::<DynamicContext>(), 56);
    }
}
```

**Step 2: Register in `compiler/src/lib.rs`**

Add `pub mod struct_layouts;`

**Step 3: Run tests**

```bash
cd /opt/data/project-jarvis/compiler
cargo test struct_layouts -- --nocapture 2>&1
```

Expected: `test result: ok. 6 passed`

**Step 4: Commit**

```bash
git add compiler/src/struct_layouts.rs compiler/src/lib.rs
git commit -m "feat: add fixed-width struct layouts (E-4.Secure Section 1)"
```

---

## Task 3: Memory Management Unit — Blacklist Registry

**Objective:** Implement the blacklisted_signatures registry from E-4.Secure Section 2 with thread-safe CRUD operations

**Files:**
- Create: `compiler/src/memory_mgmt.rs`
- Modify: `compiler/src/lib.rs`

**Step 1: Create `compiler/src/memory_mgmt.rs`**

```rust
use crate::fixed_types::*;
use crate::struct_layouts::*;
use std::sync::Mutex;

// Global constants from E-4.Secure
pub const GLOBAL_MAX_NODES: usize = 4096;
pub const GLOBAL_MAX_ERRORS: usize = 16;

/// Thread-safe Volatile Storage Register (64KB per node result)
#[derive(Debug, Clone)]
pub struct VolatileStorageRegister {
    pub is_allocated: bool,
    pub allocation_size: u32,
    pub binary_data_segment: [u8; 65536],  // 64KB static buffer
}

impl Default for VolatileStorageRegister {
    fn default() -> Self {
        Self {
            is_allocated: false,
            allocation_size: 0,
            binary_data_segment: [0u8; 65536],
        }
    }
}

/// Singleton Memory Management Unit (E-4.Secure Section 2)
pub struct MemoryManagementUnit {
    /// Blacklisted structural signatures — prevents infinite loops
    pub blacklisted_signatures_registry: [ExecutionSignature; 4096],
    pub blacklisted_signatures_count: u32,

    /// Active error ledger — bounded at 16 entries
    pub active_error_ledger: [ErrorLedgerEntry; 16],

    /// Primary data lookup registry
    pub primary_data_lookup_registry: [VolatileStorageRegister; 4096],
}

impl MemoryManagementUnit {
    pub fn new() -> Self {
        Self {
            blacklisted_signatures_registry: [ExecutionSignature::zero(); 4096],
            blacklisted_signatures_count: 0,
            active_error_ledger: [ErrorLedgerEntry::zero(); 16],
            primary_data_lookup_registry: [(); 4096].map(|_| VolatileStorageRegister::default()),
        }
    }

    /// Add a signature to the blacklist. Panics if full (kernel panic).
    pub fn add_signature_to_blacklist(&mut self, signature: ExecutionSignature) {
        if self.blacklisted_signatures_count >= GLOBAL_MAX_NODES as u32 {
            panic!("MemoryAllocationPanic: Blacklist structural tracking registry capacity breached.");
        }
        self.blacklisted_signatures_registry[self.blacklisted_signatures_count as usize] = signature;
        self.blacklisted_signatures_count += 1;
    }

    /// Check if a signature is blacklisted via structural hash matching
    pub fn check_signature_blacklist(&self, signature: &ExecutionSignature) -> bool {
        for i in 0..self.blacklisted_signatures_count as usize {
            let entry = &self.blacklisted_signatures_registry[i];
            if entry.node_index == signature.node_index
                && entry.canonical_input_hash == signature.canonical_input_hash
                && entry.call_site_context_hash == signature.call_site_context_hash
            {
                return true;
            }
        }
        false
    }

    /// Remove a signature by node_index (shifts elements to stay contiguous)
    pub fn remove_signature_from_blacklist(&mut self, signature: &ExecutionSignature) {
        for i in 0..self.blacklisted_signatures_count as usize {
            if self.blacklisted_signatures_registry[i].node_index == signature.node_index {
                // Shift elements inward
                for j in i..self.blacklisted_signatures_count as usize - 1 {
                    self.blacklisted_signatures_registry[j] = self.blacklisted_signatures_registry[j + 1];
                }
                self.blacklisted_signatures_count -= 1;
                break;
            }
        }
    }
}

impl ExecutionSignature {
    pub fn zero() -> Self {
        Self {
            node_index: 0,
            _padding_0: [0; 2],
            canonical_input_hash: [0; 32],
            call_site_context_hash: [0; 32],
        }
    }
}

impl ErrorLedgerEntry {
    pub fn zero() -> Self {
        Self {
            error_timestamp: 0,
            originating_node: 0,
            error_code_enum: 0,
            error_payload_size: 0,
            error_payload_buffer: [0u8; 1024],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_check_blacklist() {
        let mut mmu = MemoryManagementUnit::new();
        let sig = ExecutionSignature {
            node_index: 42,
            _padding_0: [0; 2],
            canonical_input_hash: [1; 32],
            call_site_context_hash: [2; 32],
        };

        assert!(!mmu.check_signature_blacklist(&sig));
        mmu.add_signature_to_blacklist(sig);
        assert!(mmu.check_signature_blacklist(&sig));
    }

    #[test]
    fn test_remove_from_blacklist() {
        let mut mmu = MemoryManagementUnit::new();
        let sig1 = ExecutionSignature { node_index: 1, ..ExecutionSignature::zero() };
        let sig2 = ExecutionSignature { node_index: 2, ..ExecutionSignature::zero() };
        let sig3 = ExecutionSignature { node_index: 3, ..ExecutionSignature::zero() };

        mmu.add_signature_to_blacklist(sig1);
        mmu.add_signature_to_blacklist(sig2);
        mmu.add_signature_to_blacklist(sig3);
        assert_eq!(mmu.blacklisted_signatures_count, 3);

        mmu.remove_signature_from_blacklist(&sig2);
        assert_eq!(mmu.blacklisted_signatures_count, 2);
        // sig3 should have shifted to index 1
        assert_eq!(mmu.blacklisted_signatures_registry[1].node_index, 3);
    }

    #[test]
    #[should_panic(expected = "MemoryAllocationPanic")]
    fn test_blacklist_full_panic() {
        let mut mmu = MemoryManagementUnit::new();
        // Fill to capacity
        for i in 0..GLOBAL_MAX_NODES as u16 {
            mmu.add_signature_to_blacklist(ExecutionSignature {
                node_index: i,
                ..ExecutionSignature::zero()
            });
        }
        // One more should panic
        mmu.add_signature_to_blacklist(ExecutionSignature::zero());
    }

    #[test]
    fn test_no_match_on_different_node_index() {
        let mut mmu = MemoryManagementUnit::new();
        let sig = ExecutionSignature { node_index: 5, canonical_input_hash: [1; 32], ..ExecutionSignature::zero() };
        mmu.add_signature_to_blacklist(sig);

        let different = ExecutionSignature { node_index: 7, canonical_input_hash: [1; 32], ..ExecutionSignature::zero() };
        assert!(!mmu.check_signature_blacklist(&different));
    }
}
```

**Step 2: Register in `compiler/src/lib.rs`**

Add `pub mod memory_mgmt;`

**Step 3: Run tests**

```bash
cd /opt/data/project-jarvis/compiler
cargo test memory_mgmt -- --nocapture 2>&1
```

Expected: `test result: ok. 4 passed`

**Step 4: Commit**

```bash
git add compiler/src/memory_mgmt.rs compiler/src/lib.rs
git commit -m "feat: add memory management unit with blacklist registry (E-4.Secure Section 2)"
```

---

## Task 4: Governor Engine — Fixed-Point Cost Analysis

**Objective:** Implement the GovernorEngine from E-4.Secure Section 3 with fixed-point micro-USD cost calculations and thermal/VRAM gating

**Files:**
- Create: `compiler/src/governor_engine.rs`
- Modify: `compiler/src/lib.rs`

**Step 1: Create `compiler/src/governor_engine.rs`**

```rust
use crate::fixed_types::*;
use crate::struct_layouts::*;
use crate::memory_mgmt::MemoryManagementUnit;

/// Runtime Registry for step coefficient lookups
pub struct RuntimeRegistry;

impl RuntimeRegistry {
    /// Lookup step coefficients for a given callable target.
    /// Returns default coefficients if target is unknown.
    pub fn lookup_step_coefficients(_callable_target: &str) -> StepCoefficients {
        // Stage 1: return sensible defaults
        StepCoefficients {
            estimated_tokens_input: 1000,
            estimated_tokens_output: 500,
            estimated_duration_seconds: 0.5,
            kv_cache_affinity_coefficient: 0.5,
        }
    }
}

/// Governor Engine — routes between High-Performance and Edge Compute profiles
/// based on thermal, VRAM, and fixed-point opex calculations.
pub struct GovernorEngine;

impl GovernorEngine {
    /// Determine execution route for a set of steps (E-4.Secure Section 3)
    pub fn determine_execution_route(
        dynamic_context: &DynamicContext,
        _execution_frame_steps: &[StepNode],  // steps for cost projection
        active_step_count: u16,
    ) -> Result<ExecutionProfile, String> {
        // -----------------------------------------------------------------------
        // GATE 1: THERMAL CIRCUIT BREAKER
        // -----------------------------------------------------------------------
        if dynamic_context.metrics.core_temperature_celsius > 78.0
            || dynamic_context.metrics.die_junction_temperature_celsius > 85.0
        {
            return Ok(ExecutionProfile::ProfileEdgeCompute);
        }

        // -----------------------------------------------------------------------
        // GATE 2: HARDWARE CAPACITY ASSERTION (VRAM)
        // -----------------------------------------------------------------------
        const BASE_MEMORY_OVERHEAD: u64 = 4_294_967_296; // 4GB
        if dynamic_context.metrics.available_vram_bytes < BASE_MEMORY_OVERHEAD {
            return Ok(ExecutionProfile::ProfileEdgeCompute);
        }

        // -----------------------------------------------------------------------
        // GATE 3: FIXED-POINT OPEX CALCULATION
        // -----------------------------------------------------------------------
        let mut cumulative_projected_tokens: u64 = 0;
        let mut cumulative_projected_duration_ms: u64 = 0;

        // Sum up costs across all active steps
        for i in 0..active_step_count as usize {
            if i >= _execution_frame_steps.len() {
                break;
            }
            let node = &_execution_frame_steps[i];
            let coeff = RuntimeRegistry::lookup_step_coefficients(&node.callable_target);

            // Fixed-point: kv_cache_affinity reduces effective token count
            let scaled_affinity = 1.0f32 - coeff.kv_cache_affinity_coefficient;
            let running_input = (coeff.estimated_tokens_input as f32 * scaled_affinity) as u64;

            cumulative_projected_tokens += running_input + coeff.estimated_tokens_output;
            cumulative_projected_duration_ms += (coeff.estimated_duration_seconds * 1000.0) as u64;
        }

        // Token Cost = (Tokens * Cost-Per-Million) / 1,000,000
        let token_cost = (cumulative_projected_tokens as i64
            * dynamic_context.cloud_proxy_cost_per_m_tokens)
            / 1_000_000;

        // Compute Cost = (Duration MS * Amortization Per Hour) / 3,600,000
        let compute_cost = (cumulative_projected_duration_ms as i64
            * dynamic_context.hardware_amortization_cost_per_hour)
            / 3_600_000;

        let total_opex = token_cost + compute_cost;

        if total_opex > dynamic_context.opex_limit_micro_usd {
            let limit_usd = dynamic_context.opex_limit_micro_usd as f64 / 1_000_000.0;
            let actual_usd = total_opex as f64 / 1_000_000.0;
            return Err(format!(
                "Hosting architecture unit value exceeds the forced allocation ceiling. Limit: USD {:.6}, Projected Cost: USD {:.6}",
                limit_usd, actual_usd
            ));
        }

        Ok(ExecutionProfile::ProfileHighPerformance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test: Optimal conditions route to High Performance
    #[test]
    fn test_nominal_conditions_high_performance() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.2,
            die_junction_temperature_celsius: 41.5,
            available_vram_bytes: 68_719_476_736, // 64GB
            total_allocatable_vram_bytes: 68_719_476_736,
            pcie_bandwidth_utilization_pct: 0.05,
            _padding_2: [0; 4],
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 1_000_000,       // $1.00
            cloud_proxy_cost_per_m_tokens: 100,     // $0.0001/million
            hardware_amortization_cost_per_hour: 50, // $0.00005/hr
        };
        let steps = [StepNode {
            identifier: "test".into(),
            identifier_index: 1,
            callable_target: "TestEngine".into(),
            fault_policy: None,
        }];

        let result = GovernorEngine::determine_execution_route(&ctx, &steps, 1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ExecutionProfile::ProfileHighPerformance);
    }

    /// Test: Thermal breach forces Edge Compute
    #[test]
    fn test_thermal_breach_routes_to_edge() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 80.0,  // Above 78°C
            ..Default::default()
        };
        let ctx = DynamicContext {
            metrics,
            ..Default::default()
        };

        let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
        assert_eq!(result.unwrap(), ExecutionProfile::ProfileEdgeCompute);
    }

    /// Test: Insufficient VRAM forces Edge Compute
    #[test]
    fn test_low_vram_routes_to_edge() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 1_073_741_824,  // 1GB — below 4GB threshold
            ..Default::default()
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 1_000_000,
            ..Default::default()
        };

        let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
        assert_eq!(result.unwrap(), ExecutionProfile::ProfileEdgeCompute);
    }

    /// Test: Opex breach returns error with exact contract string
    #[test]
    fn test_opex_breach_returns_error() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 68_719_476_736,
            ..Default::default()
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 100,    // Very tight budget
            cloud_proxy_cost_per_m_tokens: 500_000,  // Expensive tokens
            hardware_amortization_cost_per_hour: 500_000,
        };
        let steps = [StepNode {
            identifier: "big_job".into(),
            identifier_index: 1,
            callable_target: "ExpensiveEngine".into(),
            fault_policy: None,
        }];

        let result = GovernorEngine::determine_execution_route(&ctx, &steps, 1);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_lowercase();
        assert!(
            err_msg.contains("hosting architecture unit value exceeds the forced allocation ceiling"),
            "Error message does not match spec contract: {}",
            err_msg
        );
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 68_719_476_736,
            total_allocatable_vram_bytes: 68_719_476_736,
            pcie_bandwidth_utilization_pct: 0.05,
            _padding_2: [0; 4],
        }
    }
}

impl Default for DynamicContext {
    fn default() -> Self {
        Self {
            metrics: SystemMetrics::default(),
            opex_limit_micro_usd: 500_000,
            cloud_proxy_cost_per_m_tokens: 100,
            hardware_amortization_cost_per_hour: 50,
        }
    }
}
```

**Step 2: Register in `compiler/src/lib.rs`**

Add `pub mod governor_engine;`

**Step 3: Run tests**

```bash
cd /opt/data/project-jarvis/compiler
cargo test governor_engine -- --nocapture 2>&1
```

Expected: `test result: ok. 4 passed`

**Step 4: Commit**

```bash
git add compiler/src/governor_engine.rs compiler/src/lib.rs
git commit -m "feat: add governor engine with fixed-point opex (E-4.Secure Section 3)"
```

---

## Task 5: Runtime Executor — DAG Step Processor

**Objective:** Implement the RuntimeExecutor with idempotency checking, retry with exponential backoff, and fallback routing

**Files:**
- Create: `compiler/src/runtime_executor.rs`
- Modify: `compiler/src/lib.rs`
- Modify: `compiler/src/memory_mgmt.rs` (add error deltas to ExecutionPayload)

**Step 1: Add error_deltas field to ExecutionPayload in `struct_layouts.rs`**

Add `pub error_deltas: Vec<String>` to ExecutionPayload (dynamic in Rust, but matches the spec's append semantics).

**Step 2: Create `compiler/src/runtime_executor.rs`**

```rust
use crate::fixed_types::*;
use crate::struct_layouts::*;
use crate::memory_mgmt::MemoryManagementUnit;
use crate::governor_engine::GovernorEngine;

/// Result type for inference calls
#[derive(Debug)]
pub struct InferenceResponse {
    pub status_enum: u8,  // 0=SUCCESS, 1=SCHEMA_VALIDATION_ERROR
    pub payload_size: u32,
    pub raw_bytes: [u8; 65536],
    pub metadata_size: u32,
    pub metadata_bytes: [u8; 1024],
}

/// The DAG step execution engine (E-4.Secure Section 4)
pub struct RuntimeExecutor;

impl RuntimeExecutor {
    /// Execute a single linear step with idempotency and retry logic
    pub fn execute_linear_step(
        payload: &mut ExecutionPayload,
        dynamic_context: &DynamicContext,
        step_data: &StepNode,
        mmu: &mut MemoryManagementUnit,
    ) {
        // ---------------------------------------------------------------
        // 1. ASSERT IDEMPOTENCY CONTRACT
        // ---------------------------------------------------------------
        if mmu.check_signature_blacklist(&payload.signature) {
            Self::handle_halt(
                "DeterministicIdempotencyViolation",
                &format!(
                    "Repeated execution of blocked structural signature at step identifier: {}",
                    step_data.identifier
                ),
                &payload.signature,
            );
            return;
        }

        // ---------------------------------------------------------------
        // 2. QUERY GOVERNOR FOR ROUTING
        // ---------------------------------------------------------------
        let evaluation_array = [step_data.clone()];  // single-step eval
        let route_result = GovernorEngine::determine_execution_route(
            dynamic_context,
            &evaluation_array,
            1,
        );

        let selected_profile = match route_result {
            Err(msg) => {
                Self::handle_halt("FinancialBudgetBreach", &msg, &payload.signature);
                return;
            }
            Ok(profile) => profile,
        };

        // ---------------------------------------------------------------
        // 3. EXECUTE INFERENCE STEP (stub — connects to inference engine)
        // ---------------------------------------------------------------
        let inference = Self::invoke_inference_engine(
            selected_profile,
            &step_data.callable_target,
            &payload.error_deltas,
        );

        // ---------------------------------------------------------------
        // 4. PROCESS OUTCOME
        // ---------------------------------------------------------------
        match inference.status_enum {
            0 => {
                // SUCCESS: store result in data lookup registry
                let idx = step_data.identifier_index as usize;
                mmu.primary_data_lookup_registry[idx].is_allocated = true;
                mmu.primary_data_lookup_registry[idx].allocation_size = inference.payload_size;
                mmu.primary_data_lookup_registry[idx].binary_data_segment = inference.raw_bytes;
            }
            1 => {
                // SCHEMA_VALIDATION_ERROR: handle retry/fallback
                mmu.add_signature_to_blacklist(payload.signature);

                if let Some(ref policy) = step_data.fault_policy {
                    if payload.retry_attempt < policy.max_retries
                        && policy.strategy == RetryStrategy::AppendDelta
                    {
                        // Write to error ledger
                        let active_idx =
                            (mmu.blacklisted_signatures_count as usize) % crate::memory_mgmt::GLOBAL_MAX_ERRORS;
                        mmu.active_error_ledger[active_idx].error_timestamp =
                            std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as u64;
                        mmu.active_error_ledger[active_idx].originating_node =
                            step_data.identifier_index;
                        mmu.active_error_ledger[active_idx].error_payload_size =
                            inference.metadata_size;
                        mmu.active_error_ledger[active_idx].error_payload_buffer =
                            inference.metadata_bytes;

                        // Mutate payload for retry
                        payload.retry_attempt += 1;
                        let delta_str = String::from_utf8_lossy(
                            &inference.metadata_bytes[..inference.metadata_size as usize],
                        )
                        .to_string();
                        payload.error_deltas.push(delta_str);

                        // Update compiled payload hash
                        let combined: Vec<u8> = payload
                            .signature
                            .canonical_input_hash
                            .iter()
                            .chain(inference.metadata_bytes[..inference.metadata_size as usize].iter())
                            .copied()
                            .collect();
                        payload.compiled_payload_hash = sha2_hash(&combined);

                        // Remove blacklist to allow retry
                        mmu.remove_signature_from_blacklist(&payload.signature);

                        // Exponential backoff
                        let backoff_ms =
                            (policy.backoff_factor_seconds * 1000.0 * (2u64.pow(payload.retry_attempt) as f32))
                                as u32;
                        std::thread::sleep(std::time::Duration::from_millis(backoff_ms as u64));

                        // Recurse
                        Self::execute_linear_step(payload, dynamic_context, step_data, mmu);
                        return;
                    } else if policy.strategy == RetryStrategy::FallbackRouting
                        && selected_profile == ExecutionProfile::ProfileHighPerformance
                    {
                        // Fallback: strip VRAM to force edge compute
                        mmu.remove_signature_from_blacklist(&payload.signature);
                        let mut degenerate = *dynamic_context;
                        degenerate.metrics.available_vram_bytes = 0;
                        Self::execute_linear_step(payload, &degenerate, step_data, mmu);
                        return;
                    }
                }

                // Exhausted retries or unknown policy
                Self::handle_halt(
                    "RetryBoundaryExhaustion",
                    "Node processing failed. Threshold limit parameters breached for step policy strategy configuration.",
                    &payload.signature,
                );
            }
            _ => {}
        }
    }

    /// Inference engine stub — replace with real LLM calls in Phase 2
    fn invoke_inference_engine(
        _profile: ExecutionProfile,
        _target: &str,
        _error_deltas: &[String],
    ) -> InferenceResponse {
        // Stub: always returns SUCCESS
        InferenceResponse {
            status_enum: 0,
            payload_size: 0,
            raw_bytes: [0u8; 65536],
            metadata_size: 0,
            metadata_bytes: [0u8; 1024],
        }
    }

    /// HALT handler — system critical error
    fn handle_halt(error_code: &str, reason: &str, signature: &ExecutionSignature) {
        eprintln!("[CRITICAL SYSTEM HALT INTERCEPTED]");
        eprintln!("ERROR VALUE: {}", error_code);
        eprintln!("REASON     : {}", reason);
        eprintln!("SIGNATURE  : node={}, hash={:?}",
            signature.node_index,
            &signature.canonical_input_hash[..4]
        );
    }
}

/// Compute SHA-256 hash of a byte slice
fn sha2_hash(data: &[u8]) -> Hash256 {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory_mgmt::MemoryManagementUnit;

    #[test]
    fn test_idempotency_block() {
        let mut mmu = MemoryManagementUnit::new();
        let sig = ExecutionSignature {
            node_index: 1,
            _padding_0: [0; 2],
            canonical_input_hash: [1; 32],
            call_site_context_hash: [2; 32],
        };
        mmu.add_signature_to_blacklist(sig);

        let mut payload = ExecutionPayload {
            signature: sig,
            retry_attempt: 0,
            compiled_payload_hash: [0; 32],
            error_ledger_count: 0,
            _padding_1: [0; 2],
            error_deltas: vec![],
        };

        let ctx = DynamicContext::default();
        let step = StepNode {
            identifier: "test".into(),
            identifier_index: 1,
            callable_target: "TestEngine".into(),
            fault_policy: None,
        };

        // Should not panic — handle_halt prints and returns
        RuntimeExecutor::execute_linear_step(&mut payload, &ctx, &step, &mut mmu);
        // Idempotency preserved — no crash
    }

    #[test]
    fn test_sha2_hash_length() {
        let hash = sha2_hash(b"hello world");
        assert_eq!(hash.len(), 32);
    }
}
```

**Step 3: Add sha2 dependency to `compiler/Cargo.toml`**

Add `sha2 = "0.10"` under `[dependencies]`.

**Step 4: Update `compiler/src/lib.rs`**

Add `pub mod runtime_executor;`

**Step 5: Build and test**

```bash
cd /opt/data/project-jarvis/compiler
cargo build 2>&1 && cargo test runtime_executor -- --nocapture 2>&1
```

Expected: clean build, 2 tests passed

**Step 6: Commit**

```bash
git add compiler/src/runtime_executor.rs compiler/src/lib.rs compiler/src/struct_layouts.rs compiler/Cargo.toml
git commit -m "feat: add runtime executor with idempotency and retry logic (E-4.Secure Section 4)"
```

---

## Task 6: CLI Integration — `jrvc --governor` Flag

**Objective:** Add a CLI command to `jrvc` that runs the GovernorEngine with specified metrics, returning JSON output

**Files:**
- Modify: `compiler/src/main.rs`

**Step 1: Add `--governor` flag handler to `compiler/src/main.rs`**

Add to `main()` before the existing file-parsing logic:

```rust
    if args.contains(&"--governor".to_string()) {
        run_governor_cli();
        return;
    }
```

**Step 2: Implement `run_governor_cli()`**

```rust
fn run_governor_cli() {
    use jarvis_compiler::governor_engine::GovernorEngine;
    use jarvis_compiler::struct_layouts::*;

    let metrics = SystemMetrics {
        core_temperature_celsius: 37.2,
        die_junction_temperature_celsius: 41.5,
        available_vram_bytes: 68_719_476_736,
        total_allocatable_vram_bytes: 68_719_476_736,
        pcie_bandwidth_utilization_pct: 0.05,
        _padding_2: [0; 4],
    };

    let ctx = DynamicContext {
        metrics,
        opex_limit_micro_usd: 500_000,
        cloud_proxy_cost_per_m_tokens: 100,
        hardware_amortization_cost_per_hour: 50,
    };

    let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
    match result {
        Ok(profile) => println!(
            r#"{{"profile":"{:?}","status":"ok"}}"#,
            profile
        ),
        Err(msg) => println!(
            r#"{{"profile":"HALT","status":"error","message":"{}"}}"#,
            msg
        ),
    }
}
```

**Step 3: Build and test**

```bash
cd /opt/data/project-jarvis/compiler
cargo build 2>&1
./target/debug/jrvc --governor 2>&1
```

Expected: `{"profile":"ProfileHighPerformance","status":"ok"}`

**Step 4: Commit**

```bash
git add compiler/src/main.rs
git commit -m "feat: add --governor CLI flag for runtime routing diagnostics"
```

---

## Task 7: Formal Verification Test Suite

**Objective:** Implement the full E-4.Secure test suite matching the spec's exact assertion contracts

**Files:**
- Create: `compiler/tests/governor_e2e_tests.rs`

**Step 1: Create `compiler/tests/governor_e2e_tests.rs`**

```rust
use jarvis_compiler::governor_engine::GovernorEngine;
use jarvis_compiler::struct_layouts::*;

/// Test: API failure triggers fallback budget (CAPEX_LIMIT_BREACH)
#[test]
fn test_fallback_budget_enforcement_on_api_failure() {
    // Arrange: Metrics look optimal, but the platform is expensive
    let mock_metrics = SystemMetrics {
        core_temperature_celsius: 37.2,
        die_junction_temperature_celsius: 41.5,
        available_vram_bytes: 68_719_476_736, // 64GB
        total_allocatable_vram_bytes: 68_719_476_736,
        pcie_bandwidth_utilization_pct: 0.05,
        _padding_2: [0; 4],
    };

    // Budget is tight enough that any non-trivial step exceeds opex
    let ctx = DynamicContext {
        metrics: mock_metrics,
        opex_limit_micro_usd: 10,       // Extremely tight
        cloud_proxy_cost_per_m_tokens: 5_000_000,  // $5/million tokens
        hardware_amortization_cost_per_hour: 5_000_000,
    };

    let mock_steps = [
        StepNode {
            identifier: "sys_audit_0".into(),
            identifier_index: 101,
            callable_target: "DeepCoreAnalysisEngine".into(),
            fault_policy: None,
        },
        StepNode {
            identifier: "sys_audit_1".into(),
            identifier_index: 102,
            callable_target: "ComplianceValidationEngine".into(),
            fault_policy: None,
        },
    ];

    // Act
    let result = GovernorEngine::determine_execution_route(&ctx, &mock_steps, 2);

    // Assert: Must return error with exact contract substring
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_lowercase();
    assert!(
        err_msg.contains("hosting architecture unit value exceeds the forced allocation ceiling"),
        "Error message does not match spec contract. Got: {}",
        err_msg
    );
}

/// Test: Absolute economic halt when costs are extreme
#[test]
fn test_absolute_economic_halt() {
    let metrics = SystemMetrics {
        core_temperature_celsius: 37.0,
        die_junction_temperature_celsius: 40.0,
        available_vram_bytes: 24 * 1024 * 1024 * 1024, // 24GB
        total_allocatable_vram_bytes: 24 * 1024 * 1024 * 1024,
        pcie_bandwidth_utilization_pct: 0.1,
        _padding_2: [0; 4],
    };

    let ctx = DynamicContext {
        metrics,
        opex_limit_micro_usd: 10,       // $0.00001 — nearly zero
        cloud_proxy_cost_per_m_tokens: 15_000_000,   // $15/million tokens
        hardware_amortization_cost_per_hour: 10_000_000,
    };

    let steps = [StepNode {
        identifier: "big_analysis".into(),
        identifier_index: 1,
        callable_target: "ExpensiveModel".into(),
        fault_policy: None,
    }];

    let result = GovernorEngine::determine_execution_route(&ctx, &steps, 1);
    assert!(
        result.is_err(),
        "Expected economic halt but got Ok: {:?}",
        result
    );
}
```

**Step 2: Run tests**

```bash
cd /opt/data/project-jarvis/compiler
cargo test --test governor_e2e_tests -- --nocapture 2>&1
```

Expected: `test result: ok. 2 passed`

**Step 3: Run full test suite**

```bash
cd /opt/data/project-jarvis/compiler
cargo test 2>&1
```

Expected: All tests pass (unit + integration)

**Step 4: Commit**

```bash
git add compiler/tests/governor_e2e_tests.rs
git commit -m "test: add formal verification test suite (E-4.Secure Section 5)"
```

---

## Summary

After these 7 tasks, you'll have:

| Task | What's Built | Status |
|------|-------------|--------|
| 1 | Fixed-width type aliases (Hash256, NodeIndex, etc.) | Done |
| 2 | Struct layouts with exact sizes and padding | Done |
| 3 | Memory Management Unit with blacklist registry | Done |
| 4 | Governor Engine with fixed-point opex calculations | Done |
| 5 | Runtime Executor with idempotency and retry | Done |
| 6 | `jrvc --governor` CLI diagnostic | Done |
| 7 | Formal verification test suite | Done |

**What's Next (Phase 2):**
- New EBNF grammar parser (workflow/step/gate/on_fail)
- Real inference engine binding
- Graph-level SHA-256 memoization
- KV cache anchoring implementation
- JRV DSL production files using the new syntax
- Self-hosting compiler bootstrapping