use std::collections::HashMap;
use num_bigint::BigUint;
use num_traits::{Num, ToPrimitive};
use vea_shared::consts::TARGET_RESOLUTION;
use vea_shared::manifest::Manifest;
use crate::operand_checking::get_signature;
use vea_shared::opcodes::Opcode;
use vea_shared::operand_types::{Operand, OperandKind};
use vea_shared::registers::{LongRegisters, Registers};
use crate::errors::CompilationError;

pub fn parse_operands(args: &[&str], labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Result<(Vec<OperandKind>, Vec<Operand>), CompilationError> {
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

fn parse_operand(token: &&str, labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Result<Operand, CompilationError> {
    if token.starts_with("$") {
        return match parse_address_num(token[1..].to_string()) {
            Ok(addr) => Ok(Operand::Address(addr)),
            Err(err) => {
                if is_first_pass {
                    Ok(Operand::Address(0))
                } else if labels.contains_key(&token[1..]) {
                    Ok(Operand::Address(*labels.get(&token[1..]).unwrap() + rom_start(manifest)))
                } else {
                    Err(err)
                }
            }
        }
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
            _ => Err(CompilationError::UnknownRegister(token.to_string())),
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
            _ => Err(CompilationError::UnknownRegister(token.to_string())),
        }
    }
    if token.starts_with("[") && token.ends_with("]") {
        let reg_str = &token[1..token.len() - 1];
        return match parse_operand(&reg_str, labels, is_first_pass, manifest)? {
            Operand::LongRegister(reg) => Ok(Operand::IndirectAddress(reg)),
            _ => Err(CompilationError::IncorrectIndirectAddressBody(token.to_string())),
        };
    }

    parse_numerical_operand(token.to_string())

    // match parse_numerical_operand(token.to_string()) {
    //     Ok(v) => {Ok(v)}
    //     Err(e) => {
    //         if is_first_pass {
    //             Ok(Operand::Address(0))
    //         } else {
    //             Err(e)
    //         }
    //     }
    // }
}

pub fn parse_opcode(token: &str) -> Result<Opcode, CompilationError> {
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

        _ => Err(CompilationError::UnknownOpcode(token.to_string()))
    }
}

fn parse_numerical_operand(input: String) -> Result<Operand, CompilationError> {
    let mut input = input;
    if input.is_empty() {
        return Err(CompilationError::InvalidOperand(input));
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
            match num.to_u8() {
                Some(v) => Ok(Operand::Immediate(v)),
                None => Err(CompilationError::ImmediateOverflow(input)),
            }
        }
        OperandKind::LongImmediate => {
            match num.to_u64() {
                Some(v) => Ok(Operand::LongImmediate(v)),
                None => Err(CompilationError::LongImmediateOverflow(input)),
            }
        }
        OperandKind::LongerImmediate => {
            Ok(Operand::LongerImmediate(num))
        }
        _ => unreachable!()
    }
}

fn parse_num(input: String) -> Result<BigUint, CompilationError> {
    let (radix, number_str) = if let Some(hex) = input.strip_prefix("0x") {
        (16, hex)
    } else if let Some(bin) = input.strip_prefix("0b") {
        (2, bin)
    } else {
        (10, input.as_str())
    };

    BigUint::from_str_radix(number_str, radix)
        .map_err(|_| CompilationError::InvalidOperand(input))
}

pub fn parse_address_num(input: String) -> Result<u64, CompilationError> {
    let num = parse_num(input.clone())?;
    match num.to_u64() {
        Some(num) => Ok(num),
        None => Err(CompilationError::AddressOverflow(input))
    }
}

pub fn encode_opcode(opcode: Opcode, args: &[&str], line: &str, labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Result<Vec<u8>, CompilationError> {
    let (kinds, operands) = parse_operands(args, labels, is_first_pass, manifest)?;

    if !get_signature(opcode).check(&kinds){
        return Err(CompilationError::InvalidOpcodeSignature((opcode, kinds)))
    }

    let mut output = vec![opcode.to_bytecode()];

    for operand in operands {
        output.extend(operand.to_bytes());
    }

    Ok(output)
}