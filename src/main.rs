use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{Error, prelude::*};

enum FileType {
    Small(File),
    Large(File),
}

impl FileType {
    fn new(file_path: &String) -> Result<FileType, std::io::Error> {
        let file = File::open(file_path)?;
        let file_size = file.metadata()?.len();
        if file_size > 50_000_000 {
            return Ok(FileType::Large(file));
        }
        Ok(FileType::Small(file))
    }
}

struct Parser {
    pattern: String,
    pattern_len: usize,
    matches: Vec<String>,
    file_type: FileType,
}

impl Parser {
    fn new(file_path: &String, target_str: &String) -> Result<Parser, std::io::Error> {
        let file: FileType = FileType::new(file_path)?;
        Ok(Parser {
            pattern: target_str.clone(),
            pattern_len: target_str.len(),
            matches: Vec::new(),
            file_type: file,
        })
    }

    fn call(&self) {
        match self.file_type {
            FileType::Small(_) => self.parse_small(),
            FileType::Large(_) => self.parse_large(),
        }
        panic!("Not a Small or Large File?");
    }

    fn parse_small(&self) {}
    fn parse_large(&self) {}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: rusty_grep <pattern> <file>");
    }
    run(&args)?;
    Ok(())
}

fn run(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let parser: Parser = Parser::new(&args[1], &args[2])?;

    Ok(())
}
