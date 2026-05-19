use crate::ast::{Node, Stmt, Expr, Type};
use std::collections::HashMap;

pub struct TypeChecker<'a> {
    scopes: Vec<HashMap<&'a str, Type>>,
}

impl<'a> TypeChecker<'a> {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()] }
    }

    pub fn check(&mut self, node: &Node<'a>) -> Result<(), String> {
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
            Node::Function { name: _, params, return_ty, body } => {
                self.enter_scope();
                for p in params {
                    self.define_var(p, Type::Unknown);
                }
                
                for stmt in body {
                    self.check_stmt(stmt, return_ty)?;
                }
                self.exit_scope();
            }
            _ => {}
        }
        Ok(())
    }

    fn check_stmt(&mut self, stmt: &Stmt<'a>, expected_ret: &Option<Type>) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, ty, value } => {
                let val_ty = self.infer_expr_type(value)?;
                if let Some(explicit_ty) = ty {
                    if *explicit_ty != val_ty && val_ty != Type::Unknown {
                        return Err(format!("Type mismatch for variable '{}': Expected {:?}, got {:?}", name, explicit_ty, val_ty));
                    }
                    self.define_var(name, *explicit_ty);
                } else {
                    self.define_var(name, val_ty);
                }
            }
            Stmt::Return { value } => {
                let _ret_ty = if let Some(expr) = value {
                    self.infer_expr_type(expr)?
                } else {
                    Type::Unknown // Void
                };
            }
            Stmt::While { condition, body } => {
                self.infer_expr_type(condition)?;
                self.enter_scope();
                for s in body {
                    self.check_stmt(s, expected_ret)?;
                }
                self.exit_scope();
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.infer_expr_type(condition)?;
                self.enter_scope();
                for s in then_branch {
                    self.check_stmt(s, expected_ret)?;
                }
                self.exit_scope();
                if let Some(eb) = else_branch {
                    self.enter_scope();
                    for s in eb {
                        self.check_stmt(s, expected_ret)?;
                    }
                    self.exit_scope();
                }
            }
            Stmt::Expression { expr } => {
                self.infer_expr_type(expr)?;
            }
            Stmt::Memory { name, ty, .. } => {
                self.define_var(name, *ty);
            }
            Stmt::Evolve { body } => {
                self.enter_scope();
                for s in body {
                    self.check_stmt(s, expected_ret)?;
                }
                self.exit_scope();
            }
            Stmt::Budget { body, .. } => {
                self.enter_scope();
                for s in body {
                    self.check_stmt(s, expected_ret)?;
                }
                self.exit_scope();
            }
            Stmt::Prob { branches } => {
                for (_, body) in branches {
                    self.enter_scope();
                    for s in body {
                        self.check_stmt(s, expected_ret)?;
                    }
                    self.exit_scope();
                }
            }
            Stmt::Sync { body, .. } => {
                self.enter_scope();
                for s in body {
                    self.check_stmt(s, expected_ret)?;
                }
                self.exit_scope();
            }
            Stmt::Gossip { .. } => {}
            Stmt::Contract { .. } => {}
            Stmt::Knowledge { name, .. } => {
                self.define_var(name, Type::Stream);
            }
            Stmt::Publish { .. } => {}
        }
        Ok(())
    }

    fn infer_expr_type(&self, expr: &Expr<'a>) -> Result<Type, String> {
        match expr {
            Expr::NumberLiteral(n) => {
                if n.contains('.') { Ok(Type::F32) } else { Ok(Type::I32) }
            }
            Expr::StringLiteral(_) => Ok(Type::Unknown),
            Expr::Identifier(id) => {
                if let Some(ty) = self.resolve_var(id) {
                    Ok(ty)
                } else {
                    if *id == "stream" || *id == "input" { return Ok(Type::Stream); }
                    Err(format!("Undefined variable '{}'", id))
                }
            }
            Expr::BinaryOp { left, right, op } => {
                let lt = self.infer_expr_type(left)?;
                let rt = self.infer_expr_type(right)?;
                if lt != rt && lt != Type::Unknown && rt != Type::Unknown {
                    if lt == Type::Stream || rt == Type::Stream {
                        return Ok(Type::Stream);
                    }
                    return Err(format!("Type mismatch in binary op '{}': {:?} and {:?}", op, lt, rt));
                }
                Ok(lt)
            }
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    fn define_var(&mut self, name: &'a str, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }

    fn resolve_var(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    }
}
