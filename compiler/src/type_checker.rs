use crate::ast::{Node, Stmt, Expr};

pub struct TypeChecker;

impl TypeChecker {
    pub fn new() -> Self {
        Self
    }

    pub fn check(&mut self, node: &Node) -> Result<(), String> {
        match node {
            Node::Module { body, .. } => {
                for child in body {
                    self.check(child)?;
                }
            }
            Node::ComplexityBlock { content, .. } => {
                for func in content {
                    self.check(func)?;
                }
            }
            Node::Function { body, .. } => {
                for stmt in body {
                    self.check_stmt(stmt)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { value, .. } => self.check_expr(value),
            Stmt::Expression { expr } => self.check_expr(expr),
            Stmt::If { condition, then_branch, else_branch } => {
                self.check_expr(condition)?;
                for s in then_branch { self.check_stmt(s)?; }
                if let Some(eb) = else_branch {
                    for s in eb { self.check_stmt(s)?; }
                }
                Ok(())
            }
            Stmt::While { condition, body } => {
                self.check_expr(condition)?;
                for s in body { self.check_stmt(s)?; }
                Ok(())
            }
            Stmt::Budget { body, .. } => {
                for s in body { self.check_stmt(s)?; }
                Ok(())
            }
            Stmt::Assert { condition } => self.check_expr(condition),
            _ => Ok(()),
        }
    }

    fn check_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::BinaryOp { left, right, .. } => {
                self.check_expr(left)?;
                self.check_expr(right)?;
            }
            Expr::Call { args, .. } => {
                for arg in args { self.check_expr(arg)?; }
            }
            Expr::Assignment { value, .. } => {
                self.check_expr(value)?;
            }
            _ => {}
        }
        Ok(())
    }
}
