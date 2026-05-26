use std::fs;
use num_bigint::BigUint;
use num_traits::{ToBytes, ToPrimitive};
use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use raylib::prelude::KeyboardKey::{KEY_C, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP, KEY_X, KEY_Z};
use raylib::RaylibHandle;
use crate::consts::TARGET_RESOLUTION;
use crate::emulator::memory::Memory;
use crate::shared::opcodes::Opcode;
use crate::shared::operand_types::Operand;
use crate::shared::registers::{LongRegisters, Registers};

pub struct Emulator {
    memory: Memory,
    program: Vec<u8>,
    update_texture: bool,
}

impl Emulator {
    pub fn new(file: Option<String>) -> Self {
        let program = match file {
            None => {
                vec![Opcode::Hlt.to_bytecode()]  // Instant hlt
            }
            Some(filename) => {
                fs::read(filename).unwrap()
            }
        };
        Self {
            memory: Memory::new(program.len()),
            program,
            update_texture: false,
        }
    }

    pub fn new_frame(&mut self, texture: &mut Texture2D, rl: &RaylibHandle) {
        if !self.update_texture {
            return;
        }
        self.update_texture = false;

        // Input time
        if self._get_held(rl) != 0 {
            println!("{:b}", self._get_held(rl));
        }
        self.memory.put(Memory::input_held(), &[self._get_held(rl)]);
        self.memory.put(Memory::input_pressed(), &[self._get_pressed(rl)]);

        // Update VRAM
        let bytes = self.memory.vram();

        // Create a copy with forced alpha = 255
        let mut opaque_bytes = bytes.to_vec();
        for i in (3..opaque_bytes.len()).step_by(4) {
            opaque_bytes[i] = 255;
        }

        texture.update_texture(&opaque_bytes).expect("Failed to update texture");
    }

    pub fn _get_pressed(&self, rl: &RaylibHandle) -> u8 {
        let mut out = 0u8;
        for (ind, val) in [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_Z, KEY_X, KEY_C, KEY_SPACE].iter().enumerate() {
            out |= (if rl.is_key_pressed(*val) { 1 } else {0}) << (7 - ind);
        }
        out
    }

    pub fn _get_held(&self, rl: &RaylibHandle) -> u8 {
        let mut out = 0u8;
        for (ind, val) in [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_Z, KEY_X, KEY_C, KEY_SPACE].iter().enumerate() {
            out |= (if rl.is_key_down(*val) { 1 } else {0}) << (7 - ind);
        }
        out
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: Color) {
        self.memory.put(((y * TARGET_RESOLUTION.x + x) * 4) as usize, &[
            color.r, color.g, color.b, 255
        ])
    }

    pub fn load_program_to_rom(&mut self) {
        self.memory.put(self.memory.read_pc(), self.program.as_slice());
    }

