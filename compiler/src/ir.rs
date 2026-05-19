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
}

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
}
