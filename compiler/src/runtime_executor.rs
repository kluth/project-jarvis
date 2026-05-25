use crate::struct_layouts::*;
use crate::memory_mgmt::{MemoryManagementUnit, GLOBAL_MAX_ERRORS};
use crate::governor_engine::GovernorEngine;
use crate::fixed_types::Hash256;

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
        // 1. ASSERT IDEMPOTENCY CONTRACT
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

        // 2. QUERY GOVERNOR FOR ROUTING
        let evaluation_array = [step_data.clone()];
        let route_result = GovernorEngine::determine_execution_route(
            dynamic_context, &evaluation_array, 1,
        );

        let selected_profile = match route_result {
            Err(msg) => {
                Self::handle_halt("FinancialBudgetBreach", &msg, &payload.signature);
                return;
            }
            Ok(profile) => profile,
        };

        // 3. EXECUTE INFERENCE STEP (stub)
        let inference = Self::invoke_inference_engine(
            selected_profile, &step_data.callable_target, &payload.error_deltas,
        );

        // 4. PROCESS OUTCOME
        match inference.status_enum {
            0 => {
                // SUCCESS
                let idx = step_data.identifier_index as usize;
                if idx < mmu.primary_data_lookup_registry.len() {
                    mmu.primary_data_lookup_registry[idx].is_allocated = true;
                    mmu.primary_data_lookup_registry[idx].allocation_size = inference.payload_size;
                    mmu.primary_data_lookup_registry[idx].binary_data_segment = inference.raw_bytes;
                }
            }
            1 => {
                // SCHEMA_VALIDATION_ERROR: handle retry/fallback
                mmu.add_signature_to_blacklist(payload.signature);

                if let Some(ref policy) = step_data.fault_policy {
                    if payload.retry_attempt < policy.max_retries
                        && policy.strategy == RetryStrategy::AppendDelta
                    {
                        let active_idx =
                            (mmu.blacklisted_signatures_count as usize) % GLOBAL_MAX_ERRORS;
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

                        payload.retry_attempt += 1;
                        let delta_str = String::from_utf8_lossy(
                            &inference.metadata_bytes[..inference.metadata_size as usize],
                        ).to_string();
                        payload.error_deltas.push(delta_str);

                        let combined: Vec<u8> = payload.signature.canonical_input_hash.iter()
                            .chain(inference.metadata_bytes[..inference.metadata_size as usize].iter())
                            .copied().collect();
                        payload.compiled_payload_hash = sha2_hash(&combined);

                        mmu.remove_signature_from_blacklist(&payload.signature);

                        let backoff_ms =
                            (policy.backoff_factor_seconds * 1000.0 * (2u64.pow(payload.retry_attempt) as f32)) as u32;
                        std::thread::sleep(std::time::Duration::from_millis(backoff_ms as u64));

                        Self::execute_linear_step(payload, dynamic_context, step_data, mmu);
                        return;
                    } else if policy.strategy == RetryStrategy::FallbackRouting
                        && selected_profile == ExecutionProfile::ProfileHighPerformance
                    {
                        mmu.remove_signature_from_blacklist(&payload.signature);
                        let mut degenerate = *dynamic_context;
                        degenerate.metrics.available_vram_bytes = 0;
                        Self::execute_linear_step(payload, &degenerate, step_data, mmu);
                        return;
                    }
                }

                Self::handle_halt(
                    "RetryBoundaryExhaustion",
                    "Node processing failed. Threshold limit parameters breached for step policy strategy configuration.",
                    &payload.signature,
                );
            }
            _ => {}
        }
    }

    fn invoke_inference_engine(
        _profile: ExecutionProfile, _target: &str, _error_deltas: &[String],
    ) -> InferenceResponse {
        InferenceResponse {
            status_enum: 0, payload_size: 0, raw_bytes: [0u8; 65536],
            metadata_size: 0, metadata_bytes: [0u8; 1024],
        }
    }

    fn handle_halt(error_code: &str, reason: &str, signature: &ExecutionSignature) {
        eprintln!("[CRITICAL SYSTEM HALT INTERCEPTED]");
        eprintln!("ERROR VALUE: {}", error_code);
        eprintln!("REASON     : {}", reason);
        eprintln!("SIGNATURE  : node={}, hash={:?}",
            signature.node_index, &signature.canonical_input_hash[..4]);
    }
}

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
    fn test_idempotency_block_no_panic() {
        let mut mmu = MemoryManagementUnit::new();
        let sig = ExecutionSignature {
            node_index: 1, _padding_0: [0; 2],
            canonical_input_hash: [1; 32], call_site_context_hash: [2; 32],
        };
        mmu.add_signature_to_blacklist(sig);

        let mut payload = ExecutionPayload {
            signature: sig, retry_attempt: 0, compiled_payload_hash: [0; 32],
            error_ledger_count: 0, _padding_1: [0; 2], error_deltas: vec![],
        };
        let ctx = DynamicContext::default();
        let step = StepNode {
            identifier: "test".into(), identifier_index: 1,
            callable_target: "TestEngine".into(), fault_policy: None,
        };

        // Should not panic—handle_halt prints and returns
        RuntimeExecutor::execute_linear_step(&mut payload, &ctx, &step, &mut mmu);
    }

    #[test]
    fn test_sha2_hash_length() {
        let hash = sha2_hash(b"hello world");
        assert_eq!(hash.len(), 32);
    }
}