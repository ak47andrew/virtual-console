use num_bigint::BigUint;
use num_traits::{Num, ToBytes};
use crate::compiler::ParseError;

pub enum Operand {
    Address(u64),
    Immediate(BigUint),
    Register(Registers),
    LongRegister(LongRegisters),
}

pub enum Registers {
    A, X, Y, Z,
    G1, G2, G3, G4, G5,
}

impl Registers {
    pub fn to_u8(&self) -> u8 {
        match self {
            Registers::A => {0x1}
            Registers::X => {0x2}
            Registers::Y => {0x3}
            Registers::Z => {0x4}
            Registers::G1 => {0xA1}
            Registers::G2 => {0xA2}
            Registers::G3 => {0xA3}
            Registers::G4 => {0xA4}
            Registers::G5 => {0xA5}
        }
    }
}

pub enum LongRegisters {
    PC, LL,
    GP1, GP2, GP3,
}

impl LongRegisters {
    pub fn to_u8(&self) -> u8 {
        match self {
            LongRegisters::PC => {0xB1}
            LongRegisters::LL => {0xB2}
            LongRegisters::GP1 => {0xC1}
            LongRegisters::GP2 => {0xC2}
            LongRegisters::GP3 => {0xC3}
        }
    }
}

pub fn parse_operand(token: String) -> Result<Operand, ParseError> {
    if token.starts_with("$") {
        return Ok(Operand::Address(parse_u64_num(token[1..].to_string())?))
    }
    if token.starts_with("!") {
        return match &token[1..] {
            "A" => Ok(Operand::Register(Registers::A)),
            "X" => Ok(Operand::Register(Registers::X)),
            "Y" => Ok(Operand::Register(Registers::Y)),
            "Z" => Ok(Operand::Register(Registers::Z)),
            "G1" => Ok(Operand::Register(Registers::G1)),
            "G2" => Ok(Operand::Register(Registers::G2)),
            "G3" => Ok(Operand::Register(Registers::G3)),
            "G4" => Ok(Operand::Register(Registers::G4)),
            "G5" => Ok(Operand::Register(Registers::G5)),
            _ => Err(ParseError::InvalidRegister(token)),
        }
    }
    if token.starts_with("?") {
        return match &token[1..] {
            "PC" => Ok(Operand::LongRegister(LongRegisters::PC)),
            "LL" => Ok(Operand::LongRegister(LongRegisters::LL)),
            "GP1" => Ok(Operand::LongRegister(LongRegisters::GP1)),
            "GP2" => Ok(Operand::LongRegister(LongRegisters::GP2)),
            "GP3" => Ok(Operand::LongRegister(LongRegisters::GP3)),
            _ => Err(ParseError::InvalidRegister(token)),
        }
    }

    Ok(Operand::Immediate(parse_biguint_num(token.to_string())?))
}

fn parse_biguint_num(input: String) -> Result<BigUint, ParseError> {
    let (radix, number_str) = if let Some(hex) = input.strip_prefix("0x") {
        (16, hex)
    } else if let Some(bin) = input.strip_prefix("0b") {
        (2, bin)
    } else {
        (10, input.as_str())
    };

    BigUint::from_str_radix(number_str, radix)
        .map_err(|_| ParseError::InvalidNumber(input.to_string()))
}

pub fn parse_u64_num(input: String) -> Result<u64, ParseError> {
    let (radix, number_str) = if let Some(hex) = input.strip_prefix("0x") {
        (16, hex)
    } else if let Some(bin) = input.strip_prefix("0b") {
        (2, bin)
    } else {
        (10, input.as_str())
    };

    u64::from_str_radix(number_str, radix)
        .map_err(|_| ParseError::InvalidNumber(input.to_string()))
}
