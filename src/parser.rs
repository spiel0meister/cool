use std::{
    collections::HashMap,
    fmt::Display,
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

impl Display for CoolDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoolDataType::Int(val) => write!(f, "{}", val),
            CoolDataType::Float(val) => write!(f, "{}", val),
            CoolDataType::String(val) => write!(f, "{:?}", val),
            CoolDataType::Object(val) => write!(f, "{{\n{}}}", val),
            CoolDataType::List(val) => write!(f, "{}", val),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CoolDataObject(HashMap<String, CoolDataType>);

macro_rules! impl_get {
    ($func_name:ident, $func_mut_name:ident, $data_type:ident, $type:ty) => {
        pub fn $func_name(&self, name: &str) -> Result<&$type> {
            let CoolDataType::$data_type(val) = self.get_field(name)? else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Field {:?} is not a {}.", name, stringify!($type)),
                ));
            };
            Ok(val)
        }

        pub fn $func_mut_name(&mut self, name: &str) -> Result<&mut $type> {
            let CoolDataType::$data_type(val) = self.get_field_mut(name)? else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Field {:?} is not a {}.", name, stringify!($type)),
                ));
            };
            Ok(val)
        }
    };
}

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

    pub fn get_field_mut(&mut self, name: &str) -> Result<&mut CoolDataType> {
        self.0.get_mut(name).ok_or(Error::new(
            ErrorKind::InvalidInput,
            format!("Unknown field {:?}", name),
        ))
    }

    impl_get!(get_string, get_string_mut, String, String);
    impl_get!(get_int, get_int_mut, Int, i32);
    impl_get!(get_float, get_float_mut, Float, f32);
    impl_get!(get_object, get_object_mut, Object, CoolDataObject);
    impl_get!(get_list, get_list_mut, List, CoolDataList);
}

impl IntoIterator for CoolDataObject {
    type Item = (String, CoolDataType);
    type IntoIter = std::collections::hash_map::IntoIter<String, CoolDataType>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Display for CoolDataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.0.iter() {
            writeln!(f, "{} = {}", key, value)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct CoolDataList(Vec<CoolDataType>);

macro_rules! impl_at {
    ($func_name:ident, $func_mut_name:ident, $data_type:ident, $type:ty) => {
        pub fn $func_name(&self, index: usize) -> Result<&$type> {
            let CoolDataType::$data_type(val) = self.at(index)? else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Index {} is not a {}.", index, stringify!($type)),
                ));
            };
            Ok(val)
        }

        pub fn $func_mut_name(&mut self, index: usize) -> Result<&mut $type> {
            let CoolDataType::$data_type(val) = self.at_mut(index)? else {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("Index {} is not a {}.", index, stringify!($type)),
                ));
            };
            Ok(val)
        }
    };
}

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

    impl_at!(string_at, string_at_mut, String, String);
    impl_at!(int_at, int_at_mut, Int, i32);
    impl_at!(float_at, float_at_mut, Float, f32);
    impl_at!(object_at, object_at_mut, Object, CoolDataObject);
    impl_at!(list_at, list_at_mut, List, CoolDataList);
}

impl Display for CoolDataList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for value in self.0.iter() {
            writeln!(f, "{}", value)?;
        }
        Ok(())
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
