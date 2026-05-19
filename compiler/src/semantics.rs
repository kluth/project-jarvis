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
        if let Node::Function { name, .. } = func {
            let actual = "1"; // Stubbed for now
            if actual != declared {
                return Err(format!(
                    "Complexity mismatch in function '{}': Declared O({}), Analyzed O({})",
                    name, declared, actual
                ));
            }
        }
        Ok(())
    }
}
