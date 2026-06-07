use std::collections::HashMap;
use std::path::PathBuf;
use log::error;
use vea_shared::manifest::Manifest;
use crate::errors::CompilationError;
use crate::vea::parsing::{encode_opcode, parse_opcode};

pub mod operand_checking;
pub mod parsing;

pub fn assemble_vea(file: &PathBuf, manifest: &Manifest) -> Option<Vec<u8>> {
    // file ends in .vea (virtual emulator assembly)
    let file_content = std::fs::read_to_string(&file).unwrap();
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
    for (ind, line) in lines.iter().enumerate() {
        let line = line.trim();
        if let Some(label) = line.strip_suffix(":") {
            labels.insert(label.to_string(), offset);
            continue;
        }

        match encode_line(line, &labels, true, &file, ind as u32, manifest) {
            Some(d) => {offset += d.len() as u64;}
            None => {return None;}
        }
    }

    // Pass 2
    let mut output = vec![];
    for (ind, line) in lines.iter().enumerate() {
        if line.ends_with(":") {continue;}
        match encode_line(line, &labels, false, &file, ind as u32, manifest) {
            Some(d) => {output.extend(d);}
            None => {return None;}
        }
    }

    Some(output)
}

pub fn encode_line(line: &str, labels: &HashMap<String, u64>, is_first_pass: bool, filename: &PathBuf, line_number: u32, manifest: &Manifest) -> Option<Vec<u8>> {
    let parts = line.split(" ").collect::<Vec<&str>>();
    if parts[0] == "" {
        return Some(vec![]);
    }
    let op = parts[0];
    let args = &parts[1..];

    parse_opcode(op)
        .and_then(|opcode| encode_opcode(opcode, args, line, labels, is_first_pass, manifest))
        .map(Some)
        .unwrap_or_else(|e| {
            error!("{}", format_error(e, filename, line_number, line));
            None
        })
}

fn format_error(compilation_error: CompilationError, filename: &PathBuf, line_number: u32, line: &str) -> String {
    let mut string = String::new();
    let indent = " ".repeat((line_number + 1).ilog10() as usize + 2) + "|\n";

    string.push_str(format!(
        "\n\nerror: {}\n", compilation_error.to_string_error()
    ).as_str());

    string.push_str(format!(
        "  --> {}:{}\n", filename.display(), line_number
    ).as_str());

    string.push_str(indent.as_str());
    string.push_str(format!(
        "{} | {}\n", line_number, line
    ).as_str());
    string.push_str(indent.as_str());

    string
}
