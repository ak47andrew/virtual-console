use num_bigint::BigUint;
use crate::registers::{LongRegisters, Registers};

pub trait ByteReader {
    fn read_u8(&mut self) -> u8;
    fn read_u64(&mut self) -> u64;
    fn read_biguint(&mut self) -> BigUint;
    fn read_reg(&self, register: Registers) -> u8;
    fn read_reg_long(&self, reg: LongRegisters) -> u64;
}