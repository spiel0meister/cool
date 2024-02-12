use std::io::Result;

fn get_file_path() -> String {
    use std::env::args;

    let args_: Vec<String> = args().collect();

    args_
        .get(1)
        .unwrap_or_else(|| panic!("Expected file!"))
        .to_string()
}

fn main() -> Result<()> {
    let file_path = get_file_path();
    let data = cool::load(&file_path)?;
    let ip = data.get_string("ip")?;

    println!("ip: {}", ip);

    Ok(())
}
