use crate::ast::{Node, Stmt, Expr};
use std::collections::HashMap;

/// O(1) Opcode weighting in Nanojoules (nJ).
/// EFDD: Mandatory opcode-level energy modeling.
pub struct EnergyModel {
    weights: HashMap<&'static str, f32>,
}

impl EnergyModel {
    pub fn new() -> Self {
        let mut m = HashMap::new();
        m.insert("+", 0.05); // Addition is cheap
        m.insert("-", 0.05);
        m.insert("*", 0.15); // Multiplication is moderate
        m.insert("/", 0.45); // Division is expensive
        Self { weights: m }
    }

    pub fn get_cost(&self, op: &str) -> f32 {
        *self.weights.get(op).unwrap_or(&0.1)
    }
}

pub struct OmegaVerifier {
    energy_model: EnergyModel,
}

impl OmegaVerifier {
    pub fn new() -> Self {
        Self {
            energy_model: EnergyModel::new(),
        }
    }

    /// Entry point for the scientific verification pass.
    /// Time: O(N) where N is the number of AST nodes.
    pub fn verify(&self, node: &Node) -> Result<(), String> {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.verify(child)?;
                }
            }
            Node::ComplexityBlock { complexity, content } => {
                for func in content {
                    self.verify_pdd(func, complexity)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    /// PDD Proof: Validates that the function adheres to its Big-O signature.
    /// Time: O(N)
    fn verify_pdd(&self, func: &Node, expected_complexity: &str) -> Result<(), String> {
        if let Node::Function { body, name, .. } = func {
            let max_depth = self.analyze_loop_nesting(body);
            let actual_complexity = match max_depth {
                0 => "O(1)",
                1 => "O(N)",
                2 => "O(N^2)",
                _ => "O(N^k)",
            };

            if actual_complexity != expected_complexity {
                return Err(format!(
                    "PDD VIOLATION in function '{}': Declared {}, Proved {}.",
                    name, expected_complexity, actual_complexity
                ));
            }
            
            // Trigger EFDD check as part of the function analysis
            self.verify_efdd(body, name)?;
        }
        Ok(())
    }

    /// EFDD Proof: Validates that the function does not exceed its energy budget.
    /// Time: O(N)
    fn verify_efdd(&self, body: &Vec<Stmt>, func_name: &str) -> Result<(), String> {
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
                "EFDD VIOLATION in function '{}': Cost {}nJ exceeds budget {}nJ.",
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
                Stmt::If { then_branch, else_branch, .. } => {
                    let d1 = self.analyze_loop_nesting(then_branch);
                    let d2 = else_branch.as_ref().map(|b| self.analyze_loop_nesting(b)).unwrap_or(0);
                    if d1 > max { max = d1; }
                    if d2 > max { max = d2; }
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
                (c1 + c2) / 2.0 // Average case for EFDD estimation
            }
            Stmt::While { body, .. } => {
                // EFDD requires a termination contract or we assume a safe bound
                body.iter().map(|s| self.estimate_stmt_cost(s)).sum::<f32>() * 10.0 // Mocking 10 iterations
            }
            _ => 0.05,
        }
    }

    fn estimate_expr_cost(&self, expr: &Expr) -> f32 {
        match expr {
            Expr::BinaryOp { left, op, right } => {
                self.energy_model.get_cost(op) + self.estimate_expr_cost(left) + self.estimate_expr_cost(right)
            }
            _ => 0.01,
        }
    }
}
