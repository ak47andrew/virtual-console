use std::fs;
use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use crate::consts::{RAM_SIZE, TARGET_RESOLUTION};

pub struct Emulator {
    memory: Memory,
    program: Vec<u8>
}

impl Emulator {
    pub fn new(file: Option<String>) -> Self {
        let program = match file {
            None => {
                vec![2u8]  // Instant hlt
            }
            Some(filename) => {
                fs::read(filename).unwrap()
            }
        };
        Self {
            memory: Memory::new(program.len()),
            program
        }
    }

    pub fn update_frame(&self, texture: &mut Texture2D) {
        let bytes = self.memory.vram();

        // Create a copy with forced alpha = 255
        let mut opaque_bytes = bytes.to_vec();
        for i in (3..opaque_bytes.len()).step_by(4) {
            opaque_bytes[i] = 255;
        }

        texture.update_texture(&opaque_bytes).expect("Failed to update texture");
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.memory.put(((y * TARGET_RESOLUTION.x + x) * 4) as usize, &[
            color.r, color.g, color.b, 255
        ])
    }

    pub fn load_program_to_rom(&mut self) {
        self.memory.put(self.memory.pc, self.program.as_slice());
    }

    pub fn step(&mut self) {
        match self.memory.read_u8() {
            0 => {
                // Noop :P
            }
            1 => { // Mov
                let addr = self.memory.read_u64();
                let value = self.memory.read_u8();
                self.memory.put(addr as usize, &[value]);
            }
            2 => { // Movl

            }
            3 => { // Hlt
                self.memory.move_pc(-1);  // read_u8 move it forward, we move it back
            }
            4 => { // Load
                match self.memory.read_u8() {
                    0 => {
                        let n = self.memory.read_u8();
                    }
                    v => panic!("Unknown load-opcode: {}", v)
                }
            }
            v => {
                panic!("Unknown opcode: {}", v);
            },
        }
    }
}

/// Memory layout looks like this (everything is given in bytes):
/// 0-245760: VRAM (245760 = TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4 (Channels: RGBA (bc Raylib, A always 255)))
/// 245761-501761: RAM (RAM_SIZE)
/// 501762+: ROM (everything else, aka rom_size)
pub struct Memory {
    memory: Vec<u8>,
    pc: usize,
}

impl Memory {
    pub fn new(rom_size: usize) -> Self {
        Self {
            memory: vec![0; Memory::vram_size() + RAM_SIZE + rom_size],
            pc: Memory::rom_start()
        }
    }

    pub fn vram(&self) -> &[u8] {
        &self.memory[..Memory::vram_size()]
    }

    pub fn vram_size() -> usize {
        (TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4) as usize
    }

    pub fn rom_start() -> usize {
        Memory::vram_size() + RAM_SIZE
    }

    pub fn put(&mut self, addr: usize, data: &[u8]) {
        let memory_length = self.memory.len();
        for (i, byte) in data.iter().enumerate() {
            self.memory[(addr + i) % memory_length] = *byte;
        }
    }

    pub fn read(&self, addr: usize, amount: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(amount);
        let len = self.memory.len();

        for i in 0..amount {
            result.push(self.memory[(addr + i) % len]);
        }

        result
    }

    pub fn read_u8(&mut self) -> u8 {
        let output = self.read(self.pc, 1)[0];
        self.pc += 1;
        output
    }

    pub fn read_u64(&mut self) -> u64 {
        let output = u64::from_be_bytes(self.read(self.pc, 8).try_into().unwrap());
        self.pc += 8;
        output
    }

    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    pub fn move_pc(&mut self, offset: i32) {
        self.pc = (self.pc as i32 + offset) as usize;
    }
}