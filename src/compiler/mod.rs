pub mod entry;
pub mod operands;

#[derive(Debug)]
pub enum ParseError {
    InvalidRegister(String),
    InvalidNumber(String),
    InvalidOperand(String),
}