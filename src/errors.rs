use std::io;

pub static mut HAD_ERROR: bool = false;

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

#[derive(Debug)]
pub enum LoxError {
    Io(io::Error),
    Scan(ScannerError),
}
impl From<io::Error> for LoxError {
    fn from(error: io::Error) -> Self {
        LoxError::Io(error)
    }
}

pub fn error(line: usize, message: &str) {
    report(line, &String::new(), message);
    unsafe {
        HAD_ERROR = true;
    }
}

pub fn report(line: usize, location: &str, message: &str) -> ScannerError {
    let err = ScannerError::new(line, location.to_string(), message.to_string());
    eprintln!("[Line: {} ] Error {}:  {}", line, location, message);
    return err;
}
