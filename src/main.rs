use std::io::Result;

mod lexer;

fn load_input_file() -> Result<String> {
    use std::env::args;
    use std::fs::read_to_string;

    let args_: Vec<String> = args().collect();

    let file_path = args_
        .get(1)
        .unwrap_or_else(|| panic!("Expected file!"))
        .to_string();

    read_to_string(&file_path)
}

fn main() -> Result<()> {
    let content = load_input_file()?;
    let mut lexer = lexer::Tokenizer::new(content);
    let tokens = lexer.tokenize()?;

    println!("{:?}", tokens);

    Ok(())
}
