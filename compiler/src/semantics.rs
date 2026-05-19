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
            Node::ComplexityBlock { complexity, content } => {
                for func in content {
                    if let Node::Function { .. } = func {
                        // Minimal Analysis: If complexity is O(1), function must not have loops.
                        // Since we don't have loop nodes yet, O(1) always passes if empty.
                        self.verify_function_complexity(func, *complexity)?;
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn verify_function_complexity(&self, func: &Node, declared: &str) -> Result<(), String> {
        if let Node::Function { name, body, .. } = func {
            // Count loops in body
            let loops = Self::count_loops(body);
            
            let actual = if loops == 0 {
                "1"
            } else if loops == 1 {
                "N"
            } else {
                "N^2" // Simplified nesting logic
            };
            
            if actual != declared {
                return Err(format!(
                    "Complexity mismatch in function '{}': Declared O({}), Analyzed O({})",
                    name, declared, actual
                ));
            }
        }
        Ok(())
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
        }
        count
    }
}
