use crate::lexer::{Lexer, Token};
use crate::ast::Node;

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
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        self.expect(Token::CloseBrace)?;

        Ok(Node::Function { name, params: Vec::new(), body: Vec::new() })
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
        self.expect(Token::CloseBrace)?;
        Ok(Node::Test { name, body: Vec::new() })
    }
}
