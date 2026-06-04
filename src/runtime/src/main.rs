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
        .size(SCREEN_SIZE.x as i32, SCREEN_SIZE.y as i32)
        .title("Rust Raylib")
        .build();

    let is_debug = if args.len() > 0 && args[0] == "debug" {
        args.remove(0);
        true
    } else {
        false
    };

    let mut emulator = Emulator::new(args.get(0).unwrap_or(&"".to_string()).to_string());

    if is_debug {
        entry_debugger(rl, thread, emulator);
    } else {
        entry_emulator(rl, thread, emulator);
    }
}
