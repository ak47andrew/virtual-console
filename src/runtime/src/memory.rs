use std::borrow::Cow;
use num_bigint::BigUint;
use vea_shared::consts::{RAM_SIZE, STACK_SIZE, TARGET_RESOLUTION};
use vea_shared::registers::{LongRegisters, Registers};
use unsigned_varint::decode as varint;
use vea_shared::bytereader::ByteReader;

pub struct Memory {
    memory: Vec<u8>,
    rom_size: usize,
    registers: RegisterMemory
}


impl ByteReader for Memory {
    fn read_u8(&mut self) -> u8 {
        self.read(1)[0]
    }
    fn read_u64(&mut self) -> u64 {
        u64::from_be_bytes((&*self.read(8)).try_into().unwrap())
    }
    fn read_biguint(&mut self) -> BigUint {
        let len = self._read_varint_usize();
        BigUint::from_bytes_be(&*self.read(len))
    }
    fn read_reg(&self, reg: Registers) -> u8 {
        self.registers.registers[reg.to_bytecode() as usize]
    }
    fn read_reg_long(&self, reg: LongRegisters) -> u64 {
        self.registers.long_registers[reg.to_bytecode() as usize]
    }
}

impl Memory {
    pub fn new(rom_size: usize) -> Self {
        let mut obj = Self {
            memory: vec![0; Memory::vram_size() + Memory::ram_size() + Memory::stack_size() + rom_size],
            rom_size,
            registers: RegisterMemory::new(),
        };
        obj.set_pc(Memory::rom_start());
        obj.write_reg_long(LongRegisters::SP, Memory::stack_start() as u64);
        obj
    }

    pub fn vram(&self) -> &[u8] {
        &self.memory[Memory::vram_start()..Memory::vram_start() + Memory::vram_size()]
    }

    pub fn vram_size() -> usize {
        (TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4) as usize
    }
    pub fn ram_size() -> usize {
        RAM_SIZE
    }
    pub fn stack_size() -> usize {
        STACK_SIZE
    }
    pub fn rom_size(&self) -> usize {
        self.rom_size
    }
    pub fn vram_start() -> usize {
        0
    }
    pub fn ram_start() -> usize {
        Memory::vram_size()
    }
    pub fn stack_start() -> usize {
        Memory::ram_start() + Memory::ram_size()
    }
    pub fn input_held() -> usize {
        Memory::ram_start() + Memory::ram_size() - 2
    }
    pub fn input_pressed() -> usize {
        Memory::ram_start() + Memory::ram_size() - 1
    }
    pub fn rom_start() -> usize {
        Memory::stack_start() + Memory::stack_size()
    }
    pub fn total_size(&self) -> usize {
        self.memory.len()
    }

    pub fn put(&mut self, addr: usize, data: &[u8]) {
        let end = addr + data.len();
        if end <= self.memory.len() {
            self.memory[addr..end].copy_from_slice(data);  // memcpy, extremely fast
        } else {
            // wrapping case, rare
            let len = self.memory.len();
            for (i, byte) in data.iter().enumerate() {
                self.memory[(addr + i) % len] = *byte;
            }
        }
    }

    pub fn peek(&self, addr: usize, amount: usize) -> Cow<[u8]> {
        if addr + amount <= self.memory.len() {
            Cow::Borrowed(&self.memory[addr..addr + amount])
        } else {
            // slow wrapping path
            let len = self.memory.len();
            let mut result = Vec::with_capacity(amount);
            for i in 0..amount {
                result.push(self.memory[(addr + i) % len]);
            }
            Cow::Owned(result)
        }
    }

    pub fn read(&mut self, amount: usize) -> Cow<[u8]> {
        let addr = self.read_pc();
        self.move_pc(amount as i32);
        self.peek(addr, amount)
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

    pub fn set_pc(&mut self, new_pc: usize) {
        self.write_reg_long(LongRegisters::PC, new_pc as u64);
    }

    pub fn read_pc(&self) -> usize {
        self.read_reg_long(LongRegisters::PC) as usize
    }

    pub fn move_pc(&mut self, offset: i32) {
        self.set_pc((self.read_pc() as i32 + offset) as usize);
    }

    pub fn push8(&mut self, value: u8) {
        let sp = self.read_reg_long(LongRegisters::SP) as usize;
        if sp + 1 > Memory::stack_size() {
            panic!("Stack overflow");
        }
        self.memory[sp] = value;
        self.write_reg_long(LongRegisters::SP, sp as u64 + 1);
    }

    pub fn pop8(&mut self) -> u8 {
        let sp = self.read_reg_long(LongRegisters::SP) as usize;
        if sp == 0 {
            panic!("Stack underflow");
        }
        let data = self.memory[sp - 1];
        self.write_reg_long(LongRegisters::SP, sp as u64 - 1);
        data
    }

    pub fn push64(&mut self, value: u64) {
        let sp = self.read_reg_long(LongRegisters::SP);
        self.put(sp as usize, &value.to_be_bytes());
        self.write_reg_long(LongRegisters::SP, sp + 8);
    }

    pub fn pop64(&mut self) -> u64 {
        let sp = self.read_reg_long(LongRegisters::SP) - 8;
        self.write_reg_long(LongRegisters::SP, sp);
        u64::from_be_bytes((&*self.peek(sp as usize, 8)).try_into().unwrap())
    }

    pub fn write_reg(&mut self, reg: Registers, value: u8) {
        self.registers.registers[reg.to_bytecode() as usize] = value;
    }

    pub fn write_reg_long(&mut self, reg: LongRegisters, value: u64) {
        self.registers.long_registers[reg.to_bytecode() as usize] = value;
    }
}

pub struct RegisterMemory {
    pub registers: [u8; 0x8 + 1],
    pub long_registers: [u64; 0x6 + 1],
}

impl RegisterMemory {
    pub fn new() -> RegisterMemory {
        RegisterMemory {
            registers: [0; 0x8 + 1],
            long_registers: [0; 0x6 + 1],
        }
    }
}