pub mod parser;

#[derive(Debug)]
pub enum ParsePortError {
    InvalidPort,
    InvalidRange,
    EmptyInput,
}
