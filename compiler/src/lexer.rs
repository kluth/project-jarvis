#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Module,
    Complexity,
    Func,
    Verify,
    Test,
    Let,
    Return,
    If,
    Else,
    For,
    While,
    Memory,
    Evolve,
    Neural,
    Budget,
    Prob,
    Sync,
    Swarm,
    Consensus,
    Gossip,
    Contract,
    Knowledge,
    Publish,
    Reflect,
    Assert,
    Render,
    Layout,
    Component,
    View,
    TypeI32,
    TypeF32,
    TypeStream,
    TypePixelStream,
    TypeFrameBuffer,
    TypeVectorCanvas,
    Identifier(&'a str),
    BigO(&'a str),
    StringLiteral(&'a str),
    NumberLiteral(&'a str),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    Equals,
    Less,
    Greater,
    Comma,
    Colon,
    Semicolon,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Arrow,
    Eof,
    Unknown,
}

pub struct Lexer<'a> {
    pub source: &'a str,
    pub cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        if self.cursor >= self.source.len() {
            return Token::Eof;
        }

        let c = self.peek().unwrap();

        if c.is_alphabetic() || c == '_' {
            return self.lex_identifier_or_keyword();
        }

        if c.is_digit(10) {
            return self.lex_number_literal();
        }

        match c {
            '"' => self.lex_string_literal(),
            '{' => { self.consume(); Token::OpenBrace }
            '}' => { self.consume(); Token::CloseBrace }
            '(' => { self.consume(); Token::OpenParen }
            ')' => { self.consume(); Token::CloseParen }
            '[' => { self.consume(); Token::OpenBracket }
            ']' => { self.consume(); Token::CloseBracket }
            '+' => { self.consume(); Token::Plus }
            '-' => { 
                self.consume();
                if self.peek() == Some('>') {
                    self.consume();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => { self.consume(); Token::Star }
            '/' => { self.consume(); Token::Slash }
            '=' => {
                self.consume();
                if self.peek() == Some('=') {
                    self.consume();
                    Token::Equals
                } else {
                    Token::Assign
                }
            }
            '<' => { self.consume(); Token::Less }
            '>' => { self.consume(); Token::Greater }
            ',' => { self.consume(); Token::Comma }
            ':' => { self.consume(); Token::Colon }
            ';' => { self.consume(); Token::Semicolon }
            _ => { 
                self.consume(); 
                Token::Unknown 
            }
        }
    }

    fn lex_number_literal(&mut self) -> Token<'a> {
        let start = self.cursor;
        while let Some(c) = self.peek() {
            if c.is_digit(10) || c == '.' {
                self.consume();
            } else {
                break;
            }
        }
        Token::NumberLiteral(&self.source[start..self.cursor])
    }

    fn lex_string_literal(&mut self) -> Token<'a> {
        self.consume(); // '"'
        let start = self.cursor;
        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.consume();
        }
        let text = &self.source[start..self.cursor];
        if self.peek() == Some('"') {
            self.consume(); // '"'
        }
        Token::StringLiteral(text)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token<'a> {
        let start = self.cursor;
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.consume();
            } else {
                break;
            }
        }

        let text = &self.source[start..self.cursor];

        // Complexity Special Handling for Big-O
        if text == "O" {
            if self.peek() == Some('(') {
                self.consume(); // '('
                let big_o_start = self.cursor;
                while let Some(c) = self.peek() {
                    if c != ')' { self.consume(); } else { break; }
                }
                let o_val = &self.source[big_o_start..self.cursor];
                if self.peek() == Some(')') {
                    self.consume(); // ')'
                }
                return Token::BigO(o_val);
            }
        }

        match text {
            "module" => Token::Module,
            "complexity" => Token::Complexity,
            "func" => Token::Func,
            "verify" => Token::Verify,
            "test" => Token::Test,
            "let" => Token::Let,
            "return" => Token::Return,
            "if" => Token::If,
            "else" => Token::Else,
            "for" => Token::For,
            "while" => Token::While,
            "memory" => Token::Memory,
            "evolve" => Token::Evolve,
            "neural" => Token::Neural,
            "budget" => Token::Budget,
            "prob" => Token::Prob,
            "sync" => Token::Sync,
            "swarm" => Token::Swarm,
            "consensus" => Token::Consensus,
            "gossip" => Token::Gossip,
            "contract" => Token::Contract,
            "knowledge" => Token::Knowledge,
            "publish" => Token::Publish,
            "reflect" => Token::Reflect,
            "i32" | "I32" => Token::TypeI32,
            "f32" | "F32" => Token::TypeF32,
            "Stream" => Token::TypeStream,
            "PixelStream" => Token::TypePixelStream,
            "FrameBuffer" => Token::TypeFrameBuffer,
            "VectorCanvas" => Token::TypeVectorCanvas,
            "render" => Token::Render,
            "layout" => Token::Layout,
            "component" => Token::Component,
            "view" => Token::View,
            "assert" => Token::Assert,
            _ => Token::Identifier(text),
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
    }

    fn peek_next(&self) -> Option<char> {
        self.source[self.cursor..].chars().nth(1)
    }

    fn consume(&mut self) {
        if let Some(c) = self.peek() {
            self.cursor += c.len_utf8();
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.consume();
            } else if c == '/' && self.peek_next() == Some('/') {
                self.consume(); // /
                self.consume(); // /
                while let Some(nc) = self.peek() {
                    if nc == '\n' {
                        break;
                    }
                    self.consume();
                }
            } else {
                break;
            }
        }
    }
}
