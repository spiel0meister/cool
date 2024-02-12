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
    Newline,
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
        self.content.chars().nth(self.index + offset)
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

    fn parse_number(&mut self, line: usize, col: usize) -> Result<(Token, usize)> {
        let mut buf = String::new();
        buf.push(self.consume()?);
        let mut is_float = false;
        let mut col_delta = 0usize;

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
            col_delta += 1;
        }

        Ok((
            if is_float {
                Token(TokenType::Float(buf), Loc(col, line))
            } else {
                Token(TokenType::Int(buf), Loc(col, line))
            },
            col_delta + 1,
        ))
    }

    fn parse_string(&mut self, line: usize, col: usize) -> Result<(Token, usize)> {
        self.consume()?;
        let mut buf = String::new();
        let mut col_delta = 0usize;

        while self.peek(0).is_some_and(|c| c != '"') {
            if self.peek(0).unwrap() == '\n' {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Un-allowed newline at {}:{}", line, col),
                ));
            }
            buf.push(self.consume()?);
            col_delta += 1;
        }
        self.consume()?;

        Ok((
            (Token(TokenType::String(buf), Loc(line, col))),
            col_delta + 1,
        ))
    }

    fn parse_ident(&mut self, line: usize, col: usize) -> Result<(Token, usize)> {
        let mut buf = String::new();
        buf.push(self.consume()?);
        let mut col_delta = 0usize;

        while self
            .peek(0)
            .is_some_and(|c| c.is_alphabetic() && !c.is_whitespace() && c != '=')
        {
            buf.push(self.consume()?);
            col_delta += 1;
        }

        Ok((
            (Token(TokenType::Ident(buf), Loc(line, col))),
            col_delta + 1,
        ))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut line = 1usize;
        let mut col = 1usize;

        while self.peek(0).is_some() {
            let c = self.peek(0).unwrap();

            if c == '\n' {
                line += 1;
                col = 1;
                self.tokens.push(Token(TokenType::Newline, Loc(line, col)));
                self.consume()?;
            } else {
                if c.is_whitespace() {
                    self.consume()?;
                } else if c.is_digit(10) {
                    let (t, d) = self.parse_number(line, col)?;
                    self.tokens.push(t);
                    col += d;
                } else if c.is_alphabetic() {
                    let (t, d) = self.parse_ident(line, col)?;
                    self.tokens.push(t);
                    col += d;
                } else if c == '"' {
                    let (t, d) = self.parse_string(line, col)?;
                    self.tokens.push(t);
                    col += d;
                } else if c == '{' {
                    self.tokens
                        .push(Token(TokenType::LeftBrace, Loc(line, col)));
                    self.consume()?;
                } else if c == '}' {
                    self.tokens
                        .push(Token(TokenType::RightBrace, Loc(line, col)));
                    self.consume()?;
                } else if c == '=' {
                    self.tokens.push(Token(TokenType::Equals, Loc(line, col)));
                    self.consume()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unexpected character {:?} at {}:{}", c, line, col,),
                    ));
                }

                col += 1;
            }
        }

        Ok(self.tokens.to_vec())
    }
}
