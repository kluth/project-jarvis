use crate::struct_layouts::*;

pub const GLOBAL_MAX_NODES: usize = 4096;
pub const GLOBAL_MAX_ERRORS: usize = 16;

/// Thread-safe Volatile Storage Register (64KB per node result)
#[derive(Debug, Clone)]
pub struct VolatileStorageRegister {
    pub is_allocated: bool,
    pub allocation_size: u32,
    pub binary_data_segment: [u8; 65536],
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
    pub blacklisted_signatures_registry: Box<[ExecutionSignature; 4096]>,
    pub blacklisted_signatures_count: u32,
    pub active_error_ledger: Box<[ErrorLedgerEntry; 16]>,
    pub primary_data_lookup_registry: Vec<VolatileStorageRegister>,
}

impl MemoryManagementUnit {
    pub fn new() -> Self {
        Self {
            blacklisted_signatures_registry: Box::new([ExecutionSignature::zero(); 4096]),
            blacklisted_signatures_count: 0,
            active_error_ledger: Box::new([ErrorLedgerEntry::zero(); 16]),
            primary_data_lookup_registry: {
                let mut v = Vec::with_capacity(4096);
                for _ in 0..4096 {
                    v.push(VolatileStorageRegister::default());
                }
                v
            },
        }
    }

    pub fn add_signature_to_blacklist(&mut self, signature: ExecutionSignature) {
        if self.blacklisted_signatures_count >= GLOBAL_MAX_NODES as u32 {
            panic!("MemoryAllocationPanic: Blacklist structural tracking registry capacity breached.");
        }
        self.blacklisted_signatures_registry[self.blacklisted_signatures_count as usize] = signature;
        self.blacklisted_signatures_count += 1;
    }

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

    pub fn remove_signature_from_blacklist(&mut self, signature: &ExecutionSignature) {
        for i in 0..self.blacklisted_signatures_count as usize {
            if self.blacklisted_signatures_registry[i].node_index == signature.node_index {
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
        assert_eq!(mmu.blacklisted_signatures_registry[1].node_index, 3);
    }

    #[test]
    #[should_panic(expected = "MemoryAllocationPanic")]
    fn test_blacklist_full_panic() {
        let mut mmu = MemoryManagementUnit::new();
        for i in 0..GLOBAL_MAX_NODES as u16 {
            mmu.add_signature_to_blacklist(ExecutionSignature {
                node_index: i,
                ..ExecutionSignature::zero()
            });
        }
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