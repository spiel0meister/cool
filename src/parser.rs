use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::lexer::*;

#[derive(Debug, Clone)]
pub enum CoolDataType {
    Int(i32),
    Float(f32),
    String(String),
    Object(CoolData),
    List(Vec<CoolDataType>),
}

impl CoolDataType {
    pub fn int(val: &str) -> Self {
        Self::Int(val.parse().expect("Invalid value for int."))
    }

    pub fn float(val: &str) -> Self {
        Self::Float(val.parse().expect("Invalid value for float."))
    }
}

#[derive(Debug, Clone)]
pub struct CoolData(HashMap<String, CoolDataType>);

impl CoolData {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_field(&mut self, name: String, value: CoolDataType) {
        self.0.insert(name, value);
    }

    pub fn get_field(&self, name: &str) -> Result<&CoolDataType> {
        self.0.get(name).ok_or(Error::new(
            ErrorKind::InvalidInput,
            format!("Unknown field {:?}", name),
        ))
    }

    pub fn get_string(&self, name: &str) -> Result<String> {
        let CoolDataType::String(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not a string.", name),
            ));
        };
        Ok(val.clone())
    }

    pub fn get_int(&self, name: &str) -> Result<i32> {
        let CoolDataType::Int(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not an int.", name),
            ));
        };
        Ok(val.clone())
    }

    pub fn get_float(&self, name: &str) -> Result<f32> {
        let CoolDataType::Float(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not an float.", name),
            ));
        };
        Ok(val.clone())
    }

    pub fn get_object(&self, name: &str) -> Result<&CoolData> {
        let CoolDataType::Object(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not an object.", name),
            ));
        };
        Ok(val)
    }

    pub fn get_list(&self, name: &str) -> Result<&Vec<CoolDataType>> {
        let CoolDataType::List(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not a list.", name),
            ));
        };
        Ok(val)
    }
}

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, index: 0 }
    }

    fn peek(&self, offset: usize) -> Option<&Token> {
        self.tokens.get(self.index + offset)
    }

    fn consume(&mut self) -> Result<&Token> {
        let t = self
            .tokens
            .get(self.index)
            .ok_or(Error::new(ErrorKind::UnexpectedEof, "End of tokens!"));
        self.index += 1;
        t
    }

    fn parse_object(&mut self) -> Result<CoolData> {
        let mut out = CoolData::new();

        while self
            .peek(0)
            .is_some_and(|Token(tt, _)| tt != &TokenType::RightBrace)
        {
            let t = self.peek(0).unwrap().clone();
            let Token(ref token_type, loc) = t;

            match &token_type {
                TokenType::Ident(name) => {
                    self.consume()?;
                    let Some(Token(TokenType::Equals, _)) = self.peek(0) else {
                        let Some(Token(tt, loc)) = self.peek(0) else {
                            unreachable!();
                        };
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Exptected `=`, got `{}` at {}:{}", tt, loc.1, loc.0),
                        ));
                    };
                    self.consume()?;

                    if let Some(Token(TokenType::LeftBrace, _)) = self.peek(0) {
                        self.consume()?;
                        let val = self.parse_object()?;
                        let Some(Token(TokenType::RightBrace, _)) = self.peek(0) else {
                            return Err(Error::new(
                                ErrorKind::InvalidData,
                                format!("Exptected `}}`, got `{}` at {}:{}", t.0, loc.1, loc.0),
                            ));
                        };
                        self.consume()?;
                        out.add_field(name.clone(), CoolDataType::Object(val));
                    } else {
                        let Some(Token(token_type, _)) = self.peek(0) else {
                            return Err(Error::new(ErrorKind::UnexpectedEof, "End of tokens!"));
                        };
                        let data_type = match &token_type {
                            TokenType::Int(val) => CoolDataType::int(val.as_str()),
                            TokenType::Float(val) => CoolDataType::float(val.as_str()),
                            TokenType::String(val) => CoolDataType::String(val.to_string()),
                            other => unreachable!("{:?}", other),
                        };
                        self.consume()?;
                        out.add_field(name.clone(), data_type);
                    }
                }
                TokenType::Newline => {
                    self.consume()?;
                }
                other => unreachable!("{:?}", other),
            }
        }

        Ok(out)
    }

    pub fn parse(&mut self) -> Result<CoolData> {
        self.parse_object()
    }
}
