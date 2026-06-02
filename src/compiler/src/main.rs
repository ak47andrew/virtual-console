use std::{env, fs};
use crate::entry::entry_compiler;

mod entry;
mod operand_checking;
mod parsing;

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);
    if args.len() != 1 || !fs::exists(args[0].clone()).unwrap_or_else(|_| false) {
        eprintln!("Usage: {} <file.vea>", args[0]);
        return;
    }
    entry_compiler(args[0].as_str())
}
