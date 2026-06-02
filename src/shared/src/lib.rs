pub mod registers;
pub mod operand_types;
pub mod opcodes;
pub mod bytereader;
pub mod helper;
pub mod consts;

#[derive(Debug)]
pub enum ParseError {
    InvalidRegister(String),
    InvalidNumber(String),
    InvalidOperand(String),
    InvalidAddress(String),
    UnknownOpcode(String),
}