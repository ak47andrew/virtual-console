use std::io::Write;
use crate::compiler::ParseError;
use crate::compiler::parsing::{encode_opcode, parse_opcode};

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
    
    // TODO: math - sub, add, mul, div
    // TODO: branching - jmp, jne, je
    // TODO: functions - call, ret
    // TODO: bitwise - and, ro, xor, not, shl, shr
    // TODO: indirect referencing or whatever this called: [?LL1]
    match parse_opcode(op) {
        Ok(opcode) => {
            encode_opcode(opcode, args, line)
        }
        Err(e) => {
            match e {
                ParseError::UnknownOpcode(op) => println!("Unknown op {}. Line's gonna be treated as noop", op),
                e  => println!("Unexpected error happened {:?}. Line's gonna be treated as noop", e),
            }
            vec![0]
        }
    }
}
