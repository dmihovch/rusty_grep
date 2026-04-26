use std::fs::File;

enum FileType {
    Small(File),
    Large(File),
}

impl FileType {
    fn new(file_path: &str) -> Result<FileType, std::io::Error> {
        let file = File::open(file_path)?;
        let file_size = file.metadata()?.len();
        if file_size > 50_000_000 {
            return Ok(FileType::Large(file));
        }
        Ok(FileType::Small(file))
    }
}

struct Parser<'a> {
    pattern: &'a str,
    pattern_len: usize,
    matches: Vec<String>,
    file_type: FileType,
}

impl<'a> Parser<'a> {
    fn new(pattern: &'a str, file_path: &'a str) -> Result<Parser<'a>, std::io::Error> {
        let file: FileType = FileType::new(file_path)?;
        Ok(Parser {
            pattern: pattern,
            pattern_len: pattern.len(),
            matches: Vec::new(),
            file_type: file,
        })
    }

    fn call(&self) {
        match self.file_type {
            FileType::Small(_) => self.parse_small(),
            FileType::Large(_) => self.parse_large(),
            _ => panic!("Not a Small or Large File"),
        }
    }

    fn parse_small(&self) {
        println!("Parsing a small file");
    }
    fn parse_large(&self) {
        println!("Parsing a large file");
    }
}

pub fn run(pattern: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let parser: Parser = Parser::new(pattern, file_path)?;
    parser.call();

    Ok(())
}
