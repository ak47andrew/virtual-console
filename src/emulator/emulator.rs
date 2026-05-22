use std::fs;
use num_bigint::BigUint;
use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use crate::consts::{RAM_SIZE, TARGET_RESOLUTION};
use unsigned_varint::decode as varint;
use crate::shared::opcodes::Opcode;

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
        match Opcode::from_bytecode(self.memory.read_u8()).unwrap() {
            Opcode::Noop => {} // Noop
            Opcode::Hlt => {
                self.memory.move_pc(-1);  // read_u8 moves to forward, we bring it back to hlt
            }
            Opcode::Mov => {todo!()}
            Opcode::Trunc => {todo!()}
            Opcode::Ext => {todo!()}
            Opcode::Copy => {todo!()}
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

    pub fn peek(&self, addr: usize, amount: usize) -> Vec<u8> {
        let mut result = Vec::with_capacity(amount);
        let len = self.memory.len();

        for i in 0..amount {
            result.push(self.memory[(addr + i) % len]);
        }

        result
    }

    pub fn read(&mut self, amount: usize) -> Vec<u8> {
        let bytes = self.peek(self.pc, amount);
        self.move_pc(amount as i32);
        bytes
    }

    pub fn read_u8(&mut self) -> u8 {
        self.read(1)[0]
    }

    pub fn read_u64(&mut self) -> u64 {
        u64::from_be_bytes(self.read(8).try_into().unwrap())
    }

    fn _read_varint_usize(&mut self) -> usize {
        let mut buf = [0u8; 10];
        let mut i = 0;
        loop {
            let byte = self.read_u8();
            buf[i] = byte;
            i += 1;
            if byte & 0x80 == 0 {break;}
        }
        varint::usize(&buf)
            .map(|(val, _)| val)
            .unwrap()
    }

    pub fn read_biguint(&mut self) -> BigUint {
        let len = self._read_varint_usize();
        BigUint::from_bytes_be(self.read(len).as_slice())
    }

    pub fn set_pc(&mut self, new_pc: usize) {
        self.pc = new_pc;
    }

    pub fn move_pc(&mut self, offset: i32) {
        self.pc = (self.pc as i32 + offset) as usize;
    }
}
