use std::fmt::Display;

pub mod parser;

#[derive(Debug)]
pub enum ParsePortError {
    InvalidPort(String),
    InvalidRange(u16, u16),
    EmptyInput,
}

impl Display for ParsePortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePortError::EmptyInput => {
                write!(f, "Empty input")
            }

            ParsePortError::InvalidPort(port) => {
                write!(f, "Invalid port: {}", port)
            }

            ParsePortError::InvalidRange(start, end) => {
                write!(f, "Invalid range: {}-{}", start, end)
            }
        }
    }
}
