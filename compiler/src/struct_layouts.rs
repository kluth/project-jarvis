use crate::fixed_types::*;

/// 68 Bytes — Invariant tracking identification signature
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionSignature {
    pub node_index: NodeIndex,
    pub _padding_0: [u8; 2],
    pub canonical_input_hash: Hash256,
    pub call_site_context_hash: Hash256,
}

/// 108 Bytes — Execution payload with retry tracking
#[derive(Debug, Clone)]
pub struct ExecutionPayload {
    pub signature: ExecutionSignature,
    pub retry_attempt: u32,
    pub compiled_payload_hash: Hash256,
    pub error_ledger_count: u16,
    _padding_1: [u8; 2],
}

/// 1040 Bytes — Append-only error tracking entry
#[derive(Debug, Clone, Copy)]
pub struct ErrorLedgerEntry {
    pub error_timestamp: u64,
    pub originating_node: NodeIndex,
    pub error_code_enum: u16,
    pub error_payload_size: u32,
    pub error_payload_buffer: [u8; 1024],
}

/// 32 Bytes — System telemetry snapshot
#[derive(Debug, Clone, Copy)]
pub struct SystemMetrics {
    pub core_temperature_celsius: Celsius32,
    pub die_junction_temperature_celsius: Celsius32,
    pub available_vram_bytes: MemoryBytes,
    pub total_allocatable_vram_bytes: MemoryBytes,
    pub pcie_bandwidth_utilization_pct: f32,
    pub _padding_2: [u8; 4],
}

/// 24 Bytes — Static analysis step coefficients
#[derive(Debug, Clone, Copy)]
pub struct StepCoefficients {
    pub estimated_tokens_input: u64,
    pub estimated_tokens_output: u64,
    pub estimated_duration_seconds: f32,
    pub kv_cache_affinity_coefficient: ScalingFactor,
}

/// 56 Bytes — Dynamic execution context
#[derive(Debug, Clone, Copy)]
pub struct DynamicContext {
    pub metrics: SystemMetrics,
    pub opex_limit_micro_usd: CurrencyUSD64,
    pub cloud_proxy_cost_per_m_tokens: CurrencyUSD64,
    pub hardware_amortization_cost_per_hour: CurrencyUSD64,
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