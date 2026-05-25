use jarvis_compiler::governor_engine::GovernorEngine;
use jarvis_compiler::struct_layouts::*;

/// Test: API failure triggers fallback budget with exact contract string match
#[test]
fn test_fallback_budget_enforcement_on_api_failure() {
    let mock_metrics = SystemMetrics {
        core_temperature_celsius: 37.2,
        die_junction_temperature_celsius: 41.5,
        available_vram_bytes: 68_719_476_736,
        total_allocatable_vram_bytes: 68_719_476_736,
        pcie_bandwidth_utilization_pct: 0.05,
        _padding_2: [0u8; 4],
    };

    let ctx = DynamicContext {
        metrics: mock_metrics,
        opex_limit_micro_usd: 10,
        cloud_proxy_cost_per_m_tokens: 5_000_000,
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

    let result = GovernorEngine::determine_execution_route(&ctx, &mock_steps, 2);
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
        available_vram_bytes: 24 * 1024 * 1024 * 1024,
        total_allocatable_vram_bytes: 24 * 1024 * 1024 * 1024,
        pcie_bandwidth_utilization_pct: 0.1,
        _padding_2: [0u8; 4],
    };

    let ctx = DynamicContext {
        metrics,
        opex_limit_micro_usd: 10,
        cloud_proxy_cost_per_m_tokens: 15_000_000,
        hardware_amortization_cost_per_hour: 10_000_000,
    };

    let steps = [StepNode {
        identifier: "big_analysis".into(),
        identifier_index: 1,
        callable_target: "ExpensiveModel".into(),
        fault_policy: None,
    }];

    let result = GovernorEngine::determine_execution_route(&ctx, &steps, 1);
    assert!(result.is_err(), "Expected economic halt but got Ok: {:?}", result);
}