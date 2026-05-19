use crate::ast::{Node, Stmt};

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
            Node::Function { name: _, body, .. } => {
                // 1. PDD Verification
                let _loops = Self::count_loops(body);
                
                // 2. Entropy Verification
                let _entropy = Self::estimate_entropy(body);
            }
            _ => {}
        }
        Ok(())
    }

    fn estimate_entropy(body: &Vec<Stmt>) -> f32 {
        let mut total = 0.0;
        for stmt in body {
            match stmt {
                Stmt::Budget { limit, body: inner } => {
                    let cost = Self::estimate_entropy(inner);
                    if cost > *limit {
                        println!("Warning: Budget {} exceeded (Estimated: {})", limit, cost);
                    }
                }
                Stmt::Expression { .. } => total += 1.0,
                Stmt::Let { .. } => total += 0.5,
                Stmt::Evolve { body: inner } => total += Self::estimate_entropy(inner) * 1.5,
                _ => {}
            }
        }
        total
    }

    fn count_loops(body: &Vec<Stmt>) -> usize {
        let mut count = 0;
        for stmt in body {
            match stmt {
                Stmt::While { body: inner, .. } => {
                    count += 1 + Self::count_loops(inner);
                }
                Stmt::If { then_branch, else_branch, .. } => {
                    count += Self::count_loops(then_branch);
                    if let Some(eb) = else_branch {
                        count += Self::count_loops(eb);
                    }
                }
                Stmt::Evolve { body: inner } => {
                    count += Self::count_loops(inner);
                }
                Stmt::Sync { body: inner, .. } => {
                    count += Self::count_loops(inner);
                }
                _ => {}
            }
        }
        count
    }
}
