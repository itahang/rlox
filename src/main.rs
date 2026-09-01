use std::{
    fs::File,
    io::{  Read, Write, stdin, stdout},
    path::Path,
};

use rlox::errors::*;
use rlox::scanner::Scanner;

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    for tok in tokens{
        println!("{}",tok.to_string())
    }

    Ok(())
}

fn run_file(path: &Path) -> Result<(), LoxError> {
    let mut source = String::new();
    File::open(path)?.read_to_string(&mut source)?;

    run(source)?;

    Ok(())
}

fn run_prompt() -> Result<(), LoxError> {
    loop {
        print!("> ");
        stdout().flush()?;

        let mut line = String::new();

        if stdin().read_line(&mut line)? == 0 {
            break;
        }
        if line.trim() == ".exit" {
            break;
        }

        run(line)?;
    }

    Ok(())
}

fn main() -> Result<(), LoxError> {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => run_prompt()?,

        2 => run_file(Path::new(&args[1]))?,

        _ => {
            eprintln!("Usage: lox [script]");
        }
    }

    Ok(())
}
