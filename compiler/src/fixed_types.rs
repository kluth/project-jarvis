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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash256_size() {
        assert_eq!(std::mem::size_of::<Hash256>(), 32);
    }

    #[test]
    fn test_node_index_size() {
        assert_eq!(std::mem::size_of::<NodeIndex>(), 2);
    }

    #[test]
    fn test_currency_usd64_size() {
        assert_eq!(std::mem::size_of::<CurrencyUSD64>(), 8);
    }

    #[test]
    fn test_type_coherence() {
        let hash: Hash256 = [0u8; 32];
        assert_eq!(hash.len(), 32);
    }
}
