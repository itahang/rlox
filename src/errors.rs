use std::error::Error;
use std::fmt;
use std::{fmt::Display, io};

pub static mut HAD_ERROR: bool = false;

trait LError {
    fn get_line(&self) -> usize;
    fn get_location(&self) -> String;
    fn get_message(&self) -> String;
}

#[derive(Debug)]
#[allow(unused)]
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

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Line: {}] Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl Error for ScannerError {}

impl LError for ScannerError{
    fn get_line(&self) -> usize {
        self.line
    }
    fn get_location(&self) -> String {
        self.location.clone()
    }
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

#[derive(Debug)]
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Line: {}] Error {}: {}",
            self.line, self.location, self.message
        )
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    Scan(ScannerError),
    Parse(ParserError),
}
impl From<io::Error> for LoxError {
    fn from(error: io::Error) -> Self {
        LoxError::Io(error)
    }
}
impl From<ScannerError> for LoxError {
    fn from(error: ScannerError) -> Self {
        LoxError::Scan(error)
    }
}
impl From<ParserError> for LoxError {
    fn from(error: ParserError) -> Self {
        LoxError::Parse(error)
    }
}

pub fn error(line: usize, lerror: LoxError) -> LoxError {
    unsafe {
        HAD_ERROR = true;
    }
    report(line, &String::new(), lerror)
}

pub fn report(lerror: LoxError) -> LoxError {
    unsafe {
        HAD_ERROR = true;
    }

    eprintln!("[Line: {} ] Error {}:  {}", lerror, location,);

    return lerror.into();
}
