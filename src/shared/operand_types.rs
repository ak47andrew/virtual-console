use num_bigint::BigUint;
use num_traits::ToBytes;
use unsigned_varint::encode as varint;
use crate::emulator::memory::Memory;
use crate::shared::registers::{LongRegisters, Registers};

#[derive(PartialEq)]
pub enum OperandKind {
    Address,
    Immediate,  // 1 byte
    LongImmediate,  // 8 bytes
    LongerImmediate, // 9+ bytes
    Register,  // 1 byte
    LongRegister, // 8 bytes
    IndirectAddress // Address taken from LongRegister
}

pub enum Operand {
    Address(u64),
    Immediate(u8),
    LongImmediate(u64),
    LongerImmediate(BigUint),
    Register(Registers),
    LongRegister(LongRegisters),
    IndirectAddress(LongRegisters),
}

impl Operand {
    pub fn kind(&self) -> OperandKind {
        match self {
            Operand::Address(_) => OperandKind::Address,
            Operand::Immediate(_) => OperandKind::Immediate,
            Operand::LongImmediate(_) => OperandKind::LongImmediate,
            Operand::LongerImmediate(_) => OperandKind::LongerImmediate,
            Operand::Register(_) => OperandKind::Register,
            Operand::LongRegister(_) => OperandKind::LongRegister,
            Operand::IndirectAddress(_) => OperandKind::IndirectAddress,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Operand::Immediate(imm) => vec![0x01, *imm],
            Operand::LongImmediate(imm) => {
                let mut out = vec![0x02];
                out.extend(imm.to_be_bytes());
                out
            }
            Operand::LongerImmediate(imm) => {
                let mut out = vec![0x03];
                let blob = imm.to_be_bytes();

                let mut buf = varint::usize_buffer();
                let encoded = varint::usize(blob.len(), &mut buf);

                out.extend(encoded);
                out.extend(blob);

                out
            }
            Operand::Address(addr) => {
                let mut out = vec![0xAD];
                out.extend(addr.to_be_bytes());
                out
            }
            Operand::Register(reg) => {vec![0x10, reg.to_bytecode()]}
            Operand::LongRegister(reg) => {vec![0x11, reg.to_bytecode()]}
            Operand::IndirectAddress(reg) => vec![0x12, reg.to_bytecode()]
        }
    }

    pub fn from_bytes(memory: &mut Memory) -> Operand {
        match memory.read_u8() {
            0x01 => Operand::Immediate(memory.read_u8()),
            0x02 => Operand::LongImmediate(memory.read_u64()),
            0x03 => Operand::LongerImmediate(memory.read_biguint()),
            0xAD => Operand::Address(memory.read_u64()),
            0x10 => Operand::Register(Registers::from_bytecode(memory.read_u8())),
            0x11 => Operand::LongRegister(LongRegisters::from_bytecode(memory.read_u8())),
            0x12 => Operand::IndirectAddress(LongRegisters::from_bytecode(memory.read_u8())),
            _ => panic!("Uh... Corrupted program ig")
        }
    }

    pub fn unwrap_address(self, memory: &Memory) -> u64 {
        match self {
            Operand::Address(addr) => addr,
            Operand::IndirectAddress(reg) => memory.read_reg_long(reg),
            _ => panic!()
        }
    }
}
