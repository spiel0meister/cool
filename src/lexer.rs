use std::{
    fmt::Display,
    io::{Error, ErrorKind, Result},
};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Ident(String),
    Equals,
    String(String),
    Int(String),
    Float(String),
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Newline,
}

#[derive(Debug, Clone)]
/// Location of a token in form (col, line).
pub struct Loc(pub usize, pub usize);

impl Display for Loc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.1, self.0)
    }
}

#[derive(Debug, Clone)]
pub struct Token(pub TokenType, pub Loc);

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenType::Equals => write!(f, "="),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::Newline => write!(f, "\n"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Comma => write!(f, ","),
            TokenType::Ident(val)
            | TokenType::Int(val)
            | TokenType::Float(val)
            | TokenType::String(val) => {
                write!(f, "{:?}", val)
            }
        }
    }
}

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
            (Token(TokenType::String(buf), Loc(col, line))),
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
            (Token(TokenType::Ident(buf), Loc(col, line))),
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
                self.tokens.push(Token(TokenType::Newline, Loc(col, line)));
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
                        .push(Token(TokenType::LeftBrace, Loc(col, line)));
                    self.consume()?;
                } else if c == '}' {
                    self.tokens
                        .push(Token(TokenType::RightBrace, Loc(col, line)));
                    self.consume()?;
                } else if c == '=' {
                    self.tokens.push(Token(TokenType::Equals, Loc(col, line)));
                    self.consume()?;
                } else if c == '[' {
                    self.tokens
                        .push(Token(TokenType::LeftBracket, Loc(col, line)));
                    self.consume()?;
                } else if c == ']' {
                    self.tokens
                        .push(Token(TokenType::RightBracket, Loc(col, line)));
                    self.consume()?;
                } else if c == ',' {
                    self.tokens.push(Token(TokenType::Comma, Loc(col, line)));
                    self.consume()?;
                } else {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("Unexpected character {:?} at {}:{}", c, line, col),
                    ));
                }

                col += 1;
            }
        }

        Ok(self.tokens.to_vec())
    }
}
