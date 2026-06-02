mod debugger;
mod emulator;
mod memory;

use std::{env, fs};
use raylib::init;
use vea_shared::consts::SCREEN_SIZE;
use crate::debugger::entry_debugger;
use crate::emulator::{entry_emulator, Emulator};


// println!("{}", Memory::input_held());  // 495758
// println!("{}", Memory::input_pressed());  // 495759
// return;
fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);

    #[allow(unused_mut)]
    let (mut rl, mut thread) = init()
        .size(SCREEN_SIZE.x, SCREEN_SIZE.y)
        .title("Rust Raylib")
        .build();
    let mut emulator = Emulator::new();

    if args.len() > 0 && args[0] == "debug" {
        emulator.load_program_to_rom(fs::read(args.get(1).unwrap_or(&"".to_string())));
        entry_debugger(rl, thread, emulator);
    } else {
        emulator.load_program_to_rom(fs::read(args.get(0).unwrap_or(&"".to_string())));
        entry_emulator(rl, thread, emulator);
    }
}
