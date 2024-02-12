use std::io::Result;
pub mod lexer;
pub mod parser;

pub fn load(file_path: &str) -> Result<parser::CoolData> {
    use std::fs::read_to_string;
    let content = read_to_string(file_path)?;
    let mut tokenizer = lexer::Tokenizer::new(content);
    let tokens = tokenizer.tokenize().unwrap();

    let mut parser = parser::Parser::new(tokens);
    parser.parse()
}

pub fn parse(content: impl Into<String>) -> Result<parser::CoolData> {
    let mut tokenizer = lexer::Tokenizer::new(content);
    let tokens = tokenizer.tokenize().unwrap();

    let mut parser = parser::Parser::new(tokens);
    parser.parse()
}
