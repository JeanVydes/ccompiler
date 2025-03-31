pub mod token;
pub mod scanner;

use clap::Parser;
use scanner::Scanner;
use std::fs::File;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    file_path: String,
}

fn main() {
    run().unwrap_or_else(|_| {
        println!("Algo salio mal! :/")
    });
}

fn run() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let file_path = args.file_path;
    let file = File::open(&file_path)?;
    let mut scanner = Scanner::new();

    scanner.scan_file(file)?;
    for token in scanner.get_tokens() {
        println!("Token: {} \"{}\" linea {} columna {}", token.to_string(), token.lexeme.clone(), token.line, token.column);
    }

    return Ok(());
}
