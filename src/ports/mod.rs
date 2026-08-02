use std::fmt::Display;

pub mod parser;

#[derive(Debug)]
pub enum ParsePortError {
    InvalidPort,
    InvalidRange,
    EmptyInput,
}

impl Display for ParsePortError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsePortError::EmptyInput => {
                write!(f, "Empty input")
            }

            ParsePortError::InvalidPort => {
                write!(f, "Invalid port")
            }

            ParsePortError::InvalidRange => {
                write!(f, "Invalid range")
            }
        }
    }
}
