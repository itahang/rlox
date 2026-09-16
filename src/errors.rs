use std::io;
use thiserror::Error;
#[derive(Debug, Error)]
#[error("[line {line} {location}: {message}]")]
pub struct ScannerError {
    line: usize,
    location: String,
    message: String,
}
impl ScannerError {
    pub fn new(line: usize, location: String, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }
}

#[derive(Debug, Error)]
#[error("[line {line} {location}: {message}]")]
pub struct ParserError {
    line: usize,
    location: String,
    message: String,
}
impl ParserError {
    pub fn new(line: usize, location: String, message: String) -> Self {
        Self {
            line,
            location,
            message,
        }
    }
}

#[derive(Debug, Error)]
pub enum LoxError {
    #[error("IO ERROR: {0}")]
    Io(#[from] io::Error),
    #[error("Scanner Error: {0}")]
    Scan(#[from] ScannerError),
    #[error("Parser Error: {0}")]
    Parse(#[from] ParserError),
}

pub fn report_err(line: usize, wher: &str, message: &str) {
    eprintln!("[line {} ] {} : {}", line, wher, message);
}

pub fn report(lerror: LoxError) -> LoxError {
    eprintln!("{:?}", lerror);

    return lerror;
}
