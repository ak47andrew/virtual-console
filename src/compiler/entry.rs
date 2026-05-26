use std::collections::HashMap;
use std::io::Write;
use crate::compiler::ParseError;
use crate::compiler::parsing::{encode_opcode, parse_opcode};

pub fn entry_compiler(file: &str) {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(file).unwrap();
    let lines = file_content.lines().collect::<Vec<&str>>();

    // Pass 1
    let mut labels: HashMap<String, u64> = HashMap::new();
    let mut offset: u64 = 0;
    for line in &lines {
        let line = line.trim();
        if line.starts_with(";") { continue; }
        if let Some(label) = line.strip_suffix(":") {
            labels.insert(label.to_string(), offset);
        } else {
            offset += compile(line, &labels, true).len() as u64;
        }
    }

    // Pass 2
    let mut output = vec![];
    for line in &lines {
        if line.starts_with(";") || line.ends_with(":") {continue;}
        output.extend(compile(line, &labels, false));
    }

    let mut file = std::fs::File::create(
        [file.split(".").collect::<Vec<&str>>()[0], ".veb"].concat(), // .veb = virtual emulator binary
    ).unwrap();
    file.write_all(&output).unwrap();
}

pub fn compile(line: &str, labels: &HashMap<String, u64>, is_first_pass: bool) -> Vec<u8> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    if parts[0] == "" {
        return vec![];
    }
    let op = parts[0];
    let args = &parts[1..];

    match parse_opcode(op) {
        Ok(opcode) => {
            encode_opcode(opcode, args, line, labels, is_first_pass)
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
