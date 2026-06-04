use std::collections::HashMap;
use num_bigint::BigUint;
use num_traits::{Num, ToPrimitive};
use vea_shared::consts::TARGET_RESOLUTION;
use vea_shared::manifest::Manifest;
use crate::operand_checking::get_signature;
use vea_shared::ParseError;
use vea_shared::opcodes::Opcode;
use vea_shared::operand_types::{Operand, OperandKind};
use vea_shared::registers::{LongRegisters, Registers};

pub fn parse_operands(args: &[&str], labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Result<(Vec<OperandKind>, Vec<Operand>), ParseError> {
    let mut kinds = Vec::new();
    let mut operands = Vec::new();
    for arg in args {
        let operand = parse_operand(arg, labels, is_first_pass, manifest)?;
        kinds.push(operand.kind());
        operands.push(operand);
    }
    Ok((kinds, operands))
}

fn rom_start(manifest: &Manifest) -> u64 {
    TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4 + manifest.settings.ram_size + manifest.settings.stack_size
}

fn parse_operand(token: &&str, labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Result<Operand, ParseError> {
    if labels.contains_key(*token) {
        return Ok(Operand::Address(*labels.get(*token).unwrap() + rom_start(manifest)))
    }

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
            "LL1" => Ok(Operand::LongRegister(LongRegisters::LL1)),
            "LL2" => Ok(Operand::LongRegister(LongRegisters::LL2)),
            "GP1" => Ok(Operand::LongRegister(LongRegisters::GP1)),
            "GP2" => Ok(Operand::LongRegister(LongRegisters::GP2)),
            "GP3" => Ok(Operand::LongRegister(LongRegisters::GP3)),
            _ => Err(ParseError::InvalidRegister(token.to_string())),
        }
    }
    if token.starts_with("[") && token.ends_with("]") {
        let reg_str = &token[1..token.len() - 1];
        return match parse_operand(&reg_str, labels, is_first_pass, manifest)? {
            Operand::LongRegister(reg) => Ok(Operand::IndirectAddress(reg)),
            _ => Err(ParseError::InvalidOperand(token.to_string())),
        };
    }

    match parse_numerical_operand(token.to_string()) {
        Ok(v) => {Ok(v)}
        Err(e) => {
            if is_first_pass {
                Ok(Operand::Address(0))
            } else {
                Err(e)
            }
        }
    }
}

pub fn parse_opcode(token: &str) -> Result<Opcode, ParseError> {
    match token { 
        "noop" => Ok(Opcode::Noop),
        "hlt" => Ok(Opcode::Hlt),
        "vsync" => Ok(Opcode::Vsync),

        "mov" => Ok(Opcode::Mov),
        "trunc" => Ok(Opcode::Trunc),
        "ext" => Ok(Opcode::Ext),
        "copy" => Ok(Opcode::Copy),

        "add" => Ok(Opcode::Add),
        "sub" => Ok(Opcode::Sub),
        "mul" => Ok(Opcode::Mul),
        "div" => Ok(Opcode::Div),

        "and" => Ok(Opcode::And),
        "or" => Ok(Opcode::Or),
        "xor" => Ok(Opcode::Xor),
        "not" => Ok(Opcode::Not),
        "shl" => Ok(Opcode::Shl),
        "shr" => Ok(Opcode::Shr),

        "jmp" => Ok(Opcode::Jmp),
        "je" => Ok(Opcode::Je),
        "jne" => Ok(Opcode::Jne),

        "push" => Ok(Opcode::PUSH),
        "pop" => Ok(Opcode::POP),
        "ret" => Ok(Opcode::RET),
        "call" => Ok(Opcode::CALL),

        _ => Err(ParseError::UnknownOpcode(token.to_string()))
    }
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

pub fn parse_u64_num(input: String) -> Result<u64, ParseError> {
    let num = parse_num(input.clone())?;
    match num.to_u64() {
        Some(num) => Ok(num),
        None => Err(ParseError::InvalidNumber(input))
    }
}

pub fn encode_opcode(opcode: Opcode, args: &[&str], line: &str, labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Vec<u8> {
    let result = parse_operands(args, labels, is_first_pass, manifest);
    let (kinds, operands) = match result {
        Ok((kinds, operands)) => {
            (kinds, operands)
        }
        Err(e) => {
            println!("Failed parsing operands: {:?}. Line {} is gonna be treated as noop", e, line);
            return vec![0];
        }
    };

    if !get_signature(opcode).check(&kinds){
        println!("Incorrect operands\n{} is gonna be treated as noop", line);
        return vec![0];
    }

    let mut output = vec![opcode.to_bytecode()];

    for operand in operands {
        output.extend(operand.to_bytes());
    }

    output
}