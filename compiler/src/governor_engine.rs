use crate::struct_layouts::*;

/// Runtime Registry for step coefficient lookups
pub struct RuntimeRegistry;

impl RuntimeRegistry {
    pub fn lookup_step_coefficients(_callable_target: &str) -> StepCoefficients {
        StepCoefficients {
            estimated_tokens_input: 1000,
            estimated_tokens_output: 500,
            estimated_duration_seconds: 0.5,
            kv_cache_affinity_coefficient: 0.5,
        }
    }
}

/// Governor Engine — routes between High-Performance and Edge Compute profiles
pub struct GovernorEngine;

impl GovernorEngine {
    pub fn determine_execution_route(
        dynamic_context: &DynamicContext,
        _execution_frame_steps: &[StepNode],
        active_step_count: u16,
    ) -> Result<ExecutionProfile, String> {
        // GATE 1: THERMAL CIRCUIT BREAKER
        if dynamic_context.metrics.core_temperature_celsius > 78.0
            || dynamic_context.metrics.die_junction_temperature_celsius > 85.0
        {
            return Ok(ExecutionProfile::ProfileEdgeCompute);
        }

        // GATE 2: HARDWARE CAPACITY (VRAM)
        const BASE_MEMORY_OVERHEAD: u64 = 4_294_967_296; // 4GB
        if dynamic_context.metrics.available_vram_bytes < BASE_MEMORY_OVERHEAD {
            return Ok(ExecutionProfile::ProfileEdgeCompute);
        }

        // GATE 3: FIXED-POINT OPEX CALCULATION
        let mut cumulative_projected_tokens: u64 = 0;
        let mut cumulative_projected_duration_ms: u64 = 0;

        for i in 0..active_step_count as usize {
            if i >= _execution_frame_steps.len() {
                break;
            }
            let node = &_execution_frame_steps[i];
            let coeff = RuntimeRegistry::lookup_step_coefficients(&node.callable_target);
            let scaled_affinity = 1.0f32 - coeff.kv_cache_affinity_coefficient;
            let running_input = (coeff.estimated_tokens_input as f32 * scaled_affinity) as u64;
            cumulative_projected_tokens += running_input + coeff.estimated_tokens_output;
            cumulative_projected_duration_ms += (coeff.estimated_duration_seconds * 1000.0) as u64;
        }

        let token_cost = (cumulative_projected_tokens as i64
            * dynamic_context.cloud_proxy_cost_per_m_tokens)
            / 1_000_000;
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

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 68_719_476_736,
            total_allocatable_vram_bytes: 68_719_476_736,
            pcie_bandwidth_utilization_pct: 0.05,
            _padding_2: [0u8; 4],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_high_performance() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.2,
            die_junction_temperature_celsius: 41.5,
            available_vram_bytes: 68_719_476_736,
            total_allocatable_vram_bytes: 68_719_476_736,
            pcie_bandwidth_utilization_pct: 0.05,
            _padding_2: [0u8; 4],
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 1_000_000,
            cloud_proxy_cost_per_m_tokens: 100,
            hardware_amortization_cost_per_hour: 50,
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

    #[test]
    fn test_thermal_breach_routes_to_edge() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 80.0,
            ..SystemMetrics::default()
        };
        let ctx = DynamicContext {
            metrics,
            ..DynamicContext::default()
        };
        let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
        assert_eq!(result.unwrap(), ExecutionProfile::ProfileEdgeCompute);
    }

    #[test]
    fn test_low_vram_routes_to_edge() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 1_073_741_824,
            ..SystemMetrics::default()
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 1_000_000,
            ..DynamicContext::default()
        };
        let result = GovernorEngine::determine_execution_route(&ctx, &[], 0);
        assert_eq!(result.unwrap(), ExecutionProfile::ProfileEdgeCompute);
    }

    #[test]
    fn test_opex_breach_returns_error() {
        let metrics = SystemMetrics {
            core_temperature_celsius: 37.0,
            die_junction_temperature_celsius: 40.0,
            available_vram_bytes: 68_719_476_736,
            ..SystemMetrics::default()
        };
        let ctx = DynamicContext {
            metrics,
            opex_limit_micro_usd: 100,
            cloud_proxy_cost_per_m_tokens: 500_000,
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
