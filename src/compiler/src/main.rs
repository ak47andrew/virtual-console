use std::{env, fs};
use crate::entry::entry_compiler;

mod entry;
mod operand_checking;
mod parsing;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    if args.len() != 2 || !fs::exists(args[1].clone()).unwrap_or_else(|_| false) {
        eprintln!("Usage: {} <source folder>", args[0]);
        return;
    }
    entry_compiler(args[1].as_str())
}
