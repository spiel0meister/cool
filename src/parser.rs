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
    Object(CoolDataObject),
    List(CoolDataList),
}

impl CoolDataType {
    pub fn int(val: &str) -> Result<Self> {
        Ok(Self::Int(val.parse().map_err(|_| {
            Error::new(ErrorKind::InvalidInput, "Invalid value for int.")
        })?))
    }

    pub fn float(val: &str) -> Result<Self> {
        Ok(Self::Float(val.parse().map_err(|_| {
            Error::new(ErrorKind::InvalidInput, "Invalid value for float.")
        })?))
    }
}

#[derive(Debug, Clone)]
pub struct CoolDataObject(HashMap<String, CoolDataType>);

impl CoolDataObject {
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

    pub fn get_object(&self, name: &str) -> Result<&CoolDataObject> {
        let CoolDataType::Object(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not an object.", name),
            ));
        };
        Ok(val)
    }

    pub fn get_list(&self, name: &str) -> Result<&CoolDataList> {
        let CoolDataType::List(val) = self.get_field(name)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Field {:?} is not a list.", name),
            ));
        };
        Ok(val)
    }
}

#[derive(Debug, Clone)]
pub struct CoolDataList(Vec<CoolDataType>);

impl CoolDataList {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn at(&self, index: usize) -> Result<&CoolDataType> {
        self.0.get(index).ok_or(Error::new(
            ErrorKind::InvalidInput,
            format!("Index {} out of bounds.", index),
        ))
    }

    pub fn at_mut(&mut self, index: usize) -> Result<&mut CoolDataType> {
        self.0.get_mut(index).ok_or(Error::new(
            ErrorKind::InvalidInput,
            format!("Index {} out of bounds.", index),
        ))
    }

    pub fn string_at(&self, index: usize) -> Result<String> {
        let CoolDataType::String(val) = self.at(index)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Index {} is not a string.", index),
            ));
        };
        Ok(val.clone())
    }

    pub fn int_at(&self, index: usize) -> Result<i32> {
        let CoolDataType::Int(val) = self.at(index)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Index {} is not a int.", index),
            ));
        };
        Ok(val.clone())
    }

    pub fn float_at(&self, index: usize) -> Result<f32> {
        let CoolDataType::Float(val) = self.at(index)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Index {} is not a float.", index),
            ));
        };
        Ok(val.clone())
    }

    pub fn object_at(&self, index: usize) -> Result<&CoolDataObject> {
        let CoolDataType::Object(val) = self.at(index)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Index {} is not a object.", index),
            ));
        };
        Ok(val)
    }

    pub fn list_at(&self, index: usize) -> Result<&CoolDataList> {
        let CoolDataType::List(val) = self.at(index)? else {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!("Index {} is not a list.", index),
            ));
        };
        Ok(val)
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

    fn parse_list(&mut self) -> Result<CoolDataList> {
        self.consume()?;
        let mut out = CoolDataList::new();

        while self
            .peek(0)
            .is_some_and(|Token(tt, _)| tt != &TokenType::RightBracket)
        {
            let t = self.peek(0).unwrap().clone();
            let Token(ref token_type, loc) = t;

            if token_type == &TokenType::Comma {
                self.consume()?;
                // continue;
            }

            match &token_type {
                TokenType::Ident(_) => {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        format!("Expected `]`, got `{}` at {}:{}", token_type, loc.1, loc.0),
                    ));
                }
                TokenType::LeftBrace => {
                    self.consume()?;
                    let obj = self.parse_object()?;
                    let Token(TokenType::RightBrace, _) = self.consume()? else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Exptected `}}`, got `{}` at {}:{}", t.0, loc.1, loc.0),
                        ));
                    };
                    out.0.push(CoolDataType::Object(obj));
                }
                TokenType::LeftBracket => {
                    let list = self.parse_list()?;
                    let Some(Token(TokenType::RightBracket, _)) = self.peek(0) else {
                        return Err(Error::new(
                            ErrorKind::InvalidData,
                            format!("Exptected `]`, got `{}` at {}:{}", t.0, loc.1, loc.0),
                        ));
                    };
                    out.0.push(CoolDataType::List(list));
                }
                TokenType::Int(val) => {
                    out.0.push(CoolDataType::int(val)?);
                    self.consume()?;
                }
                TokenType::Float(val) => {
                    out.0.push(CoolDataType::float(val)?);
                    self.consume()?;
                }
                TokenType::String(val) => {
                    out.0.push(CoolDataType::String(val.to_string()));
                    self.consume()?;
                }
                TokenType::Newline => {
                    self.consume()?;
                }
                other => unreachable!("{:?}", other),
            }
        }
        self.consume()?;

        Ok(out)
    }

    fn parse_object(&mut self) -> Result<CoolDataObject> {
        let mut out = CoolDataObject::new();

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
                    } else if let Some(Token(TokenType::LeftBracket, _)) = self.peek(0) {
                        self.consume()?;
                        let val = self.parse_list()?;
                        out.add_field(name.clone(), CoolDataType::List(val));
                    } else {
                        let Some(Token(token_type, _)) = self.peek(0) else {
                            return Err(Error::new(ErrorKind::UnexpectedEof, "End of tokens!"));
                        };
                        let data_type = match &token_type {
                            TokenType::Int(val) => CoolDataType::int(val.as_str())?,
                            TokenType::Float(val) => CoolDataType::float(val.as_str())?,
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

    pub fn parse(&mut self) -> Result<CoolDataObject> {
        let mut out = CoolDataObject::new();
        while self.peek(0).is_some() {
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
                        let Token(TokenType::RightBrace, _) = self.consume()? else {
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
                            TokenType::Int(val) => CoolDataType::int(val.as_str())?,
                            TokenType::Float(val) => CoolDataType::float(val.as_str())?,
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
}
