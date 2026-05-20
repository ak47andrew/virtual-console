use num_bigint::BigUint;
use num_traits::{Num, ToBytes, ToPrimitive};
use unsigned_varint::encode as varint;
use crate::compiler::ParseError;

//use unsigned_varint::decode as varint;
//
// fn decode_blob(bytes: &[u8]) -> Result<(Vec<u8>, usize), String> {
//     // first byte already consumed (0x03)
//
//     let (len, len_bytes_used) =
//         varint::usize(&bytes[1..])
//             .map_err(|_| "invalid varint")?;
//
//     let start = 1 + len_bytes_used;
//     let end = start + len;
//
//     if bytes.len() < end {
//         return Err("unexpected EOF".into());
//     }
//
//     let payload = bytes[start..end].to_vec();
//
//     Ok((payload, end))
// }

#[derive(PartialEq)]
pub enum OperandKind {
    Address,
    Immediate,  // 1 byte
    LongImmediate,  // 8 bytes
    LongerImmediate, // 9+ bytes
    Register,
    LongRegister
}

pub enum Operand {
    Address(u64),
    Immediate(u8),
    LongImmediate(u64),
    LongerImmediate(BigUint),
    Register(Registers),
    LongRegister(LongRegisters),
}

impl Operand {
    pub fn kind(&self) -> OperandKind {
        match self {
            Operand::Address(_) => OperandKind::Address,
            Operand::Immediate(_) => OperandKind::Immediate,
            Operand::LongImmediate(_) => OperandKind::LongImmediate,
            Operand::LongerImmediate(_) => OperandKind::LongerImmediate,
            Operand::Register(_) => OperandKind::Register,
            Operand::LongRegister(_) => OperandKind::LongRegister,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Immediate(imm) => vec![0x01, *imm],
            Operand::LongImmediate(imm) => {
                let mut out = vec![0x02];
                out.extend(imm.to_be_bytes());
                out
            }
            Operand::LongerImmediate(imm) => {
                let mut out = vec![0x03];
                let blob = imm.to_be_bytes();

                let mut buf = varint::usize_buffer();
                let encoded = varint::usize(blob.len(), &mut buf);

                out.extend(encoded);
                out.extend(blob);

                out
            }
            Operand::Address(addr) => {
                let mut out = vec![0xAD];
                out.extend(addr.to_be_bytes());
                out
            }
            Operand::Register(reg) => {vec![0x10, reg.to_bytecode()]}
            Operand::LongRegister(reg) => {vec![0x11, reg.to_bytecode()]}
        }
    }
}

pub enum Registers {
    A, X, Y, Z,
    G1, G2, G3, G4, G5,
}

impl Registers {
    pub fn to_bytecode(&self) -> u8 {
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
    pub fn to_bytecode(&self) -> u8 {
        match self {
            LongRegisters::PC => {0xB1}
            LongRegisters::LL => {0xB2}
            LongRegisters::GP1 => {0xC1}
            LongRegisters::GP2 => {0xC2}
            LongRegisters::GP3 => {0xC3}
        }
    }
}

pub fn parse_operands(args: &[&str]) -> Result<(Vec<OperandKind>, Vec<Operand>), ParseError> {
    let mut kinds = Vec::new();
    let mut operands = Vec::new();
    for arg in args {
        let operand = parse_operand(arg)?;
        kinds.push(operand.kind());
        operands.push(operand);
    }
    Ok((kinds, operands))
}

fn parse_operand(token: &&str) -> Result<Operand, ParseError> {
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
            _ => Err(ParseError::InvalidRegister(token.to_string())),
        }
    }
    if token.starts_with("?") {
        return match &token[1..] {
            "PC" => Ok(Operand::LongRegister(LongRegisters::PC)),
            "LL" => Ok(Operand::LongRegister(LongRegisters::LL)),
            "GP1" => Ok(Operand::LongRegister(LongRegisters::GP1)),
            "GP2" => Ok(Operand::LongRegister(LongRegisters::GP2)),
            "GP3" => Ok(Operand::LongRegister(LongRegisters::GP3)),
            _ => Err(ParseError::InvalidRegister(token.to_string())),
        }
    }

    Ok(parse_numerical_operand(token.to_string())?)
}

fn encode_operand(operand: Operand) -> Result<Vec<u8>, ParseError> {
    Ok(vec![])
}


fn parse_numerical_operand(input: String) -> Result<Operand, ParseError> {
    let mut input = input;
    if input.is_empty() {
        return Err(ParseError::InvalidNumber(input));
    }
    let kind = match input.chars().next().unwrap() {
        '&' => {
            input = input.strip_prefix("&").unwrap().to_string();
            OperandKind::LongImmediate
        }
        '^' => {
            input = input.strip_prefix("^").unwrap().to_string();
            OperandKind::LongerImmediate
        }
        _ => {
            OperandKind::Immediate
        }
    };

    let num = parse_num(input.clone())?;

    match kind {
        OperandKind::Immediate => {
            if num > BigUint::from(u8::MAX) {
                return Err(ParseError::InvalidNumber(input))
            }
            Ok(Operand::Immediate(num.to_u8().unwrap()))
        }
        OperandKind::LongImmediate => {
            if num > BigUint::from(u64::MAX) {
                return Err(ParseError::InvalidNumber(input))
            }
            Ok(Operand::LongImmediate(num.to_u64().unwrap()))
        }
        OperandKind::LongerImmediate => {
            Ok(Operand::LongerImmediate(num))
        }
        _ => unreachable!()
    }
}

fn parse_num(input: String) -> Result<BigUint, ParseError> {
    let (radix, number_str) = if let Some(hex) = input.strip_prefix("0x") {
        (16, hex)
    } else if let Some(bin) = input.strip_prefix("0b") {
        (2, bin)
    } else {
        (10, input.as_str())
    };

    BigUint::from_str_radix(number_str, radix)
        .map_err(|_| ParseError::InvalidNumber(input))
}

fn parse_u64_num(input: String) -> Result<u64, ParseError> {
    let operand = parse_numerical_operand(input.clone())?;
    match operand {
        Operand::LongImmediate(num) => Ok(num),
        _ => Err(ParseError::InvalidAddress(input)),
    }
}