    pub fn step(&mut self) {
        match Opcode::from_bytecode(self.memory.read_u8()).unwrap() {
            Opcode::Noop => {} // Noop
            Opcode::Hlt => {
                println!("HALT");
                self.memory.move_pc(-1);  // read_u8 moves to forward, we bring it back to hlt
            }
            Opcode::Vsync => {
                self.update_texture = true;
            }
            Opcode::Mov => {
                let src = Operand::from_bytes(&mut self.memory);
                let dest = Operand::from_bytes(&mut self.memory);

                match (src, dest) {
                    (Operand::Immediate(v), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &[v]);
                    }
                    (Operand::LongImmediate(v), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &v.to_be_bytes());
                    }
                    (Operand::LongerImmediate(v), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &v.to_be_bytes());
                    }
                    (Operand::Register(reg), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &[self.memory.read_reg(reg)])
                    }
                    (Operand::LongRegister(reg), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &self.memory.read_reg_long(reg).to_be_bytes())
                    }
                    (Operand::Immediate(v), Operand::IndirectAddress(reg)) => {
                        self.memory.put(self.memory.read_reg_long(reg) as usize, &[v]);
                    }
                    (Operand::LongImmediate(v), Operand::IndirectAddress(reg)) => {
                        self.memory.put(self.memory.read_reg_long(reg) as usize, &v.to_be_bytes());
                    }
                    (Operand::LongerImmediate(v), Operand::IndirectAddress(reg)) => {
                        self.memory.put(self.memory.read_reg_long(reg) as usize, &v.to_be_bytes());
                    }
                    (Operand::Register(reg1), Operand::IndirectAddress(reg2)) => {
                        self.memory.put(self.memory.read_reg_long(reg2) as usize, &[self.memory.read_reg(reg1)])
                    }
                    (Operand::LongRegister(reg1), Operand::IndirectAddress(reg2)) => {
                        self.memory.put(self.memory.read_reg_long(reg2) as usize, &self.memory.read_reg_long(reg1).to_be_bytes())
                    }
                    (Operand::Address(addr), Operand::Register(reg)) => {
                        println!("{:?}", self.memory.peek(addr as usize, 1));
                        self.memory.write_reg(reg, self.memory.peek(addr as usize, 1)[0])
                    }
                    (Operand::Address(addr), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, u64::from_be_bytes((&self.memory.peek(addr as usize, 8)[..8]).try_into().unwrap()));
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        self.memory.write_reg(reg1, self.memory.read_reg(reg2))
                    }
                    (Operand::LongImmediate(v), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, v);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        self.memory.write_reg_long(reg2, self.memory.read_reg_long(reg1))
                    }
                    (Operand::Immediate(v), Operand::Register(reg)) => {
                        self.memory.write_reg(reg, v);
                    }
                    (_, _) => panic!()
                }
            }
            Opcode::Trunc => {
                let src = Operand::from_bytes(&mut self.memory);
                let dest = Operand::from_bytes(&mut self.memory);

                match (src, dest) {
                    (Operand::LongImmediate(v), Operand::Register(reg)) => {
                        self.memory.write_reg(reg, *v.to_be_bytes().last().unwrap());
                    }
                    (Operand::LongerImmediate(v), Operand::Register(reg)) => {
                        self.memory.write_reg(reg, *v.to_be_bytes().last().unwrap());
                    }
                    (Operand::LongRegister(reg1), Operand::Register(reg2)) => {
                        self.memory.write_reg(reg2, *self.memory.read_reg_long(reg1).to_be_bytes().last().unwrap())
                    }
                    (Operand::LongerImmediate(v), Operand::LongRegister(reg)) => {
                        let input = v.to_be_bytes();
                        let (int_bytes, _) = input.split_at(size_of::<u64>());
                        self.memory.write_reg_long(reg, u64::from_be_bytes(int_bytes.try_into().unwrap()))
                    }
                    (_, _) => panic!()
                }
            }
            Opcode::Ext => {
                let src = Operand::from_bytes(&mut self.memory);
                let dest = Operand::from_bytes(&mut self.memory);

                match (src, dest) {
                    (Operand::Address(addr), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, self.memory.peek(addr as usize, 1)[0] as u64)
                    }
                    (Operand::Immediate(val), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, val as u64)
                    }
                    (Operand::Register(reg1), Operand::LongRegister(reg2)) => {
                        self.memory.write_reg_long(reg2, self.memory.read_reg(reg1) as u64)
                    }
                    (_, _) => panic!()
                }
            }
            Opcode::Copy => {
                let length = match Operand::from_bytes(&mut self.memory) {
                    Operand::Immediate(v) => {BigUint::from(v)}
                    Operand::LongImmediate(v) => {BigUint::from(v)}
                    Operand::LongerImmediate(v) => {v}
                    Operand::Register(reg) => {BigUint::from(self.memory.read_reg(reg))}
                    Operand::LongRegister(reg) => {BigUint::from(self.memory.read_reg_long(reg))}
                    _ => {panic!()}
                };
                let addr1 = Operand::from_bytes(&mut self.memory).unwrap_address(&self.memory);
                let addr2 = Operand::from_bytes(&mut self.memory).unwrap_address(&self.memory);

                let mut offset = BigUint::ZERO;
                let mut remaining = length.clone();

                while remaining > BigUint::ZERO {
                    let chunk = remaining.clone().min(BigUint::from(usize::MAX)).to_usize().unwrap();
                    let data = self.memory.peek((addr1 + offset.to_u64().unwrap()) as usize, chunk).to_vec();
                    self.memory.put(
                        (addr2 + offset.to_u64().unwrap()) as usize,
                        &*data
                    );
                    offset += chunk;
                    remaining -= chunk;
                }
            }
            Opcode::Add => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let (result, overflow) = val1.overflowing_add(val2);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let (result, overflow) = val1.overflowing_add(val2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let (result, overflow) = self.memory.read_reg(reg1).overflowing_add(self.memory.read_reg(reg2));
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg1).overflowing_add(self.memory.read_reg_long(reg2));
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let (result, overflow) = self.memory.read_reg(reg).overflowing_add(val);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg).overflowing_add(val);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Sub => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let (result, overflow) = val1.overflowing_sub(val2);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let (result, overflow) = val1.overflowing_sub(val2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let (result, overflow) = self.memory.read_reg(reg1).overflowing_sub(self.memory.read_reg(reg2));
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg1).overflowing_sub(self.memory.read_reg_long(reg2));
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) => {
                        let (result, overflow) = val.overflowing_sub(self.memory.read_reg(reg));
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Register(reg), Operand::Immediate(val)) => {
                        let (result, overflow) = self.memory.read_reg(reg).overflowing_sub(val);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg).overflowing_sub(val);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let (result, overflow) = val.overflowing_sub(self.memory.read_reg_long(reg));
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Mul => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let (result, overflow) = val1.overflowing_mul(val2);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let (result, overflow) = val1.overflowing_mul(val2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let (result, overflow) = self.memory.read_reg(reg1).overflowing_mul(self.memory.read_reg(reg2));
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg1).overflowing_mul(self.memory.read_reg_long(reg2));
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let (result, overflow) = self.memory.read_reg(reg).overflowing_mul(val);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg).overflowing_mul(val);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Div => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg(Registers::A, div);
                        self.memory.write_reg(Registers::X, modulo);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg_long(LongRegisters::LL1, div);
                        self.memory.write_reg_long(LongRegisters::LL2, modulo);
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let (val1, val2) = (self.memory.read_reg(reg1), self.memory.read_reg(reg2));
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg(Registers::A, div);
                        self.memory.write_reg(Registers::X, modulo);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let (val1, val2) = (self.memory.read_reg_long(reg1), self.memory.read_reg_long(reg2));
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg_long(LongRegisters::LL1, div);
                        self.memory.write_reg_long(LongRegisters::LL2, modulo);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let (val1, val2) = (self.memory.read_reg(reg), val);
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg(Registers::A, div);
                        self.memory.write_reg(Registers::X, modulo);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let (val1, val2) = (self.memory.read_reg_long(reg), val);
                        let (div, modulo) = (val1 / val2, val1 % val2);
                        self.memory.write_reg_long(LongRegisters::LL1, div);
                        self.memory.write_reg_long(LongRegisters::LL2, modulo);
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::And => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 & val2;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let result = val1 & val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let result = self.memory.read_reg(reg1) & self.memory.read_reg(reg2);
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) & self.memory.read_reg_long(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg(reg) & val;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let result = self.memory.read_reg_long(reg) & val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Or => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 | val2;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let result = val1 | val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let result = self.memory.read_reg(reg1) | self.memory.read_reg(reg2);
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) | self.memory.read_reg_long(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg(reg) | val;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let result = self.memory.read_reg_long(reg) | val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Xor => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 ^ val2;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let result = val1 ^ val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        let result = self.memory.read_reg(reg1) ^ self.memory.read_reg(reg2);
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) ^ self.memory.read_reg_long(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg(reg) ^ val;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let result = self.memory.read_reg_long(reg) ^ val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (_, _) => {panic!()}
                }
            }
            Opcode::Not => {
                match Operand::from_bytes(&mut self.memory) {
                    Operand::Immediate(val) => {
                        self.memory.write_reg(Registers::A, !val);
                    }
                    Operand::LongImmediate(val) => {
                        self.memory.write_reg_long(LongRegisters::LL1, !val);
                    }
                    Operand::Register(reg) => {
                        self.memory.write_reg(Registers::A, !self.memory.read_reg(reg));
                    }
                    Operand::LongRegister(reg) => {
                        self.memory.write_reg_long(LongRegisters::LL1, !self.memory.read_reg_long(reg))
                    }
                    _ => panic!()
                }
            }
            Opcode::Shr => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 >> val2;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::Register(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg(reg) >> val;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) => {
                        let result = val >> self.memory.read_reg(reg);
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongImmediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 >> val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val), Operand::Register(reg)) => {
                        let result = val >> self.memory.read_reg(reg);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg_long(reg) >> val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg1), Operand::Register(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) >> self.memory.read_reg(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) >> self.memory.read_reg_long(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) => {
                        let result = self.memory.read_reg_long(reg) >> val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let result = val >> self.memory.read_reg_long(reg);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let result = val1 >> val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (_, _) => panic!()
                }
            }
            Opcode::Shl => {
                let op1 = Operand::from_bytes(&mut self.memory);
                let op2 = Operand::from_bytes(&mut self.memory);

                match (op1, op2) {
                    (Operand::Immediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 << val2;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::Register(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg(reg) << val;
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::Immediate(val), Operand::Register(reg)) => {
                        let result = val << self.memory.read_reg(reg);
                        self.memory.write_reg(Registers::A, result);
                    }
                    (Operand::LongImmediate(val1), Operand::Immediate(val2)) => {
                        let result = val1 << val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val), Operand::Register(reg)) => {
                        let result = val << self.memory.read_reg(reg);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg), Operand::Immediate(val)) => {
                        let result = self.memory.read_reg_long(reg) << val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg1), Operand::Register(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) << self.memory.read_reg(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        let result = self.memory.read_reg_long(reg1) << self.memory.read_reg_long(reg2);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) => {
                        let result = self.memory.read_reg_long(reg) << val;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let result = val << self.memory.read_reg_long(reg);
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (Operand::LongImmediate(val1), Operand::LongImmediate(val2)) => {
                        let result = val1 << val2;
                        self.memory.write_reg_long(LongRegisters::LL1, result);
                    }
                    (_, _) => panic!()
                }
            }
            Opcode::Jmp => {
                match Operand::from_bytes(&mut self.memory) {
                    Operand::Address(addr) => {self.memory.set_pc(addr as usize)},
                    Operand::IndirectAddress(reg) => {self.memory.set_pc(self.memory.read_reg_long(reg) as usize)},
                    _ => panic!()
                }
            },
            Opcode::Je => {
                if match Operand::from_bytes(&mut self.memory) {
                    Operand::Register(reg) => {self.memory.read_reg(reg) != 0}
                    Operand::LongRegister(reg) => {self.memory.read_reg_long(reg) != 0}
                    _ => panic!()
                } {
                    match Operand::from_bytes(&mut self.memory) {  // And yes, I'm gonna keep this repition here so it looks more cursed :P
                        Operand::Address(addr) => {self.memory.set_pc(addr as usize)},
                        Operand::IndirectAddress(reg) => {self.memory.set_pc(self.memory.read_reg_long(reg) as usize)},
                        _ => panic!()
                    }
                } else {
                    let _ = Operand::from_bytes(&mut self.memory); // Just so I can keep cool cursed `if match` 😎
                }
            }
            Opcode::Jne => {
                if match Operand::from_bytes(&mut self.memory) {
                    Operand::Register(reg) => {self.memory.read_reg(reg) == 0}
                    Operand::LongRegister(reg) => {self.memory.read_reg_long(reg) == 0}
                    _ => panic!()
                } {
                    match Operand::from_bytes(&mut self.memory) {
                        Operand::Address(addr) => {self.memory.set_pc(addr as usize)},
                        Operand::IndirectAddress(reg) => {self.memory.set_pc(self.memory.read_reg_long(reg) as usize)},
                        _ => panic!()
                    }
                } else {
                    let _ = Operand::from_bytes(&mut self.memory);
                }
            }
        }
    }
}
