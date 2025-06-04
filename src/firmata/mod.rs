mod constants;
mod parser;

pub use constants::*;
pub use parser::*;

#[derive(Debug)]
pub enum FirmataError {
    ConnectionError(std::io::Error),
    ConnectionClosed,
}

impl std::fmt::Display for FirmataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FirmataError::ConnectionError(e) => write!(f, "Connection error: {}", e),
            FirmataError::ConnectionClosed => write!(f, "Connection closed by peer"),
        }
    }
}

impl std::error::Error for FirmataError {}
