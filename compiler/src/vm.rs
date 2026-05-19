use crate::ir::{Instruction, Bytecode};

pub struct VM {
    registers: Vec<f32>,
    stack: Vec<f32>,
}

impl VM {
    pub fn new() -> Self {
        Self {
            registers: vec![0.0; 100],
            stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode) {
        let mut ip = 0;
        while ip < bytecode.instructions.len() {
            let instr = &bytecode.instructions[ip];
            match instr {
                Instruction::LoadConst(val) => {
                    self.stack.push(*val);
                }
                Instruction::LoadVar(reg) => {
                    self.stack.push(self.registers[*reg]);
                }
                Instruction::StoreVar(reg) => {
                    if let Some(val) = self.stack.pop() {
                        self.registers[*reg] = val;
                    }
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap_or(0.0);
                    let a = self.stack.pop().unwrap_or(0.0);
                    self.stack.push(a + b);
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap_or(0.0);
                    let a = self.stack.pop().unwrap_or(0.0);
                    self.stack.push(a * b);
                }
                Instruction::Return => {
                    // Result is on top of stack
                    if let Some(res) = self.stack.pop() {
                        println!("VM Execution Result: {}", res);
                    }
                }
                _ => {}
            }
            ip += 1;
        }
    }
}
