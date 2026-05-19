use crate::ast::{Node, Stmt, Expr};
use crate::ir::{Instruction, Bytecode};
use std::collections::HashMap;

pub struct CodeGen<'a> {
    instructions: Vec<Instruction>,
    symbol_table: HashMap<&'a str, usize>,
    next_reg: usize,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            next_reg: 0,
        }
    }

    pub fn generate(&mut self, node: &Node<'a>) -> Bytecode {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.generate_node(child);
                }
            }
            _ => {}
        }
        Bytecode { instructions: self.instructions.clone() }
    }

    fn generate_node(&mut self, node: &Node<'a>) {
        match node {
            Node::ComplexityBlock { content, .. } => {
                for func in content {
                    self.generate_node(func);
                }
            }
            Node::Function { body, .. } => {
                for stmt in body {
                    self.generate_stmt(stmt);
                }
                self.instructions.push(Instruction::Return);
            }
            _ => {}
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt<'a>) {
        match stmt {
            Stmt::Let { name, value, .. } => {
                self.generate_expr(value);
                let reg = self.next_reg;
                self.symbol_table.insert(name, reg);
                self.next_reg += 1;
                self.instructions.push(Instruction::StoreVar(reg));
            }
            Stmt::Return { value } => {
                if let Some(expr) = value {
                    self.generate_expr(expr);
                }
                self.instructions.push(Instruction::Return);
            }
            Stmt::Expression { expr } => {
                self.generate_expr(expr);
            }
            _ => {} // ignoring complex flow for now
        }
    }

    fn generate_expr(&mut self, expr: &Expr<'a>) {
        match expr {
            Expr::NumberLiteral(n) => {
                let val = n.parse::<f32>().unwrap_or(0.0);
                self.instructions.push(Instruction::LoadConst(val));
            }
            Expr::Identifier(id) => {
                if let Some(reg) = self.symbol_table.get(id) {
                    self.instructions.push(Instruction::LoadVar(*reg));
                }
            }
            Expr::BinaryOp { left, right, op } => {
                self.generate_expr(left);
                self.generate_expr(right);
                match *op {
                    "+" => self.instructions.push(Instruction::Add),
                    "*" => self.instructions.push(Instruction::Mul),
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
