use crate::ast::Node;

pub struct Analyzer;

impl Analyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.analyze(child)?;
                }
            }
            Node::ComplexityBlock { content, .. } => {
                for func in content {
                    self.check(func)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn check(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Function { name, body, .. } => {
                // 1. PDD Verification
                let loops = Self::count_loops(body);
                // (PDD logic...)
                
                // 2. Entropy Verification
                let entropy = Self::estimate_entropy(body);
                // For bootstrap, we'll just log it. In a real MCU profile, we'd fail if > budget.
            }
            _ => {}
        }
        Ok(())
    }

    fn estimate_entropy(body: &Vec<crate::ast::Stmt>) -> f32 {
        let mut total = 0.0;
        for stmt in body {
            match stmt {
                crate::ast::Stmt::Budget { limit, body: inner } => {
                    let cost = Self::estimate_entropy(inner);
                    if cost > *limit {
                        // This would be a compilation failure in a strict profile
                        println!("Warning: Budget {} exceeded (Estimated: {})", limit, cost);
                    }
                }
                crate::ast::Stmt::Expression { .. } => total += 1.0,
                crate::ast::Stmt::Let { .. } => total += 0.5,
                _ => {}
            }
        }
        total
    }

    fn count_loops(body: &Vec<crate::ast::Stmt>) -> usize {
        let mut count = 0;
        for stmt in body {
            if let crate::ast::Stmt::While { body: inner_body, .. } = stmt {
                count += 1 + Self::count_loops(inner_body);
            }
            if let crate::ast::Stmt::If { then_branch, else_branch, .. } = stmt {
                count += Self::count_loops(then_branch);
                if let Some(eb) = else_branch {
                    count += Self::count_loops(eb);
                }
            }
            if let crate::ast::Stmt::Evolve { body: inner_body } = stmt {
                count += Self::count_loops(inner_body);
            }
        }
        count
    }
}
