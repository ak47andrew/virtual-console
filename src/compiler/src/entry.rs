use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use vea_shared::cartridge::Cartridge;
use vea_shared::manifest::Manifest;
use vea_shared::ParseError;
use crate::parsing::{encode_opcode, parse_opcode};

pub fn entry_compiler(folder: &str) {
    let manifest_filepath = Path::new(folder).join("manifest.toml");
    let manifest = Manifest::from_file(manifest_filepath);
    let mut cartridge = Cartridge::new(manifest);

    let entry_path = Path::new(folder).join(cartridge.manifest.entry.clone());
    let entry_bytecode = assemble_vea(entry_path, &cartridge.manifest);

    cartridge.manifest.entry = "entry.veb".into();
    cartridge.entry_bytecode = entry_bytecode;

    cartridge.save(folder.to_string() + ".vec")  // Virtual Emulator Cartridge
}

pub fn assemble_vea(file: PathBuf, manifest: &Manifest) -> Vec<u8> {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(file).unwrap();
    let lines = file_content.lines().map(
        |x| x.split(";").next().unwrap_or(x)
    ).map(
        |x| x.trim()
    ).filter(
        |x| !x.is_empty()
    ).collect::<Vec<&str>>();

    // Pass 1
    let mut labels: HashMap<String, u64> = HashMap::new();
    let mut offset: u64 = 0;
    for line in &lines {
        let line = line.trim();
        if let Some(label) = line.strip_suffix(":") {
            labels.insert(label.to_string(), offset);
        } else {
            offset += encode_line(line, &labels, true, manifest).len() as u64;
        }
    }

    // Pass 2
    let mut output = vec![];
    for line in &lines {
        if line.ends_with(":") {continue;}
        output.extend(encode_line(line, &labels, false, manifest));
    }

    output
}

pub fn encode_line(line: &str, labels: &HashMap<String, u64>, is_first_pass: bool, manifest: &Manifest) -> Vec<u8> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    if parts[0] == "" {
        return vec![];
    }
    let op = parts[0];
    let args = &parts[1..];

    match parse_opcode(op) {
        Ok(opcode) => {
            encode_opcode(opcode, args, line, labels, is_first_pass, manifest)
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
