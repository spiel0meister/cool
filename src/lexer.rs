use std::io::{Error, ErrorKind, Result};

#[derive(Debug, Clone)]
pub enum TokenType {
    Ident(String),
    Equals,
    String(String),
    Int(String),
    Float(String),
    LeftBrace,
    RightBrace,
}

#[derive(Debug, Clone)]
pub struct Loc(usize, usize);

#[derive(Debug, Clone)]
pub struct Token(TokenType, Loc);

pub struct Tokenizer {
    content: String,
    tokens: Vec<Token>,
    index: usize,
}

impl Tokenizer {
    pub fn new(content: impl Into<String>) -> Self {
        let content = Into::into(content);
        Self {
            content,
            tokens: Vec::new(),
            index: 0,
        }
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.content.chars().nth(offset)
    }

    fn consume(&mut self) -> Result<char> {
        let c = self
            .content
            .chars()
            .nth(self.index)
            .ok_or(Error::new(ErrorKind::UnexpectedEof, "End of content!"));
        self.index += 1;
        c
    }

    fn parse_number(&mut self, line: usize, col: usize) -> Result<Token> {
        let mut buf = String::new();
        buf.push(self.consume()?);
        let mut is_float = false;

        while self.peek(0).is_some_and(|c| c.is_digit(10) || c == '.') {
            let c = self.peek(0).unwrap();
            if c == '.' {
                if is_float {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Double period `.` at {}:{}", line, col),
                    ));
                }
                is_float = true;
            }
            buf.push(c);
            self.consume()?;
        }

        Ok(if is_float {
            Token(TokenType::Float(buf), Loc(col, line))
        } else {
            Token(TokenType::Int(buf), Loc(col, line))
        })
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut line = 1usize;
        let mut col = 1usize;

        while self.peek(0).is_some() {
            let c = self.peek(0).unwrap();

            if c == '\n' {
                line += 1;
                col = 1;
            } else {
                if c.is_whitespace() {
                    self.consume()?;
                } else if c.is_digit(10) {
                    let t = self.parse_number(line, col)?;
                    self.tokens.push(t);
                }

                col += 1;
            }
        }

        Ok(self.tokens.to_vec())
    }
}
