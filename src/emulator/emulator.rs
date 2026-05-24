use std::collections::HashMap;
use std::fs;
use num_bigint::BigUint;
use num_traits::{ToBytes, ToPrimitive};
use num_traits::ops::overflowing::OverflowingAdd;
use raylib::prelude::{Color, RaylibTexture2D, Texture2D};
use crate::consts::{RAM_SIZE, TARGET_RESOLUTION};
use unsigned_varint::decode as varint;
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

    pub fn new_frame(&mut self, texture: &mut Texture2D) {
        if !self.update_texture {
            return;
        }
        self.update_texture = false;

        // Input time
        // TODO

        // Update VRAM
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
                        self.memory.put(addr as usize, &[self.memory.registers.registers[&reg]])
                    }
                    (Operand::LongRegister(reg), Operand::Address(addr)) => {
                        self.memory.put(addr as usize, &self.memory.registers.long_registers[&reg].to_be_bytes())
                    }
                    (Operand::Address(addr), Operand::Register(reg)) => {
                        self.memory.registers.registers.insert(reg, self.memory.peek(addr as usize, 1)[0]);
                    }
                    (Operand::Address(addr), Operand::LongRegister(reg)) => {
                        self.memory.write_reg_long(reg, u64::from_be_bytes((&self.memory.peek(addr as usize, 8)[..8]).try_into().unwrap()));
                    }
                    (Operand::Register(reg1), Operand::Register(reg2)) => {
                        // self.memory.registers.registers.insert(reg1, self.memory.registers.registers[&reg2]);
                        self.memory.write_reg(reg1, self.memory.read_reg(reg2))
                    }
                    (Operand::LongImmediate(v), Operand::LongRegister(reg)) => {
                        self.memory.registers.long_registers.insert(reg, v);
                    }
                    (Operand::LongRegister(reg1), Operand::LongRegister(reg2)) => {
                        self.memory.registers.long_registers.insert(reg1, self.memory.registers.long_registers[&reg2]);
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
                let addr1 = Operand::from_bytes(&mut self.memory).unwrap_address();
                let addr2 = Operand::from_bytes(&mut self.memory).unwrap_address();

                let mut offset = BigUint::ZERO;
                let mut remaining = length.clone();

                while remaining > BigUint::ZERO {
                    let chunk = remaining.clone().min(BigUint::from(usize::MAX)).to_usize().unwrap();
                    self.memory.put(
                        (addr2 + offset.to_u64().unwrap()) as usize,
                        self.memory.peek((addr1 + offset.to_u64().unwrap()) as usize, chunk).as_slice()
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
                    (Operand::Immediate(val), Operand::Register(reg)) | (Operand::Register(reg), Operand::Immediate(val)) => {
                        let (result, overflow) = self.memory.read_reg(reg).overflowing_sub(val);
                        self.memory.write_reg(Registers::A, result);
                        self.memory.write_reg(Registers::Z, if overflow { 1 } else { 0 });
                    }
                    (Operand::LongRegister(reg), Operand::LongImmediate(val)) | (Operand::LongImmediate(val), Operand::LongRegister(reg)) => {
                        let (result, overflow) = self.memory.read_reg_long(reg).overflowing_sub(val);
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
        }
    }
}

/// Memory layout looks like this (everything is given in bytes):
/// - 0-245760: VRAM (245760 = TARGET_RESOLUTION.x * TARGET_RESOLUTION.y * 4 (Channels: RGBA (bc Raylib, A always 255)))
/// - 245761-501761: RAM (RAM_SIZE)
/// - 501762+: ROM (everything else, aka rom_size)
pub struct Memory {
    memory: Vec<u8>,
    pc: usize,
    registers: RegisterMemory
}

impl Memory {
    pub fn new(rom_size: usize) -> Self {
        Self {
            memory: vec![0; Memory::vram_size() + RAM_SIZE + rom_size],
            pc: Memory::rom_start(),
            registers: RegisterMemory::new()
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

    pub fn write_reg(&mut self, reg: Registers, value: u8) {
        self.registers.registers.insert(reg, value);
    }

    pub fn write_reg_long(&mut self, reg: LongRegisters, value: u64) {
        self.registers.long_registers.insert(reg, value);
    }

    pub fn read_reg(&self, reg: Registers) -> u8 {
        self.registers.registers[&reg]
    }

    pub fn read_reg_long(&self, reg: LongRegisters) -> u64 {
        self.registers.long_registers[&reg]
    }
}

pub struct RegisterMemory {
    pub registers: HashMap<Registers, u8>,
    pub long_registers: HashMap<LongRegisters, u64>,
}

impl RegisterMemory {
    pub fn new() -> RegisterMemory {
        let mut obj = RegisterMemory {
            registers: HashMap::new(),
            long_registers: HashMap::new(),
        };

        for reg in Registers::all() {
            obj.registers.insert(reg, 0);
        }

        for reg in LongRegisters::all() {
            obj.long_registers.insert(reg, 0);
        }

        obj
    }
}