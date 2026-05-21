use std::io::Write;
use num_bigint::BigUint;
use num_traits::{ToBytes, ToPrimitive};
use crate::compiler::operand_checking::{COPY_SIGNATURE, EXT_SIGNATURE, MOV_SIGNATURE, TRUNC_SIGNATURE};
use crate::compiler::operands::{parse_operands, parse_u64_num, Operand, OperandKind};
use crate::compiler::ParseError;

pub fn entry(file: &str) {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(file).unwrap();
    let lines = file_content.lines().collect::<Vec<&str>>();

    let mut output = vec![];
    for line in lines {
        if line.starts_with(";") {continue;}
        output.extend(compile(line))
    }

    let mut file = std::fs::File::create(
        [file.split(".").collect::<Vec<&str>>()[0], ".veb"].concat(), // .veb = virtual emulator binary
    ).unwrap();
    file.write_all(&output).unwrap();
}

pub fn compile(line: &str) -> Vec<u8> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    if parts.len() == 0 {
        return vec![];
    }
    let op = parts[0];
    let args = &parts[1..];
    
    // TODO: refactor bc of repetition
    // TODO: math - sub, add, mul, div
    // TODO: branching - jmp, jne, je
    // TODO: functions - call, ret
    // TODO: bitwise - and, ro, xor, not, shl, shr
    // TODO: indirect referencing or whatever this called: [?LL1]
    match op {
        "hlt" => {  // Stop execution and enter endless loop
            vec![0x00]
        }
        "mov" => {  // instruction to move value to specified address in memory
            let result = parse_operands(args);
            let (kinds, operands) = match result {
                Ok((kinds, operands)) => {
                    (kinds, operands)
                }
                Err(e) => {
                    println!("Failed parsing operands: {:?}. Line {} is gonna be treated as noop", e, line);
                    return vec![0];
                }
            };

            if !MOV_SIGNATURE.check(&kinds){
                // TODO: add hints
                println!("Incorrect operands\n{} is gonna be treated as noop", line);
                return vec![0];
            }

            let mut output = vec![0x10];

            for operand in operands {
                output.extend(operand.to_bytes());
            }
            
            output
        }
        "trunc" => {
            let result = parse_operands(args);
            let (kinds, operands) = match result {
                Ok((kinds, operands)) => {
                    (kinds, operands)
                }
                Err(e) => {
                    println!("Failed parsing operands: {:?}. Line {} is gonna be treated as noop", e, line);
                    return vec![0];
                }
            };

            if !TRUNC_SIGNATURE.check(&kinds){
                // TODO: add hints
                println!("Incorrect operands\n{} is gonna be treated as noop", line);
                return vec![0];
            }

            let mut output = vec![0x11];

            for operand in operands {
                output.extend(operand.to_bytes());
            }

            output
        }
        "ext" => {
            let result = parse_operands(args);
            let (kinds, operands) = match result {
                Ok((kinds, operands)) => {
                    (kinds, operands)
                }
                Err(e) => {
                    println!("Failed parsing operands: {:?}. Line {} is gonna be treated as noop", e, line);
                    return vec![0];
                }
            };

            if !EXT_SIGNATURE.check(&kinds){
                // TODO: add hints
                println!("Incorrect operands\n{} is gonna be treated as noop", line);
                return vec![0];
            }

            let mut output = vec![0x12];

            for operand in operands {
                output.extend(operand.to_bytes());
            }

            output
        }
        "copy" => {
            let result = parse_operands(args);
            let (kinds, operands) = match result {
                Ok((kinds, operands)) => {
                    (kinds, operands)
                }
                Err(e) => {
                    println!("Failed parsing operands: {:?}. Line {} is gonna be treated as noop", e, line);
                    return vec![0];
                }
            };

            if !COPY_SIGNATURE.check(&kinds){
                // TODO: add hints
                println!("Incorrect operands\n{} is gonna be treated as noop", line);
                return vec![0];
            }

            let mut output = vec![0x13];

            for operand in operands {
                output.extend(operand.to_bytes());
            }

            output
        }
        _ => {
            println!("WARNING: Unknown op {}. Line's gonna be treated as noop", op);
            vec![0]
        }
    }
}
