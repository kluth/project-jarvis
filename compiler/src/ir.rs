#[derive(Debug, Clone)]
pub enum Instruction {
    LoadConst(f32),
    LoadVar(usize),
    StoreVar(usize),
    Add,
    Mul,
    Return,
    Halt,
    // Stream ops
    StreamMap,
    // GUI Ops (UDS)
    WinCreate,
    WinUpdate,
    DrawRect,
    // Comm Ops
    CommSync,
    CommGossip,
}

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}
