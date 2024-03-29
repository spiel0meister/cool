use std::io::{Result, Write};
pub mod lexer;
pub mod parser;

pub mod prelude {
    pub use super::parser::{CoolDataList, CoolDataObject, CoolDataType};
    pub use super::{load_from_file, parse, save_to_file};
}

pub fn load_from_file(file_path: &str) -> Result<parser::CoolDataObject> {
    use std::fs::read_to_string;
    let content = read_to_string(file_path)?;
    let mut tokenizer = lexer::Tokenizer::new(content);
    let tokens = tokenizer.tokenize()?;

    let mut parser = parser::Parser::new(tokens);
    parser.parse()
}

pub fn save_to_file(file_path: &str, object: &parser::CoolDataObject) -> Result<()> {
    use std::fs::File;
    let mut file = File::create(file_path)?;
    for (key, value) in object.clone().into_iter() {
        write!(file, "{} = {}\n", key, value)?;
    }
    file.flush()?;

    Ok(())
}

pub fn parse(content: impl Into<String>) -> Result<parser::CoolDataObject> {
    let mut tokenizer = lexer::Tokenizer::new(content);
    let tokens = tokenizer.tokenize()?;

    let mut parser = parser::Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod tests {
    #![allow(unused_imports)]
    use super::prelude::*;
    use std::io::Result;
}
