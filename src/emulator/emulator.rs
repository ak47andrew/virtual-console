use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use crate::consts::{RAM_SIZE, TARGET_RESOLUTION};

pub struct Emulator {
    memory: Memory,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(1)
        }
    }

    pub fn update_frame(&self, texture: &mut Texture2D) {
        let bytes = self.memory.vram();

        texture.update_texture(bytes).expect("Failed to update texture");
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.memory.put(((y * TARGET_RESOLUTION.x + x) * 4) as usize, &[
            color.r, color.g, color.b, 255
        ])
    }
}

/// Memory layout looks like this (everything is given in bytes):
/// 0-245760: VRAM (245760 = TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4 (Channels: RGBA (bc Raylib, A always 255)))
/// 245761-501761: RAM (RAM_SIZE)
/// 501762+: ROM (everything else, aka rom_size)
pub struct Memory {
    memory: Vec<u8>,
}

impl Memory {
    pub fn new(rom_size: usize) -> Self {
        Self {
            memory: vec![0; Memory::vram_size() + RAM_SIZE + rom_size],
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
}