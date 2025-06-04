mod constants;
mod parser;

pub use constants::*;
pub use parser::*;

#[derive(Debug)]
pub enum FirmataError {
    ConnectionError(std::io::Error),
}

impl std::fmt::Display for FirmataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirmataError::ConnectionError(e) => write!(f, "Connection error: {}", e),
        }
    }
}

impl std::error::Error for FirmataError {}
