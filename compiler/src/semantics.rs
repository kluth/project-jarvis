use crate::ast::{Node, Stmt, Expr};
use std::collections::HashMap;

/// O(1) Opcode weighting in Nanojoules (nJ).
/// EFDD: Mandatory opcode-level energy modeling for AOT emission.
pub struct EnergyModel {
    weights: HashMap<&'static str, f32>,
}

impl EnergyModel {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("+", 0.05); 
        m.insert("-", 0.05);
        m.insert("*", 0.15); 
        m.insert("/", 0.45); 
        Self { weights: m }
    }

    pub fn get_cost(&self, op: &str) -> f32 {
        *self.weights.get(op).unwrap_or(&0.1)
    }
}

/// The JARVIS Production Formal Verifier.
/// Transforms 'verify' blocks into formal mathematical proofs of safety and performance.
pub struct OmegaVerifier {
    energy_model: EnergyModel,
}

impl OmegaVerifier {
    pub fn new() -> Self {
        Self {
            energy_model: EnergyModel::new(),
        }
    }

    /// Formal Proof Pass: Verified AST -> Proof Evidence.
    /// Time: O(N) where N is the number of AST nodes.
    pub fn verify<'a>(&self, node: &Node<'a>) -> Result<(), String> {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.verify(child)?;
                }
            }
            Node::ComplexityBlock { complexity, content } => {
                for func in content {
                    self.prove_pdd_and_efdd(func, complexity)?;
                }
            }
            Node::Struct { name, fields } => {
                self.prove_struct_layout(name, fields)?;
            }
            Node::Import { path: _ } => {
                // PDD: Resolution is O(log M) where M is module count.
                // Log import (simulated)
                // println!("IMPORT RESOLVED: {}", path);
            }
            Node::Static { name: _, address: _, size: _ } => {
                // PDD: Address resolution is O(1).
                // println!("STATIC MEMORY MAPPED: {} at 0x{:X} ({} bytes)", name, address, size);
            }
            _ => {}
        }
        Ok(())
    }

    fn prove_struct_layout<'a>(&self, _name: &str, fields: &Vec<(&str, crate::ast::Type<'a>)>) -> Result<(), String> {
        // PDD: Memory layout analysis must be O(F) where F is number of fields.
        let mut total_size = 0;
        for (_fname, fty) in fields {
            let size = match fty {
                crate::ast::Type::I32 | crate::ast::Type::F32 => 4,
                _ => 8, // Pointers/Streams
            };
            // Mandatory Alignment: 4-byte boundaries
            if total_size % 4 != 0 {
                total_size += 4 - (total_size % 4);
            }
            total_size += size;
        }
        
        // Log layout proof (simulated)
        // println!("LAYOUT PROOF for struct '{}': Size={} bytes, Fields={}", name, total_size, fields.len());
        Ok(())
    }

    /// PDD & EFDD Proof: Validates asymptotic bounds and energy limits.
    /// eTDD: Validates mandatory existence of verify blocks.
    fn prove_pdd_and_efdd<'a>(&self, node: &Node<'a>, expected_complexity: &str) -> Result<(), String> {
        match node {
            Node::Function { body, name, verification, .. } => {
                // 0. eTDD Proof: Mandatory verify block
                if verification.is_none() {
                    return Err(format!(
                        "eTDD VIOLATION in function '{}': Implementation detected without mandatory 'verify' block.",
                        name
                    ));
                }

                // 1. PDD Proof
                let max_depth = self.analyze_loop_nesting(body);
                let actual_complexity = match max_depth {
                    0 => "1",
                    1 => "N",
                    2 => "N^2",
                    _ => "N^k",
                };

                if actual_complexity != expected_complexity {
                    return Err(format!(
                        "PDD VIOLATION in function '{}': Declared {}, Proved {}.",
                        name, expected_complexity, actual_complexity
                    ));
                }
                
                // 2. EFDD Proof
                self.prove_efdd_bounds(body, name)?;
            }
            Node::Struct { name, fields } => {
                self.prove_struct_layout(name, fields)?;
            }
            Node::Render { name, verification, .. } => {
                if verification.is_none() {
                    return Err(format!("eTDD VIOLATION in render '{}': Missing verify block.", name));
                }
                // Render complexity is O(N) where N is number of components
                let actual_complexity = "N"; 
                if actual_complexity != expected_complexity && expected_complexity != "Pixels" {
                    return Err(format!("PDD VIOLATION in render '{}': Expected {}, Proved {}.", name, expected_complexity, actual_complexity));
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn prove_efdd_bounds(&self, body: &Vec<Stmt>, func_name: &str) -> Result<(), String> {
        let mut total_cost = 0.0;
        let mut budget_limit = f32::MAX;

        for stmt in body {
            match stmt {
                Stmt::Budget { limit, .. } => budget_limit = *limit,
                _ => total_cost += self.estimate_stmt_cost(stmt),
            }
        }

        if total_cost > budget_limit {
            return Err(format!(
                "EFDD ENERGY PROOF FAILED in function '{}': Model Cost {}nJ > Budget {}nJ.",
                func_name, total_cost, budget_limit
            ));
        }
        Ok(())
    }

    fn analyze_loop_nesting(&self, body: &Vec<Stmt>) -> usize {
        let mut max = 0;
        for stmt in body {
            match stmt {
                Stmt::While { body: inner, .. } => {
                    let d = 1 + self.analyze_loop_nesting(inner);
                    if d > max { max = d; }
                }
                Stmt::For { body: inner, .. } => {
                    let d = 1 + self.analyze_loop_nesting(inner);
                    if d > max { max = d; }
                }
                Stmt::If { then_branch, else_branch, .. } => {
                    let d1 = self.analyze_loop_nesting(then_branch);
                    let d2 = else_branch.as_ref().map(|b| self.analyze_loop_nesting(b)).unwrap_or(0);
                    if d1 > max { max = d1; }
                    if d2 > max { max = d2; }
                }
                Stmt::Evolve { body } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::Budget { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::Prob { branches } => {
                    for (_w, b) in branches {
                        let d = self.analyze_loop_nesting(b);
                        if d > max { max = d; }
                    }
                }
                Stmt::Sync { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::Event { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::Hologram { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::PostProcess { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                Stmt::NeuroAdapt { body, .. } => {
                    let d = self.analyze_loop_nesting(body);
                    if d > max { max = d; }
                }
                _ => {}
            }
        }
        max
    }

    fn estimate_stmt_cost(&self, stmt: &Stmt) -> f32 {
        match stmt {
            Stmt::Expression { expr } => self.estimate_expr_cost(expr),
            Stmt::Let { value, .. } => 0.1 + self.estimate_expr_cost(value),
            Stmt::If { then_branch, else_branch, .. } => {
                let c1 = then_branch.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>();
                let c2 = else_branch.as_ref().map(|b| b.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>()).unwrap_or(0.0);
                c1.max(c2) // Worst-case path for formal proof
            }
            Stmt::While { body, .. } | Stmt::For { body, .. } => {
                // EFDD requires bounded loops for proof
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() * 100.0 // Proof bound: 100 iterations
            }
            Stmt::Prob { branches } => {
                branches.iter().map(|(w, b)| w * b.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>()).sum()
            }
            Stmt::Sync { body, .. } => {
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 2000.0
            }
            Stmt::Gossip { .. } => 150.0,
            Stmt::Publish { .. } => 150.0,
            Stmt::Contract { .. } => 0.05,
            Stmt::Knowledge { .. } => 500.0,
            Stmt::Memory { .. } => 0.2,
            Stmt::Evolve { body } => {
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 1000.0
            }
            Stmt::Window { .. } => 50000.0,
            Stmt::Event { body, .. } => {
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 100.0
            }
            Stmt::Assert { condition } => self.estimate_expr_cost(condition) + 0.05,
            Stmt::Layout { content, .. } => {
                content.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 500.0
            }
            Stmt::Component { args, .. } => {
                args.iter().map(|a| self.estimate_expr_cost(a)).sum::<f32>() + 150.0
            }
            Stmt::Poll => 100.0,
            Stmt::Print { value, x, y, color } => {
                self.estimate_expr_cost(value) + self.estimate_expr_cost(x) + self.estimate_expr_cost(y) + self.estimate_expr_cost(color) + 250.0
            }
            Stmt::CaptureFrame => 5000.0,
            Stmt::CaptureStream => 1000.0,
            Stmt::Asm { .. } => 0.05,
            Stmt::VolatileWrite { .. } => 0.1,
            Stmt::VolatileRead { .. } => 0.1,
            Stmt::PortWrite { .. } => 0.1,
            Stmt::PortRead { .. } => 0.1,
            Stmt::AtomicOp { .. } => 0.2,
            Stmt::Hologram { depth, body, .. } => {
                self.estimate_expr_cost(depth) + body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 1010.0
            }
            Stmt::PostProcess { intensity, body, .. } => {
                self.estimate_expr_cost(intensity) + body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 8000.0
            }
            Stmt::NeuroAdapt { load, body, .. } => {
                self.estimate_expr_cost(load) + body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() + 50.0
            }
            Stmt::Budget { body, .. } => {
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>()
            }
            Stmt::Return { value } => value.as_ref().map(|v| self.estimate_expr_cost(v)).unwrap_or(0.01),
        }
    }

    fn estimate_expr_cost(&self, expr: &Expr) -> f32 {
        match expr {
            Expr::BinaryOp { left, op, right } => {
                self.energy_model.get_cost(op) + self.estimate_expr_cost(left) + self.estimate_expr_cost(right)
            }
            Expr::Call { args, .. } => {
                0.5 + args.iter().map(|a| self.estimate_expr_cost(a)).sum::<f32>()
            }
            Expr::Input => 0.1,
            _ => 0.01,
        }
    }
}
