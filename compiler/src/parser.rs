use crate::lexer::{Lexer, Token};
use crate::ast::{Node, Stmt, Expr, Type};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Self { lexer, current_token }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token<'a>) -> Result<(), String> {
        if self.current_token == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.current_token))
        }
    }

    pub fn parse_module(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Module)?;
        
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected module name".to_string());
        };

        let mut body = Vec::new();

        while self.current_token != Token::Verify && self.current_token != Token::Eof {
            match self.current_token {
                Token::Complexity => {
                    body.push(self.parse_complexity_block()?);
                }
                _ => break,
            }
        }

        if self.current_token == Token::Verify {
            body.push(self.parse_verify_block()?);
        }

        Ok(Node::Module { name, body })
    }

    fn parse_complexity_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Complexity)?;
        
        let complexity = if let Token::BigO(o) = self.current_token {
            self.advance();
            o
        } else {
            return Err("Expected Big-O notation".to_string());
        };

        self.expect(Token::OpenBrace)?;
        
        let mut content = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            if self.current_token == Token::Func {
                content.push(self.parse_function()?);
            } else {
                self.advance(); // Skip unknown for now
            }
        }
        
        self.expect(Token::CloseBrace)?;
        Ok(Node::ComplexityBlock { complexity, content })
    }

    fn parse_function(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Func)?;
        
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected function name".to_string());
        };

        self.expect(Token::OpenParen)?;
        // parse params... ignoring for bootstrap
        self.expect(Token::CloseParen)?;
        
        let mut return_ty = None;
        if self.current_token == Token::Arrow {
            self.advance();
            return_ty = Some(self.parse_type()?);
        }

        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        
        Ok(Node::Function { name, params: Vec::new(), return_ty, body })
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.current_token {
            Token::TypeI32 => { self.advance(); Ok(Type::I32) },
            Token::TypeF32 => { self.advance(); Ok(Type::F32) },
            Token::TypeStream => { self.advance(); Ok(Type::Stream) },
            Token::Identifier(_) => { self.advance(); Ok(Type::Unknown) },
            _ => Err("Expected Type".to_string())
        }
    }

    fn parse_block(&mut self) -> Result<Vec<Stmt<'a>>, String> {
        let mut stmts = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            stmts.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Stmt<'a>, String> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::While => self.parse_while_statement(),
            Token::If => self.parse_if_statement(),
            _ => {
                let expr = self.parse_expression()?;
                if self.current_token == Token::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expression { expr })
            }
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // let
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected identifier after let".to_string());
        };

        let mut ty = None;
        if self.current_token == Token::Colon {
            self.advance();
            ty = Some(self.parse_type()?);
        }

        self.expect(Token::Assign)?;
        let value = self.parse_expression()?;
        
        if self.current_token == Token::Semicolon {
            self.advance();
        }

        Ok(Stmt::Let { name, ty, value })
    }

    fn parse_return_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // return
        let value = if self.current_token != Token::Semicolon {
            Some(self.parse_expression()?)
        } else {
            None
        };
        
        if self.current_token == Token::Semicolon {
            self.advance();
        }

        Ok(Stmt::Return { value })
    }

    fn parse_while_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // while
        let condition = self.parse_expression()?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::While { condition, body })
    }

    fn parse_if_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // if
        let condition = self.parse_expression()?;
        self.expect(Token::OpenBrace)?;
        let then_branch = self.parse_block()?;
        
        let mut else_branch = None;
        if self.current_token == Token::Else {
            self.advance();
            self.expect(Token::OpenBrace)?;
            else_branch = Some(self.parse_block()?);
        }

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    fn parse_expression(&mut self) -> Result<Expr<'a>, String> {
        // Very basic parsing for demo. Only literals and ids for now.
        match self.current_token {
            Token::NumberLiteral(n) => {
                self.advance();
                Ok(Expr::NumberLiteral(n))
            }
            Token::StringLiteral(s) => {
                self.advance();
                Ok(Expr::StringLiteral(s))
            }
            Token::Identifier(id) => {
                self.advance();
                Ok(Expr::Identifier(id))
            }
            _ => Err(format!("Unexpected token in expression: {:?}", self.current_token))
        }
    }

    fn parse_verify_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Verify)?;
        self.expect(Token::OpenBrace)?;
        
        let mut tests = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            if self.current_token == Token::Test {
                tests.push(self.parse_test()?)
            } else {
                self.advance();
            }
        }

        self.expect(Token::CloseBrace)?;
        Ok(Node::VerifyBlock { tests })
    }

    fn parse_test(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Test)?;
        
        let name = if let Token::StringLiteral(s) = self.current_token {
            self.advance();
            s
        } else {
            "unnamed"
        };

        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Node::Test { name, body })
    }
}
