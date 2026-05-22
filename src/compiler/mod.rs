pub mod entry;
pub mod parsing;
pub mod operand_checking;

#[derive(Debug)]
pub enum ParseError {
    InvalidRegister(String),
    InvalidNumber(String),
    InvalidOperand(String),
    InvalidAddress(String),
    UnknownOpcode(String),
}