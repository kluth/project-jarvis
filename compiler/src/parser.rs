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
        let mut body = Vec::new();

        while self.current_token == Token::Import {
            self.advance();
            if let Token::StringLiteral(path) = self.current_token {
                body.push(Node::Import { path });
                self.advance();
            } else if let Token::Identifier(path) = self.current_token {
                body.push(Node::Import { path });
                self.advance();
            }
        }

        self.expect(Token::Module)?;
        
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected module name".to_string());
        };

        while self.current_token != Token::Eof {
            match self.current_token {
                Token::Import => {
                    self.advance();
                    if let Token::StringLiteral(path) = self.current_token {
                        body.push(Node::Import { path });
                        self.advance();
                    } else if let Token::Identifier(path) = self.current_token {
                        body.push(Node::Import { path });
                        self.advance();
                    }
                }
                Token::Complexity => {
                    body.push(self.parse_complexity_block()?);
                }
                Token::Verify => {
                    body.push(self.parse_verify_block()?);
                }
                _ => self.advance(),
            }
        }

        Ok(Node::Module { name, body })
    }

    fn parse_attributes(&mut self) -> Result<Vec<crate::ast::Attribute<'a>>, String> {
        let mut attrs = Vec::new();
        while self.current_token == Token::At {
            self.advance();
            match self.current_token {
                Token::Interrupt => {
                    self.advance();
                    self.expect(Token::OpenParen)?;
                    match self.current_token {
                        Token::Identifier(id) | Token::NumberLiteral(id) | Token::StringLiteral(id) => {
                            attrs.push(crate::ast::Attribute::Interrupt(id));
                            self.advance();
                        }
                        _ => return Err("Expected identifier, number, or string in @interrupt".to_string()),
                    }
                    self.expect(Token::CloseParen)?;
                }
                Token::NoMangle => {
                    attrs.push(crate::ast::Attribute::NoMangle);
                    self.advance();
                }
                Token::Section => {
                    self.advance();
                    self.expect(Token::OpenParen)?;
                    if let Token::StringLiteral(s) = self.current_token {
                        attrs.push(crate::ast::Attribute::Section(s));
                        self.advance();
                    }
                    self.expect(Token::CloseParen)?;
                }
                _ => return Err(format!("Unexpected attribute: {:?}", self.current_token)),
            }
        }
        Ok(attrs)
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
        let mut last_verify = None;

        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            match self.current_token {
                Token::Verify => {
                    last_verify = Some(self.parse_verify_block()?);
                }
                Token::Struct => {
                    content.push(self.parse_struct_definition()?);
                }
                Token::Static => {
                    content.push(self.parse_static_block()?);
                }
                Token::At | Token::Func => {
                    let attrs = self.parse_attributes()?;
                    let mut func = self.parse_function(attrs)?;
                    if let Node::Function { ref mut verification, .. } = func {
                        *verification = last_verify.take().map(Box::new);
                    }
                    content.push(func);
                }
                Token::Render => {
                    let mut render = self.parse_render_block()?;
                    if let Node::Render { ref mut verification, .. } = render {
                        *verification = last_verify.take().map(Box::new);
                    }
                    content.push(render);
                }
                Token::Allocator => {
                    content.push(self.parse_allocator_block()?);
                }
                Token::Hologram => {
                    content.push(self.parse_hologram_block()?);
                }
                Token::PostProcess => {
                    content.push(self.parse_post_process_block()?);
                }
                Token::NeuroAdapt => {
                    content.push(self.parse_neuro_adapt_block()?);
                }
                _ => self.advance(),
            }
        }
        
        self.expect(Token::CloseBrace)?;
        Ok(Node::ComplexityBlock { complexity, content })
    }

    fn parse_struct_definition(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Struct)?;
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else {
            return Err("Expected struct name".to_string());
        };

        self.expect(Token::OpenBrace)?;
        let mut fields = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            if let Token::Identifier(fname) = self.current_token {
                self.advance();
                self.expect(Token::Colon)?;
                let fty = self.parse_type()?;
                fields.push((fname, fty));
                if self.current_token == Token::Comma {
                    self.advance();
                } else if self.current_token == Token::Semicolon {
                    self.advance();
                }
            } else {
                break;
            }
        }
        self.expect(Token::CloseBrace)?;
        Ok(Node::Struct { name, fields })
    }

    fn parse_static_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Static)?;
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "GLOBAL_STATE" };

        self.expect(Token::OpenBracket)?;
        let mut address = 0;
        if let Token::NumberLiteral(n) = self.current_token {
            address = if n.starts_with("0x") {
                usize::from_str_radix(&n[2..], 16).unwrap_or(0)
            } else {
                n.parse().unwrap_or(0)
            };
            self.advance();
        }
        self.expect(Token::CloseBracket)?;

        self.expect(Token::OpenBrace)?;
        let mut size = 0;
        if let Token::Identifier("size") = self.current_token {
            self.advance();
            self.expect(Token::Colon)?;
            if let Token::NumberLiteral(n) = self.current_token {
                size = n.parse().unwrap_or(0);
                self.advance();
            }
        }
        self.expect(Token::CloseBrace)?;

        Ok(Node::Static { name, address, size })
    }

    fn parse_function(&mut self, attributes: Vec<crate::ast::Attribute<'a>>) -> Result<Node<'a>, String> {
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
        
        Ok(Node::Function { name, params, return_ty, body, verification: None, attributes })
    }

    fn parse_type(&mut self) -> Result<Type<'a>, String> {
        match self.current_token {
            Token::TypeI32 => { self.advance(); Ok(Type::I32) },
            Token::TypeF32 => { self.advance(); Ok(Type::F32) },
            Token::TypeStream => { self.advance(); Ok(Type::Stream) },
            Token::TypePixelStream => { self.advance(); Ok(Type::PixelStream) },
            Token::TypeFrameBuffer => { self.advance(); Ok(Type::FrameBuffer) },
            Token::TypeVectorCanvas => { self.advance(); Ok(Type::VectorCanvas) },
            Token::Identifier(id) => { self.advance(); Ok(Type::Struct(id)) },
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
            Token::For => self.parse_for_statement(),
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
            Token::Window => self.parse_window_statement(),
            Token::Event => self.parse_event_block(),
            Token::Assert => self.parse_assert_statement(),
            Token::Layout => self.parse_layout_statement(),
            Token::Component => self.parse_component_statement_as_stmt(),
            Token::Poll => self.parse_poll_statement(),
            Token::Print => self.parse_print_statement(),
            Token::CaptureFrame => self.parse_capture_frame_statement(),
            Token::CaptureStream => self.parse_capture_stream_statement(),
            Token::Asm => self.parse_asm_block(),
            Token::Volatile => self.parse_volatile_op(),
            Token::Port => self.parse_port_op(),
            Token::Atomic => self.parse_atomic_op(),
            Token::Hologram => self.parse_hologram_statement(),
            Token::PostProcess => self.parse_post_process_statement(),
            Token::NeuroAdapt => self.parse_neuro_adapt_statement(),

            _ => {
                let expr = self.parse_expression()?;
                if self.current_token == Token::Semicolon {
                    self.advance();
                }
                Ok(Stmt::Expression { expr })
            }
        }
    }

    fn parse_layout_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Layout)?;
        let kind = match self.current_token {
            Token::Identifier(id) => { self.advance(); id }
            Token::StringLiteral(s) => { self.advance(); s }
            _ => "box"
        };

        self.expect(Token::OpenBrace)?;
        let mut content = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            content.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;
        Ok(Stmt::Layout { kind, content })
    }

    fn parse_component_statement_as_stmt(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Component)?;
        let kind = match self.current_token {
            Token::Identifier(id) => { self.advance(); id }
            Token::StringLiteral(s) => { self.advance(); s }
            _ => "Unknown"
        };

        self.expect(Token::OpenParen)?;
        let mut args = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
            args.push(self.parse_expression()?);
            if self.current_token == Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Component { kind, args })
    }

    fn parse_poll_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // poll
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Poll)
    }

    fn parse_print_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // print
        self.expect(Token::OpenParen)?;
        let value = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let x = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let y = self.parse_expression()?;
        self.expect(Token::Comma)?;
        let color = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Print { value, x, y, color })
    }

    fn parse_capture_frame_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // capture_frame
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::CaptureFrame)
    }

    fn parse_capture_stream_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // capture_stream
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::CaptureStream)
    }

    fn parse_asm_block(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Asm)?;
        self.expect(Token::OpenBrace)?;
        let start = self.lexer.cursor;
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            self.advance();
        }
        let block = &self.lexer.source[start..self.lexer.cursor];
        self.expect(Token::CloseBrace)?;
        Ok(Stmt::Asm { block })
    }

    fn parse_volatile_op(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Volatile)?;
        if let Token::Identifier("write") = self.current_token {
            self.advance();
            self.expect(Token::OpenParen)?;
            let address = self.parse_expression()?;
            self.expect(Token::Comma)?;
            let value = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            if self.current_token == Token::Semicolon { self.advance(); }
            Ok(Stmt::VolatileWrite { address, value })
        } else if let Token::Identifier("read") = self.current_token {
            self.advance();
            self.expect(Token::OpenParen)?;
            let address = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            self.expect(Token::Arrow)?;
            let dest = if let Token::Identifier(id) = self.current_token {
                self.advance();
                id
            } else { return Err("Expected destination identifier".to_string()); };
            if self.current_token == Token::Semicolon { self.advance(); }
            Ok(Stmt::VolatileRead { address, dest })
        } else {
            Err("Expected 'read' or 'write' after volatile".to_string())
        }
    }

    fn parse_port_op(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Port)?;
        if let Token::Identifier("write") = self.current_token {
            self.advance();
            self.expect(Token::OpenParen)?;
            let port = self.parse_expression()?;
            self.expect(Token::Comma)?;
            let value = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            if self.current_token == Token::Semicolon { self.advance(); }
            Ok(Stmt::PortWrite { port, value })
        } else if let Token::Identifier("read") = self.current_token {
            self.advance();
            self.expect(Token::OpenParen)?;
            let port = self.parse_expression()?;
            self.expect(Token::CloseParen)?;
            self.expect(Token::Arrow)?;
            let dest = if let Token::Identifier(id) = self.current_token {
                self.advance();
                id
            } else { return Err("Expected destination identifier".to_string()); };
            if self.current_token == Token::Semicolon { self.advance(); }
            Ok(Stmt::PortRead { port, dest })
        } else {
            Err("Expected 'read' or 'write' after port".to_string())
        }
    }

    fn parse_atomic_op(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Atomic)?;
        let op = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { return Err("Expected atomic operation name".to_string()); };

        self.expect(Token::OpenParen)?;
        let mut args = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
            args.push(self.parse_expression()?);
            if self.current_token == Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon { self.advance(); }
        Ok(Stmt::AtomicOp { op, args })
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
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Publish { target })
    }

    fn parse_window_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // window
        let title = if let Token::StringLiteral(s) = self.current_token {
            self.advance();
            s
        } else { "JARVIS Surface" };
        
        self.expect(Token::OpenBracket)?;
        let mut width = 1920;
        let mut height = 1080;
        if let Token::NumberLiteral(w) = self.current_token {
            width = w.parse().unwrap_or(1920);
            self.advance();
            self.expect(Token::Comma)?;
            if let Token::NumberLiteral(h) = self.current_token {
                height = h.parse().unwrap_or(1080);
                self.advance();
            }
        }
        self.expect(Token::CloseBracket)?;
        if self.current_token == Token::Semicolon { self.advance(); }
        Ok(Stmt::Window { title, width, height })
    }

    fn parse_event_block(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // event
        let kind = match self.current_token {
            Token::Identifier(id) => { self.advance(); id }
            Token::StringLiteral(s) => { self.advance(); s }
            _ => "any"
        };
        
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Event { kind, body })
    }

    fn parse_budget_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // budget
        self.expect(Token::OpenBrace)?;
        
        let mut limit = 0.0;
        if let Token::Identifier("power") = self.current_token {
            self.advance();
            self.expect(Token::Colon)?;
            if let Token::NumberLiteral(n) = self.current_token {
                limit = n.parse::<f32>().unwrap_or(0.0);
                self.advance();
                // Optionally consume '_nj' if lexed as separate identifier
                if let Token::Identifier(id) = self.current_token {
                    if id.contains("nj") { self.advance(); }
                }
            }
        }
        
        let mut body = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::CloseBrace)?;
        
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
                self.expect(Token::FatArrow)?;
                
                let branch_body = if self.current_token == Token::OpenBrace {
                    self.advance();
                    self.parse_block()?
                } else {
                    vec![self.parse_statement()?]
                };
                
                branches.push((weight, branch_body));
                
                if self.current_token == Token::Comma {
                    self.advance();
                }
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

    fn parse_for_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // for
        let var = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { return Err("Expected identifier in for loop".to_string()); };

        if let Token::Identifier("in") = self.current_token {
            self.advance();
        } else { return Err("Expected 'in' in for loop".to_string()); }

        let iterable = self.parse_expression()?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::For { var, iterable, body })
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
                Token::Less => "<",
                Token::Greater => ">",
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
            Token::Input => {
                self.advance();
                if self.current_token == Token::OpenParen {
                    self.advance();
                    self.expect(Token::CloseParen)?;
                }
                Ok(Expr::Input)
            }
            Token::Identifier(id) => {
                self.advance();
                if self.current_token == Token::OpenParen {
                    self.advance();
                    let mut args = Vec::new();
                    while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
                        args.push(self.parse_expression()?);
                        if self.current_token == Token::Comma {
                            self.advance();
                        }
                    }
                    self.expect(Token::CloseParen)?;
                    Ok(Expr::Call { name: id, args })
                } else if self.current_token == Token::Assign {
                    self.advance();
                    let value = self.parse_expression()?;
                    Ok(Expr::Assignment { name: id, value: Box::new(value) })
                } else {
                    Ok(Expr::Identifier(id))
                }
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
            Token::Equals | Token::Less | Token::Greater => 1,
            Token::Plus | Token::Minus => 2,
            Token::Star | Token::Slash => 3,
            _ => 0,
        }
    }

    fn parse_assert_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.advance(); // assert
        self.expect(Token::OpenParen)?;
        let condition = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Stmt::Assert { condition })
    }

    fn parse_render_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Render)?;
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "unnamed_render" };

        self.expect(Token::OpenParen)?;
        let mut params = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
            if let Token::Identifier(pname) = self.current_token {
                self.advance();
                if self.current_token == Token::Colon {
                    self.advance();
                    self.parse_type()?;
                }
                params.push(pname);
                if self.current_token == Token::Comma {
                    self.advance();
                }
            } else { break; }
        }
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        
        let body = self.parse_nodes()?;
        
        self.expect(Token::CloseBrace)?;
        Ok(Node::Render { name, params, body, verification: None })
    }

    fn parse_layout_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Layout)?;
        let kind = match self.current_token {
            Token::Identifier(id) => { self.advance(); id }
            Token::StringLiteral(s) => { self.advance(); s }
            _ => "box"
        };

        self.expect(Token::OpenBrace)?;
        let mut content = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            match self.current_token {
                Token::Layout => content.push(self.parse_layout_block()?),
                Token::Component => content.push(self.parse_component_statement()?),
                _ => self.advance(),
            }
        }
        self.expect(Token::CloseBrace)?;
        Ok(Node::Layout { kind, content })
    }

    fn parse_component_statement(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Component)?;
        let kind = match self.current_token {
            Token::Identifier(id) => { self.advance(); id }
            Token::StringLiteral(s) => { self.advance(); s }
            _ => "Unknown"
        };

        self.expect(Token::OpenParen)?;
        let mut args = Vec::new();
        while self.current_token != Token::CloseParen && self.current_token != Token::Eof {
            args.push(self.parse_expression()?);
            if self.current_token == Token::Comma {
                self.advance();
            }
        }
        self.expect(Token::CloseParen)?;
        if self.current_token == Token::Semicolon {
            self.advance();
        }
        Ok(Node::Component { kind, args })
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

    fn parse_allocator_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Allocator)?;
        let name = if let Token::Identifier(id) = self.current_token {
            self.advance();
            id
        } else { "Global" };
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Node::Allocator { name, body })
    }

    fn parse_hologram_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::Hologram)?;
        let kind = match self.current_token {
            Token::StringLiteral(s) | Token::Identifier(s) => { self.advance(); s }
            _ => "parallax"
        };
        self.expect(Token::OpenParen)?;
        let depth = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::Hologram { kind, depth, body })
    }

    fn parse_hologram_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::Hologram)?;
        let kind = match self.current_token {
            Token::StringLiteral(s) | Token::Identifier(s) => { self.advance(); s }
            _ => "parallax"
        };
        self.expect(Token::OpenParen)?;
        let depth = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let content = self.parse_nodes()?;
        self.expect(Token::CloseBrace)?;
        Ok(Node::Hologram { kind, depth, content })
    }

    fn parse_post_process_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::PostProcess)?;
        let effect = match self.current_token {
            Token::StringLiteral(s) | Token::Identifier(s) => { self.advance(); s }
            _ => "glitch"
        };
        self.expect(Token::OpenParen)?;
        let intensity = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::PostProcess { effect, intensity, body })
    }

    fn parse_post_process_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::PostProcess)?;
        let effect = match self.current_token {
            Token::StringLiteral(s) | Token::Identifier(s) => { self.advance(); s }
            _ => "glitch"
        };
        self.expect(Token::OpenParen)?;
        let intensity = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let content = self.parse_nodes()?;
        self.expect(Token::CloseBrace)?;
        Ok(Node::PostProcess { effect, intensity, content })
    }

    fn parse_neuro_adapt_statement(&mut self) -> Result<Stmt<'a>, String> {
        self.expect(Token::NeuroAdapt)?;
        self.expect(Token::OpenParen)?;
        let load = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let body = self.parse_block()?;
        Ok(Stmt::NeuroAdapt { load, body })
    }

    fn parse_neuro_adapt_block(&mut self) -> Result<Node<'a>, String> {
        self.expect(Token::NeuroAdapt)?;
        self.expect(Token::OpenParen)?;
        let load = self.parse_expression()?;
        self.expect(Token::CloseParen)?;
        self.expect(Token::OpenBrace)?;
        let content = self.parse_nodes()?;
        self.expect(Token::CloseBrace)?;
        Ok(Node::NeuroAdapt { load, content })
    }

    fn parse_nodes(&mut self) -> Result<Vec<Node<'a>>, String> {
        let mut nodes = Vec::new();
        while self.current_token != Token::CloseBrace && self.current_token != Token::Eof {
            match self.current_token {
                Token::Hologram => nodes.push(self.parse_hologram_block()?),
                Token::PostProcess => nodes.push(self.parse_post_process_block()?),
                Token::NeuroAdapt => nodes.push(self.parse_neuro_adapt_block()?),
                Token::Component => nodes.push(self.parse_component_statement()?),
                Token::Layout => nodes.push(self.parse_layout_block()?),
                _ => self.advance(),
            }
        }
        Ok(nodes)
    }
}
