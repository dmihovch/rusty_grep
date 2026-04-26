mod parser;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = env::args();
    args.next().expect("Program Name");
    let pattern = args.next().expect("Please provide a pattern!");
    let file_path = args.next().expect("Please provide a file path!");
    /*
    if args.len() < 3 {
        panic!("Usage: rusty_grep <pattern> <file>");
    }
    */
    parser::run(&pattern, &file_path)?;
    Ok(())
}
