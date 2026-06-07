use std::io::Error;
use num_bigint::BigUint;
use num_traits::{ToBytes, ToPrimitive};
use raylib::prelude::{Color, Image, RaylibTexture2D, Texture2D};
use raylib::prelude::KeyboardKey::{KEY_C, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_SPACE, KEY_UP, KEY_X, KEY_Z};
use raylib::{RaylibHandle, RaylibThread};
use raylib::consts::TextureFilter;
use raylib::drawing::RaylibDraw;
use raylib::math::{Rectangle, Vector2};
use vea_shared::bytereader::ByteReader;
use vea_shared::cartridge::Cartridge;
use vea_shared::consts::{SCREEN_SIZE, TARGET_RESOLUTION};
use crate::memory::Memory;
use vea_shared::opcodes::Opcode;
use vea_shared::operand_types::Operand;
use vea_shared::registers::{LongRegisters, Registers};

pub struct Emulator {
    pub memory: Memory,
    pub update_texture: bool,
    pub cartridge: Cartridge,
}

impl Emulator {
    pub fn new(path: String) -> Self {
        // Self {
        //     memory: Memory::new(0),  // Dummy until load_program is called
        //     program: vec![],
        //     update_texture: false,
        // }

        let cartridge = Cartridge::load(path);
        let memory = Memory::new(&cartridge);

        Self {
            memory,
            update_texture: false,
            cartridge
        }
    }

    pub fn new_frame(&mut self, texture: &mut Texture2D, rl: &RaylibHandle) -> bool {
        if !self.update_texture {
            return false;
        }
        self.update_texture = false;

        // Input time
        self.memory.put(self.memory.input_held(), &[self._get_held(rl)]);
        self.memory.put(self.memory.input_pressed(), &[self._get_pressed(rl)]);

        texture.update_texture(&self.calculate_vram()).expect("Failed to update texture");
        true
    }

    fn calculate_vram(&self) -> Vec<u8> {
        // Update VRAM
        let pixels = self.memory.vram();
        let mut out = Vec::new();

        for pixel in pixels {
            out.extend(self.memory.get_color(*pixel))
        }

        out
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
        self.memory.put(((y * TARGET_RESOLUTION.x as i32 + x) * 4) as usize, &[
            color.r, color.g, color.b, 255
        ])
    }

    pub fn step(&mut self) {
        let opcode = Opcode::from_bytecode(self.memory.read_u8()).unwrap();
        // println!("{:?}", opcode);
        match opcode {
            Opcode::Noop => {} // Noop
            Opcode::Hlt => {
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
                        self.memory.write_reg(reg, self.memory.peek(addr as usize, 1)[0])
                    }
                    (Operand::Address(addr), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, u64::from_be_bytes((&self.memory.peek(addr as usize, 8)[..8]).try_into().unwrap()));
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        self.memory.write_reg(reg2, self.memory.read_reg(reg1))
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
                let addr1 = match Operand::from_bytes(&mut self.memory) {
                    Operand::Address(addr) => addr,
                    Operand::IndirectAddress(reg) => self.memory.read_reg_long(reg),
                    _ => panic!()
                };
                let addr2 = match Operand::from_bytes(&mut self.memory) {
                    Operand::Address(addr) => addr,
                    Operand::IndirectAddress(reg) => self.memory.read_reg_long(reg),
                    _ => panic!()
                };

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
                // Man I love how this is implemented. It's the most cursed while still readable piece of code I've ever seen
                if match Operand::from_bytes(&mut self.memory) {
                    Operand::Register(reg) => {self.memory.read_reg(reg) != 0}
                    Operand::LongRegister(reg) => {self.memory.read_reg_long(reg) != 0}
                    _ => panic!()
                } {
                    match Operand::from_bytes(&mut self.memory) {  // And yes, I'm gonna keep this repetition here so it looks more cursed :P
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
            Opcode::PUSH => {
                let value = match Operand::from_bytes(&mut self.memory) {
                    Operand::Immediate(val) => val,
                    Operand::Register(reg) => {self.memory.read_reg(reg)}
                    _ => panic!()
                };

                self.memory.push8(value);
            }
            Opcode::POP => {
                let value = self.memory.pop8();
                self.memory.write_reg(Registers::A, value);
            }
            Opcode::CALL => {
                let target = match Operand::from_bytes(&mut self.memory) {
                    Operand::Address(addr) => addr,
                    Operand::IndirectAddress(reg) => self.memory.read_reg_long(reg),
                    _ => panic!()
                };
                let pc = self.memory.read_pc();
                self.memory.push64(pc as u64);
                self.memory.set_pc(target as usize);
            }
            Opcode::RET => {
                let addr = self.memory.pop64();
                self.memory.set_pc(addr as usize);
            }
        }
    }
}

#[allow(unused_mut)]
pub fn entry_emulator(mut rl: RaylibHandle, mut thread: RaylibThread, mut emulator: Emulator) {
    let mut texture = rl.load_texture_from_image(&thread,
         &Image::gen_image_color(
             TARGET_RESOLUTION.x as i32,
             TARGET_RESOLUTION.y as i32,
             Color::BLACK
         )
    ).unwrap();
    texture.set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

    while !rl.window_should_close() {
        emulator.step();
        emulator.new_frame(&mut texture, &mut rl);

        let mut d = rl.begin_drawing(&thread);

        d.draw_texture_pro(
            &texture,
            Rectangle::new(
                0.0, 0.0,
                texture.width() as f32, texture.height() as f32
            ),
            Rectangle::new(0.0, 0.0, SCREEN_SIZE.x as f32, SCREEN_SIZE.y as f32),
            Vector2::new(0.0, 0.0),
            0.0,
            Color::WHITE
        )
    }
}