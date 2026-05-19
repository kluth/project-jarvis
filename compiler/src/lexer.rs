#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Module,
    Complexity,
    Func,
    Verify,
    Test,
    Identifier(&'a str),
    BigO(&'a str),
    StringLiteral(&'a str),
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Arrow,
    Eof,
    Unknown,
}

pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize,
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

        if c.is_alphabetic() {
            return self.lex_identifier_or_keyword();
        }

        match c {
            '"' => self.lex_string_literal(),
            '{' => { self.consume(); Token::OpenBrace }
            '}' => { self.consume(); Token::CloseBrace }
            '(' => { self.consume(); Token::OpenParen }
            ')' => { self.consume(); Token::CloseParen }
            _ => { self.consume(); Token::Unknown }
        }
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
            _ => Token::Identifier(text),
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.cursor..].chars().next()
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

    fn peek_next(&self) -> Option<char> {
        self.source[self.cursor..].chars().nth(1)
    }
}
