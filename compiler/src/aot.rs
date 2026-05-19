//! Jarvis-ISA: An energy-modeled Instruction Set Architecture.
//! Designed for absolute EFDD compliance.

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// O(1) Load immediate into SIMD register.
    LoadImm(f32),
    /// O(1) Vector Addition (EFDD: High efficiency-per-joule).
    VecAdd,
    /// O(1) Vector Multiplication.
    VecMul,
    /// O(1) Scalar to Vector Broadcast.
    Broadcast,
    /// O(1) Formal Contract Check.
    AssertContract,
    /// O(1) Stream Head Pointer Advance.
    StreamAdv,
    /// O(1) Atomic Swap (for Evolution).
    AtomicSwap,
    /// Halt execution.
    Halt,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub op: Opcode,
    pub energy_cost_nj: f32, // Pre-calculated EFDD cost
}

pub struct NativeImage {
    pub instructions: Vec<Instruction>,
    pub entry_point: usize,
}
