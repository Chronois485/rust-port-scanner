pub mod parser;

pub enum ParsePortError {
    InvalidPort,
    InvalidRange,
    EmptyInput,
}
