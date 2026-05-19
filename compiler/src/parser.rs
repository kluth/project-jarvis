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
        let mut params = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
            if let Token::Identifier(pname) = self.current_token {
                self.advance();
                self.expect(Token::Colon)?;
                self.parse_type()?;
                params.push(pname);
                if self.current_token == Token::Comma {
                    self.advance();
                }
            } else {
                break;
            }
        }
        self.expect(Token::CloseParen)?;
        
        let mut return_ty = None;
        if self.current_token == Token::Arrow {
            self.advance();
            return_ty = Some(self.parse_type()?);
        }

        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        
        Ok(Node::Function { name, params, return_ty, body })
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
            Token::Memory => self.parse_memory_statement(),
            Token::Evolve => self.parse_evolve_block(),
            Token::Budget => self.parse_budget_statement(),
            Token::Prob => self.parse_prob_block(),
            Token::Sync => self.parse_sync_block(),
            Token::Gossip => self.parse_gossip_statement(),
            Token::Contract => self.parse_contract_block(),
            Token::Knowledge => self.parse_knowledge_statement(),
            Token::Publish => self.parse_publish_statement(),
            _ => {
                let expr = self.parse_expression()?;
                if self.current_token == Token::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expression { expr })
            }
        }
    }

    fn parse_contract_block(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // contract
        self.expect(Token::OpenBrace)?;
        let start = self.lexer.cursor;
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            self.advance();
        }
        let spec = &self.lexer.source[start..self.lexer.cursor];
        self.expect(Token::CloseBrace)?;
        Ok(Stmt::Contract { spec })
    }

    fn parse_knowledge_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // knowledge
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "unnamed" };
        self.expect(Token::Colon)?;
        self.expect(Token::Identifier("Vector"))?;
        self.expect(Token::OpenBracket)?;
        let mut dim = 0;
        if let Token::NumberLiteral(n) = self.current_token {
            dim = n.parse().unwrap_or(0);
            self.advance();
        }
        self.expect(Token::CloseBracket)?;
        Ok(Stmt::Knowledge { name, dim })
    }

    fn parse_publish_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // publish
        self.expect(Token::OpenParen)?;
        let target = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "global" };
        self.expect(Token::CloseParen)?;
        Ok(Stmt::Publish { target })
    }

    fn parse_budget_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // budget
        let limit = if let Token::NumberLiteral(n) = self.current_token {
            let val = n.parse::<f32>().unwrap_or(0.0);
            self.advance();
            val
        } else {
            return Err("Expected budget value".to_string());
        };
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Budget { limit, body })
    }

    fn parse_prob_block(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // prob
        self.expect(Token::OpenBrace)?;
        let mut branches = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            if let Token::NumberLiteral(n) = self.current_token {
                let weight = n.parse::<f32>().unwrap_or(0.0);
                self.advance();
                self.expect(Token::Arrow)?;
                self.expect(Token::OpenBrace)?;
                let branch_body = self.parse_block()?;
                branches.push((weight, branch_body));
            } else {
                break;
            }
        }
        self.expect(Token::CloseBrace)?;
        Ok(Stmt::Prob { branches })
    }

    fn parse_sync_block(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // sync
        let protocol = if self.current_token == Token::OpenParen {
            self.advance();
            self.expect(Token::Identifier("protocol"))?;
            self.expect(Token::Colon)?;
            let p = if let Token::Identifier(id) = self.current_token {
                self.advance();
                id
            } else { "default" };
            self.expect(Token::CloseParen)?;
            p
        } else {
            "default"
        };
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Sync { protocol, body })
    }

    fn parse_gossip_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // gossip
        self.expect(Token::OpenParen)?;
        let target = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "broadcast" };
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Gossip { target })
    }

    fn parse_memory_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // memory
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected name for memory state".to_string());
        };

        self.expect(Token::Colon)?;
        let ty = self.parse_type()?;
        
        let mut size = 1;
        if self.current_token == Token::OpenBracket {
            self.advance();
            if let Token::NumberLiteral(n) = self.current_token {
                size = n.parse().unwrap_or(1);
                self.advance();
            }
            self.expect(Token::CloseBracket)?;
        }

        Ok(Stmt::Memory { name, ty, size })
    }

    fn parse_evolve_block(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // evolve
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Evolve { body })
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
        let value = if self.current_token != Token::Semicolon && self.current_token != Token::CloseBrace {
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
        self.parse_expression_with_precedence(0)
    }

    fn parse_expression_with_precedence(&mut self, precedence: u8) -> Result<Expr<'a>, String> {
        let mut left = self.parse_prefix()?;

        while precedence < self.get_precedence() {
            let op_token = self.current_token;
            self.advance();
            let right = self.parse_expression_with_precedence(Self::token_precedence(op_token))?;
            
            let op_str = match op_token {
                Token::Plus => "+",
                Token::Minus => "-",
                Token::Star => "*",
                Token::Slash => "/",
                Token::Equals => "==",
                _ => return Err("Unexpected binary operator".to_string()),
            };

            left = Expr::BinaryOp {
                left: Box::new(left),
                op: op_str,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr<'a>, String> {
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
            Token::OpenParen => {
                self.advance();
                let expr = self.parse_expression()?;
                self.expect(Token::CloseParen)?;
                Ok(expr)
            }
            _ => Err(format!("Unexpected token in expression prefix: {:?}", self.current_token))
        }
    }

    fn get_precedence(&self) -> u8 {
        Self::token_precedence(self.current_token)
    }

    fn token_precedence(token: Token<'a>) -> u8 {
        match token {
            Token::Equals => 1,
            Token::Plus | Token::Minus => 2,
            Token::Star | Token::Slash => 3,
            _ => 0,
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
