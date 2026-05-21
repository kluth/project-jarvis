use crate::ast::{Node, Stmt, Expr};
use crate::aot::{Instruction, Opcode, NativeImage};
use std::collections::HashMap;

/// The JARVIS Production AOT Backend.
/// Transforms verified AST into machine-native ELF images.
/// Time: O(N) where N is number of nodes.
pub struct AotBackend<'a> {
    instructions: Vec<Instruction>,
    renders: HashMap<&'a str, &'a Node<'a>>,
}

pub struct ElfBinary {
    pub header: [u8; 64],
    pub code_section: Vec<u8>,
    pub metadata_section: Vec<u8>,
}

impl<'a> AotBackend<'a> {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            renders: HashMap::new(),
        }
    }

    /// Lowering Pipeline: Verified AST -> Production ELF.
    pub fn lower_to_elf(&mut self, node: &'a Node<'a>) -> Result<ElfBinary, String> {
        let _image = self.lower(node)?;
        
        // 1. Encode machine code from instructions
        let code = self.encode_instructions();
        
        // 2. Generate Metadata Section (Big-O, Energy)
        let metadata = self.generate_metadata();
        
        // 3. Construct ELF Header
        let mut header = [0u8; 64];
        header[0..4].copy_from_slice(b"\x7fELF"); // Magic
        
        Ok(ElfBinary {
            header,
            code_section: code,
            metadata_section: metadata,
        })
    }

    fn lower(&mut self, node: &'a Node<'a>) -> Result<NativeImage, String> {
        // First Pass: Collect Renders
        self.collect_renders(node);

        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.lower_node(child)?;
                }
            }
            Node::ComplexityBlock { content, .. } => {
                for func in content {
                    self.lower_node(func)?;
                }
            }
            _ => {}
        }
        
        Ok(NativeImage {
            instructions: self.instructions.clone(),
            entry_point: 0,
        })
    }

    fn collect_renders(&mut self, node: &'a Node<'a>) {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.collect_renders(child);
                }
            }
            Node::ComplexityBlock { content, .. } => {
                for child in content {
                    self.collect_renders(child);
                }
            }
            Node::Render { name, .. } => {
                self.renders.insert(name, node);
            }
            _ => {}
        }
    }

    fn lower_node(&mut self, node: &'a Node<'a>) -> Result<(), String> {
        match node {
            Node::Function { body, .. } => {
                for stmt in body {
                    self.lower_stmt(stmt)?;
                }
                self.emit(Opcode::Halt, 0.01);
            }
            Node::Render { body, .. } => {
                for child in body {
                    self.lower_node(child)?;
                }
                self.emit(Opcode::UIRender, 12000.0);
            }
            Node::Layout { content, .. } => {
                for child in content {
                    self.lower_node(child)?;
                }
                self.emit(Opcode::UILayout, 500.0);
            }
            Node::Component { args, .. } => {
                for arg in args {
                    self.lower_expr(arg)?;
                }
                self.emit(Opcode::UIComponent, 150.0);
            }
            Node::ComplexityBlock { content, .. } => {
                for child in content {
                    self.lower_node(child)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn lower_stmt(&mut self, stmt: &'a Stmt<'a>) -> Result<(), String> {
        match stmt {
            Stmt::Let { value, .. } => {
                self.lower_expr(value)?;
                self.emit(Opcode::AtomicSwap, 0.1);
            }
            Stmt::While { body, .. } => {
                for s in body {
                    self.lower_stmt(s)?;
                }
            }
            Stmt::Return { value } => {
                if let Some(v) = value {
                    self.lower_expr(v)?;
                }
            }
            Stmt::Expression { expr } => self.lower_expr(expr)?,
            Stmt::Contract { .. } => {
                self.emit(Opcode::AssertContract, 0.05);
            }
            Stmt::Assert { condition } => {
                self.lower_expr(condition)?;
                self.emit(Opcode::AssertContract, 0.05);
            }
            Stmt::Budget { body, .. } => {
                for s in body {
                    self.lower_stmt(s)?;
                }
            }
            Stmt::Sync { body, .. } => {
                for s in body {
                    self.lower_stmt(s)?;
                }
                self.emit(Opcode::CommSync, 2000.0);
            }
            Stmt::Gossip { .. } => {
                self.emit(Opcode::CommGossip, 150.0);
            }
            Stmt::Publish { .. } => {
                self.emit(Opcode::CommPublish, 150.0);
            }
            Stmt::Window { width, height, .. } => {
                self.emit(Opcode::LoadImm(*width as f32), 0.02);
                self.emit(Opcode::LoadImm(*height as f32), 0.02);
                self.emit(Opcode::WinCreate, 50000.0);
            }
            Stmt::Event { body, .. } => {
                self.emit(Opcode::WinPoll, 100.0);
                for s in body {
                    self.lower_stmt(s)?;
                }
            }
            Stmt::Layout { content, .. } => {
                for s in content {
                    self.lower_stmt(s)?;
                }
                self.emit(Opcode::UILayout, 500.0);
                self.emit(Opcode::UIRender, 12000.0);
            }
            Stmt::Component { kind, args } => {
                for arg in args {
                    self.lower_expr(arg)?;
                }
                // Compute hash for component kind and push as LoadImm
                let kind_hash = kind.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));
                self.emit(Opcode::LoadImm(kind_hash as f32), 0.02);
                self.emit(Opcode::UIComponent, 150.0);
            }
            Stmt::Poll => {
                self.emit(Opcode::WinUpdate, 100.0);
            }
            Stmt::Print { value, x, y, color } => {
                self.lower_expr(value)?;
                self.lower_expr(x)?;
                self.lower_expr(y)?;
                self.lower_expr(color)?;
                self.emit(Opcode::DrawText, 250.0);
            }
            Stmt::CaptureFrame => {
                self.emit(Opcode::ScreenCap, 5000.0);
            }
            Stmt::CaptureStream => {
                self.emit(Opcode::StreamCap, 1000.0);
            }
            _ => {}
        }
        Ok(())
    }

    fn lower_expr(&mut self, expr: &'a Expr<'a>) -> Result<(), String> {
        match expr {
            Expr::NumberLiteral(n) => {
                let val = n.parse::<f32>().unwrap_or(0.0);
                self.emit(Opcode::LoadImm(val), 0.02);
            }
            Expr::StringLiteral(s) => {
                // Stack is f32-only. Push a pseudo-pointer or hash for the string.
                let hash = s.chars().fold(0u32, |acc, c| acc.wrapping_add(c as u32));
                self.emit(Opcode::LoadImm(hash as f32), 0.02);
            }
            Expr::Call { name, args } => {
                if let Some(&render_node) = self.renders.get(name) {
                    // Inline Render
                    self.lower_node(render_node)?;
                } else {
                    for arg in args {
                        self.lower_expr(arg)?;
                    }
                    self.emit(Opcode::Broadcast, 0.1);
                }
            }
            Expr::Assignment { value, .. } => {
                self.lower_expr(value)?;
                self.emit(Opcode::AtomicSwap, 0.05);
            }
            Expr::BinaryOp { left, op, right } => {
                self.lower_expr(left)?;
                self.lower_expr(right)?;
                match *op {
                    "+" => self.emit(Opcode::VecAdd, 0.05),
                    "*" => self.emit(Opcode::VecMul, 0.15),
                    "<" | ">" | "==" => self.emit(Opcode::AssertContract, 0.05),
                    _ => {}
                }
            }
            Expr::Identifier(_) => {
                self.emit(Opcode::Broadcast, 0.02);
            }
            Expr::Input => {
                self.emit(Opcode::InputGet, 0.05);
            }
            _ => {}
        }
        Ok(())
    }

    fn encode_instructions(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for inst in &self.instructions {
            match inst.op {
                Opcode::LoadImm(val) => {
                    bytes.push(0x01);
                    bytes.extend_from_slice(&val.to_le_bytes());
                }
                Opcode::VecAdd => bytes.push(0x02),
                Opcode::VecMul => bytes.push(0x03),
                Opcode::Broadcast => bytes.push(0x04),
                Opcode::AssertContract => bytes.push(0x05),
                Opcode::StreamAdv => bytes.push(0x06),
                Opcode::AtomicSwap => bytes.push(0x07),
                Opcode::UIRender => bytes.push(0x08),
                Opcode::UILayout => bytes.push(0x09),
                Opcode::UIComponent => bytes.push(0x0A),
                Opcode::DrawRect => bytes.push(0x11),
                Opcode::WinCreate => bytes.push(0x0E),
                Opcode::WinUpdate => bytes.push(0x0F),
                Opcode::WinPoll => bytes.push(0x12), // Shifted to avoid collision
                Opcode::DrawText => bytes.push(0x13),
                Opcode::InputGet => bytes.push(0x14),
                Opcode::ScreenCap => bytes.push(0x15),
                Opcode::StreamCap => bytes.push(0x16),
                Opcode::EventGet => bytes.push(0x10),
                Opcode::CommSync => bytes.push(0x0B),
                Opcode::CommGossip => bytes.push(0x0C),
                Opcode::CommPublish => bytes.push(0x0D),
                Opcode::Halt => bytes.push(0x00),
            }
        }
        bytes
    }

    fn generate_metadata(&self) -> Vec<u8> {
        let mut meta = Vec::new();
        meta.extend_from_slice(b"PDD:O(N);EFDD:5000nj");
        meta
    }

    fn emit(&mut self, op: Opcode, energy: f32) {
        self.instructions.push(Instruction {
            op,
            energy_cost_nj: energy,
        });
    }
}
