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
    /// O(Pixels) Render UI Frame.
    UIRender,
    /// O(N) Apply Layout.
    UILayout,
    /// O(1) Draw UI Component.
    UIComponent,
    /// O(1) Draw Rectangle.
    DrawRect,
    /// O(1) Create GUI Window.
    WinCreate,
    /// O(1) Update GUI Window (Flush buffer).
    WinUpdate,
    /// O(1) Poll GUI Events.
    WinPoll,
    /// O(1) Get Specific Event Data.
    EventGet,
    /// O(Pixels) Draw Text String.
    DrawText,
    /// O(1) Get Keyboard Input.
    InputGet,
    /// O(Pixels) Capture Single Frame.
    ScreenCap,
    /// O(Pixels) Start/Stop Frame Stream Capture.
    StreamCap,
    /// O(log N) Swarm Consensus.
    CommSync,
    /// O(1) Swarm Gossip.
    CommGossip,
    /// O(1) Swarm Publish.
    CommPublish,
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
